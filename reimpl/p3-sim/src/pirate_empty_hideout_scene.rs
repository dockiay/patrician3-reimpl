//! Atomic preparation of a pirate empty-hideout scene and its finish.
use crate::convoy_absorption::AbsorptionState;
use p3_core::ships::{
    pirate_mission_attack::Attack,
    town_attack_finish::{
        self, EmptyHideoutFinish, FinishMessage, HumanFinishEffect, HumanHideoutFinish,
    },
    town_attack_mask::TownProjection,
    town_attack_start::{self, EmptyHideoutEffect, EmptyHideoutStart},
};
const SHIP: usize = 0x180;
const CONVOY: usize = 0x3c;
fn r16(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([b[p], b[p + 1]])
}
fn r32(b: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(b[p..p + 4].try_into().unwrap())
}

/// 0x5FC31C recorre las piezas desde el índice cero y solo incorpora al
/// asalto las torres 0x30/0x31 que 0x515D30 considera operativas.
pub(crate) fn has_operational_towers(band: &p3_save::pirate_bands::PirateBand) -> Result<bool, String> {
    let count = usize::from(r16(&band.header, 0));
    let mut visited = vec![false; count];
    let mut id = 0usize;
    while id < count {
        if visited[id] { return Err("cadena de piezas piratas cíclica".into()); }
        visited[id] = true;
        let at = id * 8;
        if matches!(band.pieces[at + 3], 0x30 | 0x31) && band.pieces[at + 6] < 200 {
            return Ok(true);
        }
        id = usize::from(r16(&band.pieces, at + 4));
    }
    Ok(false)
}

#[cfg(test)]
mod tower_tests {
    use super::has_operational_towers;
    #[test]
    fn follows_pieces_not_the_unrelated_ship_head() {
        let mut band = p3_save::pirate_bands::PirateBand { header: [0;16], plan: None, pieces: vec![0;24] };
        band.header[..2].copy_from_slice(&3u16.to_le_bytes());
        band.header[4..6].copy_from_slice(&0u16.to_le_bytes());
        band.pieces[3] = 0x24;
        band.pieces[4..6].copy_from_slice(&2u16.to_le_bytes());
        band.pieces[19] = 0x31;
        band.pieces[22] = 200;
        band.pieces[20..22].fill(255);
        assert_eq!(has_operational_towers(&band), Ok(false));
        band.pieces[22] = 199;
        assert_eq!(has_operational_towers(&band), Ok(true));
        band.pieces[22] = 200;
        band.pieces[20..22].copy_from_slice(&0u16.to_le_bytes());
        assert!(has_operational_towers(&band).unwrap_err().contains("cíclica"));
    }
}

pub struct Inputs<'a> {
    pub attack: &'a Attack,
    pub bands: &'a AbsorptionState,
    pub object: &'a [u8],
    pub merchants: &'a [u8],
    pub merchant_count: u16,
    pub tick: u32,
    pub position: (u16, u16),
    pub portrait: (u8, u8),
    pub projection: TownProjection,
    pub collision_id: u16,
    pub fallback_town: u8,
    pub report_flag: u8,
    pub first_name: &'a [u8],
    pub last_name: &'a [u8],
    pub format: &'a [u8],
    pub own_separator: &'a [u8],
    pub enemy_separator: &'a [u8],
}
pub struct HumanInputs<'a> {
    pub player: u16,
    pub battle_address: u32,
    pub dialog_active: u8,
    pub simple_text: &'a [u8],
    pub reward_format: &'a [u8],
}
#[derive(Debug, Clone)]
pub struct Prepared {
    pub object: Vec<u8>,
    pub merchants: Vec<u8>,
    pub start_effects: Vec<EmptyHideoutEffect>,
    pub letters: Vec<FinishMessage>,
    pub human_finish_effects: Vec<HumanFinishEffect>,
    pub dialog_active: Option<u8>,
    pub convoy_ship_owners: Vec<u8>,
    pub attacker_count: u16,
}

/// Exige escena nueva, convoy atacante existente y banda sin piezas activas
/// ni barcos defensores. Devuelve copias; su publicaciÃ³n y S05 son del caller.
pub fn prepare(v: Inputs<'_>) -> Result<Prepared, String> {
    prepare_inner(v, None)
}
/// Variante humana; conserva el byte de diálogo y la secuencia de efectos.
pub fn prepare_human(v: Inputs<'_>, human: HumanInputs<'_>) -> Result<Prepared, String> {
    prepare_inner(v, Some(human))
}
fn prepare_inner(v: Inputs<'_>, human_input: Option<HumanInputs<'_>>) -> Result<Prepared, String> {
    let attack = v.attack;
    if attack.manager.len() < 0xf8 {
        return Err("gestor naval truncado".into());
    }
    let ship_count = usize::from(r16(&attack.manager, 0xf4));
    let convoy_count = r16(&attack.manager, 0xf6);
    if attack.ships.len() < ship_count * SHIP
        || attack.convoys.len() < usize::from(convoy_count) * CONVOY
    {
        return Err("flota truncada".into());
    }
    if attack.convoy >= convoy_count {
        return Err("convoy atacante ausente".into());
    }
    let cp = usize::from(attack.convoy) * CONVOY;
    let head = r16(&attack.convoys, cp + 0xa);
    let leader = r16(&attack.convoys, cp + 0x10);
    if attack.head != head
        || attack.leader != leader
        || attack.owner != attack.convoys[cp]
        || usize::from(leader) >= ship_count
    {
        return Err("vistas de convoy atacante inconsistentes".into());
    }
    let owner = attack.ships[usize::from(leader) * SHIP];
    if usize::from(owner) >= usize::from(v.merchant_count)
        || v.merchants.len() < usize::from(v.merchant_count) * 0x650
    {
        return Err("propietario atacante ausente".into());
    }
    let human = r16(v.merchants, usize::from(owner) * 0x650 + 8) == 0;
    match (&human_input, human) {
        (None, true) => return Err("recursos de victoria humana ausentes".into()),
        (Some(_), false) => return Err("atacante no humano".into()),
        (Some(resources), true) if resources.player != u16::from(owner) => {
            return Err("jugador humano incompatible".into())
        }
        _ => (),
    }
    let mut id = head;
    let mut seen = vec![false; ship_count];
    let mut owners = Vec::new();
    let mut has_leader = false;
    while usize::from(id) < ship_count {
        let n = usize::from(id);
        if seen[n] {
            return Err("convoy atacante cÃ­clico".into());
        }
        seen[n] = true;
        let at = n * SHIP;
        if r16(&attack.ships, at + 8) != attack.convoy {
            return Err("barco en convoy incompatible".into());
        }
        owners.push(attack.ships[at]);
        has_leader |= id == leader;
        id = r16(&attack.ships, at + 6)
    }
    if !has_leader || owners.is_empty() || owners.len() > usize::from(u16::MAX) {
        return Err("lÃ­der ausente de convoy atacante".into());
    }
    let band_index = usize::from(attack.task.u8_at(0x10));
    if band_index >= 5 || v.bands.bands.len() != 5 {
        return Err("Ã­ndice de banda invÃ¡lido".into());
    }
    let band = v.bands.bands[band_index].as_ref().ok_or("banda ausente")?;
    band.validate()?;
    if has_operational_towers(band)? {
        return Err("guarida con torres operativas no soportada".into());
    }
    let defender = r16(&band.header, 8);
    if defender >= convoy_count {
        return Err("convoy defensor ausente".into());
    }
    if defender == attack.convoy {
        return Err("convoy atacante y defensor coinciden".into());
    }
    if usize::from(r16(&attack.convoys, usize::from(defender) * CONVOY + 0xa)) < ship_count {
        return Err("guarida con defensores no soportada".into());
    }
    let plan = band.plan.as_deref().ok_or("plano de guarida ausente")?;
    if plan.len() < 0x64 {
        return Err("plano de guarida truncado".into());
    }
    let rows = r32(plan, 0);
    let columns = r32(plan, 4);
    let level = r16(&band.header, 14);
    let mut object = v.object.to_vec();
    let start = EmptyHideoutStart {
        target: 254 - band_index as u8,
        attacker: leader,
        convoy: Some(attack.convoy),
        convoy_head: head,
        attacker_count: owners.len() as u16,
        defender_convoy: Some(defender),
        owner,
        human,
        tick: v.tick,
        position: v.position,
        portrait: v.portrait,
        plan_width: rows,
        plan_height: columns,
        cells: &plan[0x64..],
        projection: v.projection,
        collision_id: v.collision_id,
    };
    let mut start_effects = town_attack_start::prepare_empty_hideout_start(&mut object, &start)?;
    if start_effects.pop() != Some(EmptyHideoutEffect::Finish(0)) {
        return Err("cierre de escena ausente".into());
    }
    let mut merchants = v.merchants.to_vec();
    let finish = EmptyHideoutFinish {
        defender_convoy_empty: true,
        convoy_owner: attack.convoys[cp],
        convoy_count,
        convoy_ship_owners: &owners,
        band_level: level,
        fallback_town: v.fallback_town,
        report_flag: v.report_flag,
        first_name: v.first_name,
        last_name: v.last_name,
        format: v.format,
        own_separator: v.own_separator,
        enemy_separator: v.enemy_separator,
    };
    let (letters, human_finish_effects, dialog_active) = if let Some(human) = human_input {
        let outcome = town_attack_finish::prepare_human_hideout_finish(
            &mut object,
            &mut merchants,
            v.merchant_count,
            &HumanHideoutFinish {
                base: finish,
                player: human.player,
                battle_address: human.battle_address,
                dialog_active: human.dialog_active,
                simple_text: human.simple_text,
                reward_format: human.reward_format,
            },
        )?;
        (Vec::new(), outcome.effects, Some(outcome.dialog_active))
    } else {
        (
            town_attack_finish::prepare_empty_hideout_finish(
                &mut object,
                &mut merchants,
                v.merchant_count,
                &finish,
            )?,
            Vec::new(),
            None,
        )
    };
    Ok(Prepared {
        object,
        merchants,
        start_effects,
        letters,
        human_finish_effects,
        dialog_active,
        convoy_ship_owners: owners,
        attacker_count: seen.into_iter().filter(|x| *x).count() as u16,
    })
}
