//! Publicación preparada de prestigio y cartas del descubrimiento 52D0B0.
use p3_core::{ai::reputation::{Merchant,RepEnv},economy::trade_center_discovery::{self,Context,Effect}};
use crate::messages::{self,Payload,RETENTION};
#[derive(Clone)]pub struct Prepared{pub center:Vec<u8>,pub merchants:Vec<Merchant>,pub message_pool:Vec<u8>,pub presentation:Vec<Effect>}
/// `center` usa el formato serializado (30+16 por contrato), sin punteros.
/// La fecha es la que usa el GameWorld cargado; `renown` procede del EXE.
pub fn prepare(center:&[u8],convoy:u16,convoys:&[u8],ships:&[u8],captains:&[u8],merchants:&[Merchant],pool:&[u8],town_count:u32,time:u32,player:u32,hotseat:bool,date:(u8,u8,u16),renown:i16)->Result<Option<Prepared>,String>{
 if center.len()<30{return Err("centro serializado truncado".into())}let count=usize::from(u16::from_le_bytes([center[28],center[29]]));if center.len()!=30+16*count{return Err("contratos serializados truncados".into())}if merchants.iter().any(|m|m.bytes.len()!=0x650){return Err("comerciante de descubrimiento truncado".into())}
 let mut image=vec![0;44];image[..28].copy_from_slice(&center[..28]);let merchant_bytes=merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();let context=Context{convoys,ships,captains,merchants:&merchant_bytes,town_count,time,player,hotseat};let Some(plan)=trade_center_discovery::prepare(&image,convoy,&context).map_err(str::to_owned)?else{return Ok(None)};
 let mut after=center.to_vec();after[..28].copy_from_slice(&plan.center[..28]);let mut people=merchants.to_vec();let mut messages=pool.to_vec();let mut presentation=vec![];
 for effect in plan.effects{match effect{
  Effect::Reputation(owner)=>{let person=people.get_mut(usize::from(owner)).ok_or("comerciante de prestigio ausente")?;let env=RepEnv{town_count,towns:&[],renown:[0,renown,0],byte_52c:0};person.add_renown_and_social_all_towns_small(&env)},
  Effect::Letter{recipient,initialized}=>{let mut record=[0u8;16];record[..6].copy_from_slice(&initialized[..6]);record[8..].copy_from_slice(&initialized[6..]);match recipient{u32::MAX=>{for person in &mut people{messages::post_to_merchant(&mut messages,person,date.0,date.1,date.2,&record,Payload::None,&RETENTION)?;}},x if x==u32::MAX-1=>messages::add_global_inline_letter(&mut messages,date.0,date.1,date.2,&record)?,_=>return Err("destinatario de descubrimiento inválido".into())}},
  other=>presentation.push(other),
 }}
 Ok(Some(Prepared{center:after,merchants:people,message_pool:messages,presentation}))
}
