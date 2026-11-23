//! Operaciones de solicitantes de préstamo de una ciudad (`town_build_loan_applicant_operations`, `0x0051D160`).
//! Por cada una de las tres ranuras (`Town+0x9D8`, seis bytes: tres de nombre, semanas en `+3`, importe u16 en `+4`) con semanas e importe no nulos
//! rellena un registro de 0x10 bytes (`terms` en `+0` con `loan_terms::compute`) y marca `changed` si algo difiere del registro anterior.
use crate::{economy::loan_terms, rng::CrtRand, town::Town};

/// `recs` (al menos 0x30 bytes) se escribe; `global_14` es el byte `[0x6DFC14]`. Devuelve `(número de registros, ¿hubo cambios?)`.
pub fn build(town: &Town, recs: &mut [u8], global_14: u8, rng: &mut CrtRand) -> (u32, bool) {
    let mut changed = false;
    let mut count = 0usize;
    for i in 0..3usize {
        let slot = 0x9d8 + 6 * i;
        let weeks = town.u8(slot + 3);
        let amount = town.u16(slot + 4);
        if amount == 0 || weeks == 0 {
            continue;
        }
        let r = &mut recs[count * 0x10..count * 0x10 + 0x10];
        r[0xb] = 0;
        r[0xc] = global_14;
        r[0xe] = town.u8(0x2c0);
        r[0xf] = ((i as u8) << 4).wrapping_add(0x32);
        let old_amount = u16::from(r[8]) | u16::from(r[9]) << 8;
        if old_amount != amount {
            r[8..10].copy_from_slice(&amount.to_le_bytes());
            changed = true;
        }
        if r[0xa] != weeks {
            r[0xa] = weeks;
            changed = true;
        }
        let old_total = [r[0], r[1], r[2], r[3]];
        loan_terms::compute(r, rng);
        if r[0..4] != old_total {
            changed = true;
        }
        for (dst, src) in [(5usize, 0usize), (6, 1), (7, 2)] {
            let v = town.u8(slot + src);
            if r[dst] != v {
                r[dst] = v;
                changed = true;
            }
        }
        r[0xd] = 0xff;
        count += 1;
    }
    (count as u32, changed)
}
