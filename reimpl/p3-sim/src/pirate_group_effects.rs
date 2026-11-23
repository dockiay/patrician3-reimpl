//! Preparación atómica de efectos externos de bajas 514000.
use crate::{pirate_group_native_host::NativeHost,pirate_group_runtime::Prepared};
use p3_core::{ai::reputation::Merchant,economy::pirate_group_callbacks::free_route_stop_pool,ships::remove::RemoveEvent};
use p3_save::{ships::SavedRoute,world::MerchantShipGroups};

#[derive(Clone,Debug)]
pub struct PreparedEffects {
 pub route_stop_pool:Vec<u8>,pub routes:Vec<Option<SavedRoute>>,
 pub merchants:Vec<u8>,pub merchant_groups:Vec<MerchantShipGroups>,pub changed_groups:Vec<usize>,
 pub removed_plans:Vec<usize>,pub operations:Vec<p3_core::pump::Operation>,
}
/// Aplica efectos en el orden del original sobre copias. Los grupos, paradas,
/// comerciantes, rutas y bandas deben publicarse con la flota preparada.
pub fn prepare(prepared:&Prepared<NativeHost>,pool:&[u8],routes:&[Option<SavedRoute>],groups:&[MerchantShipGroups])->Result<PreparedEffects,String>{
 let mut result=PreparedEffects{route_stop_pool:pool.to_vec(),routes:routes.to_vec(),merchants:prepared.merchants.clone(),merchant_groups:groups.to_vec(),changed_groups:Vec::new(),removed_plans:prepared.host.removed_plans.clone(),operations:prepared.host.operations.clone()};
 let n=prepared.ships.len()/0x180;if prepared.ships.len()%0x180!=0||result.routes.len()>n{return Err("flota o rutas de efectos truncadas".into())}result.routes.resize(n,None);
 // None representa un slot libre en S02; un barco recién creado en una
 // ranura antigua debe convertirse en Some(None), no desaparecer al guardar.
 for (i,image) in prepared.ships.chunks_exact(0x180).enumerate(){let state=u16::from_le_bytes([image[0x134],image[0x135]]);if state>=0x17{result.routes[i]=None}else if result.routes[i].is_none(){if image[0x24..0x28]!=[0;4]{return Err("barco activado con ruta no reconstruible".into())}result.routes[i]=Some(SavedRoute::None)}}
 for &id in &prepared.host.cleared_routes{let at=usize::from(id);if at>=n{return Err("ruta liberada fuera de flota".into())}let image=&prepared.ships[at*0x180..(at+1)*0x180];let state=u16::from_le_bytes([image[0x134],image[0x135]]);result.routes[at]=if state>=0x17{None}else{Some(SavedRoute::None)}}
 for &i in &result.removed_plans {if i>=prepared.state.bands.len()||prepared.state.bands[i].as_ref().is_some_and(|b|b.plan.is_some()){return Err("plano eliminado aún presente".into())}}
 let mut snapshots=prepared.host.merchant_owners.iter();
 for event in &prepared.host.remove_events{match *event{
  RemoveEvent::PoolRelease(id)=>{
   let parsed=p3_save::tail::RouteStopPool::parse(&result.route_stop_pool).map_err(|e|format!("depósito de rutas: {e:?}"))?;
   let mut head=parsed.field_00;free_route_stop_pool(&mut head,&mut result.route_stop_pool[parsed.stops],id)?;result.route_stop_pool[..2].copy_from_slice(&head.to_le_bytes());
  },
  RemoveEvent::MerchantChanged(id)=>{
   let owner=usize::try_from(id).map_err(|_|"comerciante inválido")?;let at=owner.checked_mul(0x650).ok_or("comerciante excesivo")?;let bytes=result.merchants.get(at..at+0x650).ok_or("comerciante ausente")?.to_vec();let group=result.merchant_groups.get_mut(owner).ok_or("grupos de comerciante ausentes")?;
   let (captured_owner,owners)=snapshots.next().ok_or("falta captura temporal de dueños")?;if *captured_owner!=id{return Err("captura de dueño fuera de orden".into())}
   let mut merchant=Merchant{bytes};for (slot,list)in group.iter().enumerate(){let count=usize::from(merchant.u16(0x60c+2*slot)&0x7fff);if count>list.len()||list[..count].iter().any(|&id|usize::from(id)>=owners.len()){return Err("grupo naval inválido".into())}}
   merchant.prune_foreign_ships_from_groups(owner as u8,group,&|id|owners[usize::from(id)]);result.merchants[at..at+0x650].copy_from_slice(&merchant.bytes);if !result.changed_groups.contains(&owner){result.changed_groups.push(owner)}
  },
  _=>return Err(format!("efecto de baja sin adaptador: {event:?}")),
 }}
 if snapshots.next().is_some(){return Err("captura temporal sin evento".into())}Ok(result)
}

#[cfg(test)] mod tests {
 use super::*;use crate::{convoy_absorption::AbsorptionState,pirate_group_native_host::NativeHost};use p3_core::{economy::pirate_group_callbacks::CaptainResources,pump::Operation};
 fn fixture()->(Prepared<NativeHost>,Vec<u8>,Vec<Option<SavedRoute>>,Vec<MerchantShipGroups>){
  let mut ships=vec![0u8;2*0x180];ships[0x134..0x136].copy_from_slice(&0xffu16.to_le_bytes());ships[0]=255;ships[0x180]=255;let mut merchants=vec![0u8;0x650];merchants[0x60c..0x60e].copy_from_slice(&2u16.to_le_bytes());
  let host=NativeHost{captain_population_available:true,player:0,pool_limit:2,cleared_routes:vec![0],remove_events:vec![RemoveEvent::PoolRelease(0),RemoveEvent::MerchantChanged(0)],removed_plans:vec![0],merchant_owners:vec![(0,vec![255,0])],captain:CaptainResources{tick:0,day_of_year:1,day:1,population:0,first_names:1,last_names:1},operations:vec![Operation([0xb5,0,120,1,0])],create:None};
  let prepared=Prepared{manager:vec![0;0x110],ships,captains:vec![],convoys:vec![],merchants,towns:vec![],state:AbsorptionState{bands:vec![None,None,None,None,None],day:1,network_client:0,operations:vec![]},host};
  let mut pool=vec![0u8;4+2*220];pool[..4].copy_from_slice(&[2,0,2,0]);pool[4..6].copy_from_slice(&1u16.to_le_bytes());pool[4+220..6+220].copy_from_slice(&0u16.to_le_bytes());pool[7]=7;
  let routes=vec![Some(SavedRoute::Owned(vec![(1,2),(3,4)])),Some(SavedRoute::None)];let mut group:MerchantShipGroups=std::array::from_fn(|_|vec![]);group[0]=vec![0,1];(prepared,pool,routes,vec![group])
 }
 #[test] fn publishes_pool_routes_groups_and_snapshot_time_together(){
  let (prepared,pool,routes,groups)=fixture();let r=prepare(&prepared,&pool,&routes,&groups).unwrap();assert_eq!(&r.route_stop_pool[..2],&0u16.to_le_bytes());assert_eq!(&r.route_stop_pool[4+220..6+220],&2u16.to_le_bytes());assert_eq!(r.route_stop_pool[7],0);assert_eq!(r.routes[0],None);assert_eq!(r.merchant_groups[0][0][0],1);assert_eq!(&r.merchants[0x60c..0x60e],&1u16.to_le_bytes());assert_eq!(r.changed_groups,vec![0]);assert_eq!(r.removed_plans,vec![0]);assert_eq!(r.operations[0].0,[0xb5,0,120,1,0]);assert_eq!(&pool[..2],&2u16.to_le_bytes());assert_eq!(groups[0][0],vec![0,1]);
 }
 #[test] fn two_ordered_merchant_updates_match_real_x86(){
  use std::io::Write;use p3_core::flatmem::{Mem,SparseMem};let (Ok(exe),Ok(py))=(std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY"))else{return};
  const M:u32=0x6dd7a0;const S:u32=0x55000000;const C:u32=0x56000000;const V:u32=0x57000000;const T:u32=0x58000000;const R:u32=0x59000000;
  let mut manager=vec![0u8;0x110];for(at,v)in[(0xe0,0xffffu16),(0xe2,0xffff),(0xe4,0xffff),(0xe6,0xffff),(0xe8,0),(0xea,0xffff),(0xf2,4),(0xf4,2),(0xf6,0)]{manager[at..at+2].copy_from_slice(&v.to_le_bytes())}let mut ships=vec![0u8;2*0x180];for i in 0..2usize{let at=i*0x180;for off in [4,6,8,0xa,0xc,0x42,0x132]{ships[at+off..at+off+2].fill(255)}if i==0{ships[at+4..at+6].copy_from_slice(&1u16.to_le_bytes());ships[at+6..at+8].copy_from_slice(&1u16.to_le_bytes())}ships[at+0x134..at+0x136].copy_from_slice(&0x12u16.to_le_bytes())}ships[0x132..0x134].copy_from_slice(&0u16.to_le_bytes());let mut pool=vec![0u8;4+440];pool[..4].copy_from_slice(&[2,0,2,0]);pool[4..6].copy_from_slice(&1u16.to_le_bytes());pool[224..226].copy_from_slice(&0u16.to_le_bytes());pool[7]=9;let caps=vec![255;64];let convoys=vec![255;0x3c];let towns=vec![255;8*0x9f8];let mut merchants=vec![0;2*0x650];merchants[0xe..0x10].copy_from_slice(&0u16.to_le_bytes());merchants[0x60c..0x60e].copy_from_slice(&2u16.to_le_bytes());let hex=|data:&[u8]|data.iter().map(|b|format!("{b:02x}")).collect::<String>();
  let input=serde_json::json!({"binary":exe,"manager":hex(&manager),"ships":hex(&ships),"captains":hex(&caps),"convoys":hex(&convoys),"towns":hex(&towns),"merchants":hex(&merchants),"pool":hex(&pool[4..])});let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_pirate_events_order.py");let mut child=std::process::Command::new(py).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let x:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();
  let mut m=SparseMem::default();m.seed(M,&manager);m.seed(S,&ships);m.seed(C,&caps);m.seed(V,&convoys);m.seed(T,&towns);m.seed(R,&merchants);m.seed(0x6de4a0,&vec![0;0x100]);m.w32(M,C);m.w32(M+4,S);m.w32(M+8,V);m.w32(0x6de508,T);m.w32(0x6de518,R);m.w32(0x6de4b0,8);m.w16(0x6de4aa,2);
  let a=crate::pirate_group_remove::remove(&mut m,0,0,2).unwrap();let b=crate::pirate_group_remove::remove(&mut m,1,0,2).unwrap();let mut snapshots=a.merchant_owners.clone();snapshots.extend(b.merchant_owners.clone());assert_eq!(serde_json::json!(snapshots.iter().map(|(_,v)|v.clone()).collect::<Vec<_>>()),x["owners"]);
  let mut host=fixture().0.host;host.remove_events=a.events.into_iter().chain(b.events).collect();host.merchant_owners=snapshots;host.removed_plans.clear();host.cleared_routes.clear();host.operations.clear();let prepared=Prepared{manager:m.read(M,0x110),ships:m.read(S,0x300),captains:m.read(C,64),convoys:m.read(V,0),merchants:m.read(R,2*0x650),towns:m.read(T,8*0x9f8),state:AbsorptionState{bands:vec![None;5],day:1,network_client:0,operations:vec![]},host};let mut group:MerchantShipGroups=std::array::from_fn(|_|vec![]);group[0]=vec![0,1];let out=prepare(&prepared,&pool,&[None,None],&[group]).unwrap();assert_eq!(hex(&out.route_stop_pool),x["pool"]);assert_eq!(hex(&prepared.ships),x["ships"]);assert_eq!(hex(&out.merchants[0x60c..0x622]),x["merchant_counts"]);let mut groupbytes=vec![];for &v in &out.merchant_groups[0][0]{groupbytes.extend_from_slice(&v.to_le_bytes())}assert_eq!(hex(&groupbytes),x["group"]);
 }
 #[test] fn activated_free_slot_has_a_serializable_empty_route(){
  let (mut prepared,pool,mut routes,groups)=fixture();prepared.host.remove_events.clear();prepared.host.merchant_owners.clear();prepared.host.cleared_routes.clear();prepared.host.removed_plans.clear();prepared.ships[0x134..0x136].copy_from_slice(&0x12u16.to_le_bytes());routes[0]=None;let out=prepare(&prepared,&pool,&routes,&groups).unwrap();assert_eq!(out.routes[0],Some(SavedRoute::None));assert_eq!(routes[0],None);
  prepared.ships[0x24]=1;assert!(prepare(&prepared,&pool,&routes,&groups).unwrap_err().contains("ruta no reconstruible"));
 }
 #[test] fn corrupt_pool_or_missing_snapshot_reject_without_mutation(){
  let (mut prepared,mut pool,routes,groups)=fixture();pool[4..6].copy_from_slice(&3u16.to_le_bytes());let original=pool.clone();assert!(prepare(&prepared,&pool,&routes,&groups).is_err());assert_eq!(pool,original);assert_eq!(groups[0][0],vec![0,1]);prepared.host.remove_events.remove(0);prepared.host.merchant_owners.clear();assert!(prepare(&prepared,&pool,&routes,&groups).unwrap_err().contains("captura temporal"));
 }
}
