//! Publicación transaccional de B4 en las vistas de sesión.
use crate::{convoy_absorption::AbsorptionState, pirate_b4_live::{self, Live}, pirate_b4_stage::Resources, Sim};
use p3_core::{economy::{pirate_group_callbacks::CaptainResources, ship_trade::ShipCargo}, pump::Operation};

fn word(b: &[u8], at: usize) -> u16 { u16::from_le_bytes([b[at], b[at+1]]) }
fn dword(b: &[u8], at: usize) -> u32 { u32::from_le_bytes(b[at..at+4].try_into().unwrap()) }
fn cargo_matches(c: &ShipCargo, s: &[u8]) -> bool {
    c.owner == s[0] && c.town == s[0x39] && c.kind == s[0xe] && c.capacity == dword(s,0x10) as i32
    && c.dirty == word(s,0x3c) && c.crew == word(s,0x40) && c.cutlasses == word(s,0x154) && c.marines == s[0x156]
    && (0..24).all(|i| c.stock[i] == dword(s,0x54+i*4) as i32 && c.average_price[i].to_bits() == dword(s,0xb4+i*4) && c.guns[i] == s[0x13c+i])
    && c.cargo_value == dword(s,0x114) as i32 && c.cargo_weight == dword(s,0x118) as i32 && c.equipment_weight == dword(s,0x11c) as i32 && c.gun_power == dword(s,0x120) as i32
}
impl Sim {
    /// B4 debe seguir en cabeza hasta que todas las proyecciones se validen.
    /// Las vistas se publican juntas; no modifica el reloj ni despacha la siguiente operación.
    pub(crate) fn dispatch_live_pirate_b4(&mut self, expected: Operation) -> Result<(), String> {
        if expected.0[0] != 0xb4 || self.host.council_operations.first() != Some(&expected) { return Err("cola B4 cambió".into()); }
        if expected.0[1] >= 5 { self.host.council_operations.remove(0); return Ok(()); }
        let nav = self.host.navigation.as_ref().ok_or("navegación B4 ausente")?;
        let factory = self.host.expedition_factory.as_ref().ok_or("factoría B4 ausente")?;
        let images = self.host.pirate_band_images.as_ref().filter(|v|v.len()==5).ok_or("bandas B4 ausentes")?;
        if self.host.pirate_band_images_initial.as_ref().is_none_or(|v|v.len()!=5) { return Err("base de bandas B4 ausente".into()); }
        let targets = self.host.pirate_targets.as_ref().ok_or("destinos B4 ausentes")?;
        let missions = self.host.mission_pirates.as_ref().ok_or("misiones de banda B4 ausentes")?;
        let mut bands = Vec::with_capacity(5);
        for (i, band) in images.iter().enumerate() {
            let current = if band.is_some() { Some(self.host.reconcile_pirate_band_image(i)?) } else { None };
            let target = current.as_ref().map(|b|b.header[6]);
            if targets[i] != target || missions[i].as_ref().map(|m|m.target) != target { return Err("vistas de banda B4 desincronizadas".into()); }
            bands.push(current);
        }
        let ns = nav.snapshot.ships.len()/0x180;
        let nv = nav.snapshot.convoys.len()/0x3c;
        if nav.snapshot.ships.len()%0x180 != 0 || self.economy.world.ships.len()!=ns || usize::from(self.economy.world.ship_count)!=ns || self.economy.cargo_ships.len()!=ns || self.host.expedition_ship_images.len()!=ns || self.host.expedition_original_ship_images.len()!=ns || self.host.debt_ship_metadata.len()!=ns || nav.snapshot.convoys.len()%0x3c!=0 || self.host.expedition_convoy_images.len()!=nv || self.host.debt_convoy_flags.len()!=nv || self.economy.merchants.len()!=self.rep_merchants.len() || self.economy.world.merchants.len()!=self.rep_merchants.len() || usize::from(self.economy.world.merchant_count)!=self.rep_merchants.len() { return Err("proyecciones B4 incompletas".into()); }
        let mut location = *self.host.settlement_locations.as_ref().ok_or("posiciones B4 ausentes")?;
        let sea = self.host.settlement_sea_directions.as_ref().ok_or("orientaciones del mar B4 ausentes")?;
        // 0x513DB2 lee TownData+0x31 (orientación del mar), no +0x30 (región).
        for (loc, &direction) in location.iter_mut().zip(sea) { loc.kind = direction; }
        let occupied = self.host.settlement_inland.as_ref().ok_or("ocupación B4 ausente")?;
        let resources = self.host.expedition_resources.as_ref().ok_or("nombres de barco B4 ausentes")?;
        let (first_names,last_names) = self.host.pirate_captain_name_counts().ok_or("nombres de capitán B4 ausentes")?;
        let town0 = self.economy.world.towns.first().ok_or("ciudad 0 B4 ausente")?;
        let alderman = self.economy.world.alderman_town;
        let weapon = self.economy.world.towns.get(usize::from(alderman)).ok_or("ciudad Ältermann B4 ausente")?.u32(0x810);
        let doy = self.economy.world.day_of_year;
        let population = self.economy.world.towns.get(usize::from(doy & 15)).ok_or("población de capitán B4 ausente")?.u32(0x2e0);
        let captain = CaptainResources { tick:self.idle.clock.tick, day_of_year:doy, day:self.idle.clock.calendar.day, population, first_names, last_names };
        let manager = &nav.snapshot.manager;
        if manager.len()!=0x110 { return Err("gestor B4 truncado".into()); }
        // Identidad lógica exclusiva: nunca se guarda como puntero x86.
        let token = (0x73000000u32..0x73010000).step_by(4).find(|v| (0..5).all(|i|dword(manager,12+4*i)!=*v)).ok_or("identidad B4 agotada")?;
        let r = Resources { locations:&location, town_order:&self.host.expedition_town_ids, occupied, piece_bytes:self.host.town_pirate_piece_bytes.ok_or("bytes B4 ausentes")?, town0, alderman_town:alderman, weapon_experience:weapon, port0:(location[0].x as u16,location[0].y as u16), ship_names:&resources.ship_names, captain, new_band_token:token };
        let state = AbsorptionState { bands, day:doy, network_client:self.economy.world.network_client, operations:vec![] };
        let live = Live { nav:nav.clone(), factory:factory.clone(), bands:state, towns:self.economy.world.towns.clone(), merchants:self.rep_merchants.clone(), pool:self.host.route_stop_runtime.clone().ok_or("depósito B4 ausente")?, groups:self.host.merchant_ship_groups.clone(), operations:self.host.council_operations.clone(), changed_groups:self.host.merchant_ship_groups_changed.clone() };
        let staged = pirate_b4_live::prepare(&live,expected,&r,self.idle.clock.tick,self.economy.world.local_merchant)?;
        let total = staged.nav.snapshot.ships.len()/0x180;
        if total < ns || total > usize::from(u16::MAX) { return Err("crecimiento B4 inválido".into()); }
        // Las imágenes de otros sistemas deben coincidir en los bytes que se van a tocar.
        for i in 0..ns {
            let old=&nav.snapshot.ships[i*0x180..(i+1)*0x180]; let new=&staged.nav.snapshot.ships[i*0x180..(i+1)*0x180];
            if old==new { continue; }
            let active=word(old,0x134)<0x17;
            let projection=&self.economy.world.ships[i];
            if projection.state!=word(old,0x134) || projection.owner!=if active {old[0]} else {255} { return Err("proyección naval B4 desincronizada".into()); }
            if active && !cargo_matches(&self.economy.cargo_ships[i],old) { return Err("carga naval B4 desincronizada".into()); }
            if active {
                let image=self.host.expedition_ship_images[i].as_ref().ok_or("imagen naval B4 ausente")?;
                if image.len()!=0x180 || (0..0x180).any(|j|old[j]!=new[j] && image[j]!=old[j] && image[j]!=new[j]) { return Err("imagen naval B4 desincronizada".into()); }
            }
        }
        for i in 0..nv {
            let old=&nav.snapshot.convoys[i*0x3c..(i+1)*0x3c]; let new=&staged.nav.snapshot.convoys[i*0x3c..(i+1)*0x3c];
            let image=&self.host.expedition_convoy_images[i];
            if image.len()!=0x3c || (0..0x3c).any(|j|old[j]!=new[j] && image[j]!=old[j] && image[j]!=new[j]) { return Err("imagen de convoy B4 desincronizada".into()); }
        }
        for i in 0..self.rep_merchants.len() {
            let old=&self.rep_merchants[i].bytes; let new=&staged.merchants[i].bytes; let other=&self.economy.merchants[i].bytes;
            if old.len()!=0x650 || other.len()!=0x650 || new.len()!=0x650 || (0..0x650).any(|j|old[j]!=new[j] && other[j]!=old[j] && other[j]!=new[j]) { return Err("comerciante B4 desincronizado".into()); }
        }
        let mut old_nav=nav.snapshot.clone();
        for _ in ns..total { let mut old=vec![0;0x180];old[0x134..0x136].copy_from_slice(&0xffu16.to_le_bytes());old_nav.ships.extend_from_slice(&old); }
        // Tras este punto no hay operaciones fallibles: se confirma todo junto.
        self.economy.world.ship_count=total as u16;
        self.economy.world.ships.resize_with(total,||p3_core::economy::world::Ship{owner:255,state:255});
        self.economy.cargo_ships.resize_with(total,ShipCargo::default);
        self.host.expedition_ship_images.resize_with(total,||None);
        self.host.expedition_original_ship_images.resize_with(total,||None);
        self.host.debt_ship_metadata.resize(total,(u16::MAX,255,0,u16::MAX));
        for i in 0..self.rep_merchants.len() { for j in 0..0x650 { if self.rep_merchants[i].bytes[j]!=staged.merchants[i].bytes[j] { self.economy.merchants[i].bytes[j]=staged.merchants[i].bytes[j]; } } }
        self.rep_merchants=staged.merchants;
        self.economy.world.towns=staged.towns;
        self.host.expedition_factory=Some(staged.factory);
        self.host.navigation=Some(staged.nav);
        self.host.route_stop_runtime=Some(staged.pool);
        self.host.merchant_ship_groups=staged.groups;
        self.host.merchant_ship_groups_changed=staged.changed_groups;
        self.host.pirate_band_images=Some(staged.bands.bands.clone());
        self.host.pirate_targets=Some(std::array::from_fn(|i|staged.bands.bands[i].as_ref().map(|b|b.header[6])));
        self.host.mission_pirates=Some(std::array::from_fn(|i|staged.bands.bands[i].as_ref().map(|b|p3_core::ai::settlement::MissionPirateBand {first_ship:word(&b.header,4),target:b.header[6],status:b.header[12],reserve_strength:word(&b.header,14)})));
        if let Some(p)=self.host.hanse_pirates.as_mut() {
            p.heads=std::array::from_fn(|i|staged.bands.bands[i].as_ref().map(|b|word(&b.header,4)));
            let current=&self.host.navigation.as_ref().unwrap().snapshot;
            p.next=current.ships.chunks_exact(0x180).map(|s|word(s,4)).collect();
            p.ship_captains=current.ships.chunks_exact(0x180).map(|s|word(s,0x42)).collect();
            p.captains=current.captains.chunks_exact(16).map(|c|c.try_into().unwrap()).collect();
        }
        self.host.council_operations=staged.operations;
        self.host.publish_navigation_delta(&old_nav,&mut self.economy.world,&mut self.economy.cargo_ships);
        for i in 0..total { let s=&self.host.navigation.as_ref().unwrap().snapshot.ships[i*0x180..(i+1)*0x180]; if word(s,0x134)>=0x17 { self.economy.world.ships[i].owner=255; } }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_save::{model::SaveModel,pirate_bands,ships::Ships};
    use p3_core::ai::settlement::SettlementLocation;
    #[test]
    fn live_b4_publishes_and_saves_atomically() {
        let Ok(root)=std::env::var("P3_SAVES") else {return};
        let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let mut initial=SaveModel::parse(&raw).unwrap();
        let bands=pirate_bands::read(&initial.ships.tail).unwrap();
        pirate_bands::replace_optional(&mut initial.ships.tail,0,bands[0].as_ref(),None).unwrap();
        let file=initial.write();
        let mut sim=Sim::from_save_bytes(&file).unwrap();
        let snapshot=Ships::parse(&initial.ships.write()).unwrap().navigation_snapshot().unwrap();
        let ports=vec![(100,500);sim.economy.world.towns.len()];
        sim.host.navigation=Some(crate::navigation::LiveVoyage::idle(snapshot,ports,0));
        let mut locations=[SettlementLocation{x:0,y:0,kind:0};40];
        for (i,l) in locations.iter_mut().enumerate() { *l=SettlementLocation{x:100+i as i32,y:500+i as i32,kind:(i%6)as u8}; }
        sim.host.settlement_locations=Some(locations);
        sim.host.settlement_sea_directions=Some(std::array::from_fn(|i| locations[i].kind));
        sim.host.settlement_inland=Some([0;40]);
        sim.host.town_pirate_piece_bytes=Some((0x5a,0xa5));
        sim.host.expedition_resources=Some(crate::HanseFleetResources {ship_names:vec![b"Alpha".to_vec(),b"Brig".to_vec(),b"Crown".to_vec()],first_names:75,last_names:80,ports:vec![Some((100,500));sim.economy.world.towns.len()],hotseat:false,player:0});
        let op=Operation([0xb4,0,3,12345,67890]);
        sim.host.council_operations.push(op);
        assert!(sim.persist_all(&mut initial.clone()).unwrap_err().contains("operaciones pendientes"));
        let before=sim.host.navigation.as_ref().unwrap().snapshot.clone();
        assert!(sim.dispatch_live_pirate_b4(Operation([0xb4,1,0,0,0])).is_err());
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot,before);
        let later=Operation([0xb5,0,0,0,0]);
        sim.host.council_operations.push(later);
        let locations=sim.host.settlement_locations.take();
        let errors=sim.dispatch_pending_council_operations();
        assert_eq!(errors.len(),1);
        assert_eq!(sim.host.council_operations,vec![op,later]);
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot,before);
        sim.host.council_operations.pop();
        sim.host.settlement_locations=locations;
        assert!(sim.dispatch_pending_council_operations().is_empty());
        assert!(sim.host.council_operations.is_empty());
        let band=sim.host.pirate_band_images.as_ref().unwrap()[0].clone();
        assert!(band.is_some());
        let mut saved=initial.clone();
        sim.persist_all(&mut saved).unwrap();
        let restored=Sim::from_save_bytes(&saved.write()).unwrap();
        assert_eq!(restored.host.pirate_band_images.as_ref().unwrap()[0],band);
    }
}
