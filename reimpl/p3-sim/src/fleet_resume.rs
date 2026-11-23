//! Reconstrucción explícita de la flota desde S02 para el reloj sin pantalla.
use crate::{navigation::LiveVoyage,Sim};
use p3_core::route::finder::RouteOut;
use p3_save::{model::SaveModel,ships::SavedRoute};

impl Sim {
 /// Reanuda barcos en navegación, salida o atraque y crea la imagen de las listas.
 /// Requiere recursos navales ya configurados; no reconstruye una batalla activa.
 /// No modifica la sesión si una ruta o un estado no se pueden reconstruir.
 pub fn resume_saved_fleet(&mut self,model:&SaveModel,player:u32)->Result<usize,String>{
  if self.host.navigation.is_some() {return Err("ya hay una flota activa".into())}
  if self.host.detached_shipyard.is_some(){return Err("guardar y recargar la transacción del astillero antes de reanudar la flota".into())}
  if model.sea_battles.first().copied().unwrap_or(0)!=0 {return Err("batalla activa: reconstrucción y tick aún no disponibles".into())}
  if player>=self.economy.world.merchant_count as u32{return Err("jugador naval inválido".into())}
  let resources=self.host.expedition_resources.as_ref().ok_or("faltan recursos de puertos y flota".to_string())?;
  let ports=resources.ports.iter().map(|p|p.map(|(x,y)|(x as i16,y as i16)).ok_or("puerto ausente".to_string())).collect::<Result<Vec<_>,_>>()?;
  if ports.len()!=self.economy.world.towns.len(){return Err("puertos y ciudades incompatibles".into())}
  let count=self.economy.world.towns.len();if model.world.matrix.len()!=count*count{return Err("matriz de distancias incompatible".into())}
  let block=model.ships.write();let ships=p3_save::ships::Ships::parse(&block).map_err(|e|format!("barcos: {e:?}"))?;
  let snapshot=ships.navigation_snapshot().map_err(str::to_string)?;
  if snapshot.ships.len()!=self.economy.cargo_ships.len()*0x180||self.host.expedition_ship_images.len()!=ships.ship_slots(){return Err("flota y sesión incompatibles".into())}
  for i in 0..ships.ship_slots(){let saved=ships.ship_image(i);if saved.as_deref()!=self.host.expedition_original_ship_images.get(i).and_then(Option::as_deref){return Err(format!("imagen del barco {i} desincronizada"))}let mut live=saved;if let Some(img)=&mut live{p3_core::ships::cargo::recompute_load(img);}if live.as_deref()!=self.host.expedition_ship_images[i].as_deref(){return Err(format!("imagen viva del barco {i} desincronizada"))}}
  for (i,cargo) in self.economy.cargo_ships.iter().enumerate(){let b=&snapshot.ships[i*0x180..(i+1)*0x180];if snapshot.routes[i].is_some()&&(cargo.town!=b[0x39]||(0..24).any(|w|cargo.stock[w]!=i32::from_le_bytes(b[0x54+4*w..0x58+4*w].try_into().unwrap()))){return Err(format!("carga del barco {i} desincronizada"))}}
  let mut planner=self.host.foundation_navigation.as_ref().ok_or("faltan datos del planificador naval")?.build_planner(&self.host.expedition_town_ids,100_000)?;
  let mut staged=LiveVoyage::idle(snapshot,ports,player);let mut resumed=0;
  for i in 0..ships.ship_slots(){
   let Some(image)=ships.ship_image(i) else{continue};let state=u16::from_le_bytes([image[0x134],image[0x135]]);
   if !matches!(state,1|2|3|0x0f){continue}
   let cid=usize::from(u16::from_le_bytes([image[8],image[9]]));
   if cid<ships.convoys.len(){let cv=ships.convoy_image(cid);if usize::from(u16::from_le_bytes([cv[0x10],cv[0x11]]))!=i{continue}}
   let route=ships.saved_route(i).map_err(str::to_string)?;
   if state==1 && cid>=ships.convoys.len(){continue}
   if state==2 && matches!(route,SavedRoute::None) && staged.snapshot.lists.docked.contains(&(i as u16)) && self.rep_merchants.get(usize::from(image[0])).is_some_and(|m|m.u16(8)!=0){continue}
   let shared=if let SavedRoute::Shared{origin,destination}=route {
    let start=*staged.ports.get(usize::from(origin)).ok_or("origen de ruta compartida inválido")?;let end=*staged.ports.get(usize::from(destination)).ok_or("destino de ruta compartida inválido")?;
    match planner.find_route(&[start,end]).route{RouteOut::Shared(id)=>planner.routes.get(id).cloned(),RouteOut::Built(points)=>Some(points),_=>return Err(format!("ruta compartida del barco {i} no disponible"))}
   }else{None};
   let owner=u32::from(image[0]);let id=u16::try_from(i).map_err(|_|"identificador de barco excesivo")?;
   if staged.ship==u16::MAX {let mut first=LiveVoyage::resume(staged.snapshot.clone(),id,staged.ports.clone(),owner,shared).map_err(|e|format!("barco {i}: {e}"))?;first.player=player;staged=first;}
   else {staged.add_resumed_for_owner(id,owner,shared).map_err(|e|format!("barco {i}: {e}"))?}
   resumed+=1;
  }
  self.host.navigation_ai=Some(p3_core::ships::ai_dock::AiDockData{distances:model.world.matrix.clone(),size:count as u32,difficulty:self.commercial_sell_factor,pirate_difficulty:self.host.pirate_difficulty,convoy_plan:None,offices:vec![]});
  self.host.navigation_planner=Some(planner);self.host.navigation=Some(staged);self.economy.world.local_merchant=player;if let Some(resources)=self.host.expedition_resources.as_mut(){resources.player=player as u16}Ok(resumed)
 }
}
