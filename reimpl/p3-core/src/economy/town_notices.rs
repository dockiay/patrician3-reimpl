//! Actualización del tablón del ayuntamiento (tarea programada 0x22).
use crate::scheduler::TaskRecord;
use crate::town::Town;

const GOODS: [usize; 8] = [0, 3, 2, 11, 14, 1, 7, 8];
const BARRELS: [bool; 8] = [false, true, false, false, false, false, true, true];
const FACTORS: [f32; 8] = [0.055000003, 0.174, 0.22005, 0.0275, 0.44000003, 0.47855, 1.1, 1.034];

/// Comprueba antes de alterar el estado: un paso múltiplo de 53 puede colgar al original.
pub fn refresh(
    task: &mut TaskRecord,
    towns: &mut [Town],
    board: &mut [u8; 84],
    tick: u32,
    day_of_year: u16,
) -> Result<(), String> {
    if towns.is_empty() || towns.len() > 53 {
        return Err("número de ciudades incompatible con tablón".into());
    }
    let mut cursor = u32::from_le_bytes(task.payload[0..4].try_into().unwrap());
    let step = u32::from_le_bytes(task.payload[4..8].try_into().unwrap());
    if step % 53 == 0 {
        return Err("paso de tablón sin recorrido válido".into());
    }
    for entry in board.chunks_exact_mut(12) {
        entry[0] = 0xff;
        entry[1] = 0xff;
    }
    let mut count = 0usize;
    for round in 0..23u32 {
        loop {
            cursor = cursor.wrapping_add(step) % 53;
            if (cursor as usize) < towns.len() {
                break;
            }
        }
        let town = &towns[cursor as usize];
        let mask = town.u8(0x2c4);
        for n in 0..8 {
            let good = GOODS[n];
            let barrel = BARRELS[n];
            if mask & (1 << n) == 0 || town.i32(4 + good * 4) != 0 || town.i32(0xc4 + good * 4) != 0 {
                continue;
            }
            let qualifying = town.i32(0x4f0 + good * 4);
            if qualifying <= if barrel { 10000 } else { 20000 } {
                continue;
            }
            let threshold = town.i32(0x4f0 + good * 16);
            let at = count * 12;
            board[at] = cursor as u8;
            board[at + 1] = good as u8;
            board[at + 2] = u8::from((town.u32(0x310) & 3) < ((u32::from(day_of_year) + cursor) & 3));
            let scale =
                180 + (town.u32(0x314) & 15) + (town.u32(0x2d4) & 15) + (good as u32 & 3) + (town.u32(0x310) & 7);
            let base = scale * if barrel { 200 } else { 2000 };
            let mut amount = ((base as f64) * (FACTORS[n] as f64) * (0.01f32 as f64)) as u32;
            amount = amount.max(1);
            let price = if count >= 5 {
                amount = amount.wrapping_mul(170) / 220;
                threshold.wrapping_mul(4)
            } else {
                threshold.wrapping_add(threshold >> 1)
            };
            board[at + 3] = (price / if barrel { 200 } else { 2000 }).min(250) as u8;
            let deadline = tick.wrapping_add(((good as u32 + cursor) & 3).wrapping_add(round & 3).wrapping_add(9) << 8);
            board[at + 4..at + 8].copy_from_slice(&deadline.to_le_bytes());
            board[at + 8..at + 12].copy_from_slice(&amount.to_le_bytes());
            count += 1;
            break;
        }
        if count == 7 {
            break;
        }
    }
    for town in towns.iter_mut() {
        let mut mask = 0u16;
        for (n, good) in GOODS.iter().enumerate() {
            if town.i32(4 + good * 4) <= 0 {
                mask |= 1 << n;
            }
        }
        town.set_u16(0x2c4, mask);
    }
    task.payload[0..4].copy_from_slice(&cursor.to_le_bytes());
    task.due = task.due.wrapping_add(((towns[0].u32(0x2d4) & 3) + 7) << 8);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_step_does_not_modify_state() {
        let mut task = TaskRecord::default();
        let before = task;
        let mut towns = vec![Town::default()];
        let old = towns.clone();
        let mut board = [42; 84];
        assert!(refresh(&mut task, &mut towns, &mut board, 0, 0).is_err());
        assert_eq!(task, before);
        assert_eq!(towns, old);
        assert_eq!(board, [42; 84]);
    }
}
