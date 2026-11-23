//! Preparación atómica de la recompensa y carta global por primer hallazgo.
use p3_core::{ai::reputation::Merchant,calendar::Calendar,economy::expedition_discovery};

pub struct Prepared {pub merchant:Merchant,pub world_flag:u8,pub message_pool:Vec<u8>,pub notify:[u32;6],pub event:[u32;3]}

pub fn prepare(world_flag:u8,owner:u8,merchants:&[Merchant],merchant_count:u16,player:u32,renown:i16,pool:&[u8],calendar:Calendar)->Result<Option<Prepared>,String>{
 let Some(event)=expedition_discovery::prepare(world_flag,owner,merchants,merchant_count,player,renown)?else{return Ok(None)};
 let mut next=pool.to_vec();p3_state::messages::add_global_inline_letter(&mut next,calendar.day,calendar.month,calendar.year,&event.letter)?;
 Ok(Some(Prepared{merchant:event.merchant,world_flag:event.world_flag,message_pool:next,notify:event.notify,event:event.event}))
}

/// Resultado de UN waypoint. El llamante debe intercalarlo con los efectos
/// del centro correspondiente: el original no recorre primero todo el mapa.
pub struct PreparedMark {pub merchant:Merchant,pub global_map:Vec<u8>,pub world_flag:u8,pub message_pool:Vec<u8>,pub marked:bool,pub dirty:bool,pub notify:Option<[u32;6]>,pub event:Option<[u32;3]>}

pub fn prepare_mark(world_flag:u8,owner:u8,merchants:&[Merchant],merchant_count:u16,player:u32,renown:i16,pool:&[u8],calendar:Calendar,global_map:&[u8],points:&[(u16,u16)],waypoint:(u16,u16),wide:bool,dirty_target:bool)->Result<PreparedMark,String>{
 if u16::from(owner)>=merchant_count{return Err("comerciante de ruta fuera de rango".into())}
 if global_map.len()!=0x64||points.is_empty()||points.len()>384{return Err("mapa de descubrimiento incompleto".into())}
 let mut people=merchants.to_vec();let person=people.get_mut(usize::from(owner)).ok_or("comerciante de ruta ausente")?;
 if person.bytes.len()!=0x650{return Err("comerciante de ruta truncado".into())}
 let mut global=global_map.to_vec();let mut dirty=false;let map=p3_core::ai::misc::MapPoints{points,dirty_target};
 let marked=person.mark_map_point_seen(&mut global,&map,waypoint.0 as i16,waypoint.1 as i16,wide,&mut dirty);
 let event=if marked{prepare(world_flag,owner,&people,merchant_count,player,renown,pool,calendar)?}else{None};
 let (person,flag,messages,notification,window)=if let Some(event)=event{(event.merchant,event.world_flag,event.message_pool,Some(event.notify),Some(event.event))}else{(people[usize::from(owner)].clone(),world_flag,pool.to_vec(),None,None)};
 Ok(PreparedMark{merchant:person,global_map:global,world_flag:flag,message_pool:messages,marked,dirty,notify:notification,event:window})
}
