//! C08: operaciones del jugador sobre el buzón (`0x006DD730`) en su forma serializada de S05.
//!
//! - `op_mark_letter_read` (`0x004D69D0`, operación 0x6E): quita la marca de no leída (bit 7 de `+1`).
//! - `op_delete_letter` (`0x004D6A50`, operación 0x6F): desengancha la carta de la cadena del comerciante (o de
//!   la global si el comerciante es negativo), la devuelve a la lista libre y la vacía (`0x0051AAB0`).
//! - `op_repair_letter_chains` (`0x004D7960`, operación 0x71 con `+8 != 0`): corta las cadenas que revisitan una
//!   ranura y ajusta la cabeza libre.
//!
//! El buzón serializado tiene una cabecera de 12 bytes (`+4` ranuras, `+6` cabeza libre, `+8` cabeza global) y
//! una entrada por ranura; las cartas con texto llevan el bloque a continuación de sus 16 bytes. El enlace
//! siguiente es la palabra `+6` de cada entrada; la cabeza de cada comerciante es su palabra `+0xA`.
use p3_core::ai::reputation::Merchant;
use p3_save::tail::MessagePool;

struct Layout {
    slots: u16,
    starts: Vec<usize>,
    ends: Vec<usize>,
}

fn layout(pool: &[u8]) -> Result<Layout, String> {
    let parsed = MessagePool::parse(pool).map_err(|e| format!("buzón: {e:?}"))?;
    Ok(Layout { slots: parsed.slots, starts: parsed.letters.iter().map(|l| l.range.start).collect(), ends: parsed.letters.iter().map(|l| l.range.end).collect() })
}

fn word(pool: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([pool[at], pool[at + 1]])
}

fn set_word(pool: &mut [u8], at: usize, v: u16) {
    pool[at..at + 2].copy_from_slice(&v.to_le_bytes());
}

/// Operación 0x6E: `merchant` es `+4` (con signo) y `letter`, `+8`. Recorre como mucho tantas cartas como ranuras.
pub fn mark_read(pool: &mut [u8], merchants: &[Merchant], merchant_count: u16, merchant: i32, letter: u32) -> Result<bool, String> {
    let l = layout(pool)?;
    if letter >= u32::from(l.slots) || merchant < 0 || merchant >= i32::from(merchant_count) {
        return Ok(false);
    }
    let mut at = merchants.get(merchant as usize).ok_or("comerciante del buzón ausente")?.u16(0xa);
    let mut budget = l.slots;
    while at < l.slots {
        if budget == 0 {
            return Ok(false);
        }
        budget -= 1;
        if u32::from(at) == letter {
            pool[l.starts[usize::from(at)] + 1] &= 0x7f;
            return Ok(true);
        }
        at = word(pool, l.starts[usize::from(at)] + 6);
    }
    Ok(false)
}

/// Operación 0x6F: `merchant` (`+4`, con signo; negativo es la cadena global) y `letter` (`+8`).
/// Una cadena cíclica dejaría al original en un bucle infinito: se rechaza sin cambios.
pub fn delete(pool: &mut Vec<u8>, merchants: &mut [Merchant], merchant_count: u16, merchant: i32, letter: u32) -> Result<bool, String> {
    let l = layout(pool)?;
    if letter >= u32::from(l.slots) || merchant >= i32::from(merchant_count) {
        return Ok(false);
    }
    if merchant >= 0 && merchants.len() <= merchant as usize {
        return Err("comerciante del buzón ausente".into());
    }
    let head = if merchant < 0 { word(pool, 8) } else { merchants[merchant as usize].u16(0xa) };
    // Enlace que apunta a la carta: la cabeza (`None`) o el `+6` de la entrada anterior.
    let mut link: Option<usize> = None;
    let mut at = head;
    let mut steps = 0usize;
    while at < l.slots {
        if u32::from(at) == letter {
            let entry = l.starts[usize::from(at)];
            let next = word(pool, entry + 6);
            match link {
                None if merchant < 0 => set_word(pool, 8, next),
                None => merchants[merchant as usize].set_u16(0xa, next),
                Some(prev) => set_word(pool, prev, next),
            }
            let free = word(pool, 6);
            let mut blank = [0u8; 16];
            blank[2] = 0xff;
            blank[3] = 0xff;
            blank[4] = 0xff;
            blank[6..8].copy_from_slice(&free.to_le_bytes());
            let end = l.ends[usize::from(at)];
            pool.splice(entry..end, blank);
            set_word(pool, 6, at);
            return Ok(true);
        }
        steps += 1;
        if steps > usize::from(l.slots) {
            return Err("cadena de cartas cíclica".into());
        }
        let entry = l.starts[usize::from(at)];
        link = Some(entry + 6);
        at = word(pool, entry + 6);
    }
    Ok(false)
}

/// `op_answer_letter_with_payment` (`0x004D74C0`, operación 0x55): cobra `payment` al comerciante y, si la carta
/// `letter` de su cadena es de tipo 0x45/0x46, la marca como contestada (tipo 0x46, `+0xA |= bits`, fecha de hoy y
/// no leída) y la pasa al principio de la cadena. Recorre como mucho tantas cartas como ranuras.
#[allow(clippy::too_many_arguments)]
pub fn answer_with_payment(pool: &mut [u8], merchants: &mut [Merchant], merchant_count: u16, merchant: u32, letter: u32, payment: i32, bits: u16, date: (u8, u8, u16)) -> Result<bool, String> {
    let l = layout(pool)?;
    if merchant >= u32::from(merchant_count) {
        return Ok(false);
    }
    let m = merchants.get_mut(merchant as usize).ok_or("comerciante del buzón ausente")?;
    m.set_i32(0, m.i32(0).wrapping_sub(payment));
    let mut link: Option<usize> = None;
    let mut at = m.u16(0xa);
    let mut budget = l.slots;
    while at < l.slots {
        if budget == 0 {
            return Ok(false);
        }
        budget -= 1;
        let entry = l.starts[usize::from(at)];
        if u32::from(at) == letter {
            if !matches!(pool[entry + 4], 0x45 | 0x46) {
                return Ok(false);
            }
            pool[entry + 4] = 0x46;
            let marked = word(pool, entry + 0xa) | bits;
            set_word(pool, entry + 0xa, marked);
            pool[entry] = date.0;
            pool[entry + 1] = date.1 | 0x80;
            pool[entry + 2..entry + 4].copy_from_slice(&date.2.to_le_bytes());
            let next = word(pool, entry + 6);
            match link {
                None => m.set_u16(0xa, next),
                Some(p) => set_word(pool, p, next),
            }
            let head = m.u16(0xa);
            set_word(pool, entry + 6, head);
            m.set_u16(0xa, at);
            return Ok(true);
        }
        link = Some(entry + 6);
        at = word(pool, entry + 6);
    }
    Ok(false)
}

/// `op_repair_letter_chains`: marca las ranuras visitadas; en las cadenas de comerciantes y en la global un
/// enlace a una ranura ya vista pasa a `0xFFFF`, y en la libre al número de ranuras. La cabeza libre se limita
/// al número de ranuras.
pub fn repair_chains(pool: &mut [u8], merchants: &mut [Merchant], merchant_count: u16) -> Result<(), String> {
    let l = layout(pool)?;
    let slots = l.slots;
    let mut seen = vec![false; usize::from(slots)];
    if word(pool, 6) > slots {
        set_word(pool, 6, slots);
    }
    if merchants.len() < usize::from(merchant_count) {
        return Err("comerciantes del buzón incompletos".into());
    }
    for m in merchants.iter_mut().take(usize::from(merchant_count)) {
        let mut at = m.u16(0xa);
        let mut link: Option<usize> = None;
        while at < slots {
            if seen[usize::from(at)] {
                match link {
                    None => m.set_u16(0xa, 0xffff),
                    Some(p) => set_word(pool, p, 0xffff),
                }
                break;
            }
            seen[usize::from(at)] = true;
            let p = l.starts[usize::from(at)] + 6;
            link = Some(p);
            at = word(pool, p);
        }
    }
    for (head, cut) in [(8usize, 0xffffu16), (6usize, slots)] {
        let mut link = head;
        let mut at = word(pool, link);
        while at < slots {
            if seen[usize::from(at)] {
                set_word(pool, link, cut);
                break;
            }
            seen[usize::from(at)] = true;
            link = l.starts[usize::from(at)] + 6;
            at = word(pool, link);
        }
    }
    Ok(())
}
