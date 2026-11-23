//! Dependencias con estado real para 514D40: cartas, tareas, bajas y rutas.
use p3_core::{flatmem::Mem,calendar::Calendar,ai::reputation::Merchant,ships::remove::RemoveEvent,economy::pirate_b5_memory::{Effect,Effects,NativeBasics},scheduler::Scheduler};
use crate::route_stop_runtime::RouteStopRuntime;
const TASK_SCRATCH:u32=0xfffd0100;
#[derive(Clone)]
pub struct PirateB5Effects {
 pub native:NativeBasics,pub message_pool:Vec<u8>,pub scheduler:Scheduler,
 pub route_stops:RouteStopRuntime,pub merchant_groups:Vec<p3_save::world::MerchantShipGroups>,
 pub calendar:Calendar,pub notifications:Vec<u32>,pending_task:Option<u16>,
}
impl PirateB5Effects {
 pub fn new(native:NativeBasics,message_pool:Vec<u8>,scheduler:Scheduler,route_stops:RouteStopRuntime,merchant_groups:Vec<p3_save::world::MerchantShipGroups>,calendar:Calendar)->Self{
  Self{native,message_pool,scheduler,route_stops,merchant_groups,calendar,notifications:Vec::new(),pending_task:None}
 }
 /// Tras `pirate_b5_memory::prepare`, copia los bytes finales de la tarea que
 /// 514D40 rellenó en el temporal. Debe ejecutarse antes de publicar nada.
 pub fn finalize(&mut self,m:&mut dyn Mem)->Result<(),String>{
  if let Some(id)=self.pending_task.take(){let r=self.scheduler.records.get_mut(usize::from(id)).ok_or("tarea B5 ausente")?;let mut b=[0u8;24];for(i,v)in b.iter_mut().enumerate(){*v=m.r8(TASK_SCRATCH+i as u32)}r.due=u32::from_le_bytes(b[..4].try_into().unwrap());r.next=u16::from_le_bytes(b[4..6].try_into().unwrap());r.kind=u16::from_le_bytes(b[6..8].try_into().unwrap());r.payload.copy_from_slice(&b[8..24]);}Ok(())
 }
 fn check_scheduler(&self)->Result<(),String>{
  let s=&self.scheduler;if s.records.len()!=usize::from(s.capacity){return Err("planificador B5 truncado".into())}if s.free_head>=s.capacity&&u32::from(s.capacity)+128>u32::from(u16::MAX){return Err("planificador B5 sin capacidad".into())}
  let mut at=s.head;let mut seen=vec![false;usize::from(s.capacity)];while at<s.capacity{let i=usize::from(at);if seen[i]{return Err("planificador B5 cíclico".into())}seen[i]=true;at=s.records[i].next}Ok(())
 }
 fn free_slot(&mut self,m:&mut dyn Mem,id:u16,followers:bool)->Result<(),String>{
  let before=self.route_stops.bytes().to_vec();if before.len()<4{return Err("depósito B5 truncado".into())}let count=u16::from_le_bytes([before[2],before[3]]);if m.r16(0x6dd72a)!=count{return Err("capacidad del depósito B5 desincronizada".into())}
  let mut next=before.clone();let mut head=u16::from_le_bytes([next[0],next[1]]);
  if followers{let base=m.r32(0x6dd7a4);let n=usize::from(m.r16(0x6dd894))*0x180;let mut ships=(0..n).map(|i|m.r8(base+i as u32)).collect::<Vec<_>>();p3_core::economy::pirate_group_callbacks::free_route_stop_pool_with_followers(&mut head,&mut next[4..],&mut ships,id)?;for(i,&v)in ships.iter().enumerate(){m.w8(base+i as u32,v)}}else{p3_core::economy::pirate_group_callbacks::free_route_stop_pool(&mut head,&mut next[4..],id)?}
  next[..2].copy_from_slice(&head.to_le_bytes());self.route_stops.replace(&before,next)
 }
 fn finish_ship(&mut self,m:&mut dyn Mem,id:u16)->Result<(),String>{
  let limit=m.r16(0x6dd72a);let delta=crate::pirate_group_remove::remove(m,id,self.native.player,limit)?;if let Some(id)=delta.cleared_route{self.native.cleared_routes.push(id)}let mut snapshots=delta.merchant_owners.into_iter();
  for event in delta.events{match event{
   RemoveEvent::PoolRelease(id)=>self.free_slot(m,id,false)?,
   RemoveEvent::MerchantChanged(owner)=>{let (who,owners)=snapshots.next().ok_or("instantánea de comerciante ausente")?;if who!=owner{return Err("orden de comerciantes incoherente".into())}let groups=self.merchant_groups.get_mut(owner as usize).ok_or("grupos de comerciante ausentes")?;if groups.iter().flatten().any(|&id|usize::from(id)>=owners.len()){return Err("barco de grupo fuera de rango".into())}let ptr=m.r32(0x6de518)+owner*0x650;let mut person=Merchant{bytes:(0..0x650).map(|i|m.r8(ptr+i)).collect()};person.prune_foreign_ships_from_groups(owner as u8,groups,&|id|owners[usize::from(id)]);for(i,&b)in person.bytes.iter().enumerate(){m.w8(ptr+i as u32,b)}},
   other=>return Err(format!("efecto de baja B5 no conectado: {other:?}")),
  }}if snapshots.next().is_some(){return Err("instantáneas de comerciantes sobrantes".into())}Ok(())
 }
}
impl Effects for PirateB5Effects {
 fn native_loadout(&self)->bool{true}
 fn finalize(&mut self,m:&mut dyn Mem)->Result<(),String>{PirateB5Effects::finalize(self,m)}
 fn call(&mut self,m:&mut dyn Mem,effect:Effect)->Result<u32,String>{match effect{
  Effect::SendLetter(letter)=>{let c=self.calendar;p3_state::messages::add_global_inline_letter(&mut self.message_pool,c.day,c.month,c.year,&letter)?;Ok(0)},
  Effect::NewTask(due)=>{if self.pending_task.is_some(){return Err("dos tareas B5 simultáneas".into())}self.check_scheduler()?;let id=self.scheduler.add_task(due);let t=self.scheduler.records[usize::from(id)];let mut raw=[0u8;24];raw[..4].copy_from_slice(&t.due.to_le_bytes());raw[4..6].copy_from_slice(&t.next.to_le_bytes());raw[6..8].copy_from_slice(&t.kind.to_le_bytes());raw[8..].copy_from_slice(&t.payload);for(i,&v)in raw.iter().enumerate(){m.w8_seed(TASK_SCRATCH+i as u32);m.w8(TASK_SCRATCH+i as u32,v)}self.pending_task=Some(id);Ok(TASK_SCRATCH)},
  Effect::ShipFinish(id)=>{self.finish_ship(m,id)?;Ok(0)},
  Effect::FreeSlot(id)=>{self.free_slot(m,id,true)?;Ok(0)},
  Effect::Notify(ptr)=>{self.notifications.push(ptr);Ok(0)},
  other=>self.native.call(m,other),
 }}
}
