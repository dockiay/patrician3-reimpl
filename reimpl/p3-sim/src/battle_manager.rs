//! Listas temporales y petición de vista de `SeaBattles` (0x006127C0).
//! No ejecuta el paso de las batallas; los efectos salen al anfitrión.

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ManagerEffect { Sync, FreeGroups, CloseView, EnterBattle(u8) }

#[derive(Debug,Clone,Copy)]
pub struct ViewState { pub busy:bool, pub current_scene_matches:bool }

/// Reconstruye las listas de un gestor guardado como `0x00611AE0` y
/// `0x00612A20`: capacidad en bloques de 100, identificadores repetidos se
/// actualizan y solo se lee el byte bajo del segundo valor. Los tres bytes
/// opacos de una fila recién reservada se inicializan a cero en este anfitrión.
pub fn groups_from_save(frame:&p3_save::tail::CurrentSeaBattles<'_>)->Result<Vec<Vec<[u8;8]>>,&'static str>{
 let mut groups:Vec<Vec<[u8;8]>>=Vec::new();
 for (j,entry) in frame.pairs.iter().enumerate(){
  let index=usize::try_from(entry.index).map_err(|_|"índice de lista excesivo")?;
  if index>=100_000{return Err("índice de lista excesivo")}
  if index>=groups.len(){groups.resize_with((index/100+1)*100,Vec::new)}
  for (id,raw) in frame.entries(j).ok_or("lista de batalla ausente")?{
   let group=&mut groups[index];if let Some(existing)=group.iter_mut().find(|v|v[..4]==id.to_le_bytes()){existing[4]=raw as u8}else{let mut row=[0;8];row[..4].copy_from_slice(&id.to_le_bytes());row[4]=raw as u8;group.push(row)}
  }
 }
 Ok(groups)
}

/// Convierte las filas nativas al bloque que escribe el serializador S05.
pub fn groups_to_save(groups:&[Vec<[u8;8]>])->Vec<Vec<(u32,u32)>>{
 groups.iter().map(|g|g.iter().map(|v|(u32::from_le_bytes(v[..4].try_into().unwrap()),u32::from_le_bytes(v[4..].try_into().unwrap()))).collect()).collect()
}

/// `0x00612BD0`: registra en ambas direcciones los encuentros entre los
/// miembros actuales de dos convoyes. Conserva los bytes opacos de cada fila.
pub fn register_convoy_pairs(manager:&mut [u8],groups:&mut Vec<Vec<[u8;8]>>,ships:&[u8],convoys:&[u8],first:u16,second:u16)->Result<(),&'static str>{
 if manager.len()<0xc38||ships.len()%0x180!=0||convoys.len()%0x3c!=0{return Err("gestor o flota truncada")}
 let count=u32::from_le_bytes(manager[0xc34..0xc38].try_into().unwrap())as usize;if count!=groups.len()||count!=0&&manager[0xc30..0xc34]==[0;4]{return Err("tabla de listas incompatible")}
 let r16=|b:&[u8],at:usize|u16::from_le_bytes([b[at],b[at+1]]);let chain=|cv:u16|->Result<Vec<u16>,&'static str>{let at=usize::from(cv)*0x3c;if at+0x3c>convoys.len(){return Err("convoy ausente")}let mut id=r16(convoys,at+0xa);let mut seen=vec![false;ships.len()/0x180];let mut out=Vec::new();while usize::from(id)<seen.len(){let i=usize::from(id);if seen[i]{return Err("convoy cíclico")}seen[i]=true;out.push(id);id=r16(ships,i*0x180+6)}Ok(out)};
 let left=chain(first)?;let right=chain(second)?;let mut staged=groups.clone();
 fn upsert(groups:&mut Vec<Vec<[u8;8]>>,a:u16,b:u16)->Result<(),&'static str>{let i=usize::from(a);if i>=groups.len(){let n=(i/100+1).checked_mul(100).ok_or("tabla excesiva")?;if n>100_000{return Err("tabla excesiva")}groups.resize_with(n,Vec::new)}let id=u32::from(b).to_le_bytes();if let Some(row)=groups[i].iter_mut().find(|v|v[..4]==id){row[4]=0xff}else{let mut row=[0u8;8];row[..4].copy_from_slice(&id);row[4]=0xff;groups[i].push(row)}Ok(())}
 for &a in &left{for &b in &right{upsert(&mut staged,a,b)?;upsert(&mut staged,b,a)?}}
 if !staged.is_empty()&&count==0{manager[0xc30..0xc34].copy_from_slice(&1u32.to_le_bytes())}manager[0xc34..0xc38].copy_from_slice(&(staged.len()as u32).to_le_bytes());*groups=staged;Ok(())
}

/// Estado externo que el destructor del combate modifica junto con la flota.
pub struct DestroyWorld<'a>{
 pub manager:&'a mut [u8],pub groups:&'a mut Vec<Vec<[u8;8]>>,
 pub navigation:&'a mut [u8],pub ships:&'a mut [u8],pub convoys:&'a mut [u8],pub captains:&'a mut Vec<u8>,pub routes:&'a mut [Option<p3_save::ships::SavedRoute>],
 pub pirate_notices:&'a mut Vec<p3_core::ships::return_convoy::PirateCaptainNotice>,pub merchants:&'a mut [u8],pub merchant_groups:&'a mut [p3_save::world::MerchantShipGroups],pub band_heads:&'a mut [Option<u16>;5],pub towns:&'a mut [u8],pub scheduler:&'a mut p3_core::scheduler::Scheduler,
 pub tick:u32,pub player:u32,pub task_limit:u16,pub has_hanse_task:bool,
 /// Listas de bajas de las tareas 0x34 (`tarea+8`) por ranura, cabeza primero.
 pub hunt_losses:&'a mut std::collections::BTreeMap<usize,Vec<p3_core::ships::return_convoy::HuntLoss>>,
 /// Pool de paradas 0x6DD728 (S05): 0x4D4DD0(ranura, 0) de los hundidos IA de la caza.
 pub route_stops:&'a mut Option<crate::route_stop_runtime::RouteStopRuntime>,
 /// Recursos de 0x5097C0(1) para el líder nuevo de un convoy NPC con el líder hundido.
 pub captain:Option<p3_core::economy::pirate_group_callbacks::CaptainResources>,
}

/// Ruta sin vista de 0x00603BC0: registra encuentros, programa la posible
/// investigación y devuelve convoyes. Se confirma todo junto; combate
/// encadenado, vista, retornos que necesiten ruta y bajas no cubiertas se rechazan.
pub fn destroy_resolved(battle:&mut [u8],w:&mut DestroyWorld<'_>)->Result<Vec<u16>,&'static str>{
 use p3_core::ships::{finish_report::{investigation_after_battle,schedule_investigation},return_convoy::{recount_incoming,return_preserved}};
 if battle.len()<0x1e98||w.manager.len()<0xc38||w.navigation.len()<0xf8||w.merchants.len()%0x650!=0||w.towns.len()%0x9f8!=0{return Err("estado de destrucción truncado")}
 let r16=|b:&[u8],i:usize|u16::from_le_bytes([b[i],b[i+1]]);let slot=usize::from(r16(battle,0x662));if slot>=256||w.manager[slot*4..slot*4+4]==[0;4]{return Err("batalla no registrada")}
 if battle[0x5c]==0||r16(battle,0x15c)!=0{return Err("batalla aún no resuelta")}
 let ship_count=r16(w.navigation,0xf4);if r16(battle,0x130c)<ship_count{return Err("batalla encadenada pendiente")}
 if battle[0x12ea]!=0||(0..usize::from(battle[0x558])).any(|i|battle[0x5d+i]==0){return Err("interfaz o reloj humano pendientes")}
 let mut obj=battle.to_vec();let mut bm=w.manager.to_vec();let mut groups=w.groups.clone();let mut nav=w.navigation.to_vec();let mut ships=w.ships.to_vec();let mut convoys=w.convoys.to_vec();let mut captains=w.captains.to_vec();let old_captains=captains.clone();let old_ships=ships.clone();let mut routes=w.routes.to_vec();if routes.len()!=ships.len()/0x180{return Err("rutas de destrucción truncadas")}let mut merchants=w.merchants.to_vec();let mut merchant_groups=w.merchant_groups.to_vec();let mut band_heads=*w.band_heads;if merchant_groups.len()!=merchants.len()/0x650{return Err("grupos de comerciante truncados")}let mut towns=w.towns.to_vec();let mut scheduler=w.scheduler.clone();let mut pirate_notices=w.pirate_notices.clone();let mut refresh=Vec::new();let mut route_stops=w.route_stops.clone();let mut hunt_losses=w.hunt_losses.clone();
 bm[slot*4..slot*4+4].fill(0);while bm[0x404]!=0{let i=usize::from(bm[0x404]-1);if bm[i*4..i*4+4]!=[0;4]{break}bm[0x404]-=1}
 let a=r16(&obj,0x65e);let b=r16(&obj,0x660);if a!=u16::MAX&&b!=u16::MAX{
  register_convoy_pairs(&mut bm,&mut groups,&ships,&convoys,a,b)?;
  let(seed,event)=investigation_after_battle(&obj,ship_count)?;obj[0xadc..0xae0].copy_from_slice(&seed.to_le_bytes());
  if let Some(event)=event{let records=merchants.chunks_exact(0x650).map(|bytes|p3_core::ai::reputation::Merchant{bytes:bytes.to_vec()}).collect::<Vec<_>>();schedule_investigation(event,w.tick,&ships,&convoys,&records,&mut scheduler)?;}
 }
 if b!=u16::MAX{let i=usize::from(b)*0x3c;if i+0x14>convoys.len(){return Err("convoy ausente")}if r16(&convoys,i+0x12)==9{return Err("desvío de convoy especial pendiente")}}
 let count=merchants.len()/0x650;if count>u16::MAX as usize{return Err("demasiados comerciantes")};
 let returns=[(a,if b==u16::MAX{0xff}else{r16(&obj,0x35c)as u8}),(b,if a==u16::MAX{0xff}else{r16(&obj,0x35a)as u8})];
 for(id,credit)in returns{if id==u16::MAX{continue}let at=usize::from(id)*0x3c;if at+0x3c>convoys.len(){return Err("convoy ausente")}
  // 0x50C02F..0x50C049 + 0x50C5A6..0x50C8E5: NPC convoy left without ships. ensure_leader fails
  // (leader 0xFFFF), state 0x12/0xA00 and remove_convoy 0x50A3D0; no 0x503B00 check on this path.
  if usize::from(convoys[at])>=count&&r16(&convoys,at+0xa)>=ship_count{
   convoys[at+0x14]&=!0x20;convoys[at+0x10..at+0x12].fill(0xff);convoys[at+0x16..at+0x18].copy_from_slice(&0x0a00u16.to_le_bytes());
   release_empty_convoy(&mut nav,&mut convoys,id)?;
   if usize::from(credit)<count{recount_incoming(&nav,&ships,&convoys,&mut towns)?}
   continue;
  }
  let npc_all_sunk=usize::from(convoys[at])>=count&&{let mut id=r16(&convoys,at+0xa);let mut seen=vec![false;usize::from(ship_count)];let mut all=true;while id<ship_count&&!seen[usize::from(id)]{seen[usize::from(id)]=true;let o=usize::from(id)*0x180;if i32::from_le_bytes(ships[o+0x18..o+0x1c].try_into().unwrap())>=1{all=false}id=r16(&ships,o+6)}all};
  let dissolve=(usize::from(convoys[at])<count&&(convoys[at+0x14]&0x10!=0||r16(&convoys,at+0xa)>=ship_count))||npc_all_sunk;
  // 0x503B00: solo la tarea 0x34 en fase >= 7 que apunta al convoy.
  let task=r16(&convoys,at+2);let hanse=w.has_hanse_task||scheduler.records.get(usize::from(task)).filter(|_|task<scheduler.capacity).is_some_and(|r|r.kind==0x34&&r.u8_at(0x15)>=7&&u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)])==id);
  // 0x50C0EE: NPC convoy (pirates) returning with sunk ships.
  let npc_sunk=!npc_all_sunk&&usize::from(convoys[at])>=count&&{let mut id=r16(&convoys,at+0xa);let mut seen=vec![false;usize::from(ship_count)];let mut any=false;while id<ship_count&&!seen[usize::from(id)]{seen[usize::from(id)]=true;let o=usize::from(id)*0x180;if i32::from_le_bytes(ships[o+0x18..o+0x1c].try_into().unwrap())<1{any=true}id=r16(&ships,o+6)}any};
  let recalc=if npc_sunk{let effect=p3_core::ships::return_convoy::return_npc_with_losses(&mut nav,&mut ships,&mut convoys,&mut merchants,id,credit,w.task_limit,&mut captains,&mut towns,&mut band_heads,w.captain)?;for &ship in &effect.sunk_ships{scheduler.report_captured_ship(u32::from(ship),u32::from(credit),count as u16,&old_ships,&old_captains,r16(&nav,0xf2))?;}for &ship in &effect.removed_routes{if !matches!(routes.get(usize::from(ship)),Some(Some(p3_save::ships::SavedRoute::Owned(_)|p3_save::ships::SavedRoute::Shared{..}))){return Err("descriptor de ruta de baja ausente")}if r16(&ships,usize::from(ship)*0x180+0x134)!=0xff{routes[usize::from(ship)]=Some(p3_save::ships::SavedRoute::None)}}effect.recount_incoming}else if hanse&&!w.has_hanse_task&&usize::from(convoys[at])<count{
   // 0x50C06F: 0x503B00 true. The merchant convoy is not dissolved: state 0x16 and hunt task phase 0xA.
   let record=scheduler.records.get_mut(usize::from(task)).ok_or("tarea de expedición ausente")?;let hunt=p3_core::ships::return_convoy::return_hunt_expedition(&mut nav,&mut ships,&mut convoys,&mut merchants,id,credit,w.player,w.task_limit,record,&mut captains,&mut towns,&mut merchant_groups)?;
   // 0x50C41A: each loss node is put in front of the list at task+8 (head = last loss).
   if !hunt.losses.is_empty(){prepend_hunt_losses(record,usize::from(task),&mut hunt_losses,&hunt.losses);}
   // clear_route 0x516840 left these ships without route: drop the descriptor (owned or shared).
   for &ship in &hunt.removed_routes{if !matches!(routes.get(usize::from(ship)),Some(Some(p3_save::ships::SavedRoute::Owned(_)|p3_save::ships::SavedRoute::Shared{..}))){return Err("descriptor de ruta de baja ausente")}if r16(&ships,usize::from(ship)*0x180+0x134)!=0xff{routes[usize::from(ship)]=Some(p3_save::ships::SavedRoute::None)}}
   // remove_ship 0x507FB0 freed these stop-pool slots (0x4D4DD0 with propagation 0).
   for &slot in &hunt.released_stops{route_stops.as_mut().ok_or("depósito de paradas ausente")?.release(slot).map_err(|_|"depósito de paradas inválido")?}
   let effect=hunt.effects;if let Some(id)=effect.refresh_ship{refresh.push(id)}effect.recount_incoming
  }else if dissolve{let effect=p3_core::ships::return_convoy::return_dissolved_with_bands(&mut nav,&mut ships,&mut convoys,&mut merchants,id,credit,w.player,w.task_limit,hanse,&mut captains,&mut towns,u16::MAX,&mut merchant_groups,&mut band_heads)?;pirate_notices.extend(effect.pirate_notices.iter().copied());for &ship in &effect.sunk_ships{scheduler.report_captured_ship(u32::from(ship),u32::from(credit),count as u16,&old_ships,&old_captains,r16(&nav,0xf2))?;}for &ship in &effect.removed_routes{if !matches!(routes.get(usize::from(ship)),Some(Some(p3_save::ships::SavedRoute::Owned(_)|p3_save::ships::SavedRoute::Shared{..}))){return Err("descriptor de ruta de baja ausente")}}refresh.extend(effect.refresh_ships);effect.recount_incoming}else{let effect=return_preserved(&mut nav,&mut ships,&mut convoys,id,count as u16,credit,w.player,w.task_limit,hanse)?;if let Some(id)=effect.refresh_ship{refresh.push(id)}effect.recount_incoming};
  if recalc{recount_incoming(&nav,&ships,&convoys,&mut towns)?}
 }
 for(i,(old,new))in old_ships.chunks_exact(0x180).zip(ships.chunks_exact(0x180)).enumerate(){if r16(old,0x134)<0x17&&r16(new,0x134)==0xff{routes[i]=None;}}
 battle.copy_from_slice(&obj);w.manager.copy_from_slice(&bm);*w.groups=groups;w.navigation.copy_from_slice(&nav);w.ships.copy_from_slice(&ships);w.convoys.copy_from_slice(&convoys);*w.captains=captains;w.routes.clone_from_slice(&routes);w.merchants.copy_from_slice(&merchants);w.merchant_groups.clone_from_slice(&merchant_groups);*w.band_heads=band_heads;w.towns.copy_from_slice(&towns);*w.scheduler=scheduler;*w.pirate_notices=pirate_notices;*w.hunt_losses=hunt_losses;*w.route_stops=route_stops;Ok(refresh)
}

/// Valor no nulo que p3-sim deja en `tarea+8` mientras la tarea tiene lista; el original guarda
/// ahí el puntero de cabeza de la lista, que el lector sustituye al cargar. El comparador general
/// conserva ese campo; solo un banco dedicado con la lista validada puede reubicarlo.
pub const HUNT_LIST_HANDLE:u32=0x7d00_0000;

/// `operator new(8)` + 0x4F3980 + 0x4F3AD0 con hijo = `tarea+8`, y `tarea+8` = nodo nuevo, por cada baja en orden.
pub fn prepend_hunt_losses(task:&mut p3_core::scheduler::TaskRecord,slot:usize,lists:&mut std::collections::BTreeMap<usize,Vec<p3_core::ships::return_convoy::HuntLoss>>,losses:&[p3_core::ships::return_convoy::HuntLoss]){
 if losses.is_empty(){return}
 let list=lists.entry(slot).or_default();if task.u32_at(8)==0{list.clear()}
 for &loss in losses{list.insert(0,loss)}
 task.set_u32_at(8,HUNT_LIST_HANDLE|slot as u32);
}

/// List part of `remove_convoy` 0x0050A3D0 for a convoy without ships: unlink from `manager+0xEC`,
/// push on the free list `+0xE4` unless already there, state 0xFF.
fn release_empty_convoy(nav:&mut [u8],convoys:&mut [u8],id:u16)->Result<(),&'static str>{
 let r16=|b:&[u8],i:usize|u16::from_le_bytes([b[i],b[i+1]]);
 let total=r16(nav,0xf6);let b=usize::from(id)*0x3c;let mut cur=r16(nav,0xec);let mut slot:Option<usize>=None;let mut seen=vec![false;usize::from(total)];
 while cur<total{let i=usize::from(cur);if seen[i]{return Err("lista de convoyes cíclica")}seen[i]=true;if cur==id{let next=[convoys[b+8],convoys[b+9]];match slot{None=>nav[0xec..0xee].copy_from_slice(&next),Some(p)=>convoys[p*0x3c+8..p*0x3c+10].copy_from_slice(&next)}break}slot=Some(i);cur=r16(convoys,i*0x3c+8)}
 let free=r16(nav,0xe4);let mut f=free;seen.fill(false);
 while f<total{let i=usize::from(f);if seen[i]{return Err("lista libre de convoyes cíclica")}seen[i]=true;if f==id{convoys[b+0x12..b+0x14].copy_from_slice(&0xffu16.to_le_bytes());return Ok(())}f=r16(convoys,i*0x3c+8)}
 convoys[b+8..b+10].copy_from_slice(&free.to_le_bytes());nav[0xe4..0xe6].copy_from_slice(&id.to_le_bytes());convoys[b+0x12..b+0x14].copy_from_slice(&0xffu16.to_le_bytes());Ok(())
}

/// Estado vivo del gestor sin batallas activas ni interfaz pendiente.
pub struct Runtime {
 pub manager:Vec<u8>,
 pub groups:Vec<Vec<[u8;8]>>,
 pub initial:Vec<Vec<[u8;8]>>,
 pub original_block:Vec<u8>,
 pub error:Option<String>,
 pub rewrite:bool,
}
impl Runtime {
 pub fn from_block(block:&[u8])->Result<Self,String>{
  let frame=p3_save::tail::CurrentSeaBattles::parse(block).map_err(|e|format!("gestor de batallas: {e:?}"))?;
  if !frame.objects.is_empty(){return Err("gestor con batalla activa no admitido".into())}
  let groups=groups_from_save(&frame).map_err(str::to_string)?;let mut manager=vec![0;0xc40];
  if !groups.is_empty(){manager[0xc30..0xc34].copy_from_slice(&1u32.to_le_bytes());manager[0xc34..0xc38].copy_from_slice(&(groups.len()as u32).to_le_bytes())}
  Ok(Self{manager,initial:groups.clone(),groups,original_block:block.to_vec(),error:None,rewrite:false})
 }
 pub fn persist(&self,model:&mut p3_save::model::SaveModel)->Result<(),String>{
  if let Some(e)=&self.error{return Err(format!("gestor de batalla: {e}"))}
  if model.sea_battles!=self.original_block{return Err("gestor de batallas modificado por otro sistema".into())}
  if self.rewrite||self.groups!=self.initial {model.set_current_basic_sea_battles(&[],&groups_to_save(&self.groups))?}
  Ok(())
 }
}

/// Las filas corresponden a los registros de ocho bytes apuntados por cada
/// entrada de la tabla +0xC30. El contador es el byte +4; los demás bytes son
/// opacos y se conservan, también cuando el original sustituye una fila.
pub fn tick(manager:&mut [u8],groups:&mut Vec<Vec<[u8;8]>>,advance:bool,view:ViewState)->Result<Vec<ManagerEffect>,&'static str>{
 if manager.len()<0xc38{return Err("gestor de batallas truncado")}
 let count=u32::from_le_bytes(manager[0xc34..0xc38].try_into().unwrap()) as usize;
 if count!=groups.len(){return Err("tabla de listas incompatible")}
 if count!=0&&manager[0xc30..0xc34]==[0;4]{return Err("tabla de listas ausente")}
 let mut staged=manager.to_vec();let mut lists=groups.clone();let mut effects=vec![ManagerEffect::Sync];
 if advance {
  let mut retained=false;
  for group in &mut lists {
   let mut i=0;
   while i<group.len(){
    if group[i][4]>1{group[i][4]-=1;retained=true;i+=1}else{
     let last=*group.last().expect("lista no vacía");group[i][..4].copy_from_slice(&last[..4]);group[i][4]=last[4];group.pop();
    }
   }
  }
  if count!=0&&!retained {lists.clear();staged[0xc30..0xc38].fill(0);effects.push(ManagerEffect::FreeGroups)}
 }
 if staged[0x42c]!=0 {
  let slot=staged[0x42d];let at=usize::from(slot)*4;let present=staged[at..at+4]!=[0;4];
  if !view.busy || !present {
   if view.busy {effects.push(ManagerEffect::CloseView)} else if view.current_scene_matches&&present {effects.push(ManagerEffect::EnterBattle(slot))}
   staged[0x42c]=0;staged[0x3fd]=0;
  }
 }
 manager.copy_from_slice(&staged);*groups=lists;Ok(effects)
}
