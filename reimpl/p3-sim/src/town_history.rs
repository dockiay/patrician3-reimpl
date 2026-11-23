//! Conexión C10 del historial diario de conocimiento de ciudades.
use crate::Sim;
impl Sim {
 pub(crate) fn town_history_step(&mut self,tick:u32)->Result<(),String>{
  let history=self.host.town_history.as_ref().ok_or("historial de ciudades no compatible")?;
  if history.len()<0xa08{return Err("historial de ciudades truncado".into())}
  let n=self.economy.world.towns.len();let m=usize::from(self.idle.clock.merchant_count);
  if m>self.rep_merchants.len(){return Err("comerciantes del historial ausentes".into())}
  let kinds=self.rep_merchants[..m].iter().map(|merchant|merchant.u16(8)).collect::<Vec<_>>();
  let flags=self.economy.world.towns.iter().map(|town|town.u32(0x2c8)).collect::<Vec<_>>();
  let pool=self.host.office_pool.as_ref().ok_or("pool de oficinas ausente")?;
  let mut offices=vec![Vec::new();m];
  for &owner in &history[0xa00..0xa08]{
   let owner=usize::from(owner);if owner>=m||kinds[owner]!=0||!offices[owner].is_empty(){continue}
   let mut office=usize::from(self.rep_merchants[owner].u16(0xc));let mut seen=vec![false;pool.records.len()];
   while office<pool.records.len(){
    if seen[office]{return Err(format!("cadena de oficinas cíclica del comerciante {owner}"))}seen[office]=true;
    let record=&pool.records[office];if record.len()<0x2ca{return Err("oficina truncada".into())}
    let city=record[0x2c6];if usize::from(city)>=n{return Err("ciudad de oficina inválida".into())}
    offices[owner].push(city);office=usize::from(u16::from_le_bytes([record[0x2c8],record[0x2c9]]));
   }
  }
  let world=self.host.founded_world.as_ref().ok_or("matriz de ciudades ausente")?;
  let distances=world.matrix.get(..n.checked_mul(n).ok_or("matriz excesiva")?).ok_or("matriz de ciudades truncada")?;
  let mut next=history.clone();p3_core::economy::town_history::daily_serialized(&mut next,tick,&flags,&kinds,&offices,distances).map_err(str::to_string)?;
  if &next!=history{self.host.town_history_versions.insert(crate::save_version(history));}
  self.host.town_history=Some(next);Ok(())
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 use serde_json::Value;
 fn bytes(v:&Value)->Vec<u8>{v.as_str().unwrap().as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect()}
 #[test]
 fn real_save_history_step_matches_x86_and_persists(){
  let Ok(root)=std::env::var("P3_SAVES") else{return};
  let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
  let corpus:Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_town_history.json")).unwrap();
  let row=corpus["cases"].as_array().unwrap().iter().find(|r|r["offices"].as_array().unwrap().iter().any(|x|x.as_array().unwrap().len()>=2)).unwrap();
  let flags=row["flags"].as_array().unwrap();let kinds=row["kinds"].as_array().unwrap();let mut sim=Sim::from_save_bytes(&raw).unwrap();
  sim.economy.world.towns.truncate(flags.len());sim.idle.clock.merchant_count=kinds.len() as u16;
  for(i,flag)in flags.iter().enumerate(){sim.economy.world.towns[i].set_u32(0x2c8,flag.as_u64().unwrap() as u32)}
  sim.host.founded_world.as_mut().unwrap().matrix=row["distances"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
  let pool=sim.host.office_pool.as_mut().unwrap();pool.records.clear();
  for(i,kind)in kinds.iter().enumerate(){
   let merchant=&mut sim.rep_merchants[i];merchant.bytes[8..10].copy_from_slice(&(kind.as_u64().unwrap() as u16).to_le_bytes());
   let cities=row["offices"][i].as_array().unwrap();let start=pool.records.len();merchant.bytes[0xc..0xe].copy_from_slice(&(if cities.is_empty(){u16::MAX}else{start as u16}).to_le_bytes());
   for(j,city)in cities.iter().enumerate(){let mut office=vec![0;0x44c];office[0x2c6]=city.as_u64().unwrap() as u8;office[0x2c8..0x2ca].copy_from_slice(&(if j+1<cities.len(){(start+j+1) as u16}else{u16::MAX}).to_le_bytes());pool.records.push(office);}
  }
  let before=bytes(&row["before"]);let expected=bytes(&row["after_each"][0]);sim.host.town_history_initial=Some(before.clone());sim.host.town_history=Some(before.clone());
  sim.town_history_step(row["tick"].as_u64().unwrap() as u32).unwrap();assert_eq!(sim.host.town_history.as_ref().unwrap(),&expected);
  let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();let rest=p3_save::world_rest::WorldRest::parse(&model.world.rest).unwrap();let boundary=rest.centers[0].start;let center=rest.center(0).to_vec();model.world.rest.splice(..boundary,before);
  let mut conflict=model.clone();conflict.world.rest[0]^=1;let snapshot=conflict.clone();assert!(sim.persist_town_history(&mut conflict).is_err());assert_eq!(conflict,snapshot);
  sim.persist_town_history(&mut model).unwrap();let rest=p3_save::world_rest::WorldRest::parse(&model.world.rest).unwrap();assert_eq!(&model.world.rest[..rest.centers[0].start],expected);assert_eq!(rest.center(0),center);
  let loaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(loaded.host.town_history.unwrap(),expected);
 }
}
