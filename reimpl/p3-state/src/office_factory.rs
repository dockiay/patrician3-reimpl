//! Reserva de oficina y carta global en una transacción S05.
use p3_core::{ai::reputation::Merchant,calendar::Calendar,economy::office_factory::find_or_create};
use p3_save::{layout,model::SaveModel,office_factory,world::World};

/// Aplica el resultado solo si oficina, enlaces y carta pueden guardarse juntos.
pub fn create(model:&mut SaveModel,owner:u16,city:u16,tick:u32,show_human:bool)->Result<Option<u16>,String>{
 let mut staged=model.clone();let bytes=staged.world.write();let world=World::parse(&bytes).map_err(|e|format!("mundo: {e:?}"))?;
 let old_towns=(0..world.town_count()).map(|i|world.town(i)).collect::<Vec<_>>();let mut towns=old_towns.clone();
 let old_merchants=staged.world.merchants.iter().map(|r|Merchant{bytes:layout::image(&layout::MERCHANT,0x650,r)}).collect::<Vec<_>>();let mut merchants=old_merchants.clone();
 let old_pool=office_factory::pool_from_world(&staged.world)?;let mut pool=old_pool.clone();let count=u16::try_from(merchants.len()).map_err(|_|"demasiados comerciantes")?;
 let outcome=find_or_create(&mut pool,&mut towns,&mut merchants,count,owner,city,true,tick,show_human).map_err(str::to_string)?;
 office_factory::apply(&mut staged.world,&old_pool,&pool,&old_towns,&towns,&old_merchants,&merchants)?;
 if let Some(letter)=outcome.letter {let date=Calendar::for_tick(tick);crate::messages::add_global_inline_letter(&mut staged.message_pool,date.day,date.month,date.year,&letter)?;}
 *model=staged;Ok(outcome.index)
}
