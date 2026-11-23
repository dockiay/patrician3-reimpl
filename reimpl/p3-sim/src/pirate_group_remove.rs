//! Adaptador de baja naval 0x00507FB0 sobre imágenes de 514000.
use p3_core::{flatmem::Mem,ships::{remove::{RemoveWorld,RemoveEvent,remove_ship},tick::TickWorld,order,route::{self,RouteHost},convoy}};
const M:u32=0x6dd7a0;
fn read(m:&mut dyn Mem,base:u32,len:usize)->Vec<u8>{(0..len).map(|i|m.r8(base+i as u32)).collect()}
fn write(m:&mut dyn Mem,base:u32,data:&[u8]){for(i,&v)in data.iter().enumerate(){m.w8(base+i as u32,v)}}
#[derive(Debug)]pub struct RemoveDelta{pub events:Vec<RemoveEvent>,pub cleared_route:Option<u16>,pub merchant_owners:Vec<(u32,Vec<u8>)>}
struct Routes;impl RouteHost for Routes{fn points_of(&mut self,_:u32)->u32{0}fn free(&mut self,_:u32){}fn ui_route_cleared(&mut self,_:u16){}}
/// El llamante conserva la transacción y procesa los eventos devueltos. En
/// particular PoolRelease/MerchantChanged necesitan sus vistas vivas; no se
/// descartan. cleared_route identifica la entrada a invalidar en S02.
pub fn remove(m:&mut dyn Mem,id:u16,player:u32,pool_limit:u16)->Result<RemoveDelta,String>{
 let ship_count=m.r16(M+0xf4);let cap_count=m.r16(M+0xf2);let convoy_count=m.r16(M+0xf6);let merchant_count=m.r16(0x6de4aa);let town_count=m.r32(0x6de4b0);
 if id>=ship_count||town_count>40{return Err("baja pirata fuera del mundo".into())}
 let ship_base=m.r32(M+4);let cap_base=m.r32(M);let cv_base=m.r32(M+8);let town_base=m.r32(0x6de508);let mer_base=m.r32(0x6de518);
 if [ship_base,cap_base,cv_base,town_base,mer_base].contains(&0){return Err("vistas de baja incompletas".into())}
 let at=ship_base+u32::from(id)*0x180;let owner=m.r8(at);let mut free=m.r16(M+0xe2);let mut seen=vec![false;usize::from(ship_count)];let mut already_free=false;
 while free<ship_count{let i=usize::from(free);if seen[i]{return Err("lista libre naval cíclica".into())}seen[i]=true;if free==id{already_free=true;break}free=m.r16(ship_base+u32::from(free)*0x180+6)}
 // 515F00 ocurre ANTES de sobrescribir ship+4 en 507FB0. La traducción
 // genérica devuelve ese callback más tarde, por lo que se adelanta aquí.
 let mut detached=Vec::new();
 if !already_free&&u16::from(owner)>=merchant_count{for k in 0..5{let band=m.r32(M+0xc+4*k);if band!=0{p3_core::economy::pirate_group_callbacks::unlink_band_ship(m,band,id)?;detached.push((band,u32::from(id)))}}}
 let mut manager=read(m,M,0x108);let mut ships=read(m,ship_base,usize::from(ship_count)*0x180);let mut captains=read(m,cap_base,usize::from(cap_count)*16);let mut convoys=read(m,cv_base,usize::from(convoy_count)*0x3c);let mut towns=read(m,town_base,town_count as usize*0x9f8);let mut merchants=read(m,mer_base,usize::from(merchant_count)*0x650);
 let mut remove=RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut captains,convoys:&mut convoys,towns:&mut towns,merchants:&mut merchants,pool_limit,merchant_count,town_count,ui_a:false,ui_b:false};let events=remove_ship(&mut remove,u32::from(id));let mut world=TickWorld{remove:&mut remove,time:0,player};let mut rest=Vec::new();let mut cleared=None;let mut merchant_owners=Vec::new();
 for e in events{match e{
  RemoveEvent::ClearRoute(i)=>{let at=i as usize*0x180;let had=world.remove.ships[at+0x24..at+0x28]!=[0;4];route::clear_route(&mut world.remove.ships[at..at+0x180],i as u16,player,false,&mut Routes);if had{cleared=Some(i as u16)}},
  RemoveEvent::OrderRemove(i)=>order::remove_from_tick(&mut world,i as u16),
  RemoveEvent::ConvoyRelease(band,ship)=>{
   // Nombre histórico ambiguo: este evento es 515F00, no 50A3D0.
   // Se ha ejecutado arriba, antes de que remove_ship borre el enlace +4.
   let i=detached.iter().position(|&v|v==(band,ship)).ok_or("desenlace de cuadrilla no aplicado")?;detached.remove(i);
  },
  RemoveEvent::ConvoyRefresh(i)=>convoy::refresh_from_ships(&mut world,i).map_err(|e|format!("recuento de convoy: {e:?}"))?,
  RemoveEvent::MerchantChanged(owner)=>{merchant_owners.push((owner,world.remove.ships.chunks_exact(0x180).map(|s|s[0]).collect()));rest.push(RemoveEvent::MerchantChanged(owner))},
  other=>rest.push(other),
 }}
 if !detached.is_empty(){return Err("desenlace de cuadrilla sin callback original".into())}
 write(m,M,&manager);write(m,ship_base,&ships);write(m,cap_base,&captains);write(m,cv_base,&convoys);write(m,town_base,&towns);write(m,mer_base,&merchants);Ok(RemoveDelta{events:rest,cleared_route:cleared,merchant_owners})
}

/// Efectos de las bajas efectuadas por disolución; el consumidor debe
/// invalidar rutas y despachar cada evento externo antes de publicar.
#[derive(Default,Debug)]pub struct DissolveDelta{pub events:Vec<RemoveEvent>,pub cleared_routes:Vec<u16>,pub removed_plan:bool,pub merchant_owners:Vec<(u32,Vec<u8> )>}
fn merge(delta:&mut DissolveDelta,out:RemoveDelta){delta.events.extend(out.events);delta.merchant_owners.extend(out.merchant_owners);if let Some(id)=out.cleared_route{delta.cleared_routes.push(id)}}
fn unlink_convoy_list(m:&mut dyn Mem,id:u16,head:u32,count:u16)->Result<bool,String>{
 let base=m.r32(M+8);let mut link=head;let mut cur=m.r16(link);let mut seen=vec![false;usize::from(count)];while cur<count{let i=usize::from(cur);if seen[i]{return Err("cadena de convoy cíclica".into())}seen[i]=true;let addr=base+u32::from(cur)*0x3c;if cur==id{let next=m.r16(addr+8);m.w16(link,next);return Ok(true)}link=addr+8;cur=m.r16(link)}Ok(false)
}
/// 0x00513EC0, incluida la baja real de cada miembro del convoy/reserva y
/// la liberación del convoy. El llamante conserva y publica la copia atómica.
pub fn dissolve(m:&mut dyn Mem,band:u32,player:u32,pool_limit:u16)->Result<DissolveDelta,String>{
 let mut delta=DissolveDelta::default();let plan=m.r32(band);if plan!=0{delta.removed_plan=true;m.w32(band,0)}m.w32(band+4,0);m.w16(band+8,0);m.w16(band+0xa,0);m.w16(band+0x16,0);
 let count=m.r16(M+0xf4);let base=m.r32(M+4);let mut id=m.r16(band+0xc);let mut seen=vec![false;usize::from(count)];while id<count{let i=usize::from(id);if seen[i]{return Err("cadena de barcos piratas cíclica".into())}seen[i]=true;let s=base+u32::from(id)*0x180;let next=m.r16(s+4);let flags=m.r16(s+0x3c)|0x100;m.w16(s+0x3c,flags);m.w32(s+0x18,0);id=next}
 let cv=m.r16(band+0x10);let total=m.r16(M+0xf6);if cv<total{let addr=m.r32(M+8)+u32::from(cv)*0x3c;let mut id=m.r16(addr+0xa);m.w16(addr+0xa,0xffff);let mut seen=vec![false;usize::from(count)];while id<count{let i=usize::from(id);if seen[i]{return Err("cadena de convoy pirata cíclica".into())}seen[i]=true;let next=m.r16(base+u32::from(id)*0x180+6);merge(&mut delta,remove(m,id,player,pool_limit)?);id=next}
  unlink_convoy_list(m,cv,M+0xec,total)?;let mut f=m.r16(M+0xe4);let mut seen=vec![false;usize::from(total)];let mut found=false;while f<total{let i=usize::from(f);if seen[i]{return Err("lista libre de convoyes cíclica".into())}seen[i]=true;if f==cv{found=true;break}let cvbase=m.r32(M+8);f=m.r16(cvbase+u32::from(f)*0x3c+8)}if !found{let free=m.r16(M+0xe4);m.w16(addr+8,free);m.w16(M+0xe4,cv)}m.w16(addr+0x12,0xff);m.w16(band+0x10,0xffff);
 }
 let mut seen=vec![false;usize::from(count)];while m.r16(band+0x12)<count{let id=m.r16(band+0x12);let i=usize::from(id);if seen[i]{return Err("reserva pirata cíclica".into())}seen[i]=true;let s=base+u32::from(id)*0x180;let next=m.r16(s+6);m.w16(band+0x12,next);m.w16(s+6,0xffff);merge(&mut delta,remove(m,id,player,pool_limit)?)}
 let total=m.r16(0x6de4aa);let mer=m.r32(0x6de518);let city=m.r8(band+0xe);for i in 0..total{let a=mer+u32::from(i)*0x650;if m.r8(a+0xe1)==city{m.w8(a+0xe1,255);m.w8(a+0xe0,255)}}m.w8(band+0xe,255);m.w8(band+0xf,255);m.w8(band+0x14,255);Ok(delta)
}
