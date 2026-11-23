//! Reparto 0x18, deuda y cartas sobre copias antes del despacho.
use p3_core::{ai::reputation::Merchant,calendar::Calendar,economy::profit_share,scheduler::Scheduler};
pub struct Prepared{pub scheduler:Scheduler,pub merchants:Vec<Merchant>,pub pool:Vec<u8>,pub keep:bool}
pub fn prepare(s:&Scheduler,merchants:&[Merchant],pool:&[u8],tick:u32,stock:u32)->Result<Prepared,String>{
 let head=s.head;let task=*s.records.get(usize::from(head)).ok_or("tarea de participación ausente")?;let date=Calendar::for_tick(tick);let mut core=profit_share::prepare(task,merchants,tick,date.year,head,stock)?;let mut q=s.clone();
 if let Some(debt)=core.debt{
  if q.free_head>=q.capacity&&q.capacity.checked_add(p3_core::scheduler::GROWTH).is_none(){return Err("el crecimiento de la cola desborda la capacidad original".into())}
  let flag=q.dispatching;q.dispatching=0;let id=q.add_task(debt.due);q.dispatching=flag;
  let next=q.records[usize::from(id)].next;let mut created=debt;created.next=next;
  if q.head!=head{
   // 4E8804 vuelve a leer la cabeza tras reservar: si la deuda quedó
   // primera, sus campos pasan a ser los de la participación activa.
   if q.head!=id{return Err("la deuda no quedó en la cabeza esperada".into())}
   if merchants.len()>255{return Err("la deuda con deudor 255 necesita vistas de comerciante adicionales".into())}
   let owner=usize::from(task.u8_at(0x10));let mut people=merchants.to_vec();let m=people.get_mut(owner).ok_or("deudor de participación ausente")?;
   let gain=m.i32(0x46c).wrapping_sub(task.u32_at(0xc) as i32);if gain<=0{return Err("deuda de participación sin beneficio positivo".into())}
   let payment=i32::from(task.u8_at(0x12)).wrapping_mul(gain)/400;let cash=m.i32(0);
   m.set_i32(0x4bc,m.i32(0x4bc).wrapping_add(payment));if cash>0{m.set_i32(0,0)}if m.bytes[0x1e]<0xfe{m.bytes[0x1e]=m.bytes[0x1e].wrapping_add(2)}
   // La nueva cabeza es la propia deuda. El original lee su acreedor
   // recién escrito (0xFF) como deudor y destinatario; no hay pago ni carta
   // en esta primera llamada. El despachador elimina después esa deuda.
   core.notices.clear();core.merchants=people;core.task=task;core.task.set_u32_at(0xc,merchants[owner].i32(0x46c)as u32);created.set_u8_at(0x11,255);created.set_u8_at(0x14,0);created.due=created.due.wrapping_add(0x100+u32::from(id as u8));
  }else{core.task.next=q.records[usize::from(head)].next;}
  q.records[usize::from(id)]=created;
 }
 q.records[usize::from(head)]=core.task;let mut staged=pool.to_vec();
 for notice in &core.notices{let m=core.merchants.get_mut(usize::from(notice.recipient)).ok_or("destinatario de participación ausente")?;p3_state::messages::post_to_merchant(&mut staged,m,date.day,date.month,date.year,&notice.record,p3_state::messages::Payload::None,&p3_state::messages::RETENTION)?;}
 Ok(Prepared{scheduler:q,merchants:core.merchants,pool:staged,keep:core.keep})
}

#[cfg(test)]
mod tests{
 use super::*;use p3_core::scheduler::TaskRecord;
 #[test]fn dividend_debt_and_letters_are_staged_atomically(){
  let tick=(1400u32*365+90)<<8;let mut q=Scheduler::new();let head=q.add_task(tick);q.dispatching=1;let t=&mut q.records[usize::from(head)];*t=TaskRecord{due:tick,next:u16::MAX,kind:0x18,..Default::default()};t.set_u32_at(0xc,1000);t.set_u8_at(0x10,0);t.set_u8_at(0x11,1);t.set_u8_at(0x12,8);
  let mut m=vec![Merchant{bytes:vec![0;0x650]};2];for x in &mut m{x.set_u16(10,u16::MAX)}m[0].set_i32(0,50);m[0].set_i32(0x46c,5000);m[0].bytes[0x19]=2;
  let mut pool=vec![0u8;12];pool[..4].copy_from_slice(&0xaffe0005u32.to_le_bytes());pool[8..10].copy_from_slice(&u16::MAX.to_le_bytes());
  let out=prepare(&q,&m,&pool,tick,0).unwrap();assert!(out.keep);assert_eq!(out.merchants[0].i32(0),0);assert_eq!(out.merchants[1].i32(0),80);assert_eq!(out.merchants[1].i32(0x4c8),80);assert_eq!(out.merchants[0].i32(0x4bc),80);
  let debt=out.scheduler.records.iter().find(|t|t.kind==1).unwrap();assert_eq!(debt.u32_at(8),30);assert_eq!(debt.due,tick+0x1400);assert_eq!(out.merchants[0].u16(10),0);assert_eq!(out.merchants[1].u16(10),1);assert_eq!(out.pool[16],0x28);assert_eq!(out.pool[32],0x27);
  let before=q.clone();let err=prepare(&q,&m,&[],tick,0).err().unwrap();assert!(err.contains("pool"));assert_eq!(q,before);assert_eq!(m[0].i32(0),50);
 }
}
