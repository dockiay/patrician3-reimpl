//! Pasos navales transaccionales para los viajes humanos seleccionados (C07).
//! También avanza barcos humanos inactivos en puerto mientras hay viajes.
//! El resto de la flota conserva sus listas y no se procesa en este modo.
use p3_core::ships::{advance::{advance_along_route,AdvanceHost,AdvanceWorld},docking::{DockHost,set_sail_from_tick,finish_docking_from_tick},order::{remove_from_tick,resort_from_tick},remove::{RemoveWorld,SHIP_SIZE,TOWN_SIZE,MERCHANT_SIZE},route::{self,RouteHost},tick::{tick_docked,tick_sea,TickCall,TickHost,TickWorld,SeaCall,SeaHost}};
use p3_core::{town::Town,ai::reputation::Merchant};
use p3_save::ships::NavigationSnapshot;

#[derive(Clone,Debug)]
pub struct ExtraVoyage {pub ship:u16,pub convoy:Option<u16>,pub points:Vec<(i16,i16)>}
/// Aviso `0x00535400(ciudad, dueño)` del tick naval (barco humano atracado o convoy que llega):
/// el anfitrión lo aplica al historial de información y lo retira de `events`.
pub const KNOWN_TOWN:&str="conocer:";
/// Barco que sale de un convoy en `Convoy::advance`: `tarea:convoy:dueño:texto` (0x00502285).
pub const CONVOY_LOSS:&str="baja-convoy:";
/// Carta 0x1B de 0x00519D90 (barco humano perdido en el mar): `dueño:16 bytes en hexadecimal`.
pub const SHIP_LOST_LETTER:&str="barco-perdido:";
/// Rutas y puntos de `LiveVoyage` anteriores a un tick vivo que los cambia (`step_with_ai_bands_and_expeditions_live`).
#[derive(Clone,Debug)]
pub struct VoyageUndo {pub routes:Vec<Option<p3_save::ships::SavedRoute>>,pub points:Vec<(i16,i16)>,pub additional:Vec<ExtraVoyage>,pub convoy:Option<u16>}
#[derive(Clone,Debug)]
pub struct LiveVoyage {pub additional:Vec<ExtraVoyage>,pub baseline:NavigationSnapshot,pub snapshot:NavigationSnapshot,pub ship:u16,pub convoy:Option<u16>,pub points:Vec<(i16,i16)>,pub ports:Vec<(i16,i16)>,pub player:u32,pub events:Vec<String>}
/// Parámetros externos del encuentro comercial admitido por el tick naval.
#[derive(Clone,Copy,Debug)]
pub struct EncounterOptions<'a>{pub slot:u8,pub battle_address:u32,pub scene_busy:bool,pub dialog_text:&'a [u8]}
/// Ranura y clasificación aportadas por el anfitrión para un encuentro pirata.
#[derive(Clone,Copy,Debug)]
pub struct PirateEncounterOptions<'a>{pub slot:u8,pub battle_address:u32,pub side0_pirate_band:bool,pub scheduler:Option<&'a p3_core::scheduler::Scheduler>,pub tile_width:Option<i16>,
 /// Política sin pantalla: el convoy de un humano atacado por piratas se resuelve en batalla automática (no paridad).
 pub human_as_ai:bool}
/// Contexto del reloj y recursos externos para 503B50; se confirma con el tick.
#[derive(Clone)]
pub struct ExpeditionPlanning {
 pub lock:u8,pub countdown:u8,pub network_client:bool,
 pub trade_centers:Vec<Vec<u8>>,pub locations:Vec<(i16,i16)>,pub map_points:Vec<(i16,i16)>,
 pub rng:Option<p3_core::rng::CrtRand>,
 pub scheduler:p3_core::scheduler::Scheduler,
}
struct Host<'a>{expeditions:Option<&'a mut ExpeditionPlanning>,pending_expedition_task:Option<u16>,absorption:Option<&'a mut crate::convoy_absorption::AbsorptionState>,encounter:Option<EncounterOptions<'a>>,pirate:Option<PirateEncounterOptions<'a>>,battle:Option<p3_core::ships::battle::CommercialBattleStart>,ship:u16,convoy:Option<u16>,points:&'a [(i16,i16)],additional:&'a [ExtraVoyage],passive:Vec<u16>,current:u16,chasing:bool,planner:Option<&'a mut p3_core::route::finder::RoutePlanner>,route_updates:Vec<(u16,Vec<(i16,i16)>,p3_save::ships::SavedRoute)>,freed_routes:Vec<u16>,convoy_updates:Vec<(u16,u16)>,route_book:Vec<Option<p3_save::ships::SavedRoute>>,ports:&'a [(i16,i16)],tick:u32,events:Vec<String>,error:Option<String>,ai:Option<&'a mut p3_core::ships::ai_dock::AiDockData>}
/// Bytes de las ciudades seguidos (igual que recorrerlos de uno en uno, pero copiando por bloques).
fn town_image(towns:&[Town])->Vec<u8>{let mut out=Vec::with_capacity(towns.len()*TOWN_SIZE);for t in towns{out.extend_from_slice(&t.0[..])}out}
/// Bytes de los comerciantes seguidos.
fn merchant_image(merchants:&[Merchant])->Vec<u8>{let mut out=Vec::with_capacity(merchants.len()*MERCHANT_SIZE);for m in merchants{out.extend_from_slice(&m.bytes)}out}
fn active_convoy(snapshot:&NavigationSnapshot,id:u16,owner:u32)->bool{
 let count=u16::from_le_bytes([snapshot.manager[0xf6],snapshot.manager[0xf7]]);let mut current=u16::from_le_bytes([snapshot.manager[0xec],snapshot.manager[0xed]]);let mut seen=vec![false;usize::from(count)];let mut found=false;while current<count{let i=usize::from(current);if seen[i]{return false}seen[i]=true;let at=i*0x3c;if at+0x3c>snapshot.convoys.len(){return false}if current==id{found=u32::from(snapshot.convoys[at])==owner}current=u16::from_le_bytes([snapshot.convoys[at+8],snapshot.convoys[at+9]])}found
}
impl Host<'_>{
 fn divert(&mut self,w:&mut TickWorld,ship:u32,except:u32){
  let Some(ai)=self.ai.as_deref_mut() else{self.fail("matriz de distancias para el desvío ausente");return};
  let (dest,_)=p3_core::ships::ai_dock::nearest_port_plan(w,ai,ship as u16,0,except);
  let id=ship as u16;let at=usize::from(id)*SHIP_SIZE;
  let Some(&end)=self.ports.get(dest as usize) else{self.fail("puerto del desvío ausente");return};
  let origin=w.remove.ships[at+0x37];
  let image=&mut w.remove.ships[at..at+SHIP_SIZE];let start=(u16::from_le_bytes([image[0x1e],image[0x1f]]) as i16,u16::from_le_bytes([image[0x22],image[0x23]]) as i16);
  let Some(planner)=self.planner.as_deref_mut() else{self.fail("planificador del desvío ausente");return};
  let found=planner.find_route(&[start,end]);let (points,saved)=match found.route{
   p3_core::route::finder::RouteOut::Shared(n)=>{let Some(points)=planner.routes.get(n).cloned() else{self.fail("ruta compartida del desvío inválida");return};
    // La ruta compartida es la de la tabla ciudad→ciudad que devuelve el planificador desde la
    // posición actual; +0x37 puede ser aún la ciudad de un viaje anterior (ConvoyHecho, barco 88,
    // día 138: desvío desde 22 con +0x37 = 20, que el atraque deja después en 22).
    let tc=planner.town_count as usize;let origin=planner.town_table.iter().enumerate().find(|(p,v)|**v==Some(n)&&tc!=0&&p%tc==dest as usize).map(|(p,_)|(p/tc) as u8).unwrap_or(origin);
    (points,p3_save::ships::SavedRoute::Shared{origin,destination:dest as u8})},
   p3_core::route::finder::RouteOut::Built(points)=>{let saved=p3_save::ships::SavedRoute::Owned(points.clone());(points,saved)},
   p3_core::route::finder::RouteOut::Unset=>{self.fail("ruta del desvío no disponible");return}
  };
  if route::apply_route_plan(image,Some((dest as u8,end.0,end.1)),end,&points,found.status!=0,route_token(id),w.time).is_none(){self.fail("ruta del desvío inválida");return}
  self.route_updates.push((id,points,saved));
 }
 fn points_for(&self,id:u16)->&[(i16,i16)]{if let Some((_,points,_))=self.route_updates.iter().rev().find(|(ship,_,_)|*ship==id){return points}if id==self.ship{self.points}else{self.additional.iter().find(|v|v.ship==id).map(|v|v.points.as_slice()).unwrap_or(&[])}} fn fail(&mut self,s:&str){if self.error.is_none(){self.error=Some(s.into())}}}
impl Host<'_>{fn apply_pursuit(&mut self,w:&mut TickWorld,id:u16)->bool{
 const ZONES:[(i16,i16,i16,i16);3]=[(442,1521,658,1691),(2265,147,2332,453),(1497,1263,1578,1444)];
 match p3_core::ships::pursuit::update_pursuit(w.remove.manager,w.remove.ships,id,w.remove.town_count,w.time,&ZONES){
  Ok((result,None))=>result,
  Ok((result,Some(p3_core::ships::pursuit::PursuitEffect::ClearRoute(ship))))=>{let at=usize::from(ship)*SHIP_SIZE;route::clear_route(&mut w.remove.ships[at..at+SHIP_SIZE],ship,w.player,false,self);result},
  Ok((result,Some(p3_core::ships::pursuit::PursuitEffect::Battle(attacker,target))))=>{
   let Some(options)=self.encounter else{self.fail("encuentro marítimo pendiente de conectar");return false};
   if self.battle.is_some(){self.fail("varios encuentros en el mismo tick no admitidos");return false}
   let battle=if options.scene_busy {
    p3_core::ships::battle::start_two_commercial_battle_player_busy(w.remove.manager,w.remove.ships,w.remove.convoys,w.remove.merchants,w.remove.captains,attacker,target,options.slot,options.battle_address,w.player)
   }else{
    p3_core::ships::battle::start_two_commercial_battle_player_free(w.remove.manager,w.remove.ships,w.remove.convoys,w.remove.merchants,w.remove.captains,attacker,target,options.slot,options.battle_address,w.player,options.dialog_text)
   };
   match battle{Ok(battle)=>{self.battle=Some(battle);result},Err(e)=>{self.fail(e);false}}
  },
  Ok((result,Some(p3_core::ships::pursuit::PursuitEffect::Plan(ship,x,y))))=>{
   let at=usize::from(ship)*SHIP_SIZE;let image=&mut w.remove.ships[at..at+SHIP_SIZE];let start=(u16::from_le_bytes([image[0x1e],image[0x1f]]) as i16,u16::from_le_bytes([image[0x22],image[0x23]]) as i16);
   let Some(planner)=self.planner.as_deref_mut() else{self.fail("planificador de persecución ausente");return false};let path=planner.build_route_from_points(&[start,(x,y)]);let p3_core::route::finder::RouteOut::Built(points)=path.route else{self.fail("ruta de persecución no disponible");return false};
   if route::apply_route_plan(image,None,(x,y),&points,true,route_token(ship),w.time).is_none(){self.fail("ruta de persecución inválida");return false}self.route_updates.push((ship,points.clone(),p3_save::ships::SavedRoute::Owned(points)));result
  },
  Err(e)=>{self.fail(e);false}
 }
}}
impl DockHost for Host<'_>{
 fn plan_route(&mut self,_:u16){self.fail("planificación sin imagen")}
 fn plan_route_image(&mut self,id:u16,ship:&mut [u8]){
  let origin=usize::from(ship[0x39]);let Some(&(x,y))=self.ports.get(origin) else{self.fail("puerto de salida ausente");return};
  if ship[0x24..0x28]!=[0;4]{if route::start_route_from_port(ship,(x as u16,y as u16),None,self.tick).is_none(){self.fail("ruta de salida inválida")}return}
  let dest=usize::from(ship[0x38]);let Some(&end)=self.ports.get(dest) else{self.fail("destino de salida ausente");return};
  let Some(planner)=self.planner.as_deref_mut() else{self.fail("planificador de salida ausente");return};
  let found=planner.find_route(&[(x,y),end]);let (points,saved)=match found.route{
   p3_core::route::finder::RouteOut::Shared(n)=>{let Some(points)=planner.routes.get(n).cloned() else{self.fail("ruta compartida inválida");return};(points,p3_save::ships::SavedRoute::Shared{origin:origin as u8,destination:dest as u8})},
   p3_core::route::finder::RouteOut::Built(points)=>{let saved=p3_save::ships::SavedRoute::Owned(points.clone());(points,saved)},
   p3_core::route::finder::RouteOut::Unset=>{self.fail("ruta de salida no disponible");return}
  };
  if route::start_route_from_port(ship,(x as u16,y as u16),Some((&points,found.status!=0,route_token(id))),self.tick).is_none(){self.fail("ruta de salida inválida");return}self.route_updates.push((id,points,saved));
 }
 fn player_ship_sailed(&mut self,ship:u16){self.events.push(format!("salida:{ship}"))}
 fn notify(&mut self,args:[u32;6]){self.events.push(format!("aviso:{args:?}"))}
}
impl TickHost for Host<'_>{
 fn town_receive_with_world(&mut self,w:&mut TickWorld,town:u32,ship:u32){if let Err(e)=p3_core::ships::tick::append_repair_ship(w,town,ship as u16){self.fail(e)}}
 fn process_ship(&mut self,id:u32)->bool{id==u32::from(self.ship)||self.additional.iter().any(|v|u32::from(v.ship)==id)||self.passive.contains(&(id as u16))}
 fn call(&mut self,c:TickCall){match c{TickCall::Notify(town,owner)=>{self.events.push(format!("puerto:{c:?}"));self.events.push(format!("{KNOWN_TOWN}{town}:{owner}"))},TickCall::HarborShow(_) | TickCall::HarborLeave(_) | TickCall::HarborHide(_) | TickCall::Ui(_)=>self.events.push(format!("puerto:{c:?}")),_=>self.fail(&format!("llamada de puerto no conectada: {c:?}"))}}
 fn ai_state2(&mut self,_:u32)->bool{self.fail("salida de IA no conectada");false}
 fn ai_state2_with_world(&mut self,w:&mut TickWorld,id:u32,link:p3_core::ships::tick::Link)->bool{
  if self.ai.is_none(){return self.ai_state2(id)}
  match p3_core::ships::ai_depart::ai_depart(w,id as u16,link,self){Ok(left)=>{if left{self.events.push(format!("salida_ia:{id}"))}left},Err(e)=>{self.fail(&format!("salida de IA: {e:?}"));false}}
 }
 fn ai_docked_with_world(&mut self,w:&mut TickWorld,id:u32){
  if let Some(ai)=self.ai.as_deref_mut(){p3_core::ships::ai_dock::ai_docked_update(w,id as u16,ai);return}
  let at=id as usize*SHIP_SIZE;if !p3_core::ships::ai_docked::stop_flagged_ship(&mut w.remove.ships[at..at+SHIP_SIZE]).unwrap_or(false){self.fail("IA atracada pendiente de conectar")}}
 fn set_sail(&mut self,w:&mut TickWorld,id:u32){set_sail_from_tick(w,self,id as u16,0)}
 fn finish_docking(&mut self,w:&mut TickWorld,id:u32){finish_docking_from_tick(w,self,id as u16,0)}
}
impl p3_core::ships::ai_depart::AiDepartHost for Host<'_>{
 fn grow_convoys(&mut self,w:&mut TickWorld)->bool{p3_core::ships::ai_depart::grow_in_place(w)}
 fn clear_route(&mut self,w:&mut TickWorld,ship:u16){let at=usize::from(ship)*SHIP_SIZE;if at+SHIP_SIZE<=w.remove.ships.len(){route::clear_route(&mut w.remove.ships[at..at+SHIP_SIZE],ship,w.player,false,self)}}
}
impl Host<'_>{
 fn remove_at_sea(&mut self,w:&mut TickWorld,id:u32)->Result<(),String>{
  use p3_core::ships::remove::RemoveEvent;
  let r=|b:&[u8],o:usize|u16::from_le_bytes([b[o],b[o+1]]);
  let count=r(w.remove.manager,0xf4);if id>=u32::from(count){return Ok(())}
  let at=id as usize*SHIP_SIZE;let owner=w.remove.ships[at];
  let mut free=r(w.remove.manager,0xe2);let mut seen=vec![false;usize::from(count)];let mut already=false;
  while free<count{let i=usize::from(free);if seen[i]{return Err("lista libre naval cíclica".into())}seen[i]=true;if u32::from(free)==id{already=true;break}free=r(w.remove.ships,i*SHIP_SIZE+6)}
  let mut detached=Vec::new();
  if !already&&u16::from(owner)>=w.remove.merchant_count{
   // 0x507FB0 llama a 0x515F00 por cada cuadrilla presente (puntero +0xC del gestor). La sesión no
   // guarda esos punteros de proceso en el gestor: la cuadrilla viva de `absorption` es la presencia
   // (ConvoyPrueba, día 245: baja de 119 sin desenganchar 96 -> 119 de su cuadrilla).
   for k in 0..5usize{let pointer=u32::from_le_bytes(w.remove.manager[0xc+4*k..0x10+4*k].try_into().unwrap());
    let live=self.absorption.as_deref().is_some_and(|b|b.bands.get(k).is_some_and(Option::is_some));if pointer==0&&!live{continue}
    let bands=self.absorption.as_deref_mut().ok_or("bandas para baja marítima ausentes")?;let band=bands.bands.get_mut(k).and_then(Option::as_mut).ok_or("banda de baja marítima ausente")?;
    let mut cur=u16::from_le_bytes([band.header[4],band.header[5]]);let mut prev:Option<usize>=None;let mut guard=vec![false;usize::from(count)];
    while cur<count{let i=usize::from(cur);if guard[i]{return Err("cadena de banda cíclica".into())}guard[i]=true;let next=[w.remove.ships[i*SHIP_SIZE+4],w.remove.ships[i*SHIP_SIZE+5]];
     if u32::from(cur)==id{match prev{Some(p)=>w.remove.ships[p*SHIP_SIZE+4..p*SHIP_SIZE+6].copy_from_slice(&next),None=>band.header[4..6].copy_from_slice(&next)}w.remove.ships[i*SHIP_SIZE+4..i*SHIP_SIZE+6].fill(0xff);break}
     prev=Some(i);cur=u16::from_le_bytes(next)}
    if pointer!=0{detached.push(pointer)}}}
  for e in p3_core::ships::remove::remove_ship(w.remove,id){match e{
   RemoveEvent::UiA(_)|RemoveEvent::UiB(_)=>self.events.push(format!("mar:{e:?}")),
   RemoveEvent::ClearRoute(i)=>{let at=i as usize*SHIP_SIZE;route::clear_route(&mut w.remove.ships[at..at+SHIP_SIZE],i as u16,w.player,false,self)},
   RemoveEvent::OrderRemove(i)=>remove_from_tick(w,i as u16),
   RemoveEvent::ConvoyRelease(pointer,ship)=>{if ship!=id{return Err("desenlace de banda de otro barco".into())}let k=detached.iter().position(|&v|v==pointer).ok_or("desenlace de banda no aplicado")?;detached.remove(k);},
   RemoveEvent::ConvoyRefresh(c)=>p3_core::ships::convoy::refresh_from_ships(w,c).map_err(|e|format!("recuento de convoy: {e:?}"))?,
   other=>return Err(format!("efecto de baja marítima no conectado: {other:?}")),
  }}
  if !detached.is_empty(){return Err("desenlace de banda sin callback original".into())}Ok(())
 }
}
impl RouteHost for Host<'_>{fn points_of(&mut self,_:u32)->u32{1}fn free(&mut self,_:u32){}fn ui_route_cleared(&mut self,_:u16){}}
impl AdvanceHost for Host<'_>{
 fn route_points(&mut self,_:u32)->Vec<(i16,i16)>{self.points_for(self.current).to_vec()}
 fn points_pointer(&mut self,_:u32)->u32{1}fn free(&mut self,_:u32){}
 fn find_town_at_point(&mut self,p:(i16,i16))->i32{match self.ports.iter().position(|&v|v==p){Some(i)=>i as i32,None=>{if !self.chasing{self.fail("el final de ruta no coincide con un puerto conocido")}0x28}}}
 fn spawn_departure(&mut self,_:[u32;5]){self.fail("salida IA no admitida")}
 fn ui_route_done(&mut self,ship:u16){self.events.push(format!("fin_ruta:{ship}"))}
 fn notify(&mut self,args:[u32;6]){self.events.push(format!("aviso:{args:?}"))}
}
impl SeaHost for Host<'_>{
 fn divert_to_nearest_port_with_world(&mut self,w:&mut TickWorld,ship:u32,except:u8){self.divert(w,ship,u32::from(except))}

 fn walk_idle_human(&self)->bool{true}
 fn prepare_dead_with_world(&mut self,w:&mut TickWorld,id:u32){
  // 0x00519D90: solo si `merchant+8 == 0` compone la carta 0x1B y la entrega con 0x004D6530.
  let at=id as usize*SHIP_SIZE;let Some(ship)=w.remove.ships.get(at..at+SHIP_SIZE)else{self.fail("barco perdido fuera de la flota");return};
  let owner=usize::from(ship[0]);let Some(merchant)=w.remove.merchants.get(owner*MERCHANT_SIZE..(owner+1)*MERCHANT_SIZE)else{self.fail("comerciante del barco perdido ausente");return};
  let Some(record)=p3_core::ships::lost_letter::ship_lost_letter(ship,merchant,w.remove.captains)else{return};
  let hex:String=record.iter().map(|b|format!("{b:02x}")).collect();
  self.events.push(format!("{SHIP_LOST_LETTER}{owner}:{hex}"));
 }
 fn process_ship(&mut self,id:u32)->bool{id==u32::from(self.ship)||self.additional.iter().any(|v|u32::from(v.ship)==id)||self.route_updates.iter().any(|(ship,_,_)|u32::from(*ship)==id)}
 fn call(&mut self,c:SeaCall){match c{SeaCall::Notify(town,owner)=>self.events.push(format!("{KNOWN_TOWN}{town}:{owner}")),SeaCall::UiRefresh|SeaCall::PrepareDead(_)|SeaCall::Report{..}=>{},
  SeaCall::Remove(p3_core::ships::remove::RemoveEvent::UiA(_)|p3_core::ships::remove::RemoveEvent::UiB(_))=>self.events.push(format!("mar:{c:?}")),_=>self.fail(&format!("llamada marítima no conectada: {c:?}"))}}
 // 0x507FB0 as in pirate_group_remove: 515F00 runs before remove_ship overwrites ship+4.
 fn remove_ship_with_world(&mut self,w:&mut TickWorld,id:u32){if let Err(e)=self.remove_at_sea(w,id){self.fail(&e)}}
 fn resort(&mut self,w:&mut TickWorld,id:u32){resort_from_tick(w,id as u16)}
 fn order_remove(&mut self,w:&mut TickWorld,id:u32){remove_from_tick(w,id as u16)}
 fn clear_route(&mut self,w:&mut TickWorld,id:u32){let at=id as usize*SHIP_SIZE;route::clear_route(&mut w.remove.ships[at..at+SHIP_SIZE],id as u16,w.player,false,self)}
 fn advance(&mut self,w:&mut TickWorld,id:u32)->bool{self.current=id as u16;let at=id as usize*SHIP_SIZE;self.chasing=w.remove.ships[at+0x38]==0xff||u16::from_le_bytes([w.remove.ships[at+0x3a],w.remove.ships[at+0x3b]])<u16::from_le_bytes([w.remove.manager[0xf4],w.remove.manager[0xf5]]);let indices=(0..self.ports.len()).map(|i|i as u8).collect::<Vec<_>>();let ports=self.ports;let (time,player)=(w.time,w.player);let remove=&mut *w.remove;
  // Vistas sobre los registros de comerciante: el paso solo los lee y no tiene acceso a `w`, así que no hace falta copiarlos por barco.
  let merchants=remove.merchants.chunks_exact(MERCHANT_SIZE).collect::<Vec<&[u8]>>();let aw=AdvanceWorld{time,merchant_count:remove.merchant_count,town_count:remove.town_count,player_merchant:player,paused:0,port_index:&indices,positions:ports,merchants:&merchants};match advance_along_route(&mut remove.ships[at..at+SHIP_SIZE],id,&aw,self){Ok(v)=>v!=0,Err(_)=>{self.fail("velocidad naval inválida");true}}}
 fn pursuit(&mut self,_:u32)->bool{self.fail("persecución sin mundo");false}
 fn pursuit_with_world(&mut self,w:&mut TickWorld,id:u32)->bool{self.apply_pursuit(w,id as u16)}fn fleet_check(&mut self,_:u32)->bool{self.fail("flota no admitida");false}fn convoy_add(&mut self,_:u32)->u16{self.fail("convoy no admitido");0}fn scene_slot(&mut self,_:u8,_:u8)->u8{self.fail("escena no admitida");255}fn new_scene_slot(&mut self)->u8{self.fail("escena no admitida");255}
}
impl p3_core::ships::tick::ConvoyHost for Host<'_>{
 fn convoy_ship_finished_with_world(&mut self,w:&mut TickWorld,convoy:u16,ship:u16){
  // El planificador no cambia dentro del tick naval: se evalúa al publicar (`Sim::apply_navigation_convoy_losses`).
  let (c,s)=(usize::from(convoy)*0x3c,usize::from(ship)*SHIP_SIZE);
  let (Some(cv),Some(sh))=(w.remove.convoys.get(c..c+0x3c),w.remove.ships.get(s..s+SHIP_SIZE)) else{self.fail("baja de convoy fuera de la flota");return};
  let task=u16::from_le_bytes([cv[2],cv[3]]);let own=u16::from_le_bytes([sh[8],sh[9]]);
  self.events.push(format!("{CONVOY_LOSS}{task}:{own}:{}:{}",sh[0],u16::from_le_bytes([sh[0x15e],sh[0x15f]])));
 }
 fn plan_expedition_with_world(&mut self,w:&mut TickWorld,convoy:u16){
  let Some(context)=self.expeditions.as_deref_mut() else{
   // 503B50 retorna para propietarios no comerciantes y humanos antes de
   // consultar recursos o CRT. Las órdenes humanas no requieren esta IA.
   let at=usize::from(convoy)*0x3c;
   let Some(&owner)=w.remove.convoys.get(at)else{self.fail("convoy de expedición inexistente");return};
   if u16::from(owner)>=w.remove.merchant_count{return}
   let at=usize::from(owner)*MERCHANT_SIZE+8;
   if w.remove.merchants.get(at..at+2)==Some(&[0,0][..]){return}
   self.fail("contexto de expedición ausente");return
  };
  let env=p3_core::ships::convoy_expedition::PlanEnv{expedition_lock:context.lock,expedition_countdown:context.countdown,network_client:context.network_client,trade_centers:&context.trade_centers,locations:&context.locations,map_points:&context.map_points};
  match p3_core::ships::convoy_expedition::plan(w,convoy,&env,&mut context.rng){
   Ok(ops)=>{if !ops.is_empty(){if let Some(state)=self.absorption.as_deref_mut(){state.operations.extend(ops)}else{self.fail("cola de expedición ausente")}}},
   Err(error)=>self.fail(&error),
  }
 }

 fn absorb_convoy_with_world(&mut self,w:&mut TickWorld,object:u16,convoy:u16){
  let Some(state)=self.absorption.take() else{self.fail("bandas de absorción no configuradas");return};
  let result=crate::convoy_absorption::absorb(w,object,convoy,state,self);
  self.absorption=Some(state);if let Err(e)=result{self.fail(&e)}
 }

 fn divert_to_nearest_port_with_world(&mut self,w:&mut TickWorld,ship:u32,except:u32){self.divert(w,ship,except)}
 fn trade_for_next_destination_with_world(&mut self,w:&mut TickWorld,ship:u16){
  let Some(ai)=self.ai.as_deref_mut() else{self.fail("comercio de convoy de IA no conectado");return};
  if p3_core::ships::ai_dock::trade_for_next_destination(w,ai,ship).is_err(){self.fail("comercio de convoy de IA con datos inválidos")}
 }
 fn engage_pirate_with_world(&mut self,w:&mut TickWorld,convoy:u16){
  let Some(options)=self.pirate else{self.fail("encuentro pirata pendiente de conectar");return};
  if self.battle.is_some(){self.fail("varios encuentros en el mismo tick no admitidos");return}
  let count=usize::from(u16::from_le_bytes([w.remove.manager[0xf4],w.remove.manager[0xf5]]));let descriptors=(0..count).map(|id|route_token(id as u16)).collect::<Vec<_>>();
  let band=if let Some(scheduler)=options.scheduler{let at=usize::from(convoy)*0x3c;match p3_core::ships::pirate_engage::is_hanse_expedition(&w.remove.convoys[at..at+0x3c],convoy,scheduler){Ok(v)=>v,Err(e)=>{self.fail(e);return}}}else{options.side0_pirate_band};
  let mut result=match p3_core::ships::pirate_engage::prepare_ai_pirate_encounter_with_policy(w,convoy,options.slot,options.battle_address,band,&descriptors,options.human_as_ai){Ok(v)=>v,Err(e)=>{self.fail(&e);return}};
  if let Some(width)=options.tile_width {
   if let Err(e)=crate::battle_opening::open_automatic(&mut result.battle,w.remove.ships,w.remove.captains,w.player,width){self.fail(e);return}
  }
  for (ship,source,owned) in result.owned_routes.iter().map(|r|(r.ship,r.source,true)).chain(result.shared_routes.iter().map(|&(ship,source)|(ship,source,false))){
   let saved=self.route_updates.iter().rev().find(|(id,_,_)|*id==source).map(|(_,_,route)|route.clone()).or_else(||self.route_book.get(usize::from(source)).and_then(Clone::clone));
   let mut points=self.points_for(source).to_vec();if points.is_empty(){if let Some(p3_save::ships::SavedRoute::Owned(v))=&saved{points=v.clone()}}
   let saved=match (owned,saved){(true,Some(p3_save::ships::SavedRoute::Owned(v)))=>p3_save::ships::SavedRoute::Owned(v),(false,Some(p3_save::ships::SavedRoute::Shared{origin,destination}))=>p3_save::ships::SavedRoute::Shared{origin,destination},_=>{self.fail("ruta copiada no reconstruible");return}};
   if points.is_empty(){self.fail("puntos de ruta copiada ausentes");return}self.route_updates.push((ship,points,saved));
  }
  for ship in result.ui_ships{self.events.push(format!("convoy:cv_ui:[{ship}]"))}self.battle=Some(result.battle);
 }

 fn plan_env_with_world(&mut self,w:&TickWorld)->Option<p3_core::ships::convoy_plan::PlanEnv>{let mut env=self.ai.as_ref()?.convoy_plan.clone()?;env.time=w.time;env.town_flags=w.remove.towns.chunks_exact(TOWN_SIZE).take(w.remove.town_count as usize).map(|t|u32::from_le_bytes(t[0x2c8..0x2cc].try_into().unwrap())).collect();Some(env)}
 fn set_destination(&mut self,w:&mut TickWorld,ship:u16,x:u32,y:u32){
  let at=usize::from(ship)*SHIP_SIZE;let image=&mut w.remove.ships[at..at+SHIP_SIZE];let start=(u16::from_le_bytes([image[0x1e],image[0x1f]]) as i16,u16::from_le_bytes([image[0x22],image[0x23]]) as i16);let end=(x as u16 as i16,y as u16 as i16);
  let Some(planner)=self.planner.as_deref_mut() else{self.fail("planificador pirata ausente");return};let found=planner.find_route(&[start,end]);let points=match found.route{p3_core::route::finder::RouteOut::Shared(n)=>planner.routes.get(n).cloned(),p3_core::route::finder::RouteOut::Built(points)=>Some(points),_=>None};let Some(points)=points else{self.fail("destino pirata sin ruta");return};
  if route::apply_route_plan(image,None,end,&points,found.status!=0,route_token(ship),w.time).is_none(){self.fail("ruta pirata inválida");return}self.route_updates.push((ship,points.clone(),p3_save::ships::SavedRoute::Owned(points)));
 }
 fn pirate_attack_with_world(&mut self,w:&mut TickWorld,ship:u16)->bool{
  const ZONES:[(i16,i16,i16,i16);3]=[(442,1521,658,1691),(2265,147,2332,453),(1497,1263,1578,1444)];
  let at=usize::from(ship)*SHIP_SIZE;let has_route=w.remove.ships[at+0x24..at+0x28]!=[0;4];let points=self.points_for(ship).to_vec();if has_route&&points.is_empty(){self.fail("ruta pirata no reconstruida");return false}
  let Some(ai)=self.ai.as_ref() else{self.fail("configuración pirata ausente");return false};let route=has_route.then_some(points.as_slice());
  match p3_core::ships::prey::check_attack(w,ship,ai.pirate_difficulty,route,&ZONES){Ok((result,effect))=>{if effect.is_some(){route::clear_route(&mut w.remove.ships[at..at+SHIP_SIZE],ship,w.player,false,self)}result},Err(e)=>{self.fail(e);false}}
 }
 fn find_prey_with_world(&mut self,w:&mut TickWorld,ship:u16)->bool{
  const ZONES:[(i16,i16,i16,i16);3]=[(442,1521,658,1691),(2265,147,2332,453),(1497,1263,1578,1444)];
  match p3_core::ships::prey::find_prey(w.remove.manager,w.remove.ships,ship,&ZONES){Ok(found)=>found,Err(e)=>{self.fail(e);false}}
 }
 fn convoy_ai_dock_with_world(&mut self,w:&mut TickWorld,_convoy:u16,leader:u16){if let Some(ai)=self.ai.as_deref_mut(){p3_core::ships::ai_dock::ai_docked_update(w,leader,ai)}else{self.fail("IA de convoy atracado no conectada")}}
 fn ship_delivery_with_world(&mut self,w:&mut TickWorld,owner:u8,count:u32,success:u32){
  let at=usize::from(owner)*MERCHANT_SIZE;let Some(bytes)=w.remove.merchants.get_mut(at..at+MERCHANT_SIZE) else{self.fail("comerciante de convoy inexistente");return};
  if w.remove.town_count>40{self.fail("ciudades de reputación incompatibles");return}
  let mut merchant=Merchant{bytes:bytes.to_vec()};let env=p3_core::ai::reputation::RepEnv{town_count:w.remove.town_count,towns:&[],renown:[0;3],byte_52c:0};merchant.apply_ship_delivery_outcome(&env,count,success);bytes.copy_from_slice(&merchant.bytes);
 }
 fn pursuit_with_world(&mut self,w:&mut TickWorld,id:u16)->bool{self.apply_pursuit(w,id)}
 fn remove_order_with_world(&mut self,w:&mut TickWorld,ship:u16){p3_core::ships::order::remove_from_tick(w,ship)}
 fn process_convoy(&mut self,id:u16)->bool{if self.convoy==Some(id){self.current=self.ship;true}else if let Some(v)=self.additional.iter().find(|v|v.convoy==Some(id)){self.current=v.ship;true}else{false}}
 fn process_convoy_with_world(&mut self,w:&mut TickWorld,id:u16)->bool{
  let full_fleet=self.ai.is_some()&&self.planner.is_some();if !full_fleet&&!self.process_convoy(id){return false}
  let base=usize::from(id)*0x3c;let actual=u16::from_le_bytes([w.remove.convoys[base+0x10],w.remove.convoys[base+0x11]]);let count=u16::from_le_bytes([w.remove.manager[0xf4],w.remove.manager[0xf5]]);
  if actual<count{
   self.current=actual;self.convoy_updates.push((id,actual));
   let at=usize::from(actual)*SHIP_SIZE;let has_route=w.remove.ships[at+0x24..at+0x28]!=[0;4];
   if !has_route {
    let saved=self.route_book.get(usize::from(actual)).and_then(Option::as_ref).cloned();
    let points=match saved.as_ref(){
     Some(p3_save::ships::SavedRoute::Owned(points))=>Some(points.clone()),
     Some(p3_save::ships::SavedRoute::Shared{origin,destination})=>{
      let Some(&from)=self.ports.get(usize::from(*origin)) else{self.fail("origen de convoy no disponible");return false};let Some(&to)=self.ports.get(usize::from(*destination)) else{self.fail("destino de convoy no disponible");return false};
      let Some(planner)=self.planner.as_deref_mut() else{self.fail("planificador de convoy ausente");return false};match planner.find_route(&[from,to]).route{p3_core::route::finder::RouteOut::Shared(n)=>planner.routes.get(n).cloned(),p3_core::route::finder::RouteOut::Built(points)=>Some(points),_=>None}
     },_=>None
    };
    if let Some(points)=points{if points.is_empty(){self.fail("ruta de convoy vacía");return false}w.remove.ships[at+0x24..at+0x28].copy_from_slice(&route_token(actual).to_le_bytes());self.route_updates.push((actual,points,saved.expect("ruta comprobada")))}else if matches!(saved,Some(p3_save::ships::SavedRoute::Owned(_)|p3_save::ships::SavedRoute::Shared{..})){self.fail("ruta de convoy no reconstruible");return false}
   }
  }true
 }
 fn hook(&mut self,name:&'static str,args:&[u32])->u32{match name {
  "show"|"leave"|"cv_ui"|"report"=>{self.events.push(format!("convoy:{name}:{args:?}"));0},
  "cv_501e10"=>0,
  "cv_free"=>{let points=(u32::from(self.current)+1)<<16;let descriptor=route_token(self.current);if args!=[points]&&args!=[descriptor]{self.fail("liberación de ruta de convoy desconocida")}else if !self.freed_routes.contains(&self.current){self.freed_routes.push(self.current)}0},
  "cv_obj_new"=>{if self.pending_expedition_task.is_some(){self.fail("tarea de expedición anterior sin completar");return 0}let Some(context)=self.expeditions.as_deref_mut()else{self.fail("planificador para expedición perdida ausente");return 0};let scheduler=&mut context.scheduler;if scheduler.records.len()!=usize::from(scheduler.capacity)||scheduler.free_head>scheduler.capacity||(scheduler.free_head==scheduler.capacity&&scheduler.capacity.checked_add(p3_core::scheduler::GROWTH).is_none()){self.fail("planificador de expedición inválido");return 0}let index=scheduler.add_task(args[0]);self.pending_expedition_task=Some(index);u32::from(index)},
  "cv_obj_set"=>{let Some(index)=self.pending_expedition_task.take()else{self.fail("tarea de expedición no reservada");return 0};let Some(context)=self.expeditions.as_deref_mut()else{self.fail("planificador para expedición perdida ausente");return 0};let Some(task)=context.scheduler.records.get_mut(usize::from(index))else{self.fail("tarea de expedición ausente");return 0};task.kind=args[2]as u16;task.payload[..2].copy_from_slice(&(args[0]as u16).to_le_bytes());task.payload[2..4].copy_from_slice(&(args[1]as u16).to_le_bytes());task.set_u32_at(0xc,args[3]);0},
  "cv_map"=>{if let Some(ai)=self.ai.as_deref_mut(){u32::from(p3_core::ships::ai_dock::AiDockHost::distance(ai,args[0],args[1]))}else{self.fail("matriz de distancias naval ausente");0xff}},
  "cv_517260"=>{let point=(args[0] as u16 as i16,args[1] as u16 as i16);if let Some(planner)=self.planner.as_deref().filter(|p|p.town_count>0){let index=p3_core::route::planner::find_town_at_point(&planner.town_slots,&planner.town_order,planner.town_count,point,None);if index<40{index}else{u32::MAX}}else{self.ports.iter().position(|&p|p==point).map_or(u32::MAX,|i|i as u32)}},
  _=>{self.fail(&format!("efecto de convoy no conectado: {name}"));0}
 }}
 // Cada dirección solo puede pertenecer a un barco (descriptor `0x10000+16·id`, puntos en `(id+1)<<16`): se calcula el
 // candidato y se comprueba que es un viaje conocido, en vez de recorrer todos los viajes buscando sus puntos.
 fn read(&mut self,addr:u32,width:u8)->u32{
  let known=|h:&Self,id:u16|id==h.ship||h.additional.iter().any(|v|v.ship==id)||h.route_updates.iter().any(|(s,_,_)|*s==id);
  if width==4{
   let rel=addr.wrapping_sub(0x10000);
   if (rel%16==0||rel%16==4)&&rel/16<=0xffff{let id=(rel/16) as u16;if known(self,id){return if rel%16==0{(u32::from(id)+1)<<16}else{self.points_for(id).len() as u32}}}
  }else if width==2{
   let id=(addr>>16).wrapping_sub(1) as u16;
   if known(self,id){let offset=(addr&0xffff) as usize;if let Some(&(x,y))=self.points_for(id).get(offset/4){return if offset%4==0{x as u16 as u32}else if offset%4==2{y as u16 as u32}else{self.fail("lectura de ruta desalineada");0}}}
  }
  self.fail("lectura de ruta inexistente");0}
 fn begin_departure(&mut self,w:&mut TickWorld,cv:u16)->u32{let base=usize::from(cv)*0x3c;let leader=u16::from_le_bytes([w.remove.convoys[base+0x10],w.remove.convoys[base+0x11]]);let at=usize::from(leader)*SHIP_SIZE;if at+SHIP_SIZE>w.remove.ships.len(){self.fail("líder de convoy inexistente");return 0}self.current=leader;self.convoy_updates.push((cv,leader));let town=usize::from(w.remove.ships[at+0x39]);let Some(&(x,y))=self.ports.get(town) else{self.fail(&format!("puerto ausente: convoy {cv}, líder {leader}, ciudad {town}"));return 0};match p3_core::ships::convoy::begin_departure(w,cv,(x as u16,y as u16),self){
  Ok(v)=>u32::from(v),Err(e) if e=="ruta ausente"=>{
   let image=&w.remove.ships[at..at+SHIP_SIZE];let destination=usize::from(image[0x38]);let Some(&end)=self.ports.get(destination) else{self.fail("destino de convoy ausente");return 0};
   let Some(planner)=self.planner.as_deref_mut() else{self.fail("planificador de convoy ausente");return 0};let found=planner.find_route(&[(x,y),end]);
   let (points,saved)=match found.route{p3_core::route::finder::RouteOut::Shared(n)=>{let Some(points)=planner.routes.get(n).cloned() else{self.fail("ruta compartida de convoy inválida");return 0};(points,p3_save::ships::SavedRoute::Shared{origin:town as u8,destination:destination as u8})},p3_core::route::finder::RouteOut::Built(points)=>{let saved=p3_save::ships::SavedRoute::Owned(points.clone());(points,saved)},_=>{self.fail("ruta de convoy no disponible");return 0}};
   match p3_core::ships::convoy::begin_departure_with_route(w,cv,(x as u16,y as u16),self,Some((&points,found.status!=0,route_token(leader)))){Ok(v)=>{if v{self.route_updates.push((leader,points,saved))}u32::from(v)},Err(e)=>{self.fail(&e);0}}
  },Err(e)=>{self.fail(&e);0}}}
 fn set_half_speed(&mut self,w:&mut TickWorld,cv:u16){if let Err(e)=p3_core::ships::convoy::set_ship_speeds_to_half(w,cv){self.fail(&e)}}
 fn insert_order(&mut self,w:&mut TickWorld,s:u16,after:u16){p3_core::ships::order::insert_from_tick(w,s,after)}
 fn recompute_ship(&mut self,w:&mut TickWorld,s:u16){let at=usize::from(s)*SHIP_SIZE;p3_core::ships::cargo::recompute_load(&mut w.remove.ships[at..at+SHIP_SIZE]);}
 fn refresh_convoy(&mut self,w:&mut TickWorld,cv:u16){if p3_core::ships::convoy::refresh_from_ships(w,cv).is_err(){self.fail("recuento de convoy inválido")}}
 fn set_velocity(&mut self,w:&mut TickWorld,s:u16,v:u32,x:u16,y:u16){let at=usize::from(s)*SHIP_SIZE;if p3_core::ships::motion::set_velocity_towards(&mut w.remove.ships[at..at+SHIP_SIZE],v as i32,x as i16,y as i16).is_err(){self.fail("velocidad de convoy inválida")}}
 fn set_motion(&mut self,w:&mut TickWorld,t:u32,v:u32,cv:u16,h:u8){if let Err(e)=p3_core::ships::convoy::set_convoy_motion(w,cv,t,v,h){self.fail(&e)}}
 fn clear_ship_route(&mut self,w:&mut TickWorld,s:u16){let at=usize::from(s)*SHIP_SIZE;route::clear_route(&mut w.remove.ships[at..at+SHIP_SIZE],s,w.player,false,self)}
 fn resort_ship(&mut self,w:&mut TickWorld,s:u16){resort_from_tick(w,s)}
 fn remove_order(&mut self,w:&mut TickWorld,s:u16){remove_from_tick(w,s)}
 fn finish_docking(&mut self,w:&mut TickWorld,s:u16){let threshold=u16::from_le_bytes([w.remove.manager[0xf6],w.remove.manager[0xf7]]);finish_docking_from_tick(w,self,s,threshold)}
}
pub fn refresh_lists(snapshot:&mut NavigationSnapshot)->Result<(),String>{
 let count=snapshot.ships.len()/SHIP_SIZE;let mut seen=vec![false;count];
 let docked_head=u16::from_le_bytes([snapshot.manager[0xe8],snapshot.manager[0xe9]]);let sea_head=u16::from_le_bytes([snapshot.manager[0xea],snapshot.manager[0xeb]]);
 let mut walk=|mut current:u16|->Result<Vec<u16>,String>{let mut out=Vec::new();while usize::from(current)<count{let i=usize::from(current);if seen[i]{return Err("ciclo o duplicado en listas navales".into())}seen[i]=true;out.push(current);let at=i*SHIP_SIZE;current=u16::from_le_bytes([snapshot.ships[at+6],snapshot.ships[at+7]]);}Ok(out)};
 let docked=walk(docked_head)?;let sea=walk(sea_head)?;snapshot.lists=p3_save::ships::NavigationLists{docked_head,sea_head,docked,sea};Ok(())
}
fn route_token(id:u16)->u32{0x10000+u32::from(id)*16}
/// Publica el descriptor lógico junto con los puntos de una orden. S02 y la
/// fábrica usan esta tabla; no basta con conservar los puntos en LiveVoyage.
fn record_order_route(snapshot:&mut NavigationSnapshot,ship:u16,points:&[(i16,i16)],town_count:usize)->Result<(),String>{
 let i=usize::from(ship);let image=snapshot.ships.get(i*SHIP_SIZE..(i+1)*SHIP_SIZE).ok_or("barco de ruta ausente")?;
 let route=if image[0x24..0x28]==[0;4]{p3_save::ships::SavedRoute::None}
 else if image[0x36]!=0||usize::from(image[0x37])>=town_count||usize::from(image[0x38])>=town_count{
  if points.is_empty(){return Err("orden con ruta propia sin puntos".into())}p3_save::ships::SavedRoute::Owned(points.to_vec())
 }else{p3_save::ships::SavedRoute::Shared{origin:image[0x37],destination:image[0x38]}};
 *snapshot.routes.get_mut(i).ok_or("tabla de rutas truncada")?=Some(route);Ok(())
}
impl LiveVoyage {
 pub fn add_resumed(&mut self,ship:u16,shared:Option<Vec<(i16,i16)>>)->Result<(),String>{if self.ship==ship||self.additional.iter().any(|v|v.ship==ship){return Err("viaje duplicado".into())}let other=Self::resume(self.snapshot.clone(),ship,self.ports.clone(),self.player,shared)?;self.snapshot=other.snapshot;self.additional.push(ExtraVoyage{ship,convoy:other.convoy,points:other.points});Ok(())}
 /// Reanuda un barco de cualquier propietario conservando el jugador visual.
 pub fn add_resumed_for_owner(&mut self,ship:u16,owner:u32,shared:Option<Vec<(i16,i16)>>)->Result<(),String>{if self.ship==ship||self.additional.iter().any(|v|v.ship==ship){return Err("viaje duplicado".into())}let other=Self::resume(self.snapshot.clone(),ship,self.ports.clone(),owner,shared)?;self.snapshot=other.snapshot;self.additional.push(ExtraVoyage{ship,convoy:other.convoy,points:other.points});Ok(())}
 pub fn add_order(&mut self,ship:u16,destination:u8,points:Vec<(i16,i16)>,towns:&[Town],tick:u32)->Result<(),String>{if self.ship==ship||self.additional.iter().any(|v|v.ship==ship){return Err("viaje duplicado".into())}let other=Self::begin(self.snapshot.clone(),ship,destination,points,self.ports.clone(),towns,self.player,tick)?;self.snapshot=other.snapshot;self.additional.push(ExtraVoyage{ship,convoy:other.convoy,points:other.points});Ok(())}
 pub fn add_order_with_merchants(&mut self,ship:u16,destination:u8,points:Vec<(i16,i16)>,towns:&[Town],merchants:&mut [Merchant],tick:u32)->Result<(),String>{if self.ship==ship||self.additional.iter().any(|v|v.ship==ship){return Err("viaje duplicado".into())}let other=Self::begin_with_merchants(self.snapshot.clone(),ship,destination,points,self.ports.clone(),towns,merchants,self.player,tick)?;self.snapshot=other.snapshot;self.additional.push(ExtraVoyage{ship,convoy:other.convoy,points:other.points});Ok(())}
 pub fn manages_ship(&self,ship:u16)->bool{if self.ship==ship||self.additional.iter().any(|v|v.ship==ship){return true}let at=usize::from(ship)*SHIP_SIZE;let Some(image)=self.snapshot.ships.get(at..at+SHIP_SIZE) else{return false};let cid=u16::from_le_bytes([image[8],image[9]]);self.convoy==Some(cid)||self.additional.iter().any(|v|v.convoy==Some(cid))}
 pub fn reverse_phase(&mut self,ship:u16,destination:u8,points:Vec<(i16,i16)>,towns:&[Town],merchants:&mut [Merchant],tick:u32)->Result<(),String>{
  let at=usize::from(ship)*SHIP_SIZE;let image=self.snapshot.ships.get(at..at+SHIP_SIZE).ok_or("barco inexistente")?;if u32::from(image[0])!=self.player{return Err("barco ajeno".into())}let convoy=u16::from_le_bytes([image[8],image[9]]);let member=self.ship!=ship&&!self.additional.iter().any(|v|v.ship==ship&&v.convoy==Some(convoy));if !self.manages_ship(ship)||!active_convoy(&self.snapshot,convoy,self.player){return Err("convoy no activo".into())}let port=*self.ports.get(usize::from(destination)).ok_or("destino inválido")?;
  let mut staged=self.snapshot.clone();let mut town_bytes=town_image(towns);let mut merchant_bytes=merchant_image(merchants);let mut remove=RemoveWorld{manager:&mut staged.manager,ships:&mut staged.ships,captains:&mut staged.captains,convoys:&mut staged.convoys,merchants:&mut merchant_bytes,towns:&mut town_bytes,pool_limit:0,merchant_count:merchants.len() as u16,town_count:towns.len() as u32,ui_a:false,ui_b:false};let mut world=TickWorld{remove:&mut remove,time:tick,player:self.player};let routed=if member{p3_core::ships::departure::order_member_during_maneuver(&mut world,ship,destination,&points,port,route_token(ship))?}else{p3_core::ships::departure::reverse_convoy_phase(&mut world,ship,destination,&points,port,route_token(ship))?};record_order_route(&mut staged,ship,&points,towns.len())?;refresh_lists(&mut staged)?;for (m,src) in merchants.iter_mut().zip(merchant_bytes.chunks_exact(MERCHANT_SIZE)){m.bytes.copy_from_slice(src)}self.snapshot=staged;let points=if routed{points}else{Vec::new()};if member{self.additional.push(ExtraVoyage{ship,convoy:None,points})}else if self.ship==ship{self.points=points}else{self.additional.iter_mut().find(|v|v.ship==ship).expect("viaje comprobado").points=points}Ok(())
 }
 pub fn reroute_at_sea(&mut self,ship:u16,destination:u8,points:Vec<(i16,i16)>,towns:&[Town],merchants:&mut [Merchant],tick:u32)->Result<(),String>{
  let at=usize::from(ship)*SHIP_SIZE;let image=self.snapshot.ships.get(at..at+SHIP_SIZE).ok_or("barco inexistente")?;if u32::from(image[0])!=self.player{return Err("barco ajeno".into())}let convoy=u16::from_le_bytes([image[8],image[9]]);let managed=if self.ship==ship{self.convoy==Some(convoy)}else{self.additional.iter().any(|v|v.ship==ship&&v.convoy==Some(convoy))};let member=!managed;
  if member&&!std::iter::once(self.convoy).chain(self.additional.iter().map(|v|v.convoy)).any(|v|v==Some(convoy)){return Err("convoy no gestionado".into())}
  let port=*self.ports.get(usize::from(destination)).ok_or("destino inválido")?;if points.len()<2||points.last()!=Some(&port){return Err("ruta inválida".into())}if !active_convoy(&self.snapshot,convoy,self.player){return Err("convoy no activo".into())}
  let mut staged=self.snapshot.clone();let mut town_bytes=town_image(towns);let mut merchant_bytes=merchant_image(merchants);let mut remove=RemoveWorld{manager:&mut staged.manager,ships:&mut staged.ships,captains:&mut staged.captains,convoys:&mut staged.convoys,merchants:&mut merchant_bytes,towns:&mut town_bytes,pool_limit:0,merchant_count:merchants.len() as u16,town_count:towns.len() as u32,ui_a:false,ui_b:false};let mut world=TickWorld{remove:&mut remove,time:tick,player:self.player};if member{p3_core::ships::departure::reroute_convoy_member_at_sea(&mut world,ship,destination,&points,port,route_token(ship))?}else{p3_core::ships::departure::reroute_convoy_at_sea(&mut world,ship,destination,&points,port,route_token(ship))?};
  record_order_route(&mut staged,ship,&points,towns.len())?;refresh_lists(&mut staged)?;for (m,src) in merchants.iter_mut().zip(merchant_bytes.chunks_exact(MERCHANT_SIZE)){m.bytes.copy_from_slice(src)}self.snapshot=staged;if member{self.additional.push(ExtraVoyage{ship,convoy:None,points})}else if self.ship==ship{self.points=points}else{self.additional.iter_mut().find(|v|v.ship==ship).expect("viaje comprobado").points=points}Ok(())
 }
 pub fn idle(snapshot:NavigationSnapshot,ports:Vec<(i16,i16)>,player:u32)->Self{Self{additional:vec![],baseline:snapshot.clone(),snapshot,ship:u16::MAX,convoy:None,points:vec![],ports,player,events:vec![]}}
 pub fn active(&self)->bool{std::iter::once(self.ship).chain(self.additional.iter().map(|v|v.ship)).any(|id|{if id==u16::MAX{return false}let at=usize::from(id)*SHIP_SIZE;self.snapshot.ships.get(at+0x134..at+0x136)!=Some(&[0,0][..])})}
 /// Retira la caché de un viaje cuya ruta fue liberada durante una captura.
 pub fn forget_captured_route(&mut self,id:u16){
  if self.ship==id{if self.additional.is_empty(){self.ship=u16::MAX;self.convoy=None;self.points.clear()}else{let next=self.additional.remove(0);self.ship=next.ship;self.convoy=next.convoy;self.points=next.points}}
  else{self.additional.retain(|v|v.ship!=id)}
 }
 pub fn route_for(&self,id:u16)->Option<&[(i16,i16)]>{if self.ship==id{Some(&self.points)}else{self.additional.iter().find(|v|v.ship==id).map(|v|v.points.as_slice())}}
 /// Reanuda una ruta propia o el último tramo de atraque desde S02.
 pub fn resume(mut snapshot:NavigationSnapshot,ship:u16,ports:Vec<(i16,i16)>,player:u32,shared:Option<Vec<(i16,i16)>>)->Result<Self,String>{
  if ship==u16::MAX{return Err("identificador naval reservado".into())}
  let at=usize::from(ship)*SHIP_SIZE;let image=snapshot.ships.get(at..at+SHIP_SIZE).ok_or("barco inexistente")?;let state=u16::from_le_bytes([image[0x134],image[0x135]]);let destination=image[0x38];
  if u32::from(image[0])!=player||!matches!(state,1|2|3|0xf){return Err("barco no reanudable".into())}
  let cid=u16::from_le_bytes([image[8],image[9]]);let total=u16::from_le_bytes([snapshot.manager[0xf6],snapshot.manager[0xf7]]);let convoy=if cid<total {let at=usize::from(cid)*0x3c;if snapshot.convoys.get(at+0x10..at+0x12)!=Some(&ship.to_le_bytes())||!active_convoy(&snapshot,cid,player){return Err("convoy inactivo o líder inválido".into())}Some(cid)}else{None};
  if state==1&&convoy.is_none(){return Err("espera sin convoy no reanudable".into())}
  if convoy.is_none()&&((state==0xf&&!snapshot.lists.sea.contains(&ship))||(state!=0xf&&!snapshot.lists.docked.contains(&ship))){return Err("lista naval incompatible".into())}
  // El paso 1→2 del convoy puede guardarse antes de su siguiente tick.
  // Con fase 0 el original prepara la ruta al iniciar la salida (501EB0).
  let convoy_awaiting_departure=state==2&&convoy.is_some_and(|id|{let at=usize::from(id)*0x3c;u16::from_le_bytes([snapshot.convoys[at+0x12],snapshot.convoys[at+0x13]])==2&&u16::from_le_bytes([snapshot.convoys[at+0x16],snapshot.convoys[at+0x17]])==0});
  let baseline=snapshot.clone();let points=match snapshot.routes.get(usize::from(ship)).and_then(Option::as_ref){
   Some(p3_save::ships::SavedRoute::Owned(_))=>snapshot.bind_route(usize::from(ship),route_token(ship),|_,_|None).map_err(str::to_string)?,
   Some(p3_save::ships::SavedRoute::Shared{..})=>snapshot.bind_route(usize::from(ship),route_token(ship),|_,_|shared.clone()).map_err(str::to_string)?,
   Some(p3_save::ships::SavedRoute::None) if state==3||(state==1&&convoy.is_some())||convoy_awaiting_departure||(state==0xf&&destination==0xff)=>Vec::new(),
   _=>return Err("ruta ausente no reanudable".into()),
  };
  if !points.is_empty()&&destination!=0xff&&!ports.contains(points.last().unwrap())&&u16::from_le_bytes([snapshot.ships[at+0x3a],snapshot.ships[at+0x3b]])>=u16::from_le_bytes([snapshot.manager[0xf4],snapshot.manager[0xf5]]){return Err("destino de ruta no identificado".into())}
  Ok(Self{additional:vec![],baseline,snapshot,ship,convoy,points,ports,player,events:vec![]})
 }
 /// Prepara una orden de barco humano individual, con ruta propia y sin convoy.
 pub fn begin(snapshot:NavigationSnapshot,ship:u16,destination:u8,points:Vec<(i16,i16)>,ports:Vec<(i16,i16)>,towns:&[Town],player:u32,tick:u32)->Result<Self,String>{Self::begin_with_merchants(snapshot,ship,destination,points,ports,towns,&mut [],player,tick)}
 pub fn begin_with_merchants(mut snapshot:NavigationSnapshot,ship:u16,destination:u8,points:Vec<(i16,i16)>,ports:Vec<(i16,i16)>,towns:&[Town],merchant_images:&mut [Merchant],player:u32,tick:u32)->Result<Self,String>{
  if ship==u16::MAX{return Err("identificador naval reservado".into())}
  if towns.len()!=ports.len()||usize::from(destination)>=towns.len()||points.len()<2||points.last()!=ports.get(destination as usize){return Err("ruta o destino incompatible".into())}
  if ports.iter().filter(|&&p|p==ports[destination as usize]).count()!=1{return Err("puerto de destino ambiguo".into())}
  let baseline=snapshot.clone();let at=usize::from(ship).checked_mul(SHIP_SIZE).ok_or("barco inválido")?;let image=snapshot.ships.get_mut(at..at+SHIP_SIZE).ok_or("barco inexistente")?;
  if u32::from(image[0])!=player||usize::from(image[0x39])>=towns.len(){return Err("barco o propietario inválido".into())}
  let cap=u16::from_le_bytes([snapshot.manager[0xf2],snapshot.manager[0xf3]]);let conv=u16::from_le_bytes([snapshot.manager[0xf6],snapshot.manager[0xf7]]);
  if !snapshot.lists.docked.contains(&ship){
   let cid=u16::from_le_bytes([image[8],image[9]]);let offset=usize::from(cid)*0x3c;let is_leader=cid<conv&&snapshot.convoys.get(offset+0x10..offset+0x12)==Some(&ship.to_le_bytes());
   let mut towns_bytes=town_image(towns);let mut merchants=merchant_images.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();
   let mut remove=RemoveWorld{manager:&mut snapshot.manager,ships:&mut snapshot.ships,captains:&mut snapshot.captains,convoys:&mut snapshot.convoys,merchants:&mut merchants,towns:&mut towns_bytes,pool_limit:0,merchant_count:merchant_images.len() as u16,town_count:towns.len() as u32,ui_a:false,ui_b:false};let mut world=TickWorld{remove:&mut remove,time:tick,player};
   if is_leader {if !active_convoy(&baseline,cid,player){return Err("convoy fuera de la lista activa".into())}p3_core::ships::departure::start_convoy_leader(&mut world,ship,destination,&points,ports[destination as usize],route_token(ship))?;record_order_route(&mut snapshot,ship,&points,towns.len())?;for (m,src) in merchant_images.iter_mut().zip(merchants.chunks_exact(MERCHANT_SIZE)){m.bytes.copy_from_slice(src)}return Ok(Self{additional:vec![],baseline,snapshot,ship,convoy:Some(cid),points,ports,player,events:vec![]})}
   p3_core::ships::departure::start_convoy_member(&mut world,ship,destination,&points,ports[destination as usize],route_token(ship))?;
   record_order_route(&mut snapshot,ship,&points,towns.len())?;for (m,src) in merchant_images.iter_mut().zip(merchants.chunks_exact(MERCHANT_SIZE)){m.bytes.copy_from_slice(src)}snapshot.lists.docked.insert(0,ship);snapshot.lists.docked_head=ship;
   return Ok(Self{additional:vec![],baseline,snapshot,ship,convoy:None,points,ports,player,events:vec![]});
  }
  let ready=p3_core::ships::departure::ready_to_sail(image,towns,cap).ok_or("barco inválido")?;if !ready{return Err("el barco no está preparado para zarpar".into())}
  let end=ports[destination as usize];let token=route_token(ship);let mut planned=false;
  let started=p3_core::ships::departure::start_docked(image,destination,towns.len() as u32,conv,true,|image,d|{planned=route::apply_route_plan(image,Some((d,end.0,end.1)),end,&points,true,token,tick).is_some()}).ok_or("el barco no está disponible o pertenece a un convoy")?;
  if !started||!planned{return Err("no se pudo preparar la ruta".into())}
  record_order_route(&mut snapshot,ship,&points,towns.len())?;
  Ok(Self{additional:vec![],baseline,snapshot,ship,convoy:None,points,ports,player,events:vec![]})
 }
 /// Avanza las ramas conectadas de flota aunque no queden viajes gestionados.
 pub fn step_idle(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32)->Result<(),String>{let mut idle=Self::idle(self.snapshot.clone(),self.ports.clone(),self.player);idle.step(towns,merchants,tick)?;self.snapshot=idle.snapshot;self.events.extend(idle.events);Ok(())}
 /// Como `step_idle`, pero la IA atracada comercia y zarpa con `ai` (distancias, oficinas, dificultad): compra, venta,
 /// armado y salida o unión a un convoy en formación. El viaje posterior de esos convoyes en el mar sigue sin conectar.
 pub fn step_idle_with_ai_dock(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,ai:&mut p3_core::ships::ai_dock::AiDockData)->Result<(),String>{let mut idle=Self::idle(self.snapshot.clone(),self.ports.clone(),self.player);idle.step_with_ai(towns,merchants,tick,None,ai)?;self.snapshot=idle.snapshot;self.events.extend(idle.events);Ok(())}
 /// Ejecuta IA y viajes en la misma transacción; revierte las oficinas ante error.
 pub fn step_with_ai(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,planner:Option<&mut p3_core::route::finder::RoutePlanner>,ai:&mut p3_core::ships::ai_dock::AiDockData)->Result<(),String>{
  let mut staged=self.clone();let mut staged_towns=towns.to_vec();let mut staged_merchants=merchants.to_vec();let mut staged_ai=ai.clone();let mut staged_planner=planner.as_deref().cloned();
  staged.step_internal(&mut staged_towns,&mut staged_merchants,tick,staged_planner.as_mut(),None,None,Some(&mut staged_ai),None,None,None)?;
  if staged.snapshot.convoys.len()!=self.snapshot.convoys.len(){return Err("crecimiento del pool de convoyes aún no persistible".into())}
  towns.clone_from_slice(&staged_towns);merchants.clone_from_slice(&staged_merchants);if let (Some(dst),Some(src))=(planner,staged_planner){*dst=src}*self=staged;*ai=staged_ai;Ok(())
 }
 /// Avanza flota e IA y devuelve un encuentro pirata preparado. La ranura y
 /// clasificación las aporta el anfitrión; aún debe publicar la batalla en S05.
 pub fn step_with_ai_and_pirate(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,planner:Option<&mut p3_core::route::finder::RoutePlanner>,ai:&mut p3_core::ships::ai_dock::AiDockData,options:PirateEncounterOptions<'_>)->Result<Option<p3_core::ships::battle::CommercialBattleStart>,String>{
  let mut staged=self.clone();let mut staged_towns=towns.to_vec();let mut staged_merchants=merchants.to_vec();let mut staged_ai=ai.clone();let mut staged_planner=planner.as_deref().cloned();
  let battle=staged.step_internal(&mut staged_towns,&mut staged_merchants,tick,staged_planner.as_mut(),None,Some(options),Some(&mut staged_ai),None,None,None)?;
  if staged.snapshot.convoys.len()!=self.snapshot.convoys.len(){return Err("crecimiento del pool de convoyes aún no persistible".into())}
  towns.clone_from_slice(&staged_towns);merchants.clone_from_slice(&staged_merchants);if let (Some(dst),Some(src))=(planner,staged_planner){*dst=src}*self=staged;*ai=staged_ai;Ok(battle)
 }
 /// Tick completo con bandas, operaciones de absorción y encuentro opcional.
 pub fn step_with_ai_and_bands(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,planner:Option<&mut p3_core::route::finder::RoutePlanner>,ai:&mut p3_core::ships::ai_dock::AiDockData,options:Option<PirateEncounterOptions<'_>>,bands:&mut crate::convoy_absorption::AbsorptionState)->Result<Option<p3_core::ships::battle::CommercialBattleStart>,String>{
  let mut staged=self.clone();let mut staged_towns=towns.to_vec();let mut staged_merchants=merchants.to_vec();let mut staged_ai=ai.clone();let mut staged_planner=planner.as_deref().cloned();let mut staged_bands=bands.clone();
  let battle=staged.step_internal(&mut staged_towns,&mut staged_merchants,tick,staged_planner.as_mut(),None,options,Some(&mut staged_ai),Some(&mut staged_bands),None,None)?;
  if staged.snapshot.convoys.len()!=self.snapshot.convoys.len(){return Err("crecimiento del pool de convoyes aún no persistible".into())}
  towns.clone_from_slice(&staged_towns);merchants.clone_from_slice(&staged_merchants);if let (Some(dst),Some(src))=(planner,staged_planner){*dst=src}*self=staged;*ai=staged_ai;*bands=staged_bands;Ok(battle)
 }
 /// Flota completa: absorción y expediciones comparten la cola en orden original.
 pub fn step_with_ai_bands_and_expeditions(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,planner:Option<&mut p3_core::route::finder::RoutePlanner>,ai:&mut p3_core::ships::ai_dock::AiDockData,options:Option<PirateEncounterOptions<'_>>,bands:&mut crate::convoy_absorption::AbsorptionState,expeditions:&mut ExpeditionPlanning)->Result<Option<p3_core::ships::battle::CommercialBattleStart>,String>{
  let mut staged=self.clone();let mut staged_towns=towns.to_vec();let mut staged_merchants=merchants.to_vec();let mut staged_ai=ai.clone();let mut staged_planner=planner.as_deref().cloned();let mut staged_bands=bands.clone();let mut staged_expeditions=expeditions.clone();
  let battle=staged.step_internal(&mut staged_towns,&mut staged_merchants,tick,staged_planner.as_mut(),None,options,Some(&mut staged_ai),Some(&mut staged_bands),Some(&mut staged_expeditions),None)?;
  if staged.snapshot.convoys.len()!=self.snapshot.convoys.len(){return Err("crecimiento del pool de convoyes aún no persistible".into())}
  towns.clone_from_slice(&staged_towns);merchants.clone_from_slice(&staged_merchants);if let (Some(dst),Some(src))=(planner,staged_planner){*dst=src}*self=staged;*ai=staged_ai;*bands=staged_bands;*expeditions=staged_expeditions;Ok(battle)
 }
 /// Igual que `step_with_ai_bands_and_expeditions`, sin la copia de respaldo: para un llamante que ya trabaja sobre
 /// copias propias y las descarta enteras ante un error (el tick naval de `NullHost`). Con `Ok` el resultado es idéntico;
 /// con `Err` los argumentos quedan en un estado intermedio que el llamante no debe usar.
 pub fn step_with_ai_bands_and_expeditions_staged(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,planner:Option<&mut p3_core::route::finder::RoutePlanner>,ai:&mut p3_core::ships::ai_dock::AiDockData,options:Option<PirateEncounterOptions<'_>>,bands:&mut crate::convoy_absorption::AbsorptionState,expeditions:&mut ExpeditionPlanning)->Result<Option<p3_core::ships::battle::CommercialBattleStart>,String>{
  let convoys=self.snapshot.convoys.len();
  let battle=self.step_internal(towns,merchants,tick,planner,None,options,Some(ai),Some(bands),Some(expeditions),None)?;
  if self.snapshot.convoys.len()!=convoys{return Err("crecimiento del pool de convoyes aún no persistible".into())}
  Ok(battle)
 }
 /// Igual que `step_with_ai_bands_and_expeditions_staged`, pero el tick trabaja directamente sobre `self.snapshot`,
 /// `planner`, `bands` y `expeditions` en vez de sobre copias que se confirman al final. Con `Ok` el resultado es
 /// idéntico. Con `Err` no deshace nada: esos cuatro quedan a medio tick y el llamante debe restaurarlos desde su
 /// propia copia de respaldo, como hace el tick naval de `NullHost`. Ciudades, comerciantes, rutas y puntos de ruta
 /// solo se escriben al final, cuando ya no puede fallar; si va a cambiar rutas o puntos deja antes en `undo` los
 /// anteriores, para que el llamante pueda deshacer el tick si falla algo posterior (el clima).
 pub fn step_with_ai_bands_and_expeditions_live(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,planner:Option<&mut p3_core::route::finder::RoutePlanner>,ai:&mut p3_core::ships::ai_dock::AiDockData,options:Option<PirateEncounterOptions<'_>>,bands:&mut crate::convoy_absorption::AbsorptionState,expeditions:&mut ExpeditionPlanning,undo:&mut Option<VoyageUndo>)->Result<Option<p3_core::ships::battle::CommercialBattleStart>,String>{
  let convoys=self.snapshot.convoys.len();
  let battle=self.step_internal(towns,merchants,tick,planner,None,options,Some(ai),Some(bands),Some(expeditions),Some(undo))?;
  if self.snapshot.convoys.len()!=convoys{return Err("crecimiento del pool de convoyes aún no persistible".into())}
  Ok(battle)
 }
 /// Un tick atómico de la ruta seleccionada. Un estado no soportado no se aplica.
 pub fn step(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32)->Result<(),String>{self.step_with_planner(towns,merchants,tick,None)}
 pub fn step_with_planner(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,planner:Option<&mut p3_core::route::finder::RoutePlanner>)->Result<(),String>{
  self.step_internal(towns,merchants,tick,planner,None,None,None,None,None,None).map(|_|())
 }
 /// Devuelve el encuentro para que el anfitrión publique sus efectos y detenga
 /// el reloj si corresponde. El tick se revierte ante cualquier error.
 pub fn step_with_encounter(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,planner:Option<&mut p3_core::route::finder::RoutePlanner>,options:EncounterOptions<'_>)->Result<Option<p3_core::ships::battle::CommercialBattleStart>,String>{
  self.step_internal(towns,merchants,tick,planner,Some(options),None,None,None,None,None)
 }
 fn step_internal(&mut self,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,planner:Option<&mut p3_core::route::finder::RoutePlanner>,encounter:Option<EncounterOptions<'_>>,pirate:Option<PirateEncounterOptions<'_>>,ai:Option<&mut p3_core::ships::ai_dock::AiDockData>,absorption:Option<&mut crate::convoy_absorption::AbsorptionState>,expeditions:Option<&mut ExpeditionPlanning>,undo:Option<&mut Option<VoyageUndo>>)->Result<Option<p3_core::ships::battle::CommercialBattleStart>,String>{
  if self.ports.len()!=towns.len()||towns.len()>255{return Err("puertos y ciudades incompatibles".into())}
  let Some(undo)=undo else{let mut staged=self.snapshot.clone();let battle=self.step_on(&mut staged,None,None,towns,merchants,tick,planner,encounter,pirate,ai,absorption,expeditions)?;self.snapshot=staged;return Ok(battle)};
  // Modo vivo: la instantánea sale de `self` mientras dura el tick (sin copia) y vuelve siempre, también con error.
  // De la versión anterior solo hace falta saber qué barcos tenían ruta.
  let had_route:Vec<bool>=(0..self.snapshot.routes.len()).map(|i|{let at=i*SHIP_SIZE+0x24;self.snapshot.ships[at..at+4]!=[0;4]}).collect();
  let empty=NavigationSnapshot{manager:Vec::new(),ships:Vec::new(),captains:Vec::new(),convoys:Vec::new(),routes:Vec::new(),lists:p3_save::ships::NavigationLists{docked_head:0,sea_head:0,docked:Vec::new(),sea:Vec::new()}};
  let mut staged=std::mem::replace(&mut self.snapshot,empty);
  let result=self.step_on(&mut staged,Some(&had_route),Some(undo),towns,merchants,tick,planner,encounter,pirate,ai,absorption,expeditions);
  self.snapshot=staged;result
 }
 /// Cuerpo del tick sobre `staged`. Con `had_route == None` (modo transaccional) `staged` es una copia de
 /// `self.snapshot`, y planificador, bandas y expediciones se copian y se confirman al final; con `Some` (modo vivo)
 /// se trabaja sobre los originales. No asigna `self.snapshot`: lo hace el llamante.
 fn step_on(&mut self,staged:&mut NavigationSnapshot,had_route:Option<&[bool]>,undo:Option<&mut Option<VoyageUndo>>,towns:&mut [Town],merchants:&mut [Merchant],tick:u32,mut planner:Option<&mut p3_core::route::finder::RoutePlanner>,encounter:Option<EncounterOptions<'_>>,pirate:Option<PirateEncounterOptions<'_>>,ai:Option<&mut p3_core::ships::ai_dock::AiDockData>,mut absorption:Option<&mut crate::convoy_absorption::AbsorptionState>,mut expeditions:Option<&mut ExpeditionPlanning>)->Result<Option<p3_core::ships::battle::CommercialBattleStart>,String>{
  let live=had_route.is_some();
  let mut town_bytes=town_image(towns);let mut merchant_bytes=merchant_image(merchants);
  if town_bytes.len()!=towns.len()*TOWN_SIZE||merchant_bytes.len()!=merchants.len()*MERCHANT_SIZE{return Err("registros incompatibles".into())}
  let with_ai=ai.is_some();let convoy_bytes=staged.convoys.len();if with_ai{let cap=usize::from(u16::from_le_bytes([staged.manager[0xf6],staged.manager[0xf7]]));staged.convoys.resize(staged.convoys.len().max((cap+p3_core::ships::ai_depart::GROW_STEP)*0x3c),0)}let passive=staged.lists.docked.iter().copied().filter(|&id|{let at=usize::from(id)*SHIP_SIZE;let Some(image)=staged.ships.get(at..at+SHIP_SIZE)else{return false};let owner=usize::from(image[0]);let Some(merchant)=merchants.get(owner)else{return false};let state=u16::from_le_bytes([image[0x134],image[0x135]]);(with_ai&&planner.is_some())||state==1||(state==2&&with_ai&&merchant.u16(8)!=0)||(state==0&&if merchant.u16(8)==0{image[0x136]&1==0}else{with_ai||image[0x136]&0x20!=0})}).collect();
  let count=u16::from_le_bytes([staged.manager[0xf4],staged.manager[0xf5]]);let might_plan=tick&7==0&&std::iter::once(self.ship).chain(self.additional.iter().map(|v|v.ship)).any(|id|{let at=usize::from(id)*SHIP_SIZE;staged.ships.get(at+0x3a..at+0x3c).is_some_and(|b|u16::from_le_bytes([b[0],b[1]])<count)});let use_planner=might_plan||with_ai;let mut staged_planner=if !live&&use_planner{planner.as_deref().cloned()}else{None};
  let mut absorption_copy=if live{None}else{absorption.as_deref().cloned()};let mut expeditions_copy=if live{None}else{expeditions.as_deref().cloned()};
  let host_planner=if live{if use_planner{planner.as_deref_mut()}else{None}}else{staged_planner.as_mut()};
  let host_absorption=if live{absorption.as_deref_mut()}else{absorption_copy.as_mut()};let host_expeditions=if live{expeditions.as_deref_mut()}else{expeditions_copy.as_mut()};
  let mut host=Host{expeditions:host_expeditions,pending_expedition_task:None,absorption:host_absorption,encounter,pirate,battle:None,ship:self.ship,convoy:self.convoy,points:&self.points,additional:&self.additional,passive,current:self.ship,chasing:false,planner:host_planner,route_updates:Vec::new(),freed_routes:Vec::new(),convoy_updates:Vec::new(),route_book:staged.routes.clone(),ports:&self.ports,tick,events:Vec::new(),error:None,ai};
  let mut remove=RemoveWorld{manager:&mut staged.manager,ships:&mut staged.ships,captains:&mut staged.captains,convoys:&mut staged.convoys,merchants:&mut merchant_bytes,towns:&mut town_bytes,pool_limit:0,merchant_count:merchants.len() as u16,town_count:towns.len() as u32,ui_a:false,ui_b:false};let mut w=TickWorld{remove:&mut remove,time:tick,player:self.player};
  tick_docked(&mut w,tick as u8,&mut host).map_err(|_|"división naval inválida")?;tick_sea(&mut w,&mut host);if with_ai||self.convoy.is_some()||self.additional.iter().any(|v|v.convoy.is_some()){p3_core::ships::tick::tick_convoys(&mut w,tick as u8,&mut host).map_err(|e|format!("tick de convoy: {e:?}"))?;}if let Some(e)=host.error{return Err(e)}if host.pending_expedition_task.is_some(){return Err("tarea de expedición sin completar".into())}
  if with_ai{let cap=usize::from(u16::from_le_bytes([staged.manager[0xf6],staged.manager[0xf7]]));staged.convoys.truncate(convoy_bytes.max(cap*0x3c))}
  refresh_lists(staged)?;
  let updates=std::mem::take(&mut host.route_updates);let mut freed_routes=std::mem::take(&mut host.freed_routes);let events=std::mem::take(&mut host.events);let convoy_updates=std::mem::take(&mut host.convoy_updates);let battle=host.battle.take();drop(host);for i in 0..staged.routes.len(){let at=i*SHIP_SIZE+0x24;if (match had_route{Some(h)=>h[i],None=>self.snapshot.ships[at..at+4]!=[0;4]})&&staged.ships[at..at+4]==[0;4]&&!freed_routes.contains(&(i as u16)){freed_routes.push(i as u16)}}for (id,_,_) in &updates{if usize::from(*id)>=staged.routes.len(){return Err("ruta de barco inexistente".into())}}
  if freed_routes.iter().any(|&id|{let index=usize::from(id);index>=staged.routes.len()||index*SHIP_SIZE+0x28>staged.ships.len()}){return Err("ruta liberada fuera de la flota".into())}
  if let Some(undo)=undo{if !updates.is_empty()||!freed_routes.is_empty()||!convoy_updates.is_empty(){*undo=Some(VoyageUndo{routes:staged.routes.clone(),points:self.points.clone(),additional:self.additional.clone(),convoy:self.convoy})}}
  for (id,points,saved) in updates{staged.routes[usize::from(id)]=Some(saved);if id==self.ship{self.points=points}else if let Some(v)=self.additional.iter_mut().find(|v|v.ship==id){v.points=points}else{self.additional.push(ExtraVoyage{ship:id,convoy:None,points})}}
  for id in freed_routes{let index=usize::from(id);let at=index*SHIP_SIZE+0x24;if staged.ships[at..at+4]==[0;4]{staged.routes[index]=Some(p3_save::ships::SavedRoute::None);if id==self.ship{self.points.clear()}else if let Some(v)=self.additional.iter_mut().find(|v|v.ship==id){v.points.clear()}}}
  for (cv,leader) in convoy_updates{let at=usize::from(cv)*0x3c;let owner=u32::from(staged.convoys[at]);if !active_convoy(staged,cv,owner)||staged.convoys[at+0x10..at+0x12]!=leader.to_le_bytes(){continue}if self.convoy==Some(cv)&&self.ship!=leader{self.convoy=None}for v in &mut self.additional{if v.convoy==Some(cv)&&v.ship!=leader{v.convoy=None}}if self.ship==leader{self.convoy=Some(cv)}else if let Some(v)=self.additional.iter_mut().find(|v|v.ship==leader){v.convoy=Some(cv)}else{self.additional.push(ExtraVoyage{ship:leader,convoy:Some(cv),points:Vec::new()})}}
  for (t,src) in towns.iter_mut().zip(town_bytes.chunks_exact(TOWN_SIZE)){t.0.copy_from_slice(src)}for (m,src) in merchants.iter_mut().zip(merchant_bytes.chunks_exact(MERCHANT_SIZE)){m.bytes.copy_from_slice(src)}if let (Some(dst),Some(src))=(planner,staged_planner){*dst=src}self.events.extend(events);if let (Some(dst),Some(src))=(absorption,absorption_copy){*dst=src}if let (Some(dst),Some(src))=(expeditions,expeditions_copy){*dst=src}Ok(battle)
 }
}
