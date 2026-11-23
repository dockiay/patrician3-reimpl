//! Vistas privadas usadas para conciliar la escena con la base anterior.
use crate::{convoy_absorption::AbsorptionState, pirate_empty_scene_publish::{CollisionIndex,LiveState,Scene}, Sim};
use std::collections::BTreeMap;

impl Sim {
    pub(crate) fn capture_pirate_live(&self)->Result<LiveState,String>{
        let nav=self.host.navigation.as_ref().ok_or("flota pirata ausente")?;
        let factory=self.host.expedition_factory.as_ref().ok_or("factoría pirata ausente")?;
        let rng=self.host.investigation_rng.ok_or("estado CRT pirata ausente")?;
        let images=self.host.pirate_band_images.as_ref().filter(|v|v.len()==5).ok_or("bandas piratas ausentes")?;
        let mut bands=Vec::with_capacity(5);
        for (i,b) in images.iter().enumerate(){bands.push(if b.is_some(){Some(self.host.reconcile_pirate_band_image(i)?)}else{None});}
        let targets=*self.host.pirate_targets.as_ref().ok_or("destinos piratas ausentes")?;
        let heads=std::array::from_fn(|i|bands[i].as_ref().map(|b|u16::from_le_bytes([b.header[4],b.header[5]])));
        let state=AbsorptionState{bands,day:self.economy.world.day_of_year,network_client:self.economy.world.network_client,operations:vec![]};
        let mut collisions=BTreeMap::new();
        let (scene,scene_manager)=if let Some(scene)=self.host.town_attack_active.as_ref(){
            let pool=self.host.town_attack_collision.as_ref().ok_or("colisión pirata ausente")?;
            collisions.insert(scene.slot,CollisionIndex{address:scene.address,pool:pool.clone()});
            (Some(Scene{manager:scene.manager.clone(),object:scene.object.clone(),address:scene.address,saved_block:scene.original_block.clone()}),scene.manager.clone())
        }else{
            if self.host.town_attack_collision.is_some(){return Err("colisión sin escena".into())}
            (None,self.idle.battle_manager.as_ref().map(|m|m.manager.clone()).unwrap_or_else(||vec![0;0xc40]))
        };
        Ok(LiveState{nav:nav.snapshot.clone(),factory:factory.clone(),merchants:self.rep_merchants.clone(),towns:self.economy.world.towns.clone(),bands:state,scheduler:self.host.council_scheduler.clone(),operations:self.host.council_operations.clone(),rng,scene,scene_manager,collision_indices:collisions,pending_effects:self.host.town_attack_frame_effects.clone(),projected_ships:self.economy.world.ships.clone(),changed_ships:vec![],pirate_targets:targets,pirate_heads:heads})
    }
}
