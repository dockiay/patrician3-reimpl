//! Reconciliación S05 de toda la escena, desde antes de abrir hasta destruirla.
//! El estado activo nunca se guarda: solo se usa para acreditar ranura y origen.
use crate::pirate_empty_scene_publish::{self, LiveState};
use p3_save::model::SaveModel;

/// Instantánea capturada antes de abrir la escena, tras sincronizar la sesión.
/// `model` incluye el pool y todos los bloques originales, no un guardado
/// intermedio con una escena urbana activa.
pub struct Baseline { pub model: SaveModel, pub live: LiveState, pub groups: Vec<Vec<[u8;8]>> }

fn merge_bytes(old:&[u8],new:&[u8],current:&[u8],name:&str)->Result<Vec<u8>,String>{
    if old==new{return Ok(current.to_vec())}
    if current==old || current==new{return Ok(new.to_vec())}
    Err(format!("{name}: contenido modificado por otro sistema"))
}

/// Concilia tramos serializados conocidos como unidades; los bytes opacos que
/// cambian se tratan conjuntamente, para no aceptar palabras mezcladas.
fn merge_fields(old:&[u8],new:&[u8],current:&[u8],ranges:&[(usize,usize)],name:&str)->Result<Vec<u8>,String>{
    if old.len()!=new.len()||old.len()!=current.len(){return merge_bytes(old,new,current,name)}
    let mut out=current.to_vec();let mut covered=vec![false;old.len()];
    for &(start,len) in ranges {let end=start.checked_add(len).ok_or("tramo excesivo")?;if end>old.len()||covered[start..end].iter().any(|v|*v){return Err(format!("{name}: tramo inválido"))}
        covered[start..end].fill(true);
        if old[start..end]!=new[start..end]{
            if current[start..end]!=old[start..end]&&current[start..end]!=new[start..end]{return Err(format!("{name}: conflicto de campo +0x{start:X}"))}
            out[start..end].copy_from_slice(&new[start..end]);
        }
    }
    if (0..old.len()).any(|i|!covered[i]&&old[i]!=new[i]){
        if (0..old.len()).any(|i|!covered[i]&&current[i]!=old[i]) && (0..old.len()).any(|i|!covered[i]&&current[i]!=new[i]) {return Err(format!("{name}: conflicto en datos opacos"))}
        for i in 0..old.len(){if !covered[i]&&old[i]!=new[i]{out[i]=new[i]}}
    }
    Ok(out)
}

fn merge_records(old:&[Vec<u8>],new:&[Vec<u8>],current:&[Vec<u8>],name:&str)->Result<Vec<Vec<u8>>,String>{
    if old.len()!=new.len() || old.len()!=current.len(){
        if current==old || current==new{return Ok(new.to_vec())}
        return Err(format!("{name}: recuento modificado por otro sistema"))
    }
    old.iter().zip(new).zip(current).enumerate().map(|(i,((a,b),c))|merge_bytes(a,b,c,&format!("{name}[{i}]"))).collect()
}

/// `active` es la escena abierta; `finished` procede de `prepare_frame` y no
/// debe conservar escena, B4 ni efectos sin atender. `final_pool` incluye las
/// cartas de apertura ya publicadas en la sesión. No modifica argumentos.
pub fn prepare_save(model:&SaveModel,baseline:&Baseline,active:&LiveState,finished:&LiveState,finished_groups:&[Vec<[u8;8]>],final_pool:&[u8],pending_opening_effects:&[crate::pirate_empty_hideout_messages::Effect])->Result<SaveModel,String>{
    if baseline.live.scene.is_some(){return Err("la base debe preceder la apertura".into())}
    if !baseline.live.operations.is_empty()||!baseline.live.pending_effects.is_empty(){return Err("la base contiene efectos pendientes".into())}
    if !pending_opening_effects.is_empty(){return Err("efectos de apertura pendientes".into())}
    let scene=active.scene.as_ref().ok_or("escena activa ausente")?;
    if scene.saved_block!=baseline.model.sea_battles{return Err("origen de escena distinto de la base S05".into())}
    if model.sea_battles!=baseline.model.sea_battles{return Err("gestor de escenas modificado por otro sistema".into())}
    let prior=p3_save::tail::CurrentSeaBattles::parse(&baseline.model.sea_battles).map_err(|e|format!("gestor anterior inválido: {e:?}"))?;
    if crate::battle_manager::groups_from_save(&prior).map_err(str::to_owned)?!=baseline.groups{return Err("listas de encuentros no coinciden con la base S05".into())}
    if finished.scene_manager.len()<0xc38{return Err("gestor final de encuentros truncado".into())}
    let manager=&finished.scene_manager;
    let group_pointer=u32::from_le_bytes(manager[0xc30..0xc34].try_into().unwrap());
    let group_count=u32::from_le_bytes(manager[0xc34..0xc38].try_into().unwrap()) as usize;
    if group_count!=finished_groups.len() || (group_count!=0 && group_pointer==0) || (group_count==0 && group_pointer!=0) {return Err("listas de encuentros no coinciden con el gestor final".into())}
    if manager[..0x3fc].iter().any(|b|*b!=0){return Err("gestor final con escena activa".into())}
    let mut source=baseline.live.clone();source.scene=Some(scene.clone());
    let desired=pirate_empty_scene_publish::prepare_save(&baseline.model,&source,finished)?;
    let old=&baseline.model;let mut out=model.clone();
    out.ships.header=merge_bytes(&old.ships.header,&desired.ships.header,&model.ships.header,"cabecera naval")?;
    out.ships.captains=merge_records(&old.ships.captains,&desired.ships.captains,&model.ships.captains,"capitanes")?;
    out.ships.ships=merge_records(&old.ships.ships,&desired.ships.ships,&model.ships.ships,"barcos")?;
    out.ships.convoys=merge_records(&old.ships.convoys,&desired.ships.convoys,&model.ships.convoys,"convoyes")?;
    out.ships.tail=merge_bytes(&old.ships.tail,&desired.ships.tail,&model.ships.tail,"cola naval y bandas")?;
    if old.world.merchants.len()!=desired.world.merchants.len()||old.world.merchants.len()!=model.world.merchants.len(){return Err("recuento de comerciantes modificado".into())}
    let merchant_fields=p3_save::layout::MERCHANT.iter().map(|s|(usize::from(s.pos),usize::from(s.len))).collect::<Vec<_>>();
    out.world.merchants=old.world.merchants.iter().zip(&desired.world.merchants).zip(&model.world.merchants).enumerate().map(|(i,((a,b),c))|merge_fields(a,b,c,&merchant_fields,&format!("comerciante[{i}]"))).collect::<Result<Vec<_>,_>>()?;
    let mut scheduler_fields=vec![(0,2),(2,2),(4,2)];
    for i in 0..usize::from(baseline.live.scheduler.capacity){let p=6+i*24;scheduler_fields.extend([(p,4),(p+4,2),(p+6,2),(p+8,16)])}
    out.scheduled_tasks=merge_fields(&old.scheduled_tasks,&desired.scheduled_tasks,&model.scheduled_tasks,&scheduler_fields,"planificador")?;
    out.message_pool=merge_bytes(&old.message_pool,final_pool,&model.message_pool,"depósito de mensajes")?;
    let new_manager=if baseline.groups==finished_groups {desired.sea_battles.clone()} else {
        let groups=crate::battle_manager::groups_to_save(finished_groups);
        p3_save::tail::serialize_basic_sea_battles(&[],&groups).map_err(|e|format!("gestor final inválido: {e:?}"))?
    };
    out.sea_battles=merge_bytes(&old.sea_battles,&new_manager,&model.sea_battles,"gestor de batallas")?;
    p3_save::tail::MessagePool::parse(&out.message_pool).map_err(|e|format!("depósito conciliado inválido: {e:?}"))?;
    p3_save::tail::ScheduledTasks::parse(&out.scheduled_tasks).map_err(|e|format!("planificador conciliado inválido: {e:?}"))?;
    SaveModel::parse(&out.write()).map_err(|e|format!("S05 conciliado inválido: {e:?}"))?;
    Ok(out)
}

#[cfg(test)]
mod tests{
 use super::{merge_bytes,merge_fields,merge_records};
 #[test] fn field_merge_preserves_unrelated_and_rejects_conflicts(){
  assert_eq!(merge_fields(&[1,2,3],&[1,5,3],&[9,2,3],&[(0,1),(1,2)],"x").unwrap(),[9,5,3]);
  assert!(merge_fields(&[1,2,3],&[1,5,3],&[1,4,3],&[(0,1),(1,2)],"x").is_err());
  assert!(merge_fields(&[0,0],&[1,1],&[1,0],&[(0,2)],"x").is_err());
  assert_eq!(merge_bytes(&[1],&[1,2],&[1],"x").unwrap(),[1,2]);
  assert!(merge_bytes(&[1],&[1,2],&[3],"x").is_err());
  let old=vec![vec![1]];let grown=vec![vec![2],vec![3]];
  assert_eq!(merge_records(&old,&grown,&old,"pool").unwrap(),grown);
  assert!(merge_records(&old,&grown,&[vec![9]],"pool").is_err());
 }
}
