//! `op_tavern_interaction` (0x0053C2C0): las diez órdenes de la taberna (reserva de huéspedes, avisos y cancelaciones).
//! Traducción 1:1 sobre la memoria del original ([`Mem`]); la creación de tareas (`0x4D8CF0`) pasa por el anfitrión.
use crate::flatmem::Mem;

pub(crate) const TOWN_COUNT: u32 = 0x6DE4B0;
pub(crate) const TOWNS: u32 = 0x6DE508;
pub(crate) const TOWN_STRIDE: u32 = 0x9F8;
pub(crate) const MERCHANT_COUNT: u32 = 0x6DE4AA;
pub(crate) const MERCHANT_OBJ: u32 = 0x6DE4A0;
pub(crate) const MERCHANT_STRIDE: u32 = 0x650;
pub(crate) const TICK: u32 = 0x6DE4B4;
pub(crate) const PLAYER: u32 = 0x6DE52D;
pub(crate) const SCHED: u32 = 0x6DD73C;
pub(crate) const LETTERS: u32 = 0x6DD730;
pub(crate) const VESSELS: u32 = 0x6DD7A0;
pub(crate) const VESSEL_COUNT: u32 = 0x6DD892;

/// Servicios que el original resuelve en otras funciones.
pub trait TavernHost {
    /// `scheduler_add_task` (0x004D8CF0): dirección del registro nuevo.
    fn add_task(&mut self, due: u32) -> u32;
}

/// `0x005303C0`: comerciante número `i`.
pub(crate) fn merchant(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(MERCHANT_OBJ + 0x78).wrapping_add(i.wrapping_mul(MERCHANT_STRIDE))
}

/// `scheduler_get_task` (0x004D8DB0): 0 si el índice supera la capacidad.
pub(crate) fn get_task(m: &mut dyn Mem, index: u32) -> u32 {
    if index > u32::from(m.r16(SCHED + 0xC)) {
        return 0;
    }
    m.r32(SCHED).wrapping_add(index.wrapping_mul(24))
}

/// `0x004D7900(carta, clave)`: siguiente carta tipo 0x71 con esa clave cuyo objeto aún no ha vencido; `0xFFFF` si no hay.
pub fn find_letter(m: &mut dyn Mem, start: u32, key: u32) -> u32 {
    let count = u32::from(m.r16(LETTERS + 6));
    let mut a = start & 0xffff;
    if a >= count {
        return 0xffff;
    }
    let table = m.r32(LETTERS);
    loop {
        let e = table.wrapping_add(a << 4);
        if m.r8(e + 4) == 0x71 && (key & 0xffff) == u32::from(m.r8(e + 5)) {
            let obj = m.r32(e + 8);
            if m.r32(obj + 4) > m.r32(TICK) {
                return a;
            }
        }
        a = u32::from(m.r16(e + 6));
        if a >= count {
            return 0xffff;
        }
    }
}

/// Alta de la tarea de aviso (tipo 5) que cierra las órdenes 3, 6, 7 y 8.
fn queue_notice(m: &mut dyn Mem, host: &mut dyn TavernHost, op: u32, who: u32, town_field: u32, town: u32) {
    let boss = merchant(m, who);
    let tick = m.r32(TICK);
    let task = host.add_task(tick.wrapping_add(0x200) | 0x80);
    m.w16(task + 6, 5);
    let (a, b) = (m.r8(op + 8), m.r8(op + 0xC));
    m.w8(task + 8, a);
    m.w8(task + 9, b);
    let home = m.r8(boss + 0x19);
    m.w8(task + 0xB, home);
    let now = m.r32(TICK);
    m.w32(task + 0xC, now);
    m.w8(task + 0x10, 0);
    m.w8(task + 0x11, 0);
    let now = m.r32(TICK);
    m.w32(town + town_field, now);
}

/// `op`: dirección de la orden (`+4`, `+6`, `+8`, `+0xC` ciudad, `+0x10` tipo).
pub fn run(m: &mut dyn Mem, host: &mut dyn TavernHost, op: u32) {
    let town_index = m.r32(op + 0xC);
    if town_index >= m.r32(TOWN_COUNT) {
        return;
    }
    let kind = m.r32(op + 0x10).wrapping_sub(1);
    let town = m.r32(TOWNS).wrapping_add(town_index.wrapping_mul(TOWN_STRIDE));
    if kind > 9 {
        return;
    }
    let who = m.r32(op + 8);
    let mc = u32::from(m.r16(MERCHANT_COUNT));
    match kind {
        5 => {
            if who >= mc {
                m.w8(town + 0x83D, 0xff);
                return;
            }
            if m.r8(town + 0x83D) != 0xff {
                return;
            }
            m.w8(town + 0x83D, who as u8);
            if m.r8(PLAYER) as u32 == who {
                return;
            }
            let t = m.r32(TOWNS).wrapping_add(town_index.wrapping_mul(TOWN_STRIDE));
            if u32::from(m.r8(t + 0x6F1)) == who {
                return;
            }
            if m.r32(town + 0x2C8) & 0xE10 != 0 {
                return;
            }
            if m.r32(op + 4) & 0x3FF >= 0x66 {
                return;
            }
            if m.r32(town + 0x9EC) >= m.r32(TICK).wrapping_add(0x200) {
                return;
            }
            queue_notice(m, host, op, who, 0x9EC, town);
        }
        0 | 4 => {
            let slot = town + if kind == 0 { 0x83E } else { 0x83F };
            let cur = m.r8(slot);
            if who >= mc {
                if cur != 0 {
                    m.w8(slot, 0xff);
                }
            } else if cur == 0xff {
                m.w8(slot, who as u8);
            }
        }
        6 => {
            if who >= mc {
                m.w8(town + 0x83C, 0xff);
                return;
            }
            if m.r8(town + 0x83C) != 0xff {
                return;
            }
            let v = m.r8(op + 8);
            m.w8(town + 0x83C, v);
            if m.r32(op + 4) & 0x3FF >= 0x66 {
                return;
            }
            if m.r32(town + 0x9F4) >= m.r32(TICK).wrapping_add(0x200) {
                return;
            }
            queue_notice(m, host, op, who, 0x9F4, town);
        }
        2 | 7 => {
            let idx = m.r32(op + 4) & 0xffff;
            if idx >= u32::from(m.r16(VESSEL_COUNT)) {
                return;
            }
            let e = m.r32(VESSELS).wrapping_add(idx << 4);
            if who >= mc {
                m.w8(e + 0xF, 0xff);
                return;
            }
            if m.r8(e + 0xF) != 0xff {
                return;
            }
            m.w8(e + 0xF, who as u8);
            if m.r32(op + 0x10) != 8 {
                return;
            }
            if m.r16(op + 6) & 0x3FF >= 0x66 {
                return;
            }
            if m.r32(town + 0x9F0) >= m.r32(TICK).wrapping_add(0x200) {
                return;
            }
            queue_notice(m, host, op, who, 0x9F0, town);
        }
        9 => cancel_guest(m, op, town, who, mc),
        8 => {
            let idx = m.r32(op + 4) & 0xffff;
            let slot = u32::from(m.r16(op + 6));
            if u32::from(m.r16(SCHED + 0xC)) <= idx {
                return;
            }
            let task = get_task(m, idx);
            if m.r16(task + 6) != 0x1B {
                return;
            }
            if u32::from(m.r16(task + 0x12)) <= slot {
                return;
            }
            let array = m.r32(task + 0xC);
            let cell = array.wrapping_add(slot << 2);
            if who >= mc {
                m.w32(cell, 0xffff_ffff);
            } else if m.r32(cell) >= mc {
                m.w32(cell, who);
            }
        }
        _ => {}
    }
}

/// Orden 10: libera todas las reservas del comerciante en la ciudad, en los barcos y en las cartas pendientes.
fn cancel_guest(m: &mut dyn Mem, op: u32, town: u32, _who: u32, mc: u32) {
    for field in [0x83D, 0x83E, 0x83F, 0x83C] {
        if u32::from(m.r8(town + field)) == m.r32(op + 8) {
            m.w8(town + field, 0xff);
        }
    }
    let mut i = u32::from(m.r16(town + 0x82E));
    while i < u32::from(m.r16(VESSEL_COUNT)) {
        let e = m.r32(VESSELS).wrapping_add(i << 4);
        if u32::from(m.r8(e + 0xF)) == m.r32(op + 8) {
            m.w8(e + 0xF, 0xff);
        }
        i = u32::from(m.r16(e));
    }
    let who = m.r32(op + 8);
    if who >= u32::from(m.r16(MERCHANT_COUNT)) {
        return;
    }
    let boss = merchant(m, who);
    let first = u32::from(m.r16(boss + 0xA));
    let key = u32::from(m.r16(op + 0xC));
    let mut letter = find_letter(m, first, key) & 0xffff;
    let _ = mc;
    while u32::from(m.r16(MERCHANT_COUNT)) > letter {
        let count = u32::from(m.r16(LETTERS + 6));
        let entry = if letter > count {
            0
        } else {
            m.r32(LETTERS).wrapping_add(letter << 4)
        };
        let obj = m.r32(entry + 8);
        let next = u32::from(m.r16(entry + 6));
        let task_index = u32::from(m.r16(obj + 8));
        if task_index < u32::from(m.r16(SCHED + 0xC)) {
            let task = get_task(m, task_index);
            if m.r16(task + 6) == 0x1B {
                let slot = u32::from(m.r8(obj + 0xC));
                if u32::from(m.r16(task + 0x12)) > slot {
                    let array = m.r32(task + 0xC);
                    let cell = array.wrapping_add(slot << 2);
                    if m.r32(cell) == m.r32(op + 8) {
                        m.w32(cell, 0xffff);
                    }
                }
            }
        }
        let key = u32::from(m.r16(op + 0xC));
        letter = find_letter(m, next, key) & 0xffff;
    }
}
