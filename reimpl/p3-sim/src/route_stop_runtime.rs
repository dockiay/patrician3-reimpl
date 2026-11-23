//! Estado vivo y publicación transaccional del depósito de paradas S05.
use sha2::{Digest,Sha256};
use p3_save::{model::SaveModel,tail::RouteStopPool};
use std::collections::BTreeSet;
fn validate(block:&[u8])->Result<(),String>{RouteStopPool::parse(block).map(|_|()).map_err(|e|format!("depósito de rutas S05: {e:?}"))}
fn digest(block:&[u8])->[u8;32]{Sha256::digest(block).into()}
/// Las versiones anteriores permiten continuar la misma transacción tras un
/// guardado intermedio; un bloque ajeno produce conflicto sin tocar el modelo.
#[derive(Clone,Debug)]
pub struct RouteStopRuntime {current:Vec<u8>,versions:BTreeSet<[u8;32]>}
impl RouteStopRuntime {
 pub fn from_model(model:&SaveModel)->Result<Self,String>{Self::from_block(&model.route_stop_pool)}
 pub fn from_block(block:&[u8])->Result<Self,String>{validate(block)?;Ok(Self{current:block.to_vec(),versions:BTreeSet::new()})}
 pub fn bytes(&self)->&[u8]{&self.current}
 /// Acepta un bloque provisional calculado por el anfitrión sobre esta misma
 /// versión. No acepta un resultado basado en otra revisión del depósito.
 pub fn replace(&mut self,before:&[u8],after:Vec<u8>)->Result<(),String>{
  if before!=self.current{return Err("depósito de rutas cambió durante la transacción".into())}validate(&after)?;
  if self.current!=after{self.versions.insert(digest(&self.current));self.current=after}Ok(())
 }
 /// 4D4DD0(índice,0), preparado sobre copia; un ciclo corrupto no muta nada.
 pub fn release(&mut self,id:u16)->Result<(),String>{
  let mut next=self.current.clone();let mut head=u16::from_le_bytes([next[0],next[1]]);
  p3_core::economy::pirate_group_callbacks::free_route_stop_pool(&mut head,&mut next[4..],id)?;
  next[..2].copy_from_slice(&head.to_le_bytes());let before=self.current.clone();self.replace(&before,next)
 }
 /// Publica en S05 sólo si contiene la versión actual o una versión previa
 /// conocida. El error no altera ningún bloque ni el propio runtime.
 pub fn persist_into(&self,model:&mut SaveModel)->Result<(),String>{
  let block=&model.route_stop_pool;validate(block)?;
  if *block!=self.current&&!self.versions.contains(&digest(block)){return Err("depósito de rutas modificado por otro sistema".into())}
  model.route_stop_pool=self.current.clone();Ok(())
 }
}
