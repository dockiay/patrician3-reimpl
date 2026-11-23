//! C06: venta de edificios de la IA. `op_queue_building_work_task` (`0x004DAB00`, `ret 8`, ECX = planificador),
//! llamado por la operación 0x28 (`op_put_building_up_for_auction`, `0x0053D190`).
//!
//! Si el edificio no está ya en subasta y su dueño tiene gremio en la ciudad, lo añade como lote libre de una subasta
//! anunciada (tarea 0x36 en fase 1 que vence antes de `(tick | 0xFF) + 0x301`) o en curso (fase 2, hasta
//! `tick + 0x3200`); si no, crea una subasta nueva (tarea 0x36 en fase 1) eligiendo en un calendario de 400 días seis
//! días seguidos libres de otras subastas (tareas 0x0A y 0x36).
//!
//! Memoria del original ([`Mem`]); el alta de tarea (`0x004D8CF0`) pasa por [`OpsHost`].
use super::ops_misc::OpsHost;
use crate::flatmem::Mem;

const SCHED: u32 = 0x006d_d73c;
const TOWN_COUNT: u32 = 0x006d_e4b0;
const TOWNS: u32 = 0x006d_e508;
const MERCHANT_COUNT: u32 = 0x006d_e4aa;
const MERCHANTS: u32 = 0x006d_e518;
const TICK: u32 = 0x006d_e4b4;
const OPTIONS: u32 = 0x006c_c3e8;

fn task(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(SCHED).wrapping_add(i.wrapping_mul(0x18))
}

/// `scheduled_tasks_get_building_auction_info` (`0x004DAFE0`) reducida a lo que usa esta función: si el edificio
/// figura en algún lote de una tarea 0x36 de la ciudad.
fn in_auction(m: &mut dyn Mem, town: u8, building: u16) -> bool {
    let capacity = u32::from(m.r16(SCHED + 0xc));
    let mut at = u32::from(m.r16(SCHED + 8));
    let mut steps = 0;
    while at < capacity && steps <= capacity {
        let t = task(m, at);
        if m.r16(t + 6) == 0x36 && m.r8(t + 0x15) == town && (0..3).any(|k| m.r16(t + 8 + 2 * k) & 0x7fff == building) {
            return true;
        }
        at = u32::from(m.r16(t + 4));
        steps += 1;
    }
    false
}

/// Escribe el lote en la primera ranura libre de la subasta `t` (las ocupadas tienen un edificio de la ciudad).
/// Devuelve `true` si había hueco. Con la celda del edificio sin marcar, el lote queda sin el bit 0x8000.
fn append_lot(m: &mut dyn Mem, t: u32, town: u32, record: u32, building: u16) -> bool {
    let count = m.r16(town + 0x776);
    for k in 0..3 {
        if m.r16(t + 8 + 2 * k) & 0x7fff >= count {
            m.w16(t + 8 + 2 * k, building);
            if cell_marked(m, town, record) {
                m.w16(t + 8 + 2 * k, building | 0x8000);
            }
            return true;
        }
    }
    false
}

fn cell_marked(m: &mut dyn Mem, town: u32, record: u32) -> bool {
    let x = u32::from(m.r8(record));
    let y = u32::from(m.r8(record + 1));
    let at = m.r32(town + 0x7ac).wrapping_mul(x).wrapping_add(y).wrapping_add(m.r32(town + 0x80c));
    m.r8(at) & 0x80 != 0
}

/// `op_queue_building_work_task(town, building)`. Los argumentos se reciben completos; el original usa el byte bajo
/// de la ciudad y la palabra baja del edificio. Una subasta cuyo fin ya pasó haría escribir al original fuera de su
/// calendario de 400 días: se devuelve `Err` sin cambios en el planificador.
pub fn queue_building_work_task(m: &mut dyn Mem, host: &mut dyn OpsHost, town_arg: u32, building_arg: u32) -> Result<(), &'static str> {
    let city = town_arg & 0xff;
    if city >= m.r32(TOWN_COUNT) {
        return Ok(());
    }
    let town = m.r32(TOWNS).wrapping_add(city * 0x9f8);
    let building = building_arg as u16;
    if building >= m.r16(town + 0x776) {
        return Ok(());
    }
    if in_auction(m, city as u8, building) {
        return Ok(());
    }
    let record = m.r32(town + 0x75c).wrapping_add(u32::from(building) * 8);
    let owner = m.r8(record + 2);
    if u16::from(owner) >= m.r16(MERCHANT_COUNT) {
        return Ok(());
    }
    let merchant = m.r32(MERCHANTS).wrapping_add(u32::from(owner) * 0x650);
    if m.r32(merchant + 0x468) & (1u32 << (city & 31)) == 0 {
        return Ok(());
    }
    let tick = m.r32(TICK);
    let capacity = u32::from(m.r16(SCHED + 0xc));
    let options = m.r32(OPTIONS);
    let mut at = u32::from(m.r16(SCHED + 8));
    // Fase 1: subastas anunciadas que vencen antes de (tick | 0xFF) + 0x301.
    let soon = (tick | 0xff).wrapping_add(0x301);
    let single = m.r8(options + 0xd) != 0;
    while at < capacity {
        let t = task(m, at);
        if m.r32(t) >= soon {
            break;
        }
        if !single && m.r16(t + 6) == 0x36 && m.r8(t + 0x15) == city as u8 && m.r8(t + 0x14) == 1 && append_lot(m, t, town, record, building) {
            return Ok(());
        }
        at = u32::from(m.r16(t + 4));
    }
    // Fase 2: subastas en curso hasta tick + 0x3200, continuando desde donde quedó la primera búsqueda.
    let later = tick.wrapping_add(0x3200);
    while at < capacity {
        let t = task(m, at);
        if m.r32(t) > later {
            break;
        }
        if m.r16(t + 6) == 0x36 && m.r8(t + 0x15) == city as u8 && m.r8(t + 0x14) == 2 && append_lot(m, t, town, record, building) {
            return Ok(());
        }
        at = u32::from(m.r16(t + 4));
    }
    // Calendario de 400 días: 1 = libre; cada subasta 0x0A/0x36 ocupa seis días desde su final.
    let mut days = [1u8; 0x190];
    let mut at = u32::from(m.r16(SCHED + 8));
    let mut steps = 0;
    while at < capacity && steps <= capacity {
        let t = task(m, at);
        let kind = m.r16(t + 6);
        if kind == 0x0a || kind == 0x36 {
            let due = m.r32(t);
            let end = match m.r8(t + 0x14) {
                0 => due.wrapping_add(0x2000),
                1 => due.wrapping_add(u32::from(m.r16(t + 0xe))).wrapping_add(0x200),
                2 => due.wrapping_add(0x200),
                _ => due,
            };
            let day = (end.wrapping_sub(tick) as i32) >> 8;
            if day < 0x17c {
                if day < 0 {
                    return Err("subasta terminada antes del día actual (el original escribiría fuera del calendario)");
                }
                for d in 0..6 {
                    days[day as usize + d] = 0;
                }
            }
        }
        at = u32::from(m.r16(t + 4));
        steps += 1;
    }
    let new = host.call(m, 0x4d8cf0, SCHED, &[tick.wrapping_add(1)]);
    if new == 0 {
        return Err("alta de subasta rechazada");
    }
    m.w16(new + 6, 0x36);
    m.w8(new + 0x17, 0xff);
    m.w16(new + 8, building);
    if cell_marked(m, town, record) {
        m.w16(new + 8, building | 0x8000);
    }
    m.w16(new + 0xa, 0xffff);
    m.w16(new + 0xc, 0xffff);
    m.w16(new + 0x10, 0);
    m.w16(new + 0x12, 0);
    m.w8(new + 0x14, 1);
    m.w8(new + 0x15, city as u8);
    m.w8(new + 0x16, 0);
    let mut start = ((tick & 3) + (u32::from(building) & 3) + 0xe) as u16;
    // Desliza el inicio hasta seis días seguidos libres (comprobados de `inicio + 5` hacia atrás).
    loop {
        let mut cursor = u32::from(start) + 5;
        if cursor >= 0x190 {
            break;
        }
        let mut left = 6u16;
        let mut blocked = false;
        while left > 0 {
            if days[cursor as usize] == 0 {
                blocked = true;
                break;
            }
            cursor -= 1;
            left -= 1;
        }
        if !blocked {
            break;
        }
        start = start.wrapping_add(left);
    }
    m.w16(new + 0xe, start << 8);
    Ok(())
}
