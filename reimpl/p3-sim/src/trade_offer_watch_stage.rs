//! Adaptador transaccional de 0x13 a las imágenes vivas de navegación y S04.
use crate::navigation::{self,LiveVoyage};
use p3_core::{ai::reputation::Merchant,economy::trade::OfficeStock,scheduler::TaskRecord,town::Town};
use p3_state::trade_offer_watch::{self,State};

pub struct Inputs<'a>{pub task:&'a TaskRecord,pub nav:Option<&'a LiveVoyage>,pub offers:&'a [[u8;16]],pub pool:&'a [u8],pub merchants:&'a [Merchant],pub towns:&'a [Town],pub office_images:&'a [Vec<u8>],pub stocks:&'a [OfficeStock]}
pub struct Prepared {pub task:TaskRecord,pub nav:Option<LiveVoyage>,pub offers:Vec<[u8;16]>,pub pool:Vec<u8>,pub merchants:Vec<Merchant>,pub office_images:Vec<Vec<u8>>,pub stocks:Vec<OfficeStock>,pub keep:bool}
fn u16at(b:&[u8],o:usize)->Result<u16,String>{let s=b.get(o..o+2).ok_or("imagen naval truncada")?;Ok(u16::from_le_bytes([s[0],s[1]]))}
/// Las entradas permanecen intactas incluso si falla una lista o el pool.
/// El llamante publica todas las vistas y el registro del scheduler juntos.
pub fn prepare(i:Inputs<'_>)->Result<Prepared,String>{
 if i.office_images.len()!=i.stocks.len(){return Err("oficinas de oferta desincronizadas".into())}
 let mut offices=i.office_images.to_vec();for (image,stock) in offices.iter_mut().zip(i.stocks){if image.len()!=0x44c{return Err("oficina de oferta truncada".into())}for (ware,qty) in stock.stock.iter().enumerate(){image[4+4*ware..8+4*ware].copy_from_slice(&qty.to_le_bytes())}}
 let (ships,convoys,docked)=if let Some(nav)=i.nav{let n=usize::from(u16at(&nav.snapshot.manager,0xf4)?);let c=usize::from(u16at(&nav.snapshot.manager,0xf6)?);if nav.snapshot.ships.len()!=n*0x180||nav.snapshot.convoys.len()!=c*0x3c{return Err("pool naval de oferta incompleto".into())}(nav.snapshot.ships.chunks_exact(0x180).map(|v|v.to_vec()).collect(),nav.snapshot.convoys.chunks_exact(0x3c).map(|v|v.to_vec()).collect(),u16at(&nav.snapshot.manager,0xe8)?)}else{(Vec::new(),Vec::new(),0xffff)};
 let state=State{offers:i.offers.to_vec(),pool:i.pool.to_vec(),merchants:i.merchants.iter().map(|m|m.bytes.clone()).collect(),towns:i.towns.iter().map(|t|t.0.to_vec()).collect(),offices,ships,convoys,docked};let result=trade_offer_watch::prepare(i.task,&state)?;
 // Solo la cancelación efectiva puede necesitar la flota. El vigilante de
 // una oferta intacta u otra clave no debe depender de ella.
 if i.nav.is_none() && result.state.offers.len()<state.offers.len(){
  let key=i.task.u32_at(8);let seller=i.task.u32_at(0x10);let buyer=i.task.u32_at(0x14);
  let offer=state.offers.iter().find(|v|u32::from_le_bytes(v[..4].try_into().unwrap())==key&&u32::from(v[8])==seller&&u32::from(v[9])==buyer).ok_or("oferta cancelada ausente")?;
  let mut needs_fleet=offer[11]==1;
  if offer[11]==0 && u16::from_le_bytes([offer[12],offer[13]])<24 && (seller as usize)<state.merchants.len(){
   let mut found=false;if let Some(t)=state.towns.get(usize::from(offer[10])){let mut at=usize::from(u16at(t,0x784)?);let mut seen=vec![false;state.offices.len()];while at<state.offices.len(){if seen[at]{return Err("ciclo en oficinas de oferta".into())}seen[at]=true;let image=&state.offices[at];if usize::from(u16at(image,0x2c4)?)==seller as usize{found=true;break}at=usize::from(u16at(image,0x2ca)?);}}needs_fleet=!found;
  }
  if needs_fleet{return Err("flota ausente para devolución de oferta".into())}
 }
 let mut nav=i.nav.cloned();if let Some(n)=&mut nav{n.snapshot.ships=result.state.ships.concat();n.snapshot.convoys=result.state.convoys.concat();n.snapshot.manager[0xe8..0xea].copy_from_slice(&result.state.docked.to_le_bytes());navigation::refresh_lists(&mut n.snapshot)?;}
 let mut stocks=i.stocks.to_vec();for (stock,image) in stocks.iter_mut().zip(&result.state.offices){for ware in 0..24{stock.stock[ware]=i32::from_le_bytes(image[4+4*ware..8+4*ware].try_into().unwrap())}}
 Ok(Prepared{task:result.task,nav,offers:result.state.offers,pool:result.state.pool,merchants:result.state.merchants.into_iter().map(|bytes|Merchant{bytes}).collect(),office_images:result.state.offices,stocks,keep:result.keep})
}

#[cfg(test)] mod tests {
 use super::*;
 #[test] fn gold_refund_and_error_are_transactional(){
  let mut task=TaskRecord{due:10,next:0xffff,kind:0x13,payload:[0;16]};task.set_u32_at(8,17);task.set_u32_at(0x10,0);task.set_u32_at(0x14,1);
  let mut offer=[0u8;16];offer[..4].copy_from_slice(&17u32.to_le_bytes());offer[9]=1;offer[11]=2;offer[12..].copy_from_slice(&500u32.to_le_bytes());let offers=[offer];
  let mut m=vec![Merchant{bytes:vec![0;0x650]};2];for v in &mut m{v.set_u16(10,0xffff)}let mut pool=vec![0u8;12];pool[..4].copy_from_slice(&0xaffe0005u32.to_le_bytes());pool[6..10].fill(255);
  let input=||Inputs{task:&task,nav:None,offers:&offers,pool:&pool,merchants:&m,towns:&[],office_images:&[],stocks:&[]};let out=prepare(input()).unwrap();assert!(!out.keep);assert!(out.offers.is_empty());assert_eq!(out.merchants[0].i32(0),500);assert_eq!(m[0].i32(0),0);assert_eq!(out.pool,pool);
  let broken=[1,2,3];let result=prepare(Inputs{pool:&broken,..input()});assert!(result.is_err());assert_eq!(m[0].i32(0),0);assert_eq!(offers[0],offer);
 }
}

#[cfg(test)] mod live_test {
 use super::*;use crate::Sim;use p3_core::economy::world::TownHost;
 #[test] fn scheduler_gold_offer_save_reload_and_rollback(){
  let Ok(root)=std::env::var("P3_SAVES") else{eprintln!("SKIP real save");return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();let tick=Sim::from_save_bytes(&raw).unwrap().idle.clock.tick;
  for i in 0..2{let mut m=p3_state::office::PersistedMerchant::from_save_v85(&model.world.merchants[i]).unwrap();m.merchant.set_u16(10,0xffff);model.world.merchants[i]=m.to_save_v85()}
  let mut pool=vec![0u8;12+4*16];pool[..4].copy_from_slice(&0xaffe0005u32.to_le_bytes());pool[4..6].copy_from_slice(&4u16.to_le_bytes());pool[8..10].fill(255);for i in 0..4{let p=12+i*16;pool[p+4]=255;pool[p+6..p+8].copy_from_slice(&((i+1)as u16).to_le_bytes())}model.message_pool=pool;
  let mut offer=[0u8;16];offer[..4].copy_from_slice(&0xdeadbeefu32.to_le_bytes());offer[9]=1;offer[11]=2;offer[12..].copy_from_slice(&1234u32.to_le_bytes());p3_save::world_rest::replace_offers(&mut model.world.rest,&[],&[offer]).unwrap();
  let mut task=TaskRecord{due:tick,next:0xffff,kind:0x13,payload:[0;16]};task.set_u32_at(8,0xdeadbeef);task.set_u32_at(0x10,0);task.set_u32_at(0x14,1);let record=[task.due.to_le_bytes().as_slice(),task.next.to_le_bytes().as_slice(),task.kind.to_le_bytes().as_slice(),task.payload.as_slice()].concat();model.scheduled_tasks=[0u16.to_le_bytes().as_slice(),u16::MAX.to_le_bytes().as_slice(),1u16.to_le_bytes().as_slice(),record.as_slice()].concat();
  let mut sim=Sim::from_save_bytes(&model.write()).unwrap();let before=sim.rep_merchants[0].i32(0);let actual_gold=before.wrapping_add(777);let actual_income=sim.rep_merchants[0].i32(0x4c8).wrapping_add(333);let other_gold=sim.rep_merchants[1].i32(0).wrapping_add(111);let other_income=sim.rep_merchants[1].i32(0x4c8).wrapping_add(222);sim.economy.world.merchants[0].gold=actual_gold;sim.economy.world.merchants[0].income=actual_income;sim.economy.world.merchants[1].gold=other_gold;sim.economy.world.merchants[1].income=other_income;assert_eq!(sim.host.trade_offers.as_ref().unwrap().len(),1);
  {let mut bridge=p3_state::office::OfficeTownHost{offices:&mut sim.persisted,merchants:&mut sim.rep_merchants,businesses:&mut sim.businesses,captain_wages:&mut sim.wages,difficulty:sim.commercial_sell_factor,other:&mut sim.host,completed:Vec::new(),errors:Vec::new()};bridge.dispatch_scheduled_tasks_fleet(&mut sim.economy.world,&mut sim.economy.merchants,&[],&mut sim.economy.offices,&mut sim.economy.cargo_ships,2.0,tick);assert!(bridge.errors.is_empty(),"{:?}",bridge.errors)}
  assert!(sim.host.council_errors.is_empty(),"{:?}",sim.host.council_errors);assert_eq!(sim.rep_merchants[0].i32(0),actual_gold.wrapping_add(1234));assert_eq!(sim.rep_merchants[0].i32(0x4c8),actual_income);assert_eq!(sim.rep_merchants[1].i32(0),other_gold);assert_eq!(sim.rep_merchants[1].i32(0x4c8),other_income);assert_eq!(sim.economy.world.merchants[1].gold,other_gold);assert_eq!(sim.economy.world.merchants[1].income,other_income);assert!(sim.host.trade_offers.as_ref().unwrap().is_empty());
  sim.persist_all(&mut model).unwrap();let saved=model.write();assert_eq!(model.write(),saved);let reloaded=Sim::from_save_bytes(&saved).unwrap();assert!(reloaded.host.trade_offers.as_ref().unwrap().is_empty());assert_eq!(reloaded.rep_merchants[0].i32(0),actual_gold.wrapping_add(1234));assert_eq!(reloaded.rep_merchants[0].i32(0x4c8),actual_income);assert_eq!(reloaded.rep_merchants[1].i32(0),other_gold);assert_eq!(reloaded.rep_merchants[1].i32(0x4c8),other_income);let previous=model.clone();reloaded.persist_all(&mut model).unwrap();assert_eq!(model.world.rest,previous.world.rest);assert_eq!(model.world.merchants[0],previous.world.merchants[0]);assert_eq!(model.message_pool,previous.message_pool);assert_eq!(model.scheduled_tasks,previous.scheduled_tasks);
 }
}

#[cfg(test)] mod fleet_test {
 use super::*;use p3_save::ships::{NavigationLists,NavigationSnapshot,SavedRoute};
 #[test] fn refunded_ship_is_linked_in_live_docked_list(){
  let mut manager=vec![0u8;0x110];manager[0xe8..0xec].fill(255);manager[0xf4..0xf6].copy_from_slice(&1u16.to_le_bytes());let mut ship=vec![0u8;0x180];ship[6..10].fill(255);ship[0x134..0x136].copy_from_slice(&0x16u16.to_le_bytes());let snapshot=NavigationSnapshot{manager,ships:ship,captains:vec![],convoys:vec![],routes:vec![Some(SavedRoute::None)],lists:NavigationLists{docked_head:0xffff,sea_head:0xffff,docked:vec![],sea:vec![]}};let nav=LiveVoyage{additional:vec![],baseline:snapshot.clone(),snapshot,ship:0xffff,convoy:None,points:vec![],ports:vec![],player:0,events:vec![]};
  let mut task=TaskRecord{due:10,next:0xffff,kind:0x13,payload:[0;16]};task.set_u32_at(8,17);task.set_u32_at(0x14,1);let mut offer=[0u8;16];offer[..4].copy_from_slice(&17u32.to_le_bytes());offer[9]=1;offer[11]=1;
  let mut merchants=vec![Merchant{bytes:vec![0;0x650]};2];for m in &mut merchants{m.set_u16(10,0xffff)}let mut pool=vec![0u8;12];pool[..4].copy_from_slice(&0xaffe0005u32.to_le_bytes());pool[6..10].fill(255);
  let out=prepare(Inputs{task:&task,nav:Some(&nav),offers:&[offer],pool:&pool,merchants:&merchants,towns:&[],office_images:&[],stocks:&[]}).unwrap();let result=out.nav.unwrap();assert_eq!(result.snapshot.lists.docked,vec![0]);assert_eq!(u16at(&result.snapshot.ships,0x134).unwrap(),0);assert_eq!(nav.snapshot.lists.docked,Vec::<u16>::new());
 }
}


#[cfg(test)] mod successive_test {
 use crate::Sim;
 #[test] fn successive_offer_saves_accept_own_versions_and_reject_conflicts(){
  let Ok(root)=std::env::var("P3_SAVES") else{eprintln!("SKIP real save");return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();let mut a=[0u8;16];let mut b=[0u8;16];a[0]=1;b[0]=2;p3_save::world_rest::replace_offers(&mut model.world.rest,&[],&[a,b]).unwrap();let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.trade_offers=Some(vec![b]);sim.persist_all(&mut model).unwrap();let mut first=model.clone();assert_eq!(p3_save::world_rest::WorldRest::parse(&model.world.rest).unwrap().offer_records(),vec![b]);sim.host.trade_offers=Some(vec![]);let mut external=first.clone();let range=p3_save::world_rest::WorldRest::parse(&external.world.rest).unwrap().offers.clone();external.world.rest[range.start+17..range.start+19].copy_from_slice(&1u16.to_le_bytes());external.world.rest.insert(range.start+19,0x99);assert_eq!(p3_save::world_rest::WorldRest::parse(&external.world.rest).unwrap().offer_records(),vec![b]);let original=external.clone();assert!(sim.persist_all(&mut external).is_err());assert_eq!(external.world.rest,original.world.rest);sim.persist_all(&mut model).unwrap();assert!(p3_save::world_rest::WorldRest::parse(&model.world.rest).unwrap().offer_records().is_empty());sim.persist_all(&mut first).unwrap();assert!(p3_save::world_rest::WorldRest::parse(&first.world.rest).unwrap().offer_records().is_empty());
  let mut alien=model.clone();let foreign=[[9u8;16]];p3_save::world_rest::replace_offers(&mut alien.world.rest,&[],&foreign).unwrap();let old=alien.clone();assert!(sim.persist_all(&mut alien).is_err());assert_eq!(alien.world.rest,old.world.rest);assert_eq!(alien.scheduled_tasks,old.scheduled_tasks);
 }
}


#[cfg(test)] mod no_fleet_tests {
 use super::*;
 #[test]fn requires_fleet_only_for_actual_ship_dependent_refund(){
  let mut task=TaskRecord{due:10,next:0xffff,kind:0x13,payload:[0;16]};task.set_u32_at(8,17);task.set_u32_at(0x14,1);let mut merchants=vec![Merchant{bytes:vec![0;0x650]};2];let mut pool=vec![0u8;12+2*16];pool[..4].copy_from_slice(&0xaffe0005u32.to_le_bytes());pool[4..6].copy_from_slice(&2u16.to_le_bytes());pool[6..10].fill(255);for (i,m) in merchants.iter_mut().enumerate(){m.set_u16(10,i as u16);let p=12+i*16;pool[p+4]=0x1e+i as u8;pool[p+6..p+8].fill(255);pool[p+8..p+12].copy_from_slice(&17u32.to_le_bytes());pool[p+14..p+16].copy_from_slice(&1u16.to_le_bytes())}
  let mut ship_offer=[0u8;16];ship_offer[..4].copy_from_slice(&17u32.to_le_bytes());ship_offer[9]=1;ship_offer[11]=1;let input=|offers:&[[u8;16]],pool:&[u8],merchants:&[Merchant]|prepare(Inputs{task:&task,nav:None,offers,pool,merchants,towns:&[],office_images:&[],stocks:&[]});let result=input(&[ship_offer],&pool,&merchants).unwrap();assert!(result.keep);assert_eq!(result.task.due,74);assert_eq!(result.offers,vec![ship_offer]);
  let mut other=ship_offer;other[0]=18;let result=input(&[other],&pool,&merchants).unwrap();assert!(!result.keep);assert_eq!(result.offers,vec![other]);
  merchants[1].set_u16(10,0xffff);let result=input(&[ship_offer],&pool,&merchants);assert!(matches!(result,Err(ref e) if e=="flota ausente para devolución de oferta"));
  let mut town=Town::from_bytes(&vec![0;0x9f8]);town.set_u16(0x784,0);let mut office=vec![0u8;0x44c];office[0x2ca..0x2cc].fill(255);let mut goods=ship_offer;goods[11]=0;goods[12..].copy_from_slice(&((3u32<<16)|3).to_le_bytes());let offices=[office];let stocks=[OfficeStock::default()];let towns=[town];let result=prepare(Inputs{task:&task,nav:None,offers:&[goods],pool:&pool,merchants:&merchants,towns:&towns,office_images:&offices,stocks:&stocks}).unwrap();assert_eq!(result.stocks[0].stock[3],600);assert!(result.offers.is_empty());
 }
}
