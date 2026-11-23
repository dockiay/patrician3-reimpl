//! Inicio de obras de pozo, negocios y vivienda sobre copias, con materiales y pagos.
use p3_core::{ai::reputation::RepEnv,buildings::construction::{BuildCtx,BuildHost,Hook},economy::prices::{buy_price,Market},pump::Operation,town::{off,Town}};
use crate::{office_take,wall_work::WallState};

struct Host<'a> { state:&'a mut WallState, town:Town, order:Vec<usize>, current:usize, office:Option<usize>, changed:Vec<usize>, error:Option<String> }
impl Host<'_> {
 fn fail(&mut self,e:impl Into<String>){if self.error.is_none(){self.error=Some(e.into())}}
 fn head(&self)->u16{let b=&self.state.merchants[self.current].bytes;u16::from_le_bytes([b[14],b[15]])}
}
impl BuildHost for Host<'_> {
 fn call(&mut self,hook:Hook,args:&[u32])->u32 {
  match hook {
   Hook::FindOrCreateOffice=>{self.office=self.order.iter().copied().find(|&i|u32::from(self.state.stocks[i].owner)==args[0]);self.office.map_or(0,|i|i as u32+1)},
   Hook::GetMerchant=>0,
   Hook::TakeWaresFromOffice=>{
    let i=self.office.unwrap();let head=self.head();
    match office_take::take(&mut self.state.stocks[i],&mut self.state.cargo,&self.state.links,&mut self.state.convoys,head,args[0] as usize,args[1] as i32){
     Ok((left,changed))=>{self.changed.extend(changed);left as u32},Err(e)=>{self.fail(e);args[1]}
    }
   },
   Hook::TakeWaresFromShips=>{
    let head=self.head();match office_take::take_ships(&mut self.state.cargo,&self.state.links,head,args[1] as u8,args[0] as usize,args[2] as i32){
     Ok((left,changed))=>{self.changed.extend(changed);left as u32},Err(e)=>{self.fail(e);args[2]}
    }
   },
   Hook::GetBuyPrice=>{let w=args[0] as usize;let t=&self.town;buy_price(w,&Market{trading:t.more_flags()&0x100!=0,stock:t.stock(w),thresholds:std::array::from_fn(|i|t.threshold(w,i)),marked_ware:t.u8(0x6f0)},args[2] as i32) as u32},
   // Objetos de mapa y barra de progreso: solo dibujo del original; la sesión los deriva de los registros del plano.
   Hook::AddMapObject|Hook::SetBuildingProgress=>0,
   Hook::ReputationHospital=>{
    let index=args[0] as usize;let mut towns=vec![Town::default();index+1];towns[index]=self.town.clone();
    let env=RepEnv{town_count:towns.len() as u32,towns:&towns,renown:[0;3],byte_52c:0};
    self.state.merchants[self.current].add_buildings_reputation_hospital_chapel(&env,args[0],args[1]);0
   },
   Hook::ReputationCivic=>{self.state.merchants[self.current].add_buildings_reputation_civic_building(args[0]);0},
   _=>{self.fail(format!("dependencia de pozo no admitida: {hook:?}"));0}
  }
 }
 fn office_building_head(&mut self,office:u32)->u16{
  match office.checked_sub(1).and_then(|i|self.state.offices.get(i as usize)){Some(o)=>o.head,None=>{self.fail("consulta de edificio inesperada");u16::MAX}}
 }
 fn reputation_well(&mut self,town:&Town,owner:u32){
  let index=owner as usize;let mut towns=vec![Town::default();index+1];towns[index]=town.clone();
  let env=RepEnv{town_count:towns.len() as u32,towns:&towns,renown:[0;3],byte_52c:0};
  self.state.merchants[self.current].add_buildings_reputation_well(&env,owner);
 }
}
/// Ejecuta el inicio de una obra de forma transaccional; el llamante publica sus efectos.
pub fn execute(input:&WallState,op:&Operation)->Result<(WallState,Vec<usize>,bool),String>{
 let b=op.0[1].to_le_bytes();let ty=op.0[2].to_le_bytes();
 crate::lender::validate(&input.town,&input.site,input.merchant_count)?;
 if !matches!(op.0[0],0x22|0x23)||ty[0]>=0x30||b[1]!=input.town.index()||i32::from(b[2])>=input.town.i32(off::PLAN_ROWS)||i32::from(b[3])>=input.town.i32(off::PLAN_STRIDE){return Err("operación de pozo incompatible".into())}
 if input.cargo.len()!=input.links.len()||input.offices.len()!=input.stocks.len()||usize::from(input.office_count)>input.offices.len()||usize::from(input.merchant_count)>input.merchants.len(){return Err("estado de pozo incompleto".into())}
 let mut order=Vec::new();let mut seen=vec![false;usize::from(input.office_count)];let mut p=input.town.u16(off::FIRST_OFFICE);
 while p<input.office_count{let i=usize::from(p);if seen[i]{return Err("cadena de oficinas cíclica".into())}seen[i]=true;order.push(i);p=input.offices[i].next;}
 let mut state=input.clone();let current=usize::from(b[0]);let private=current<usize::from(input.merchant_count);
 if private && state.merchants[current].bytes.len()<0x650{return Err("comerciante truncado".into())}
 let (mut gold,mut income)=if private{(state.merchants[current].i32(0),state.merchants[current].i32(0x4c0))}else{(0,0)};
 let mut site=state.site.clone();let mut town=state.town.clone();let mut host=Host{town:town.clone(),state:&mut state,order,current,office:None,changed:vec![],error:None};let classes=[0;256];
 let mut ctx=BuildCtx{merchant_count:input.merchant_count,visual_owner:None,special:0xe0,owner_class:&classes,merchant_gold:&mut gold,merchant_income:&mut income,host:&mut host};
 let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||site.start_building_construction(&mut town,&mut ctx,b[0],b[2],b[3],ty[0],u32::from(ty[1])))).map_err(|_|"obra de pozo fuera del plano o pool")?;
 let Host{error,mut changed,..}=host;if let Some(e)=error{return Err(e)}
 if private{state.merchants[current].set_i32(0,gold);state.merchants[current].set_i32(0x4c0,income)}
 state.town=town;state.site=site;changed.sort_unstable();changed.dedup();Ok((state,changed,result))
}
