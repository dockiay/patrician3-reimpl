//! Órdenes de jugador sobre oficinas, ciudades y tareas, sobre la imagen de memoria del original: barco auxiliar de la ciudad (`0x0053AD90`),
//! transferencia con la caja de la ciudad (`0x0053AF30`), sacar un barco de su convoy (`0x00539270`), nivel de mantenimiento (`0x0053DC80`),
//! personal pagado (`0x0053DCE0`), bloqueo de comercio del administrador (`0x0053DD90`), contratar/despedir administrador (`0x0053D990`),
//! ajuste de comercio automático (`0x0053D5C0`), tareas programadas `0x00` y `0x1C` (`0x0053FC70`, `0x0053F810`) y `0x0053A290`.
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

const WORLD: u32 = 0x006d_e4a0;
const G_MERCHANTS: u32 = 0x006d_e4aa;
const G_TOWNS: u32 = 0x006d_e4b0;
const G_TOWN_TABLE: u32 = 0x006d_e508;
const LETTER_COUNT: u32 = 0x006d_d892;
const SHIP_COUNT: u32 = 0x006d_d894;
const SHIP_TABLE: u32 = 0x006d_d7a4;
const CONVOY_COUNT: u32 = 0x006d_d896;
const CONVOY_TABLE: u32 = 0x006d_d7a8;
const LETTERS: u32 = 0x006d_d7a0;
const SCHED: u32 = 0x006d_d73c;

fn town_at(m: &mut dyn Mem, index: u32) -> u32 {
    m.r32(G_TOWN_TABLE).wrapping_add(index.wrapping_mul(0x9f8))
}

/// `0x0053AD90`: cambia el barco auxiliar de la ciudad `op+4`: libera el anterior (tipo 8 vuelve a 3) y toma el barco `op+8` si cumple.
pub fn set_town_outrigger_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let t = m.r32(op + 4);
    if t >= m.r32(G_TOWNS) {
        return;
    }
    let town = town_at(m, t);
    let old = m.r16(town + 0x994);
    let count16 = m.r16(SHIP_COUNT);
    if old < count16 {
        let ship = m.r32(SHIP_TABLE).wrapping_add(u32::from(old) * 0x180);
        if m.r16(ship + 0x134) == 8 {
            let f = m.r8(ship + 0x3c);
            m.w8(ship + 0x3c, f | 0xa0);
            m.w16(ship + 0x134, 3);
            let v = host.call(m, 0x5170f0, ship, &[]);
            m.w32(ship + 0x30, v >> 1);
            m.w16(ship + 0x138, 0x20);
        }
        m.w16(town + 0x994, 0xffff);
    }
    let idx = m.r32(op + 8);
    if idx >= u32::from(m.r16(SHIP_COUNT)) {
        return;
    }
    let ship = m.r32(SHIP_TABLE).wrapping_add(idx.wrapping_mul(0x180));
    if u32::from(m.r8(ship + 0x39)) != m.r32(op + 4) || m.r16(ship + 0x134) > 1 || m.r16(ship + 8) < m.r16(CONVOY_COUNT) {
        return;
    }
    let f = m.r8(ship + 0x3c);
    m.w8(ship + 0x3c, f | 0xa0);
    m.w16(ship + 0x134, 8);
    let v = host.call(m, 0x5170f0, ship, &[]);
    m.w32(ship + 0x30, v >> 1);
    m.w16(ship + 0x138, 0);
    let a = m.r32(op + 0xc);
    m.w32(town + 0x990, a);
    let b = m.r16(op + 8);
    m.w16(town + 0x994, b);
}

/// `0x0053AF30`: mueve dinero entre el comerciante `op+4` y la caja de la ciudad `op+8` (`op+0xC` ≠ 0: ingreso en la ciudad, si no: retirada limitada a la caja).
pub fn town_treasury_transfer(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let who = m.r32(op + 4);
    if who >= u32::from(m.r16(G_MERCHANTS)) {
        return;
    }
    let t = m.r32(op + 8);
    if t >= m.r32(G_TOWNS) {
        return;
    }
    let merchant = host.call(m, 0x5303c0, WORLD, &[who]);
    let town = town_at(m, t);
    if m.r32(op + 0xc) != 0 {
        let amount = m.r32(op + 0x10);
        host.call(m, 0x529f90, town, &[amount, 3]);
        let amount = m.r32(op + 0x10);
        let cash = m.r32(merchant).wrapping_sub(amount);
        m.w32(merchant, cash);
        let sum = m.r32(merchant + 0x4bc).wrapping_add(m.r32(op + 0x10));
        m.w32(merchant + 0x4bc, sum);
    } else {
        let have = m.r32(town + 0x700);
        let want = m.r32(op + 0x10);
        if have < want {
            m.w32(op + 0x10, if (have as i32) <= 0 { 0 } else { have });
        }
        let amount = m.r32(op + 0x10);
        host.call(m, 0x529f90, town, &[amount, 9]);
        let amount = m.r32(op + 0x10);
        let cash = m.r32(merchant).wrapping_add(amount);
        m.w32(merchant, cash);
        let sum = m.r32(merchant + 0x4b8).wrapping_add(m.r32(op + 0x10));
        m.w32(merchant + 0x4b8, sum);
    }
}

/// `0x00539270`: saca el barco `op+4` de su convoy (disuelve el convoy si era el líder) y limpia la selección si coincide.
pub fn remove_ship_from_convoy(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let idx = m.r32(op + 4);
    if idx >= u32::from(m.r16(SHIP_COUNT)) {
        return;
    }
    let ship = m.r32(SHIP_TABLE).wrapping_add(idx.wrapping_mul(0x180));
    let convoy = m.r16(ship + 8);
    if convoy >= m.r16(CONVOY_COUNT) {
        return;
    }
    let record = m.r32(CONVOY_TABLE).wrapping_add(u32::from(convoy) * 60);
    if u32::from(m.r16(record + 0x10)) == idx {
        host.call(m, 0x50a580, LETTERS, &[u32::from(convoy)]);
    } else {
        host.call(m, 0x509f80, LETTERS, &[idx]);
    }
    if m.r16(0x006e_627c) != m.r16(record + 0x1c) {
        return;
    }
    let eax = host.call(m, 0x6141d0, 0x006e_6220, &[0xff, 1]);
    // `movzx ax, byte` conserva los 16 bits altos del EAX que dejó la llamada anterior.
    let sel = (eax & 0xffff_0000) | u32::from(m.r8(record + 0x1c));
    host.call(m, 0x6141d0, 0x006e_6220, &[sel, 0]);
}

fn office(m: &mut dyn Mem, host: &mut dyn OpsHost, merchant: u32, town: u32) -> u32 {
    host.call(m, 0x5308a0, WORLD, &[merchant & 0xffff, town & 0xffff, 0])
}

/// `0x0053DC80`: fija el nivel `op+0xC` (≤4) del mantenimiento `op+0x10` (≤2) de la oficina del comerciante `op+4` en la ciudad `op+8`.
pub fn set_office_upkeep_level(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    if m.r32(op + 4) >= u32::from(m.r16(G_MERCHANTS)) || m.r32(op + 8) >= m.r32(G_TOWNS) || m.r32(op + 0x10) > 2 || m.r32(op + 0xc) > 4 {
        return;
    }
    let (a, b) = (m.r32(op + 4), m.r32(op + 8));
    let o = office(m, host, a, b);
    if o == 0 {
        return;
    }
    let k = m.r32(op + 0x10);
    let v = m.r8(op + 0xc);
    m.w8(o.wrapping_add(k).wrapping_add(0x2e4), v);
}

/// `0x0053DCE0`: fija el personal pagado (`op+0xC`) de la oficina del comerciante `op+4` en la ciudad `op+8`.
pub fn set_office_paid_staff_count(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    if m.r32(op + 4) >= u32::from(m.r16(G_MERCHANTS)) || m.r32(op + 8) >= m.r32(G_TOWNS) {
        return;
    }
    let (a, b) = (m.r32(op + 4), m.r32(op + 8));
    let o = office(m, host, a, b);
    if o == 0 {
        return;
    }
    let v = m.r8(op + 0xc);
    m.w8(o + 0x2c7, v);
}

/// `0x0053DD90`: activa o desactiva (`op+0x10`) el bit `op+4` (<0x18) del bloqueo de comercio de la oficina (comerciante `op+8`, ciudad `op+0xC`).
pub fn set_office_administrator_trade_lock(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let (a, b) = (m.r16(op + 8), m.r16(op + 0xc));
    let o = host.call(m, 0x5308a0, WORLD, &[u32::from(a), u32::from(b), 0]);
    if o == 0 {
        return;
    }
    let bit = m.r32(op + 4);
    if bit >= 0x18 {
        return;
    }
    let mask = 1u32 << bit;
    let v = m.r32(o + 0x3b4);
    if m.r32(op + 0x10) != 0 {
        m.w32(o + 0x3b4, v | mask);
    } else {
        m.w32(o + 0x3b4, v & !mask);
    }
}

/// `0x0053D990`: despide (`op+0xC` = 1) o contrata (`op+0xC` = 0) al administrador de la oficina (comerciante `op+4`, ciudad `op+8`).
pub fn hire_or_fire_office_administrator(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let (a, b) = (m.r16(op + 4), m.r16(op + 8));
    let o = host.call(m, 0x5308a0, WORLD, &[u32::from(a), u32::from(b), 0]);
    if o == 0 {
        return;
    }
    match m.r32(op + 0xc) {
        0 => {
            if m.r16(o + 0x2f2) < m.r16(LETTER_COUNT) {
                return;
            }
            let letter = host.call(m, 0x5097c0, LETTERS, &[0]) & 0xffff;
            m.w16(o + 0x2f2, letter as u16);
            let rec = m.r32(LETTERS).wrapping_add(letter << 4);
            m.w8(rec + 0xa, 0);
            host.call(m, 0x4fe160, rec, &[]);
        }
        1 => {
            if m.r16(o + 0x2f2) >= m.r16(LETTER_COUNT) {
                return;
            }
            host.call(m, 0x5098b0, LETTERS, &[o + 0x2f2]);
            let f = m.r8(o + 0x2d6);
            m.w8(o + 0x2d6, f & 0xfc);
        }
        _ => {}
    }
}

/// `0x0053D5C0`: fija la cantidad (`op+8`) y el precio (`op+4`) de comercio automático de la mercancía `op+0x10` en la oficina del comerciante `op+0xC`.
pub fn set_office_autotrade_ware_setting(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let who = m.r32(op + 0xc);
    let o = host.call(m, 0x530400, WORLD, &[who]);
    if o == 0 {
        return;
    }
    if m.r16(o + 0x2f2) >= m.r16(LETTER_COUNT) {
        let v = m.r16(o + 0x2f2) & 0xfffc;
        m.w16(o + 0x2f2, v);
        for k in 0..24u32 {
            m.w32(o + 0x2f4 + 4 * k, 0);
            m.w32(o + 0x354 + 4 * k, 0);
        }
        return;
    }
    let k = m.r32(op + 0x10);
    let price = m.r32(op + 4);
    m.w32(o.wrapping_add(k.wrapping_mul(4)).wrapping_add(0x354), price);
    let amount = m.r32(op + 8);
    m.w32(o.wrapping_add(k.wrapping_mul(4)).wrapping_add(0x2f4), amount);
    let k = m.r32(op + 0x10);
    let slot = o.wrapping_add(k.wrapping_mul(4)).wrapping_add(0x2f4);
    let value = m.r32(slot);
    if (value as i32) < 0 {
        let owner = u32::from(m.r16(o + 0x2c4));
        let merchant = host.call(m, 0x5303c0, WORLD, &[owner]);
        let k = m.r32(op + 0x10);
        let neg = (value as u16).wrapping_neg();
        m.w16(merchant.wrapping_add(k.wrapping_mul(4)).wrapping_add(0x5a8), neg);
    } else if (value as i32) > 0 {
        let owner = u32::from(m.r16(o + 0x2c4));
        let merchant = host.call(m, 0x5303c0, WORLD, &[owner]);
        let k = m.r32(op + 0x10);
        let low = m.r16(slot);
        m.w16(merchant.wrapping_add(k.wrapping_mul(4)).wrapping_add(0x5aa), low);
    }
    if m.r32(op + 8) != 0 {
        let f = m.r8(o + 0x2d6);
        m.w8(o + 0x2d6, f | 1);
        return;
    }
    for i in 0..24u32 {
        if m.r32(o + 0x2f4 + 4 * i) != 0 {
            return;
        }
    }
    let f = m.r8(o + 0x2d6);
    m.w8(o + 0x2d6, f & 0xfc);
}

/// `0x0053FC70`: programa una tarea de tipo 0 con el día `op+4` y los datos `op+8` y `op+0xC`.
pub fn schedule_task_00(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let when = m.r32(op + 4);
    let rec = host.call(m, 0x4d8cf0, SCHED, &[when]);
    m.w16(rec + 6, 0);
    let a = m.r32(op + 4);
    m.w32(rec + 8, a);
    let b = m.r32(op + 8);
    m.w32(rec + 0xc, b);
    let c = m.r8(op + 0xc);
    m.w8(rec + 0x10, c);
}

/// `0x0053F810`: programa la tarea `0x1C` en el día del año `op+0x10` (este año o el siguiente) con los 16 bytes que empiezan en `op+4`.
pub fn schedule_task_1c_on_day_of_year(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let day = u32::from(m.r16(op + 0x10));
    let when = if day < u32::from(m.r16(0x006d_e4a4)) {
        (m.r32(0x006d_e4b4) | 0xff).wrapping_add(1)
    } else {
        let year = u32::from(m.r16(0x006d_e4a2));
        (year.wrapping_mul(365).wrapping_add(day)) << 8
    };
    let rec = host.call(m, 0x4d8cf0, SCHED, &[when]);
    m.w16(rec + 6, 0x1c);
    for i in 0..16 {
        let b = m.r8(op + 4 + i);
        m.w8(rec + 8 + i, b);
    }
}

/// `0x0053A290`: copia `op+8` al campo `+4` del comerciante `op+4`.
pub fn bathe_set_merchant_field_4(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let who = m.r32(op + 4);
    if who >= u32::from(m.r16(G_MERCHANTS)) {
        return;
    }
    let merchant = host.call(m, 0x5303c0, WORLD, &[who]);
    let v = m.r32(op + 8);
    m.w32(merchant + 4, v);
}

/// Peso en unidades de un lote de armamento (tabla de `u16` en `0x00672CB4`, seis tipos).
pub const ARMAMENT_LOT_WEIGHT: [u32; 6] = [1000, 1000, 2000, 2000, 2000, 1000];
const STOCK_CAP: u32 = 1_000_000_000;

/// `0x0053CAF0` (orden `0x56`): compra (`op+0x10 < 0x100`) o vende un lote de armamento entre el comerciante `op+4` y el almacén de la ciudad
/// `op+0x10 & 0xFF`, a través de su oficina. `op+8` es el tipo (6 o más: una pieza suelta en `+0x2BC`) y `op+0xC` el precio.
pub fn op_buy_sell_ship_armament_lot(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let who = m.r32(op + 4);
    if who >= u32::from(m.r16(G_MERCHANTS)) {
        return;
    }
    let flag = m.r32(op + 0x10);
    let tw = flag & 0xff;
    if tw >= m.r32(G_TOWNS) {
        return;
    }
    let town = town_at(m, tw);
    let off = host.call(m, 0x5308a0, WORLD, &[who & 0xffff, tw, 0]);
    let merchant = host.call(m, 0x5303c0, WORLD, &[who]);
    if off == 0 {
        return;
    }
    let kind = m.r32(op + 8);
    let price = m.r32(op + 0xc);
    let (src, dst) = if flag < 0x100 {
        if (m.r32(merchant) as i32) < price as i32 {
            return;
        }
        (town, off)
    } else {
        (off, town)
    };
    if kind >= 6 {
        let v = m.r32(src + 0x2bc);
        if v < 1 {
            return;
        }
        m.w32(src + 0x2bc, v - 1);
        let d = m.r32(dst + 0x2bc);
        m.w32(dst + 0x2bc, d.wrapping_add(1));
    } else {
        let w = ARMAMENT_LOT_WEIGHT[kind as usize];
        let a = src + 0x124 + 4 * kind;
        let have = m.r32(a);
        if (have as i32) < w as i32 {
            return;
        }
        m.w32(a, have.wrapping_sub(w));
        let b = dst + 0x124 + 4 * kind;
        let d = m.r32(b);
        m.w32(b, d.wrapping_add(w));
    }
    let money = m.r32(merchant);
    let tally = if flag < 0x100 { 0x4bc } else { 0x4b8 };
    m.w32(merchant, if flag < 0x100 { money.wrapping_sub(price) } else { money.wrapping_add(price) });
    let t = m.r32(merchant + tally);
    m.w32(merchant + tally, t.wrapping_add(price));
}

/// `0x0053D6D0` (orden `0x5C`): compra (`op+0x10 != 0`, el comerciante paga `op+4`) o vende armamento de la oficina que tiene el comerciante
/// `op+8` en la ciudad `op+0xA`, tipo `op+0xC` y cantidad `op+0xE` (todos `u16`). Los almacenes se saturan en 10^9.
pub fn op_trade_ship_armament_with_office(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let who = u32::from(m.r16(op + 8));
    if who >= u32::from(m.r16(G_MERCHANTS)) {
        return;
    }
    let tw = u32::from(m.r16(op + 0xa));
    if tw >= m.r32(G_TOWNS) {
        return;
    }
    let merchant = host.call(m, 0x5303c0, WORLD, &[who]);
    let off = host.call(m, 0x5308a0, WORLD, &[who, tw, 0]);
    if off == 0 {
        return;
    }
    let town = town_at(m, tw);
    let kind = u32::from(m.r16(op + 0xc));
    let qty = u32::from(m.r16(op + 0xe));
    let price = m.r32(op + 4);
    let buy = m.r8(op + 0x10) != 0;
    let money = m.r32(merchant);
    if kind >= 6 {
        if buy {
            let t = m.r32(town + 0x2bc);
            if qty > t || qty > 0xffff_ffff - m.r32(off + 0x2bc) || (money as i32) < price as i32 {
                return;
            }
            m.w32(town + 0x2bc, t - qty);
            let o = m.r32(off + 0x2bc);
            m.w32(off + 0x2bc, o.wrapping_add(qty));
            m.w32(merchant, money.wrapping_sub(price));
            let a = m.r32(merchant + 0x4a0);
            m.w32(merchant + 0x4a0, a.wrapping_add(price));
        } else {
            let o = m.r32(off + 0x2bc);
            if qty > o {
                return;
            }
            let t = m.r32(town + 0x2bc);
            m.w32(town + 0x2bc, if STOCK_CAP - qty >= t { t + qty } else { STOCK_CAP });
            m.w32(off + 0x2bc, o - qty);
            m.w32(merchant, money.wrapping_add(price));
            let a = m.r32(merchant + 0x49c);
            m.w32(merchant + 0x49c, a.wrapping_add(price));
        }
        return;
    }
    let w = ARMAMENT_LOT_WEIGHT[kind as usize];
    let ts = town + 0x124 + 4 * kind;
    let os = off + 0x124 + 4 * kind;
    if buy {
        let have = m.r32(ts) as i32;
        if qty as i32 > have / w as i32 {
            return;
        }
        let room = 0xffff_ffffu32.wrapping_sub(m.r32(os).wrapping_mul(w));
        if qty > room || (money as i32) < price as i32 {
            return;
        }
        let lot = w.wrapping_mul(qty);
        m.w32(ts, (have as u32).wrapping_sub(lot));
        let o = m.r32(os);
        m.w32(os, o.wrapping_add(lot));
        m.w32(merchant, money.wrapping_sub(price));
        let a = m.r32(merchant + 0x4a0);
        m.w32(merchant + 0x4a0, a.wrapping_add(price));
    } else {
        let lot = w.wrapping_mul(qty);
        let have = m.r32(os);
        if lot as i32 > have as i32 {
            return;
        }
        let t = m.r32(ts);
        m.w32(ts, if (STOCK_CAP.wrapping_sub(lot) as i32) >= t as i32 { t.wrapping_add(lot) } else { STOCK_CAP });
        m.w32(os, have.wrapping_sub(lot));
        m.w32(merchant, money.wrapping_add(price));
        let a = m.r32(merchant + 0x49c);
        m.w32(merchant + 0x49c, a.wrapping_add(price));
    }
}
