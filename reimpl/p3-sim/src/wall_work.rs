//! Transacción de una operación 0x24 de obra de muralla.
use p3_core::{ai::reputation::Merchant,buildings::{Site,defpiece::{Letter,MerchantInfo,OfficeInfo,StartEnv,StartPieceHost},evaluate::{BuildRequest,EvalEnv,EvalHost,EvalShip}},economy::{militia::book_transaction,prices::{buy_price,Market},ship_trade::ShipCargo,trade::OfficeStock},town::{off,Town}};
use crate::office_take::{self,ShipLink};
use p3_core::buildings::defense::{PitchShootHost,TowerStageHost,PitchShootTables};

struct Selector<'a>{world:&'a p3_core::economy::world::EconomyWorld,offices:&'a [p3_core::economy::extra_tax::TaxOffice]}
impl p3_core::economy::wall_planner::WallPlannerHost for Selector<'_>{
    type Error=String;
    fn office(&mut self,index:u16)->Result<(u16,u16),String>{let i=usize::from(index);Ok((self.offices.get(i).ok_or("oficina ausente")?.owner,self.world.offices.get(i).ok_or("enlace de oficina ausente")?.next))}
    fn merchant_kind(&mut self,index:u16)->Result<u16,String>{Ok(self.world.merchants.get(usize::from(index)).ok_or("comerciante ausente")?.kind)}
    fn invalid_wall_record(&mut self)->String{"registro de muralla inválido".into()}
    fn invalid_office_chain(&mut self)->String{"cadena de oficinas cíclica".into()}
}
pub fn plan(town:&Town,site:&Site,world:&p3_core::economy::world::EconomyWorld,offices:&[p3_core::economy::extra_tax::TaxOffice])->Result<Vec<p3_core::pump::Operation>,String>{
    p3_core::economy::wall_planner::plan(town,site,world.merchant_count,world.office_count,&mut Selector{world,offices})
}

#[derive(Clone)]
pub struct WallState {
    pub town:Town,pub site:Site,pub offices:Vec<OfficeInfo>,pub stocks:Vec<OfficeStock>,
    pub merchants:Vec<Merchant>,pub cargo:Vec<ShipCargo>,pub links:Vec<ShipLink>,pub convoys:Vec<u32>,
    pub merchant_count:u16,pub office_count:u16,pub tick:u32,
}
pub struct WallResult {pub state:WallState,pub changed_ships:Vec<usize>,pub letters:Vec<(u32,Letter)>,pub tasks:Vec<(u32,[u8;0x20])>}

struct Evaluation<'a>{stocks:&'a [OfficeStock],office_order:&'a [usize],town:&'a Town}
impl EvalHost for Evaluation<'_>{
    fn get_merchant(&mut self,_:u32){}
    fn find_office(&mut self,merchant:u32,town:u32,_:u32)->Option<[i32;4]>{
        if town!=u32::from(self.town.index()){return None}
        self.office_order.iter().map(|&i|&self.stocks[i]).find(|o|u32::from(o.owner)==merchant).map(|o|[o.stock[19],o.stock[11],o.stock[12],o.stock[17]])
    }
    fn buy_price(&mut self,ware:u32,_:u32,qty:u32)->i32{
        let w=ware as usize;let t=self.town;
        buy_price(w,&Market{trading:t.more_flags()&0x100!=0,stock:t.stock(w),thresholds:std::array::from_fn(|i|t.threshold(w,i)),marked_ware:t.u8(0x6f0)},qty as i32)
    }
}

struct Host<'a>{
    town:&'a Town,stocks:&'a mut Vec<OfficeStock>,office_order:&'a [usize],merchants:&'a mut Vec<Merchant>,cargo:&'a mut Vec<ShipCargo>,links:&'a [ShipLink],convoys:&'a mut Vec<u32>,merchant_count:u16,
    current:Option<usize>,error:Option<String>,changed:Vec<usize>,books:Vec<(i32,u32)>,letters:Vec<(u32,Letter)>,tasks:Vec<(u32,[u8;0x20])>,
}
impl Host<'_>{
    fn fail(&mut self,s:&str){if self.error.is_none(){self.error=Some(s.into())}}
    fn head(&self,i:usize)->Option<u16>{let b=&self.merchants.get(i)?.bytes;(b.len()>=0x10).then(||u16::from_le_bytes([b[0xe],b[0xf]]))}
}
impl StartPieceHost for Host<'_>{
    fn find_office(&mut self,merchant:u32,town:u32,_:u32)->Option<u32>{
        if town!=u32::from(self.town.index()){return None}
        self.office_order.iter().copied().find(|&i|u32::from(self.stocks[i].owner)==merchant).map(|i|i as u32)
    }
    fn get_merchant(&mut self,merchant:u32)->MerchantInfo{
        let i=merchant as usize;self.current=Some(i);let Some(m)=self.merchants.get(i) else{self.fail("comerciante ausente");return MerchantInfo::default()};
        if m.bytes.len()<0x4c0{self.fail("comerciante truncado");return MerchantInfo::default()}
        MerchantInfo{word8:u16::from_le_bytes([m.bytes[8],m.bytes[9]]),b1c:m.bytes[0x1c],b1d:m.bytes[0x1d]}
    }
    fn evaluate(&mut self,ty:u16,merchant:u16)->Option<BuildRequest>{
        if merchant>=self.merchant_count{return None}
        let i=usize::from(merchant);let Some(head)=self.head(i) else{self.fail("comerciante truncado");return None};
        let Some(m)=self.merchants.get(i) else{self.fail("comerciante ausente");return None};
        let ships=self.cargo.iter().zip(self.links).map(|(s,l)|EvalShip{town:s.town,next:l.next,state:l.state,wares:[s.stock[19],s.stock[11],s.stock[12],s.stock[17]]}).collect::<Vec<_>>();
        let mut pos=head;let mut seen=vec![false;ships.len()];while usize::from(pos)<ships.len(){let k=usize::from(pos);if seen[k]{self.fail("ciclo en lista de barcos");return None}seen[k]=true;pos=ships[k].next;}
        let mut request=BuildRequest{ty,merchant,..Default::default()};let mut bridge=Evaluation{stocks:self.stocks,office_order:self.office_order,town:self.town};
        let mut env=EvalEnv{merchant_count:self.merchant_count,ship_count:ships.len() as u16,ship_head:head,merchant_gold:m.i32(0),mayor:self.town.u8(off::MAYOR_ID),ships:&ships,host:&mut bridge};
        Site::evaluate_build_request(self.town,&mut request,&mut env).then_some(request)
    }
    fn take_ware(&mut self,office:u32,ware:u32,quantity:i32)->i32{
        let i=office as usize;let Some(owner)=self.stocks.get(i).map(|s|usize::from(s.owner)) else{self.fail("oficina ausente");return quantity};
        let Some(head)=self.head(owner) else{self.fail("comerciante truncado");return quantity};
        match office_take::take(&mut self.stocks[i],self.cargo,self.links,self.convoys,head,ware as usize,quantity){
            Ok((left,changed))=>{for i in changed{if !self.changed.contains(&i){self.changed.push(i)}}left},Err(e)=>{self.fail(&e);quantity}
        }
    }
    fn merchant_credit(&mut self,amount:i32){if let Some(m)=self.current.and_then(|i|self.merchants.get_mut(i)){let gold=m.i32(0).wrapping_add(amount);let total=m.i32(0x49c).wrapping_add(amount);m.set_i32(0,gold);m.set_i32(0x49c,total)}else{self.fail("comerciante de pago ausente")}}
    fn merchant_add_4bc(&mut self,amount:i32){if let Some(m)=self.current.and_then(|i|self.merchants.get_mut(i)){let v=m.i32(0x4bc).wrapping_add(amount);m.set_i32(0x4bc,v)}else{self.fail("comerciante de devolución ausente")}}
    fn book_transaction(&mut self,amount:i32,kind:u32){self.books.push((amount,kind))}
    fn add_map_object(&mut self,_:u32,_:u32,_:u32,_:u32,_:u32){self.fail("vista visual no admitida")}
    fn set_progress(&mut self,_:u32,_:u32,_:u32,_:u32,_:u32){self.fail("vista visual no admitida")}
    fn remove_building(&mut self,_:u32,_:u32){self.fail("vista visual no admitida")}
    fn add_message(&mut self,merchant:u32,letter:&Letter){self.letters.push((merchant,*letter))}
    fn add_task(&mut self,time:u32,task:&[u8;0x20]){self.tasks.push((time,*task))}
}

impl PitchShootHost for Host<'_>{
 fn find_office(&mut self,merchant:u32,town:u32,create:u32)->Option<u32>{StartPieceHost::find_office(self,merchant,town,create)}
 fn take_ware(&mut self,office:u32,ware:u32,quantity:i32)->i32{StartPieceHost::take_ware(self,office,ware,quantity)}
 fn buy_price(&mut self,ware:u32,_owner:u32,quantity:i32)->i32{let w=ware as usize;let t=self.town;buy_price(w,&Market{trading:t.more_flags()&0x100!=0,stock:t.stock(w),thresholds:std::array::from_fn(|i|t.threshold(w,i)),marked_ware:t.u8(0x6f0)},quantity)}
 fn buy_price_at(&mut self,t:&Town,ware:u32,_owner:u32,quantity:i32)->i32{let w=ware as usize;buy_price(w,&Market{trading:t.more_flags()&0x100!=0,stock:t.stock(w),thresholds:std::array::from_fn(|i|t.threshold(w,i)),marked_ware:t.u8(0x6f0)},quantity)}
 fn book_transaction(&mut self,amount:i32,kind:u32){self.books.push((amount,kind))}
 fn add_map_object(&mut self,_:u32,_:u32,_:u32,_:u32,_:u32){self.fail("vista visual no admitida")}
}
impl TowerStageHost for Host<'_>{
 fn merchant_credit(&mut self,merchant:u32,amount:i32){self.current=Some(merchant as usize);StartPieceHost::merchant_credit(self,amount)}
 fn set_progress(&mut self,_:u32,_:u32,_:u32,_:u32,_:u32){self.fail("vista visual no admitida")}
}

fn chain(site:&Site,head:u16,count:u16)->Result<(),String>{let mut p=head;let mut seen=vec![false;usize::from(count)];while p<count{let i=usize::from(p);if seen[i]{return Err("cadena de edificios cíclica".into())}seen[i]=true;let at=i*8;if at+8>site.records.len(){return Err("registro de edificio truncado".into())}p=u16::from_le_bytes([site.records[at+4],site.records[at+5]]);}Ok(())}

/// Ejecuta sobre copias y devuelve efectos para publicarlos con la cola y S05.
pub fn execute(input:&WallState,merchant:u32,mode:u32)->Result<WallResult,String>{
    execute_inner(input,merchant,mode,None)
}
pub fn execute_tower(input:&WallState,op:&p3_core::pump::Operation)->Result<WallResult,String>{
    let b=op.0[1].to_le_bytes();let ty=op.0[2].to_le_bytes()[0];
    if op.0[0]!=0x22||b[1]!=input.town.index()||!(0x30..=0x35).contains(&ty)||i32::from(b[2])>=input.town.i32(off::PLAN_ROWS)||i32::from(b[3])>=input.town.i32(off::PLAN_STRIDE){return Err("operación de defensa incompatible".into())}
    execute_inner(input,u32::from(b[0]),0,Some(*op))
}
fn execute_inner(input:&WallState,merchant:u32,mode:u32,tower:Option<p3_core::pump::Operation>)->Result<WallResult,String>{
    if input.cargo.len()!=input.links.len() || input.cargo.len()>u16::MAX as usize || usize::from(input.merchant_count)>input.merchants.len() || usize::from(input.office_count)>input.offices.len() || input.offices.len()!=input.stocks.len(){return Err("estado de muralla incompleto".into())}
    if input.merchants.iter().any(|m|m.bytes.len()<0x4cc){return Err("comerciante de muralla truncado".into())}
    crate::lender::validate(&input.town,&input.site,input.merchant_count)?;
    let count=input.town.u16(off::BUILDING_RECORD_COUNT);
    for head in [input.town.u16(off::WALL_HEAD),input.town.u16(off::DEFENSE_HEAD),input.town.u16(off::CONSTRUCTION_HEAD)]{chain(&input.site,head,count)?}
    let mut p=input.town.u16(off::FIRST_OFFICE);let mut seen=vec![false;usize::from(input.office_count)];let mut office_order=Vec::new();while p<input.office_count {let i=usize::from(p);if seen[i]{return Err("cadena de oficinas cíclica".into())}seen[i]=true;office_order.push(i);let o=input.offices[i];chain(&input.site,o.head,count)?;chain(&input.site,o.head2,count)?;p=o.next;}
    let mut state=input.clone();let original=state.town.clone();let offices=state.offices.clone();
    let env=StartEnv{merchant_count:state.merchant_count,office_count:state.office_count,offices:&offices,visual_owner:None,tick:state.tick,special:0xe0,factors:[f32::from_bits(0x3d236e30),f32::from_bits(0x3ce147af),f32::from_bits(0x3fa39581),f32::from_bits(0x3e6147af)]};
    let mut host=Host{town:&original,stocks:&mut state.stocks,office_order:&office_order,merchants:&mut state.merchants,cargo:&mut state.cargo,links:&state.links,convoys:&mut state.convoys,merchant_count:state.merchant_count,current:None,error:None,changed:vec![],books:vec![],letters:vec![],tasks:vec![]};
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(||if let Some(op)=tower {let b=op.0[1].to_le_bytes();let ty=op.0[2].to_le_bytes()[0];let tables=PitchShootTables::default();if ty<=0x33{state.site.build_wall_tower_stage(&mut state.town,b[2],b[3],b[0],env.merchant_count,None,&tables.factors,&mut host)}else{state.site.complete_pitch_shoot(&mut state.town,b[2],b[3],b[0],env.merchant_count,None,&tables,&mut host)}}else{state.site.start_town_defense_piece(&mut state.town,&env,merchant,mode,&mut host)})).map_err(|_|"obra de muralla fuera del plano")?;
    let Host{error,books,changed,letters,tasks,..}=host;
    if let Some(error)=error{return Err(error)}
    for (amount,kind) in books {book_transaction(&mut state.town,state.tick,amount,kind)}
    Ok(WallResult{state,changed_ships:changed,letters,tasks})
}

impl crate::Sim {
    /// Aplica 0x24 y sus cartas y tareas sobre una transacción de la sesión.
    pub fn start_wall_work(&mut self,city:usize,merchant:u32,mode:u32)->Result<(),String>{
        self.start_construction_work(city,merchant,mode,None)
    }
    /// Aplica pozos, negocios y viviendas privadas, con sus cobros, en la transacción de obras.
    pub fn start_well_work(&mut self,city:usize,op:&p3_core::pump::Operation)->Result<(),String>{
        self.start_construction_work(city,u32::from(op.0[1].to_le_bytes()[0]),0,Some(op))
    }
    /// Orden de construcción del jugador: empieza la obra de `ty` en (`row`, `col`) con el mismo compromiso atómico que el pozo.
    pub fn start_building_work(&mut self,city:usize,merchant:u8,row:u8,col:u8,ty:u8,extra:u8)->Result<(),String>{
        let town=self.economy.world.towns.get(city).ok_or("ciudad de obra ausente")?.index();
        let op=p3_core::pump::Operation([0x23,u32::from_le_bytes([merchant,town,row,col]),u32::from_le_bytes([ty,extra,0,0]),0,0]);
        self.start_construction_work(city,u32::from(merchant),0,Some(&op))
    }
    pub fn start_tower_work(&mut self,city:usize,op:&p3_core::pump::Operation)->Result<(),String>{
        self.start_construction_work(city,u32::from(op.0[1].to_le_bytes()[0]),0,Some(op))
    }
    fn start_construction_work(&mut self,city:usize,merchant:u32,mode:u32,well:Option<&p3_core::pump::Operation>)->Result<(),String>{
        let town=self.economy.world.towns.get(city).ok_or("ciudad de muralla ausente")?.clone();
        let site=self.host.sites.get(city).ok_or("plano de muralla ausente")?.clone();
        let count=usize::from(self.economy.world.office_count);
        if self.economy.offices.len()<count||self.persisted.len()<count||self.host.lists.len()<count||self.host.housing.len()<count||self.economy.world.offices.len()<count{return Err("oficinas de muralla incompletas".into())}
        let mut offices=Vec::with_capacity(count);
        for i in 0..count{
            let stock=&self.economy.offices[i];let saved=&self.persisted[i].stock;
            if stock.owner<self.economy.world.merchant_count && (stock.stock!=saved.stock||stock.owner!=saved.owner||stock.town!=saved.town){return Err(format!("oficina {i} de muralla desincronizada"))}
            offices.push(OfficeInfo{next:self.economy.world.offices[i].next,head:self.host.lists[i].building_head,head2:self.host.housing[i].houses_head,merchant:stock.owner});
        }
        let ships=self.economy.cargo_ships.len();
        if self.host.expedition_ship_images.len()!=ships||self.host.navigation.as_ref().is_some_and(|n|n.snapshot.ships.len()!=ships*0x180){return Err("flota de muralla incompleta".into())}
        let mut links=Vec::with_capacity(ships);let mut present=Vec::with_capacity(ships);
        for i in 0..ships{
            if self.host.expedition_ship_images[i].as_ref().is_some_and(|x|x.len()<0x180){return Err("imagen de barco truncada".into())}
            let image=if let Some(nav)=&self.host.navigation{nav.snapshot.ships.get(i*0x180..(i+1)*0x180)}else{self.host.expedition_ship_images[i].as_deref()};
            if let Some(b)=image{
                let cargo=&self.economy.cargo_ships[i];if b.len()<0x180||cargo.town!=b[0x39]||(0..24).any(|w|i32::from_le_bytes(b[0x54+4*w..0x58+4*w].try_into().unwrap())!=cargo.stock[w]){return Err("barco de muralla desincronizado".into())}
                links.push(ShipLink{next:crate::u16_at(b,4),state:crate::u16_at(b,0x134),convoy:crate::u16_at(b,8)});present.push(true);
            }else{links.push(ShipLink{next:u16::MAX,state:u16::MAX,convoy:u16::MAX});present.push(false)}
        }
        if merchant<u32::from(self.economy.world.merchant_count){
            let m=self.rep_merchants.get(merchant as usize).ok_or("comerciante de muralla ausente")?;if m.bytes.len()<0x10{return Err("comerciante truncado".into())}
            let mut p=crate::u16_at(&m.bytes,0xe);let mut seen=vec![false;ships];while usize::from(p)<ships{let k=usize::from(p);if seen[k]||!present[k]{return Err("lista de barcos de muralla inválida".into())}seen[k]=true;p=links[k].next;}
        }
        if let Some(nav)=&self.host.navigation{
            if nav.snapshot.convoys.len()!=self.host.debt_convoy_flags.len()*0x3c{return Err("convoyes incompletos".into())}
            for (i,flag) in self.host.debt_convoy_flags.iter().enumerate(){if crate::u32_at(&nav.snapshot.convoys,i*0x3c+0x18)!=*flag{return Err("convoy de muralla desincronizado".into())}}
        }
        let input=WallState{town,site,offices,stocks:self.economy.offices[..count].to_vec(),merchants:self.rep_merchants.clone(),cargo:self.economy.cargo_ships.clone(),links,convoys:self.host.debt_convoy_flags.clone(),merchant_count:self.economy.world.merchant_count,office_count:self.economy.world.office_count,tick:self.idle.clock.tick};
        let mut out=if let Some(op)=well {if matches!(op.0[2].to_le_bytes()[0],0x04..=0x1d|0x28)||op.0[0]==0x23{let (state,changed_ships,started)=crate::well_work::execute(&input,op)?;if op.0[0]==0x23&&!started{return Err("la obra no se empezó (materiales o condiciones de la ciudad)".into())}WallResult{state,changed_ships,letters:vec![],tasks:vec![]}}else{execute_tower(&input,op)?}}else{execute(&input,merchant,mode)?};
        let mut pool=self.host.message_pool.clone();let date=p3_core::calendar::Calendar::for_tick(input.tick);
        for (id,l) in &out.letters{
            let i=*id as usize;let m=out.state.merchants.get_mut(i).ok_or("destinatario de muralla ausente")?;
            if l.kind!=0x2f{return Err("carta de muralla desconocida".into())}
            p3_state::messages::add_wall_bill(&mut pool,m,date.day,date.month,date.year,l.owner,l.w8,l.wa,l.wc,self.host.council_message_enabled.get(i).copied().unwrap_or(true))?;
        }
        let mut scheduler=self.host.council_scheduler.clone();
        for (due,raw) in &out.tasks{
            if scheduler.free_head>=scheduler.capacity && scheduler.capacity>u16::MAX-128{return Err("planificador de tareas lleno".into())}
            let i=usize::from(scheduler.add_task(*due));let record=scheduler.records.get_mut(i).ok_or("tarea de muralla ausente")?;
            record.kind=u16::from_le_bytes([raw[6],raw[7]]);record.payload.copy_from_slice(&raw[8..24]);
        }
        let mut economy_merchants=self.economy.merchants.clone();
        if economy_merchants.len()!=out.state.merchants.len(){return Err("copias de comerciantes incompatibles".into())}
        if self.economy.world.merchants.len()<usize::from(self.economy.world.merchant_count){return Err("cuentas de comerciantes incompletas".into())}
        for i in 0..economy_merchants.len(){let before=&input.merchants[i].bytes;let after=&out.state.merchants[i].bytes;let target=&mut economy_merchants[i].bytes;
            if before.len()!=after.len()||before.len()!=target.len(){return Err("comerciante de longitud incompatible".into())}
            for j in 0..before.len(){if before[j]!=after[j]{if target[j]!=before[j]&&target[j]!=after[j]{return Err("comerciante modificado por otro sistema".into())}target[j]=after[j]}}
        }
        for i in 0..usize::from(self.economy.world.merchant_count){let before=&input.merchants[i];let after=&out.state.merchants[i];let world=&self.economy.world.merchants[i];
            if (before.i32(0)!=after.i32(0)&&world.gold!=before.i32(0)&&world.gold!=after.i32(0))||(before.i32(0x4c8)!=after.i32(0x4c8)&&world.income!=before.i32(0x4c8)&&world.income!=after.i32(0x4c8)){return Err("cuentas de comerciante desincronizadas".into())}
        }
        for &i in &out.changed_ships{
            let s=&out.state.cargo[i];let write=|image:&mut [u8]|{for w in 0..24{image[0x54+4*w..0x58+4*w].copy_from_slice(&s.stock[w].to_le_bytes())}image[0x3c..0x3e].copy_from_slice(&s.dirty.to_le_bytes());for (p,v) in [(0x114,s.cargo_value),(0x118,s.cargo_weight),(0x11c,s.equipment_weight),(0x120,s.gun_power)]{image[p..p+4].copy_from_slice(&v.to_le_bytes())}};
            if let Some(image)=self.host.expedition_ship_images.get_mut(i).and_then(Option::as_mut){write(image)}
            if let Some(nav)=self.host.navigation.as_mut(){write(&mut nav.snapshot.ships[i*0x180..(i+1)*0x180])}
            if !self.host.debt_changed_ships.contains(&i){self.host.debt_changed_ships.push(i)}
        }
        for (i,(&before,&after)) in input.convoys.iter().zip(&out.state.convoys).enumerate(){if before!=after{
            if let Some(nav)=self.host.navigation.as_mut(){let p=i*0x3c+0x18;nav.snapshot.convoys[p..p+4].copy_from_slice(&after.to_le_bytes())}
            if !self.host.debt_changed_convoys.contains(&i){self.host.debt_changed_convoys.push(i)}
        }}
        for i in 0..count{if input.stocks[i]!=out.state.stocks[i]{self.persisted[i].stock=out.state.stocks[i].clone();self.economy.offices[i]=out.state.stocks[i].clone();self.economy.world.offices[i].weapons.copy_from_slice(&out.state.stocks[i].stock[20..24]);}}
        for i in 0..usize::from(self.economy.world.merchant_count){if input.merchants[i].i32(0)!=out.state.merchants[i].i32(0){self.economy.world.merchants[i].gold=out.state.merchants[i].i32(0)}if input.merchants[i].i32(0x4c8)!=out.state.merchants[i].i32(0x4c8){self.economy.world.merchants[i].income=out.state.merchants[i].i32(0x4c8)}}
        self.economy.world.towns[city]=out.state.town;self.host.sites[city]=out.state.site;
        self.economy.cargo_ships=out.state.cargo;self.host.debt_convoy_flags=out.state.convoys;self.rep_merchants=out.state.merchants;self.economy.merchants=economy_merchants;
        self.host.message_pool=pool;self.host.council_scheduler=scheduler;
        if !self.host.mission_changed_sites.contains(&city){self.host.mission_changed_sites.push(city)}
        Ok(())
    }
}
