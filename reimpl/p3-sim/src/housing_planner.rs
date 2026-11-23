//! Conecta las decisiones de vivienda al plano, CRT y oferta del concejo.
use p3_core::{buildings::{Site,construction::{BuildCtx,BuildHost,Hook,PlanCtx}},economy::{council,housing_planner::HousingPlannerHost,world::EconomyWorld},pump::Operation,rng::CrtRand,scheduler::Scheduler,town::{off,Town}};

#[derive(Default)]
struct Build {op:Option<Operation>,unexpected:bool}
impl BuildHost for Build {
 fn call(&mut self,hook:Hook,args:&[u32])->u32{if hook==Hook::EnqueueOperation&&args.len()==5{self.op=Some(Operation([args[0],args[1],args[2],args[3],args[4]]));0}else{self.unexpected=true;0}}
 fn office_building_head(&mut self,_:u32)->u16{self.unexpected=true;u16::MAX}
}
struct Host<'a>{town:&'a Town,site:&'a Site,world:&'a EconomyWorld,scheduler:&'a Scheduler,rng:&'a mut Option<CrtRand>,ops:Vec<Operation>}
impl HousingPlannerHost for Host<'_>{
 type Error=String;
 fn place(&mut self,kind:u8)->Result<u32,String>{
  crate::lender::validate(self.town,self.site,self.world.merchant_count)?;
  let count=self.town.u16(off::BUILDING_RECORD_COUNT);let mut at=self.town.u16(off::PUBLIC_BUILDING_HEAD);let mut seen=vec![false;usize::from(count)];
  while at<count{let i=usize::from(at);if seen[i]||i*8+8>self.site.records.len(){return Err("lista pública de edificios inválida".into())}seen[i]=true;at=self.site.next(i)}
  let mut t=self.town.clone();let mut s=self.site.clone();let mut host=Build::default();let mut gold=0;let mut income=0;let classes=[0;256];let mut rng=self.rng.unwrap_or_default();let before=rng.state;
  let build=BuildCtx{merchant_count:self.world.merchant_count,visual_owner:None,special:0xe0,owner_class:&classes,merchant_gold:&mut gold,merchant_income:&mut income,host:&mut host};
  let mut ctx=PlanCtx{build,rng:&mut rng,day:u32::from(self.world.day_of_year),month:self.world.month,alt_special:0x20};
  let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||s.place_planned_building(&mut t,&mut ctx,kind,false))).map_err(|_|"solar de vivienda fuera del plano")?;
  if host.unexpected{return Err("dependencia inesperada de vivienda".into())}
  if rng.state!=before&&self.rng.is_none(){return Err("estado CRT compartido no suministrado".into())}
  if self.rng.is_some(){*self.rng=Some(rng)}if let Some(op)=host.op{self.ops.push(op)}Ok(result)
 }
 fn expand_walls(&mut self)->Result<(),String>{if let Some(op)=council::propose_wall_enlargement(self.town,self.world.tick,self.scheduler,self.world.world_8a){self.ops.push(op)}Ok(())}
 fn invalid_construction_chain(&mut self)->String{"cadena de obras inválida".into()}
}

pub fn plan(town:&Town,site:&Site,world:&EconomyWorld,scheduler:&Scheduler,rng:&mut Option<CrtRand>)->Result<Vec<Operation>,String>{
 let mut staged=*rng;let mut host=Host{town,site,world,scheduler,rng:&mut staged,ops:Vec::new()};
 p3_core::economy::housing_planner::plan(town,site,world.day_of_year,&mut host)?;
 let ops=host.ops;*rng=staged;Ok(ops)
}

/// Búsqueda de puerta defensiva con el mismo CRT que el resto del planificador.
pub(crate) fn plan_gate(town:&Town,site:&Site,world:&EconomyWorld,scheduler:&Scheduler,rng:&mut Option<CrtRand>)->Result<Vec<Operation>,String>{
 let mut staged=*rng;let mut host=Host{town,site,world,scheduler,rng:&mut staged,ops:Vec::new()};
 host.place(0x30)?;let ops=host.ops;*rng=staged;Ok(ops)
}
