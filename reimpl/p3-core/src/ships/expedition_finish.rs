//! Regreso físico del convoy de expedición: 0x00502B20.
//!
//! El adaptador prepara una copia de `SparseMem` y del anfitrión. Cada llamada
//! externa es obligatoria y falible; al fallar no se publica ninguna mutación.
//! El llamante proporciona las imágenes x86 de los gestores y traduce sus
//! punteros al confirmar. Las dependencias no implementadas deben devolver Err.
use crate::flatmem::{Mem, SparseMem};

const SHIPS: u32 = 0x6dd7a4;
const CAPTAINS: u32 = 0x6dd7a0;
const SHIP_COUNT: u32 = 0x6dd894;
const CAP_COUNT: u32 = 0x6dd892;
const MERCHANTS: u32 = 0x6de518;
const MERCHANT_COUNT: u32 = 0x6de4aa;
const TOWN_COUNT: u32 = 0x6de4b0;
const TICK: u32 = 0x6de4b4;
const RENOWN_DELTA: u32 = 0x672dd0;

/// Cada efecto representa una llamada original y debe aplicarse sobre la misma
/// copia de memoria; los campos auxiliares de presentación los posee el host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Effect {
    LostLetter { owner: u8, record: [u8; 16] },
    LostWindow { owner: u8, town: u8, count: u32 },
    ReleaseConvoy { index: u16 },
    PickLeader { convoy: u32 },
    FreeCaptain { index: u16 },
    PlanRoute { ship: u16, destination: u8 },
    RefreshConvoy { convoy: u32 },
    InsertOrder { ship: u16 },
    ReturnWindow { owner: u8, ship: u16 },
}
/// `Clone` debe aislar también los efectos externos hasta confirmar la copia.
pub trait FinishHost: Clone {
    fn call(&mut self, m: &mut dyn Mem, e: Effect) -> Result<(), String>;
}

/// Clona toda la transacción; rechaza entradas inválidas y errores de callbacks.
/// El destructor o el planificador pueden tocar además estructuras del host.
pub fn prepare<H: FinishHost>(before: &SparseMem, host: &H, convoy: u32, start: u32) -> Result<(SparseMem, H), String> {
    let mut memory = before.clone();
    let mut after = host.clone();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run(&mut memory, &mut after, convoy, start)
    }));
    match outcome {
        Ok(Ok(())) => Ok((memory, after)),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("memoria incompleta al finalizar expedición".into()),
    }
}
fn run(m: &mut dyn Mem, h: &mut dyn FinishHostDyn, cv: u32, start: u32) -> Result<(), String> {
    let ships = m.r32(SHIPS);
    let captains = m.r32(CAPTAINS);
    let count = m.r16(SHIP_COUNT);
    let cap_count = m.r16(CAP_COUNT);
    let owner = m.r8(cv);
    let mer_count = m.r16(MERCHANT_COUNT);
    if u16::from(owner) >= mer_count {
        return Err("convoy de expedición sin comerciante válido".into());
    }
    let mer = m
        .r32(MERCHANTS)
        .checked_add(u32::from(owner) * 0x650)
        .ok_or("comerciante fuera de memoria")?;
    let ship_ptr = |id: u16| {
        ships
            .checked_add(u32::from(id) * 0x180)
            .ok_or_else(|| "barco fuera de memoria".to_owned())
    };
    let mut cursor = m.r16(cv + 0xa);
    if cursor >= count {
        return Err("convoy sin miembros: índice de baja original indeterminado".into());
    }
    let mut previous: Option<u32> = None;
    let mut last_convoy = 0xffffu16;
    let mut saved_captain = 0xffffu16;
    let mut had_before = false;
    let mut letters = 0u32;
    let mut seen = vec![false; usize::from(count)];
    let now = m.r32(TICK);
    let human = m.r16(mer + 8) == 0;
    let town = m.r8(mer + 0x19);
    while cursor < count {
        let idx = usize::from(cursor);
        if seen[idx] {
            return Err("cadena de convoy cíclica".into());
        }
        seen[idx] = true;
        let sp = ship_ptr(cursor)?;
        last_convoy = m.r16(sp + 8);
        let next = m.r16(sp + 6);
        if (m.r32(sp + 0x18) as i32) > 0 {
            m.w16(sp + 0x1e, 0x76);
            m.w16(sp + 0x22, 0x640);
            m.w16(sp + 0x134, 0xf);
            m.w8(sp + 0x15c, 0);
            previous = Some(sp);
            had_before = true;
            cursor = next;
            continue;
        }
        if cursor == m.r16(cv + 0x10) {
            m.w16(cv + 0x10, 0xffff);
            let cap = m.r16(sp + 0x42);
            if cap < cap_count {
                saved_captain = cap;
                let cp = captains
                    .checked_add(u32::from(cap) * 16)
                    .ok_or("capitán fuera de memoria")?;
                let old = m.r16(cp);
                m.w16(sp + 0x42, old);
                m.w16(cp, 0xffff)
            }
        }
        if let Some(p) = previous {
            m.w16(p + 6, next)
        } else {
            m.w16(cv + 0xa, next)
        }
        for off in [6, 8, 0x50, 0x52] {
            m.w16(sp + off, 0xffff)
        }
        m.w16(sp + 0x134, 0x11);
        let t = if had_before {
            let n = u32::from(cursor).wrapping_add(10).wrapping_mul(now.wrapping_sub(start));
            start.wrapping_add(n / (u32::from(count) + 10))
        } else {
            had_before = true;
            now.wrapping_sub(0x100)
        };
        if human {
            let mut record = [0u8; 16];
            record[1] = 0x80;
            record[2..4].copy_from_slice(&0xffffu16.to_le_bytes());
            record[4] = 0x34;
            record[5] = town;
            record[8..10].copy_from_slice(&m.r16(sp + 0x15e).to_le_bytes());
            record[10..12].copy_from_slice(&u16::from(m.r8(sp + 0xe)).to_le_bytes());
            record[12..16].copy_from_slice(&t.to_le_bytes());
            h.call(m, Effect::LostLetter { owner, record })?;
            letters += 1
        }
        cursor = next;
    }
    if letters > 0 {
        h.call(
            m,
            Effect::LostWindow {
                owner,
                town,
                count: letters,
            },
        )?
    }
    if m.r16(cv + 0xa) >= count {
        h.call(m, Effect::ReleaseConvoy { index: last_convoy })?;
        return Ok(());
    }
    if m.r16(cv + 0x10) >= count {
        h.call(m, Effect::PickLeader { convoy: cv })?;
        let leader = m.r16(cv + 0x10);
        if leader < count && saved_captain < cap_count {
            let sp = ship_ptr(leader)?;
            if m.r16(sp + 0x42) < cap_count {
                h.call(m, Effect::FreeCaptain { index: saved_captain })?
            } else {
                m.w16(sp + 0x42, saved_captain)
            }
        }
        let flags = m.r8(cv + 0x14);
        m.w8(cv + 0x14, flags & 0xef)
    }
    let leader = m.r16(cv + 0x10);
    if leader >= count {
        return Err("convoy sin líder al regresar".into());
    }
    m.w16(cv + 0x12, 0xf);
    m.w16(cv + 0x38, 0xffff);
    let gold = m.r32(mer);
    let purse = m.r32(cv + 0x20);
    m.w32(mer, gold.wrapping_add(purse));
    m.w32(cv + 0x20, 0);
    let towns = m.r32(TOWN_COUNT);
    if towns == 0 {
        return Err("expedición con cero ciudades".into());
    }
    let destination = (u32::from(m.r8(cv + 0x1e)) % towns) as u8;
    h.call(
        m,
        Effect::PlanRoute {
            ship: leader,
            destination,
        },
    )?;
    m.w8(cv + 0x1f, destination);
    h.call(m, Effect::RefreshConvoy { convoy: cv })?;
    let leader = m.r16(cv + 0x10);
    if leader >= count {
        return Ok(());
    }
    h.call(m, Effect::InsertOrder { ship: leader })?;
    if human {
        h.call(m, Effect::ReturnWindow { owner, ship: leader })?
    }
    let old = i32::from(m.r16(mer + 0x14));
    let delta = i32::from(m.r16(RENOWN_DELTA) as i16);
    let value = if delta > 0 {
        (old + delta).min(0xc800)
    } else {
        (old + delta).max(0)
    };
    m.w16(mer + 0x14, value as u16);
    Ok(())
}
// Adaptador para no exponer el tipo genérico en la implementación de recorrido.
trait FinishHostDyn {
    fn call(&mut self, m: &mut dyn Mem, e: Effect) -> Result<(), String>;
}
impl<T: FinishHost> FinishHostDyn for T {
    fn call(&mut self, m: &mut dyn Mem, e: Effect) -> Result<(), String> {
        FinishHost::call(self, m, e)
    }
}
