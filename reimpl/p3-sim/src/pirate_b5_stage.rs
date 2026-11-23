//! Imágenes temporales y publicación diferida de B5 sobre estado vivo C10.
use p3_core::{economy::pirate_b5_memory::{self,Effects},flatmem::{Mem,SparseMem},pump::Operation,ships::tick::TickWorld};
use p3_save::pirate_bands::PirateBand;
const M:u32=0x6dd7a0;const BAND:u32=0x56000000;const PIECES:u32=0x57000000;const SHIPS:u32=0x58000000;const CV:u32=0x59000000;const CAPS:u32=0x5a000000;const MER:u32=0x5b000000;const TOWNS:u32=0x5c000000;const PLAN:u32=0x5d000000;
/// Entrada al llamante con campos suplementarios (EXE, ajustes, pool y fechas)
/// semánticamente explícitos. Los rangos no pueden solaparse con otras vistas.
#[derive(Clone)]pub struct Prepared<H>{pub manager:Vec<u8>,pub ships:Vec<u8>,pub captains:Vec<u8>,pub convoys:Vec<u8>,pub merchants:Vec<u8>,pub towns:Vec<u8>,pub bands:Vec<Option<PirateBand>>,pub host:H}
fn read(m:&SparseMem,p:u32,n:usize)->Result<Vec<u8>,String>{
 if p.checked_add(u32::try_from(n).map_err(|_|"región excesiva")?).is_none()||!m.contains_range(p,n){return Err("imagen B5 no asignada o truncada".into())}Ok(m.read(p,n))
}
pub fn prepare<H:Effects>(w:&TickWorld,bands:&[Option<PirateBand>],extra_memory:&[(u32,Vec<u8>)],op:Operation,host:&H)->Result<Prepared<H>,String>{
 if op.0[0]!=0xb5||bands.len()!=5||w.remove.manager.len()<0x110{return Err("estado B5 incompatible".into())}
 if [w.remove.ships.len(),w.remove.captains.len(),w.remove.convoys.len(),w.remove.merchants.len(),w.remove.towns.len()].into_iter().any(|n|n>0x1000000){return Err("región B5 excesiva".into())}
 let mut mem=SparseMem::default();for (p,b)in[(M,&*w.remove.manager),(SHIPS,&*w.remove.ships),(CAPS,&*w.remove.captains),(CV,&*w.remove.convoys),(MER,&*w.remove.merchants),(TOWNS,&*w.remove.towns)]{mem.seed(p,b)}
 mem.w32(M,CAPS);mem.w32(M+4,SHIPS);mem.w32(M+8,CV);
 for(i,b)in bands.iter().enumerate(){let a=BAND+i as u32*0x10000;let p=PIECES+i as u32*0x100000;let q=PLAN+i as u32*0x100000;
  if let Some(b)=b{b.validate().map_err(str::to_owned)?;if b.pieces.len()>0x100000||b.plan.as_ref().is_some_and(|p|p.len()>0x100000){return Err("cuadrilla B5 excesiva".into())}mem.seed(a,&[0;24]);mem.seed(a+8,&b.header);if !b.pieces.is_empty(){mem.seed(p,&b.pieces);mem.w32(a+4,p)}if let Some(plan)=&b.plan{mem.seed(q,plan);mem.w32(a,q)}mem.w32(M+12+4*i as u32,a)}else{mem.w32(M+12+4*i as u32,0)}
 }
 mem.seed(0x6de518,&MER.to_le_bytes());mem.seed(0x6de508,&TOWNS.to_le_bytes());mem.seed(0x6de4aa,&w.remove.merchant_count.to_le_bytes());mem.seed(0x6de4b0,&w.remove.town_count.to_le_bytes());mem.seed(0x6de4b4,&w.time.to_le_bytes());
 for (p,b)in extra_memory{if p.checked_add(u32::try_from(b.len()).map_err(|_|"semilla B5 excesiva")?).is_none()||mem.contains_any(*p,b.len()){return Err("semilla B5 solapada".into())}mem.seed(*p,b)}
 let (mut result,after_host)=pirate_b5_memory::prepare(op,&mem,host)?;
 let ships_count=usize::from(result.r16(M+0xf4));let caps_count=usize::from(result.r16(M+0xf2));let cv_count=usize::from(result.r16(M+0xf6));
 let ships=read(&result,SHIPS,w.remove.ships.len().max(ships_count*0x180))?;let captains=read(&result,CAPS,w.remove.captains.len().max(caps_count*16))?;let convoys=read(&result,CV,w.remove.convoys.len().max(cv_count*0x3c))?;let merchants=read(&result,MER,w.remove.merchants.len())?;let towns=read(&result,TOWNS,w.remove.towns.len())?;
 let mut manager=read(&result,M,w.remove.manager.len())?;manager[..0x20].copy_from_slice(&w.remove.manager[..0x20]);
 let mut after_bands=bands.to_vec();for(i,old)in bands.iter().enumerate(){let a=BAND+i as u32*0x10000;let ptr=result.r32(M+12+4*i as u32);let Some(old)=old else{if ptr!=0{return Err("alta de banda B5 no soportada".into())}continue};
  if ptr==0{after_bands[i]=None;continue}if ptr!=a{return Err("banda B5 reubicada".into())}let mut b=old.clone();b.header.copy_from_slice(&read(&result,a+8,16)?);
  let count=usize::from(u16::from_le_bytes([b.header[0],b.header[1]]));let piece_ptr=result.r32(a+4);b.pieces=if count==0{Vec::new()}else{read(&result,piece_ptr,count*8)?};let plan_ptr=result.r32(a);b.plan=if plan_ptr==0{None}else{let hdr=read(&result,plan_ptr,0x64)?;let rows=u32::from_le_bytes(hdr[..4].try_into().unwrap())as usize;let cols=u32::from_le_bytes(hdr[4..8].try_into().unwrap())as usize;let len=rows.checked_mul(cols).and_then(|n|n.checked_add(0x64)).filter(|n|*n<=0x100000).ok_or("plano B5 inválido")?;Some(read(&result,plan_ptr,len)?)};b.validate().map_err(str::to_owned)?;after_bands[i]=Some(b);
 }
 Ok(Prepared{manager,ships,captains,convoys,merchants,towns,bands:after_bands,host:after_host})
}
