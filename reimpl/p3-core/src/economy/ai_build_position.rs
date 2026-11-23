//! Posición inicial cercana a edificios de una oficina (0x00500F50).
use crate::{buildings::Site, town::Town};

pub fn near(
    town: &Town,
    site: &Site,
    office: &[u8],
    row: &mut u8,
    col: &mut u8,
    mut skip: i32,
    kind: u8,
    tick: u32,
) -> Result<(), String> {
    if office.len() < 0x2d2 {
        return Err("oficina de construcción truncada".into());
    }
    let count = usize::from(town.u16(0x776));
    if site.records.len() != count * 8 {
        return Err("registros de construcción incompletos".into());
    }
    let stride = town.i32(0x7ac);
    if stride <= 0 {
        return Err("plano de ciudad inválido".into());
    }
    let mut output = (*row, *col);
    let cell = |r: u8, c: u8| -> Result<bool, String> {
        let index = (stride as usize)
            .checked_mul(usize::from(r))
            .and_then(|v| v.checked_add(usize::from(c)))
            .ok_or("casilla fuera del plano")?;
        Ok(site.cells.get(index).ok_or("casilla fuera del plano")? & 0x30 == 0x30)
    };
    let word = |at: usize| u16::from_le_bytes([office[at], office[at + 1]]);
    let mut at = usize::from(word(if kind > 20 { 0x2d0 } else { 0x2ce }));
    let mut seen = vec![false; count];
    let mut good = None;
    let mut fallback = None;
    while at < count {
        if seen[at] {
            return Err("cadena de edificios cíclica".into());
        }
        seen[at] = true;
        let rec = &site.records[at * 8..at * 8 + 8];
        let x = rec[0];
        let y = rec[1];
        if rec[3] == kind {
            if kind > 20 {
                output = (x, y);
                let old = skip;
                skip = skip.wrapping_sub(1);
                if old < 0 {
                    *row = output.0;
                    *col = output.1;
                    return Ok(());
                }
            } else {
                let mut found = false;
                for k in 0..4u32 {
                    let d = (u32::from(x).wrapping_add(tick).wrapping_add(k)) & 3;
                    let (r, c) = match d {
                        0 => (x.wrapping_add(2), y.wrapping_add(4)),
                        1 => (x.wrapping_add(y & 1).wrapping_add(2), y.wrapping_sub(5)),
                        2 => (x.wrapping_sub(2), y.wrapping_sub(4)),
                        _ => (x.wrapping_add(y & 1).wrapping_sub(3), y.wrapping_add(5)),
                    };
                    if cell(r, c)? {
                        found = true;
                        if skip < 1 {
                            output = (x, y)
                        } else {
                            good = Some((x, y))
                        }
                        skip = skip.wrapping_sub(1);
                        break;
                    }
                }
                if !found {
                    fallback = Some((x, y))
                }
            }
        }
        at = usize::from(u16::from_le_bytes([rec[4], rec[5]]));
    }
    if kind > 20 {
        *row = output.0;
        *col = output.1;
        return Ok(());
    }
    if good.is_none() {
        let mut at = usize::from(town.u16(0x768));
        let mut seen = vec![false; count];
        while at < count {
            if seen[at] {
                return Err("defensas cíclicas".into());
            }
            seen[at] = true;
            let rec = &site.records[at * 8..at * 8 + 8];
            let (x, y) = (rec[0], rec[1]);
            let candidates = match rec[3] {
                0x35 | 0x38 => Some([
                    (x.wrapping_add(3), y),
                    (x.wrapping_sub(1), y.wrapping_sub(8)),
                    (x.wrapping_add(y & 1).wrapping_sub(2), y.wrapping_add(9)),
                    (x.wrapping_add(y & 1).wrapping_sub(6), y.wrapping_add(1)),
                ]),
                0x34 | 0x37 => Some([
                    (x.wrapping_add(5), y),
                    (x.wrapping_add(1), y.wrapping_sub(8)),
                    (x.wrapping_add(y & 1).wrapping_sub(4), y.wrapping_add(1)),
                    (x.wrapping_add(y & 1), y.wrapping_add(9)),
                ]),
                _ => None,
            };
            if let Some(candidates) = candidates {
                for (r, c) in candidates {
                    if cell(r, c)? {
                        *row = r;
                        *col = c;
                        return Ok(());
                    }
                }
            }
            at = usize::from(u16::from_le_bytes([rec[4], rec[5]]));
        }
    }
    if let Some((r, c)) = good.or(fallback) {
        output = (r, c)
    }
    *row = output.0;
    *col = output.1;
    Ok(())
}
