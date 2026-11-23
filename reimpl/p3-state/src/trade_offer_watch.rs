//! Watchdog 0x4E7A14 y cancelación 0x5340D0, sobre imágenes transaccionales.
use p3_core::{scheduler::TaskRecord,ships::cargo::recompute_load};
use p3_save::tail::MessagePool;
const UNITS:[u8;24]=[0,0,0,1,1,1,1,1,1,1,1,0,1,1,0,1,0,0,1,0,1,1,1,1];
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct State {pub offers:Vec<[u8;16]>,pub pool:Vec<u8>,pub merchants:Vec<Vec<u8>>,pub towns:Vec<Vec<u8>>,pub offices:Vec<Vec<u8>>,pub ships:Vec<Vec<u8>>,pub convoys:Vec<Vec<u8>>,pub docked:u16}
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Prepared {pub task:TaskRecord,pub state:State,pub keep:bool}
fn w(b:&[u8],at:usize)->u16{u16::from_le_bytes([b[at],b[at+1]])}
fn d(b:&[u8],at:usize)->u32{u32::from_le_bytes(b[at..at+4].try_into().unwrap())}
fn sw(b:&mut[u8],at:usize,v:u16){b[at..at+2].copy_from_slice(&v.to_le_bytes())}
fn sd(b:&mut[u8],at:usize,v:u32){b[at..at+4].copy_from_slice(&v.to_le_bytes())}
fn validate(s:&State)->Result<(),String>{if s.merchants.iter().any(|v|v.len()!=0x650)||s.towns.iter().any(|v|v.len()!=0x9f8)||s.offices.iter().any(|v|v.len()!=0x44c)||s.ships.iter().any(|v|v.len()!=0x180)||s.convoys.iter().any(|v|v.len()!=0x3c){return Err("imágenes de oferta truncadas".into())}Ok(())}
fn find(s:&State,owner:u32,kind:u8,key:u32,seller:u32,buyer:u32)->Result<Option<usize>,String>{
 let Some(m)=s.merchants.get(owner as usize) else{return Ok(None)};let p=MessagePool::parse(&s.pool).map_err(|e|format!("pool: {e:?}"))?;let mut at=usize::from(w(m,10));let mut seen=vec![false;p.letters.len()];
 while at<p.letters.len(){if seen[at]{return Err("ciclo en cartas de oferta".into())}seen[at]=true;let b=p.letter(at);if b[4]==kind && u32::from(w(b,12))==seller && u32::from(w(b,14))==buyer && d(b,8)==key{return Ok(Some(at))}at=usize::from(w(b,6));}Ok(None)
}
fn remove(s:&mut State,owner:usize,id:usize)->Result<(),String>{
 let p=MessagePool::parse(&s.pool).map_err(|e|format!("pool: {e:?}"))?;let ranges=p.letters.iter().map(|v|v.range.clone()).collect::<Vec<_>>();let mut at=usize::from(w(&s.merchants[owner],10));let mut prev=None;let mut seen=vec![false;ranges.len()];while at<ranges.len()&&at!=id{if seen[at]{return Err("ciclo en cartas de oferta".into())}seen[at]=true;prev=Some(at);at=usize::from(w(&s.pool,ranges[at].start+6));}if at>=ranges.len(){return Err("carta de oferta ausente".into())}
 let start=ranges[id].start;if ranges[id].len()!=16{return Err("carta de oferta no inline".into())}let next=w(&s.pool,start+6);if let Some(p)=prev{sw(&mut s.pool,ranges[p].start+6,next)}else{sw(&mut s.merchants[owner],10,next)}let free=w(&s.pool,6);s.pool[start..start+6].copy_from_slice(&[0,0,255,255,255,0]);sw(&mut s.pool,start+6,free);s.pool[start+8..start+16].fill(0);sw(&mut s.pool,6,id as u16);Ok(())
}
fn refund(s:&mut State,offer:[u8;16])->Result<(),String>{
 let seller=usize::from(offer[8]);let city=usize::from(offer[10]);let kind=offer[11];let object=d(&offer,12);match kind {
 0=>{let ware=usize::from(object as u16);if ware>=24{return Ok(())}let amount=u32::from((object>>16) as u16).wrapping_mul(if UNITS[ware]!=0{200}else{2000});let mut office=None;
 if seller<s.merchants.len(){if let Some(t)=s.towns.get(city){let mut at=usize::from(w(t,0x784));let mut seen=vec![false;s.offices.len()];while at<s.offices.len(){if seen[at]{return Err("ciclo en oficinas de oferta".into())}seen[at]=true;if usize::from(w(&s.offices[at],0x2c4))==seller{office=Some(at);break}at=usize::from(w(&s.offices[at],0x2ca));}}}
 if let Some(id)=office{let pos=4+ware*4;let value=d(&s.offices[id],pos).wrapping_add(amount);sd(&mut s.offices[id],pos,value)}else if let Some(m)=s.merchants.get(seller){let mut id=usize::from(w(m,0xe));let mut seen=vec![false;s.ships.len()];while id<s.ships.len(){if seen[id]{return Err("ciclo en barcos de oferta".into())}seen[id]=true;let next=usize::from(w(&s.ships[id],4));if usize::from(s.ships[id][0x39])==city && w(&s.ships[id],0x134)<4 {let free=recompute_load(&mut s.ships[id]);if free>=amount as i32{let pos=0x54+ware*4;let value=d(&s.ships[id],pos).wrapping_add(amount);sd(&mut s.ships[id],pos,value);let flags=w(&s.ships[id],0x3c)|4;sw(&mut s.ships[id],0x3c,flags);let convoy=usize::from(w(&s.ships[id],8));if let Some(cv)=s.convoys.get_mut(convoy){let f=d(cv,0x18)|4;sd(cv,0x18,f)}recompute_load(&mut s.ships[id]);break}}id=next;}}},
 1=>{let id=usize::from(object as u16);if let Some(ship)=s.ships.get_mut(id){if w(ship,6)==0xffff{sw(ship,0x134,0);ship[0x3c]|=0x20;sw(ship,6,s.docked);s.docked=id as u16}}},
 2=>if let Some(m)=s.merchants.get_mut(seller){let gold=d(m,0).wrapping_add(object);sd(m,0,gold)},_=>()};Ok(())
}
/// Rechaza cadenas cíclicas y prepara todos los cambios antes de publicarlos.
pub fn prepare(task:&TaskRecord,state:&State)->Result<Prepared,String>{
 if task.kind!=0x13{return Err("tarea de oferta inválida".into())}validate(state)?;let mut s=state.clone();let mut t=*task;let key=t.u32_at(8);let seller=t.u32_at(0x10);let buyer=t.u32_at(0x14);
 let a=find(&s,seller,0x1e,key,seller,buyer)?;let b=find(&s,buyer,0x1f,key,seller,buyer)?;let offer=s.offers.iter().position(|v|d(v,0)==key&&u32::from(v[8])==seller&&u32::from(v[9])==buyer);
 if a.is_some()&&b.is_some()&&offer.is_some(){t.due=t.due.wrapping_add(0x40);return Ok(Prepared{task:t,state:s,keep:true})}
 if let Some(id)=a{remove(&mut s,seller as usize,id)?}if let Some(id)=b{remove(&mut s,buyer as usize,id)?}if let Some(id)=offer{let v=s.offers.remove(id);refund(&mut s,v)?}Ok(Prepared{task:t,state:s,keep:false})
}
