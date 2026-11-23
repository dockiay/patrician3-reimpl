//! Candidatos de la elección municipal original 0x00528CF0.
use p3_core::{ai::reputation::Merchant, town::Town};

/// Devuelve las cinco columnas de cuatro candidatos y actualiza en cada IA la
/// bandera de candidatura, igual que el consultor del ejecutable.
pub(crate) fn candidates(town: &Town, town_count: usize, merchants: &mut [Merchant]) -> Result<[u8; 20], String> {
    let city = usize::from(town.u8(0x2c0));
    if city >= town_count
        || city >= 40
        || merchants.len() > u8::MAX as usize
        || merchants.iter().any(|m| m.bytes.len() < 0x650)
    {
        return Err("datos de candidatos incompletos".into());
    }
    let mut ids = [0xfcu8, 0xfd, 0xfe, 0xff];
    let mut scores = [80i32, 70, 65, 59];
    for id in (0..merchants.len()).rev() {
        let m = &mut merchants[id];
        if m.get_mayor_candidacy_flag() == 0 {
            continue;
        }
        if (m.i32(0x468) as u32) & (1u32 << (city & 31)) == 0
            || usize::from(m.bytes[0x19]) != city
            || m.bytes[0x39c + city] <= 4
        {
            continue;
        }
        let rep = m.f32(0x2fc + city * 4);
        let rep = if rep > 32758.0 {
            32767
        } else {
            p3_core::numeric::ftol_i32(rep as f64) as i16 as i32
        };
        for i in (0..4).rev() {
            if rep <= scores[i] {
                break;
            }
            if i < 3 {
                scores[i + 1] = scores[i];
                ids[i + 1] = ids[i]
            }
            scores[i] = rep;
            ids[i] = id as u8;
        }
    }
    let mut out = [0u8; 20];
    out[..4].copy_from_slice(&ids);
    for i in 0..4 {
        let id = usize::from(ids[i]);
        if let Some(m) = merchants.get(id) {
            out[4 + i] = m.get_merchant_epithet_index();
            out[8 + i] = m.bytes[0x39c + city];
            out[12 + i] = m.bytes[0x1c];
            out[16 + i] = m.bytes[0x1d]
        } else {
            let slot = (!ids[i] & 3) as usize;
            out[4 + i] = town.u8(0x6e0 + slot);
            out[8 + i] = town.u8(0x6e4 + slot);
            out[12 + i] = town.u8(0x6d4 + slot);
            out[16 + i] = town.u8(0x6d8 + slot)
        }
    }
    Ok(out)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Ballot {
    pub votes: [i32; 4],
    pub winner: u8,
    pub terms: u8,
    pub bribes: [u8; 4],
    pub statue: bool,
}

/// Reparte 120 votos, aplica sobornos de concejales y elige por orden en caso de empate.
/// Los valores de reputación corresponden a los cuatro candidatos ya seleccionados.
pub(crate) fn ballot(
    ids: [u8; 4],
    mut reputation: [i32; 4],
    mut bribes: [u8; 4],
    merchant_count: u16,
    tick: u32,
    old_mayor: u8,
    old_terms: u8,
) -> Result<Ballot, String> {
    if ids.iter().all(|&id| u16::from(id) >= merchant_count) {
        for (i, rep) in reputation.iter_mut().enumerate() {
            let bonus = if *rep == 80 {
                31
            } else {
                (((tick >> 8).wrapping_mul(i as u32 + 7) % 67) & 31) as i32
            };
            *rep = rep.wrapping_add(bonus)
        }
    }
    let total = reputation.iter().fold(0i32, |a, &b| a.wrapping_add(b));
    if total == 0 {
        return Err("reputación electoral total nula".into());
    }
    let mut remaining = 120i32;
    let mut votes = [0i32; 4];
    for bribe in &mut bribes {
        if u16::from(*bribe) < merchant_count {
            if let Some(i) = ids.iter().position(|id| id == bribe) {
                votes[i] += 3;
                remaining -= 3;
                *bribe = 0xff
            }
        }
    }
    for i in 0..4 {
        let product = reputation[i].wrapping_mul(remaining);
        let share = product.checked_div(total).ok_or("división electoral fuera de rango")?;
        votes[i] = votes[i].wrapping_add(share)
    }
    let mut winner = 0usize;
    for i in 1..4 {
        if votes[i] > votes[winner] {
            winner = i
        }
    }
    let winner = ids[winner];
    let terms = if winner == old_mayor {
        old_terms.wrapping_add(1).min(250)
    } else {
        0
    };
    Ok(Ballot {
        votes,
        winner,
        terms,
        bribes,
        statue: winner == old_mayor && terms == 1 && u16::from(winner) < merchant_count,
    })
}

/// Efectos externos de `merchant_update_rank_in_town` (`0x004F75F0`) durante la elección.
#[derive(Debug, Default)]
pub(crate) struct RankEffects {
    /// Cartas `0x004D6530` (destinatario, registro) en el orden del original.
    pub messages: Vec<(i32, [u8; 16])>,
    /// `0x005326E0`: el comerciante alcanzó el umbral de prestigio fuera de red y se programan misiones del alcalde.
    pub prepare_missions: bool,
}

/// Actualiza el rango de un comerciante (humano o IA) tras la elección. La consulta `0x004D7AC0` de cartas 0x5F
/// pendientes se resuelve sobre `pool`; las cartas y la programación de misiones se devuelven para que el llamante
/// las aplique en orden.
#[allow(clippy::too_many_arguments)]
pub(crate) fn update_rank(
    merchant: &mut Merchant,
    towns: &[Town],
    city: usize,
    office_owners: &[u8],
    alderman: u8,
    alderman_town: u8,
    merchant_count: u16,
    network: bool,
    pool: &[u8],
) -> Result<RankEffects, String> {
    use p3_core::ai::misc::{MessageHost, RankEnv, RankHost};
    struct Effects<'a> {
        out: RankEffects,
        pool: &'a [u8],
        error: Option<String>,
    }
    impl MessageHost for Effects<'_> {
        fn post_message(&mut self, kind: i32, record: &[u8; 16]) {
            self.out.messages.push((kind, *record));
        }
    }
    impl RankHost for Effects<'_> {
        fn init_world(&mut self) {
            self.out.prepare_missions = true;
        }
        fn message_pending(&mut self, word: u32, level: u32) -> bool {
            match p3_state::messages::rank_message_pending(self.pool, word as u16, level) {
                Ok(found) => found,
                Err(e) => {
                    self.error.get_or_insert(e);
                    true
                }
            }
        }
    }
    if merchant.bytes.len() < 0x650 || city >= towns.len() || towns.len() > 40 || office_owners.len() > usize::from(u16::MAX) {
        return Err("datos de rango electoral incompatibles".into());
    }
    let env = RankEnv {
        town_count: towns.len() as u32,
        towns,
        office_count: office_owners.len() as u16,
        offices: office_owners,
        byte_52d: alderman,
        byte_52e: alderman_town,
        word_4aa: merchant_count,
        world_ready: network,
    };
    let mut effects = Effects { out: RankEffects::default(), pool, error: None };
    merchant.update_rank_in_town(&env, city as u32, &mut effects);
    match effects.error {
        Some(e) => Err(e),
        None => Ok(effects.out),
    }
}

/// `merchant_refresh_first_name_ptr` (`0x004F6420`): normaliza el bit 7 del nombre (`+0x1C`) según el sexo (`+0x1A`)
/// y devuelve el índice con el que el original busca el nombre de pila.
/// También cachea el puntero `+0xE8` que leen `%v` y las ventanas (véase `mission_format::name_ptr`).
pub(crate) fn refresh_first_name(merchant: &mut Merchant) -> u8 {
    p3_core::mission_format::refresh_first_name_ptr(merchant)
}

/// Actualiza el rango del alcalde IA; las misiones de la Liga pendientes se rechazan.
pub(crate) fn update_rank_ai(
    merchant: &mut Merchant,
    towns: &[Town],
    city: usize,
    office_owners: &[u8],
    alderman: u8,
    alderman_town: u8,
    merchant_count: u16,
    network: bool,
) -> Result<(), String> {
    use p3_core::ai::misc::{MessageHost, RankEnv, RankHost};
    struct Effects {
        messages: bool,
        missions: bool,
        query: bool,
    }
    impl MessageHost for Effects {
        fn post_message(&mut self, _: i32, _: &[u8; 16]) {
            self.messages = true
        }
    }
    impl RankHost for Effects {
        fn init_world(&mut self) {
            self.missions = true
        }
        fn message_pending(&mut self, _: u32, _: u32) -> bool {
            self.query = true;
            false
        }
    }
    if merchant.bytes.len() < 0x650
        || merchant.u16(8) == 0
        || city >= towns.len()
        || towns.len() > 40
        || office_owners.len() > usize::from(u16::MAX)
    {
        return Err("rango humano o datos de alcalde incompatibles".into());
    }
    let env = RankEnv {
        town_count: towns.len() as u32,
        towns,
        office_count: office_owners.len() as u16,
        offices: office_owners,
        byte_52d: alderman,
        byte_52e: alderman_town,
        word_4aa: merchant_count,
        world_ready: network,
    };
    let mut effects = Effects {
        messages: false,
        missions: false,
        query: false,
    };
    merchant.update_rank_in_town(&env, city as u32, &mut effects);
    if effects.missions || effects.messages || effects.query {
        return Err("efectos del rango electoral no conectados".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn hex(b: &[u8]) -> String {
        b.iter().map(|v| format!("{v:02x}")).collect()
    }
    #[test]
    fn ballot_matches_original() {
        let (Ok(exe), Ok(python)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else {
            return;
        };
        let (mut cases, mut actual) = (Vec::new(), Vec::new());
        for n in 0..256u32 {
            let ids = if n % 2 == 0 {
                [0xfc, 0xfd, 0xfe, 0xff]
            } else {
                [0, 1, 0xfe, 0xff]
            };
            let reps = if n % 2 == 0 {
                [80, 70, 65, 59]
            } else {
                [70 + (n % 71) as i32, 60 + ((n * 3) % 71) as i32, 65, 59]
            };
            let bribes = [(n % 4) as u8, ((n / 3) % 4) as u8, 0xff, ((n / 7) % 4) as u8];
            let tick = n * 1987;
            let old = 0xfd;
            let terms = (n % 252) as u8;
            let mut town = Town::default();
            town.set_u8(0x2c0, 0);
            town.set_u8(0x6f1, old);
            town.set_u8(0x6f3, terms);
            for (i, &v) in bribes.iter().enumerate() {
                town.set_u8(0x6dc + i, v)
            }
            let mut merchants = vec![0u8; 0x650 * 2];
            for i in 0..2usize {
                merchants[i * 0x650 + 8] = 1;
                let rep = reps[i] as f32;
                merchants[i * 0x650 + 0x2fc..i * 0x650 + 0x300].copy_from_slice(&rep.to_le_bytes());
            }
            let mut cand = [0u8; 20];
            cand[..4].copy_from_slice(&ids);
            cases.push(serde_json::json!({"town":hex(town.0.as_slice()),"merchants":hex(&merchants),"candidates":hex(&cand),"tick":tick}));
            let out = ballot(ids, reps, bribes, 2, tick, old, terms).unwrap();
            actual
                .push(serde_json::json!({"winner":out.winner,"terms":out.terms,"bribes":out.bribes,"votes":out.votes}));
        }
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_mayor_ballot.py");
        let mut child = std::process::Command::new(python)
            .arg(script)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(serde_json::json!({"binary":exe,"cases":cases}).to_string().as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let expected: Vec<serde_json::Value> = serde_json::from_slice(&out.stdout).unwrap();
        for (i, (a, b)) in actual.iter().zip(expected.iter()).enumerate() {
            assert_eq!(a, b, "caso {i}")
        }
    }

    #[test]
    fn candidates_match_original() {
        let (Ok(exe), Ok(python)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else {
            return;
        };
        let (mut cases, mut actual) = (Vec::new(), Vec::new());
        for n in 0..128usize {
            let city = n % 24;
            let mut town = Town::default();
            town.set_u8(0x2c0, city as u8);
            for i in 0..4 {
                town.set_u8(0x6d4 + i, (n + i) as u8);
                town.set_u8(0x6d8 + i, (n + 4 + i) as u8);
                town.set_u8(0x6e0 + i, (i + 1) as u8);
                town.set_u8(0x6e4 + i, (i + 4) as u8)
            }
            let mut merchants = Vec::new();
            for i in 0..8usize {
                let mut m = Merchant { bytes: vec![0; 0x650] };
                let ctrl = match (n + i) % 4 {
                    0 => 0u16,
                    1 => 4,
                    _ => 1,
                };
                m.bytes[8..10].copy_from_slice(&ctrl.to_le_bytes());
                m.bytes[0x118..0x11c].copy_from_slice(&(((n + i) % 2) as u32).to_le_bytes());
                m.bytes[0x19] = if (n + i) % 5 == 0 {
                    ((city + 1) % 24) as u8
                } else {
                    city as u8
                };
                m.bytes[0x1c] = i as u8;
                m.bytes[0x1d] = (i + 10) as u8;
                m.bytes[0x39c + city] = if (n + i) % 3 == 0 { 4 } else { 5 };
                m.bytes[0x468..0x46c]
                    .copy_from_slice(&(if (n + i) % 7 == 0 { 0u32 } else { 1u32 << city }).to_le_bytes());
                let rep = 40.0f32 + ((n * 17 + i * 11) % 81) as f32;
                m.bytes[0x2fc + city * 4..0x300 + city * 4].copy_from_slice(&rep.to_le_bytes());
                merchants.push(m);
            }
            let raw = merchants
                .iter()
                .flat_map(|m| m.bytes.iter().copied())
                .collect::<Vec<_>>();
            cases.push(serde_json::json!({"town":hex(town.0.as_slice()),"merchants":hex(&raw),"merchant_count":merchants.len(),"town_count":24}));
            let selected = candidates(&town, 24, &mut merchants).unwrap();
            let out = merchants
                .iter()
                .flat_map(|m| m.bytes.iter().copied())
                .collect::<Vec<_>>();
            actual.push(serde_json::json!({"candidates":hex(&selected),"merchants":hex(&out)}));
        }
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_mayor_candidates.py");
        let mut child = std::process::Command::new(python)
            .arg(script)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(serde_json::json!({"binary":exe,"cases":cases}).to_string().as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let expected: Vec<serde_json::Value> = serde_json::from_slice(&out.stdout).unwrap();
        for (i, (a, b)) in actual.iter().zip(expected.iter()).enumerate() {
            assert_eq!(a, b, "caso {i}")
        }
    }
}
