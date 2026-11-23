//! Tramo de la tabla de tareas de `Convoy::advance` (0x00502285..0x00502331): al terminar un barco, si la tarea del convoy
//! (`convoy+2`) es de tipo 0x34, con `byte+0x15 >= 7` y `word+0x16` igual al convoy del barco, se antepone a su lista
//! (`tarea+8`) un registro de 8 bytes `{siguiente, global 0x672856, dueño, word ship+0x15E}`. Fixture: `oracle/x86_convoy_report.py`.

use super::r16;

/// Tamaño de un elemento de la tabla de tareas 0x6DD73C.
pub const TASK_SIZE: usize = 24;

/// Reserva (`operator new(8)` + 0x4F3980 + 0x4F3AD0) del registro: devuelve su puntero.
pub trait ReportHost {
    fn new_record(&mut self, next: u32, global: u8, owner: u8, word: u16) -> u32;
}

/// `tasks`: elementos de 24 bytes; `count`: word 0x6DD748; `task`: `convoy+2`; `convoy`: word `ship+8`.
/// Devuelve `true` si se antepuso un registro.
pub fn report_to_task(
    tasks: &mut [u8],
    count: u16,
    task: u16,
    convoy: u16,
    owner: u8,
    global: u8,
    word: u16,
    host: &mut dyn ReportHost,
) -> bool {
    if task >= count {
        return false;
    }
    let b = task as usize * TASK_SIZE;
    if r16(tasks, b + 6) != 0x34 || tasks[b + 0x15] < 7 || r16(tasks, b + 0x16) != convoy {
        return false;
    }
    let next = u32::from_le_bytes(tasks[b + 8..b + 12].try_into().unwrap());
    let record = host.new_record(next, global, owner, word);
    super::w32(tasks, b + 8, record as i32);
    true
}
