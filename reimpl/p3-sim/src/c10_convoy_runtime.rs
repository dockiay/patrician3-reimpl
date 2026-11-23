//! Publicación atómica de las operaciones de expedición en la sesión C10.
use crate::{Sim,navigation::ExtraVoyage};
use p3_core::{pump::Operation,route::finder::RouteOut,ships::{remove::{RemoveWorld,SHIP_SIZE,MERCHANT_SIZE,TOWN_SIZE},tick::TickWorld}};

impl Sim {
 pub(crate) fn apply_convoy_expedition_operation(&mut self,op:Operation)->Result<(),String>{
  let nav=self.host.navigation.as_ref().ok_or("flota de expedición ausente")?;
  let before=nav.snapshot.clone();let mut staged=nav.clone();
  if op.0[0]==0x1f {
   crate::c10_convoy_operations::set_waypoint(&mut staged.snapshot,&self.economy.world.towns,op)?;
   self.host.navigation=Some(staged);
   self.host.publish_navigation_delta(&before,&mut self.economy.world,&mut self.economy.cargo_ships);
   return Ok(())
  }
  let Some(plan)=crate::c10_convoy_operations::prepare_start(&staged.snapshot,&self.economy.world.towns,op)? else{return Ok(())};
  let count=usize::from(self.economy.world.merchant_count);
  if self.rep_merchants.len()<count||self.economy.merchants.len()<count||self.economy.world.merchants.len()<count{return Err("comerciantes de expedición incompletos".into())}
  let mut merchants=self.rep_merchants.clone();let mut countdown=self.idle.clock.expedition_countdown;
  let mut planner=self.host.navigation_planner.clone().ok_or("planificador de expedición ausente")?;
  let at=usize::from(plan.leader)*SHIP_SIZE;let ship=&staged.snapshot.ships[at..at+SHIP_SIZE];
  let start=(i16::from_le_bytes([ship[0x1e],ship[0x1f]]),i16::from_le_bytes([ship[0x22],ship[0x23]]));
  let end=(plan.move_operation.0[3] as i16,plan.move_operation.0[4] as i16);
  let result=planner.find_route(&[start,end]);
  let points=match result.route{RouteOut::Built(v)=>v,RouteOut::Shared(_)=>return Err("salida de expedición requiere ruta propia a punto externo".into()),RouteOut::Unset=>return Err("ruta de expedición no disponible".into())};
  let mut town_bytes=self.economy.world.towns.iter().flat_map(|t|t.0.iter().copied()).collect::<Vec<_>>();
  let mut merchant_bytes=merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();
  let snapshot=&mut staged.snapshot;
  let mut remove=RemoveWorld{manager:&mut snapshot.manager,ships:&mut snapshot.ships,captains:&mut snapshot.captains,convoys:&mut snapshot.convoys,merchants:&mut merchant_bytes,towns:&mut town_bytes,pool_limit:0,merchant_count:count as u16,town_count:self.economy.world.towns.len() as u32,ui_a:false,ui_b:false};
  let mut world=TickWorld{remove:&mut remove,time:self.idle.clock.tick,player:self.economy.world.local_merchant};
  p3_core::ships::departure::reroute_convoy_to_point(&mut world,plan.leader,&points,end,0x10000+u32::from(plan.leader)*16)?;
  for (dst,src) in merchants.iter_mut().zip(merchant_bytes.chunks_exact(MERCHANT_SIZE)){dst.bytes.copy_from_slice(src)}
  crate::c10_convoy_operations::finish_start(&plan,&mut staged.snapshot,&mut merchants,&mut countdown)?;
  *staged.snapshot.routes.get_mut(usize::from(plan.leader)).ok_or("tabla de rutas incompleta")?=Some(p3_save::ships::SavedRoute::Owned(points.clone()));
  if staged.ship==plan.leader{staged.points=points;staged.convoy=Some(plan.convoy)}
  else if let Some(v)=staged.additional.iter_mut().find(|v|v.ship==plan.leader){v.points=points;v.convoy=Some(plan.convoy)}
  else{staged.additional.push(ExtraVoyage{ship:plan.leader,convoy:Some(plan.convoy),points})}
  // Ninguna operación falible queda tras publicar las vistas de la transacción.
  for (dst,src) in self.economy.world.towns.iter_mut().zip(town_bytes.chunks_exact(TOWN_SIZE)){dst.0.copy_from_slice(src)}
  for i in 0..count{self.economy.world.merchants[i].gold=merchants[i].i32(0);self.economy.merchants[i]=merchants[i].clone()}
  self.rep_merchants=merchants;self.idle.clock.expedition_countdown=countdown;
  self.host.navigation=Some(staged);self.host.navigation_planner=Some(planner);
  self.host.publish_navigation_delta(&before,&mut self.economy.world,&mut self.economy.cargo_ships);Ok(())
 }
}
