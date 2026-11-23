//! Puente de los campos de oficina entre S02 y el modelo de economía C04.
use p3_core::economy::office::OfficeWorkers;
use p3_core::economy::taxes::OfficeAccounts;
use p3_core::economy::trade::OfficeStock;
use p3_save::office::{decode_office,encode_office,OFFICE_LEN};

fn u16_at(b:&[u8],p:usize)->u16 {u16::from_le_bytes([b[p],b[p+1]])}
fn u32_at(b:&[u8],p:usize)->u32 {u32::from_le_bytes(b[p..p+4].try_into().unwrap())}
fn w16(b:&mut [u8],p:usize,v:u16) {b[p..p+2].copy_from_slice(&v.to_le_bytes());}
fn w32(b:&mut [u8],p:usize,v:u32) {b[p..p+4].copy_from_slice(&v.to_le_bytes());}

/// Campos económicos interpretados y resto de la oficina conservado sin pérdida.
/// La experiencia del administrador no está en el registro: debe resolverla el anfitrión.
pub struct PersistedOffice {
    memory:[u8; OFFICE_LEN],
    pub stock:OfficeStock,
    pub accounts:OfficeAccounts,
    pub workers:OfficeWorkers,
}
impl PersistedOffice {
    /// Actualiza las dos listas de edificios tras una operación sobre el plano vivo.
    pub fn set_building_heads(&mut self, businesses:u16, houses:u16) {
        w16(&mut self.memory,0x2ce,businesses);w16(&mut self.memory,0x2d0,houses);
    }
    /// Lee un registro guardado de la versión 0x85.
    pub fn from_save_v85(saved:&[u8; OFFICE_LEN])->Self {Self::from_memory(&decode_office(saved))}
    /// Igual que `from_save_v85(&encode_office(memory))` sin pasar por el orden del guardado: `layout::OFFICE` es una
    /// permutación completa del registro (prueba `office_layout_is_a_full_permutation`), así que ida y vuelta es la identidad.
    pub fn from_memory(memory:&[u8; OFFICE_LEN])->Self {
        let m=*memory;let owner=u16_at(&m,0x2c4);let town=m[0x2c6];
        let stock=OfficeStock {owner,town,
            stock:std::array::from_fn(|i|u32_at(&m,4+4*i) as i32),
            business_consumption:std::array::from_fn(|i|u32_at(&m,0x64+4*i) as i32),
            average_price:std::array::from_fn(|i|f32::from_bits(u32_at(&m,0x3b8+4*i))),
            ai_target:std::array::from_fn(|i|u32_at(&m,0x354+4*i) as i32),
            administrator_limit:std::array::from_fn(|i|u32_at(&m,0x2f4+4*i) as i32),
            automation_flags:u16_at(&m,0x2d6),administrator_experience:None};
        let accounts=OfficeAccounts {owner,town,business_buildings:u16_at(&m,0x2d2),houses:u16_at(&m,0x2d4),
            storage:u16_at(&m,0),previous_storage:u16_at(&m,2),
            current:std::array::from_fn(|i|u32_at(&m,0x418+8*i) as i32),
            previous:std::array::from_fn(|i|u32_at(&m,0x41c+8*i) as i32)};
        let workers=OfficeWorkers{total:u16_at(&m,0x448),auxiliary:u16_at(&m,0x44a)};
        Self{memory:m,stock,accounts,workers}
    }
    /// Produce y valora un negocio de mercancías usando los campos de oficina y ciudad.
    /// Devuelve `None` para tipos especiales 0..3 o fuera de rango.
    pub fn produce_business(&mut self,town:&mut p3_core::town::Town,kind:u8,efficiency:i32,
                            employees:u16,auxiliary:u16,capacity:u16,tick:u32,mask:&mut u32)
                            ->Option<p3_core::economy::office::ProductionValuation> {
        let mut memory=self.to_memory();
        let mut storage:[u8;0x2c0]=memory[..0x2c0].try_into().unwrap();
        let result=p3_core::economy::office::produce_business(town,&mut storage,kind,efficiency,employees,capacity,tick,mask)?;
        let mut average=self.stock.average_price;
        let ok=p3_core::economy::office::value_business(kind,employees,auxiliary,town.u16(0x2c6),result,
                                                        &storage,&mut average,town.u32(0x2cc)!=0);
        assert!(ok);memory[..0x2c0].copy_from_slice(&storage);
        for (i,v) in average.into_iter().enumerate(){w32(&mut memory,0x3b8+4*i,v.to_bits());}
        let experience=self.stock.administrator_experience;
        *self=Self::from_memory(&memory);self.stock.administrator_experience=experience;
        Some(result)
    }
    /// Ejecuta un tipo especial con puertos de construcción y astillero.
    pub fn produce_special_business(&mut self,town:&mut p3_core::town::Town,business:&mut BusinessInput,
        tick:u32,activity:&mut u32,host:&mut dyn p3_core::economy::office::SpecialBusinessHost)->bool {
        let mut memory=self.to_memory();
        let mut storage:[u8;0x2c0]=memory[..0x2c0].try_into().unwrap();
        let ok=p3_core::economy::office::produce_special_business(town,&mut storage,business.kind,&mut business.efficiency,
            business.employees,tick,activity,host);
        if ok {memory[..0x2c0].copy_from_slice(&storage);let experience=self.stock.administrator_experience;
            *self=Self::from_memory(&memory);self.stock.administrator_experience=experience;}
        ok
    }
    /// Reescribe los campos portados y conserva los demás bytes de la oficina.
    pub fn to_save_v85(&self)->[u8; OFFICE_LEN] {encode_office(&self.to_memory())}
    /// Imagen de memoria de la oficina con los campos portados reescritos; igual que `self.to_memory()`.
    pub fn to_memory(&self)->[u8; OFFICE_LEN] {
        assert_eq!(self.stock.owner,self.accounts.owner);assert_eq!(self.stock.town,self.accounts.town);
        let mut m=self.memory;w16(&mut m,0x2c4,self.stock.owner);m[0x2c6]=self.stock.town;
        for i in 0..24 {
            w32(&mut m,4+4*i,self.stock.stock[i] as u32);w32(&mut m,0x64+4*i,self.stock.business_consumption[i] as u32);
            w32(&mut m,0x3b8+4*i,self.stock.average_price[i].to_bits());w32(&mut m,0x354+4*i,self.stock.ai_target[i] as u32);
            w32(&mut m,0x2f4+4*i,self.stock.administrator_limit[i] as u32);
        }
        w16(&mut m,0x2d6,self.stock.automation_flags);w16(&mut m,0x2d2,self.accounts.business_buildings);
        w16(&mut m,0x2d4,self.accounts.houses);w16(&mut m,0,self.accounts.storage);w16(&mut m,2,self.accounts.previous_storage);
        for i in 0..6 {w32(&mut m,0x418+8*i,self.accounts.current[i] as u32);w32(&mut m,0x41c+8*i,self.accounts.previous[i] as u32);}
        w16(&mut m,0x448,self.workers.total);w16(&mut m,0x44a,self.workers.auxiliary);m
    }
}
/// Estado comercial del registro S02; conserva listas de barcos y bytes no interpretados.
/// El formato y los campos proceden de la tabla S02 de versión 0x85.
pub struct PersistedMerchant {
    record:Vec<u8>,
    pub merchant:p3_core::ai::reputation::Merchant,
}
impl PersistedMerchant {
    pub fn from_save_v85(record:&[u8])->Option<Self> {
        if record.len()<p3_save::layout::MERCHANT_RECORD_LEN{return None;}
        let bytes=p3_save::layout::image(&p3_save::layout::MERCHANT,p3_save::layout::MERCHANT_OBJECT_LEN,record);
        Some(Self{record:record.to_vec(),merchant:p3_core::ai::reputation::Merchant{bytes}})
    }
    pub fn to_save_v85(&self)->Vec<u8> {
        let mut record=self.record.clone();
        for seg in &p3_save::layout::MERCHANT {
            let p=seg.pos as usize;let o=seg.obj as usize;let n=seg.len as usize;
            record[p..p+n].copy_from_slice(&self.merchant.bytes[o..o+n]);
        }
        record
    }
}
/// Parámetros del negocio ya resueltos desde su registro S02.
#[derive(Clone,Copy,Debug)]
pub struct BusinessInput {pub kind:u8,pub efficiency:i32,pub employees:u16,pub auxiliary:u16,pub capacity:u16}
/// Produce en una oficina ocupada del modelo S05 y actualiza la capacidad de su ciudad.
/// Los demás registros se conservan. El anfitrión determina el orden de los negocios.
pub fn produce_model_business(world:&mut p3_save::model::WorldModel,index:usize,business:BusinessInput,
                              tick:u32,mask:&mut u32)->Result<(),String> {
    let record=world.offices.get(index).ok_or("índice de oficina")?;
    let saved:&[u8;OFFICE_LEN]=record.as_slice().try_into().map_err(|_|"oficina libre o tamaño inválido")?;
    let mut office=PersistedOffice::from_save_v85(saved);let town_index=office.stock.town as usize;
    if town_index>=world.towns.len(){return Err("ciudad de oficina inexistente".into());}
    let raw=world.write();let parsed=p3_save::world::World::parse(&raw).map_err(|e|format!("mundo: {e:?}"))?;
    let mut town=parsed.town(town_index);
    office.produce_business(&mut town,business.kind,business.efficiency,business.employees,business.auxiliary,business.capacity,tick,mask)
        .ok_or("tipo de negocio no cubierto")?;
    world.towns[town_index].economy[0x490..0x4f0].copy_from_slice(&town.0[0x490..0x4f0]);
    world.offices[index]=office.to_save_v85().to_vec();Ok(())
}
impl PersistedOffice {
    /// Tick diario con negocios de mercancías 4..20 y sin ventanas de interfaz.
    /// `businesses` debe estar en el orden de la cadena original. `captain_wage` es None
    /// cuando no hay administrador válido. Retorna si terminó una ampliación de almacén.
    pub fn daily_tick(&mut self,towns:&mut [p3_core::town::Town],merchant:&mut p3_core::ai::reputation::Merchant,
                      businesses:&[BusinessInput],tick:u32,difficulty:f32,captain_wage:Option<u16>) -> Result<bool,String> {
        let mut copy=businesses.to_vec();
        self.daily_tick_inner(towns,merchant,&mut copy,tick,difficulty,captain_wage,None)
    }
    /// Variante con negocios especiales y efectos externos; actualiza la eficiencia del armero.
    pub fn daily_tick_with_specials(&mut self,towns:&mut [p3_core::town::Town],merchant:&mut p3_core::ai::reputation::Merchant,
                      businesses:&mut [BusinessInput],tick:u32,difficulty:f32,captain_wage:Option<u16>,
                      host:&mut dyn p3_core::economy::office::SpecialBusinessHost) -> Result<bool,String> {
        self.daily_tick_inner(towns,merchant,businesses,tick,difficulty,captain_wage,Some(host))
    }
    fn daily_tick_inner(&mut self,towns:&mut [p3_core::town::Town],merchant:&mut p3_core::ai::reputation::Merchant,
                      businesses:&mut [BusinessInput],tick:u32,difficulty:f32,captain_wage:Option<u16>,
                      mut host:Option<&mut dyn p3_core::economy::office::SpecialBusinessHost>) -> Result<bool,String> {
        use p3_core::economy::{office,trade};
        if businesses.iter().any(|b|b.kind>20 || (b.kind<4 && host.is_none())){return Err("tipo de negocio no cubierto o sin puerto especial".into());}
        let city=self.stock.town as usize;if city>=towns.len(){return Err("ciudad inexistente".into());}
        let mut memory=self.to_memory();
        if (0..3).any(|i|memory[0x2e4+i]>=8){return Err("nivel de alquiler fuera de tabla".into());}
        memory[0x64..0x124].fill(0);merchant.set_i32(0x40+4*city,0);
        let rent_level:[i32;8]=[0,64,128,142,166,0,0,0];let rent_class:[i32;3]=[274,141,54];
        for i in 0..3 {
            let level=memory[0x2e4+i] as usize;
            let rent=(rent_level[level].wrapping_mul(rent_class[i]).wrapping_mul(i32::from(u16_at(&memory,0x2de+2*i))).wrapping_add(15000))>>15;
            for off in [0x488,0x48c]{merchant.set_i32(off,merchant.i32(off).wrapping_add(rent));}
            let cur=u32_at(&memory,0x418) as i32;w32(&mut memory,0x418,cur.wrapping_add(rent) as u32);
        }
        let day=((tick>>8)&7) as usize;for i in 0..24 {w16(&mut memory,0x13c+16*i+2*day,0);}
        w16(&mut memory,0x448,0);let cap=u16_at(&memory,0x2ec);w16(&mut memory,0x44a,cap/200+2);
        let experience=self.stock.administrator_experience;*self=Self::from_memory(&memory);self.stock.administrator_experience=experience;
        let mut activity=0;
        if host.as_deref().and_then(|h|h.live_business_count()).is_some(){
            let mut cursor=u16_at(&memory,0x2cc);let mut steps=0usize;
            loop {
                let special=host.as_deref_mut().expect("validado");let capacity=special.live_business_count().ok_or("gestor de negocios desaparecido")?;
                if usize::from(cursor)>=capacity{break}steps+=1;if steps>65536{return Err("recorrido de negocios no termina".into())}
                let mut rec=special.live_business(cursor).ok_or("negocio inexistente")?;rec[7]=city as u8;special.put_live_business(cursor,rec)?;
                let mut b=BusinessInput{kind:rec[6],efficiency:u32_at(&rec,0) as i32,employees:u16_at(&rec,4),auxiliary:u16_at(&rec,12),capacity:u16_at(&rec,14)};
                if b.kind>20{return Err("tipo de negocio vivo fuera de tabla".into())}
                if b.kind<4 {special.sync_active_office(&self.to_memory());special.sync_active_merchant(merchant);if !self.produce_special_business(&mut towns[city],&mut b,tick,&mut activity,special){return Err("producción especial".into());}special.apply_active_merchant(merchant)?;let mut updated=self.to_memory();special.apply_active_office(&mut updated)?;let experience=self.stock.administrator_experience;*self=Self::from_memory(&updated);self.stock.administrator_experience=experience;}
                else{self.produce_business(&mut towns[city],b.kind,b.efficiency,b.employees,b.auxiliary,b.capacity,tick,&mut activity).ok_or("producción")?;}
                let mut after=special.take_relocated_business(cursor).or_else(||special.live_business(cursor)).ok_or("negocio desaparecido")?;
                if b.kind==3{w32(&mut after,0,b.efficiency as u32);special.put_live_business(cursor,after)?;}
                if after[6]>20{return Err("tipo de negocio vivo fuera de tabla tras producción".into())}
                office::pay_wages(after[6] as usize,u16_at(&after,4),u16_at(&after,12),towns[city].u32(0x2cc),merchant,&mut self.accounts,&mut self.workers);
                cursor=u16_at(&after,8);
            }
        }else{
            for b in businesses {
                if b.kind<4 {
                    let special=host.as_deref_mut().expect("validado");
                    special.sync_active_merchant(merchant);
                    if !self.produce_special_business(&mut towns[city],b,tick,&mut activity,special){return Err("producción especial".into());}
                    special.apply_active_merchant(merchant)?;
                } else {
                    self.produce_business(&mut towns[city],b.kind,b.efficiency,b.employees,b.auxiliary,b.capacity,tick,&mut activity).ok_or("producción")?;
                }
                office::pay_wages(b.kind as usize,b.employees,b.auxiliary,towns[city].u32(0x2cc),merchant,&mut self.accounts,&mut self.workers);
            }
        }
        merchant.set_i32(0x40+4*city,activity as i32);
        if let Some(wage)=captain_wage {
            let amount=i32::from(wage)+i32::from(self.accounts.business_buildings);
            if amount>0 {for off in [0x480,0x484]{merchant.set_i32(off,merchant.i32(off).wrapping_add(amount));}
                self.accounts.current[3]=self.accounts.current[3].wrapping_add(amount);self.workers.total=self.workers.total.wrapping_add(1);}
        }
        let paid=memory[0x2c7] as u16;if paid>0 {let amount=i32::from(paid)*10;
            for off in [0x480,0x484]{merchant.set_i32(off,merchant.i32(off).wrapping_add(amount));}
            self.accounts.current[2]=self.accounts.current[2].wrapping_add(amount);self.workers.total=self.workers.total.wrapping_add(paid);
        }
        let choice=trade::office_commerce_phase(self.stock.automation_flags,merchant.i32(8) as u16==0);
        self.stock.automation_flags=choice.flags;
        if choice.ai {trade::ai_office_trade(towns,&mut self.stock,&mut self.accounts,merchant,difficulty);}
        if choice.inputs {trade::buy_business_inputs(towns,&mut self.stock,&mut self.accounts,merchant);}
        if choice.administrator {trade::administrator_trade(towns,&mut self.stock,&mut self.accounts,merchant,difficulty);}
        memory=self.to_memory();let mut excess=-(i32::from(u16_at(&memory,0x2ec))*2000);for v in self.stock.stock {excess=excess.wrapping_add(v);}
        excess=excess.wrapping_add(u32_at(&memory,0x2e8) as i32);
        if excess>0 {let rent=excess.wrapping_add(1999)/2000;for off in [0x4cc,0x4d0]{merchant.set_i32(off,merchant.i32(off).wrapping_add(rent));}}
        memory=self.to_memory();let remaining=u16_at(&memory,0x2f0);let mut finished=false;
        if remaining>0 {w16(&mut memory,0x2f0,remaining-1);if remaining==1 {finished=true;
            let value=u16_at(&memory,0x2ec);w16(&mut memory,0x2ec,value.wrapping_add(50));
            let pending=u16_at(&memory,0x2ee);w16(&mut memory,0x2ee,pending.wrapping_sub(50));}}
        *self=Self::from_memory(&memory);self.stock.administrator_experience=experience;Ok(finished)
    }
}
/// Ejecuta el tick acotado de una oficina del modelo S05 y guarda sus ciudades y comerciante.
/// El anfitrión resuelve la cadena de negocios y el administrador; las ventanas no se ejecutan.
/// Devuelve el aviso de ampliación que el anfitrión debe entregar al sistema de mensajes.
pub fn tick_model_office(world:&mut p3_save::model::WorldModel,index:usize,businesses:&[BusinessInput],
                         tick:u32,difficulty:f32,captain_wage:Option<u16>,experience:Option<u8>) -> Result<bool,String> {
    let saved=world.offices.get(index).ok_or("índice de oficina")?;
    let saved:&[u8;OFFICE_LEN]=saved.as_slice().try_into().map_err(|_|"oficina libre o tamaño inválido")?;
    let mut office=PersistedOffice::from_save_v85(saved);office.stock.administrator_experience=experience;
    let owner=office.stock.owner as usize;
    let record=world.merchants.get(owner).ok_or("comerciante inexistente")?;
    let mut merchant=PersistedMerchant::from_save_v85(record).ok_or("registro de comerciante incompleto")?;
    let raw=world.write();let parsed=p3_save::world::World::parse(&raw).map_err(|e|format!("mundo: {e:?}"))?;
    let mut towns:Vec<_>=(0..parsed.town_count()).map(|i|parsed.town(i)).collect();let before=towns.clone();
    let sites:Vec<_>=(0..parsed.town_count()).map(|i|p3_core::buildings::Site{records:parsed.building_records(i).to_vec(),cells:parsed.plan_cells(i).to_vec()}).collect();
    let finished=office.daily_tick(&mut towns,&mut merchant.merchant,businesses,tick,difficulty,captain_wage)?;
    let mut targets=world.towns.clone();for i in 0..towns.len(){targets[i].apply_town(&before[i],&towns[i],&sites[i])?;}
    world.towns=targets;world.offices[index]=office.to_save_v85().to_vec();world.merchants[owner]=merchant.to_save_v85();Ok(finished)
}
/// Conecta las oficinas al puerto de `handle_town_tick`; las demás llamadas se delegan.
/// Los datos de negocios y administradores se resuelven previamente por el anfitrión.
pub struct OfficeTownHost<'a,H> {
    pub offices:&'a mut [PersistedOffice],
    pub merchants:&'a mut [p3_core::ai::reputation::Merchant],
    pub businesses:&'a mut [Vec<BusinessInput>],
    pub captain_wages:&'a mut [Option<u16>],
    pub difficulty:f32,
    pub other:&'a mut H,
    pub completed:Vec<(u16,u8)>,
    pub errors:Vec<String>,
}
impl<H:p3_core::economy::world::TownHost> OfficeTownHost<'_,H> {
    fn sync_office_links(&mut self,w:&p3_core::economy::world::EconomyWorld){
        for (saved,live) in self.offices.iter_mut().zip(w.offices.iter()) {
            let mut m=saved.to_memory();w16(&mut m,0x2ca,live.next);w16(&mut m,0x2cc,live.first_facility);
            let exp=saved.stock.administrator_experience;*saved=PersistedOffice::from_memory(&m);saved.stock.administrator_experience=exp;
        }
    }
    fn prepare_construction(&mut self,w:&p3_core::economy::world::EconomyWorld){
        self.other.construction_prepare(w);
        for (i,inputs) in self.businesses.iter().enumerate(){
            let mut cur=w.offices.get(i).map_or(u16::MAX,|o|o.first_facility);
            for b in inputs {let Some(f)=w.merchant_facilities.get(cur as usize) else{break};self.other.construction_business_efficiency(cur,b.efficiency);cur=f.next;}
        }
    }
}
struct LiveSpecial { city:u8,site:p3_core::buildings::Site,capture:BuildingCapture,merchant_count:u16 }
struct TownHostSpecial<'a,H>{other:&'a mut H,live:Option<LiveSpecial>,yard_merchants:Option<Vec<p3_core::ai::reputation::Merchant>>,active:usize,yard_error:Option<String>}
impl<H:p3_core::economy::world::TownHost> p3_core::economy::office::SpecialBusinessHost for TownHostSpecial<'_,H> {
    fn shipyard_construction(&mut self,t:u8,w:u32){self.other.shipyard_construction(t,w)}
    fn shipyard_repairs(&mut self,t:u8,w:u32){self.other.shipyard_repairs(t,w)}
    fn shipyard_advance_with_town(&mut self,t:&mut p3_core::town::Town,w:u32){if let Some(merchants)=self.yard_merchants.as_mut(){if let Err(e)=self.other.shipyard_advance_transaction(t,w,merchants){self.yard_error=Some(e)}}else{self.yard_error=Some("comerciantes del astillero ausentes".into())}}
    fn sync_active_merchant(&mut self,m:&p3_core::ai::reputation::Merchant){if let Some(merchants)=self.yard_merchants.as_mut(){if let Some(active)=merchants.get_mut(self.active){*active=m.clone()}else{self.yard_error=Some("propietario del astillero ausente".into())}}}
    fn apply_active_merchant(&mut self,m:&mut p3_core::ai::reputation::Merchant)->Result<(),String>{if let Some(error)=self.yard_error.take(){return Err(format!("astillero privado: {error}"))}if let Some(merchants)=self.yard_merchants.as_ref(){*m=merchants.get(self.active).ok_or("propietario del astillero ausente")?.clone()}Ok(())}
    fn advance_construction(&mut self,t:&mut p3_core::town::Town,n:u16){
        use p3_core::buildings::advance::AdvanceCtx;
        if let Some(live)=self.live.as_mut(){
            if let Err(e)=advance_construction_snapshot(t,&live.site,&live.capture.offices,&live.capture.pool,live.merchant_count,n){live.capture.error=Some(e);return}
            let mut cx=AdvanceCtx{merchant_count:live.merchant_count,visual_owner:None,host:&mut live.capture};live.site.advance_construction(t,&mut cx,u32::from(n));
        }else{self.other.advance_construction(t,n)}
    }
    fn city_site(&self,t:u8)->Option<&p3_core::buildings::Site>{self.live.as_ref().filter(|l|l.city==t).map(|l|&l.site)}
    fn live_business_count(&self)->Option<usize>{self.live.as_ref().map(|l|l.capture.pool.records.len())}
    fn live_business(&self,i:u16)->Option<[u8;20]>{self.live.as_ref()?.capture.pool.records.get(i as usize).copied()}
    fn take_relocated_business(&mut self,i:u16)->Option<[u8;20]>{self.live.as_mut()?.capture.relocated.take().and_then(|p|p.get(i as usize).copied())}
    fn put_live_business(&mut self,i:u16,r:[u8;20])->Result<(),String>{let l=self.live.as_mut().ok_or("sin pool vivo")?;*l.capture.pool.records.get_mut(i as usize).ok_or("negocio inexistente")?=r;Ok(())}
    fn sync_active_office(&mut self,image:&[u8]){if let Some(l)=self.live.as_mut(){p3_core::buildings::advance::AdvanceHost::sync_active_office(&mut l.capture,image)}}
    fn apply_active_office(&mut self,image:&mut [u8])->Result<(),String>{match self.live.as_mut(){Some(l)=>p3_core::buildings::advance::AdvanceHost::apply_active_office(&mut l.capture,image),None=>Ok(())}}
}
impl<H:p3_core::economy::world::TownHost> p3_core::economy::world::TownHost for OfficeTownHost<'_,H> {
    fn interrupt_before_sea_battle(&self)->bool {self.other.interrupt_before_sea_battle()}
    fn ships_tick_with_offices(&mut self,w:&mut p3_core::economy::world::EconomyWorld,m:&mut [p3_core::ai::reputation::Merchant],s:&mut Vec<p3_core::economy::ship_trade::ShipCargo>,stocks:&mut [OfficeStock],accounts:&mut [OfficeAccounts],tick:u32)->bool {
        if !self.other.ship_ai_enabled(){return self.ships_tick_live(w,m,s,tick)}
        if self.offices.len()!=stocks.len()||stocks.len()!=accounts.len()||w.offices.len()!=stocks.len(){self.other.ship_office_sync_error("tablas de oficinas incompatibles");return false}
        for (i,(saved,live)) in self.offices.iter().zip(stocks.iter()).enumerate(){
            if live.owner<w.merchant_count && (saved.stock.owner!=live.owner||saved.stock.town!=live.town||saved.stock.stock!=live.stock){self.other.ship_office_sync_error(&format!("oficina naval {i} desincronizada: dueño {}/{}, ciudad {}/{}, existencias {:?}",saved.stock.owner,live.owner,saved.stock.town,live.town,(0..24).filter(|&k|saved.stock.stock[k]!=live.stock[k]).map(|k|(k,saved.stock.stock[k],live.stock[k])).collect::<Vec<_>>()));return false}
        }
        let mut images=Vec::with_capacity(self.offices.len()*OFFICE_LEN);for (o,live) in self.offices.iter().zip(stocks.iter()){let mut b=o.to_memory();if live.owner>=w.merchant_count{b[0x2c4..0x2c7].fill(0xff)}images.extend_from_slice(&b[..])}let merchant_count=w.merchant_count;
        // Copia previa de los comerciantes en un solo bloque (antes: un `Vec` por comerciante).
        let mut merchants_before=Vec::with_capacity(m.iter().map(|x|x.bytes.len()).sum());let mut merchant_ends=Vec::with_capacity(m.len());for x in m.iter(){merchants_before.extend_from_slice(&x.bytes);merchant_ends.push(merchants_before.len())}
        let changed=self.other.ships_tick_with_office_images(w,m,s,&mut images,tick);
        if !changed{return false}
        // La imagen anterior de cada oficina se recalcula (nada de lo que la forma cambia durante el tick) en vez de
        // guardar una copia de las 64.
        for i in 0..self.offices.len(){let range=i*OFFICE_LEN..(i+1)*OFFICE_LEN;let mut before=self.offices[i].to_memory();if stocks[i].owner>=merchant_count{before[0x2c4..0x2c7].fill(0xff)}if images[range.clone()]!=before[..]{
            let image:&[u8;OFFICE_LEN]=images[range].try_into().expect("tamaño fijo");let experience=self.offices[i].stock.administrator_experience;let mut office=PersistedOffice::from_memory(image);office.stock.administrator_experience=experience;
            stocks[i]=office.stock.clone();accounts[i]=office.accounts;w.offices[i].weapons.copy_from_slice(&office.stock.stock[20..24]);self.offices[i]=office;
        }}
        let mut start=0;for ((dst,src),&end) in self.merchants.iter_mut().zip(m.iter()).zip(merchant_ends.iter()) {let base=&merchants_before[start..end];start=end;
            // Un registro sin cambios no publica nada: se descarta con una comparación por bloques.
            if src.bytes.as_slice()==base{continue}
            for i in 0..dst.bytes.len().min(src.bytes.len()).min(base.len()){if src.bytes[i]!=base[i]{dst.bytes[i]=src.bytes[i]}}}
        true
    }
    fn ships_tick_live(&mut self,w:&mut p3_core::economy::world::EconomyWorld,m:&mut [p3_core::ai::reputation::Merchant],s:&mut Vec<p3_core::economy::ship_trade::ShipCargo>,tick:u32) -> bool {
        let changed=self.other.ships_tick_live(w,m,s,tick);
        if changed { for (dst,src) in self.merchants.iter_mut().zip(m.iter()) {
            if let (Some(now),Some(to))=(src.bytes.get(0x540..0x546),dst.bytes.get_mut(0x540..0x546)) {to.copy_from_slice(now)}
        }}
        changed
    }
    fn before_day_close(&mut self,w:&p3_core::economy::world::EconomyWorld,a:&mut p3_core::economy::taxes::Accounts,m:&mut [p3_core::ai::reputation::Merchant]){
        for (dst,src) in a.offices.iter_mut().zip(self.offices.iter()){*dst=src.accounts;}
        for (i,(dst,src)) in m.iter_mut().zip(self.merchants.iter_mut()).enumerate(){
            if let Some(live)=w.merchants.get(i){src.set_i32(0,live.gold);src.set_i32(0x4c8,live.income);}
            *dst=src.clone();
        }
    }
    fn after_day_close(&mut self,a:&p3_core::economy::taxes::Accounts,m:&[p3_core::ai::reputation::Merchant]){
        for (dst,src) in self.offices.iter_mut().zip(a.offices.iter()){dst.accounts=*src;}
        for (dst,src) in self.merchants.iter_mut().zip(m.iter()){*dst=src.clone();}
    }
    fn before_facility(&mut self,w:&mut p3_core::economy::world::EconomyWorld,town:usize,facility:usize){
        if w.towns[town].facility_type(facility)==1{self.other.construction_prepare(w);return}
        if w.towns[town].facility_type(facility)!=2 || w.towns[town].employees(facility)==0{return}
        self.sync_office_links(w);
        self.prepare_construction(w);
    }
    fn after_facility(&mut self,w:&mut p3_core::economy::world::EconomyWorld,_town:usize,_facility:usize){
        if !self.other.construction_changed(){return}
        let mut records=Vec::new();while records.len()<u16::MAX as usize {match self.other.employment_business(records.len() as u16){Some(x)=>records.push(x),None=>break}}
        w.merchant_facility_count=records.len() as u16;
        w.merchant_facilities=records.iter().map(|r|p3_core::economy::world::MerchantFacility{employees:u16_at(r,4),town:r[7],next:u16_at(r,8)}).collect();
        for (i,office) in self.offices.iter().enumerate(){
            let m=office.to_memory();if let Some(o)=w.offices.get_mut(i){o.first_facility=u16_at(&m,0x2cc);o.next=u16_at(&m,0x2ca);}
            if let Some(inputs)=self.businesses.get_mut(i){inputs.clear();let mut cur=usize::from(u16_at(&m,0x2cc));let mut seen=vec![false;records.len()];
                while cur<records.len(){if seen[cur]{self.errors.push(format!("ciclo en negocios de oficina {i}"));break}seen[cur]=true;let r=&records[cur];
                    inputs.push(BusinessInput{kind:r[6],efficiency:u32_at(r,0) as i32,employees:u16_at(r,4),auxiliary:u16_at(r,12),capacity:u16_at(r,14)});cur=usize::from(u16_at(r,8));}
            }
        }
    }
    fn office_update_business_employees(&mut self,w:&mut p3_core::economy::world::EconomyWorld,o:u16)->u16{
        let result=self.other.office_update_business_employees(w,o);
        if let Some(count)=self.other.employment_office_count(o) {
            if let Some(office)=self.offices.get_mut(o as usize){office.accounts.business_buildings=count;}
            let mut cursor=w.offices.get(o as usize).map(|x|x.first_facility).unwrap_or(u16::MAX);
            if let Some(inputs)=self.businesses.get_mut(o as usize) {for b in inputs {
                if let Some(raw)=self.other.employment_business(cursor) {b.auxiliary=u16_at(&raw,12);b.capacity=u16_at(&raw,14);b.employees=u16_at(&raw,4);cursor=u16_at(&raw,8);} else {break;}
            }}
        }
        result
    }
    fn employment_office_count(&self,o:u16)->Option<u16>{self.other.employment_office_count(o)}
    fn employment_business(&self,i:u16)->Option<[u8;20]>{self.other.employment_business(i)}
    fn office_daily_tick(&mut self,w:&mut p3_core::economy::world::EconomyWorld,o:u16,_arg:u32){
        let i=o as usize;
        let live=if self.businesses.get(i).is_some_and(|v|v.iter().any(|b|b.kind==2)) {
            self.sync_office_links(w);self.prepare_construction(w);
            let city=self.offices.get(i).map(|x|x.stock.town);
            match city.and_then(|c|self.other.construction_assets(c).map(|a|(c,a))) {
                Some((city,(site,pool)))=>{
                    let images=self.offices.iter().map(|o|Some(o.to_memory().to_vec())).collect::<Vec<_>>();
                    match make_live_capture(&w.towns[city as usize],&images,pool,i){Ok(capture)=>Some(LiveSpecial{city,site,capture,merchant_count:self.merchants.len() as u16}),Err(e)=>{self.errors.push(e);return}}
                },None=>None,
            }
        }else{None};
        let yard_merchants=if self.businesses.get(i).is_some_and(|v|v.iter().any(|b|b.kind==1)){self.other.construction_prepare(w);Some(self.merchants.to_vec())}else{None};
        let Some(office)=self.offices.get_mut(i) else {self.errors.push(format!("oficina {i} ausente"));return};
        let owner=office.stock.owner as usize;
        let (Some(merchant),Some(wm),Some(businesses),Some(wage))=(self.merchants.get_mut(owner),w.merchants.get(owner),self.businesses.get_mut(i),self.captain_wages.get(i)) else {
            self.errors.push(format!("dependencia de oficina {i} ausente"));return;
        };
        // employ_beggars modifica la tabla global después del recuento y antes de producir.
        if self.other.employment_office_count(o).is_some() {
            let mut cursor=w.offices.get(i).map(|x|x.first_facility).unwrap_or(u16::MAX);
            for b in businesses.iter_mut() {
                let Some(f)=w.merchant_facilities.get(cursor as usize) else {break};
                b.employees=f.employees;cursor=f.next;
            }
        }
        merchant.set_i32(0,wm.gold);merchant.set_i32(0x4c8,wm.income);
        let mut special=TownHostSpecial{other:self.other,live,yard_merchants,active:owner,yard_error:None};
        let result=office.daily_tick_with_specials(&mut w.towns,merchant,businesses,w.tick,self.difficulty,*wage,&mut special);
        if let Ok(finished)=result.as_ref(){
            w.merchants[owner].gold=merchant.i32(0);w.merchants[owner].income=merchant.i32(0x4c8);
            if let Some(wo)=w.offices.get_mut(i){wo.weapons.copy_from_slice(&office.stock.stock[20..24]);}
            if *finished{self.completed.push((office.stock.owner,office.stock.town));}
            if let Some(live)=special.live.as_mut(){live.capture.offices[i]=Some(office.to_memory().to_vec());}
        }
        let live=special.live.take();let yard_merchants=special.yard_merchants.take();drop(special);
        if let Err(e)=result{self.errors.push(format!("oficina {i}: {e}"));return}
        if let Some(yard_merchants)=yard_merchants{for (j,source) in yard_merchants.into_iter().enumerate(){if j!=owner{self.merchants[j]=source;}}}
        if let Some(live)=live {
            let events=live.capture.letters.borrow().iter().filter_map(|e|match *e {ConstructionLetter::Building(o,t,a,b)=>Some((o,t,a,b)),_=>None}).collect::<Vec<_>>();
            if let Err(e)=self.other.construction_commit(live.city,live.site,live.capture.pool,&live.capture.offices,&events,self.merchants){self.errors.push(e);return}
            for (old,new) in self.offices.iter_mut().zip(live.capture.offices.iter()){if let Some(m)=new {let exp=old.stock.administrator_experience;*old=PersistedOffice::from_memory(m.as_slice().try_into().expect("oficina validada"));old.stock.administrator_experience=exp;}}
            p3_core::economy::world::TownHost::after_facility(self,w,live.city as usize,2);
        }
    }
    fn office_links_changed(&mut self,w:&p3_core::economy::world::EconomyWorld){
        self.sync_office_links(w);self.other.office_links_changed(w);
    }
    fn try_add_type_2f_building(&mut self,t:&mut p3_core::town::Town,n:u32){self.other.try_add_type_2f_building(t,n)}
    fn count_street_cells(&mut self,t:&mut p3_core::town::Town){self.other.count_street_cells(t)}
    fn recount_buildings(&mut self,t:&mut p3_core::town::Town){
        self.other.recount_buildings(t);
        // 0x520BD0 escribe directamente en las oficinas del original. La
        // vista de viviendas debe publicarse antes de la siguiente ciudad:
        // construcción, alquileres y balance del comerciante leen el registro.
        for (i,saved) in self.offices.iter_mut().enumerate(){
            if saved.stock.owner==u16::MAX{continue}
            let Some(h)=self.other.office_housing(i)else{continue};
            let mut image=saved.to_memory();w16(&mut image,0x2d4,h.house_count);
            for g in 0..3{w16(&mut image,0x2d8+2*g,h.capacity[g]);w16(&mut image,0x2de+2*g,h.assigned[g]);}
            let experience=saved.stock.administrator_experience;
            *saved=PersistedOffice::from_memory(&image);saved.stock.administrator_experience=experience;
        }
    }
    fn office_housing(&self,office:usize)->Option<p3_core::buildings::housing::OfficeHousing>{self.other.office_housing(office)}
    fn recount_defense_buildings(&mut self,t:&mut p3_core::town::Town){self.other.recount_defense_buildings(t)}
    fn church_daily_update(&mut self,t:&mut p3_core::town::Town,i:u8){self.other.church_daily_update(t,i)}
    fn tavern_daily(&mut self,t:&mut p3_core::town::Town){self.other.tavern_daily(t)}
    fn tavern_daily_with_world(&mut self,w:&p3_core::economy::world::EconomyWorld,t:&mut p3_core::town::Town){self.other.tavern_daily_with_world(w,t)}
    fn council_offer(&mut self,t:&mut p3_core::town::Town){self.other.council_offer(t)}
    fn council_offer_with_world(&mut self,w:&p3_core::economy::world::EconomyWorld,t:&mut p3_core::town::Town){self.other.council_offer_with_world(w,t)}
    fn dispatch_scheduled_tasks(&mut self,w:&mut p3_core::economy::world::EconomyWorld,m:&mut [p3_core::ai::reputation::Merchant],tick:u32){self.other.dispatch_scheduled_tasks(w,self.merchants,tick);for (dst,src) in m.iter_mut().zip(self.merchants.iter()){dst.bytes[10..12].copy_from_slice(&src.bytes[10..12]);dst.bytes[0x1f]=src.bytes[0x1f];dst.bytes[0xf0..0x118].copy_from_slice(&src.bytes[0xf0..0x118]);dst.bytes[0x608..0x60c].copy_from_slice(&src.bytes[0x608..0x60c]);}}
    fn dispatch_scheduled_tasks_full(&mut self,w:&mut p3_core::economy::world::EconomyWorld,m:&mut [p3_core::ai::reputation::Merchant],offices:&[p3_core::economy::extra_tax::TaxOffice],tick:u32){
        self.dispatch_scheduled_tasks_assets(w,m,offices,&mut [],tick)
    }
    fn dispatch_scheduled_tasks_assets(&mut self,w:&mut p3_core::economy::world::EconomyWorld,m:&mut [p3_core::ai::reputation::Merchant],_:&[p3_core::economy::extra_tax::TaxOffice],stocks:&mut [p3_core::economy::trade::OfficeStock],tick:u32){
        self.dispatch_scheduled_tasks_fleet(w,m,&[],stocks,&mut Vec::new(),self.difficulty,tick)
    }
    fn dispatch_scheduled_tasks_fleet(&mut self,w:&mut p3_core::economy::world::EconomyWorld,m:&mut [p3_core::ai::reputation::Merchant],_:&[p3_core::economy::extra_tax::TaxOffice],stocks:&mut [p3_core::economy::trade::OfficeStock],ships:&mut Vec<p3_core::economy::ship_trade::ShipCargo>,_:f32,tick:u32){
        if self.other.needs_scheduled_office_images(tick)&&self.captain_wages.len()<self.offices.len(){self.errors.push("salarios de oficinas incompletos".into());return}
        let offices=w.offices.iter().zip(self.offices.iter()).map(|(o,p)|p3_core::economy::extra_tax::TaxOffice{owner:p.accounts.owner,next:o.next,businesses:p.accounts.business_buildings,houses:p.accounts.houses}).collect::<Vec<_>>();
        for (i,(dst,src)) in self.merchants.iter_mut().zip(m.iter()).enumerate(){if let Some(wm)=w.merchants.get(i){dst.set_i32(0,wm.gold);}dst.bytes[0x4bc..0x4c0].copy_from_slice(&src.bytes[0x4bc..0x4c0]);}
        let mut live=self.offices.iter().map(|o|o.stock.clone()).collect::<Vec<_>>();
        if self.other.needs_scheduled_office_images(tick){let images=self.offices.iter().enumerate().map(|(i,office)|{let mut image=office.to_memory().to_vec();if let Some(live)=w.offices.get(i){image[0x2ca..0x2cc].copy_from_slice(&live.next.to_le_bytes())}image}).collect();self.other.set_scheduled_office_images(images);}else{self.other.set_scheduled_office_images(Vec::new());}
        // Los huecos del pool no tienen una oficina persistida válida: su owner
        // autoritativo está en la vista viva, no en el registro de relleno.
        for (stock, current) in live.iter_mut().zip(stocks.iter()) { if current.owner == u16::MAX { stock.owner = u16::MAX; } }
        self.other.dispatch_scheduled_tasks_fleet(w,self.merchants,&offices,&mut live,ships,self.difficulty,tick);
        for update in self.other.take_scheduled_office_updates(){let Some(saved)=self.offices.get_mut(update.index)else{self.errors.push("oficina de tarea ausente".into());continue};let Some(stock)=live.get_mut(update.index)else{self.errors.push("existencias de tarea ausentes".into());continue};let Some(wage)=self.captain_wages.get_mut(update.index)else{self.errors.push("salario de tarea ausente".into());continue};let full=update.image.is_some();let experience=if full{stock.administrator_experience}else{update.experience};let mut image=if let Some(raw)=update.image{match raw.try_into(){Ok(image)=>image,Err(_)=>{self.errors.push("imagen de oficina de tarea truncada".into());continue}}}else{saved.to_memory()};image[0x2f2..0x2f4].copy_from_slice(&update.administrator.to_le_bytes());image[0x2d6..0x2d8].copy_from_slice(&update.flags.to_le_bytes());*saved=PersistedOffice::from_memory(&image);saved.stock.administrator_experience=experience;if full{
            saved.stock=stock.clone();
            if let Some(inputs)=self.businesses.get_mut(update.index){
                let mut chain=Vec::new();let mut cur=u16_at(&image,0x2cc);let mut seen=std::collections::BTreeSet::new();
                while let Some(r)=self.other.employment_business(cur){
                    if !seen.insert(cur){self.errors.push(format!("ciclo en negocios de oficina {}",update.index));break}
                    chain.push(BusinessInput{kind:r[6],efficiency:u32_at(&r,0) as i32,employees:u16_at(&r,4),auxiliary:u16_at(&r,12),capacity:u16_at(&r,14)});cur=u16_at(&r,8);
                }
                *inputs=chain;
            }else{self.errors.push("negocios de oficina de tarea ausentes".into())}
        }else{stock.automation_flags=update.flags;stock.administrator_experience=experience;*wage=update.wage}}
        for (i,stock) in live.into_iter().enumerate(){if stocks.get(i).is_some_and(|old| old.owner == u16::MAX) && stock.owner == u16::MAX { continue; } if let Some(o)=self.offices.get_mut(i){o.stock=stock.clone();}if let Some(o)=stocks.get_mut(i){*o=stock.clone();}if let Some(o)=w.offices.get_mut(i){o.weapons.copy_from_slice(&stock.stock[20..24]);}}
        for (i,(dst,src)) in m.iter_mut().zip(self.merchants.iter()).enumerate(){dst.bytes[0..4].copy_from_slice(&src.bytes[0..4]);dst.bytes[10..12].copy_from_slice(&src.bytes[10..12]);dst.bytes[0xe..0x10].copy_from_slice(&src.bytes[0xe..0x10]);dst.bytes[0x27..0x34].copy_from_slice(&src.bytes[0x27..0x34]);dst.bytes[0x478..0x47a].copy_from_slice(&src.bytes[0x478..0x47a]);dst.bytes[0x22..0x24].copy_from_slice(&src.bytes[0x22..0x24]);dst.bytes[0x4c8..0x4cc].copy_from_slice(&src.bytes[0x4c8..0x4cc]);dst.bytes[0x1f]=src.bytes[0x1f];dst.bytes[0x37]=src.bytes[0x37];dst.bytes[0xe1]=src.bytes[0xe1];dst.bytes[0xf0..0x118].copy_from_slice(&src.bytes[0xf0..0x118]);dst.bytes[0x608..0x60c].copy_from_slice(&src.bytes[0x608..0x60c]);dst.bytes[0x4bc..0x4c0].copy_from_slice(&src.bytes[0x4bc..0x4c0]);dst.bytes[0x464..0x468].copy_from_slice(&src.bytes[0x464..0x468]);for t in 0..w.towns.len().min(40){for base in [0x11c,0x120]{let p=base+12*t;dst.bytes[p..p+4].copy_from_slice(&src.bytes[p..p+4]);}}if let Some(wm)=w.merchants.get_mut(i){wm.gold=src.i32(0);}}
    }
    fn construction_planner(&mut self,t:&mut p3_core::town::Town){self.other.construction_planner(t)}
    fn construction_planner_with_world(&mut self,w:&p3_core::economy::world::EconomyWorld,t:&mut p3_core::town::Town){self.other.construction_planner_with_world(w,t)}
    fn advance_construction(&mut self,t:&mut p3_core::town::Town,n:u16){
        let Some((site,pool))=self.other.construction_assets(t.index()) else {self.other.advance_construction(t,n);return};
        let offices=self.offices.iter().map(|o|Some(o.to_memory().to_vec())).collect::<Vec<_>>();
        match advance_construction_snapshot(t,&site,&offices,&pool,self.merchants.len() as u16,n) {
            Ok(out)=>{
                if let Err(e)=self.other.construction_commit(t.index(),out.site,out.pool,&out.offices,&out.letters,self.merchants){self.errors.push(e);return}
                for (old,new) in self.offices.iter_mut().zip(out.offices.iter()){if let Some(m)=new {let exp=old.stock.administrator_experience;*old=PersistedOffice::from_memory(m.as_slice().try_into().expect("oficina validada"));old.stock.administrator_experience=exp;}}
                *t=out.town;
            }
            Err(e)=>self.errors.push(format!("avance de obra: {e}")),
        }
    }
    fn shipyard_construction(&mut self,i:u8,n:u32){self.other.shipyard_construction(i,n)}
    fn shipyard_repairs(&mut self,i:u8,n:u32){self.other.shipyard_repairs(i,n)}
    fn shipyard_advance_with_town(&mut self,t:&mut p3_core::town::Town,n:u32){if let Err(e)=self.other.shipyard_advance_transaction(t,n,self.merchants){self.errors.push(format!("astillero {}: {e}",t.index()));}}
    fn wall_expansion_level(&mut self,t:&p3_core::town::Town)->u8{self.other.wall_expansion_level(t)}
    fn find_or_create_office(&mut self,w:&mut p3_core::economy::world::EconomyWorld,m:u8,t:u8)->Option<usize>{self.other.find_or_create_office(w,m,t)}
    fn siege_pitch_demand(&mut self,i:u8)->i32{self.other.siege_pitch_demand(i)}
    fn siege_pitch_demand_for_town(&mut self,t:&p3_core::town::Town)->i32{self.other.siege_pitch_demand_for_town(t)}
    fn famine_message(&mut self,i:u8){self.other.famine_message(i)}
    fn famine_message_with_world(&mut self,w:&p3_core::economy::world::EconomyWorld,i:u8){self.other.famine_message_with_world(w,i)}
    fn famine_notify(&mut self,m:u16,t:u8,l:bool){self.other.famine_notify(m,t,l)}
}
/// Recorre la cadena de negocios de una oficina S05 y ejecuta su tick.
/// Los tipos especiales se rechazan antes de modificar el modelo; el capitán sigue siendo externo.
pub fn tick_model_office_resolved(world:&mut p3_save::model::WorldModel,index:usize,tick:u32,difficulty:f32,
                                  captain_wage:Option<u16>,experience:Option<u8>)->Result<bool,String>{
    let saved=world.offices.get(index).ok_or("índice de oficina")?;
    let saved:&[u8;OFFICE_LEN]=saved.as_slice().try_into().map_err(|_|"oficina libre o tamaño inválido")?;
    let memory=decode_office(saved);let town=memory[0x2c6];let mut cursor=u16_at(&memory,0x2cc) as usize;
    let mut seen=vec![false;world.businesses.len()];let mut chain=Vec::new();let mut inputs=Vec::new();
    while cursor<world.businesses.len(){
        if seen[cursor]{return Err("ciclo en cadena de negocios".into());}seen[cursor]=true;
        let b=&world.businesses[cursor];if b.len()!=20{return Err("tamaño de negocio inválido".into());}
        let kind=b[6];if !(4..=20).contains(&kind){return Err(format!("tipo especial de negocio {kind} no cubierto"));}
        inputs.push(BusinessInput{kind,efficiency:u32_at(b,0) as i32,employees:u16_at(b,4),auxiliary:u16_at(b,12),capacity:u16_at(b,14)});
        chain.push(cursor);cursor=u16_at(b,8) as usize;
    }
    let finished=tick_model_office(world,index,&inputs,tick,difficulty,captain_wage,experience)?;
    for i in chain{world.businesses[i][7]=town;}Ok(finished)
}
/// Variante S05 que admite los negocios especiales mediante puertos externos.
/// Conserva los campos de `Town` que se guardan; rechaza cambios en los no guardados.
/// El puerto de obras debe entregar su `Site` actualizado cuando se ejecute un negocio tipo 2.
/// Los efectos externos ya realizados por el puerto no se revierten.
pub fn tick_model_office_resolved_with_specials(world:&mut p3_save::model::WorldModel,index:usize,tick:u32,difficulty:f32,
    captain_wage:Option<u16>,experience:Option<u8>,host:&mut dyn p3_core::economy::office::SpecialBusinessHost)->Result<bool,String>{
    let saved=world.offices.get(index).ok_or("índice de oficina")?;
    let saved:&[u8;OFFICE_LEN]=saved.as_slice().try_into().map_err(|_|"oficina libre o tamaño inválido")?;
    let memory=decode_office(saved);let town_index=memory[0x2c6];let mut cursor=u16_at(&memory,0x2cc) as usize;
    let mut seen=vec![false;world.businesses.len()];let mut chain=Vec::new();let mut inputs=Vec::new();
    while cursor<world.businesses.len(){
        if seen[cursor]{return Err("ciclo en cadena de negocios".into());}seen[cursor]=true;
        let b=&world.businesses[cursor];if b.len()!=20{return Err("tamaño de negocio inválido".into());}
        if b[6]>20{return Err(format!("tipo de negocio {} no cubierto",b[6]));}
        inputs.push(BusinessInput{kind:b[6],efficiency:u32_at(b,0) as i32,employees:u16_at(b,4),auxiliary:u16_at(b,12),capacity:u16_at(b,14)});
        chain.push(cursor);cursor=u16_at(b,8) as usize;
    }
    let mut office=PersistedOffice::from_save_v85(saved);office.stock.administrator_experience=experience;
    let owner=office.stock.owner as usize;let record=world.merchants.get(owner).ok_or("comerciante inexistente")?;
    let mut merchant=PersistedMerchant::from_save_v85(record).ok_or("registro de comerciante incompleto")?;
    let raw=world.write();let parsed=p3_save::world::World::parse(&raw).map_err(|e|format!("mundo: {e:?}"))?;
    let mut towns:Vec<_>=(0..parsed.town_count()).map(|i|parsed.town(i)).collect();let before=towns.clone();
    let mut sites:Vec<_>=(0..parsed.town_count()).map(|i|p3_core::buildings::Site{records:parsed.building_records(i).to_vec(),cells:parsed.plan_cells(i).to_vec()}).collect();
    let finished=office.daily_tick_with_specials(&mut towns,&mut merchant.merchant,&mut inputs,tick,difficulty,captain_wage,host)?;
    if inputs.iter().any(|b|b.kind==2){
        let site=host.city_site(town_index).ok_or("el puerto de obras no entregó su estado de registros y plano")?;
        sites[town_index as usize]=site.clone();
    }
    let mut targets=world.towns.clone();for i in 0..towns.len(){targets[i].apply_town(&before[i],&towns[i],&sites[i])?;}
    world.towns=targets;world.offices[index]=office.to_save_v85().to_vec();world.merchants[owner]=merchant.to_save_v85();
    for (i,b) in chain.into_iter().zip(inputs){world.businesses[i][7]=town_index;world.businesses[i][..4].copy_from_slice(&b.efficiency.to_le_bytes());}
    Ok(finished)
}
/// Salario y experiencia del administrador situado en la tabla de capitanes S05.
/// Como en x86, un índice fuera de la capacidad significa que no hay administrador.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct SavedAdministrator {pub index:u16,pub wage:u16,pub experience:u8}
pub fn administrator_from_model(model:&p3_save::model::SaveModel,office_index:usize)->Result<Option<SavedAdministrator>,String>{
    let record=model.world.offices.get(office_index).ok_or("índice de oficina")?;
    let saved:&[u8;OFFICE_LEN]=record.as_slice().try_into().map_err(|_|"oficina libre o tamaño inválido")?;
    let mem=decode_office(saved);let index=u16_at(&mem,0x2f2);
    let Some(captain)=model.ships.captains.get(index as usize) else{return Ok(None)};
    let saved:&[u8;16]=captain.as_slice().try_into().map_err(|_|"registro de capitán inválido")?;
    let captain=p3_save::captain::decode_v85(saved);
    Ok(Some(SavedAdministrator{index,wage:u16_at(&captain,0xc),experience:captain[0xa]}))
}
/// Resuelve capitán y negocios desde S05 y ejecuta el tick con sus puertos especiales.
/// La limitación de persistencia de cambios urbanos de `tick_model_office_resolved_with_specials` sigue aplicándose.
pub fn tick_save_model_office(model:&mut p3_save::model::SaveModel,index:usize,tick:u32,difficulty:f32,
    host:&mut dyn p3_core::economy::office::SpecialBusinessHost)->Result<bool,String>{
    let admin=administrator_from_model(model,index)?;
    tick_model_office_resolved_with_specials(&mut model.world,index,tick,difficulty,
        admin.map(|a|a.wage),admin.map(|a|a.experience),host)
}
/// Ejecuta el tick S05 y entrega el aviso de ampliación 0x60, sin interfaz.
/// Las preferencias del comerciante y los efectos externos del anfitrión se suministran aparte.
pub fn tick_save_model_office_with_messages(model:&mut p3_save::model::SaveModel,index:usize,tick:u32,difficulty:f32,
    enabled:bool,host:&mut dyn p3_core::economy::office::SpecialBusinessHost)->Result<(bool,bool),String>{
    let mut staged=model.clone();let finished=tick_save_model_office(&mut staged,index,tick,difficulty,host)?;
    let mut delivered=false;
    if finished {
        let record=staged.world.offices.get(index).ok_or("índice de oficina")?;
        let saved:&[u8;OFFICE_LEN]=record.as_slice().try_into().map_err(|_|"oficina libre")?;
        let mem=decode_office(saved);let owner=u16_at(&mem,0x2c4) as usize;let town=mem[0x2c6];
        let merchant_record=staged.world.merchants.get(owner).ok_or("comerciante inexistente")?;
        let mut merchant=PersistedMerchant::from_save_v85(merchant_record).ok_or("comerciante inválido")?;
        let header=&staged.world.header;if header.len()<4{return Err("cabecera de mundo truncada".into());}
        delivered=crate::messages::add_office_expansion(&mut staged.message_pool,&mut merchant.merchant,header[0],header[1],u16_at(header,2),town,enabled)?;
        staged.world.merchants[owner]=merchant.to_save_v85();
    }
    *model=staged;Ok((finished,delivered))
}
/// Variante S05 que también conserva los cambios de barco del gestor de astillero.
/// El gestor debe reflejar el modelo al comenzar; rechaza botaduras y cambios de listas o entradas.
/// Los efectos externos del anfitrión no se revierten si falla la persistencia.
pub fn tick_save_model_office_with_shipyard(model:&mut p3_save::model::SaveModel,index:usize,tick:u32,difficulty:f32,
    enabled:bool,host:&mut dyn p3_core::economy::office::SpecialBusinessHost)->Result<(bool,bool),String>{
    let before=host.shipyard_manager().ok_or("gestor de astillero ausente")?.clone();let mut staged=model.clone();
    let result=tick_save_model_office_with_messages(&mut staged,index,tick,difficulty,enabled,host)?;
    let after=host.shipyard_manager().ok_or("gestor de astillero ausente")?;
    staged.ships.apply_shipyard_manager(&before,after)?;*model=staged;Ok(result)
}

/// Acciones visuales que se entregan después de confirmar el guardado.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum YardPresentation { Notify(u16,u32,u8,u32), Berth(u32), EventOpen, EventShow(u8,u32,u32) }
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct YardTickResult {pub expansion_finished:bool,pub expansion_delivered:bool,pub launch_letters:usize,pub building_letters:usize,pub presentation:Vec<YardPresentation>}
#[derive(Clone,Copy)]
enum ConstructionLetter { Launch(u8,u8,u16,u16), Building(u32,u8,u16,u16) }
type ConstructionLetters = std::rc::Rc<std::cell::RefCell<Vec<ConstructionLetter>>>;
struct YardCapture { counters:Vec<p3_core::shipyard::LaunchCounters>, letters:ConstructionLetters, presentation:Vec<YardPresentation> }
impl p3_core::shipyard::ShipyardHost for YardCapture {
 fn merchant_flag(&mut self,o:u8)->u16{self.counters.get(o as usize).map_or(0,|c|c.flag)}
 fn notify(&mut self,o:u16,t:u32,k:u8,s:u32){self.presentation.push(YardPresentation::Notify(o,t,k,s));}
 fn berth(&mut self,s:u32){self.presentation.push(YardPresentation::Berth(s));}
}
impl p3_core::shipyard::LaunchHost for YardCapture {
 fn add_message(&mut self,o:u8,t:u8,s:u16,c:u16){self.letters.borrow_mut().push(ConstructionLetter::Launch(o,t,s,c));}
 fn launch_counters(&mut self,o:u8)->&mut p3_core::shipyard::LaunchCounters{&mut self.counters[o as usize]}
 fn launch_notify(&mut self,o:u16,t:u32,s:u32){self.presentation.push(YardPresentation::Notify(o,t,6,s));}
 fn event_open(&mut self){self.presentation.push(YardPresentation::EventOpen)}
 fn event_show(&mut self,k:u8,t:u32,s:u32){self.presentation.push(YardPresentation::EventShow(k,t,s))}
}

/// Resultado independiente del avance de las dos colas de un astillero.
#[derive(Clone,Debug)]
pub struct ShipyardSnapshot {
 pub town:p3_core::town::Town,
 pub manager:p3_core::shipyard::ShipManager,
 pub merchants:Vec<p3_core::ai::reputation::Merchant>,
 pub message_pool:Vec<u8>,
 pub presentation:Vec<YardPresentation>,
}

/// Ejecuta construcción y reparaciones en ese orden, sobre copias. Los avisos
/// visuales quedan registrados para el anfitrión y las cartas se entregan al
/// buzón real. Rechaza cadenas cíclicas antes de entrar en el puerto original.
pub fn advance_shipyard_snapshot(
 town:&p3_core::town::Town,manager:&p3_core::shipyard::ShipManager,
 merchants:&[p3_core::ai::reputation::Merchant],message_pool:&[u8],
 enabled:&[bool],work:u32,tick:u32,player:u32,
)->Result<ShipyardSnapshot,String>{
 use p3_core::shipyard::{LaunchCounters,ShipyardCtx,SHIP_SIZE,YARD_SIZE};
 let count=u16::try_from(merchants.len()).map_err(|_|"demasiados comerciantes")?;
 if manager.ships.len()<usize::from(manager.ship_count)*SHIP_SIZE||manager.entries.len()<usize::from(manager.entry_count)*YARD_SIZE{return Err("pool de astillero truncado".into())}
 if merchants.iter().any(|m|m.bytes.len()<0x650){return Err("comerciante de astillero truncado".into())}
 let walk_ships=|mut id:u16|->Result<(),String>{let mut seen=vec![false;usize::from(manager.ship_count)];while id<manager.ship_count{let i=usize::from(id);if seen[i]{return Err("cadena de barcos del astillero cíclica".into())}seen[i]=true;id=manager.s16(id,6);}Ok(())};
 walk_ships(town.u16(0x81c))?;walk_ships(town.u16(0x81e))?;walk_ships(manager.docked_head)?;
 let mut seen=vec![false;usize::from(manager.entry_count)];let mut entry=town.u16(0x820);
 while entry<manager.entry_count{let i=usize::from(entry);if seen[i]{return Err("cadena de obras del astillero cíclica".into())}seen[i]=true;let lead=manager.e16(entry,0x10);if lead>=manager.ship_count{return Err("líder de obra inexistente".into())}walk_ships(manager.e16(entry,0xa))?;entry=manager.e16(entry,8);}
 let mut town=town.clone();let mut manager=manager.clone();let mut merchants=merchants.to_vec();let mut message_pool=message_pool.to_vec();
 let counters=merchants.iter().map(|m|LaunchCounters{flag:m.u16(8),launched:m.u16(0x478)}).collect();
 let letters=ConstructionLetters::default();let mut capture=YardCapture{counters,letters:letters.clone(),presentation:Vec::new()};
 let town_index=u32::from(town.index());
 manager.advance_ship_construction(&mut town,town_index,work as i32,tick,player,count,&mut capture);
 manager.advance_repairs(&mut town,town_index,work as i32,&mut ShipyardCtx{merchant_count:count,host:&mut capture});
 for (merchant,counter) in merchants.iter_mut().zip(&capture.counters){merchant.bytes[0x478..0x47a].copy_from_slice(&counter.launched.to_le_bytes());}
 let date=p3_core::calendar::Calendar::for_tick(tick);
 for letter in letters.borrow().iter(){if let ConstructionLetter::Launch(owner,city,ship,class)=*letter{if let Some(merchant)=merchants.get_mut(usize::from(owner)){crate::messages::add_ship_launch(&mut message_pool,merchant,date.day,date.month,date.year,city,ship,class,enabled.get(usize::from(owner)).copied().unwrap_or(true))?;}}}
 Ok(ShipyardSnapshot{town,manager,merchants,message_pool,presentation:capture.presentation})
}
/// Estado transaccional de obras, oficinas y pool de negocios.
struct BuildingCapture { letters:ConstructionLetters, housing:Vec<p3_core::buildings::housing::OfficeHousing>, recounted:bool,
 offices:Vec<Option<Vec<u8>>>, changed:Vec<bool>, active:usize, selected:Option<usize>, error:Option<String>, first_office:u16,
 pool:p3_core::economy::facility_pool::FacilityPool, relocated:Option<Vec<[u8;20]>> }
impl BuildingCapture {
 fn refresh_housing(&mut self,i:usize){if let Some(ref img)=self.offices[i]{let h=&mut self.housing[i];h.houses_head=u16_at(img,0x2d0);h.house_count=u16_at(img,0x2d4);for k in 0..3{h.capacity[k]=u16_at(img,0x2d8+2*k);h.assigned[k]=u16_at(img,0x2de+2*k);h.level[k]=img[0x2e4+k];}}}
 fn apply_housing(&mut self){for (i,h) in self.housing.iter().enumerate(){if let Some(ref mut img)=self.offices[i]{w16(img,0x2d4,h.house_count);for k in 0..3{w16(img,0x2d8+2*k,h.capacity[k]);w16(img,0x2de+2*k,h.assigned[k]);}self.changed[i]=true;}}}
}
struct HiddenHousing;
impl p3_core::buildings::housing::HousingHost for HiddenHousing {
 fn add_map_object(&mut self,_:u32,_:u32,_:u32,_:u32,_:u32){unreachable!("presentación desactivada")}
}
impl p3_core::buildings::advance::AdvanceHost for BuildingCapture {
 fn find_office(&mut self,owner:u32,_town:u32,_:u32)->bool{let mut at=usize::from(self.first_office);self.selected=None;for _ in 0..self.offices.len(){let Some(Some(m))=self.offices.get(at) else{break};if u32::from(u16_at(m,0x2c4))==owner{self.selected=Some(at);break}at=usize::from(u16_at(m,0x2ca));}self.selected.is_some()}
 fn swap_office_head(&mut self,which:usize,new:u16)->u16{let i=self.selected.expect("oficina validada");let m=self.offices[i].as_mut().expect("oficina validada");let off=0x2ce+2*which;let old=u16_at(m,off);w16(m,off,new);self.changed[i]=true;old}
 fn recount_office(&mut self){unreachable!("recuento necesita estado de ciudad")}
 fn recount_office_with_site(&mut self,site:&mut p3_core::buildings::Site,town:&mut p3_core::town::Town){
  let i=self.selected.expect("oficina validada");let office=self.offices[i].as_mut().expect("oficina validada");let mut pool=p3_core::economy::office_recount::NativeFacilities{pool:&mut self.pool,relocated:Some(&mut self.relocated)};
    let result=p3_core::economy::office_recount::recount(office.as_mut_slice().try_into().expect("oficina validada"),town,site,&mut pool);
  if let Err(err)=result{self.error=Some(err)}self.changed[i]=true;self.refresh_housing(i);
 }
 fn recount_buildings(&mut self){unreachable!("recuento necesita estado de ciudad")}
 fn recount_buildings_with_site(&mut self,site:&mut p3_core::buildings::Site,town:&mut p3_core::town::Town){
  let mut hidden=HiddenHousing;let mut cx=p3_core::buildings::housing::HousingCtx{office_count:self.housing.len() as u16,offices:&mut self.housing,visual_owner:None,host:&mut hidden};
  site.recount_buildings(town,&mut cx);self.recounted=true;self.apply_housing();
 }
 fn set_progress(&mut self,_:u32,_:u32,_:u32,_:u32,_:u32){unreachable!("presentación desactivada")}
 fn add_message(&mut self,owner:u32,_:u8,town:u8,kind:u16,record:u16){self.letters.borrow_mut().push(ConstructionLetter::Building(owner,town,kind,record));}
 fn live_business_count(&self)->Option<usize>{Some(self.pool.records.len())}
 fn live_business(&self,i:u16)->Option<[u8;20]>{self.pool.records.get(i as usize).copied()}
 fn take_relocated_business(&mut self,i:u16)->Option<[u8;20]>{self.relocated.take().and_then(|p|p.get(i as usize).copied())}
 fn put_live_business(&mut self,i:u16,r:[u8;20])->Result<(),String>{let slot=self.pool.records.get_mut(i as usize).ok_or("negocio inexistente")?;*slot=r;Ok(())}
 fn sync_active_office(&mut self,image:&[u8]){self.offices[self.active]=Some(image.to_vec());self.refresh_housing(self.active);}
 fn apply_active_office(&mut self,image:&mut [u8])->Result<(),String>{if let Some(err)=self.error.take(){return Err(err)}if self.changed[self.active]{let old=self.offices[self.active].as_ref().ok_or("oficina activa inexistente")?;for (a,b) in [(0x2cc,0x2d6),(0x2d8,0x2e4),(0x2ec,0x2f0)]{image[a..b].copy_from_slice(&old[a..b]);}self.changed[self.active]=false;}Ok(())}
}
fn make_live_capture(town:&p3_core::town::Town,offices:&[Option<Vec<u8>>],pool:p3_core::economy::facility_pool::FacilityPool,active:usize)->Result<BuildingCapture,String>{
 let mut housing=Vec::with_capacity(offices.len());
 for office in offices {if let Some(img)=office {if img.len()!=OFFICE_LEN{return Err("oficina truncada".into())}
  housing.push(p3_core::buildings::housing::OfficeHousing{next:u16_at(img,0x2ca),houses_head:u16_at(img,0x2d0),house_count:u16_at(img,0x2d4),capacity:std::array::from_fn(|k|u16_at(img,0x2d8+2*k)),assigned:std::array::from_fn(|k|u16_at(img,0x2de+2*k)),level:[img[0x2e4],img[0x2e5],img[0x2e6]]});
 }else{housing.push(p3_core::buildings::housing::OfficeHousing{next:0xffff,houses_head:0xffff,..Default::default()});}}
 Ok(BuildingCapture{letters:ConstructionLetters::default(),housing,recounted:false,offices:offices.to_vec(),changed:vec![false;offices.len()],active,selected:None,error:None,first_office:town.u16(0x784),pool,relocated:None})
}
struct PersistedYard<'a>{manager:&'a mut p3_core::shipyard::ShipManager,capture:&'a mut YardCapture,count:u16,tick:u32,player:u32}
impl p3_core::economy::office::BusinessShipyardHost for PersistedYard<'_>{
 fn construction(&mut self,_:u8,_:u32){unreachable!()}
 fn repairs(&mut self,_:u8,_:u32){unreachable!()}
 fn construction_with_town(&mut self,t:&mut p3_core::town::Town,w:u32){self.manager.advance_ship_construction(t,u32::from(t.index()),w as i32,self.tick,self.player,self.count,self.capture)}
 fn repairs_with_town(&mut self,t:&mut p3_core::town::Town,w:u32){let i=u32::from(t.index());let mut cx=p3_core::shipyard::ShipyardCtx{merchant_count:self.count,host:self.capture};self.manager.advance_repairs(t,i,w as i32,&mut cx)}
 fn manager(&self)->Option<&p3_core::shipyard::ShipManager>{Some(self.manager)}
}
/// Tick con astillero S05; el modelo solo se modifica al completar la transacción.
/// Las preferencias corresponden a la categoría 2. Si el pool se relocaliza, se
/// preserva la imagen antigua para la lectura pendiente del negocio en curso.
/// El último parámetro se conserva por compatibilidad; los callbacks de obra no
/// se ejecutan fuera de la transacción y se captura internamente la carta.
pub fn tick_save_model_office_complete_yard(model:&mut p3_save::model::SaveModel,index:usize,tick:u32,difficulty:f32,player:u32,
 enabled:&[bool],_advance_host:&mut dyn p3_core::buildings::advance::AdvanceHost)->Result<YardTickResult,String>{
 use p3_core::shipyard::{ShipManager,LaunchCounters};use p3_core::economy::office::SiteBusinessHost;
 let mut staged=model.clone();let office=staged.world.offices.get(index).ok_or("índice de oficina")?;
 let office=decode_office(office.as_slice().try_into().map_err(|_|"oficina libre")?);let owner=usize::from(u16_at(&office,0x2c4));let city=usize::from(office[0x2c6]);
 let mut cur=usize::from(u16_at(&office,0x2cc));let mut seen=vec![false;staged.world.businesses.len()];
 let mut has_building=false;let mut building_budget=0usize;
 while cur<seen.len(){if seen[cur]{return Err("ciclo en cadena de negocios".into())}seen[cur]=true;let b=&staged.world.businesses[cur];if b.len()!=20{return Err("negocio truncado".into())}if b[6]==2{has_building=true;building_budget=building_budget.saturating_add(usize::from(u16_at(b,4)));}cur=usize::from(u16_at(b,8));}
 let count=staged.world.merchants.len();if count>u16::MAX as usize||enabled.len()!=count{return Err("preferencias o número de comerciantes inválidos".into())}
 let mut counters=Vec::with_capacity(count);for r in &staged.world.merchants{let m=PersistedMerchant::from_save_v85(r).ok_or("comerciante inválido")?;counters.push(LaunchCounters{flag:u16_at(&m.merchant.bytes,8),launched:u16_at(&m.merchant.bytes,0x478)});}
 let initial=counters.clone();let letters=ConstructionLetters::default();let mut capture=YardCapture{counters,letters:letters.clone(),presentation:Vec::new()};
 let raw=staged.ships.write();let parsed=p3_save::ships::Ships::parse(&raw).map_err(|e|format!("barcos: {e:?}"))?;
 let mut ships=vec![0;parsed.ships.len()*0x180];for i in 0..parsed.ships.len(){if let Some(image)=parsed.ship_image(i){ships[i*0x180..(i+1)*0x180].copy_from_slice(&image);}else{let r=parsed.ship(i);ships[i*0x180+0x134..i*0x180+0x136].copy_from_slice(&r[..2]);ships[i*0x180+6..i*0x180+8].copy_from_slice(&r[2..4]);ships[i*0x180+0x137]=r[4];}}
 let entries=(0..parsed.convoys.len()).flat_map(|i|parsed.convoy_image(i)).collect();let h=&staged.ships.header;
 let mut manager=ShipManager{ships,entries,docked_head:u16_at(h,8),free_entry_head:u16_at(h,12),ship_count:parsed.ships.len() as u16,entry_count:parsed.convoys.len() as u16};let before=manager.clone();
 let raw=staged.world.write();let world=p3_save::world::World::parse(&raw).map_err(|e|format!("mundo: {e:?}"))?;if city>=world.town_count(){return Err("ciudad inexistente".into())}
 let site=p3_core::buildings::Site{records:world.building_records(city).to_vec(),cells:world.plan_cells(city).to_vec()};
 let mut needs_housing=false;
 if has_building {let town=world.town(city);let mut at=usize::from(town.u16(0x762));let mut seen=vec![false;site.records.len()/8];while at<seen.len(){if seen[at]{return Err("ciclo en lista de obras".into())}seen[at]=true;let rec=&site.records[at*8..at*8+8];let owner=usize::from(rec[2]);let ty=rec[3];let completes=usize::from(rec[6])<=building_budget;if completes && owner<count && (4..=30).contains(&ty){let mut i=usize::from(town.u16(p3_core::town::off::FIRST_OFFICE));let mut visited=vec![false;world.office_slots()];let mut found=false;while i<visited.len(){if visited[i]{return Err("ciclo en oficinas urbanas".into())}visited[i]=true;let Some(m)=world.office_image(i) else{break};if usize::from(u16_at(&m,0x2c4))==owner{found=true;break}i=usize::from(u16_at(&m,0x2ca));}if !found{return Err("obra privada sin oficina".into())}}needs_housing|=completes&&owner>=count&&(21..=29).contains(&ty);at=usize::from(u16::from_le_bytes([rec[4],rec[5]]));}}
 if staged.world.offices.len()>u16::MAX as usize{return Err("demasiadas oficinas".into())}
 let mut housing=Vec::with_capacity(staged.world.offices.len());
 for i in 0..staged.world.offices.len(){if let Some(img)=world.office_image(i){housing.push(p3_core::buildings::housing::OfficeHousing{next:u16_at(&img,0x2ca),houses_head:u16_at(&img,0x2d0),house_count:u16_at(&img,0x2d4),capacity:std::array::from_fn(|k|u16_at(&img,0x2d8+2*k)),assigned:std::array::from_fn(|k|u16_at(&img,0x2de+2*k)),level:[img[0x2e4],img[0x2e5],img[0x2e6]]});}else{housing.push(p3_core::buildings::housing::OfficeHousing{next:0xffff,houses_head:0xffff,..Default::default()});}}
 if needs_housing {let town=world.town(city);let mut public=usize::from(town.u16(p3_core::town::off::PUBLIC_BUILDING_HEAD));let mut pv=vec![false;site.records.len()/8];while public<pv.len(){if pv[public]{return Err("ciclo en edificios públicos".into())}pv[public]=true;public=usize::from(u16_at(&site.records,public*8+4));}let mut i=usize::from(town.u16(p3_core::town::off::FIRST_OFFICE));let mut visited=vec![false;housing.len()];while i<housing.len(){if visited[i]{return Err("ciclo en oficinas urbanas".into())}visited[i]=true;if housing[i].level.iter().any(|&n|n>=5){return Err("nivel de vivienda fuera de tabla".into())}let mut h=usize::from(housing[i].houses_head);let mut hv=vec![false;site.records.len()/8];while h<hv.len(){if hv[h]{return Err("ciclo en casas de oficina".into())}hv[h]=true;h=usize::from(u16_at(&site.records,h*8+4));}i=usize::from(housing[i].next);}}
 if staged.world.header.len()<0x1e{return Err("cabecera de mundo truncada".into())}let free=u16_at(&staged.world.header,0x1c);let records=staged.world.businesses.iter().map(|r|r.as_slice().try_into().map_err(|_|"negocio truncado".to_owned())).collect::<Result<Vec<[u8;20]>,_>>()?;
 let offices=(0..world.office_slots()).map(|i|world.office_image(i)).collect::<Vec<_>>();let changed=vec![false;offices.len()];
 let mut building=BuildingCapture{letters:letters.clone(),housing,recounted:false,offices,changed,active:index,selected:None,error:None,first_office:world.town(city).u16(p3_core::town::off::FIRST_OFFICE),pool:p3_core::economy::facility_pool::FacilityPool{records,free},relocated:None};
 let finished={let mut yard=PersistedYard{manager:&mut manager,capture:&mut capture,count:count as u16,tick,player};let mut host=SiteBusinessHost{town:city as u8,site,merchant_count:count as u16,visual_owner:None,advance_host:&mut building,shipyard:&mut yard};tick_save_model_office(&mut staged,index,tick,difficulty,&mut host)?};
 if building.recounted {for (i,h) in building.housing.iter().enumerate(){let r=&staged.world.offices[i];if r.len()!=OFFICE_LEN{continue}let mut img=decode_office(r.as_slice().try_into().map_err(|_|"oficina inválida")?);w16(&mut img,0x2d4,h.house_count);for k in 0..3{w16(&mut img,0x2d8+2*k,h.capacity[k]);w16(&mut img,0x2de+2*k,h.assigned[k]);}staged.world.offices[i]=encode_office(&img).to_vec();}}
 for (i,changed) in building.changed.iter().enumerate(){if *changed&&i!=index{let m=building.offices[i].as_ref().ok_or("oficina inexistente")?;staged.world.offices[i]=encode_office(m.as_slice().try_into().map_err(|_|"oficina inválida")?).to_vec();}}
 staged.world.businesses=building.pool.records.iter().map(|r|r.to_vec()).collect();w16(&mut staged.world.header,0x1c,building.pool.free);
 staged.ships.apply_shipyard_lists(&before,&manager)?;
 for (i,c) in capture.counters.iter().enumerate(){if c.launched!=initial[i].launched{let mut m=PersistedMerchant::from_save_v85(&staged.world.merchants[i]).ok_or("comerciante inválido")?;w16(&mut m.merchant.bytes,0x478,c.launched);staged.world.merchants[i]=m.to_save_v85();}}
 if staged.world.header.len()<4{return Err("cabecera del mundo truncada".into())}let day=staged.world.header[0];let month=staged.world.header[1];let year=u16_at(&staged.world.header,2);let mut delivered=0;let mut buildings_delivered=0;
 for event in letters.borrow().iter(){let (who,town,a,b,is_launch)=match *event{ConstructionLetter::Launch(o,t,a,b)=>(usize::from(o),t,a,b,true),ConstructionLetter::Building(o,t,a,b)=>(o as usize,t,a,b,false)};if who>=count{continue}let mut m=PersistedMerchant::from_save_v85(&staged.world.merchants[who]).ok_or("comerciante inválido")?;let sent=if is_launch{crate::messages::add_ship_launch(&mut staged.message_pool,&mut m.merchant,day,month,year,town,a,b,enabled[who])?}else{crate::messages::add_building_finished(&mut staged.message_pool,&mut m.merchant,day,month,year,town,a,b,enabled[who])?};if sent{if is_launch{delivered+=1}else{buildings_delivered+=1}}staged.world.merchants[who]=m.to_save_v85();}
 let mut expansion_delivered=false;if finished{let r=staged.world.merchants.get(owner).ok_or("propietario inexistente")?;let mut m=PersistedMerchant::from_save_v85(r).ok_or("comerciante inválido")?;expansion_delivered=crate::messages::add_office_expansion(&mut staged.message_pool,&mut m.merchant,day,month,year,city as u8,enabled[owner])?;staged.world.merchants[owner]=m.to_save_v85();}
 *model=staged;Ok(YardTickResult{expansion_finished:finished,expansion_delivered,launch_letters:delivered,building_letters:buildings_delivered,presentation:capture.presentation})
}

/// Resultado transaccional del avance de obras sobre imágenes vivas de oficina.
#[derive(Clone,Debug)]
pub struct ConstructionSnapshot {
    pub town:p3_core::town::Town,
    pub site:p3_core::buildings::Site,
    pub offices:Vec<Option<Vec<u8>>>,
    pub pool:p3_core::economy::facility_pool::FacilityPool,
    /// Propietario, ciudad, tipo y registro de cada aviso de obra.
    pub letters:Vec<(u32,u8,u16,u16)>,
}
/// Ejecuta avance, recuentos y cambios del pool sobre una copia independiente.
/// Las imágenes de oficina tienen formato de memoria, no de guardado.
pub fn advance_construction_snapshot(
    town:&p3_core::town::Town,site:&p3_core::buildings::Site,offices:&[Option<Vec<u8>>],
    pool:&p3_core::economy::facility_pool::FacilityPool,merchant_count:u16,workers:u16,
)->Result<ConstructionSnapshot,String>{
    use p3_core::buildings::advance::AdvanceCtx;
    let count=usize::from(town.u16(0x776));
    if site.records.len()!=count*8{return Err("registros de obra truncados".into())}
    if offices.len()>u16::MAX as usize || pool.records.len()>u16::MAX as usize{return Err("capacidad fuera de rango".into())}
    let mut visited=vec![false;count];let mut i=usize::from(town.u16(0x762));let mut budget=u32::from(workers);
    while i<count && budget>0 {
        if visited[i]{return Err("ciclo de obras".into())}visited[i]=true;
        let r=&site.records[i*8..i*8+8];let next=usize::from(u16_at(r,4));
        if town.flags()&0x10!=0 {
            let cell=(town.u32(0x7ac) as usize).checked_mul(usize::from(r[0])).and_then(|x|x.checked_add(usize::from(r[1]))).ok_or("índice de plano")?;
            if *site.cells.get(cell).ok_or("celda fuera de plano")?<0x80 {i=next;continue}
        }
        if town.flags()&0x800!=0 && r[3]==0x30{i=next;continue}
        let progress=u32::from(r[6]);
        if progress<=5 && progress<=budget {
            budget-=progress;
            if u16::from(r[2])<merchant_count && (4..=30).contains(&r[3]) {
                let mut at=usize::from(town.u16(0x784));let mut seen=vec![false;offices.len()];let mut found=false;
                while at<offices.len(){if seen[at]{return Err("ciclo de oficinas".into())}seen[at]=true;
                    let image=offices[at].as_ref().ok_or("oficina libre en cadena")?;
                    if image.len()!=OFFICE_LEN{return Err("oficina truncada".into())}
                    if u16_at(image,0x2c4)==u16::from(r[2]){found=true;break}at=usize::from(u16_at(image,0x2ca));
                }
                if !found{return Err("obra privada sin oficina".into())}
            }
        }else{budget=budget.saturating_sub(5);}
        i=next;
    }
    let mut housing=Vec::with_capacity(offices.len());
    for office in offices {
        if let Some(img)=office {if img.len()!=OFFICE_LEN{return Err("oficina truncada".into())}
            housing.push(p3_core::buildings::housing::OfficeHousing{next:u16_at(img,0x2ca),houses_head:u16_at(img,0x2d0),house_count:u16_at(img,0x2d4),capacity:std::array::from_fn(|k|u16_at(img,0x2d8+2*k)),assigned:std::array::from_fn(|k|u16_at(img,0x2de+2*k)),level:[img[0x2e4],img[0x2e5],img[0x2e6]]});
        }else{housing.push(p3_core::buildings::housing::OfficeHousing{next:0xffff,houses_head:0xffff,..Default::default()});}
    }
    let letters=ConstructionLetters::default();
    let mut capture=BuildingCapture{letters:letters.clone(),housing,recounted:false,offices:offices.to_vec(),changed:vec![false;offices.len()],active:0,selected:None,error:None,first_office:town.u16(0x784),pool:pool.clone(),relocated:None};
    let mut t=town.clone();let mut s=site.clone();
    {let mut cx=AdvanceCtx{merchant_count,visual_owner:None,host:&mut capture};s.advance_construction(&mut t,&mut cx,u32::from(workers));}
    if let Some(e)=capture.error{return Err(e)}
    let events=letters.borrow().iter().filter_map(|e|match *e {ConstructionLetter::Building(owner,city,kind,record)=>Some((owner,city,kind,record)),_=>None}).collect();
    Ok(ConstructionSnapshot{town:t,site:s,offices:capture.offices,pool:capture.pool,letters:events})
}

/// Finaliza obras directamente en un modelo S05. Todos los efectos se confirman juntos.
/// `enabled` contiene las preferencias de mensajes de la categoría de obras.
pub fn advance_save_model_construction(model:&mut p3_save::model::SaveModel,city:usize,workers:u16,enabled:&[bool])->Result<usize,String>{
    use p3_core::economy::facility_pool::FacilityPool;
    let mut staged=model.clone();let raw=staged.world.write();
    let world=p3_save::world::World::parse(&raw).map_err(|e|format!("mundo: {e:?}"))?;
    if city>=world.town_count(){return Err("ciudad inexistente".into())}
    if enabled.len()!=staged.world.merchants.len() || enabled.len()>u16::MAX as usize{return Err("preferencias o número de comerciantes inválidos".into())}
    if staged.world.header.len()<0x1e{return Err("cabecera de mundo truncada".into())}
    let town=world.town(city);let site=p3_core::buildings::Site{records:world.building_records(city).to_vec(),cells:world.plan_cells(city).to_vec()};
    let offices=(0..world.office_slots()).map(|i|world.office_image(i)).collect::<Vec<_>>();
    let records=staged.world.businesses.iter().map(|r|r.as_slice().try_into().map_err(|_|"negocio truncado".to_owned())).collect::<Result<Vec<[u8;20]>,_>>()?;
    let pool=FacilityPool{records,free:u16_at(&staged.world.header,0x1c)};
    let outcome=advance_construction_snapshot(&town,&site,&offices,&pool,enabled.len() as u16,workers)?;
    staged.world.towns[city].apply_town(&town,&outcome.town,&outcome.site)?;
    for (i,img) in outcome.offices.iter().enumerate(){if let Some(img)=img {
        let memory:&[u8;OFFICE_LEN]=img.as_slice().try_into().map_err(|_|"oficina truncada")?;
        staged.world.offices[i]=encode_office(memory).to_vec();
    }}
    staged.world.businesses=outcome.pool.records.iter().map(|r|r.to_vec()).collect();w16(&mut staged.world.header,0x1c,outcome.pool.free);
    let day=staged.world.header[0];let month=staged.world.header[1];let year=u16_at(&staged.world.header,2);let mut delivered=0;
    for (owner,town,kind,record) in outcome.letters {
        let i=owner as usize;if i>=enabled.len(){continue}
        let mut m=PersistedMerchant::from_save_v85(&staged.world.merchants[i]).ok_or("comerciante truncado")?;
        if crate::messages::add_building_finished(&mut staged.message_pool,&mut m.merchant,day,month,year,town,kind,record,enabled[i])? {delivered+=1}
        staged.world.merchants[i]=m.to_save_v85();
    }
    *model=staged;Ok(delivered)
}
