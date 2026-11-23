//! C08: respuesta a una carta de guion (`op_answer_script_letter`, `0x0053EC30`, operación 0x74), sobre la memoria
//! del original ([`Mem`]). `+4` tarea, `+8` variable, `+0xC` valor y `+0x10` carta.
use crate::flatmem::Mem;

const SCHED: u32 = 0x006d_d73c;
const LETTERS: u32 = 0x006d_d730;

/// Escribe la respuesta en la variable de la tarea 0x1B y marca la carta 0x3C..0x40 como contestada (palabra `+4`
/// de su objeto a cero). El original compara con `<=` en los dos límites y escribiría una posición más allá de la
/// lista o de la tabla de cartas: esos casos devuelven `Err` sin cambios.
pub fn answer_script_letter(m: &mut dyn Mem, op: u32) -> Result<(), &'static str> {
    let task_index = m.r32(op + 4);
    let capacity = u32::from(m.r16(SCHED + 0xc));
    let var = m.r32(op + 8);
    let letter = m.r32(op + 0x10);
    let slots = u32::from(m.r16(LETTERS + 6));
    let task = (task_index < capacity).then(|| m.r32(SCHED).wrapping_add(task_index * 0x18));
    let write_var = match task {
        Some(t) if m.r16(t + 6) == 0x1b => {
            let count = u32::from(m.r16(t + 0x12));
            if var == count {
                return Err("variable de guion una posición tras la lista");
            }
            (var < count).then_some(t)
        }
        _ => None,
    };
    if letter == slots {
        return Err("carta de guion una posición tras la tabla");
    }
    if let Some(t) = write_var {
        let list = m.r32(t + 0xc);
        let value = m.r32(op + 0xc);
        m.w32(list.wrapping_add(var * 4), value);
    }
    if letter < slots {
        let entry = m.r32(LETTERS).wrapping_add(letter << 4);
        if entry != 0 && (0x3c..=0x40).contains(&m.r8(entry + 4)) {
            let object = m.r32(entry + 8);
            m.w32(object + 4, 0);
        }
    }
    Ok(())
}
