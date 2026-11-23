//! Creación de oficina en el estado vivo, en la frontera del planificador.
use crate::{Sim,Office,OfficeHousing,OfficeLists,PersistedOffice};
use p3_core::economy::{office_factory::{self,OfficePool},extra_tax::TaxOffice};
use p3_save::office::{decode_office,encode_office};

impl Sim {
 pub(crate) fn current_office_pool(&self)->Result<OfficePool,String>{
  let mut pool=self.host.office_pool.clone().ok_or("pool de oficinas ausente")?;
  if pool.records.len()!=self.persisted.len()||pool.records.len()!=self.economy.world.offices.len()||pool.records.len()!=self.economy.offices.len()||pool.records.len()!=self.economy.accounts.offices.len()||pool.records.len()!=self.businesses.len()||pool.records.len()!=self.wages.len()||pool.records.len()!=self.host.housing.len()||pool.records.len()!=self.host.lists.len()||pool.records.len()!=self.host.tax_offices.len()||pool.records.len()!=self.host.employment_counts.len(){return Err("vistas de oficinas incompletas".into())}
  // Las ranuras libres son las de la cadena libre del propio pool (enlace +0x2C8): las vistas de oficina las cargan
  // con dueño 0, así que no sirven para distinguirlas; refrescarlas pisaría el enlace libre.
  let mut free=vec![false;pool.records.len()];let mut at=usize::from(pool.free);while at<pool.records.len(){if free[at]{return Err("cadena libre de oficinas cíclica".into())}free[at]=true;at=usize::from(u16::from_le_bytes([pool.records[at][0x2c8],pool.records[at][0x2c9]]));}
  for i in 0..pool.records.len(){if free[i]||self.economy.offices[i].owner==u16::MAX{continue}let mut image=decode_office(&self.persisted[i].to_save_v85());let live=&self.economy.world.offices[i];image[0x2ca..0x2cc].copy_from_slice(&live.next.to_le_bytes());image[0x2cc..0x2ce].copy_from_slice(&live.first_facility.to_le_bytes());pool.records[i]=image.to_vec();}
  Ok(pool)
 }
 pub(crate) fn create_live_office(&mut self,owner:u8,city:u8,tick:u32)->Result<(),String>{
  let mut pool=self.current_office_pool()?;let mut towns=self.economy.world.towns.clone();let mut merchants=self.rep_merchants.clone();let mut messages=self.host.message_pool.clone();
  let outcome=office_factory::find_or_create(&mut pool,&mut towns,&mut merchants,self.economy.world.merchant_count,u16::from(owner),u16::from(city),true,tick,self.economy.world.world_8a!=0).map_err(str::to_string)?;
  let id=usize::from(outcome.index.ok_or("oficina no creada")?);let raw:&[u8;0x44c]=pool.records[id].as_slice().try_into().map_err(|_|"oficina truncada")?;
  if let Some(letter)=outcome.letter{let c=p3_core::calendar::Calendar::for_tick(tick);p3_state::messages::add_global_inline_letter(&mut messages,c.day,c.month,c.year,&letter)?;}
  let saved=encode_office(raw);let persisted=PersistedOffice::from_save_v85(&saved);let stock=persisted.stock.clone();let account=persisted.accounts.clone();
  let old=self.persisted.len();let total=pool.records.len();if total<old{return Err("pool de oficinas reducido".into())}
  for _ in old..total {
   let mut empty=[0u8;0x44c];p3_core::economy::office_factory::init(&mut empty,false).map_err(str::to_string)?;let saved=encode_office(&empty);let p=PersistedOffice::from_save_v85(&saved);
   self.economy.offices.push(p.stock.clone());self.economy.accounts.offices.push(p.accounts.clone());self.persisted.push(p);self.economy.world.offices.push(Office{weapons:[0;4],next:u16::MAX,first_facility:u16::MAX});self.businesses.push(Vec::new());self.wages.push(None);self.host.housing.push(OfficeHousing{next:u16::MAX,houses_head:u16::MAX,..Default::default()});self.host.lists.push(OfficeLists{next:u16::MAX,building_head:u16::MAX});self.host.tax_offices.push(TaxOffice{owner:u16::MAX,next:u16::MAX,businesses:0,houses:0});self.host.employment_counts.push(None);
  }
  let next=u16::from_le_bytes([raw[0x2ca],raw[0x2cb]]);self.persisted[id]=persisted;self.economy.offices[id]=stock;self.economy.accounts.offices[id]=account;self.economy.world.offices[id]=Office{weapons:[0;4],next,first_facility:u16::MAX};self.businesses[id].clear();self.wages[id]=None;
  self.host.housing[id]=OfficeHousing{next,houses_head:u16::MAX,level:[2;3],..Default::default()};self.host.lists[id]=OfficeLists{next,building_head:u16::MAX};self.host.tax_offices[id]=TaxOffice{owner:u16::from(owner),next,businesses:0,houses:0};self.host.employment_counts[id]=None;
  self.economy.world.office_count=total as u16;self.host.office_count=total as u16;self.economy.world.towns=towns;self.rep_merchants=merchants.clone();self.economy.merchants=merchants;self.host.message_pool=messages;self.host.office_pool=Some(pool);Ok(())
 }
}

#[cfg(test)]
mod expansion_tests{
 use super::*;
 use p3_core::economy::world::TownHost;
 use std::io::Write;
 fn hex(b:&[u8])->String{b.iter().map(|v|format!("{v:02x}")).collect()}
 fn unhex(s:&str)->Vec<u8>{s.as_bytes().chunks_exact(2).map(|v|u8::from_str_radix(std::str::from_utf8(v).unwrap(),16).unwrap()).collect()}
 #[test]
 fn ai_expansion_creates_office_and_reloads_like_x86(){
  let (Ok(root),Ok(exe),Ok(python))=(std::env::var("P3_SAVES"),std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY"))else{return};let raw=std::fs::read(format!("{root}/lab-english/save/Ein/TEST25_ConvoyMar.pat")).unwrap();
  for grow in [false,true]{let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();let probe=Sim::from_save_bytes(&model.write()).unwrap();let tick=probe.idle.clock.tick;let owner=20u8;assert!(model.world.merchants.len()>usize::from(owner));let city=(0..probe.economy.world.towns.len()).find(|&c|!probe.economy.offices.iter().any(|o|o.owner==u16::from(owner)&&usize::from(o.town)==c)).unwrap()as u8;
   for(i,record)in model.world.merchants.iter_mut().enumerate(){let mut m=p3_state::office::PersistedMerchant::from_save_v85(record).unwrap();m.merchant.set_u16(8,u16::from(i==usize::from(owner)));m.merchant.set_u16(10,u16::MAX);if i==usize::from(owner){m.merchant.set_i32(0,1000000);m.merchant.set_u16(0xc,u16::MAX)}*record=m.to_save_v85()}if grow{let capacity=model.world.offices.len()as u16;model.world.header[0x1e..0x20].copy_from_slice(&capacity.to_le_bytes());}
   let task=p3_core::scheduler::TaskRecord{due:tick,next:u16::MAX,kind:0x2a,..Default::default()};let record=[task.due.to_le_bytes().as_slice(),task.next.to_le_bytes().as_slice(),task.kind.to_le_bytes().as_slice(),task.payload.as_slice()].concat();model.scheduled_tasks=[0u16.to_le_bytes().as_slice(),u16::MAX.to_le_bytes().as_slice(),1u16.to_le_bytes().as_slice(),record.as_slice()].concat();let mut pool=vec![0u8;12+8*16];pool[..4].copy_from_slice(&0xaffe0005u32.to_le_bytes());pool[4..6].copy_from_slice(&8u16.to_le_bytes());pool[8..10].copy_from_slice(&u16::MAX.to_le_bytes());for i in 0..8{pool[12+16*i+4]=0xff;pool[12+16*i+6..12+16*i+8].copy_from_slice(&((i+1)as u16).to_le_bytes())}model.message_pool=pool;
   let mut sim=Sim::from_save_bytes(&model.write()).unwrap();for(i,town)in sim.economy.world.towns.iter_mut().enumerate(){let flags=town.u32(0x76c);town.set_u32(0x76c,if i==usize::from(city){flags|0x100}else{flags&!0x100});if i==usize::from(city){town.set_u16(0x848+3*16,1001)}}sim.persist_all(&mut model).unwrap();sim=Sim::from_save_bytes(&model.write()).unwrap();let pool=sim.current_office_pool().unwrap();assert_eq!(usize::from(pool.free)>=pool.records.len(),grow,"la prueba debe cubrir reserva y crecimiento");let offices=pool.records.concat();let towns=sim.economy.world.towns.iter().flat_map(|t|t.0.iter().copied()).collect::<Vec<_>>();let merchants=sim.rep_merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();let mut businesses=sim.host.employment_businesses.clone();for(b,f)in businesses.iter_mut().zip(&sim.economy.world.merchant_facilities){b[4..6].copy_from_slice(&f.employees.to_le_bytes());b[8..10].copy_from_slice(&f.next.to_le_bytes())}let d=p3_core::calendar::Calendar::for_tick(tick);let input=serde_json::json!({"binary":exe,"tasks":hex(&model.scheduled_tasks),"towns":hex(&towns),"merchants":hex(&merchants),"offices":hex(&offices),"businesses":hex(&businesses.concat()),"town_count":sim.economy.world.towns.len(),"merchant_count":sim.economy.world.merchant_count,"office_count":pool.records.len(),"office_free":pool.free,"business_count":businesses.len(),"world_8a":sim.economy.world.world_8a,"pool":hex(&model.message_pool),"tick":tick,"date":{"day":d.day,"month":d.month,"year":d.year,"doy":d.day_of_year},"owner":owner,"city":city});let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_ai_expansion_live.py");let mut child=std::process::Command::new(&python).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let expected:serde_json::Value=serde_json::from_slice(&out.stdout).unwrap();
   let dispatch=|sim:&mut Sim|{let mut bridge=p3_state::office::OfficeTownHost{offices:&mut sim.persisted,merchants:&mut sim.rep_merchants,businesses:&mut sim.businesses,captain_wages:&mut sim.wages,difficulty:sim.commercial_sell_factor,other:&mut sim.host,completed:Vec::new(),errors:Vec::new()};bridge.dispatch_scheduled_tasks_fleet(&mut sim.economy.world,&mut sim.economy.merchants,&[],&mut sim.economy.offices,&mut sim.economy.cargo_ships,2.0,tick);assert!(sim.host.council_errors.is_empty(),"{:?}",sim.host.council_errors)};dispatch(&mut sim);assert_eq!(sim.host.pending_office,Some((owner,city,tick)));assert!(sim.host.pending_ai_expansion.is_some());let before=model.write();assert!(sim.persist_all(&mut model).is_err());assert_eq!(model.write(),before);{let mut clock=crate::merchant_daily::DailyClock{sim:&mut sim,world_ready:false,error:None};p3_core::tick::TickSystems::scheduled_tasks_tick(&mut clock,tick);assert!(clock.error.is_none(),"{:?}",clock.error);}assert!(sim.host.pending_ai_expansion.is_none());sim.persist_all(&mut model).unwrap();assert_eq!(model.scheduled_tasks,unhex(expected["tasks"].as_str().unwrap()));assert_eq!(model.message_pool,unhex(expected["pool"].as_str().unwrap()));assert_eq!(model.world.merchants[owner as usize],unhex(expected["merchant"].as_str().unwrap()));assert_eq!(u16::from_le_bytes(model.world.header[0x1e..0x20].try_into().unwrap()),expected["office_free"].as_u64().unwrap()as u16);let actual=model.world.offices.iter().map(|v|hex(v)).collect::<Vec<_>>();assert_eq!(serde_json::json!(actual),expected["offices"]);let loaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(loaded.economy.world.towns[city as usize].u16(0x784),expected["town_head"].as_u64().unwrap()as u16);assert!(loaded.economy.offices.iter().any(|o|o.owner==u16::from(owner)&&o.town==city));
  }
 }
}

#[cfg(test)]
mod free_chain_tests{
 use super::*;
 /// Las ranuras libres del guardado se cargan con dueño 0 en las vistas de oficina: dos altas seguidas deben
 /// recorrer la cadena libre del pool (37 → 38 → 39) sin reutilizar oficinas ocupadas.
 #[test]
 fn consecutive_offices_follow_the_saved_free_chain(){
  let Ok(root)=std::env::var("P3_SAVES")else{return};let raw=std::fs::read(format!("{root}/lab-english/save/Ein/TEST25_ConvoyMar.pat")).unwrap();
  let mut sim=Sim::from_save_bytes(&raw).unwrap();let tick=sim.idle.clock.tick;
  let first=sim.current_office_pool().unwrap().free;assert_eq!(first,37);
  let used=|sim:&Sim|(0..sim.economy.world.towns.len()).flat_map(|t|{let mut v=Vec::new();let mut p=sim.economy.world.towns[t].u16(0x784);while usize::from(p)<sim.economy.world.offices.len()&&v.len()<=sim.economy.world.offices.len(){v.push(p);p=sim.economy.world.offices[usize::from(p)].next;}v}).collect::<Vec<_>>();
  let before=used(&sim);
  for (k,(owner,city)) in [(25u8,1u8),(26,2)].into_iter().enumerate(){
   sim.create_live_office(owner,city,tick).unwrap();
   assert_eq!(sim.host.office_pool.as_ref().unwrap().free,first+1+k as u16,"cabeza libre tras el alta {k}");
  }
  let after=used(&sim);
  let mut sorted=after.clone();sorted.sort();sorted.dedup();
  assert_eq!(sorted.len(),after.len(),"una oficina en dos cadenas de ciudad");
  assert_eq!(after.len(),before.len()+2);
  assert!(after.contains(&37)&&after.contains(&38));
 }
}
