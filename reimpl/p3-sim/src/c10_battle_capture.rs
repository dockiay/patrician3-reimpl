//! Publication of synchronous pirate capture effects in the C10 session.
use crate::{Sim,pirate_capture::PirateCaptureState};
use p3_save::ships::{NavigationSnapshot,SavedRoute};
fn word(b:&[u8],at:usize)->u16{u16::from_le_bytes([b[at],b[at+1]])}
/// 0x503B00 para cada convoy: tarea 0x34 en fase >= 7 cuyo `+0x16` es el convoy y a la que apunta
/// `convoy+2` (con la ranura dentro de la capacidad del planificador).
pub fn hunt_tasks_of(sched:&p3_core::scheduler::Scheduler,convoys:&[Vec<u8>])->Vec<(usize,p3_core::scheduler::TaskRecord)>{
    sched.records.iter().enumerate().filter(|(i,r)|r.kind==0x34&&r.u8_at(0x15)>=7&&*i<usize::from(sched.capacity)&&convoys.get(usize::from(u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)]))).is_some_and(|c|usize::from(word(c,2))==*i)).map(|(i,r)|(i,*r)).collect()
}
#[cfg(test)]
mod hunt_tests{
    use super::*;
    #[test]
    fn convoy_must_point_back_at_the_hunt_task(){
        let mut s=p3_core::scheduler::Scheduler::new();
        let a=s.add_task(10);let b=s.add_task(10);
        for (slot,convoy) in [(a,3u16),(b,4u16)]{let r=&mut s.records[usize::from(slot)];r.kind=0x34;r.set_u8_at(0x15,7);r.set_u8_at(0x16,convoy as u8);r.set_u8_at(0x17,0);}
        let mut convoys=vec![vec![0u8;0x3c];6];
        convoys[3][2..4].copy_from_slice(&a.to_le_bytes());
        convoys[4][2..4].copy_from_slice(&a.to_le_bytes()); // apunta a otra tarea: no es la caza
        let got:Vec<usize>=hunt_tasks_of(&s,&convoys).into_iter().map(|(i,_)|i).collect();
        assert_eq!(got,vec![usize::from(a)]);
        s.records[usize::from(a)].set_u8_at(0x15,6);
        assert!(hunt_tasks_of(&s,&convoys).is_empty());
    }
}
impl Sim {
    /// Same inputs as the B4 session (0x4FDF50): population of town `day_of_year & 15`.
    pub fn capture_captain_resources(&self)->Option<p3_core::economy::pirate_group_callbacks::CaptainResources>{
        let (first_names,last_names)=self.host.pirate_captain_name_counts()?;let doy=self.economy.world.day_of_year;
        let population=self.economy.world.towns.get(usize::from(doy&15))?.u32(0x2e0);
        Some(p3_core::economy::pirate_group_callbacks::CaptainResources{tick:self.idle.clock.tick,day_of_year:doy,day:self.idle.clock.calendar.day,population,first_names,last_names})
    }
    pub fn prepare_battle_capture(&self)->Result<PirateCaptureState,String>{
        let resources=self.host.expedition_resources.as_ref().ok_or("recursos de captura ausentes")?;
        let bands=self.host.pirate_band_images.as_ref().ok_or("bandas de captura ausentes")?;
        if bands.len()!=5{return Err("bandas de captura incompletas".into())}
        let missions=self.host.mission_pirates.as_ref().ok_or("misiones piratas ausentes")?;
        let portraits=self.host.hanse_pirates.as_ref().ok_or("vistas piratas ausentes")?;
        for i in 0..5 {
            if bands[i].is_some()!=missions[i].is_some()||portraits.heads[i]!=missions[i].map(|m|m.first_ship){return Err("cabezas de bandas desincronizadas antes de captura".into())}
        }
        let bands=bands.iter().enumerate().map(|(i,b)|if b.is_some(){self.host.reconcile_pirate_band_image(i).map(Some)}else{Ok(None)}).collect::<Result<Vec<_>,String>>()?;
        Ok(PirateCaptureState{bands,factory:self.host.expedition_factory.clone().ok_or("fábrica de captura ausente")?,towns:self.economy.world.towns.clone(),ship_names:resources.ship_names.clone(),ports:resources.ports.iter().map(|p|p.ok_or_else(||"puerto de captura ausente".to_owned())).collect::<Result<Vec<_>,String>>()?,locations:self.host.settlement_locations.as_ref().ok_or("lugares piratas de captura ausentes")?.iter().map(|l|(l.x as u16,l.y as u16)).collect(),tick:self.idle.clock.tick,news_stamp:self.host.battle_news_stamp,news:Vec::new(),day_of_year:self.idle.clock.calendar.day_of_year,day:self.economy.world.alderman_town,captain:self.capture_captain_resources(),hunt_convoys:{let f=&self.host.expedition_factory.as_ref().ok_or("fábrica de captura ausente")?.convoys;hunt_tasks_of(&self.host.council_scheduler,f).into_iter().map(|(_,r)|u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)])).collect()},hunt_tasks:{let f=&self.host.expedition_factory.as_ref().ok_or("fábrica de captura ausente")?.convoys;hunt_tasks_of(&self.host.council_scheduler,f).into_iter().map(|(i,r)|(i as u16,r)).collect()},hunt_added:Vec::new()})
    }
    pub fn validate_battle_capture_publication(&self,state:&PirateCaptureState,after:&NavigationSnapshot)->Result<(),String>{
        let nav=self.host.navigation.as_ref().ok_or("flota de captura ausente")?;
        if after.ships.len()%0x180!=0||after.ships.len()<nav.snapshot.ships.len()||after.routes.len()!=after.ships.len()/0x180||after.manager.len()<0xf8||usize::from(word(&after.manager,0xf4))!=after.routes.len()||state.bands.len()!=5{return Err("flota final de captura incompatible".into())}
        let missions=self.host.mission_pirates.as_ref().ok_or("misiones piratas ausentes")?;
        let portraits=self.host.hanse_pirates.as_ref().ok_or("vistas piratas ausentes")?;
        for i in 0..5{if missions[i].is_some()!=state.bands[i].is_some()||portraits.heads[i].is_some()!=state.bands[i].is_some(){return Err("presencia de bandas desincronizada".into())}if let Some(b)=&state.bands[i]{b.validate()?;}}
        Ok(())
    }
    /// Call only after validate and the rest of the session preflight succeeds.
    pub fn commit_battle_capture(&mut self,mut state:PirateCaptureState,after:&NavigationSnapshot){
        let nav=self.host.navigation.as_mut().expect("flota validada");let old_count=nav.snapshot.ships.len()/0x180;let count=after.ships.len()/0x180;
        for (i,image) in after.ships.chunks_exact(0x180).enumerate(){
            let allocated=word(image,0x134)<0x17&&(i>=old_count||word(&nav.snapshot.ships,i*0x180+0x134)>=0x17);
            if i>=old_count{nav.baseline.ships.extend_from_slice(image);nav.baseline.routes.push(if word(image,0x134)<0x17{Some(SavedRoute::None)}else{None});}
            else if allocated{nav.baseline.ships[i*0x180..(i+1)*0x180].copy_from_slice(image);nav.baseline.routes[i]=Some(SavedRoute::None);}
        }
        nav.baseline.manager[0xf4..0xf6].copy_from_slice(&(count as u16).to_le_bytes());
        self.economy.world.ship_count=count as u16;
        self.economy.world.ships.resize_with(count,||p3_core::economy::world::Ship{owner:255,state:255});
        self.economy.cargo_ships.resize_with(count,p3_core::economy::ship_trade::ShipCargo::default);
        self.host.expedition_ship_images.resize_with(count,||None);self.host.expedition_original_ship_images.resize_with(count,||None);self.host.debt_ship_metadata.resize(count,(u16::MAX,255,0,u16::MAX));
        for(i,image)in after.ships.chunks_exact(0x180).enumerate(){if i>=old_count||word(&nav.snapshot.ships,i*0x180+0x134)>=0x17&&word(image,0x134)<0x17{self.host.expedition_ship_images[i]=(word(image,0x134)<0x17).then(||image.to_vec());}}
        state.factory.ships=after.ships.chunks_exact(0x180).map(<[u8]>::to_vec).collect();state.factory.convoys=after.convoys.chunks_exact(0x3c).map(<[u8]>::to_vec).collect();state.factory.captains=after.captains.chunks_exact(16).map(|c|c.try_into().unwrap()).collect();
        state.factory.free_ship=word(&after.manager,0xe2);state.factory.free_captain=word(&after.manager,0xe0);state.factory.free_convoy=word(&after.manager,0xe4);state.factory.convoy_head=word(&after.manager,0xec);state.factory.name_cursor=word(&after.manager,0xee);state.factory.name_step=word(&after.manager,0xf0);
        let missions=self.host.mission_pirates.as_mut().expect("misiones validadas");let portraits=self.host.hanse_pirates.as_mut().expect("vistas validadas");
        for i in 0..5{if let(Some(m),Some(b))=(missions[i].as_mut(),state.bands[i].as_ref()){m.first_ship=word(&b.header,4);m.target=b.header[6];m.status=b.header[12];m.reserve_strength=word(&b.header,14);portraits.heads[i]=Some(m.first_ship);}}
        portraits.next=state.factory.ships.iter().map(|s|word(s,4)).collect();portraits.ship_captains=state.factory.ships.iter().map(|s|word(s,0x42)).collect();portraits.captains=state.factory.captains.clone();
        self.host.battle_news_stamp=state.news_stamp;self.host.battle_pirate_news.extend(state.news);
        self.host.expedition_factory=Some(state.factory);self.host.pirate_band_images=Some(state.bands);
    }
}
