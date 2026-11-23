//! Conexión transaccional del avance naval al estado reconstruido.
//! Las ramas que todavía necesitan interfaz, disparo o resolución se rechazan.
use p3_core::{rng::CrtRand,ships::{boarding::{Boarding,BoardingHost,Captains},collide::{PairHost,Slots},projectile::{self,Advance,ProjectileTable,ShotContext},sea_update::{self,Globals,SeaCtx,UpdateHost},spatial::CollisionIndex,weapon_damage::{self,DamageEvent},weapons::{self,SelectView,WeaponPlan},volley::{self,Volley}}};
use p3_save::tail::RebuiltInitialCommercialBattle;
use crate::battle_spatial::{into_live,into_saved};

#[derive(Clone,Copy)]
pub struct Environment<'a>{pub player:u32,pub hotseat:bool,pub merchants:&'a [u16],pub merchant_data:&'a [u8],pub routes:&'a [Option<p3_save::ships::SavedRoute>],pub alderman_town:u8,pub town_count:u32,pub merchant_towns:&'a [u8],pub report_enabled:&'a [bool],pub report_texts:Option<&'a crate::battle_text::BattleTextBank>,pub fleets:&'a [u8],pub captains:&'a [u8],pub manager:&'a [u8],pub captain_free_head:u16,pub difficulty:u8}

/// World images at the exact 0x515F70 call, including the cargo just looted.
pub struct PirateCaptureWorld<'a>{
 pub ships:&'a mut Vec<u8>,pub merchants:&'a mut [u8],pub convoys:&'a mut Vec<u8>,
 pub manager:&'a mut Vec<u8>,pub captains:&'a mut p3_core::ships::captains::CaptainPool,
}
/// A cloneable implementation is staged by `advance_with_capture_services`.
/// `objects` must reflect the five real pointers, including absent bands.
pub trait PirateCaptureServices {
 fn objects(&self)->[bool;5];
 fn finish_clock(&self)->Option<(u32,u32)>{None}
 fn finish_news(&mut self,_stamp:u32,_names:Vec<u8>){}
 fn register_capture(&mut self,index:u8,world:PirateCaptureWorld<'_>)->Result<(),String>;
 /// `remove_ship_from_pirate_groups` 0x005066F0: 0x00515F00 on each present band over `ships`.
 fn remove_ship(&mut self,_ships:&mut [u8],_target:u16)->Result<(),String>{Err("grupos piratas pendientes".into())}
 fn captain_resources(&self)->Option<p3_core::economy::pirate_group_callbacks::CaptainResources>{None}
 /// 0x503B00: el convoy lleva una caza hanseática activa.
 fn hunt_convoy(&self,_convoy:u16)->bool{false}
 /// Tarea 0x34 de `convoy` (0x4D8DB0 con `convoy+2`) y nodo antepuesto a su `+8`.
 fn hunt_task(&mut self,_convoy:u16)->Option<&mut p3_core::scheduler::TaskRecord>{None}
 fn hunt_loss(&mut self,_convoy:u16,_loss:p3_core::ships::return_convoy::HuntLoss)->Result<(),String>{Err("lista de bajas de expedición hanseática pendiente".into())}
 /// Called once after the complete advance, so the returned factory contains
 /// changes from later movement, impacts or captures as well as the band call.
 fn synchronize(&mut self,_world:PirateCaptureWorld<'_>)->Result<(),String>{Ok(())}
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum BattleNews { Global([u8;16]), Loot { recipient:u16, record:[u8;16] }, CapturedCaptain { recipient:u16, record:[u8;16] } }
#[derive(Clone,Debug)]
enum PendingNews { Sink(u16,u16,Vec<u8>), Capture(BattleNews) }

pub(crate) fn can_remove(index:&CollisionIndex,id:u16)->bool{
 let n=index.capacity;let i=usize::from(id);if i>=n{return false}let mut free=index.free_head;let mut seen=vec![false;n];while free!=u16::MAX{let j=usize::from(free);if j>=n||seen[j]||free==id{return false}seen[j]=true;free=index.free_next[j]}
 for axis in [&index.x,&index.y]{for ep in [i*2,i*2+1]{let prior=axis.previous[ep];let next=axis.next[ep];if axis.head!=ep as u16&&(usize::from(prior)>=n*2||axis.next[usize::from(prior)]!=ep as u16){return false}if next!=u16::MAX&&(usize::from(next)>=n*2||axis.previous[usize::from(next)]!=ep as u16){return false}}}true
}
fn apply_damage(index:&mut CollisionIndex,near:&[u16],far:&[u16],merchant_data:&[u8],sink_notices:&mut Vec<PendingNews>,battle:&mut [u8],ships:&mut [Vec<u8>],worlds:&mut [u8],id:usize,amount:i32,casualties:i16,mask:u8)->Result<u8,&'static str>{
 let local=ships.get_mut(id).ok_or("barco local ausente")?;let world_id=u32::from_le_bytes(local[4..8].try_into().unwrap())as usize;let at=world_id.checked_mul(0x180).ok_or("barco mundial inválido")?;let world=worlds.get_mut(at..at+0x180).ok_or("barco mundial ausente")?;
 let near=*near.get(id).ok_or("índice cercano ausente")?;let far=*far.get(id).ok_or("índice lejano ausente")?;let mut events=Vec::new();let result=weapon_damage::apply_weapon_damage(world,local,battle,(near,far),amount,casualties,mask,&mut events)?;
 for e in events{match e{DamageEvent::SinkNotice(captain)=>{let side=usize::from(local[0x13d]);if side>1{return Err("bando inválido")}let at=0x35a+2*(1-side);sink_notices.push(PendingNews::Sink(captain,u16::from_le_bytes([battle[at],battle[at+1]]),merchant_data.to_vec()));},DamageEvent::RemoveSpatial(slot)=>{if !can_remove(index,slot){return Err("índice espacial inválido")}index.remove(slot)},DamageEvent::RecalculateEquipment=>{}}}Ok(result)
}
fn notify(battle:&mut [u8],code:u16){let old=u16::from_le_bytes([battle[0x1e94],battle[0x1e95]]);if old>>10<code{battle[0x1e94..0x1e96].copy_from_slice(&((old&0x3ff)|(code<<10)).to_le_bytes())}}
#[derive(Default)]
struct DeferredRemoval(Vec<u16>);
impl projectile::Spatial for DeferredRemoval{
 fn bounds(&mut self,_:u16,_:i16,_:i16,_:i16,_:i16){unreachable!()}
 fn remove(&mut self,slot:u16){self.0.push(slot)}
 fn add(&mut self,_:u8,_:u8,_:i16,_:i16,_:i16,_:i16)->Option<u16>{unreachable!()}
}
/// La reserva original amplía el índice por páginas de 32 al crear disparos.
struct GrowingIndex<'a>(&'a mut CollisionIndex);
impl projectile::Spatial for GrowingIndex<'_>{
 fn bounds(&mut self,slot:u16,x0:i16,y0:i16,x1:i16,y1:i16){self.0.bounds(slot,x0,y0,x1,y1)}
 fn remove(&mut self,slot:u16){self.0.remove(slot)}
 fn add(&mut self,kind:u8,reference:u8,x0:i16,y0:i16,x1:i16,y1:i16)->Option<u16>{
  if self.0.free_head==u16::MAX{let mut pool=into_saved(self.0).ok()?;pool.grow().ok()?;*self.0=into_live(&pool).ok()?}
  self.0.add(kind,reference,x0,y0,x1,y1)
 }
}
struct Effects<'a>{error:Option<&'static str>,projectiles:&'a mut ProjectileTable,index:&'a mut CollisionIndex,near:&'a [u16],far:&'a [u16],merchant_data:&'a [u8],sink_notices:&'a mut Vec<PendingNews>}
impl PairHost for Effects<'_>{
 fn hit_mask(&mut self,battle:&mut [u8],ships:&mut [Vec<u8>],worlds:&mut [u8],projectile:u16,ship:u16)->u8{
  let Some(Some(p))=self.projectiles.slots.get(usize::from(projectile))else{self.error=Some("proyectil ausente");return 0};let Some(local)=ships.get(usize::from(ship))else{self.error=Some("barco local ausente");return 0};let id=u32::from_le_bytes(local[4..8].try_into().unwrap())as usize;let Some(world)=worlds.get(id.saturating_mul(0x180)..id.saturating_mul(0x180).saturating_add(0x180))else{self.error=Some("barco mundial ausente");return 0};let kind=usize::from(world[0xe]);if kind>=4{self.error=Some("casco desconocido");return 0}let read=|a:&[u8],at:usize|u32::from_le_bytes(a[at..at+4].try_into().unwrap());p3_core::ships::hull::hit_mask(&p3_core::ships::hull::HullQuery{hull_type:kind,heading:local[0x11],ship:(read(local,8),read(local,12)),projectile:(read(p,4),read(p,8)),velocity:(read(p,12)as i32,read(p,16)as i32),direct:battle[0xac8]!=0})
 }
 fn impact(&mut self,battle:&mut [u8],ships:&mut [Vec<u8>],worlds:&mut [u8],projectile:u16,target:u16,mask:u8){
  let result=(||->Result<(),&'static str>{let p=self.projectiles.slots.get(usize::from(projectile)).and_then(Option::as_ref).ok_or("proyectil ausente")?;let own=u16::from_le_bytes([p[0x2c],p[0x2d]]);let slot=u16::from_le_bytes([p[0x2e],p[0x2f]]);let shooter=u16::from_le_bytes([p[0x24],p[0x25]]);if own!=projectile||!can_remove(self.index,slot){return Err("proyectil inconsistente")}let state=ships.iter().map(|s|(s[0x13d],s[0x12a])).collect::<Vec<_>>();if usize::from(target)>=state.len()||shooter!=u16::MAX&&usize::from(shooter)>=state.len()||state.iter().any(|(side,_)|*side>1){return Err("barco de proyectil inválido")}
  let mut pending=DeferredRemoval::default();let mut events=Vec::new();let mut damage_result=Ok(0);let graphic=projectile::impact(self.projectiles,usize::from(projectile),target,battle,&|i|state[usize::from(i)],&mut pending,false,|battle,amount|{damage_result=apply_damage(self.index,self.near,self.far,self.merchant_data,self.sink_notices,battle,ships,worlds,usize::from(target),amount,-1,mask);damage_result.unwrap_or(0)},&mut events);damage_result?;for slot in pending.0{if !can_remove(self.index,slot){return Err("índice espacial inválido")}self.index.remove(slot)}for event in events{match event{projectile::ImpactEvent::Notify(code)=>notify(battle,u16::from(code)),projectile::ImpactEvent::View{..}=>return Err("vista de impacto pendiente")}}if graphic!=0{return Err("recurso gráfico pendiente")}Ok(())})();if let Err(e)=result{self.error=Some(e)}
 }
 fn damage(&mut self,battle:&mut [u8],ships:&mut [Vec<u8>],worlds:&mut [u8],ship:u16,amount:i32){if let Err(e)=apply_damage(self.index,self.near,self.far,self.merchant_data,self.sink_notices,battle,ships,worlds,usize::from(ship),amount,0,0){self.error=Some(e)}}
}
struct Host<'a>{alderman_reports:Vec<(u16,u8,Option<(u8,u8)>)>,sponsor_tasks:Vec<SponsorTask>,town_attack:bool,town_scene:Vec<u8>,town_water:Vec<u8>,updated_slots:Vec<bool>,pirate_services:Option<&'a mut dyn PirateCaptureServices>,capture_error:Option<String>,spawned_worlds:Vec<u8>,world_snapshot:Vec<u8>,pending_world:Option<(Vec<u8>,usize,usize)>,capture_events:Vec<p3_core::ships::capture_world::WorldCaptureEvent>,previous:Vec<u16>,town_count:u32,routes:Vec<Option<p3_save::ships::SavedRoute>>,cleared_routes:Vec<u16>,merchant_data:Vec<u8>,index:CollisionIndex,projectiles:ProjectileTable,error:Option<&'static str>,destroy_requested:bool,sink_notices:Vec<PendingNews>,loss_notices:Vec<p3_core::ships::finish_report::LossNotice>,reports:Vec<p3_core::ships::finish_report::BattleReport>,report_positions:Vec<usize>,next:Vec<u16>,near:Vec<u16>,far:Vec<u16>,captains:Vec<u8>,manager:Vec<u8>,captain_free_head:u16,captain_pool_changed:bool,merchants:Vec<u16>,merchant_towns:Vec<u8>,report_enabled:Vec<bool>,report_texts:Option<&'a crate::battle_text::BattleTextBank>,fleets:Vec<u8>,player:u32,difficulty:u8,rng:Option<CrtRand>}
impl Host<'_>{
 fn can_remove(&self,id:u16)->bool{
  can_remove(&self.index,id)
 }
 fn apply_damage(&mut self,battle:&mut [u8],ships:&mut [Vec<u8>],worlds:&mut [u8],id:usize,amount:i32,casualties:i16,mask:u8)->Result<u8,&'static str>{
  apply_damage(&mut self.index,&self.near,&self.far,&self.merchant_data,&mut self.sink_notices,battle,ships,worlds,id,amount,casualties,mask)
 }
 fn fire(&mut self,battle:&mut [u8],ships:&mut [Vec<u8>],worlds:&mut [u8],id:usize)->Result<(),&'static str>{
  let own=ships.get(id).ok_or("barco local ausente")?.clone();if own.len()<0x170{return Err("barco local truncado")}
  for ship in ships.iter(){if ship.len()<0x170||usize::from(ship[0x13d])>=2{return Err("barco local inválido")}let w=u32::from_le_bytes(ship[4..8].try_into().unwrap())as usize;if w.checked_mul(0x180).and_then(|v|v.checked_add(0x180)).is_none_or(|end|end>worlds.len()){return Err("barco mundial ausente")}if ship[0x12a]&8!=0&&usize::from(u16::from_le_bytes([ship[0x128],ship[0x129]]))>=ships.len(){return Err("objetivo de abordaje inválido")}}
  let world_id=u32::from_le_bytes(own[4..8].try_into().unwrap())as usize;let at=world_id*0x180;let world=&worlds[at..at+0x180];let equipment:[u8;24]=world[0x13c..0x154].try_into().unwrap();let cap=u16::from_le_bytes([world[0x42],world[0x43]])as usize;let captain_experience=self.captains.get(cap*16+11).copied();let owner=world[0];let human_difficulty=self.merchants.get(usize::from(owner)).filter(|v|**v==0).map(|_|self.difficulty);
  let shot=ShotContext{ship_x:i16::from_le_bytes([own[0x0a],own[0x0b]]),ship_y:i16::from_le_bytes([own[0x0e],own[0x0f]]),local_id:u16::from_le_bytes([own[0x138],own[0x139]]),battle_pointer:u32::from_le_bytes(own[0x12c..0x130].try_into().unwrap()),owner,maintenance:world[0x3f]as i8,captain_experience,human_difficulty};
  let plan={let local=|n:u16|ships[usize::from(n)].as_slice();let world_of=|n:u32|&worlds[n as usize*0x180..(n as usize+1)*0x180];let view=SelectView{battle,next:&self.next,local:&local,world:&world_of};let boarding=Boarding{battle,virtual_10:self.town_attack,world:&world_of,captains:Captains{table:&self.captains,count:(self.captains.len()/16)as u16}};weapons::plan_weapons(&view,&own,self.town_attack,&mut |target|boarding.board_filter(&own,local(target),true))?};
  if self.town_attack && !matches!(plan, WeaponPlan::None) { return Err("armas de asalto urbano pendientes") }
  match plan{
   WeaponPlan::None=>{},
   WeaponPlan::Direct(shots)=>for shot_plan in shots{let estimate=weapons::estimate(&equipment,shot_plan.band,shot_plan.length,&shot,battle)?;let target=usize::from(shot_plan.target);let result=self.apply_damage(battle,ships,worlds,target,estimate.damage,-1,3)?;ships[id][0x130+usize::from(shot_plan.band)]=0x96;let side=usize::from(battle[0x559+usize::from(own[0x13d])]);if side>1{return Err("bando inválido")}let counter=0xaca+side;if battle[counter]<0x7f{battle[counter]+=1}if ships[target][0x12a]&0x20!=0{battle[counter]=0xff}if usize::from(battle[0xad2])==side+1{let code=if result==2{11u16}else{9};let old=u16::from_le_bytes([battle[0x1e94],battle[0x1e95]]);if old>>10<code{battle[0x1e94..0x1e96].copy_from_slice(&((old&0x3ff)|(code<<10)).to_le_bytes())}}},
   WeaponPlan::Volleys(shots)=>{if !shots.is_empty(){let rng=self.rng.as_mut().ok_or("estado CRT necesario para disparar")?;let mut volley=Volley{local:&mut ships[id],weapons:equipment,ship:(shot.ship_x,shot.ship_y)};let mut index=GrowingIndex(&mut self.index);for(band,target)in shots{volley::fire_volley_real(&mut volley,&shot,&mut self.projectiles,battle,&mut index,&mut ||rng.rand()as u32,band,target)?}}}
  }Ok(())
 }
}
impl BoardingHost for Host<'_>{
 fn capture(&mut self,_:&mut [u8],_:u16,_:u16){self.error=Some("captura pendiente")}
 fn capture_pair(&mut self,battle:&mut [u8],first:u16,second:u16,first_ship:&mut [u8],second_ship:&mut [u8],first_world:&mut [u8],second_world:&mut [u8]){
  if self.town_attack { self.error=Some("abordaje de asalto urbano pendiente"); return; }
  use p3_core::ships::capture::{capture_ships,CaptureData,CaptureEnv,CaptureHost,FleetShip};
  struct Effects<'a>{unsupported:bool,ownership:Vec<(u32,u32)>,news:Vec<BattleNews>,prepared:Option<(Vec<u8>,Vec<u8>,Vec<u8>)>,notify:&'a mut dyn FnMut(u8,u32,u32,&mut [u8],&mut [u8],&mut [u8])->Result<(),String>,error:Option<String>}
  impl CaptureHost for Effects<'_>{
   fn text(&mut self,_:u32){self.unsupported=true}fn ui_message(&mut self){self.unsupported=true}
   fn news(&mut self,target:i32,message:&[u8;0x18]){
    let mut record=[0u8;16];record.copy_from_slice(&message[..16]);
    match (target,message[4]) {
     (-2,0x2a)=>self.news.push(BattleNews::Global(record)),
     (0..=65535,0x30)=>self.news.push(BattleNews::Loot{recipient:target as u16,record}),
     (0..=65535,0x54)=>self.news.push(BattleNews::CapturedCaptain{recipient:target as u16,record}),
     _=>self.unsupported=true,
    }
   }
   fn ownership_event_with_state(&mut self,a:u32,b:u32,first:&mut [u8],second:&mut [u8],merchants:&mut [u8]){
    self.ownership_event(a,b);
    if let Some((new_first,new_second,new_merchants))=self.prepared.take(){first.copy_from_slice(&new_first);second.copy_from_slice(&new_second);merchants.copy_from_slice(&new_merchants)}else{self.unsupported=true}
   }
   fn ownership_event(&mut self,a:u32,b:u32){self.ownership.push((a,b))}fn rating(&mut self,merchant:&[u8])->u8{p3_core::ai::reputation::Merchant{bytes:merchant.to_vec()}.get_merchant_epithet_index()}fn notify_object(&mut self,_:u8){self.unsupported=true}
   fn notify_object_with_state(&mut self,index:u8,a:u32,b:u32,first:&mut [u8],second:&mut [u8],merchants:&mut [u8]){if let Err(error)=(self.notify)(index,a,b,first,second,merchants){self.error=Some(error)}}
  }
  let result=(||->Result<(),&'static str>{
   if first_ship.len()<0x170||second_ship.len()<0x170||first_world.len()<0x180||second_world.len()<0x180||battle.len()<0x1e98{return Err("abordaje truncado")}
   let first_id=u32::from_le_bytes(first_ship[4..8].try_into().unwrap())as usize;let second_id=u32::from_le_bytes(second_ship[4..8].try_into().unwrap())as usize;
   if first_id==second_id||first_id>=self.world_snapshot.len()/0x180||second_id>=self.world_snapshot.len()/0x180||usize::from(first)>=self.next.len()||usize::from(second)>=self.next.len(){return Err("barcos de abordaje inválidos")}
   if first_ship[0x13d]>1||second_ship[0x13d]>1||battle[0x559]>1||battle[0x55a]>1{return Err("bando de abordaje inválido")}
   let captains=Captains{table:&self.captains,count:(self.captains.len()/16)as u16};let lookup=|id:u32|if id as usize==first_id{&*first_world}else{&*second_world};
   let boarding=Boarding{battle,virtual_10:false,world:&lookup,captains};let take=boarding.can_capture(first_ship,second_ship,true);
   for ship in [&*first_ship,&*second_ship]{let side=usize::from(ship[0x13d]);let at=0x35a+2*side;if self.player==u32::from(u16::from_le_bytes([battle[at],battle[at+1]])){return Err("diálogo de abordaje pendiente")}}
   let fleet=u16::from_le_bytes([first_world[8],first_world[9]])as usize;let fleet_at=fleet.checked_mul(60).and_then(|v|v.checked_add(0x12)).ok_or("convoy inválido")?;if !take&&fleet_at>self.fleets.len(){return Err("convoy de abordaje ausente")}
   let mut snapshot=self.world_snapshot.clone();snapshot[first_id*0x180..(first_id+1)*0x180].copy_from_slice(first_world);snapshot[second_id*0x180..(second_id+1)*0x180].copy_from_slice(second_world);
   let lookup_snapshot=snapshot.clone();let lookup_fleets=self.fleets.clone();
   let mut npc_letters=Vec::new();let mut npc_reports=Vec::new();let mut npc_tasks=Vec::new();let mut capture_events=Vec::new();let mut fleets=self.fleets.clone();let mut merchants=self.merchant_data.clone();let mut new_captain_pool=None;let mut new_manager=None;let mut prepared=None;
   if take{
    if self.manager.len()<0xf8||u16::from_le_bytes([self.manager[0xe0],self.manager[0xe1]])!=self.captain_free_head||usize::from(u16::from_le_bytes([self.manager[0xf2],self.manager[0xf3]]))!=self.captains.len()/16{return Err("gestor de capitanes incoherente")}
    if self.routes.len()!=snapshot.len()/0x180{return Err("estado de rutas ausente")}
    let route=self.routes[second_id].as_ref().ok_or("ruta de barco libre")?;
    let route_exists=!matches!(route,p3_save::ships::SavedRoute::None);
    let changing_owner=first_world[0]!=second_world[0];
    let at=second_id*0x180;
    if changing_owner&&route_exists{if snapshot[at+0x24..at+0x28]==[0;4]{let token=0x7f00_0000u32|u32::try_from(second_id+1).map_err(|_|"ruta excesiva")?;snapshot[at+0x24..at+0x28].copy_from_slice(&token.to_le_bytes());}snapshot[at+0x36]=u8::from(matches!(route,p3_save::ships::SavedRoute::Owned(_)));}
    if changing_owner&&!route_exists&&snapshot[at+0x24..at+0x28]!=[0;4]{return Err("descriptor de ruta incoherente")}
    if self.merchant_data.len()!=self.merchants.len()*0x650||self.fleets.len()%0x3c!=0||self.fleets.len()/0x3c>u16::MAX as usize{return Err("datos de captura ausentes")}
    if self.previous.len()!=self.next.len(){return Err("listas locales truncadas")}
    for i in 0..self.next.len(){let after=self.next[i];let before=self.previous[i];if after!=u16::MAX&&(usize::from(after)>=self.next.len()||self.previous[usize::from(after)]!=i as u16){return Err("lista local incoherente")}if before!=u16::MAX&&(usize::from(before)>=self.next.len()||self.next[usize::from(before)]!=i as u16){return Err("lista local incoherente")}}
    snapshot[second_id*0x180+0x40..second_id*0x180+0x42].fill(0);
    let mut world=p3_core::ships::capture_world::WorldCapture{ships:&mut snapshot,merchants:&mut merchants,convoys:&mut fleets,convoy_count:(self.fleets.len()/0x3c)as u16,town_count:self.town_count,prestige:65};let mut pool=p3_core::ships::captains::CaptainPool{entries:self.captains.clone(),capacity:(self.captains.len()/16)as u16,free_head:self.captain_free_head};let mut manager=self.manager.clone();let mut npc=NpcCapture{services:self.pirate_services.as_deref_mut(),letters:Vec::new(),reports:Vec::new(),tasks:Vec::new(),error:None};let outcome=p3_core::ships::capture_world::capture_world_with_npc(&mut world,first_id as u32,second_id as u32,Some(0),&mut pool,&mut manager,u16::MAX,&mut npc);let NpcCapture{letters,error,reports,tasks,..}=npc;if let Some(error)=error{self.capture_error=Some(error)}let (events,_freed)=outcome?;npc_letters=letters;npc_reports=reports;npc_tasks=tasks;capture_events=events;new_captain_pool=Some(pool);new_manager=Some(manager);
    // Prepare transactionally, but publish only at the original operation91.
    // Before it, capture_ships reads the old captain, owner and reputation.
    prepared=Some((snapshot[first_id*0x180..(first_id+1)*0x180].to_vec(),snapshot[second_id*0x180..(second_id+1)*0x180].to_vec(),merchants.clone()));merchants.clone_from(&self.merchant_data);
   }
   let fleet_ship=|id:u16|{let at=usize::from(id)*0x180;let w=lookup_snapshot.get(at..at+0x180)?;Some(FleetShip{health:u32::from_le_bytes(w[0x18..0x1c].try_into().unwrap()),captain:u16::from_le_bytes([w[0x42],w[0x43]]),owner:w[0x15d],kind_word:u16::from_le_bytes([w[0x15e],w[0x15f]])})};
   if !take{let reward=u16::from_le_bytes([self.fleets[fleet_at-2],self.fleets[fleet_at-1]]);if usize::from(reward)<snapshot.len()/0x180{if let Some(ship)=fleet_ship(reward){if usize::from(ship.owner)<self.merchants.len()&&usize::from(ship.captain)<self.captains.len()/16&&self.merchant_data.len()!=self.merchants.len()*0x650{return Err("datos de botín ausentes")}}}}
   let env=CaptureEnv{captains,player_slot:self.player,fleet_threshold:0,merchant_limit:self.merchants.len()as u16,fleet_count:(snapshot.len()/0x180)as u16,fleets:&lookup_fleets,fleet_ship:&fleet_ship,// Without metadata we deliberately take a possible notification and reject it;
    // absence of all bands cannot be inferred from the old environment.
    objects:self.pirate_services.as_ref().map(|s|s.objects()).unwrap_or([true;5]),virtual_10:false,battle_address:0};
   let mut previous=self.previous.clone();let mut next=self.next.clone();let mut ui=false;
   let mut notify=|index:u8,a:u32,b:u32,first:&mut [u8],second:&mut [u8],ms:&mut [u8]|->Result<(),String>{
    let services=self.pirate_services.as_deref_mut().ok_or("estado de bandas piratas ausente")?;
    let a=usize::try_from(a).map_err(|_|"índice de captor excesivo")?;let b=usize::try_from(b).map_err(|_|"índice de objetivo excesivo")?;
    snapshot.get_mut(a*0x180..(a+1)*0x180).ok_or("captor ausente")?.copy_from_slice(first);
    snapshot.get_mut(b*0x180..(b+1)*0x180).ok_or("objetivo ausente")?.copy_from_slice(second);
    let mut pool=p3_core::ships::captains::CaptainPool{entries:self.captains.clone(),capacity:(self.captains.len()/16)as u16,free_head:self.captain_free_head};let mut manager=self.manager.clone();
    services.register_capture(index,PirateCaptureWorld{ships:&mut snapshot,merchants:ms,convoys:&mut fleets,manager:&mut manager,captains:&mut pool})?;
    if snapshot.len()%0x180!=0||snapshot.len()<self.world_snapshot.len()||fleets.len()%0x3c!=0||pool.entries.len()!=usize::from(pool.capacity)*16{return Err("resultado de banda inválido".into())}
    first.copy_from_slice(&snapshot[a*0x180..(a+1)*0x180]);second.copy_from_slice(&snapshot[b*0x180..(b+1)*0x180]);new_captain_pool=Some(pool);new_manager=Some(manager);Ok(())
   };
   let mut effects=Effects{unsupported:false,ownership:Vec::new(),news:Vec::new(),prepared,notify:&mut notify,error:None};capture_ships(first,second,CaptureData{battle,prev:&mut previous,next:&mut next,first_ship,second_ship,first_world,second_world,merchants:&mut merchants,ui_flag:&mut ui},&env,&mut effects);
   if effects.unsupported{return Err("efectos de abordaje pendientes")};
   let Effects{ownership,mut news,error,..}=effects;drop(notify);
   // Letters of the original 0x540520 run inside operation 0x91, after the capture news above.
   for (recipient,record) in npc_letters{news.push(if record[4]==0x30{BattleNews::Loot{recipient:u16::from(recipient),record}}else{BattleNews::CapturedCaptain{recipient:u16::from(recipient),record}})}
   if let Some(error)=error{self.capture_error=Some(error);return Err("efecto de banda pirata fallido")}
   if take{if ownership!=[(first_id as u32,second_id as u32)]{return Err("evento de captura incoherente")}self.previous=previous;self.next=next;self.capture_events.extend(capture_events);self.alderman_reports.extend(npc_reports.drain(..));self.sponsor_tasks.extend(npc_tasks.drain(..));if first_world[0]!=self.world_snapshot[second_id*0x180]&&self.routes[second_id]!=Some(p3_save::ships::SavedRoute::None){self.routes[second_id]=Some(p3_save::ships::SavedRoute::None);self.cleared_routes.push(second_id as u16);}}else if !ownership.is_empty(){return Err("captura inesperada")};
   if take||new_manager.is_some(){
    if snapshot.len()/0x180>self.routes.len(){for ship in snapshot[self.routes.len()*0x180..].chunks_exact(0x180){self.routes.push(if u16::from_le_bytes([ship[0x134],ship[0x135]])<0x17{Some(p3_save::ships::SavedRoute::None)}else{None});}}
    self.pending_world=Some((snapshot,first_id,second_id));self.fleets=fleets;if let Some(manager)=new_manager{self.manager=manager}if let Some(pool)=new_captain_pool{if pool.entries!=self.captains||pool.free_head!=self.captain_free_head{self.captain_pool_changed=true;self.captains=pool.entries;self.captain_free_head=pool.free_head;}}}
   self.merchant_data=merchants;self.sink_notices.extend(news.into_iter().map(PendingNews::Capture));Ok(())
  })();if let Err(error)=result{self.error=Some(error)}
 }
}
/// NPC side of 0x540520 for the battle runtime. No scheduler is connected here:
/// a sponsor task (kind 5) is rejected explicitly instead of being dropped.
struct NpcCapture<'s,'a>{services:Option<&'s mut (dyn PirateCaptureServices+'a)>,letters:Vec<(u8,[u8;16])>,reports:Vec<(u16,u8,Option<(u8,u8)>)>,tasks:Vec<SponsorTask>,error:Option<String>}
impl p3_core::ships::capture_world::NpcCaptureHost for NpcCapture<'_,'_>{
 fn band_objects(&self)->[bool;5]{self.services.as_ref().map(|s|s.objects()).unwrap_or([true;5])}
 fn remove_from_bands(&mut self,ships:&mut [u8],target:u16)->Result<(),String>{let r=self.services.as_deref_mut().ok_or_else(||"estado de bandas piratas ausente".to_string()).and_then(|s|s.remove_ship(ships,target));if let Err(e)=&r{self.error=Some(e.clone())}r}
 // 0x4D8CF0 (tipo 5) is deferred to the caller's scheduler, like 0x4EC864.
 fn sponsor_task(&mut self,sponsor:u8,home:u8,captor:u8,word:u16)->Result<(),String>{self.tasks.push(SponsorTask{sponsor,home,captor,word});Ok(())}
 fn register_capture(&mut self,index:u8,w:p3_core::ships::capture_world::NpcCaptureWorld<'_>)->Result<(),String>{let r=self.services.as_deref_mut().ok_or_else(||"estado de bandas piratas ausente".to_string()).and_then(|s|s.register_capture(index,PirateCaptureWorld{ships:w.ships,merchants:&mut w.merchants[..],convoys:w.convoys,manager:w.manager,captains:w.captains}));if let Err(e)=&r{self.error=Some(e.clone())}r}
 fn letter(&mut self,recipient:u8,record:[u8;16]){self.letters.push((recipient,record))}
 fn captain_resources(&self)->Option<p3_core::economy::pirate_group_callbacks::CaptainResources>{self.services.as_ref().and_then(|s|s.captain_resources())}
 // 0x503B00: the loss node hangs from task+8; the services keep it with the task (C10).
 fn hanse_expedition(&mut self,convoy:u16)->bool{self.services.as_ref().is_some_and(|s|s.hunt_convoy(convoy))}
 fn hunt_task(&mut self,convoy:u16)->Option<&mut p3_core::scheduler::TaskRecord>{self.services.as_deref_mut().and_then(|s|s.hunt_task(convoy))}
 fn hunt_loss(&mut self,convoy:u16,loss:p3_core::ships::return_convoy::HuntLoss)->Result<(),String>{let r=self.services.as_deref_mut().ok_or_else(||"estado de bandas piratas ausente".to_string()).and_then(|s|s.hunt_loss(convoy,loss));if let Err(e)=&r{self.error=Some(e.clone())}r}
 // 0x4EC864 is deferred to the caller's scheduler with the captain bytes read at the call.
 fn alderman_report(&mut self,target:u16,captor:u8,ships:&[u8],captains:Option<&p3_core::ships::captains::CaptainPool>)->Result<(),String>{let at=usize::from(target)*0x180;let ship=ships.get(at..at+0x180).ok_or("barco del regidor ausente")?;let cap=u16::from_le_bytes([ship[0x42],ship[0x43]]);let look=match captains{Some(p) if cap<p.capacity=>{let e=&p.entries[usize::from(cap)*16..];Some((e[2],e[3]))},_=>None};self.reports.push((target,captor,look));Ok(())}
}
impl UpdateHost for Host<'_>{
 fn prepare_ship_step(&mut self,worlds:&[u8]){self.world_snapshot.clear();self.world_snapshot.extend_from_slice(worlds);self.world_snapshot.extend_from_slice(&self.spawned_worlds)}
 fn sync_capture(&mut self,worlds:&mut [u8],next:&mut [u16]){if let Some((snapshot,a,b))=self.pending_world.take(){if snapshot.len()<worlds.len()||snapshot.len()%0x180!=0||next.len()!=self.next.len(){self.error=Some("estado de captura incompatible");return}for(i,record)in snapshot[..worlds.len()].chunks_exact(0x180).enumerate(){if i!=a&&i!=b{worlds[i*0x180..(i+1)*0x180].copy_from_slice(record)}}self.spawned_worlds=snapshot[worlds.len()..].to_vec();next.copy_from_slice(&self.next);}}
 fn ui_busy(&mut self)->bool{false}
 fn ui_release(&mut self){self.error=Some("interfaz de batalla pendiente")}
 fn show_message(&mut self,_:&[u8]){self.error=Some("diálogo de batalla pendiente")}
 fn notify_battle(&mut self,_:u16,_:i16,_:i16){self.error=Some("aviso de batalla pendiente")}
 fn push_event(&mut self,_:u32,_:u16,_:u32,_:u32){self.error=Some("operación de batalla pendiente")}
 fn projectile_present(&mut self,id:u16)->bool{self.projectiles.slots.get(usize::from(id)).is_some_and(Option::is_some)}
 fn advance_projectile(&mut self,battle:&mut [u8],_:&mut [Vec<u8>],_:&mut [u8],id:u16){
  let Some(Some(p))=self.projectiles.slots.get(usize::from(id))else{self.error=Some("proyectil ausente");return};let own=u16::from_le_bytes([p[0x2c],p[0x2d]]);let spatial=u16::from_le_bytes([p[0x2e],p[0x2f]]);if own!=id||!self.can_remove(spatial){self.error=Some("proyectil inconsistente");return}
  if let Advance::Finished{view_effect,graphic}=projectile::advance(&mut self.projectiles,usize::from(id),battle[0xac8]!=0,false,&mut self.index){if view_effect||graphic!=0{self.error=Some("efecto gráfico de proyectil pendiente")}}
 }
 fn weapons(&mut self,battle:&mut [u8],ships:&mut [Vec<u8>],worlds:&mut [u8],id:usize){if let Err(e)=self.fire(battle,ships,worlds,id){self.error=Some(e)}}
 fn index_bounds(&mut self,id:u16,x0:i32,y0:i32,x1:i32,y1:i32){if usize::from(id)>=self.index.capacity{self.error=Some("índice espacial inválido")}else{self.index.bounds(id,x0 as i16,y0 as i16,x1 as i16,y1 as i16);self.updated_slots[usize::from(id)]=true}}
 fn index_remove(&mut self,id:u16){if !self.can_remove(id){self.error=Some("índice espacial inválido")}else{self.index.remove(id)}}
 fn release_graphic(&mut self,_:i32){self.error=Some("recurso gráfico pendiente")}
 fn collision_pairs(&mut self,f:&mut dyn FnMut(&Slots,u16,u16,&mut dyn PairHost)->Result<(),&'static str>)->Result<(),&'static str>{
  let mut error=None;self.index.update(|index,a,b|{if error.is_some(){return}if self.town_attack {let first=index.types[usize::from(a)];let second=index.types[usize::from(b)];if first==0 && second==7{let i=usize::from(a);if !self.updated_slots[i]{error=Some("posición de costa sin actualizar");return}let x=(i32::from(index.x.coordinates[i*2])+i32::from(index.x.coordinates[i*2+1]))/2;let y=(i32::from(index.y.coordinates[i*2])+i32::from(index.y.coordinates[i*2+1]))/2;if index.x.coordinates[i*2]>=index.x.coordinates[i*2+1]||index.y.coordinates[i*2]>=index.y.coordinates[i*2+1]||x<0||x>=4352||y<0||y>=3842{error=Some("posición de costa no contrastada");return}let clear=match crate::pirate_active_frame::town_coast_fast_clear(&self.town_water,x as u16,y as u16){Ok(v)=>v,Err(e)=>{error=Some(e);return}};if !clear{error=Some("colisión de asalto urbano pendiente");return}return}if first==8 && second==1{let piece=usize::from(index.references[usize::from(a)] as u8);let id=usize::from(index.references[usize::from(b)] as u8);let Some(Some(projectile))=self.projectiles.slots.get(id) else {error=Some("proyectil de torre ausente");return};match crate::pirate_active_frame::town_tower_projectile_intersects(&self.town_scene,piece,projectile){Ok(false)=>return,Ok(true)=>{error=Some("impacto de torre pendiente");return},Err(e)=>{error=Some(e);return}}}if first!=0 || second==8{return}}let (x,y,kind,reference)=(index.x.coordinates.clone(),index.y.coordinates.clone(),index.types.clone(),index.references.clone());let slots=Slots{x:&x,y:&y,kind:&kind,reference:&reference};let mut effects=Effects{error:None,projectiles:&mut self.projectiles,index,near:&self.near,far:&self.far,merchant_data:&self.merchant_data,sink_notices:&mut self.sink_notices};if let Err(e)=f(&slots,a,b,&mut effects){error=Some(e)}else if effects.error.is_some(){error=effects.error}});if let Some(e)=error{Err(e)}else{Ok(())}
 }
 fn destroy(&mut self){self.destroy_requested=true}
 fn finish(&mut self,battle:&mut [u8],ships:&mut [Vec<u8>],worlds:&mut [u8],survivors:bool)->Result<(),&'static str>{
  if self.town_attack { return Err("finalización de asalto urbano pendiente") }
  let side=u8::from(survivors);
  match p3_core::ships::finish_report::finish_ai_without_losses(battle,ships,worlds,&self.next,&self.merchants,side){
   Ok(())=>Ok(()),
   Err(_)=>{let lookup=|slot|self.report_texts.and_then(|bank|bank.get(slot));let texts=self.report_texts.map(|_|&lookup as &dyn Fn(u32)->Option<Vec<u8>>);let effects=if let Some((tick,stamp))=self.pirate_services.as_ref().and_then(|s|s.finish_clock()){
     p3_core::ships::finish_report::finish_headless_with_news(battle,ships,worlds,&self.next,&self.fleets,&self.merchants,&self.merchant_towns,&self.report_enabled,texts,self.player,side,Some(&self.merchant_data),(self.captains.len()/16)as u16,tick,stamp)?
    }else{p3_core::ships::finish_report::finish_headless_with_reports(battle,ships,worlds,&self.next,&self.fleets,&self.merchants,&self.merchant_towns,&self.report_enabled,texts,self.player,side)?};
    if let Some(service)=self.pirate_services.as_deref_mut(){service.finish_news(effects.news_stamp,effects.pirate_news)}
    let base=self.sink_notices.len();self.report_positions.extend(effects.report_positions.iter().map(|p|base+p));self.sink_notices.extend(effects.global_letters.into_iter().map(|v|PendingNews::Capture(BattleNews::Global(v))));self.loss_notices.extend(effects.losses);self.reports.extend(effects.reports);Ok(())},
  }
 }
}

/// Avanza una batalla sin vista y confirma ambos estados solo al completar la
/// llamada. Las ramas externas no conectadas provocan error y rollback total.
pub fn advance(battle:&mut RebuiltInitialCommercialBattle,worlds:&mut Vec<u8>,env:Environment<'_>,ticks:u32)->Result<(),String>{
 advance_with_rng(battle,worlds,env,ticks,&mut None)
}
/// Como `advance`, pero admite un estado CRT explícito para disparos que lo
/// necesiten. El estado se confirma junto con la batalla y la flota.
pub fn advance_with_rng(battle:&mut RebuiltInitialCommercialBattle,worlds:&mut Vec<u8>,env:Environment<'_>,ticks:u32,rng:&mut Option<CrtRand>)->Result<(),String>{
 let mut staged=battle.clone();let mut staged_world=worlds.clone();let mut staged_rng=*rng;
 let outcome=advance_with_rng_outcome(&mut staged,&mut staged_world,env,ticks,&mut staged_rng)?;if outcome.destroyed{return Err("destrucción de batalla pendiente".into())}if outcome.manager_data.is_some()||!outcome.loss_notices.is_empty()||!outcome.reports.is_empty()||!outcome.news.is_empty()||outcome.merchant_data.is_some()||outcome.fleet_data.is_some()||!outcome.capture_events.is_empty()||!outcome.cleared_routes.is_empty()||outcome.captain_pool.is_some()||!outcome.alderman_reports.is_empty(){return Err("entrega de efectos navales pendiente".into())}
 *battle=staged;*worlds=staged_world;*rng=staged_rng;Ok(())
}

/// Efectos que deben confirmarse junto con la batalla, la flota y S05.
/// Tarea tipo 5 que `op_capture_ship` 0x540520 programa al patrocinador de un barco pirata capturado
/// por un comerciante distinto: `add_task(tick + 0x300)` y la carga de [`SponsorTask::write`].
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct SponsorTask{pub sponsor:u8,pub home:u8,pub captor:u8,pub word:u16}
impl SponsorTask{
 /// Escribe el registro tal como lo hace 0x540520 tras 0x4D8CF0.
 pub fn write(&self,scheduler:&mut p3_core::scheduler::Scheduler,tick:u32){let i=usize::from(scheduler.add_task(tick.wrapping_add(0x300)));let r=&mut scheduler.records[i];r.kind=5;r.set_u8_at(8,self.sponsor);r.set_u8_at(9,self.home);r.set_u8_at(0xa,self.captor);r.set_u8_at(0xb,self.home);r.set_u32_at(0xc,tick);r.set_u8_at(0x10,4);r.set_u8_at(0x16,self.word as u8);r.set_u8_at(0x17,(self.word>>8)as u8);}
}
pub struct BattleAdvance{pub alderman_reports:Vec<(u16,u8,Option<(u8,u8)>)>,pub sponsor_tasks:Vec<SponsorTask>,pub manager_data:Option<Vec<u8>>,pub global_letters:Vec<[u8;16]>,pub news:Vec<BattleNews>,pub merchant_data:Option<Vec<u8>>,pub fleet_data:Option<Vec<u8>>,pub capture_events:Vec<p3_core::ships::capture_world::WorldCaptureEvent>,pub cleared_routes:Vec<u16>,pub captain_pool:Option<p3_core::ships::captains::CaptainPool>,pub destroyed:bool,pub loss_notices:Vec<p3_core::ships::finish_report::LossNotice>,pub reports:Vec<p3_core::ships::finish_report::BattleReport>,
 /// Índice de `news` ante el que va cada informe 0x29 (orden de envío del original).
 pub report_positions:Vec<usize>}
pub fn advance_with_rng_outcome(battle:&mut RebuiltInitialCommercialBattle,worlds:&mut Vec<u8>,env:Environment<'_>,ticks:u32,rng:&mut Option<CrtRand>)->Result<BattleAdvance,String>{
 advance_with_services_inner(battle,worlds,env,ticks,rng,None,false)
}
/// Paso base de escena urbana; rechaza transaccionalmente armas, abordajes,
/// contacto con tierra y finalización hasta conectar esos callbacks.
pub fn advance_town_attack_before_contact(battle:&mut RebuiltInitialCommercialBattle,worlds:&mut Vec<u8>,env:Environment<'_>,ticks:u32,rng:&mut Option<CrtRand>)->Result<BattleAdvance,String>{
 advance_with_services_inner(battle,worlds,env,ticks,rng,None,true)
}
/// Stages band/factory effects and the battle together. No service mutation is
/// published if any later callback, report or validation rejects this advance.
pub fn advance_with_capture_services<S:PirateCaptureServices+Clone>(battle:&mut RebuiltInitialCommercialBattle,worlds:&mut Vec<u8>,env:Environment<'_>,ticks:u32,rng:&mut Option<CrtRand>,services:&mut S)->Result<BattleAdvance,String>{
 let mut staged=services.clone();let outcome=advance_with_services_inner(battle,worlds,env,ticks,rng,Some(&mut staged),false)?;*services=staged;Ok(outcome)
}
fn advance_with_services_inner<'a>(battle:&mut RebuiltInitialCommercialBattle,worlds:&mut Vec<u8>,env:Environment<'a>,ticks:u32,rng:&mut Option<CrtRand>,services:Option<&'a mut dyn PirateCaptureServices>,town_attack:bool)->Result<BattleAdvance,String>{
 if worlds.len()%0x180!=0||worlds.len()/0x180>u16::MAX as usize||env.captains.len()%16!=0||env.captains.len()/16>u16::MAX as usize{return Err("tablas navales truncadas".into())}
 let mut staged=battle.clone();let mut staged_world=worlds.clone();let mut table=ProjectileTable::new(staged.state.projectiles.len());
 let town_speed=if town_attack{if staged.state.object.len()<0x2f24||staged.state.object[0x5c]!=0||staged.state.object[0x2f1a]!=0||staged.state.object[0x558]!=2||staged.state.object[0x5d..0x5f]!=[1,1]||staged.state.object[0x12e9..0x12eb]!=[0,0]||staged.state.object[0x1ec4..0x1ed4].iter().any(|&v|v!=0)||staged.state.object[0x658..0x65c]!=[1,0,1,0]||staged.state.local_ships.len()!=2||staged.state.object[0x130f]>15||((u32::from(staged.state.object[0x130f])+u32::from(ticks as u16))>>4)!=1||!staged.state.projectiles.is_empty(){return Err("paso urbano fuera del alcance contrastado".into())}Some(staged.state.object[0xaa8..0xab0].to_vec())}else{None};
 // El cargador original reserva 0x164; el puerto del paso exige 0x170,
 // aunque sus campos utilizados terminan en +0x162. La cola es solo espacio
 // del anfitrión y se retira antes de publicar o serializar.
 for ship in &mut staged.state.local_ships{if ship.len()!=0x164{return Err("barco local de tamaño no admitido".into())}ship.resize(0x170,0)}
 if table.bitmap.len()!=staged.state.bitmap.len(){return Err("bitmap de proyectiles inválido".into())}table.bitmap.clone_from(&staged.state.bitmap);
 for id in 0..table.slots.len(){if table.bitmap[id/32]&(1<<(id%32))!=0{table.slots[id]=Some(staged.state.projectiles[id].as_slice().try_into().map_err(|_|"proyectil truncado")?)}}
 let town_water=if town_attack{staged.state.object.get(0x1ee0..0x2ee0).ok_or("máscara de costa truncada")?.to_vec()}else{Vec::new()};let updated_slots=vec![false;staged.collisions.kinds.len()];
 // En la variante urbana admitida ambos bandos son de IA; sea_update
 // fija +0xAC8 a 1 antes de ejecutar colisiones del primer tick.
 let town_scene=if town_attack{let mut scene=staged.state.object.clone();scene[0xac8]=1;scene}else{Vec::new()};
 let mut host=Host{alderman_reports:Vec::new(),sponsor_tasks:Vec::new(),town_attack,town_scene,town_water,updated_slots,pirate_services:services,capture_error:None,spawned_worlds:Vec::new(),world_snapshot:Vec::new(),pending_world:None,capture_events:Vec::new(),previous:staged.state.previous.clone(),town_count:env.town_count,routes:env.routes.to_vec(),cleared_routes:Vec::new(),merchant_data:env.merchant_data.to_vec(),index:into_live(&staged.collisions).map_err(str::to_string)?,projectiles:table,error:None,destroy_requested:false,sink_notices:Vec::new(),loss_notices:Vec::new(),reports:Vec::new(),report_positions:Vec::new(),next:staged.state.next.clone(),near:staged.spatial_0.clone(),far:staged.spatial_1.clone(),captains:env.captains.to_vec(),manager:env.manager.to_vec(),captain_free_head:env.captain_free_head,captain_pool_changed:false,merchants:env.merchants.to_vec(),merchant_towns:env.merchant_towns.to_vec(),report_enabled:env.report_enabled.to_vec(),report_texts:env.report_texts,fleets:env.fleets.to_vec(),player:env.player,difficulty:env.difficulty,rng:*rng};let human=|owner:u8|env.merchants.get(usize::from(owner)).is_some_and(|v|*v==0);
 let mut globals=Globals{player:env.player,hotseat:env.hotseat,interface_present:false,interface_matches:false,merchant_words:env.merchants,fleets:env.fleets,world_count:(worlds.len()/0x180)as u16,dialog_open:0,viewed:0xff};
 let ctx=SeaCtx{captains:Captains{table:env.captains,count:(env.captains.len()/16)as u16},human_owner:&human,difficulty:env.difficulty,virtual_10:town_attack};
 sea_update::sea_update(&mut staged.state.object,&mut staged.state.local_ships,&mut staged_world,&mut staged.state.next,&staged.spatial_0,&staged.spatial_1,&mut globals,&ctx,&mut host,ticks).map_err(str::to_string)?;
 if let Some(error)=host.capture_error.take(){return Err(error)}if let Some(e)=host.error{return Err(e.into())}
 if let Some(speed)=town_speed{let read=|at:usize|u32::from_le_bytes(staged.state.object[at..at+4].try_into().unwrap());let quit=u32::from(staged.state.object[0xab1]);if read(0xa94).wrapping_mul(quit).wrapping_mul(4)<read(0xa90){return Err("decisión de velocidad de asalto pendiente".into())}staged.state.object[0xaa8..0xab0].copy_from_slice(&speed);}
 let mut global_letters=Vec::new();let mut news=Vec::new();let mut news_at=Vec::with_capacity(host.sink_notices.len());for event in &host.sink_notices{news_at.push(news.len());match event{PendingNews::Sink(captain,opponent,merchants_at_event)=>{if usize::from(*captain)<env.captains.len()/16&&usize::from(*opponent)<env.merchants.len()&&merchants_at_event.len()!=env.merchants.len()*0x650{return Err("datos de comerciantes para noticia ausentes".into())}if let Some(letter)=weapon_damage::pirate_sink_notice(*captain,*opponent,env.captains,merchants_at_event,env.alderman_town).map_err(str::to_string)?{global_letters.push(letter);news.push(BattleNews::Global(letter))}},PendingNews::Capture(item)=>{if let BattleNews::Global(letter)=item{global_letters.push(*letter)}news.push(item.clone())}}}
 let report_positions:Vec<usize>=host.report_positions.iter().map(|&p|news_at.get(p).copied().unwrap_or(news.len())).collect();
 staged_world.extend_from_slice(&host.spawned_worlds);
 if let Some(services)=host.pirate_services.as_deref_mut(){let mut pool=p3_core::ships::captains::CaptainPool{capacity:(host.captains.len()/16)as u16,entries:host.captains.clone(),free_head:host.captain_free_head};services.synchronize(PirateCaptureWorld{ships:&mut staged_world,merchants:&mut host.merchant_data,convoys:&mut host.fleets,manager:&mut host.manager,captains:&mut pool})?;if pool.entries!=host.captains||pool.free_head!=host.captain_free_head{return Err("sincronización final alteró capitanes".into())}}
 let manager_data=(host.manager!=env.manager).then(||host.manager.clone());let merchant_data=(host.merchant_data!=env.merchant_data).then(||host.merchant_data.clone());let fleet_data=(host.fleets!=env.fleets).then(||host.fleets.clone());staged.state.previous.clone_from(&host.previous);
 for ship in &mut staged.state.local_ships{ship.truncate(0x164)}
 staged.collisions=into_saved(&host.index).map_err(str::to_string)?;staged.state.bitmap=host.projectiles.bitmap;staged.state.projectiles=host.projectiles.slots.into_iter().map(|p|p.map(|v|v.to_vec()).unwrap_or_else(||vec![0;0x38])).collect();
 for(at,v)in [(0x20,staged.collisions.free_head),(0x34,staged.collisions.x_head),(0x44,staged.collisions.y_head),(0x54,staged.collisions.high_water),(0x56,host.index.capacity as u16),(0x58,(host.index.capacity*2)as u16),(0x5a,(host.index.capacity/32)as u16)]{staged.state.object[at..at+2].copy_from_slice(&v.to_le_bytes())}
 let captain_pool=host.captain_pool_changed.then(||p3_core::ships::captains::CaptainPool{capacity:(host.captains.len()/16)as u16,entries:host.captains.clone(),free_head:host.captain_free_head});
 *rng=host.rng;*battle=staged;*worlds=staged_world;Ok(BattleAdvance{alderman_reports:host.alderman_reports,sponsor_tasks:host.sponsor_tasks,manager_data,global_letters,news,merchant_data,fleet_data,capture_events:host.capture_events,cleared_routes:host.cleared_routes,captain_pool,destroyed:host.destroy_requested,loss_notices:host.loss_notices,reports:host.reports,report_positions})
}


/// Connects the independently contrasted original band-growth implementation.
impl PirateCaptureServices for crate::pirate_capture::PirateCaptureState {
 fn finish_clock(&self)->Option<(u32,u32)>{Some((self.tick,self.news_stamp))}
 fn captain_resources(&self)->Option<p3_core::economy::pirate_group_callbacks::CaptainResources>{self.captain}
 fn hunt_convoy(&self,convoy:u16)->bool{self.hunt_convoys.contains(&convoy)}
 fn hunt_task(&mut self,convoy:u16)->Option<&mut p3_core::scheduler::TaskRecord>{self.hunt_tasks.iter_mut().find(|(_,r)|u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)])==convoy).map(|(_,r)|r)}
 fn hunt_loss(&mut self,convoy:u16,loss:p3_core::ships::return_convoy::HuntLoss)->Result<(),String>{let slot=self.hunt_tasks.iter().find(|(_,r)|u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)])==convoy).map(|(i,_)|*i).ok_or("tarea de la caza ausente")?;self.hunt_added.push((slot,loss));Ok(())}
 fn finish_news(&mut self,stamp:u32,names:Vec<u8>){self.news_stamp=stamp;self.news.extend(names)}
 fn objects(&self)->[bool;5]{std::array::from_fn(|i|self.bands.get(i).is_some_and(Option::is_some))}
 fn synchronize(&mut self,w:PirateCaptureWorld<'_>)->Result<(),String>{
  if self.bands.len()!=5||w.manager.len()<0xf8||w.ships.len()%0x180!=0||w.convoys.len()%0x3c!=0||w.captains.entries.len()!=usize::from(w.captains.capacity)*16{return Err("tablas de fábrica pirata truncadas".into())}
  let r=|at:usize|u16::from_le_bytes([w.manager[at],w.manager[at+1]]);
  if usize::from(r(0xf4))!=w.ships.len()/0x180||usize::from(r(0xf6))!=w.convoys.len()/0x3c||r(0xf2)!=w.captains.capacity||r(0xe0)!=w.captains.free_head{return Err("gestor de fábrica pirata incoherente".into())}
  self.factory.ships=w.ships.chunks_exact(0x180).map(<[u8]>::to_vec).collect();self.factory.convoys=w.convoys.chunks_exact(0x3c).map(<[u8]>::to_vec).collect();self.factory.captains=w.captains.entries.chunks_exact(16).map(|c|c.try_into().unwrap()).collect();
  self.factory.free_captain=r(0xe0);self.factory.free_ship=r(0xe2);self.factory.free_convoy=r(0xe4);self.factory.convoy_head=r(0xec);self.factory.name_cursor=r(0xee);self.factory.name_step=r(0xf0);Ok(())
 }
 fn remove_ship(&mut self,ships:&mut [u8],target:u16)->Result<(),String>{
  // 0x515F00 on each band head (band+0xC = header+4), links ship+4.
  let count=ships.len()/0x180;let mut bands=self.bands.clone();
  for band in bands.iter_mut().flatten(){let mut cur=u16::from_le_bytes([band.header[4],band.header[5]]);let mut prev:Option<usize>=None;let mut seen=vec![false;count];
   while usize::from(cur)<count{let i=usize::from(cur);if seen[i]{return Err("cadena de banda cíclica".into())}seen[i]=true;let next=[ships[i*0x180+4],ships[i*0x180+5]];
    if cur==target{match prev{Some(p)=>ships[p*0x180+4..p*0x180+6].copy_from_slice(&next),None=>band.header[4..6].copy_from_slice(&next)}ships[i*0x180+4..i*0x180+6].fill(0xff);break}
    prev=Some(i);cur=u16::from_le_bytes(next)}}
  self.bands=bands;Ok(())
 }
 fn register_capture(&mut self,index:u8,mut w:PirateCaptureWorld<'_>)->Result<(),String>{
  self.synchronize(PirateCaptureWorld{ships:&mut w.ships,merchants:&mut w.merchants,convoys:&mut w.convoys,manager:&mut w.manager,captains:&mut w.captains})?;
  crate::pirate_capture::PirateCaptureState::register_capture(self,index)?;
  *w.ships=self.factory.ships.concat();*w.convoys=self.factory.convoys.concat();w.captains.entries=self.factory.captains.iter().flatten().copied().collect();w.captains.capacity=u16::try_from(self.factory.captains.len()).map_err(|_|"demasiados capitanes")?;w.captains.free_head=self.factory.free_captain;
  for(at,value)in [(0xe0,self.factory.free_captain),(0xe2,self.factory.free_ship),(0xe4,self.factory.free_convoy),(0xec,self.factory.convoy_head),(0xee,self.factory.name_cursor),(0xf0,self.factory.name_step),(0xf2,w.captains.capacity),(0xf4,u16::try_from(self.factory.ships.len()).map_err(|_|"demasiados barcos")?),(0xf6,u16::try_from(self.factory.convoys.len()).map_err(|_|"demasiados convoyes")?)]{w.manager[at..at+2].copy_from_slice(&value.to_le_bytes())}Ok(())
 }
}
