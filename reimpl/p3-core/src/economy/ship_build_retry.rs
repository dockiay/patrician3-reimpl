//! Reintento de pedido de astillero, tarea 0x0B (0x004E37F4).
use crate::scheduler::TaskRecord;

/// Reproduce la busqueda circular y el plazo. El constructor se ejecuta de
/// inmediato por el anfitrion; true significa que el pedido fue aceptado.
pub fn step<F>(task: &mut TaskRecord, merchant_count: u16, cash: i32, home: u8, towns: u32, mut build: F) -> bool
where
    F: FnMut(u32, u8, u8) -> bool,
{
    let word = |t: &TaskRecord, o| u16::from_le_bytes([t.u8_at(o), t.u8_at(o + 1)]);
    let owner = word(task, 8);
    if owner >= merchant_count || cash < 0x80e8 {
        return false;
    }
    task.due = task.due.wrapping_add(u32::from(word(task, 0x12)) + 1);
    let mut city = u32::from(home);
    if city < towns {
        loop {
            if task.u32_at(0xc) & (1u32 << (city & 31)) != 0 && build(city, owner as u8, task.u8_at(0xa)) {
                return false;
            }
            city += 1;
            if city >= towns {
                city = 0
            }
            if city == u32::from(home) {
                break;
            }
        }
    }
    let left = word(task, 0x10);
    let next = left.wrapping_sub(1);
    task.set_u8_at(0x10, next as u8);
    task.set_u8_at(0x11, (next >> 8) as u8);
    (left as i16) > 0
}
