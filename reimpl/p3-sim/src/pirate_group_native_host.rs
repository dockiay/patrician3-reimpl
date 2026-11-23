//! Dependencias nativas del controlador 514000. Las rutas que todavía
//! requieren adaptar servicios de mundo fallan explícitamente antes de publicar.
use p3_core::{economy::{pirate_group_callbacks::{self,CaptainResources},pirate_group_tick::{self,PirateGroupTickHost}},flatmem::Mem,pump::Operation};

#[derive(Clone)]
pub struct CreateInputs{pub town0:p3_core::town::Town,pub port0:(u16,u16),pub ports:Vec<(u16,u16)>,pub weapon_experience:u32,pub ship_names:Vec<Vec<u8>>}
#[derive(Clone)]
pub struct NativeHost {
 pub captain:CaptainResources,
 pub captain_population_available:bool,
 pub player:u32,
 pub pool_limit:u16,
 pub cleared_routes:Vec<u16>,
 pub remove_events:Vec<p3_core::ships::remove::RemoveEvent>,
 pub removed_plans:Vec<usize>,
 pub merchant_owners:Vec<(u32,Vec<u8>)>,
 pub create:Option<CreateInputs>,
 pub operations:Vec<Operation>,
}
impl NativeHost {
 fn ship(m:&mut dyn Mem,address:u32)->Result<Vec<u8>,String>{
  let base=m.r32(0x6dd7a4);let count=u32::from(m.r16(0x6dd894));
  let offset=address.checked_sub(base).ok_or("puntero naval anterior al pool")?;
  if offset%0x180!=0||offset/0x180>=count{return Err("puntero naval fuera del pool".into())}
  Ok((0..0x180).map(|i|m.r8(address+i)).collect())
 }
 fn write_ship(m:&mut dyn Mem,address:u32,ship:&[u8]){for(i,&b)in ship.iter().enumerate(){m.w8(address+i as u32,b)}}
}
impl PirateGroupTickHost for NativeHost {
 fn remove_ship(&mut self,m:&mut dyn Mem,id:u16)->Result<(),String>{
  let delta=crate::pirate_group_remove::remove(m,id,self.player,self.pool_limit)?;
  if let Some(id)=delta.cleared_route{self.cleared_routes.push(id)}self.merchant_owners.extend(delta.merchant_owners);self.remove_events.extend(delta.events);Ok(())
 }
 fn dissolve(&mut self,m:&mut dyn Mem,band:u32)->Result<(),String>{
  let delta=crate::pirate_group_remove::dissolve(m,band,self.player,self.pool_limit)?;
  if delta.removed_plan{let id=band.checked_sub(crate::pirate_group_runtime::BAND).filter(|v|v%0x10000==0).map(|v|(v/0x10000)as usize).filter(|&v|v<5).ok_or("puntero de cuadrilla inválido")?;self.removed_plans.push(id)}
  self.cleared_routes.extend(delta.cleared_routes);self.merchant_owners.extend(delta.merchant_owners);self.remove_events.extend(delta.events);Ok(())
 }
 fn destroy_band(&mut self,m:&mut dyn Mem,band:u32)->Result<(),String>{
  // El pool de staging se libera al terminar la transacción. El controlador
  // pone la ranura del gestor a cero y el puente publica None.
  if m.r32(band)!=0||m.r32(band+4)!=0{return Err("514000: liberar cuadrilla no disuelta".into())}Ok(())
 }
 fn create_ship(&mut self,m:&mut dyn Mem,band:u32,kind:u32,a1:u32,power:u32)->Result<u32,String>{
  if a1 as u8!=0&&!self.captain_population_available{return Err("514000: ciudad de población para capitán ausente".into())}
  let c=self.create.as_ref().ok_or("514000: recursos de alta naval ausentes")?;let port=*c.ports.get(usize::from(m.r8(band+0xe))).ok_or("puerto de cuadrilla ausente")?;
  crate::pirate_group_create::create(m,band,kind,a1,power,&crate::pirate_group_create::CreateResources{town0:&c.town0,port0:c.port0,band_port:port,weapon_experience:c.weapon_experience,ship_names:&c.ship_names,captain:self.captain})
 }
 fn upgrade(&mut self,m:&mut dyn Mem,address:u32,level:u32)->Result<(),String>{let mut ship=Self::ship(m,address)?;crate::absorb_ship_refresh::refresh(&mut ship,level).map_err(str::to_owned)?;Self::write_ship(m,address,&ship);Ok(())}
 fn crew(&mut self,m:&mut dyn Mem,address:u32)->Result<u16,String>{let ship=Self::ship(m,address)?;pirate_group_tick::target_crew(&ship).ok_or("dotación pirata inválida".into())}
 fn create_captain(&mut self,m:&mut dyn Mem)->Result<u16,String>{if !self.captain_population_available{return Err("514000: ciudad de población para capitán ausente".into())}pirate_group_callbacks::allocate_pirate_captain(m,self.captain)}
 fn enqueue(&mut self,_:&mut dyn Mem,record:[u32;5])->Result<(),String>{self.operations.push(Operation(record));Ok(())}
}

#[cfg(test)]
mod tests {
 use super::*;use crate::{pirate_group_runtime::{self,GroupEnv},convoy_absorption::AbsorptionState};use p3_core::ships::{remove::RemoveWorld,tick::TickWorld};
 #[test] fn empty_destroyed_band_clears_merchant_references(){
  let mut manager=vec![0u8;0x110];let mut ships=vec![];let mut caps=vec![];let mut cvs=vec![];let mut merchants=vec![0u8;0x650];merchants[0xe0]=12;merchants[0xe1]=3;let mut towns=vec![];
  let mut remove=RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut caps,convoys:&mut cvs,merchants:&mut merchants,towns:&mut towns,pool_limit:0,merchant_count:1,town_count:0,ui_a:false,ui_b:false};let w=TickWorld{remove:&mut remove,time:0,player:0};
  let mut header=[0u8;16];header[4..6].fill(255);header[6]=3;header[8..12].fill(255);header[13]=1;let band=p3_save::pirate_bands::PirateBand{header,plan:None,pieces:vec![]};let state=AbsorptionState{bands:vec![Some(band),None,None,None,None],day:1,network_client:0,operations:vec![]};
  let env=GroupEnv{calendar_byte:1,suppress_events:0,operation_host:1,level_thresholds:[0;256],extra_memory:vec![]};let host=NativeHost{captain_population_available:true,player:0,pool_limit:0,cleared_routes:vec![],remove_events:vec![],removed_plans:vec![],merchant_owners:vec![],create:None,captain:CaptainResources{tick:0,day_of_year:1,day:1,population:0,first_names:1,last_names:1},operations:vec![]};
  let out=pirate_group_runtime::prepare(&w,&state,0,&env,&host).unwrap();assert!(out.state.bands[0].is_none());assert_eq!(&out.merchants[0xe0..0xe2],&[255,255]);assert_eq!(&w.remove.merchants[0xe0..0xe2],&[12,3]);
 }
 #[test] fn full_group_spawns_reserve_and_captain_like_original(){
  use std::io::Write;use sha2::{Digest,Sha256};let (Ok(exe),Ok(py))=(std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY"))else{return};let binary=std::fs::read(&exe).unwrap();assert_eq!(format!("{:x}",Sha256::digest(&binary)),"fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece");
  let mut manager=vec![0u8;0x110];manager[0xe0..0xe4].copy_from_slice(&[8,0,8,0]);manager[0xf0..0xf6].copy_from_slice(&[1,0,8,0,8,0]);let mut ships=vec![0u8;8*0x180];let mut caps=vec![255u8;8*16];for i in 0..8u16{caps[usize::from(i)*16..usize::from(i)*16+2].copy_from_slice(&(i+1).to_le_bytes())}let mut cvs=vec![];let mut merchants=vec![0u8;0x650];let mut towns=vec![0u8;16*0x9f8];let doy=220u16;let population=3000u32;towns[usize::from(doy&15)*0x9f8+0x2e0..usize::from(doy&15)*0x9f8+0x2e4].copy_from_slice(&population.to_le_bytes());
  let mut header=[0u8;16];header[4..6].fill(255);header[6]=2;header[8..12].fill(255);let band=p3_save::pirate_bands::PirateBand{header,plan:None,pieces:vec![]};let mut rawband=vec![0u8;0x18];rawband[8..24].copy_from_slice(&header);let hex=|data:&[u8]|data.iter().map(|b|format!("{b:02x}")).collect::<String>();
  let tick=125740230u32;let cap=CaptainResources{tick,day_of_year:doy,day:8,population,first_names:75,last_names:80};let names=vec![b"Alpha".to_vec(),b"Brig".to_vec(),b"Crown".to_vec()];let town0=p3_core::town::Town::from_bytes(&towns[..0x9f8]);
  let case=serde_json::json!({"band":hex(&rawband),"ships":hex(&ships),"captains":hex(&caps),"towns":hex(&towns),"ship_count":8,"cap_count":8,"ship_head":8,"cap_head":8,"cursor":0,"step":1,"doy":doy,"day":cap.day,"tick":tick,"population":population,"first":75,"last":80,"names":["Alpha","Brig","Crown"],"port":[100,500],"band_port":[700,900],"band_city":2,"kind":2,"a1":0,"power":4000});
  let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_pirate_group_native.py");let mut child=std::process::Command::new(py).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"binary":exe,"cases":[case]}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let original:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();let x=&original[0];
  let mut remove=RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut caps,convoys:&mut cvs,merchants:&mut merchants,towns:&mut towns,pool_limit:0,merchant_count:1,town_count:16,ui_a:false,ui_b:false};let w=TickWorld{remove:&mut remove,time:tick,player:0};let state=AbsorptionState{bands:vec![Some(band),None,None,None,None],day:doy,network_client:0,operations:vec![]};let env=GroupEnv{calendar_byte:doy as u8,suppress_events:0,operation_host:0,level_thresholds:[0;256],extra_memory:vec![]};let host=NativeHost{captain_population_available:true,player:0,pool_limit:0,cleared_routes:vec![],remove_events:vec![],removed_plans:vec![],merchant_owners:vec![],captain:cap,operations:vec![],create:Some(CreateInputs{town0,port0:(100,500),ports:vec![(0,0),(0,0),(700,900)],weapon_experience:0,ship_names:names})};
  let out=pirate_group_runtime::prepare(&w,&state,0,&env,&host).unwrap();let id=u16::from_le_bytes(out.state.bands[0].as_ref().unwrap().header[10..12].try_into().unwrap());assert_eq!(id,8);assert_eq!(hex(&out.state.bands[0].as_ref().unwrap().header),&x["band"].as_str().unwrap()[16..48]);for (offset,key) in [(0xf4,"ship_count"),(0xf2,"cap_count"),(0xee,"cursor")]{assert_eq!(u16::from_le_bytes(out.manager[offset..offset+2].try_into().unwrap()),x[key].as_u64().unwrap() as u16,"{key}");}assert_eq!(hex(&out.ships[8*0x180..9*0x180]),x["ship"]);assert_eq!(hex(&out.captains),x["captains"]);assert_eq!(u16::from_le_bytes(out.manager[0xe2..0xe4].try_into().unwrap()),x["ship_head"].as_u64().unwrap()as u16);assert_eq!(u16::from_le_bytes(out.manager[0xe0..0xe2].try_into().unwrap()),x["cap_head"].as_u64().unwrap()as u16);assert!(out.host.operations.is_empty());
 }
 #[test] fn doomed_reserve_removal_and_dissolution_match_original(){
  use std::io::Write;let (Ok(exe),Ok(py))=(std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY"))else{return};let hex=|data:&[u8]|data.iter().map(|b|format!("{b:02x}")).collect::<String>();
  let mut manager=vec![0u8;0x110];for (at,v)in [(0xe0,0xffffu16),(0xe2,0xffff),(0xe6,0xffff),(0xe8,0),(0xea,0xffff),(0xf2,4),(0xf4,1),(0xf6,0)]{manager[at..at+2].copy_from_slice(&v.to_le_bytes())}let mut ships=vec![0;0x180];ships[0]=255;for at in [4,6,8,0xa,0xc,0x42,0x132]{ships[at..at+2].fill(255)}ships[0x134..0x136].copy_from_slice(&0x12u16.to_le_bytes());let mut caps=vec![255;64];let mut cvs=vec![255;0x3c];let mut towns=vec![255;0x9f8*8];let mut merchants=vec![255;0x650*2];let mut header=[0u8;16];header[6]=3;header[8..10].fill(255);header[13]=1;let band=p3_save::pirate_bands::PirateBand{header,plan:None,pieces:vec![]};let mut rawband=vec![0u8;24];rawband[8..].copy_from_slice(&header);
  let case=serde_json::json!({"band":hex(&rawband),"ships":hex(&ships),"captains":hex(&caps),"convoys":hex(&cvs),"towns":hex(&towns),"merchants":hex(&merchants),"manager":hex(&manager),"id":0,"merchant_count":2,"town_count":8});let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_pirate_group_remove_chain.py");let mut p=std::process::Command::new(py).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();p.stdin.take().unwrap().write_all(serde_json::json!({"binary":exe,"cases":[case]}).to_string().as_bytes()).unwrap();let o=p.wait_with_output().unwrap();assert!(o.status.success(),"{}",String::from_utf8_lossy(&o.stderr));let x:serde_json::Value=serde_json::from_slice(&o.stdout).unwrap();let x=&x[0];
  let mut remove=RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut caps,convoys:&mut cvs,merchants:&mut merchants,towns:&mut towns,pool_limit:0,merchant_count:2,town_count:8,ui_a:false,ui_b:false};let w=TickWorld{remove:&mut remove,time:0,player:0};let state=AbsorptionState{bands:vec![Some(band),None,None,None,None],day:1,network_client:0,operations:vec![]};let env=GroupEnv{calendar_byte:1,suppress_events:0,operation_host:0,level_thresholds:[0;256],extra_memory:vec![]};let host=NativeHost{captain_population_available:true,player:0,pool_limit:0,cleared_routes:vec![],remove_events:vec![],removed_plans:vec![],merchant_owners:vec![],create:None,captain:CaptainResources{tick:0,day_of_year:1,day:1,population:0,first_names:1,last_names:1},operations:vec![]};let out=pirate_group_runtime::prepare(&w,&state,0,&env,&host).unwrap();assert!(out.state.bands[0].is_none());assert_eq!(hex(&out.ships),x["ships"]);assert_eq!(hex(&out.captains),x["captains"]);assert_eq!(hex(&out.merchants),x["merchants"]);assert_eq!(hex(&out.towns),x["towns"]);let original_manager=x["manager"].as_str().unwrap();assert_eq!(hex(&out.manager[0x20..]),&original_manager[0x40..]);assert!(out.host.cleared_routes.is_empty());assert!(out.host.remove_events.is_empty());
 }
 #[test] fn repair_enqueues_b5_like_original(){
  use std::io::Write;let (Ok(exe),Ok(py))=(std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY"))else{return};let hex=|data:&[u8]|data.iter().map(|b|format!("{b:02x}")).collect::<String>();
  let mut manager=vec![0u8;0x110];manager[0xf2..0xf6].copy_from_slice(&[1,0,1,0]);let mut ships=vec![0u8;0x180];ships[4..8].fill(255);ships[0x14..0x18].copy_from_slice(&10000u32.to_le_bytes());ships[0x18..0x1c].copy_from_slice(&9500u32.to_le_bytes());let mut caps=vec![255;16];let mut cvs=vec![];let mut merchants=vec![0;0x650];let mut towns=vec![];
  let mut header=[0u8;16];header[..2].copy_from_slice(&1u16.to_le_bytes());header[8..10].fill(255);let mut pieces=vec![0u8;8];pieces[4..6].fill(255);pieces[6]=8;let band=p3_save::pirate_bands::PirateBand{header,plan:None,pieces:pieces.clone()};let mut rawband=vec![0u8;0x20];rawband[8..24].copy_from_slice(&header);
  let case=serde_json::json!({"band":hex(&rawband),"ships":hex(&ships),"pieces":hex(&pieces),"convoys":hex(&vec![0u8;0x3c]),"index":0,"globals":[[0x6dd892u32,1,2],[0x6dd894u32,1,2],[0x6dd896u32,0,2],[0x6de4aau32,1,2],[0x6de4a4u32,1,1],[0x6de52au32,0,1],[0x6df31cu32,0,4]]});let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_pirate_group_tick.py");let mut child=std::process::Command::new(py).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"binary":exe,"cases":[case]}).to_string().as_bytes()).unwrap();let o=child.wait_with_output().unwrap();assert!(o.status.success(),"{}",String::from_utf8_lossy(&o.stderr));let x:serde_json::Value=serde_json::from_slice(&o.stdout).unwrap();let x=&x[0];
  let mut remove=RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut caps,convoys:&mut cvs,merchants:&mut merchants,towns:&mut towns,pool_limit:0,merchant_count:1,town_count:0,ui_a:false,ui_b:false};let w=TickWorld{remove:&mut remove,time:0,player:0};let state=AbsorptionState{bands:vec![Some(band),None,None,None,None],day:1,network_client:0,operations:vec![]};let env=GroupEnv{calendar_byte:1,suppress_events:0,operation_host:0,level_thresholds:[0;256],extra_memory:vec![]};let host=NativeHost{captain_population_available:true,player:0,pool_limit:0,cleared_routes:vec![],remove_events:vec![],removed_plans:vec![],merchant_owners:vec![],create:None,captain:CaptainResources{tick:0,day_of_year:1,day:1,population:0,first_names:1,last_names:1},operations:vec![]};let out=pirate_group_runtime::prepare(&w,&state,0,&env,&host).unwrap();assert_eq!(hex(&out.ships),x["ships"]);assert_eq!(hex(&out.state.bands[0].as_ref().unwrap().pieces),x["pieces"]);let ops=out.host.operations.iter().map(|o|serde_json::json!(["enqueue",o.0[0],o.0[1],o.0[2],o.0[3],o.0[4]])).collect::<Vec<_>>();assert_eq!(serde_json::json!(ops),x["events"]);assert_eq!(out.host.operations[0].0,[0xb5,0,120,1,0]);
 }
 #[test] fn reserve_repair_and_real_captain_growth_are_transactional(){
  let mut manager=vec![0u8;0x110];manager[0xf4..0xf6].copy_from_slice(&1u16.to_le_bytes());let before=manager.clone();let mut ships=vec![0u8;0x180];ships[4..8].fill(255);ships[0x42..0x44].fill(255);ships[0x14..0x18].copy_from_slice(&10000u32.to_le_bytes());ships[0x18..0x1c].copy_from_slice(&9500u32.to_le_bytes());let mut caps=vec![];let mut cvs=vec![];let mut merchants=vec![];let mut towns=vec![];
  let mut remove=RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut caps,convoys:&mut cvs,merchants:&mut merchants,towns:&mut towns,pool_limit:0,merchant_count:1,town_count:0,ui_a:false,ui_b:false};let w=TickWorld{remove:&mut remove,time:12345,player:0};
  let mut header=[0u8;16];header[8..10].fill(255);header[10..12].copy_from_slice(&0u16.to_le_bytes());let band=p3_save::pirate_bands::PirateBand{header,plan:None,pieces:vec![]};let state=AbsorptionState{bands:vec![Some(band),None,None,None,None],day:1,network_client:0,operations:vec![]};
  let env=GroupEnv{calendar_byte:1,suppress_events:0,operation_host:1,level_thresholds:[0;256],extra_memory:vec![]};let host=NativeHost{captain_population_available:true,player:0,pool_limit:0,cleared_routes:vec![],remove_events:vec![],removed_plans:vec![],merchant_owners:vec![],create:None,captain:CaptainResources{tick:12345,day_of_year:220,day:8,population:3000,first_names:20,last_names:20},operations:vec![]};
  let prepared=pirate_group_runtime::prepare(&w,&state,0,&env,&host).unwrap();assert_eq!(w.remove.manager,&before);assert_eq!(u32::from_le_bytes(prepared.ships[0x18..0x1c].try_into().unwrap()),10000);assert_eq!(&prepared.ships[0x42..0x44],&0u16.to_le_bytes());assert_eq!(prepared.captains.len(),64*16);assert_eq!(&prepared.manager[0xf2..0xf4],&64u16.to_le_bytes());assert_eq!(&prepared.manager[0xe0..0xe2],&1u16.to_le_bytes());assert!(prepared.host.operations.is_empty());
 }
}
