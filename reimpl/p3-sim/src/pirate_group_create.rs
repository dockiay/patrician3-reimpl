//! Alta real 0x00516170 de un barco de reserva pirata.
use p3_core::{flatmem::Mem,town::Town,ships::{factory,cargo},economy::pirate_group_callbacks::{self as callbacks,CaptainResources}};
/// Recursos no serializados y posición del pueblo de la banda.
pub struct CreateResources<'a>{pub town0:&'a Town,pub port0:(u16,u16),pub band_port:(u16,u16),pub weapon_experience:u32,pub ship_names:&'a [Vec<u8>],pub captain:CaptainResources}
/// Debe ejecutarse sobre una copia transaccional de memoria; el llamante
/// publica pool, contador, listas y datos de la banda solamente tras éxito.
pub fn create(m:&mut dyn Mem,band:u32,kind:u32,a1:u32,desired_power:u32,r:&CreateResources<'_>)->Result<u32,String>{
 if kind>=4||r.ship_names.is_empty(){return Err("tipo o nombres de barco pirata ausentes".into())}
 let (addr,id)=callbacks::allocate_reserve_ship(m,r.captain.tick,r.port0)?;
 let first=m.r16(band+0xc);let head=m.r16(band+0x12);m.w16(addr+4,first);m.w16(band+0xc,id);m.w16(addr+6,head);m.w16(band+0x12,id);
 let mut ship=[0u8;0x180];for(i,x)in ship.iter_mut().enumerate(){*x=m.r8(addr+i as u32)}
 factory::init_ship_stats(r.town0,&mut ship,kind as u8).ok_or("estadísticas de barco pirata inválidas")?;
 factory::recompute_weapon_slots(&mut ship).ok_or("ranuras piratas inválidas")?;
 let mut cursor=m.r16(0x6dd88e);let step=m.r16(0x6dd890);let name=factory::next_ship_name_index(&mut cursor,step,r.ship_names.len()).ok_or("banco de nombres piratas inválido")?;m.w16(0x6dd88e,cursor);ship[0x15e..0x160].copy_from_slice(&name.to_le_bytes());
 let name=&r.ship_names[usize::from(name)];let len=name.iter().position(|&v|v==0).unwrap_or(name.len()).min(31);ship[0x160..0x160+len].copy_from_slice(&name[..len]);ship[0x160+len]=0;ship[0x3c]|=1;
 crate::absorb_ship_refresh::refresh(&mut ship,1).map_err(str::to_owned)?;
 callbacks::arm_ship(&mut ship,desired_power&0xffff,callbacks::town_weapon_level(r.weapon_experience))?;
 let complement=factory::additional_sailors(&mut ship).ok_or("capacidad de tripulación inválida")? as u16;ship[0x40..0x42].copy_from_slice(&complement.to_le_bytes());ship[0x154..0x156].copy_from_slice(&complement.to_le_bytes());cargo::recompute_load(&mut ship);
 ship[0]=255;ship[0x13a]=0;let max=ship[0x14..0x18].to_vec();ship[0x18..0x1c].copy_from_slice(&max);ship[0x1e..0x20].copy_from_slice(&r.band_port.0.to_le_bytes());ship[0x22..0x24].copy_from_slice(&r.band_port.1.to_le_bytes());
 if a1 as u8!=0{let cap=callbacks::allocate_pirate_captain(m,r.captain)?;ship[0x42..0x44].copy_from_slice(&cap.to_le_bytes())}
 for(i,&v)in ship.iter().enumerate(){m.w8(addr+i as u32,v)}Ok(addr)
}
