//! Preparación transaccional de los eventos decenales de capitanes de C10.
//! El llamante conecta el resto de `0x03` antes de publicar este resultado.
use p3_core::{ai::reputation::Merchant,economy::{captain_operations::captain_skill_gain,ten_day_captains::{self,Context,Host}},pump::Operation,rng::CrtRand,scheduler::Scheduler};
use p3_core::{economy::{ship_trade::ShipCargo,ten_day},town::Town};
use p3_save::ships::NavigationSnapshot;

#[derive(Clone,Debug)]
pub struct PreparedCaptains {pub scheduler:Scheduler,pub captains:Vec<u8>,pub operations:Vec<Operation>,pub notices:Vec<p3_core::economy::captain_operations::SkillNotice>,pub rng:Option<CrtRand>}

struct Adapter<'a> {scheduler:Scheduler,head:u16,ships:&'a[u8],merchants:&'a[Merchant],towns:usize,rng:Option<CrtRand>,operations:Vec<Operation>,notices:Vec<p3_core::economy::captain_operations::SkillNotice>}
impl Host for Adapter<'_> {
    fn random(&mut self)->Result<u32,String>{Ok(self.rng.as_mut().ok_or("estado CRT ausente")?.rand() as u32)}
    fn schedule_retirement(&mut self,due:u32,ship:u16,captain:u16)->Result<(),String>{
        if self.scheduler.free_head>=self.scheduler.capacity{return Err("crecimiento del planificador decenal pendiente".into())}
        self.scheduler.dispatching=0;let id=self.scheduler.add_task(due);self.scheduler.dispatching=1;
        if self.scheduler.head!=self.head{return Err("la jubilación reordena la tarea activa".into())}
        let task=&mut self.scheduler.records[usize::from(id)];task.kind=0x27;task.set_u32_at(8,u32::from(ship));task.set_u32_at(0xc,u32::from(captain));Ok(())
    }
    fn direct_skill(&mut self,op:Operation,captains:&mut[u8])->Result<(),String>{
        if let Some(notice)=captain_skill_gain(op,self.ships,captains,self.merchants,self.merchants.len() as u16,self.towns).map_err(str::to_owned)?{self.notices.push(notice)}Ok(())
    }
    fn enqueue(&mut self,op:Operation)->Result<(),String>{self.operations.push(op);Ok(())}
}

/// Simula la fase sin modificar sus entradas. La tarea activa y sus nuevos
/// eventos se devuelven juntos; no ejecuta ni publica el resto de `0x03`.
pub fn prepare_captains(scheduler:&Scheduler,captains:&[u8],ctx:&Context<'_>,rng:Option<CrtRand>,town_count:usize)->Result<PreparedCaptains,String>{
    if scheduler.dispatching!=1||scheduler.head>=scheduler.capacity{return Err("tarea decenal no despachada".into())}
    let head=scheduler.head;let mut task=*scheduler.records.get(usize::from(head)).ok_or("tarea decenal ausente")?;if task.kind!=3{return Err("tarea decenal inválida".into())}
    let mut people=captains.to_vec();let mut host=Adapter{scheduler:scheduler.clone(),head,ships:ctx.ships,merchants:ctx.merchants,towns:town_count,rng,operations:Vec::new(),notices:Vec::new()};
    ten_day_captains::update(&mut task,&mut people,ctx,&mut host)?;
    // `add_task` puede actualizar el enlace de la tarea activa al insertar
    // una jubilación después de ella; no restaurar el enlace anterior.
    task.next=host.scheduler.records[usize::from(head)].next;
    host.scheduler.records[usize::from(head)]=task;
    Ok(PreparedCaptains{scheduler:host.scheduler,captains:people,operations:host.operations,notices:host.notices,rng:host.rng})
}

pub struct TenDayInputs<'a> {
    pub snapshot:&'a NavigationSnapshot,pub towns:&'a[Town],pub sites:&'a[p3_core::buildings::Site],pub merchants:&'a[Merchant],pub cargo:&'a[ShipCargo],pub offices:&'a[u8],
    pub band_heads:Option<[Option<u16>;5]>,pub distance_data:Option<&'a p3_core::ships::ai_dock::AiDockData>,pub locations:Option<&'a [p3_core::ai::settlement::SettlementLocation;40]>,
    pub message_pool:&'a[u8],pub pending_operations:usize,pub tick:u32,pub day_of_year:u16,
    pub candidate_names:Option<&'a[u8]>,pub candidate_looks:Option<(u16,u16)>,
    pub network_client:bool,pub world_8a:u8,pub alderman:u8,pub operations_mode:u32,pub rng:Option<CrtRand>,pub celebration:Option<p3_core::economy::celebration::CelebrationState>,pub player:u32,
}
pub struct PreparedTenDay {pub scheduler:Scheduler,pub captains:Vec<u8>,pub towns:Vec<Town>,pub merchants:Vec<Merchant>,pub message_pool:Vec<u8>,pub operations:Vec<Operation>,pub rng:Option<CrtRand>,pub celebration:Option<p3_core::economy::celebration::CelebrationState>}

struct TenDayAdapter<'a>{inputs:&'a TenDayInputs<'a>,prepared:PreparedTenDay,head:u16}
impl ten_day::Host for TenDayAdapter<'_>{
    type Error=String;
    fn event(&mut self,event:ten_day::Event,task:&mut p3_core::scheduler::TaskRecord)->Result<(),String>{
        match event {
            ten_day::Event::ExpireMail(owner)=>{let merchant=self.prepared.merchants.get_mut(usize::from(owner)).ok_or("comerciante decenal ausente")?;p3_state::messages::purge_expired(&mut self.prepared.message_pool,merchant,self.inputs.tick)?;}
            ten_day::Event::RecountIncoming=>{
                let snap=self.inputs.snapshot;if snap.ships.len()%0x180!=0||self.inputs.cargo.len()!=snap.ships.len()/0x180{return Err("vistas de carga naval incompletas".into())}
                for (i,image) in snap.ships.chunks_exact(0x180).enumerate(){for ware in 0..24{let at=0x54+4*ware;let stock=i32::from_le_bytes(image[at..at+4].try_into().unwrap());if stock!=self.inputs.cargo[i].stock[ware]{return Err(format!("carga naval desincronizada: barco {i}, mercancía {ware}"))}}}
                let mut bytes=self.prepared.towns.iter().flat_map(|t|t.0.iter().copied()).collect::<Vec<_>>();p3_core::ships::return_convoy::recount_incoming(&snap.manager,&snap.ships,&snap.convoys,&mut bytes).map_err(str::to_owned)?;for(t,raw)in self.prepared.towns.iter_mut().zip(bytes.chunks_exact(0x9f8)){t.0[0x670..0x6d0].copy_from_slice(&raw[0x670..0x6d0]);}
            }
            ten_day::Event::CaptainEvents=>{
                self.prepared.scheduler.records[usize::from(self.head)]=*task;
                let ctx=Context{tick:self.inputs.tick,day_of_year:self.inputs.day_of_year,network_client:self.inputs.network_client,queued:self.inputs.pending_operations.min(u16::MAX as usize) as u16,merchants:&self.prepared.merchants,ships:&self.inputs.snapshot.ships,offices:self.inputs.offices};
                let out=prepare_captains(&self.prepared.scheduler,&self.prepared.captains,&ctx,self.prepared.rng,self.prepared.towns.len())?;let date=p3_core::calendar::Calendar::for_tick(self.inputs.tick);for notice in &out.notices{let merchant=self.prepared.merchants.get_mut(usize::from(notice.merchant)).ok_or("comerciante de aviso decenal ausente")?;p3_state::messages::add_captain_skill_notice(&mut self.prepared.message_pool,merchant,date.day,date.month,date.year,notice)?;}*task=out.scheduler.records[usize::from(self.head)];self.prepared.scheduler=out.scheduler;self.prepared.captains=out.captains;self.prepared.operations.extend(out.operations);self.prepared.rng=out.rng;
            }
            ten_day::Event::AssignTownSlots=>{let locations=self.inputs.locations.ok_or("regiones de ciudades no reconstruidas")?;let op=ten_day::assign_town_slots(&self.prepared.towns,locations,&mut self.prepared.rng).map_err(str::to_owned)?;self.prepared.operations.push(op);}
            ten_day::Event::TavernPassenger=>{let op=prepare_passenger(self.inputs.snapshot,self.prepared.towns.as_slice(),self.inputs.band_heads,self.inputs.distance_data,self.inputs.tick,&mut self.prepared.rng)?;self.prepared.operations.push(op);}
            ten_day::Event::CriminalInvestigation=>{let count=u16::try_from(self.prepared.merchants.len()).map_err(|_|"demasiados comerciantes")?;if let Some(op)=ten_day::criminal_investigation(task,&self.prepared.merchants,count,self.prepared.towns.first(),&mut self.prepared.rng).map_err(str::to_owned)?{self.prepared.operations.push(op)}}
            ten_day::Event::TownCelebration=>{let state=self.prepared.celebration.ok_or("estado transitorio de fiestas ausente")?;let (state,ops,rng)=p3_core::economy::celebration::prepare(state,self.inputs.tick,&self.prepared.towns,self.inputs.offices,&self.prepared.merchants,self.inputs.player,&self.prepared.scheduler,self.prepared.rng).map_err(str::to_owned)?;self.prepared.celebration=Some(state);self.prepared.operations.extend(ops);self.prepared.rng=rng;}
            ten_day::Event::Plague=>{if let Some(op)=ten_day::plague(&self.prepared.towns,self.inputs.day_of_year,self.inputs.tick,&mut self.prepared.rng).map_err(str::to_owned)?{self.prepared.operations.push(op)}}
            ten_day::Event::Fire=>{if let Some(op)=ten_day::fire(&self.prepared.towns,self.inputs.sites,self.inputs.tick,&mut self.prepared.rng).map_err(str::to_owned)?{self.prepared.operations.push(op)}}
            ten_day::Event::YearAuctions=>{let year=p3_core::calendar::Calendar::for_tick(self.inputs.tick).year;let ops=ten_day::year_auctions(&self.prepared.scheduler,&self.prepared.towns,self.inputs.tick,year,&mut self.prepared.rng).map_err(str::to_owned)?;self.prepared.operations.extend(ops);}
            ten_day::Event::NotableDemand=>{if let Some(op)=ten_day::notable_demand(&self.prepared.towns,&self.prepared.merchants,self.inputs.alderman,self.inputs.tick,&mut self.prepared.rng).map_err(str::to_owned)?{self.prepared.operations.push(op)}}
            ten_day::Event::WarehouseTheft=>{let ops=ten_day::warehouse_theft(&self.prepared.merchants,self.inputs.offices,&mut self.prepared.rng).map_err(str::to_owned)?;self.prepared.operations.extend(ops);}
        }Ok(())
    }
    fn enqueue(&mut self,op:Operation)->Result<(),String>{self.prepared.operations.push(op);Ok(())}
    fn rand(&mut self)->Result<u32,String>{Ok(self.prepared.rng.as_mut().ok_or("estado CRT ausente")?.rand() as u32)}
    fn captain_candidate(&mut self,index:u8)->Result<bool,String>{
        use p3_core::ai::candidate::{CandidateEnv,CandidateHost,CaptainEntry};
        struct Capture<'a>{names:&'a[u8],rng:CrtRand,ops:Vec<Operation>}
        impl CandidateHost for Capture<'_>{fn name_length(&mut self,index:u32)->usize{usize::from(self.names[index as usize])}fn rand(&mut self)->u32{self.rng.rand() as u32}fn enqueue(&mut self,r:&[u8;20]){let mut words=[0u32;5];for i in 0..5{words[i]=u32::from_le_bytes(r[4*i..4*i+4].try_into().unwrap())}self.ops.push(Operation(words))}}
        let merchant=self.prepared.merchants.get(usize::from(index)).ok_or("comerciante candidato ausente")?;if merchant.bytes.len()<0x650{return Err("comerciante candidato truncado".into())}let limit=self.inputs.tick.wrapping_sub(0x11700);if self.prepared.towns.len() as u32<=u32::from(merchant.bytes[0x31])||self.inputs.tick<=merchant.i32(0x28)as u32||limit<merchant.i32(0x2c)as u32{return Ok(false)}
        let names=self.inputs.candidate_names.ok_or("banco de nombres de candidato ausente")?;if names.len()!=256{return Err("banco de nombres de candidato truncado".into())}let(a,b)=self.inputs.candidate_looks.ok_or("contadores de nombres de candidato ausentes")?;if a==0||a>128||b==0||b>128{return Err("contadores de nombres de candidato inválidos".into())}if u32::from(names[usize::from(merchant.bytes[0x1c])]%3)+2<=u32::from(merchant.bytes[0x36]){return Ok(false)}
        if self.prepared.captains.len()%16!=0||self.prepared.captains.len()/16>u16::MAX as usize{return Err("pool de capitanes truncado".into())}let caps=self.prepared.captains.chunks_exact(16).map(|r|CaptainEntry{next:u16::from_le_bytes([r[0],r[1]]),look:r[2],hired:u32::from_le_bytes(r[4..8].try_into().unwrap())}).collect::<Vec<_>>();let mut seen=vec![false;caps.len()];let mut at=usize::from(merchant.u16(0x34));let mut available=[false;256];available[..usize::from(a)].fill(true);available[128..].fill(true);while at<caps.len(){if seen[at]{return Err("lista de candidatos cíclica".into())}seen[at]=true;if limit<caps[at].hired{return Ok(false)}available[usize::from(caps[at].look)]=false;at=usize::from(caps[at].next)}
        let state=self.prepared.rng.ok_or("estado CRT ausente")?;
        {let mut probe=state;let mut look=probe.rand() as u8;let mut visited=[false;256];while !available[usize::from(look)]{if visited[usize::from(look)]{return Err("selección de aspecto de candidato cíclica".into())}visited[usize::from(look)]=true;look=if look>0x7f{((u16::from(look&0x7f)%b)as u8).wrapping_sub(0x7f)}else{((u16::from(look)+1)%a)as u8};}}
        let env=CandidateEnv{town_count:self.prepared.towns.len() as u32,clock:self.inputs.tick,look_a:u32::from(a),look_b:b,captain_count:caps.len() as u16,captains:&caps};let mut capture=Capture{names,rng:state,ops:Vec::new()};let result=merchant.try_enqueue_captain_candidate(index,&env,&mut capture);self.prepared.rng=Some(capture.rng);self.prepared.operations.extend(capture.ops);Ok(result)
    }
}

/// Prepara una oferta con la flota y la matriz reconstruidas de la sesión.
pub fn prepare_passenger(snapshot:&NavigationSnapshot,towns:&[Town],heads:Option<[Option<u16>;5]>,distances:Option<&p3_core::ships::ai_dock::AiDockData>,tick:u32,rng:&mut Option<CrtRand>)->Result<Operation,String>{
    let heads=heads.ok_or("bandas piratas no reconstruidas")?;let count=ten_day::count_band_ships(&heads,&snapshot.ships).map_err(str::to_owned)?;let data=distances.ok_or("matriz naval no reconstruida")?;if data.size as usize!=towns.len(){return Err("matriz naval incompatible".into())}let matrix=if data.distances.is_empty(){None}else{Some(data.distances.as_slice())};ten_day::tavern_passenger(towns,matrix,count,tick,rng).map_err(str::to_owned)
}

/// Prepara 0x03 completo únicamente si no alcanza servicios todavía ausentes.
/// El cliente y los mundos que no alcanzan otros servicios siguen la vía cubierta.
pub fn prepare_ten_day(scheduler:&Scheduler,input:&TenDayInputs<'_>)->Result<PreparedTenDay,String>{
    let head=scheduler.head;if scheduler.dispatching!=1||head>=scheduler.capacity{return Err("tarea decenal no despachada".into())}
    if input.merchants.len()>u16::MAX as usize||input.towns.len()>u32::MAX as usize{return Err("mundo decenal demasiado grande".into())}
    let mut task=*scheduler.records.get(usize::from(head)).ok_or("tarea decenal ausente")?;if task.kind!=3{return Err("tarea decenal inválida".into())}
    let prepared=PreparedTenDay{scheduler:scheduler.clone(),captains:input.snapshot.captains.clone(),towns:input.towns.to_vec(),merchants:input.merchants.to_vec(),message_pool:input.message_pool.to_vec(),operations:Vec::new(),rng:input.rng,celebration:input.celebration};
    let mut host=TenDayAdapter{inputs:input,prepared,head};let ctx=ten_day::Context{day_of_year:input.day_of_year,merchants:input.merchants.len() as u16,towns:input.towns.len() as u32,network_client:input.network_client,world_8a:input.world_8a,operations_mode:input.operations_mode};ten_day::update(&mut task,ctx,&mut host)?;host.prepared.scheduler.records[usize::from(head)]=task;Ok(host.prepared)
}

#[cfg(test)]
mod candidate_phase_tests{
 use super::*;use p3_core::{economy::ten_day::Host,scheduler::TaskRecord};use p3_save::ships::NavigationLists;use std::io::Write;
 fn hex(b:&[u8])->String{b.iter().map(|v|format!("{v:02x}")).collect()}
 #[test]
 fn decennial_candidate_phase_matches_original(){
  let (Ok(exe),Ok(python))=(std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY"))else{return};let mut cases=Vec::new();let mut inputs=Vec::new();for i in 0..512u32{let tick=0x200000+i*256;let names=vec![(i%12)as u8;256];let mut merchants=vec![Merchant{bytes:vec![0;0x650]};8];let mut caps=vec![0u8;4*16];for k in 0..4{caps[k*16..k*16+2].fill(255);caps[k*16+2]=(i as u8).wrapping_add(k as u8);caps[k*16+4..k*16+8].copy_from_slice(&(if (i+k as u32)%5==0{tick-100}else{0}).to_le_bytes())}for(j,m)in merchants.iter_mut().enumerate(){m.set_u16(0x34,if (i+j as u32)%4==0{(j%4)as u16}else{u16::MAX});m.set_i32(0x28,if (i+j as u32)%9==0{tick as i32}else{0});m.set_i32(0x2c,0);m.bytes[0x31]=if (i+j as u32)%7==0{255}else{0};m.bytes[0x36]=((i+j as u32)%5)as u8;m.bytes[0x1c]=(i+j as u32)as u8;}let seed=i.wrapping_mul(0x192875).wrapping_add(42);cases.push(serde_json::json!({"merchants":hex(&merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>()),"captains":hex(&caps),"towns":20,"tick":tick,"name_length":i%12,"look_a":80,"look_b":80,"seed":seed}));inputs.push((merchants,caps,names,tick,seed));}
  let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_candidate_phase.py");let mut child=std::process::Command::new(python).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"binary":exe,"cases":cases}).to_string().as_bytes()).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let expected:serde_json::Value=serde_json::from_slice(&out.stdout).unwrap();let mut emitted=0;
  for(i,(merchants,caps,names,tick,seed))in inputs.into_iter().enumerate(){let towns=vec![Town::default();20];let snapshot=NavigationSnapshot{manager:vec![],ships:vec![],captains:caps.clone(),convoys:vec![],routes:vec![],lists:NavigationLists{docked_head:u16::MAX,sea_head:u16::MAX,docked:vec![],sea:vec![]}};let scheduler=Scheduler::new();let input=TenDayInputs{snapshot:&snapshot,towns:&towns,sites:&[],merchants:&merchants,cargo:&[],offices:&[],band_heads:None,distance_data:None,locations:None,message_pool:&[],pending_operations:0,tick,day_of_year:0,candidate_names:Some(&names),candidate_looks:Some((80,80)),network_client:false,world_8a:0,alderman:0,operations_mode:0,rng:Some(CrtRand{state:seed}),celebration:None,player:0};let prepared=PreparedTenDay{scheduler,captains:caps,towns:towns.clone(),merchants:merchants.clone(),message_pool:vec![],operations:vec![],rng:input.rng,celebration:None};let mut host=TenDayAdapter{inputs:&input,prepared,head:0};let mut index=(host.rand().unwrap()%merchants.len() as u32)as u8;let mut attempts=host.rand().unwrap()&7;loop{attempts+=1;if host.captain_candidate(index).unwrap(){break}index=((u32::from(index)+1)%merchants.len() as u32)as u8;if attempts>=10{break}}let actual=host.prepared.operations.iter().map(|op|hex(&op.0.iter().flat_map(|v|v.to_le_bytes()).collect::<Vec<_>>())).collect::<Vec<_>>();emitted+=actual.len();let x=expected[i]["ops"].as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect::<Vec<_>>();assert_eq!(actual,x,"ops {i}");assert_eq!(u64::from(host.prepared.rng.unwrap().state),expected[i]["seed"].as_u64().unwrap(),"rng {i}");}assert!(emitted>100);let _=TaskRecord::default();
 }
}

#[cfg(test)]
mod direct_notice_tests {
 use super::*;
 #[test]
 fn ai_chain_with_human_owned_ship_produces_direct_skill_notice(){
  let mut scheduler=Scheduler::new();let head=scheduler.add_task(0x800000);scheduler.records[head as usize].kind=3;scheduler.dispatching=1;
  let mut merchants=vec![Merchant{bytes:vec![0;0x650]};2];merchants[0].set_u16(8,1);merchants[0].set_u16(0xe,0);merchants[0].set_u16(0xc,u16::MAX);merchants[1].set_u16(0xe,u16::MAX);merchants[1].set_u16(0xc,u16::MAX);
  let mut ships=vec![0u8;0x180];ships[0]=1;ships[4..6].copy_from_slice(&u16::MAX.to_le_bytes());ships[0x42..0x44].copy_from_slice(&0u16.to_le_bytes());ships[0x15e..0x160].copy_from_slice(&11u16.to_le_bytes());
  let mut captains=vec![0u8;16];captains[4..8].copy_from_slice(&0x800000u32.to_le_bytes());captains[2]=3;captains[3]=4;
  let ctx=Context{tick:0x800000,day_of_year:0,network_client:false,queued:0,merchants:&merchants,ships:&ships,offices:&[]};
  let out=prepare_captains(&scheduler,&captains,&ctx,None,1).unwrap();assert_eq!(out.notices.len(),1);let n=out.notices[0];assert_eq!((n.merchant,n.town,n.look_a,n.look_b,n.ship_name,n.wage),(1,0,3,4,11,10));assert_eq!(&out.captains[9..12],&[8,8,8]);
 }
}
