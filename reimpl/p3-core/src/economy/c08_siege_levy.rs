//! C08: operaciones 0x8F (`op_levy_town_contribution`, `0x00540240`) y 0x90 (`op_siege_dialog_action`,
//! `0x00540490`) del despachador.
//!
//! 0x8F: la ciudad sitiada `op+0x10` reparte un tributo entre las oficinas de su cadena (`Town+0x784`, enlace
//! `+0x2CA`). Si `op+0xC` no es nulo, avisa antes a todos (carta 0x38 con `op+8..+0xF`). Cada comerciante paga
//! `⌊((70·v/(v+100000) + v mod 21 + 10)·cuota + 5000)/10000⌋·100` (tope 65.500), con `v = Merchant+0x46C` y la cuota
//! `op+4` (si pasa de 65.000, `((Town+0x2E0 & 0xF) + 50)·1000`). La IA paga al instante (oro y `+0x4BC`); a los
//! humanos se les envía la carta 0x37 y se programa la deuda (tarea 1) para dentro de 0x1400 ticks.
//!
//! 0x90: sin escena solo cambia `SiegeRecord+0x101A` (`op+9` lo pone a 1, `op+0xA` a 0); comprar la libertad, la
//! rendición y el ataque leen el objeto de escena (`+8`) y se rechazan aquí.
use super::ops_misc::OpsHost;
use crate::flatmem::Mem;

const WORLD: u32 = 0x006d_e4a0;
const SCHED: u32 = 0x006d_d73c;
const QUEUE: u32 = 0x006d_d730;
const TOWN_COUNT: u32 = 0x006d_e4b0;
const TOWNS: u32 = 0x006d_e508;
const OFFICE_COUNT: u32 = 0x006d_e4a8;
const TICK: u32 = 0x006d_e4b4;
const SIEGES: u32 = 0x006d_f270;

fn send(m: &mut dyn Mem, host: &mut dyn OpsHost, to: u32, msg: &[u8; 16]) {
    let mut args = vec![to];
    args.extend(msg.iter().map(|b| u32::from(*b)));
    host.call(m, 0x4d6530, QUEUE, &args);
}

/// Parte de un comerciante en el tributo.
pub fn levy_share(merchant_46c: i32, quota: u32) -> u32 {
    let v = merchant_46c;
    let ratio = v.wrapping_mul(70).wrapping_div(v.wrapping_add(100_000));
    let factor = ratio.wrapping_add(v.wrapping_rem(21)).wrapping_add(10) as u32;
    let share = (factor.wrapping_mul(quota).wrapping_add(5000) / 10_000) * 100;
    share.min(0xffdc)
}

/// `op_levy_town_contribution`. `op` es la dirección de la operación de 20 bytes.
pub fn levy_town_contribution(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) -> Result<(), &'static str> {
    let city = u32::from(m.r8(op + 0x10));
    if city >= m.r32(TOWN_COUNT) {
        return Ok(());
    }
    let mut msg = [0u8; 16];
    msg[1] = 0x80;
    msg[2..6].fill(0xff);
    let warning = m.r16(op + 0xc);
    if warning != 0 {
        msg[8..10].copy_from_slice(&warning.to_le_bytes());
        msg[0xa..0xc].copy_from_slice(&m.r16(op + 8).to_le_bytes());
        msg[5] = city as u8;
        msg[0xc..0xe].copy_from_slice(&m.r16(op + 0xa).to_le_bytes());
        msg[4] = 0x38;
        msg[0xe..0x10].copy_from_slice(&m.r16(op + 0xe).to_le_bytes());
        send(m, host, 0xffff_fffe, &msg);
    } else {
        msg[5] = city as u8;
    }
    msg[4] = 0x37;
    let quota_in = m.r32(op + 4);
    msg[8..0xc].copy_from_slice(&quota_in.to_le_bytes());
    let town = m.r32(TOWNS).wrapping_add(city * 0x9f8);
    if quota_in > 0xfde8 {
        let base = (m.r32(town + 0x2e0) & 0xf) + 0x32;
        m.w32(op + 4, base * 1000);
    }
    let quota = m.r32(op + 4);
    let offices = u32::from(m.r16(OFFICE_COUNT));
    let mut at = u32::from(m.r16(town + 0x784));
    let mut steps = 0;
    while at < offices {
        steps += 1;
        if steps > offices {
            return Err("cadena de oficinas del tributo cíclica");
        }
        let office = host.call(m, 0x530400, WORLD, &[at]);
        let owner = u32::from(m.r16(office + 0x2c4));
        let merchant = host.call(m, 0x5303c0, WORLD, &[owner]);
        if merchant == 0 {
            return Err("comerciante del tributo ausente");
        }
        let v = m.r32(merchant + 0x46c) as i32;
        if v.wrapping_add(100_000) == 0 {
            return Err("tributo con divisor nulo (el original divide por cero)");
        }
        let share = levy_share(v, quota);
        if m.r16(merchant + 8) == 0 {
            msg[8..0xc].copy_from_slice(&share.to_le_bytes());
            send(m, host, owner, &msg);
            let tick = m.r32(TICK);
            let task = host.call(m, 0x4d8cf0, SCHED, &[tick.wrapping_add(0x1400)]);
            if task == 0 {
                return Err("alta de la deuda del tributo rechazada");
            }
            m.w16(task + 6, 1);
            m.w8(task + 0x12, 0);
            m.w8(task + 0x15, 0);
            m.w8(task + 0x16, 0);
            m.w8(task + 0x17, 0);
            m.w32(task + 8, share);
            m.w8(task + 0x13, 0);
            m.w8(task + 0x10, 0xff);
            let owner_byte = m.r8(office + 0x2c4);
            m.w8(task + 0x11, owner_byte);
            let now = m.r32(TICK);
            m.w32(task + 0xc, now);
            m.w8(task + 0xc, 0);
            m.w8(task + 0x14, city as u8);
        } else {
            let (gold, paid) = (m.r32(merchant), m.r32(merchant + 0x4bc));
            m.w32(merchant, gold.wrapping_sub(share));
            m.w32(merchant + 0x4bc, paid.wrapping_add(share));
        }
        at = u32::from(m.r16(office + 0x2ca));
    }
    Ok(())
}

/// `op_siege_dialog_action`: solo las ramas que no tocan la escena.
pub fn siege_dialog_action(m: &mut dyn Mem, op: u32) -> Result<(), &'static str> {
    let city = u32::from(m.r8(op + 8));
    if city >= m.r32(TOWN_COUNT) {
        return Ok(());
    }
    let r = m.r32(SIEGES).wrapping_add(city * 0x1024);
    if m.r32(op + 4) != 0 {
        return Err("compra de la libertad del asedio (0x00634660) sobre la escena no conectada");
    }
    if m.r8(op + 9) != 0 {
        m.w8(r + 0x101a, 1);
        return Ok(());
    }
    if m.r8(op + 0xa) != 0 {
        m.w8(r + 0x101a, 0);
        return Ok(());
    }
    Err("rendición o ataque del asedio (0x006347B0/0x00634820) sobre la escena no conectados")
}
