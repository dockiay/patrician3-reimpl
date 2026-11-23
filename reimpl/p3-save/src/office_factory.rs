//! Aplicación transaccional del pool de oficinas a un mundo S05.
use p3_core::{ai::reputation::Merchant,buildings::Site,economy::office_factory::{OfficePool,OFFICE_SIZE},town::Town};
use crate::{layout,model::WorldModel,office::{decode_office,encode_office},world::World};

fn get16(b:&[u8],at:usize)->u16{u16::from_le_bytes([b[at],b[at+1]])}
fn encode(record:&[u8])->Result<Vec<u8>,String>{
 let image:&[u8;OFFICE_SIZE]=record.try_into().map_err(|_|"oficina truncada")?;
 if get16(image,0x2c4)==u16::MAX {let mut out=Vec::with_capacity(6);out.extend_from_slice(&image[0x2c4..0x2c6]);out.extend_from_slice(&image[0x2c8..0x2cc]);Ok(out)} else {Ok(encode_office(image).to_vec())}
}

/// Reconstruye los bytes persistidos del pool; los huecos de memoria libres no están guardados.
pub fn pool_from_world(world:&WorldModel)->Result<OfficePool,String>{
 if world.header.len()<0x20{return Err("cabecera de mundo truncada".into())}
 let mut records=Vec::with_capacity(world.offices.len());
 for saved in &world.offices {
  if saved.len()==6&&get16(saved,0)==u16::MAX {let mut image=vec![0;OFFICE_SIZE];image[0x2c4..0x2c6].copy_from_slice(&saved[..2]);image[0x2c8..0x2cc].copy_from_slice(&saved[2..6]);records.push(image)}
  else {let saved:&[u8;OFFICE_SIZE]=saved.as_slice().try_into().map_err(|_|"oficina guardada truncada")?;records.push(decode_office(saved).to_vec())}
 }
 Ok(OfficePool{records,free:get16(&world.header,0x1e)})
}

/// Aplica únicamente cambios de fábrica y rechaza estados concurrentes o incompatibles.
pub fn apply(world:&mut WorldModel,before:&OfficePool,after:&OfficePool,old_towns:&[Town],new_towns:&[Town],old_merchants:&[Merchant],new_merchants:&[Merchant])->Result<(),String>{
 if old_towns.len()!=world.towns.len()||new_towns.len()!=world.towns.len()||old_merchants.len()!=world.merchants.len()||new_merchants.len()!=world.merchants.len(){return Err("dimensiones de fábrica incompatibles".into())}
 if world.header.len()<0x20||before.records.len()!=world.offices.len()||after.records.len()<before.records.len()||after.records.len()>u16::MAX as usize||get16(&world.header,0x1e)!=before.free{return Err("cabecera de oficinas desincronizada".into())}
 for (saved,prior) in world.offices.iter().zip(&before.records){if *saved!=encode(prior)?{return Err("oficina anterior desincronizada".into())}}
 let raw=world.write();let parsed=World::parse(&raw).map_err(|e|format!("mundo: {e:?}"))?;let mut staged=world.clone();
 for i in 0..old_towns.len(){
  if old_towns[i].0.iter().zip(new_towns[i].0.iter()).enumerate().any(|(offset,(a,b))|!(0x784..0x786).contains(&offset)&&a!=b){return Err("cambio ajeno a fábrica en ciudad".into())}
  if old_towns[i].u16(0x784)!=parsed.town(i).u16(0x784){return Err("cadena urbana de oficinas desincronizada".into())}
  if old_towns[i].u16(0x784)!=new_towns[i].u16(0x784){let site=Site{records:parsed.building_records(i).to_vec(),cells:parsed.plan_cells(i).to_vec()};let prior=parsed.town(i);let mut next=prior.clone();next.set_u16(0x784,new_towns[i].u16(0x784));staged.towns[i].apply_town(&prior,&next,&site)?;}
 }
 for i in 0..old_merchants.len(){if old_merchants[i].bytes.len()!=0x650||new_merchants[i].bytes.len()!=0x650||old_merchants[i].bytes.iter().zip(&new_merchants[i].bytes).enumerate().any(|(offset,(a,b))|!(0xc..0xe).contains(&offset)&&a!=b){return Err("cambio ajeno a fábrica en comerciante".into())}let before=old_merchants[i].u16(0xc);let after=new_merchants[i].u16(0xc);let rec=staged.merchants.get_mut(i).ok_or("comerciante ausente")?;if rec.len()<layout::MERCHANT_RECORD_LEN||get16(rec,0xc)!=before{return Err("cadena de comerciante desincronizada".into())}rec[0xc..0xe].copy_from_slice(&after.to_le_bytes());}
 staged.offices=after.records.iter().map(|r|encode(r)).collect::<Result<_,_>>()?;staged.header[0x1e..0x20].copy_from_slice(&after.free.to_le_bytes());*world=staged;Ok(())
}
