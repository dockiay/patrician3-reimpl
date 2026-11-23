//! Textos del informe de batalla leídos del ejecutable local compatible.
use std::collections::HashMap;
use sha2::{Digest,Sha256};

#[derive(Clone)]
pub struct BattleTextBank { strings:HashMap<u32,Vec<u8>> }
impl BattleTextBank{
 pub fn from_english_exe(bytes:&[u8])->Result<Self,String>{
  let hash=format!("{:x}",Sha256::digest(bytes));if hash!="fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece"{return Err("ejecutable inglés no reconocido para textos de batalla".into())}
  let image=p3_assets::exe::PeImage::parse(bytes).map_err(str::to_string)?;let mut strings=HashMap::new();
  let fixed=[0x6aa1a4,0x6aa1e4,0x6a1dac,0x6a226c,0x6a2278,0x6aa180,0x6a2274,0x6a2270,0x6a2254,0x6a2248,0x6a2250,0x6a224c,0x6a225c,0x6a2244,0x6a2258,0x6a2240,0x6a2260,0x6a2264,0x6a2268];
  for slot in fixed.into_iter().chain((0..8).map(|i|0x6a174c+i*4)){let value=image.pointed_cstr(slot).map_err(|e|format!("texto {slot:#x}: {e}"))?;strings.insert(slot,value.to_vec());}
  Ok(Self{strings})
 }
 pub fn get(&self,slot:u32)->Option<Vec<u8>>{self.strings.get(&slot).cloned()}
}
