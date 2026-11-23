//! Transacción de preparación de escena para una misión pirata.
use crate::convoy_absorption::AbsorptionState;
use crate::{
    pirate_empty_hideout_scene::{self, HumanInputs, Inputs},
    pirate_mission_attack_stage::Stage,
    town_attack_manager,
};
use p3_core::{
    ai::reputation::Merchant,
    ships::{pirate_mission_attack::Attack, town_attack_mask::TownProjection},
};

pub struct Resources<'a> {
    pub previous_manager: &'a [u8],
    pub heap_image: &'a [u8],
    pub address: u32,
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
    pub player: u32,
    pub dialog_active: u8,
    pub simple_text: &'a [u8],
    pub reward_format: &'a [u8],
}
pub struct AttackTransaction {
    pub attack: Attack,
    pub bands: AbsorptionState,
    pub captains: Vec<u8>,
    pub manager: town_attack_manager::Runtime,
    pub scene: pirate_empty_hideout_scene::Prepared,
}
pub enum Prepared {
    Wait(p3_core::scheduler::TaskRecord),
    Attack(AttackTransaction),
}

/// Una apertura con defensores preparada a partir de la misión viva. No ha
/// publicado gestor, flota ni tarea: deben confirmarse juntos en sesión.
pub struct PopulatedAttackTransaction {
    pub attack: Attack,
    pub bands: AbsorptionState,
    pub captains: Vec<u8>,
    pub manager: town_attack_manager::Runtime,
    pub scene: p3_save::tail::RebuiltInitialCommercialBattle,
}
pub enum PopulatedPrepared {
    Wait(p3_core::scheduler::TaskRecord),
    Attack(PopulatedAttackTransaction),
}

impl PopulatedAttackTransaction {
    /// Avanza una apertura aún no publicada. Rechaza efectos externos hasta
    /// que el consumidor pueda entregarlos junto con la escena y la flota.
    /// Devuelve si se ejecutó un tick; ante error no cambia datos ni RNG.
    pub fn advance_without_external_effects(&mut self,env:crate::battle_runtime::Environment<'_>,
        delta:u32,rng:&mut Option<p3_core::rng::CrtRand>)->Result<bool,String> {
        let mut scene=self.scene.clone();let mut ships=self.attack.ships.clone();let mut next_rng=*rng;
        let outcome=crate::pirate_active_frame::advance_populated_ai_frame(&mut scene,&mut ships,env,delta,&mut next_rng)?;
        if let Some(ref out)=outcome {
            if out.destroyed || out.manager_data.is_some() || !out.global_letters.is_empty() || !out.news.is_empty()
                || out.merchant_data.is_some() || out.fleet_data.is_some() || !out.capture_events.is_empty()
                || !out.cleared_routes.is_empty() || out.captain_pool.is_some()
                || !out.loss_notices.is_empty() || !out.reports.is_empty() || !out.alderman_reports.is_empty() {
                return Err("efectos de asalto pendientes de publicar".into());
            }
        }
        let mut manager=self.manager.clone();manager.replace_object(&scene.state.object)?;
        self.scene=scene;self.attack.ships=ships;self.manager=manager;*rng=next_rng;
        Ok(outcome.is_some())
    }
}

/// Prepara el camino de IA con un barco por bando y sin torres operativas.
/// La máscara, las listas y la colocación son Rust; la geometría es del host.
pub fn prepare_populated<F>(stage:Stage,merchants:&[Merchant],merchant_count:u16,
    town_count:u32,r:Resources<'_>,pixel_to_cell:F)->Result<PopulatedPrepared,String>
where F:FnMut(i16,i16)->(u32,u32) {
    let Stage::Attack {attack,bands,captains}=stage else {
        let Stage::Wait(task)=stage else {unreachable!()};return Ok(PopulatedPrepared::Wait(task));
    };
    if merchants.len()!=usize::from(merchant_count) || r.collision_id!=0 {
        return Err("comerciantes o primera colisión incompatibles".into());
    }
    let index=usize::from(attack.task.u8_at(0x10));
    let band=bands.bands.get(index).and_then(Option::as_ref).ok_or("banda de asalto ausente")?;
    if band.header[6]!=attack.task.u8_at(0x11) {return Err("destino de banda desincronizado".into())}
    let ship_at=usize::from(attack.leader)*0x180;
    let leader=attack.ships.get(ship_at..ship_at+0x180).ok_or("líder de asalto ausente")?;
    let convoy_at=usize::from(attack.convoy)*0x3c;
    let attacker_cv=attack.convoys.get(convoy_at..convoy_at+0x3c).ok_or("convoy atacante ausente")?;
    if leader[0]!=attack.owner || attacker_cv[0]!=attack.owner
        || u16::from_le_bytes([leader[8],leader[9]])!=attack.convoy
        || u16::from_le_bytes([attacker_cv[0xa],attacker_cv[0xb]])!=attack.head
        || u16::from_le_bytes([attacker_cv[0x10],attacker_cv[0x11]])!=attack.leader {
        return Err("vistas del atacante desincronizadas".into());
    }
    let plan=band.plan.as_deref().ok_or("plano de asalto ausente")?;
    let defender_cv=u16::from_le_bytes([band.header[8],band.header[9]]);
    let at=usize::from(defender_cv).checked_mul(0x3c).ok_or("convoy defensor inválido")?;
    let row=attack.convoys.get(at..at+0x3c).ok_or("convoy defensor ausente")?;
    let defender=u16::from_le_bytes([row[0xa],row[0xb]]);
    let mut manager=town_attack_manager::Runtime::allocate(r.previous_manager,r.heap_image,r.address)?;
    let raw=merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();
    let scene=crate::pirate_active_frame::prepare_populated_opening(
        crate::pirate_active_frame::PopulatedStartInput{
            object:&manager.object,manager:&attack.manager,world_ships:&attack.ships,
            convoys:&attack.convoys,captains:&captains,merchants:&raw,plan,address:r.address,
            town_count,attacker:attack.leader,defender,tick:r.tick,
        },index,band,r.position,r.portrait,r.projection,pixel_to_cell)?;
    manager.replace_object(&scene.state.object)?;
    Ok(PopulatedPrepared::Attack(PopulatedAttackTransaction {attack,bands,captains,manager,scene}))
}
/// Las entradas proceden del asignador, vista, banco de nombres y recursos
/// reales del anfitrión. Solo prepara; la publicación de flota, mensajes,
/// gestor y tarea requiere una única transacción externa.
pub fn prepare(
    stage: Stage,
    merchants: &[Merchant],
    merchant_count: u16,
    r: Resources<'_>,
) -> Result<Prepared, String> {
    let Stage::Attack { attack, bands, captains } = stage else {
        let Stage::Wait(task) = stage else { unreachable!() };
        return Ok(Prepared::Wait(task));
    };
    let ship = usize::from(attack.leader) * 0x180;
    let owner = *attack.ships.get(ship).ok_or("líder ausente")?;
    if usize::from(owner) >= usize::from(merchant_count) {
        return Err("dueño atacante ausente".into());
    }
    let person = merchants
        .get(usize::from(owner))
        .ok_or("comerciante atacante ausente")?;
    if person.bytes.len() < 0x650 {
        return Err("comerciante atacante truncado".into());
    }
    let human = person.u16(8) == 0;
    let mut manager = town_attack_manager::Runtime::allocate(r.previous_manager, r.heap_image, r.address)?;
    let raw = merchants
        .iter()
        .flat_map(|p| p.bytes.iter().copied())
        .collect::<Vec<_>>();
    let inputs = Inputs {
        attack: &attack,
        bands: &bands,
        object: &manager.object,
        merchants: &raw,
        merchant_count,
        tick: r.tick,
        position: r.position,
        portrait: r.portrait,
        projection: r.projection,
        collision_id: r.collision_id,
        fallback_town: r.fallback_town,
        report_flag: r.report_flag,
        first_name: r.first_name,
        last_name: r.last_name,
        format: r.format,
        own_separator: r.own_separator,
        enemy_separator: r.enemy_separator,
    };
    let scene = if human {
        pirate_empty_hideout_scene::prepare_human(
            inputs,
            HumanInputs {
                player: u16::try_from(r.player).map_err(|_| "jugador humano fuera de rango")?,
                battle_address: r.address,
                dialog_active: r.dialog_active,
                simple_text: r.simple_text,
                reward_format: r.reward_format,
            },
        )?
    } else {
        pirate_empty_hideout_scene::prepare(inputs)?
    };
    manager.replace_object(&scene.object)?;
    Ok(Prepared::Attack(AttackTransaction {
        attack,
        bands,
        captains,
        manager,
        scene,
    }))
}
