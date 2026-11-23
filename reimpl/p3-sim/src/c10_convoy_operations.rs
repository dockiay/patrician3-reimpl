//! Operaciones 0x1E/0x1F (0x00539B50/0x00539C60). El anfitrión conserva
//! la transacción y calcula la ruta real antes de publicar el movimiento.
use p3_core::{ai::reputation::Merchant,pump::Operation,ships::departure::ready_to_sail,town::Town};
use p3_save::ships::NavigationSnapshot;

fn u16_at(b:&[u8],at:usize)->Result<u16,String>{Ok(u16::from_le_bytes(b.get(at..at+2).ok_or("imagen naval truncada")?.try_into().unwrap()))}
fn gate(n:&NavigationSnapshot,towns:&[Town],convoy:u16)->Result<Option<u16>,String>{
 if convoy>=u16_at(&n.manager,0xf6)?{return Ok(None)}
 let c=usize::from(convoy)*0x3c;let image=n.convoys.get(c..c+0x3c).ok_or("convoy ausente")?;let leader=u16_at(image,0x10)?;
 if leader>=u16_at(&n.manager,0xf4)?{return Ok(None)}
 let s=usize::from(leader)*0x180;let ship=n.ships.get(s..s+0x180).ok_or("líder ausente")?;
 if u16_at(image,0x12)?<15&&!ready_to_sail(ship,towns,u16_at(&n.manager,0xf2)?).ok_or("líder truncado")?{return Ok(None)}
 Ok(Some(leader))
}

#[derive(Clone,Debug)]
pub struct StartPlan { pub convoy:u16,pub leader:u16,pub requested_funds:i32,pub selector:u16,pub move_operation:Operation }

pub fn prepare_start(n:&NavigationSnapshot,towns:&[Town],op:Operation)->Result<Option<StartPlan>,String>{
 if op.0[0]!=0x1e{return Err("operación de expedición incorrecta".into())}
 let convoy=op.0[2] as u16;let Some(leader)=gate(n,towns,convoy)?else{return Ok(None)};
 Ok(Some(StartPlan{convoy,leader,requested_funds:op.0[1] as i32,selector:(op.0[2]>>16) as u16,move_operation:Operation([0x1e,u32::from(leader),0xff,0x76,0x640])}))
}

/// Ejecutar después de 0x00537670, sobre el estado provisional de la misma
/// transacción. El reembolso anterior cambia el oro disponible para el límite.
pub fn finish_start(p:&StartPlan,n:&mut NavigationSnapshot,merchants:&mut [Merchant],expedition_countdown:&mut u8)->Result<(),String>{
 let c=usize::from(p.convoy)*0x3c;let image=n.convoys.get_mut(c..c+0x3c).ok_or("convoy ausente después de la orden")?;
 let mut funds=p.requested_funds;let owner=usize::from(image[0]);
 if let Some(merchant)=merchants.get_mut(owner){let gold=merchant.i32(0);if funds>gold{funds=gold.max(0)}merchant.set_i32(0,gold.wrapping_sub(funds));*expedition_countdown=(*expedition_countdown).max(p.move_operation.0[3] as u8)}
 image[0x38..0x3a].copy_from_slice(&p.selector.to_le_bytes());image[0x24..0x34].fill(0xff);image[0x34..0x38].fill(0);
 let old=i32::from_le_bytes(image[0x20..0x24].try_into().unwrap());image[0x20..0x24].copy_from_slice(&old.wrapping_add(funds).to_le_bytes());Ok(())
}

pub fn set_waypoint(n:&mut NavigationSnapshot,towns:&[Town],op:Operation)->Result<bool,String>{
 if op.0[0]!=0x1f{return Err("operación de punto de expedición incorrecta".into())}
 let convoy=op.0[1] as u16;if gate(n,towns,convoy)?.is_none(){return Ok(false)}let slot=(op.0[1]>>16) as usize;
 // El original permite escribir fuera del array; no reinterpretar esa
// corrupción de memoria como un quinto punto válido.
 if slot>=4{return Err("punto de expedición fuera de los cuatro elementos".into())}
 let c=usize::from(convoy)*0x3c;n.convoys[c+0x24+4*slot..c+0x28+4*slot].copy_from_slice(&op.0[2].to_le_bytes());n.convoys[c+0x34+slot]=op.0[3] as u8;Ok(true)
}
