//! Planificación y despacho de la casa del prestamista municipal.
use p3_core::{buildings::{construction::{BuildCtx,BuildHost,Hook,PlanCtx},Site},pump::Operation,rng::CrtRand,town::{off,Town}};

#[derive(Default)]
struct Host { operation:Option<Operation>, unexpected:bool }
impl BuildHost for Host {
    fn call(&mut self,hook:Hook,args:&[u32])->u32 {
        if hook==Hook::EnqueueOperation && args.len()==5 {
            self.operation=Some(Operation([args[0],args[1],args[2],args[3],args[4]]));0
        } else {self.unexpected=true;0}
    }
    fn office_building_head(&mut self,_:u32)->u16 {self.unexpected=true;u16::MAX}
    fn place_defense_pieces(&mut self,site:&mut Site,town:&mut Town,point:u32) {
        site.place_town_defense_pieces(town,point,&p3_core::buildings::walls::DEFAULT_KINDS);
    }
}
pub(crate) fn validate(t:&Town,s:&Site,merchant_count:u16)->Result<(),String>{
    let rows=t.i32(off::PLAN_ROWS);let cols=t.i32(off::PLAN_STRIDE);let count=t.u16(off::BUILDING_RECORD_COUNT);let free=t.u16(off::FREE_BUILDING_HEAD);
    if merchant_count>255||rows<=0||cols<=0||rows>255||cols>255||i32::from(t.u8(0x800))>=rows||i32::from(t.u8(0x801))>=cols||s.cells.len()<(rows*cols) as usize||s.records.len()<usize::from(count)*8||free>count||(free==count&&count>u16::MAX-128){return Err("plano o pool de prestamista inválido".into())}
    Ok(())
}

/// El tipo 0x25 parte del centro del plano y no consume `rand`.
pub fn plan(t:&Town,s:&Site,merchant_count:u16)->Result<Option<Operation>,String>{
    Ok(plan_kind(t,s,merchant_count,0x25)?.1)
}

/// Edificios municipales de partida fija: la búsqueda de solar no consume rand.
pub fn plan_kind(t:&Town,s:&Site,merchant_count:u16,kind:u8)->Result<(u32,Option<Operation>),String>{
    validate(t,s,merchant_count)?;
    if !matches!(kind,1|3|0x1f..=0x25) {return Err("tipo municipal no admitido".into())}
    if kind==1 && (i32::from(t.u8(0x80a))>=t.i32(off::PLAN_ROWS)||i32::from(t.u8(0x80b))>=t.i32(off::PLAN_STRIDE)){return Err("ancla municipal inválida".into())}
    let mut town=t.clone();let mut site=s.clone();let mut host=Host::default();let mut gold=0;let mut income=0;let classes=[0;256];let mut rng=CrtRand::default();
    let build=BuildCtx{merchant_count,visual_owner:None,special:0xe0,owner_class:&classes,merchant_gold:&mut gold,merchant_income:&mut income,host:&mut host};
    let before=rng.state;
    let mut ctx=PlanCtx{build,rng:&mut rng,day:0,month:0,alt_special:0x20};
    let result=site.place_planned_building(&mut town,&mut ctx,kind,false);
    if host.unexpected||rng.state!=before{return Err("dependencia inesperada al planificar obra municipal".into())}Ok((result,host.operation))
}

/// El puerto solo acepta la operación municipal de prestamista generada por el planificador.
pub fn start(t:&mut Town,s:&mut Site,merchant_count:u16,op:&Operation)->Result<bool,String>{
    validate(t,s,merchant_count)?;
    let b=op.0[1].to_le_bytes();let ty=op.0[2].to_le_bytes();
    if op.0[0]!=0x22||b[0]!=0xff||b[1]!=t.u8(0x2c0)||!matches!(ty[0],1|3|0x15|0x18|0x1b|0x1f..=0x25){return Err("operación municipal incompatible".into())}
    if i32::from(b[2])>=t.i32(off::PLAN_ROWS)||i32::from(b[3])>=t.i32(off::PLAN_STRIDE){return Err("coordenadas de prestamista inválidas".into())}
    let mut staged_t=t.clone();let mut staged_s=s.clone();let mut host=Host::default();let mut gold=0;let mut income=0;let classes=[0;256];
    let mut ctx=BuildCtx{merchant_count,visual_owner:None,special:0xe0,owner_class:&classes,merchant_gold:&mut gold,merchant_income:&mut income,host:&mut host};
    let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| staged_s.start_building_construction(&mut staged_t,&mut ctx,0xff,b[2],b[3],ty[0],u32::from(ty[1])))).map_err(|_|"obra municipal fuera del plano o del pool")?;
    if host.unexpected{return Err("dependencia inesperada al construir prestamista".into())}*t=staged_t;*s=staged_s;Ok(result)
}
