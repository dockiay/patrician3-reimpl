//! Prepara la fase diaria 514000 antes de la lista de barcos atracados.
use crate::{convoy_absorption::AbsorptionState,navigation::LiveVoyage,pirate_group_native_host::{CreateInputs,NativeHost},pirate_group_runtime::{self,GroupEnv},route_stop_runtime::RouteStopRuntime,HanseFleetResources};
use p3_core::{ai::{reputation::Merchant,settlement::SettlementLocation},economy::pirate_group_callbacks::CaptainResources,ships::{remove::RemoveWorld,tick::TickWorld},town::Town};
use p3_save::world::MerchantShipGroups;

pub struct Inputs<'a>{pub tick:u32,pub day:u8,pub day_of_year:u16,pub player:u32,pub alderman_town:u8,pub world_lock:u8,pub network:u32,pub thresholds:Option<[u8;256]>,pub names:Option<(u16,u16)>,pub resources:Option<&'a HanseFleetResources>,pub locations:Option<&'a [SettlementLocation;40]>}
pub struct Staged{pub nav:LiveVoyage,pub bands:AbsorptionState,pub towns:Vec<Town>,pub merchants:Vec<Merchant>,pub pool:RouteStopRuntime,pub groups:Vec<MerchantShipGroups>,pub changed_groups:Vec<usize>}
/// No modifica sus entradas. Las operaciones y el orden de bandas se conservan.
pub fn run(nav:&LiveVoyage,bands:&AbsorptionState,towns:&[Town],merchants:&[Merchant],pool:&RouteStopRuntime,groups:&[MerchantShipGroups],input:Inputs<'_>)->Result<Staged,String>{
 let mut out=Staged{nav:nav.clone(),bands:bands.clone(),towns:towns.to_vec(),merchants:merchants.to_vec(),pool:pool.clone(),groups:groups.to_vec(),changed_groups:Vec::new()};
 if out.bands.bands.len()!=5{return Err("cuadrillas incompletas".into())}
 for i in 0..5{
  let Some(band)=out.bands.bands[i].as_ref()else{continue};
  let count=u16::from_le_bytes([out.nav.snapshot.manager[0xf6],out.nav.snapshot.manager[0xf7]]);
  let convoy=u16::from_le_bytes([band.header[8],band.header[9]]);
  if input.thresholds.is_none()&&band.header[13]!=1&&convoy>=count{return Err("tabla original de cuadrillas ausente: suministra el EXE inglés".into())}
  let town0=out.towns.first().cloned();let weapons=out.towns.get(usize::from(input.alderman_town)).map(|town|town.u32(0x810));
  let create=match (town0,weapons,input.resources,input.locations){(Some(town0),Some(weapon_experience),Some(r),Some(locations))=>r.ports.first().copied().flatten().map(|port0|CreateInputs{weapon_experience,town0,port0,ports:locations.iter().map(|p|(p.x as u16,p.y as u16)).collect(),ship_names:r.ship_names.clone()}),_=>None};
  let (first_names,last_names)=input.names.unwrap_or((0,0));let population=out.towns.get(usize::from(input.day_of_year&15)).map(|t|t.u32(0x2e0));
  let captain=CaptainResources{tick:input.tick,day_of_year:input.day_of_year,day:input.day,population:population.unwrap_or(0),first_names,last_names};
  let parsed=p3_save::tail::RouteStopPool::parse(out.pool.bytes()).map_err(|e|format!("depósito de rutas: {e:?}"))?;
  let host=NativeHost{captain_population_available:population.is_some(),captain,create,player:input.player,pool_limit:parsed.count,cleared_routes:vec![],remove_events:vec![],removed_plans:vec![],merchant_owners:vec![],operations:vec![]};
  let env=GroupEnv{calendar_byte:input.day_of_year as u8,suppress_events:input.world_lock,operation_host:input.network,level_thresholds:input.thresholds.unwrap_or([0;256]),extra_memory:vec![]};
  let mut tb=out.towns.iter().flat_map(|t|t.0.iter().copied()).collect::<Vec<_>>();let mut mb=out.merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();
  let snapshot=&mut out.nav.snapshot;
  let mut remove=RemoveWorld{manager:&mut snapshot.manager,ships:&mut snapshot.ships,captains:&mut snapshot.captains,convoys:&mut snapshot.convoys,towns:&mut tb,merchants:&mut mb,pool_limit:parsed.count,merchant_count:out.merchants.len() as u16,town_count:out.towns.len() as u32,ui_a:false,ui_b:false};
  let world=TickWorld{remove:&mut remove,time:input.tick,player:input.player};
  let prepared=pirate_group_runtime::prepare(&world,&out.bands,i,&env,&host)?;
  let effects=crate::pirate_group_effects::prepare(&prepared,out.pool.bytes(),&out.nav.snapshot.routes,&out.groups)?;
  let old_pool=out.pool.bytes().to_vec();out.pool.replace(&old_pool,effects.route_stop_pool)?;
  out.nav.snapshot.manager=prepared.manager;out.nav.snapshot.ships=prepared.ships;out.nav.snapshot.captains=prepared.captains;out.nav.snapshot.convoys=prepared.convoys;out.nav.snapshot.routes=effects.routes;
  for &id in &prepared.host.cleared_routes{out.nav.forget_captured_route(id)}
  crate::navigation::refresh_lists(&mut out.nav.snapshot)?;
  for (t,src) in out.towns.iter_mut().zip(prepared.towns.chunks_exact(0x9f8)){t.0.copy_from_slice(src)}
  for (m,src) in out.merchants.iter_mut().zip(effects.merchants.chunks_exact(0x650)){m.bytes.copy_from_slice(src)}
  out.groups=effects.merchant_groups;for id in effects.changed_groups{if !out.changed_groups.contains(&id){out.changed_groups.push(id)}}
  out.bands=prepared.state;out.bands.operations.extend(effects.operations);
 }
 Ok(out)
}
