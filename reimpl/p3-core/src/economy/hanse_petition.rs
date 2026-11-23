//! Primera fase acreditada de `st_hanse_petition` (0x004EA594).
use crate::{ai::reputation::Merchant, scheduler::TaskRecord};
/// Petición por recargo con Ältermann humano, anterior al primer plazo de diez días.
pub fn can_announce(r: &TaskRecord, alderman: u8, merchants: &[Merchant]) -> bool {
    let created = r.u32_at(8);
    r.kind == 0x30
        && created != 0
        && r.due < created.wrapping_add(0xa00)
        && merchants.get(alderman as usize).is_some_and(|m| m.u16(8) == 0)
}
/// Reprograma y devuelve el destino de la petición anunciada.
pub fn announce(r: &mut TaskRecord, alderman: u8, merchants: &[Merchant]) -> Option<u8> {
    if !can_announce(r, alderman, merchants) {
        return None;
    }
    r.due = r.u32_at(8).wrapping_add(0xa00);
    Some(r.u8_at(0x12))
}

/// Segundo plazo de una petición de recargo con representante humano.
pub fn can_remind(r: &TaskRecord, alderman: u8, merchants: &[Merchant]) -> bool {
    let created = r.u32_at(8);
    r.kind == 0x30
        && created != 0
        && r.due >= created.wrapping_add(0xa00)
        && r.due < created.wrapping_add(0x1400)
        && merchants.get(alderman as usize).is_some_and(|m| m.u16(8) == 0)
}
/// Aplaza hasta el día 21; la gravedad 6 suprime el recordatorio.
pub fn remind(r: &mut TaskRecord, alderman: u8, merchants: &[Merchant]) -> Option<bool> {
    if !can_remind(r, alderman, merchants) {
        return None;
    }
    let send = r.u8_at(0xc) != 6 && !matches!(r.u8_at(0x13), 5 | 6);
    r.due = r.u32_at(8).wrapping_add(0x1500) & 0xffffff00;
    Some(send)
}

/// Tercer plazo de una petición por recargo con representante humano.
pub fn can_finish(r: &TaskRecord, alderman: u8, merchants: &[Merchant]) -> bool {
    let created = r.u32_at(8);
    r.kind == 0x30
        && created != 0
        && r.due >= created.wrapping_add(0x1400)
        && merchants.get(alderman as usize).is_some_and(|m| m.u16(8) == 0)
}
/// Termina y aplica la pérdida de reputación; gravedad 6 se descarta sin carta.
pub fn finish(r: &TaskRecord, alderman: u8, merchants: &mut [Merchant], towns: &[crate::town::Town]) -> Option<bool> {
    if !can_finish(r, alderman, merchants) {
        return None;
    }
    if r.u8_at(0xc) == 6 || matches!(r.u8_at(0x13), 5 | 6) {
        return Some(false);
    }
    let env = crate::ai::reputation::RepEnv {
        town_count: towns.len() as u32,
        towns,
        renown: [0; 3],
        byte_52c: 0,
    };
    merchants[alderman as usize].subtract_social_reputation_flagged_towns(&env);
    Some(true)
}

/// Una petición con marca de resolución cero termina sin efectos.
pub fn is_terminal(r: &TaskRecord) -> bool {
    r.kind == 0x30 && r.u32_at(8) == 0
}

/// Las clases de queja superiores a 4 se descartan si no hay representante humano.
pub fn is_ai_terminal(r: &TaskRecord, alderman: u8, merchants: &[Merchant]) -> bool {
    r.kind == 0x30
        && r.u32_at(8) != 0
        && r.u8_at(0x13) > 4
        && merchants.get(alderman as usize).is_none_or(|m| m.u16(8) != 0)
}

/// Datos de la carta 0x77 de primera fase; el retrato de piratas requiere otro puerto.
pub fn first_payload(r: &TaskRecord, alderman: u8, merchants: &[Merchant]) -> Option<([u8; 8], bool)> {
    if r.u8_at(0x13) == 2 {
        return None;
    }
    let kind = r.u8_at(0x13);
    let mut payload = [0; 8];
    payload[0] = r.u8_at(0x12);
    payload[1] = kind;
    if matches!(kind, 0 | 5 | 6 | 7) {
        payload[2] = r.u8_at(0x14)
    }
    if matches!(kind, 5 | 6 | 7) {
        payload[3] = r.u8_at(0x15);
        let m = merchants.get(alderman as usize)?;
        payload[4] = m.bytes[0x1c];
        payload[5] = m.bytes[0x1d];
    }
    Some((payload, matches!(kind, 5 | 6)))
}

/// Evidencia de bandas y capitanes obtenida del bloque S02 de barcos.
#[derive(Clone, Debug, Default)]
pub struct PiratePortraits {
    pub heads: [Option<u16>; 5],
    pub next: Vec<u16>,
    pub ship_captains: Vec<u16>,
    pub captains: Vec<[u8; 16]>,
    pub default_portrait: Option<u8>,
}

/// El retrato predeterminado es el byte bajo del contador Familiennamen de
/// `scripts/Namen_eng.ini`, cargado por el banco de nombres original.
pub fn family_name_count(config: &[u8]) -> Option<u8> {
    let text = std::str::from_utf8(config).ok()?;
    let mut section = false;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].eq_ignore_ascii_case("PERSONENNAMEN");
            continue;
        }
        if section {
            if let Some((key, value)) = line.split_once('=') {
                if key.trim().eq_ignore_ascii_case("Familiennamen") {
                    return value.trim().parse::<u16>().ok().map(|v| v as u8);
                }
            }
        }
    }
    None
}

/// Carta de petición de clase 2. Una banda sin representación accesible se
/// rechaza: el original intentaría desreferenciar su puntero nulo.
pub fn pirate_payload(r: &mut TaskRecord, bands: &PiratePortraits, human: bool) -> Option<[u8; 8]> {
    if r.u8_at(0x13) != 2 {
        return None;
    }
    let mut index = usize::from(r.u8_at(0x14));
    let portrait = if human {
        index %= 5;
        if bands.heads[index].is_none() {
            index = bands.heads.iter().position(Option::is_some)?;
        }
        r.set_u8_at(0x14, index as u8);
        25
    } else {
        let mut ship = bands.heads.get(index).copied().flatten()?;
        let mut seen = vec![false; bands.next.len()];
        let mut portrait = None;
        while usize::from(ship) < bands.next.len() {
            let i = usize::from(ship);
            if seen[i] {
                return None;
            }
            seen[i] = true;
            let captain = usize::from(*bands.ship_captains.get(i)?);
            if let Some(c) = bands.captains.get(captain) {
                if c[10] > 0x20 {
                    portrait = Some(c[3]);
                    break;
                }
            }
            ship = bands.next[i];
        }
        portrait.or(bands.default_portrait)?
    };
    let mut payload = [0; 8];
    payload[0] = r.u8_at(0x12);
    payload[1] = 2;
    payload[2] = portrait;
    Some(payload)
}

/// Petición por recargo con representante no humano o inexistente.
pub fn can_ai(r: &TaskRecord, alderman: u8, merchants: &[Merchant], towns: &[crate::town::Town], seat: u8) -> bool {
    r.kind == 0x30
        && r.u32_at(8) != 0
        && r.u8_at(0x13) <= 4
        && usize::from(seat) < towns.len()
        && merchants.get(alderman as usize).is_none_or(|m| m.u16(8) != 0)
}
/// Primer anuncio no humano: se fija el segundo plazo a un día de la creación.
pub fn ai_announce(
    r: &mut TaskRecord,
    alderman: u8,
    merchants: &[Merchant],
    towns: &[crate::town::Town],
    seat: u8,
) -> Option<u8> {
    if !can_ai(r, alderman, merchants, towns, seat) || r.due >= r.u32_at(8).wrapping_add(0x100) {
        return None;
    }
    r.due = r.u32_at(8).wrapping_add(0x100);
    Some(r.u8_at(0x12))
}
/// Convoca la votación 0x21 y devuelve la sede y los dos payloads de la carta 0x78.
pub fn ai_schedule(
    s: &mut crate::scheduler::Scheduler,
    alderman: u8,
    merchants: &[Merchant],
    towns: &[crate::town::Town],
    seat: u8,
    tick: u32,
) -> Option<(u8, u32, u32)> {
    let head = s.head as usize;
    let r = *s.records.get(head)?;
    if !can_ai(&r, alderman, merchants, towns, seat) || r.due < r.u32_at(8).wrapping_add(0x100) {
        return None;
    }
    let due = s.next_free_day_for_town(tick.wrapping_add(0xa00), seat);
    if due < r.due || (s.free_head >= s.capacity && s.capacity > u16::MAX - 128) {
        return None;
    }
    s.dispatching = 0;
    let idx = s.add_task(due);
    s.dispatching = 1;
    let out = &mut s.records[idx as usize];
    out.kind = 0x21;
    out.set_u8_at(8, seat);
    out.set_u8_at(9, if r.u8_at(0xc) == 0 { 255 } else { r.u8_at(0x12) });
    out.set_u8_at(10, r.u8_at(0x13));
    out.set_u8_at(11, r.u8_at(0xc));
    out.set_u8_at(12, r.u8_at(0x14));
    out.set_u8_at(13, r.u8_at(0x15));
    out.set_u8_at(14, r.u8_at(0xc));
    for p in 15..=0x16 {
        out.set_u8_at(p, 0);
    }
    let (epithet, name, family) = if let Some(m) = merchants.get(alderman as usize) {
        (m.get_merchant_epithet_index(), m.bytes[0x1c], m.bytes[0x1d])
    } else {
        let town = &towns[seat as usize];
        let k = town.u8(0x6f1).wrapping_neg().wrapping_sub(1) & 3;
        (
            town.u8(0x6e0 + k as usize),
            town.u8(0x6d4 + k as usize),
            town.u8(0x6d8 + k as usize),
        )
    };
    let d = crate::calendar::Calendar::for_tick(due);
    let a = u32::from(d.day)
        | (u32::from(d.month + 1) << 8)
        | (u32::from(out.u8_at(9)) << 16)
        | (u32::from(out.u8_at(14).wrapping_mul(16).wrapping_add(out.u8_at(10))) << 24);
    let b = u32::from(if out.u8_at(10) == 0 { out.u8_at(12) } else { 0 })
        | (u32::from(epithet) << 8)
        | (u32::from(name) << 16)
        | (u32::from(family) << 24);
    Some((seat, a, b))
}

/// La Dieta cancela al comienzo la moción contra una ciudad hanseática.
pub fn cancel_hanse_vote(r: &TaskRecord, towns: &[crate::town::Town]) -> bool {
    r.kind == 0x21 && r.due & 255 == 0 && towns.get(r.u8_at(9) as usize).is_some_and(|t| t.flags() & 0x40 != 0)
}

/// Recuento de una fase de 0x21; el llamante conserva la tarea hasta el resultado.
pub fn advance_vote(r: &mut TaskRecord, towns: &[crate::town::Town]) -> bool {
    let phase = r.due & 255;
    if r.kind != 0x21 || phase == 0 || phase > 0xe8 || towns.is_empty() {
        return false;
    }
    if phase < 0xe8 {
        let side = (r.due & 1) as usize;
        let pending = 0x10 + side;
        let counted = 0x12 + side;
        let noise = towns[0].stock((r.due & 15) as usize) & (0xe8 - phase) as i32;
        if noise < i32::from(r.u8_at(pending)) {
            r.set_u8_at(counted, r.u8_at(counted).wrapping_add(1));
            r.set_u8_at(pending, r.u8_at(pending).wrapping_sub(1));
        }
        if r.u8_at(0x15) != 0 {
            r.set_u8_at(0x14, r.u8_at(0x14).wrapping_add(1));
            r.set_u8_at(0x15, r.u8_at(0x15).wrapping_sub(1));
        }
    } else {
        r.set_u8_at(0x13, r.u8_at(0x13).wrapping_add(r.u8_at(0x11)));
        r.set_u8_at(0x12, r.u8_at(0x12).wrapping_add(r.u8_at(0x10)));
        r.set_u8_at(0x14, r.u8_at(0x14).wrapping_add(r.u8_at(0x15)));
    }
    r.due = r.due.wrapping_add(1);
    true
}

/// Inicia la votación y devuelve los comerciantes humanos a los que se avisa.
pub fn start_vote(r: &mut TaskRecord, towns: &[crate::town::Town], merchants: &[Merchant]) -> Option<Vec<u16>> {
    if r.kind != 0x21 || r.due & 255 != 0 || towns.is_empty() || cancel_hanse_vote(r, towns) {
        return None;
    }
    let mut notified = Vec::new();
    let mut weight = 0u32;
    let mut mask = 0u8;
    for (i, m) in merchants.iter().enumerate() {
        let home = usize::from(*m.bytes.get(0x19)?);
        let rank = *m.bytes.get(0x39c + home)?;
        if m.u16(8) == 0 && rank > 3 {
            mask |= 1u32.wrapping_shl((notified.len() as u32) & 31) as u8;
            notified.push(i as u16);
            weight += if rank < 6 {
                1
            } else if rank == 6 {
                2
            } else {
                3
            };
        }
    }
    r.set_u8_at(0x16, mask);
    let mut bribes = 0u8;
    for t in towns {
        for k in 0..4 {
            if usize::from(t.u8(0x6dc + k)) < merchants.len() {
                bribes = bribes.wrapping_add(1)
            }
        }
    }
    r.set_u8_at(0xf, bribes);
    weight += u32::from(bribes);
    let total = if weight < 28 { 38 - weight } else { 10 };
    let sum = towns.iter().fold(0i32, |a, t| a.wrapping_add(t.stock(0)));
    let bias = (i32::from(r.u8_at(0xb)) * 2 - i32::from(r.u8_at(0xe)) * 2 + 7 + sum % 5).min(16);
    let against = ((bias.wrapping_mul(total as i32) as u32) >> 4) as u8;
    r.set_u8_at(0x11, against);
    r.set_u8_at(0x10, (total as u8).wrapping_sub(against));
    for p in [0x12, 0x13, 0x14, 0x15] {
        r.set_u8_at(p, 0)
    }
    r.due = r.due.wrapping_add(10);
    Some(notified)
}

/// Termina la votación, aplica la multa o convierte la tarea en expedición.
/// Devuelve conservación de la tarea, sede y payload de la carta 0x0B.
pub fn finish_vote(
    r: &mut TaskRecord,
    towns: &mut [crate::town::Town],
    seat: u8,
    tick: u32,
) -> Option<(bool, u8, [u8; 8])> {
    if r.kind != 0x21 || r.due & 255 <= 0xe8 {
        return None;
    }
    let target = r.u8_at(9);
    let kind = r.u8_at(0xe);
    let yes = r.u8_at(0x12);
    let no = r.u8_at(0x13);
    if yes > no && matches!(kind, 3..=5) && usize::from(target) >= towns.len() {
        return None;
    }
    let mut payload = [0; 8];
    payload[..2].copy_from_slice(&u16::from(target).to_le_bytes());
    payload[2..4].copy_from_slice(&(u16::from(r.u8_at(10)) + (u16::from(kind) << 8)).to_le_bytes());
    payload[4..6].copy_from_slice(&u16::from(yes).to_le_bytes());
    payload[6..8].copy_from_slice(&u16::from(no).to_le_bytes());
    let town = r.u8_at(8);
    if yes > no && kind <= 2 {
        r.due = r.due.wrapping_add(0x100);
        r.kind = [0x34, 0x24, 0x23][usize::from(kind)];
        for p in 8..16 {
            r.set_u8_at(p, 255)
        }
        r.set_u32_at(0x10, 0);
        r.set_u8_at(0x14, if kind == 0 { seat } else { target });
        r.set_u8_at(0x15, 0);
        r.set_u8_at(0x16, 255);
        r.set_u8_at(0x17, 255);
        return Some((true, town, payload));
    }
    if yes > no && matches!(kind, 3..=5) {
        let mut remaining =
            [160000u32, 120000, 80000][usize::from(kind - 3)].min(towns[usize::from(target)].u32(0x700));
        crate::economy::militia::book_transaction(&mut towns[usize::from(target)], tick, remaining as i32, 10);
        let mut other = towns.len() as u32;
        for (i, town) in towns.iter_mut().enumerate() {
            if i == usize::from(target) {
                continue;
            }
            other -= 1;
            let share = (remaining + other / 2) / other;
            crate::economy::militia::book_transaction(town, tick, share as i32, 4);
            remaining -= share;
        }
    }
    Some((false, town, payload))
}

/// Operación 0x3F sobre la primera elección o votación del día; las elecciones
/// 0x20 pertenecen a otro sistema y se dejan al despachador correspondiente.
pub fn cast_hanse_ballot(
    s: &mut crate::scheduler::Scheduler,
    towns: &[crate::town::Town],
    merchants: &[Merchant],
    tick: u32,
    actor: usize,
    choice: u32,
) -> bool {
    let Some(m) = merchants.get(actor) else { return false };
    let home = usize::from(m.bytes[0x19]);
    let Some(&rank) = m.bytes.get(0x39c + home) else {
        return false;
    };
    if rank < 4 {
        return false;
    }
    let mut at = s.head;
    let mut steps = 0;
    let day = tick & 0xffffff00;
    while at < s.capacity && usize::from(at) < s.records.len() && steps < s.records.len() {
        let r = s.records[usize::from(at)];
        if r.due.wrapping_sub(day) > 255 {
            return false;
        }
        if r.kind == 0x20 {
            return false;
        }
        if r.kind == 0x21 {
            break;
        }
        at = r.next;
        steps += 1;
    }
    if at >= s.capacity || usize::from(at) >= s.records.len() || steps >= s.records.len() {
        return false;
    }
    let humans = merchants[..actor].iter().filter(|m| m.u16(8) == 0).count();
    let mask = 1u32.wrapping_shl((humans as u32) & 31) as u8;
    let record = &mut s.records[usize::from(at)];
    if record.u8_at(0x16) & mask == 0 {
        return false;
    }
    record.set_u8_at(0x16, record.u8_at(0x16) & !mask);
    let bribes = towns
        .iter()
        .map(|t| (0..4).filter(|&k| usize::from(t.u8(0x6dc + k)) == actor).count())
        .sum::<usize>();
    let weight = if rank < 6 {
        1
    } else if rank == 6 {
        2
    } else {
        3
    };
    if choice > 1 {
        record.set_u8_at(0x14, record.u8_at(0x14).wrapping_add(weight));
        record.set_u8_at(0x15, (usize::from(record.u8_at(0x15)) + bribes).min(250) as u8);
    } else {
        let counted = 0x12 + choice as usize;
        let pending = 0x10 + choice as usize;
        record.set_u8_at(counted, record.u8_at(counted).wrapping_add(weight));
        record.set_u8_at(pending, record.u8_at(pending).wrapping_add(bribes as u8));
    }
    true
}
