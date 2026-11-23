//! Selección de obra defensiva y generación de operaciones sin publicar efectos parciales.
use p3_core::{buildings::{Site,defense::DefenseHost},economy::{defense_planner::{self,DefenseAction},world::EconomyWorld},pump::Operation,rng::CrtRand,scheduler::Scheduler,town::{off,Town}};
struct Host<'a>{world:&'a EconomyWorld,ops:Vec<Operation>}
impl DefenseHost for Host<'_>{
 fn add_map_object(&mut self,_:u32,_:u32,_:u32,_:u32,_:u32){unreachable!("el planificador no dibuja")}
 fn enqueue_operation(&mut self,record:&[u32;5]){self.ops.push(Operation(*record))}
 fn merchant_fields(&mut self,id:u32)->(u16,u8){let m=&self.world.merchants[id as usize];(m.kind,m.home_town)}
}
pub fn plan(t:&Town,site:&Site,world:&EconomyWorld,scheduler:&Scheduler,rng:&mut Option<CrtRand>)->Result<Vec<Operation>,String>{
 if usize::from(world.merchant_count)>world.merchants.len(){return Err("comerciantes de defensa incompletos".into())}
 let count=t.u16(off::BUILDING_RECORD_COUNT);let mut p=t.u16(off::DEFENSE_HEAD);let mut seen=vec![false;usize::from(count)];
 while p<count{let i=usize::from(p);if seen[i]||i*8+8>site.records.len(){return Err("cadena defensiva inválida".into())}seen[i]=true;p=site.next(i)}
 let local_human=world.merchants[..usize::from(world.merchant_count)].iter().any(|m|m.kind==0&&m.home_town==t.index());
 let action=defense_planner::plan(t,site,world.day_of_year,local_human).map_err(str::to_string)?;
 let mut host=Host{world,ops:vec![]};
 match action{
  Some(DefenseAction::Upgrade)=>site.queue_next_defense_upgrade_op(t,&mut host),
  Some(DefenseAction::QueuePitch)=>site.queue_op_upgrade_first_tower_site(t,&mut host),
  Some(DefenseAction::QueuePitchShoot)=>site.queue_op_for_first_pitch_shoot(t,world.merchant_count,&mut host),
  Some(DefenseAction::PlaceGate)=>return crate::housing_planner::plan_gate(t,site,world,scheduler,rng),
  None=>{}
 }
 Ok(host.ops)
}
