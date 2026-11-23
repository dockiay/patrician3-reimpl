//! Recuento de pérdidas y acumulador de investigación en SeaBattle::finish
//! (0x0060D299). El texto localizado y las demás consecuencias son externos.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Losses {
    pub sunk: u16,
    pub captured: u16,
    pub looted: u16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LossNotice {
    pub recipient: u16,
    pub opponent: u16,
    pub losses: Losses,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BattleReport {
    pub recipient: u16,
    pub town: u8,
    pub text: Vec<u8>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FinishEffects {
    pub losses: Vec<LossNotice>,
    pub reports: Vec<BattleReport>,
    /// Por cada informe, cuántas cartas globales 0x2A emitió el cierre antes que él: el original
    /// las envía en el orden de `sea_finish` (ConvoyPrueba, día 222: 0x29 antes que 0x2A).
    pub report_positions: Vec<usize>,
    pub global_letters: Vec<[u8; 16]>,
    pub pirate_news: Vec<u8>,
    pub news_stamp: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub crime: u16,
    pub losses: [Losses; 2],
    pub notices: Vec<LossNotice>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Investigation {
    pub convoy: u16,
    pub merchant: u16,
    pub random: u32,
    pub class: u16,
}

/// Tiradas del destructor 0x00603BC0 cuando devuelve ambos convoyes.
/// La creación de la tarea de investigación es un efecto del anfitrión.
pub fn investigation_after_battle(
    battle: &[u8],
    ship_count: u16,
) -> Result<(u32, Option<Investigation>), &'static str> {
    if battle.len() < 0x1e96 {
        return Err("batalla truncada");
    }
    let r16 = |at: usize| u16::from_le_bytes([battle[at], battle[at + 1]]);
    let seed = u32::from_le_bytes(battle[0xadc..0xae0].try_into().unwrap());
    if battle[0x5c] == 0
        || r16(0x15c) != 0
        || r16(0x130c) < ship_count
        || r16(0x65e) == u16::MAX
        || r16(0x660) == u16::MAX
        || !matches!(battle[0xad2], 1 | 2)
    {
        return Ok((seed, None));
    }
    let first = super::wind::next_seed(seed);
    let crime = r16(0x1e94);
    if first % 1000 >= u32::from(crime & 0x3ff) {
        return Ok((first, None));
    }
    let second = super::wind::next_seed(first);
    let (convoy, merchant) = if battle[0xad2] == 1 {
        (r16(0x65e), r16(0x35c))
    } else {
        (r16(0x660), r16(0x35a))
    };
    Ok((
        second,
        Some(Investigation {
            convoy,
            merchant,
            random: second,
            class: crime >> 10,
        }),
    ))
}

/// `0x0050CB80`: crea la tarea 0x05 si el convoy pirata tiene dueño válido.
pub fn schedule_investigation(
    event: Investigation,
    tick: u32,
    ships: &[u8],
    convoys: &[u8],
    merchants: &[crate::ai::reputation::Merchant],
    scheduler: &mut crate::scheduler::Scheduler,
) -> Result<Option<u16>, &'static str> {
    if ships.len() % 0x180 != 0 || convoys.len() % 0x3c != 0 {
        return Err("flota truncada");
    }
    let at = usize::from(event.convoy) * 0x3c;
    let cv = convoys.get(at..at + 0x3c).ok_or("convoy ausente")?;
    let owner = usize::from(cv[0]);
    let Some(merchant) = merchants.get(owner) else {
        return Ok(None);
    };
    let home = *merchant.bytes.get(0x19).ok_or("comerciante truncado")?;
    let read = |off: usize| u16::from_le_bytes([cv[off], cv[off + 1]]);
    let n = ships.len() / 0x180;
    let leader = usize::from(read(0x10));
    let head = usize::from(read(0xa));
    let ship = if leader < n { leader } else { head };
    let tag = if ship < n {
        let at = ship * 0x180 + 0x15e;
        u16::from_le_bytes([ships[at], ships[at + 1]])
    } else {
        0
    };
    let due = tick.wrapping_add(0x500).wrapping_add(u32::from(event.convoy));
    let id = scheduler.add_task(due);
    let task = &mut scheduler.records[usize::from(id)];
    task.kind = 5;
    task.set_u8_at(8, owner as u8);
    let class = event.class as u8;
    if class < 13 {
        task.set_u8_at(9, home);
        task.set_u8_at(10, event.merchant as u8)
    } else {
        task.set_u8_at(9, event.merchant as u8);
        task.set_u8_at(10, 0xff)
    }
    task.set_u8_at(11, home);
    task.set_u32_at(0xc, tick);
    task.set_u8_at(0x10, class);
    task.set_u8_at(0x11, 0);
    task.payload[10..12].copy_from_slice(&(event.random as u16).to_le_bytes());
    task.payload[12..14].fill(0);
    task.payload[14..16].copy_from_slice(&tag.to_le_bytes());
    Ok(Some(id))
}

/// Primer bloque de 0x0060D299: posible barco para un combate encadenado.
/// Devuelve el índice mundial y la semilla posterior, sin publicar efectos.
pub fn select_followup(battle: &[u8], worlds: &[u8], convoys: &[u8], outcome: u8) -> Result<(u16, u32), &'static str> {
    if battle.len() < 0x1e96 || worlds.len() % 0x180 != 0 || convoys.len() % 0x3c != 0 {
        return Err("batalla o flota truncada");
    }
    let r16 = |b: &[u8], at: usize| u16::from_le_bytes([b[at], b[at + 1]]);
    let seed = u32::from_le_bytes(battle[0xadc..0xae0].try_into().unwrap());
    if outcome != 0 || battle[0x12e8] != 0 {
        return Ok((u16::MAX, seed));
    }
    let cv = usize::from(r16(battle, 0x660));
    if cv >= convoys.len() / 0x3c {
        return Ok((u16::MAX, seed));
    }
    let mut id = r16(convoys, cv * 0x3c + 0xa);
    let mut seen = vec![false; worlds.len() / 0x180];
    let mut choices = Vec::new();
    let mut total = 0u32;
    while usize::from(id) < seen.len() {
        let i = usize::from(id);
        if seen[i] {
            return Err("convoy cíclico");
        }
        seen[i] = true;
        let ship = &worlds[i * 0x180..(i + 1) * 0x180];
        let owner = u16::from(ship[0]);
        if owner != r16(battle, 0x35a) && owner != r16(battle, 0x35c) {
            let weight = i32::from_le_bytes(ship[0x10..0x14].try_into().unwrap());
            if weight < 0 {
                return Err("capacidad negativa no admitida");
            }
            total = total.wrapping_add(weight as u32);
            choices.push((id, weight as u32))
        }
        id = r16(ship, 6);
    }
    if (total as i32) < 1 {
        return Ok((u16::MAX, seed));
    }
    let next = super::wind::next_seed(seed);
    let mut draw = next % total + 1;
    for (id, weight) in choices {
        if draw <= weight {
            return Ok((id, next));
        }
        draw -= weight
    }
    Err("sorteo de convoy inválido")
}

/// Añade temporalmente los dueños presentes en ambos convoyes (0x0060D410).
/// Devuelve el número usado para los informes; el byte +0x558 no se reescribe.
pub fn expand_report_participants(battle: &mut [u8], worlds: &[u8], convoys: &[u8]) -> Result<usize, &'static str> {
    if battle.len() < 0x1e96 || worlds.len() % 0x180 != 0 || convoys.len() % 0x3c != 0 {
        return Err("batalla o flota truncada");
    }
    let mut staged = battle.to_vec();
    let r16 = |b: &[u8], at: usize| u16::from_le_bytes([b[at], b[at + 1]]);
    let mut count = usize::from(staged[0x558]);
    if staged[0x12e8] == 0 {
        for side in 0..2 {
            if r16(&staged, 0x35a + side * 2) == 0xfe {
                continue;
            }
            let cv = usize::from(r16(&staged, 0x65e + side * 2));
            if cv >= convoys.len() / 0x3c {
                continue;
            }
            let mut id = r16(convoys, cv * 0x3c + 0xa);
            let mut seen = vec![false; worlds.len() / 0x180];
            while usize::from(id) < seen.len() {
                let i = usize::from(id);
                if seen[i] {
                    return Err("convoy cíclico");
                }
                seen[i] = true;
                let ship = &worlds[i * 0x180..(i + 1) * 0x180];
                let owner = u16::from(ship[0]);
                if !(0..count).any(|j| r16(&staged, 0x35a + j * 2) == owner) {
                    if count >= 255 {
                        return Err("demasiados participantes");
                    }
                    staged[0x35a + count * 2..0x35c + count * 2].copy_from_slice(&owner.to_le_bytes());
                    staged[0x5d + count] = 1;
                    staged[0x559 + count] = side as u8;
                    count += 1
                }
                id = r16(ship, 6);
            }
        }
    }
    battle.copy_from_slice(&staged);
    Ok(count)
}

/// Cierre sin pérdidas ni destinatarios humanos, para una batalla que no
/// puede encadenar otra. Las cartas 0x29 producidas por esta vía se descartan
/// en `add_message` porque los buzones de todos sus destinatarios son IA.
/// Las demás variantes se rechazan antes de publicar cambios.
pub fn finish_ai_without_losses(
    battle: &mut [u8],
    ships: &[Vec<u8>],
    worlds: &[u8],
    next: &[u16],
    merchants: &[u16],
    outcome: u8,
) -> Result<(), &'static str> {
    if battle.len() < 0x1e98 || outcome > 1 {
        return Err("batalla inválida");
    }
    let r16 = |b: &[u8], at: usize| u16::from_le_bytes([b[at], b[at + 1]]);
    let timer = r16(battle, 0x15c);
    if timer != 0 && timer < 0xf000 {
        return Ok(());
    }
    if battle[0x12e8] == 0 {
        return Err("cierre de convoyes ampliados pendiente");
    }
    let count = usize::from(battle[0x558]);
    if count > 255 {
        return Err("participantes inválidos");
    }
    for i in 0..count {
        let owner = usize::from(r16(battle, 0x35a + i * 2));
        if battle[0x5d + i] == 0 || merchants.get(owner) == Some(&0) {
            return Err("informe humano de batalla pendiente");
        }
    }
    for s in ships {
        if s.len() < 0x143 || s[0x12a] & 0x64 != 0 || s[0x13d] != s[0x142] {
            return Err("cierre con pérdidas pendiente");
        }
    }
    let report = summarize(battle, ships, worlds, next)?;
    if !report.notices.is_empty() {
        return Err("avisos de bajas pendientes");
    }
    battle[0x1e94..0x1e96].copy_from_slice(&report.crime.to_le_bytes());
    battle[0x130c..0x130e].copy_from_slice(&u16::MAX.to_le_bytes());
    battle[0x130e] = outcome;
    battle[0x5c] = 1;
    let delay = if battle[0xac8] != 0 { 1u16 } else { 250 };
    battle[0x15c..0x15e].copy_from_slice(&delay.to_le_bytes());
    Ok(())
}

/// Ejecuta el cierre completo cuando sus únicos efectos externos son cartas
/// 0x29 o 0x72 dirigidas a IA (el gestor original las descarta). Cualquier
/// aviso humano, noticia o intervención pendiente rechaza toda la operación.
/// Los textos vacíos solo se usan para cartas que no llegan a almacenarse.
pub fn finish_ai_headless(
    battle: &mut [u8],
    ships: &[Vec<u8>],
    worlds: &[u8],
    next: &[u16],
    fleets: &[u8],
    merchants: &[u16],
    player: u32,
    outcome: u8,
) -> Result<(), &'static str> {
    let mut staged = battle.to_vec();
    let notices = finish_headless_with_losses(&mut staged, ships, worlds, next, fleets, merchants, player, outcome)?;
    if !notices.is_empty() {
        return Err("entrega de bajas pendiente");
    }
    battle.copy_from_slice(&staged);
    Ok(())
}

/// Igual que `finish_ai_headless`, pero entrega al anfitrión las cartas 0x72
/// para sus destinatarios humanos. Solo se confirma tras preparar el informe.
pub fn finish_headless_with_losses(
    battle: &mut [u8],
    ships: &[Vec<u8>],
    worlds: &[u8],
    next: &[u16],
    fleets: &[u8],
    merchants: &[u16],
    player: u32,
    outcome: u8,
) -> Result<Vec<LossNotice>, &'static str> {
    let effects = finish_headless_with_reports(
        battle,
        ships,
        worlds,
        next,
        fleets,
        merchants,
        &[],
        &[],
        None,
        player,
        outcome,
    )?;
    Ok(effects.losses)
}

/// Como el cierre sin vista, suministrando los textos originales para poder
/// entregar también informes 0x29 a jugadores humanos.
pub fn finish_headless_with_reports(
    battle: &mut [u8],
    ships: &[Vec<u8>],
    worlds: &[u8],
    next: &[u16],
    fleets: &[u8],
    merchants: &[u16],
    towns: &[u8],
    enabled: &[bool],
    texts: Option<&dyn Fn(u32) -> Option<Vec<u8>>>,
    player: u32,
    outcome: u8,
) -> Result<FinishEffects, &'static str> {
    finish_headless_with_news(
        battle, ships, worlds, next, fleets, merchants, towns, enabled, texts, player, outcome, None, 0, 0, 0,
    )
}
/// Full no-view finish with real merchant/captain data and transient news clock.
pub fn finish_headless_with_news(
    battle: &mut [u8],
    ships: &[Vec<u8>],
    worlds: &[u8],
    next: &[u16],
    fleets: &[u8],
    merchants: &[u16],
    towns: &[u8],
    enabled: &[bool],
    texts: Option<&dyn Fn(u32) -> Option<Vec<u8>>>,
    player: u32,
    outcome: u8,
    merchant_data: Option<&[u8]>,
    captain_count: u16,
    clock: u32,
    news_stamp: u32,
) -> Result<FinishEffects, &'static str> {
    use super::finish::{sea_finish, FinishGlobals, FinishHost};
    if battle.len() < 0x1e98
        || outcome > 1
        || worlds.len() % 0x180 != 0
        || fleets.len() % 0x3c != 0
        || worlds.len() / 0x180 > u16::MAX as usize
        || fleets.len() / 0x3c > u16::MAX as usize
        || merchants.len() > u16::MAX as usize
        || next.len() < ships.len()
        || ships.iter().any(|s| s.len() < 0x143)
    {
        return Err("tablas de batalla inválidas");
    }
    let word = |b: &[u8], i: usize| u16::from_le_bytes([b[i], b[i + 1]]);
    if merchant_data.is_none() && (word(battle, 0x35a) == 0xff || word(battle, 0x35c) == 0xff || battle[0xad2] & 3 != 0)
    {
        return Err("noticia de piratas pendiente");
    }
    // Las dos cadenas no pueden repetirse ni contener enlaces inválidos.
    let mut seen = vec![false; ships.len()];
    for mut id in [word(battle, 0x10), word(battle, 0x65c)] {
        while id != u16::MAX {
            let i = usize::from(id);
            if i >= ships.len() || seen[i] {
                return Err("lista local de barcos inválida");
            }
            seen[i] = true;
            id = next[i];
        }
    }
    struct Silent<'a> {
        merchant_data: Option<&'a [u8]>,
        global_letters: Vec<[u8; 16]>,
        pirate_news: Vec<u8>,
        merchants: &'a [u16],
        towns: &'a [u8],
        enabled: &'a [bool],
        texts: Option<&'a dyn Fn(u32) -> Option<Vec<u8>>>,
        missing_text: bool,
        unsupported: bool,
        notices: Vec<LossNotice>,
        reports: Vec<BattleReport>,
        report_positions: Vec<usize>,
    }
    impl FinishHost for Silent<'_> {
        fn text(&mut self, slot: u32) -> Vec<u8> {
            if let Some(text) = self.texts.and_then(|f| f(slot)) {
                text
            } else {
                self.missing_text = true;
                Vec::new()
            }
        }
        fn cstr(&mut self, _: u32) -> Vec<u8> {
            self.unsupported = true;
            Vec::new()
        }
        fn pirate_name(&mut self, _: u32, _: u8) -> Vec<u8> {
            self.unsupported = true;
            Vec::new()
        }
        fn merchant_flag(&mut self, id: u32) -> u8 {
            if let Some(record) = self
                .merchant_data
                .and_then(|v| v.get(id as usize * 0x650..(id as usize + 1) * 0x650))
            {
                crate::ai::reputation::Merchant { bytes: record.to_vec() }.get_merchant_epithet_index()
            } else {
                self.unsupported = true;
                0
            }
        }
        fn event(&mut self, target: u32, kind: u8, town: u8, d8: u32, d12: u32, text: Option<&[u8]>) {
            match (kind, self.merchants.get(target as usize)) {
                (0x2a, _) if target == u32::MAX - 1 => {
                    let mut letter = [0u8; 16];
                    letter[4] = kind;
                    letter[5] = town;
                    letter[8..12].copy_from_slice(&d8.to_le_bytes());
                    letter[12..16].copy_from_slice(&d12.to_le_bytes());
                    self.global_letters.push(letter)
                }
                (0x29 | 0x72, None) if self.merchant_data.is_some() => {}
                (0x29 | 0x72, Some(kind)) if *kind != 0 => {}
                (0x72, Some(_)) => self.notices.push(LossNotice {
                    recipient: target as u16,
                    opponent: d8 as u16,
                    losses: Losses {
                        sunk: (d8 >> 16) as u16,
                        captured: d12 as u16,
                        looted: (d12 >> 16) as u16,
                    },
                }),
                (0x29, Some(_)) if self.enabled.get(target as usize) == Some(&false) => {}
                (0x29, Some(_))
                    if !self.missing_text && self.towns.get(target as usize).is_some() && text.is_some() =>
                {
                    self.report_positions.push(self.global_letters.len());
                    self.reports.push(BattleReport {
                        recipient: target as u16,
                        town,
                        text: text.unwrap().to_vec(),
                    })
                }
                _ => self.unsupported = true,
            }
        }
        fn show_message(&mut self, _: &[u8]) {
            self.unsupported = true
        }
        fn news_open(&mut self) {}
        fn news(&mut self, name: u8) {
            self.pirate_news.push(name)
        }
        fn trace(&mut self, _: &[u8]) {}
    }
    let mut records = vec![0; merchants.len() * 0x650];
    for (i, &kind) in merchants.iter().enumerate() {
        records[i * 0x650 + 8..i * 0x650 + 10].copy_from_slice(&kind.to_le_bytes());
        records[i * 0x650 + 0x19] = towns.get(i).copied().unwrap_or(0)
    }
    if let Some(data) = merchant_data {
        if data.len() != records.len() {
            return Err("datos mercantiles de noticias truncados");
        }
        records.copy_from_slice(data)
    }
    let mut g = FinishGlobals {
        fleets,
        world_count: (worlds.len() / 0x180) as u16,
        fleet_count: (fleets.len() / 0x3c) as u16,
        captain_count,
        merchant_count: merchants.len() as u16,
        merchants: &records,
        player,
        dialog_open: 0,
        news_stamp,
        clock,
    };
    let mut host = Silent {
        merchant_data,
        global_letters: Vec::new(),
        pirate_news: Vec::new(),
        merchants,
        towns,
        enabled,
        texts,
        missing_text: false,
        unsupported: false,
        notices: Vec::new(),
        reports: Vec::new(),
        report_positions: Vec::new(),
    };
    let mut staged = battle.to_vec();
    sea_finish(&mut staged, ships, worlds, next, 0, &mut g, &mut host, outcome)?;
    // Los participantes nuevos (convoy ampliado) pueden convertir un 0xFE inicial en 0xFF antes de
    // la carta pirata 0x2A. Sin datos mercantiles el número de capitanes es 0 y esa carta (con su
    // consulta 0x004F9430) se omitiría en silencio: se rechaza mirando el estado final.
    if merchant_data.is_none() && (word(&staged, 0x35a) == 0xff || word(&staged, 0x35c) == 0xff) {
        return Err("noticia de piratas pendiente");
    }
    if host.unsupported {
        return Err("efectos de cierre de batalla pendientes");
    }
    battle.copy_from_slice(&staged);
    Ok(FinishEffects {
        losses: host.notices,
        reports: host.reports,
        report_positions: host.report_positions,
        global_letters: host.global_letters,
        pirate_news: host.pirate_news,
        news_stamp: g.news_stamp,
    })
}

/// Recorre las dos listas locales en el orden original; no modifica el estado
/// ni envía cartas. `worlds` contiene los registros de 0x180 bytes.
pub fn summarize(battle: &[u8], ships: &[Vec<u8>], worlds: &[u8], next: &[u16]) -> Result<Report, &'static str> {
    if battle.len() < 0x1e96 || worlds.len() % 0x180 != 0 {
        return Err("batalla o flota truncada");
    }
    let r16 = |b: &[u8], at: usize| u16::from_le_bytes([b[at], b[at + 1]]);
    let mut crime = r16(battle, 0x1e94);
    let mut losses = [Losses::default(); 2];
    let mut seen = vec![false; ships.len()];
    for head in [r16(battle, 0x10), r16(battle, 0x65c)] {
        let mut id = head;
        while id != u16::MAX {
            let i = usize::from(id);
            if i >= ships.len() || i >= next.len() || seen[i] {
                return Err("lista de batalla inválida");
            }
            seen[i] = true;
            let local = &ships[i];
            if local.len() < 0x143 {
                return Err("barco local truncado");
            }
            let world_id = u32::from_le_bytes(local[4..8].try_into().unwrap()) as usize;
            let at = world_id.checked_mul(0x180).ok_or("barco mundial inválido")?;
            let world = worlds.get(at..at + 0x180).ok_or("barco mundial ausente")?;
            let max_health = i32::from_le_bytes(world[0x14..0x18].try_into().unwrap());
            if max_health != 0 {
                let side = usize::from(local[0x142] != 0);
                let flags = local[0x12a];
                let penalty = if flags & 4 != 0 {
                    losses[side].sunk = losses[side].sunk.wrapping_add(1);
                    100
                } else if local[0x13d] != local[0x142] {
                    losses[side].captured = losses[side].captured.wrapping_add(1);
                    200
                } else if flags & 0x40 != 0 {
                    losses[side].looted = losses[side].looted.wrapping_add(1);
                    50
                } else {
                    50
                };
                if (side == 0 && battle[0xad2] == 2) || (side == 1 && battle[0xad2] == 1) {
                    let old = i32::from(crime & 0x3ff);
                    let low = 1000 - ((1000 - old) * (1000 - penalty) + 500) / 1000;
                    crime = (crime & 0xfc00).wrapping_add(low as u16);
                }
            }
            id = next[i];
        }
    }
    let participants = [r16(battle, 0x35a), r16(battle, 0x35c)];
    let mut notices = Vec::new();
    for side in 0..2 {
        let other = 1 - side;
        let item = losses[other];
        if participants[side] != 0xff && (item.sunk != 0 || item.captured != 0 || item.looted != 0) {
            notices.push(LossNotice {
                recipient: participants[side],
                opponent: participants[other],
                losses: item,
            })
        }
    }
    Ok(Report { crime, losses, notices })
}
