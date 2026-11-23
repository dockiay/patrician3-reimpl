//! Puente explícito del día de comerciante hacia el estado vivo C10.
use crate::Sim;
use p3_core::ai::{asset_values::{update_merchant_daily_with_query,DailyOfficeAssets,MerchantDailyAssets},daily::{DailyEnv,DailyOffice,DailyShip,Facility},misc::RankEnv};
fn word(b:&[u8],p:usize)->u16{u16::from_le_bytes([b[p],b[p+1]])}
fn int(b:&[u8],p:usize)->i32{i32::from_le_bytes(b[p..p+4].try_into().unwrap())}

impl crate::NullHost{
 /// `4FDF50` usa D0 y D4: nombres y apellidos ordinarios, sin el banco pirata D6.
 pub(crate) fn pirate_captain_name_counts(&self)->Option<(u16,u16)>{self.foundation_names.or_else(||self.expedition_resources.as_ref().map(|v|(v.first_names,v.last_names)))}
 /// `4E8CA5` usa NameBank+D0, pero `4E8CC0` usa D6: apellidos normales
 /// más los nombres piratas. D4 solo cuenta los apellidos normales.
 /// `0x006DDB74`: apellidos ordinarios, sin las veinte ranuras personalizadas. El casamentero
 /// (`0x004E8234` oferta 0x21 y dossier 0x22) divide por este contador, no por el total del banco.
 pub(crate) fn ordinary_family_name_count(&self)->Option<u16>{
  self.foundation_names.map(|v|v.1).or_else(||self.expedition_resources.as_ref().map(|v|v.last_names))
 }
 pub(crate) fn loan_applicant_name_counts(&self)->Option<(u16,u16)>{
  let (first,ordinary)=self.foundation_names.or_else(||self.expedition_resources.as_ref().map(|v|(v.first_names,v.last_names)))?;
  // El banco de textos termina con las veinte ranuras personalizadas; D6
  // señala el comienzo de esas ranuras, no el tamaño total del vector.
  let all=match &self.family_names{Some(names)=>u16::try_from(names.len().checked_sub(20)?).ok()?,None=>ordinary};
  Some((first,all))
 }
}

#[cfg(test)]
mod loan_name_resource_tests{
 use std::{io::Write,process::{Command,Stdio}};
 #[test]
 fn loaded_name_bank_matches_original_loan_applicants(){
  let(Ok(root),Ok(binary),Ok(python))=(std::env::var("P3_SAVES"),std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY"))else{return};
  let archive_path=format!("{root}/iso-english/app/p2arch0_eng.cpr");
  let archive=std::fs::read(&archive_path).unwrap();let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
  let mut sim=crate::Sim::from_save_bytes(&raw).unwrap();sim.configure_hanse_fleet_archives(&[&archive],"scripts/Namen_eng.ini",false,36).unwrap();
  let names=sim.host.loan_applicant_name_counts().unwrap();
  let cases=(0..256u32).map(|i|serde_json::json!({"seed":i.wrapping_mul(0x9e3779b9),"slot":i%6,"population":[i*17,i*31]})).collect::<Vec<_>>();
  let input=serde_json::json!({"binary":binary,"archives":[archive_path],"cases":cases});
  let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_loan_name_resources.py");
  let mut child=Command::new(python).arg(script).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
  child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
  let expected:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(u64::from(names.0),expected["name_counts"]["given"]);assert_eq!(u64::from(names.1),expected["name_counts"]["all_family"]);
  assert!(expected["name_counts"]["all_family"].as_u64().unwrap()>expected["name_counts"]["ordinary_family"].as_u64().unwrap());
  for i in 0..256u32{
   let mut rng=p3_core::rng::CrtRand{state:i.wrapping_mul(0x9e3779b9)};let got=p3_core::economy::loan_slots::applicant([i*17,i*31],i%6,names,&mut rng).unwrap();
   assert_eq!(serde_json::json!(got),expected["cases"][i as usize]["bytes"],"caso {i}");assert_eq!(u64::from(rng.state),expected["cases"][i as usize]["seed"]);
  }
 }
}

/// La carga original 0x005121C0/0x0051E440 lee los 40 registros de
/// StadtDaten.ini: AnfahrtXPos/YPos en +0x20/+0x24 y Region en +0x30
/// respecto a 0x006DDB90 (la tabla consultada empieza en +0x20).
pub(crate) fn parse_settlement_locations(data:&[u8])->Result<[p3_core::ai::settlement::SettlementLocation;40],String>{
 use p3_core::ai::settlement::SettlementLocation;use std::collections::BTreeMap;
 let mut fields=BTreeMap::new();let mut section=String::new();
 for raw in data.split(|&b|b==b'\n'||b==b'\r'){let line=String::from_utf8_lossy(raw);let line=line.trim();if line.starts_with('[')&&line.ends_with(']'){section=line[1..line.len()-1].to_ascii_lowercase()}else if let Some((k,v))=line.split_once('='){fields.insert((section.clone(),k.trim().to_ascii_lowercase()),v.trim().to_owned());}}
 let mut out=[SettlementLocation{x:0,y:0,kind:0};40];
 for (i,loc) in out.iter_mut().enumerate(){let section=format!("stadt{i}");let read=|key:&str|->Result<i32,String>{fields.get(&(section.clone(),key.into())).ok_or_else(||format!("falta {section}.{key}"))?.parse().map_err(|_|format!("valor inválido: {section}.{key}"))};let region=read("region")?;loc.x=read("anfahrtxpos")?;loc.y=read("anfahrtypos")?;loc.kind=region as u8;}
 Ok(out)
}
/// `0x0051E62B..0x0051E642` compara con 1 y usa SETE: +0x2F es
/// `AmFluss == 1`; no es la región (+0x30). Contraste de las 40 filas reales
/// en `c10_real_town_resources.py`, ejecutando el cargador original.
pub(crate) fn parse_settlement_inland(data:&[u8])->Result<[u8;40],String>{
 let mut values=[0i32;40];let mut current=None;
 for raw in data.split(|&b|b==b'\n'||b==b'\r'){let line=String::from_utf8_lossy(raw);let line=line.trim();if line.starts_with('[')&&line.ends_with(']'){let name=line[1..line.len()-1].to_ascii_lowercase();current=name.strip_prefix("stadt").and_then(|v|v.parse::<usize>().ok()).filter(|&i|i<40);}else if let(Some(i),Some((key,value)))=(current,line.split_once('=')){if key.trim().eq_ignore_ascii_case("AmFluss"){values[i]=value.trim().parse().map_err(|_|format!("AmFluss inválido en Stadt{i}"))?;}}}
 Ok(values.map(|value|u8::from(value==1)))
}

/// 0x0051E655 carga AusrichtungDesMeeres en TownData+0x31; la clave
/// ausente tiene valor cero y solo se conserva el byte bajo.
pub(crate) fn parse_settlement_sea_directions(data:&[u8])->Result<[u8;40],String>{
 let mut values=[0u8;40];let mut current=None;
 for raw in data.split(|&b|b==b'\n'||b==b'\r'){let line=String::from_utf8_lossy(raw);let line=line.trim();if line.starts_with('[')&&line.ends_with(']'){let name=line[1..line.len()-1].to_ascii_lowercase();current=name.strip_prefix("stadt").and_then(|v|v.parse::<usize>().ok()).filter(|&i|i<40);}else if let(Some(i),Some((key,value)))=(current,line.split_once('=')){if key.trim().eq_ignore_ascii_case("AusrichtungDesMeeres"){let value=value.trim().parse::<i32>().map_err(|_|format!("AusrichtungDesMeeres inválida en Stadt{i}"))?;values[i]=value as u8;}}}
 Ok(values)
}

/// Rectángulos de ciudad y puertos de la tabla cargada por `0x0051E440`.
pub(crate) fn parse_town_slots(data:&[u8])->Result<Vec<p3_core::route::planner::TownSlot>,String>{
 let mut slots=vec![p3_core::route::planner::TownSlot::default();40];let mut current=None;
 for raw in data.split(|&b|b==b'\n'||b==b'\r'){let line=String::from_utf8_lossy(raw);let line=line.trim();if line.starts_with('[')&&line.ends_with(']'){let name=line[1..line.len()-1].to_ascii_lowercase();current=name.strip_prefix("stadt").and_then(|v|v.parse::<usize>().ok()).filter(|&i|i<40);}else if let(Some(i),Some((key,value)))=(current,line.split_once('=')){let target=match key.trim().to_ascii_lowercase().as_str(){"stadtsymbolxpos"=>Some(&mut slots[i].x),"stadtsymbolypos"=>Some(&mut slots[i].y),"anfahrtxpos"=>Some(&mut slots[i].anchor_x),"anfahrtypos"=>Some(&mut slots[i].anchor_y),_=>None};if let Some(target)=target{*target=value.trim().parse().map_err(|_|format!("coordenada inválida en Stadt{i}"))?;}}}
 Ok(slots)
}

pub(crate) fn parse_foundation_names(data:&[u8])->Result<(u16,u16),String>{
 let mut section=String::new();let(mut first,mut last)=(None,None);
 for raw in data.split(|&b|b==b'\n'||b==b'\r'){let line=String::from_utf8_lossy(raw);let line=line.trim();if line.starts_with('[')&&line.ends_with(']'){section=line[1..line.len()-1].to_ascii_lowercase()}else if section=="personennamen"{if let Some((key,value))=line.split_once('='){let target=match key.trim().to_ascii_lowercase().as_str(){"maennernamen"=>Some(&mut first),"familiennamen"=>Some(&mut last),_=>None};if let Some(target)=target{*target=Some(value.trim().parse::<u16>().map_err(|_|"contador de nombres inválido")?);}}}}
 let(first,last)=(first.ok_or("faltan nombres masculinos")?,last.ok_or("faltan apellidos")?);if !(8..=256).contains(&first)||!(8..=256).contains(&last){return Err("banco de nombres de concejo incompatible".into())}Ok((first,last))
}

#[cfg(test)]
mod map_flag_tests {
    #[test]
    fn sea_direction_default_and_byte_cast(){
        let values=super::parse_settlement_sea_directions(b"[Stadt0]\nAusrichtungDesMeeres=257\n[Stadt1]\nAusrichtungDesMeeres=-2\n[Stadt2]\nRegion=7\n").unwrap();
        assert_eq!(&values[..3],&[1,254,0]);
        assert!(super::parse_settlement_sea_directions(b"[Stadt0]\nAusrichtungDesMeeres=x\n").is_err());
    }
    #[test]
    fn sea_directions_match_all_original_loader_rows(){
        let (Ok(cpr),Ok(exe),Ok(py))=(std::env::var("P3_CPR"),std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY"))else{return};
        let bytes=std::fs::read(&cpr).unwrap();let archive=p3_assets::cpr::Archive::parse(&bytes).unwrap();let original=archive.read(archive.find(b"scripts/StadtDaten.ini").unwrap()).unwrap();
        let native=super::parse_settlement_sea_directions(original).unwrap();
        let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_town_sea_directions.py");
        let output=std::process::Command::new(py).arg(script).arg(exe).arg(cpr).output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let reference:Vec<u8>=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(native.as_slice(),reference.as_slice());assert_eq!(reference.len(),40);
    }
    #[test]
    fn inland_uses_amfluss_not_region_and_preserves_original_default(){
        let flags=super::parse_settlement_inland(b"[Stadt0]\nRegion=5\nAmFluss=1\n[Stadt1]\nRegion=0\n[Stadt2]\nAmFluss=2\n").unwrap();
        assert_eq!(&flags[..3],&[1,0,0]);
        assert!(super::parse_settlement_inland(b"[Stadt0]\nAmFluss=x\n").is_err());
    }
    #[test]
    fn inland_matches_all_forty_original_resource_loader_rows(){
        use sha2::{Digest,Sha256};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../tests/fixtures/c10_real_town_resources.json")).unwrap();
        let expected=fixture["flags"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>();
        let text=fixture["amfluss"].as_array().unwrap().iter().enumerate().map(|(i,v)|format!("[Stadt{i}]\nAmFluss={}\n",v.as_u64().unwrap())).collect::<String>();
        assert_eq!(super::parse_settlement_inland(text.as_bytes()).unwrap().as_slice(),expected);
        if let Ok(path)=std::env::var("P3_CPR"){
            let bytes=std::fs::read(path).unwrap();let archive=p3_assets::cpr::Archive::parse(&bytes).unwrap();
            let original=archive.read(archive.find(b"scripts/StadtDaten.ini").unwrap()).unwrap();
            assert_eq!(format!("{:x}",Sha256::digest(original)),fixture["resource_sha256"].as_str().unwrap());
            assert_eq!(super::parse_settlement_inland(original).unwrap().as_slice(),expected);
        }
    }
}
impl Sim{
 /// Publica la variante poblada y reanuda el tick hasta el gestor de batalla.
 /// La geometría y el heap proceden del anfitrión; si falla la preparación,
 /// conserva el corte y la tarea para reintentar.
 pub fn resume_populated_pirate_boundary<F>(&mut self,
  resources:crate::pirate_mission_transaction::Resources<'_>,stack_initial:Option<u16>,pixel_to_cell:F)
  ->Result<p3_core::tick::AdvanceEnd,String>
 where F:FnMut(i16,i16)->(u32,u32){
  if self.host.pirate_boundary_model.is_none(){return Err("falta capturar la base anterior al asalto".into())}
  let (target,world_ready)=self.host.pending_pirate_target.ok_or("objetivo del corte ausente")?;
  let task=self.host.pending_pirate_attack.take().ok_or("ataque pendiente ausente")?;
  let boundary=match self.host.pending_pirate_boundary.take(){Some(v)=>v,None=>{self.host.pending_pirate_attack=Some(task);return Err("marcador de corte ausente".into())}};
  if let Err(error)=self.publish_live_populated_pirate_scene(task.0,task.1,resources,stack_initial,pixel_to_cell){
   self.host.pending_pirate_attack=Some(task);self.host.pending_pirate_boundary=Some(boundary);
   return Err(format!("ataque pirata: {error}"));
  }
  let mut bridge=DailyClock{sim:self,world_ready,error:None};
  let outcome=p3_core::tick::resume_after_task_boundary(&mut bridge,boundary,|b|b.sim.host.pending_pirate_attack.is_some(),
   |b|b.error.is_some()||b.sim.host.town_attack_active.is_some()||p3_core::economy::world::TownHost::interrupt_before_sea_battle(&b.sim.host));
  if let Some(error)=bridge.error{bridge.sim.host.clock_partial_error=Some(error.clone());return Err(format!("fase diaria: {error}"))}
  let end=match outcome{
   p3_core::tick::TaskBoundaryEnd::Pending(next)=>{bridge.sim.host.pending_pirate_boundary=Some(next);return Err("otra tarea de ataque requiere resolución mientras hay una escena activa".into())},
   p3_core::tick::TaskBoundaryEnd::Complete(end)=>end,
  };
  bridge.sim.host.pending_pirate_target=None;
  if end==p3_core::tick::AdvanceEnd::InterruptedBeforeBattle{
   bridge.sim.host.clock_before_battle=true;
   if bridge.sim.host.town_attack_active.is_some(){bridge.sim.host.town_attack_tick_pending=true;bridge.sim.host.town_attack_clock_target=Some(target);bridge.sim.host.town_attack_manager_processed=false;}
  }
  if let Some(error)=&bridge.sim.host.navigation_error{return Err(format!("navegación: {error}"))}
  Ok(end)
 }

 /// Publica el asalto únicamente después de capturar la base privada y
 /// reanuda el mismo tick hasta el límite existente anterior a batalla.
 pub fn resume_pirate_boundary(&mut self)->Result<p3_core::tick::AdvanceEnd,String>{
  if self.host.pirate_boundary_model.is_none(){return Err("falta capturar la base anterior al asalto".into())}
  let (target,world_ready)=self.host.pending_pirate_target.ok_or("objetivo del corte ausente")?;
  let task=self.host.pending_pirate_attack.take().ok_or("ataque pendiente ausente")?;
  let boundary=match self.host.pending_pirate_boundary.take(){Some(v)=>v,None=>{self.host.pending_pirate_attack=Some(task);return Err("marcador de corte ausente".into())}};
  if let Err(error)=self.apply_live_pirate_scene(task.0,task.1){
   self.host.pending_pirate_attack=Some(task);
   self.host.pending_pirate_boundary=Some(boundary);
   return Err(format!("ataque pirata: {error}"))
  }
  let mut bridge=DailyClock{sim:self,world_ready,error:None};
  let outcome=p3_core::tick::resume_after_task_boundary(&mut bridge,boundary,|b|b.sim.host.pending_pirate_attack.is_some(),|b|b.error.is_some()||b.sim.host.town_attack_active.is_some()||p3_core::economy::world::TownHost::interrupt_before_sea_battle(&b.sim.host));
  if let Some(error)=bridge.error{bridge.sim.host.clock_partial_error=Some(error.clone());return Err(format!("fase diaria: {error}"))}
  let end=match outcome{
   p3_core::tick::TaskBoundaryEnd::Pending(next)=>{bridge.sim.host.pending_pirate_boundary=Some(next);return Err("otra tarea de ataque requiere resolución mientras hay una escena activa".into())},
   p3_core::tick::TaskBoundaryEnd::Complete(end)=>end,
  };
  bridge.sim.host.pending_pirate_target=None;
  if end==p3_core::tick::AdvanceEnd::InterruptedBeforeBattle{
   bridge.sim.host.clock_before_battle=true;
   if bridge.sim.host.town_attack_active.is_some(){bridge.sim.host.town_attack_tick_pending=true;bridge.sim.host.town_attack_clock_target=Some(target);bridge.sim.host.town_attack_manager_processed=false;}
  }
  if let Some(error)=&bridge.sim.host.navigation_error{return Err(format!("navegación: {error}"))}
  Ok(end)
 }

 /// Avanza con pasadas de cola y operaciones C4 de hasta `max_ticks`.
 /// El llamante elige los pasos que determina el reloj de pared; 8 es el
 /// máximo del modo normal y 256 el del modo rápido. No simula interfaz,
 /// red ni tiempo de pared, y deja pendiente lo emitido por el último C4.
 pub fn advance_pumped_until(&mut self,target:u32,max_ticks:u32,world_ready:bool)->Result<p3_core::tick::AdvanceEnd,String>{
  self.advance_pumped_until_inner(target,max_ticks,world_ready,false)
 }
 pub(crate) fn advance_pumped_until_inner(&mut self,target:u32,max_ticks:u32,world_ready:bool,allow_encounter:bool)->Result<p3_core::tick::AdvanceEnd,String>{
  self.require_resolved_saved_battle()?;
  self.require_no_active_town_attack_save()?;
  if max_ticks==0||max_ticks>256{return Err("tamaño de avance fuera de 1..=256".into())}
  if let Some(error)=&self.host.clock_partial_error{return Err(format!("tick incompleto: {error}; hay que recargar"))}
  if target<self.idle.clock.tick{return Err("objetivo anterior al reloj".into())}
  if !self.host.council_errors.is_empty(){return Err(self.host.council_errors.join("; "))}
  if target>self.idle.clock.tick {self.reject_unsimulated_siege_scenes()?;}
  // Una llamada C4 sin ticks aún aplica el límite de oro del original.
  if target==self.idle.clock.tick{return self.advance_with_merchant_daily_inner(target,world_ready,allow_encounter)}
  while self.idle.clock.tick<target {
   // El bombeo normal no añade C4 con 51 registros en cabeza. La cola
   // se captura antes de ejecutar, de modo que las nuevas esperan otra pasada.
   let allow_advance=self.host.council_operations.len()<51;
   if !self.host.council_operations.is_empty(){let errors=self.dispatch_pending_council_operations();if !errors.is_empty(){return Err(errors.join("; "))}}
   if !allow_advance {continue}
   let previous=self.idle.clock.tick;let next=previous.saturating_add(max_ticks).min(target);
   let end=self.advance_with_merchant_daily_inner(next,world_ready,allow_encounter)?;
   if !self.host.council_errors.is_empty(){return Err(self.host.council_errors.join("; "))}
   if end!=p3_core::tick::AdvanceEnd::Reached{return Ok(end)}
   if self.idle.clock.tick<=previous{return Err("el reloj no avanzó".into())}
  }
  Ok(p3_core::tick::AdvanceEnd::Reached)
 }
 /// Actualiza un comerciante desde las listas y registros vivos. El llamante
 /// aporta el valor de red original `0x006DF31C != 0`. La entrega de
 /// cartas directas se hace sin interfaz; destinatarios especiales y ramas de
 /// programación de misiones aún no conectadas rechazan el cambio atómicamente.
 pub fn merchant_daily_step(&mut self,index:usize,world_ready:bool)->Result<(),String>{
  let towns=&self.economy.world.towns;let town_count=towns.len();if town_count==0||town_count>40||self.host.sites.len()<town_count{return Err("ciudades incompletas".into())}
  let factory=self.host.expedition_factory.as_ref().ok_or("flota no disponible")?;
  let mut images=if let Some(nav)=&self.host.navigation{if nav.snapshot.ships.len()%0x180!=0{return Err("flota truncada".into())}nav.snapshot.ships.chunks_exact(0x180).map(|v|v.to_vec()).collect::<Vec<_>>()}else{factory.ships.clone()};
  if images.len()>u16::MAX as usize||images.len()!=self.economy.cargo_ships.len(){return Err("carga naval incompatible".into())}
  let mut ships=Vec::with_capacity(images.len());for(i,image)in images.iter_mut().enumerate(){if image.len()<0x180{return Err("barco truncado".into())}let cargo=&self.economy.cargo_ships[i];image[0x40..0x42].copy_from_slice(&cargo.crew.to_le_bytes());ships.push(DailyShip{owner:image[0],next:word(image,4),state:word(image,0x134),f10:int(image,0x10),f14:int(image,0x14),f18:int(image,0x18),w40:cargo.crew,stock:cargo.stock});}
  let wages=factory.captains.iter().map(|c|word(c,12)).collect::<Vec<_>>();let raws=self.persisted.iter().map(|o|p3_save::office::decode_office(&o.to_save_v85())).collect::<Vec<_>>();
  let mut offices=Vec::with_capacity(raws.len());let mut flags=Vec::with_capacity(raws.len());let mut assets=Vec::with_capacity(raws.len());
  for(i,img)in raws.iter().enumerate(){let town=img[0x2c6];let site=self.host.sites.get(usize::from(town)).ok_or("ciudad de oficina ausente")?;let t=towns.get(usize::from(town)).ok_or("ciudad de oficina ausente")?;offices.push(DailyOffice{rep_index:town,next:word(img,0x2c8),first_facility:word(img,0x2cc),words:[word(img,0x2de),word(img,0x2e0),word(img,0x2e2)],bytes:[img[0x2e4],img[0x2e5],img[0x2e6]]});flags.push(img[0x2c4]);assets.push(DailyOfficeAssets{town,store_town_flag:t.i32(0x2cc),business_head:word(img,0x2ce),house_head:word(img,0x2d0),stock:&self.persisted[i].stock.stock,site});}
  if offices.len()>u16::MAX as usize||self.economy.world.merchant_facilities.len()>u16::MAX as usize{return Err("tablas excesivas".into())}
  let facilities=self.economy.world.merchant_facilities.iter().map(|f|Facility{price:f.employees,next:f.next}).collect::<Vec<_>>();let refs=towns.iter().map(|t|t.u16(0x994)).collect::<Vec<_>>();
  let rank=RankEnv{town_count:town_count as u32,towns,office_count:offices.len()as u16,offices:&flags,byte_52d:self.economy.world.alderman_merchant,byte_52e:self.economy.world.alderman_town,word_4aa:self.economy.merchants.len().try_into().map_err(|_|"comerciantes excesivos")?,world_ready};
  let env=DailyEnv{index:index.try_into().map_err(|_|"comerciante inválido")?,date:self.idle.clock.tick,word_4a4:u32::from(self.idle.clock.calendar.day_of_year),world_active:self.idle.clock.calendar.day==1,ships:&ships,ship_count:ships.len()as u16,town_ship_refs:&refs,offices:&offices,facilities:&facilities,facility_count:facilities.len()as u16,rank};
  let inputs=MerchantDailyAssets{scheduler:&self.host.council_scheduler,ship_images:&images,captain_wages:&wages,offices:&assets};let mut merchant=self.rep_merchants.get(index).ok_or("comerciante ausente")?.clone();let accounts=self.economy.world.merchants.get(index).ok_or("cuentas ausentes")?;merchant.set_i32(0,accounts.gold);merchant.set_i32(0x4c8,accounts.income);
  let query=|head:u32,value:u32|p3_state::messages::rank_message_pending(&self.host.message_pool,head as u16,value);let effects=update_merchant_daily_with_query(&mut merchant,&env,&inputs,Some(&query))?;
  let prepared=if effects.prepare_alderman_missions{
   let locations=self.host.settlement_locations.as_ref().ok_or("faltan las 40 posiciones de StadtDaten.ini")?;let pirates=self.host.mission_pirates.as_ref().ok_or("bandas piratas ausentes")?;
   let captains=if let Some(nav)=&self.host.navigation{if nav.snapshot.captains.len()%16!=0{return Err("capitanes truncados".into())}nav.snapshot.captains.chunks_exact(16).map(|v|v.try_into().unwrap()).collect::<Vec<[u8;16]>>()}else{factory.captains.clone()};
   let mission_offices=raws.iter().map(|image|p3_core::ai::settlement::SettlementOffice{stock:std::array::from_fn(|i|int(image,4+4*i)),demand:std::array::from_fn(|i|int(image,0x64+4*i)),next:word(image,0x2ca)}).collect::<Vec<_>>();
   Some(p3_core::ai::settlement::prepare_alderman_missions(self.idle.clock.calendar.year,self.idle.clock.calendar.month,towns,&self.host.expedition_town_ids,&mission_offices,pirates,&images,&captains,self.economy.world.merchant_count,self.host.pirate_difficulty,locations).map_err(str::to_string)?)
  }else{None};
  if self.economy.merchants.len()!=self.rep_merchants.len(){return Err("tablas de comerciantes incompatibles".into())}
  let mut staged=self.rep_merchants.clone();staged[index]=merchant;let mut pool=self.host.message_pool.clone();let calendar=self.idle.clock.calendar;
  for (recipient,record) in effects.messages {
   if recipient == -2 {
    p3_state::messages::add_global_inline_letter(&mut pool,calendar.day,calendar.month,calendar.year,&record)?;
   } else {
    let target=usize::try_from(recipient).map_err(|_|"destinatario especial no implementado")?;
    let target=staged.get_mut(target).ok_or("destinatario ausente")?;
    p3_state::messages::add_merchant_daily_record(&mut pool,target,calendar.day,calendar.month,calendar.year,&record)?;
   }
  }
  let merchant=&staged[index];let live=&mut self.economy.world.merchants[index];live.gold=merchant.i32(0);live.income=merchant.i32(0x4c8);self.economy.merchants[index]=merchant.clone();
  for (i,m) in staged.iter().enumerate(){if i!=index{self.economy.merchants[i].bytes[0xa..0xc].copy_from_slice(&m.bytes[0xa..0xc]);}}
  self.rep_merchants=staged;self.host.message_pool=pool;if let Some(prepared)=prepared{self.host.mission_pirates=Some(prepared.pirates);self.host.council_operations.extend(prepared.operations);}Ok(())
 }
}

/// Puente con día de comerciante real. Mantiene los demás sistemas y su
/// orden en `EconomyTickBridge`; los efectos aún no soportados interrumpen
/// antes de barcos y permiten reintentar el mismo tick.
pub(crate) struct DailyClock<'a>{pub(crate) sim:&'a mut Sim,pub(crate) world_ready:bool,pub(crate) error:Option<String>}
impl DailyClock<'_>{
 /// Fases que no tocan comerciantes: sin fusión.
 fn bridge<R>(&mut self,f:impl FnOnce(&mut p3_core::economy::system::EconomyTickBridge<'_,crate::IdleSystems>)->R)->R{self.bridge_with(false,f)}
 /// Fases que sí los tocan: al terminar funde las dos copias del comerciante.
 fn bridge_m<R>(&mut self,f:impl FnOnce(&mut p3_core::economy::system::EconomyTickBridge<'_,crate::IdleSystems>)->R)->R{self.bridge_with(true,f)}
 fn bridge_with<R>(&mut self,merge:bool,f:impl FnOnce(&mut p3_core::economy::system::EconomyTickBridge<'_,crate::IdleSystems>)->R)->R{
  let s=&mut self.sim;
  let base=if merge{s.economy.merchants.clone()}else{Vec::new()};
  let mut offices=p3_state::office::OfficeTownHost{offices:&mut s.persisted,merchants:&mut s.rep_merchants,businesses:&mut s.businesses,captain_wages:&mut s.wages,difficulty:s.commercial_sell_factor,other:&mut s.host,completed:Vec::new(),errors:Vec::new()};
  let mut bridge=p3_core::economy::system::EconomyTickBridge{economy:&mut s.economy,town_host:&mut offices,other:&mut s.idle};
  let result=f(&mut bridge);let errors=std::mem::take(&mut offices.errors);
  if let Some(error)=errors.iter().find(|e|e.contains("astillero")){self.error=Some(error.clone());s.host.navigation_error=Some(error.clone());}
  s.host.council_errors.extend(errors);
  if merge{s.reconcile_merchants(&base);}result
 }
}
impl p3_core::tick::TickSystems for DailyClock<'_>{
 fn clock(&mut self)->&mut p3_core::tick::WorldClock{&mut self.sim.idle.clock}
 fn time_token(&self)->u32{p3_core::tick::TickSystems::time_token(&self.sim.idle)}
 fn preflight_tick(&mut self)->bool{if self.error.is_some(){return false}match self.sim.reject_unsimulated_siege_scenes(){Ok(())=>true,Err(error)=>{self.error=Some(error);false}}}
 fn close_day(&mut self,w:bool){self.bridge_m(|b|b.close_day(w))}
 fn statistics_daily(&mut self){if let Err(error)=self.sim.statistics_step(){self.error=Some(format!("estadísticas: {error}"));}}
 fn town_history_daily(&mut self,t:u32){if let Err(error)=self.sim.town_history_step(t){self.error=Some(format!("historial urbano: {error}"));}}
 fn merchant_daily(&mut self,m:u32){if let Err(e)=self.sim.merchant_daily_step(m as usize,self.world_ready){self.error=Some(e)}}
 fn merchant_phase_failed(&self)->bool{self.error.is_some()}
 fn town_daily(&mut self,t:u32,d:u16){
  let navigation_before=self.sim.host.navigation.as_ref().map(|nav|nav.snapshot.clone());
  let yard_before=self.sim.host.detached_shipyard.as_ref().map(|yard|yard.current.clone());
  self.bridge_m(|b|b.town_daily(t,d));
  if navigation_before.is_none(){self.sim.host.publish_detached_shipyard_delta(yard_before.as_ref(),&mut self.sim.economy.world,&mut self.sim.economy.cargo_ships);}
  if let Some(before)=navigation_before.as_ref(){self.sim.host.publish_navigation_delta(before,&mut self.sim.economy.world,&mut self.sim.economy.cargo_ships);}
  // OfficeTownHost ejecuta el tick de oficina sobre PersistedOffice; publicar
  // esas vistas antes de que el mismo tick alcance la flota y sus comercios.
  for (i,office) in self.sim.persisted.iter().enumerate(){
   if office.stock.owner>=self.sim.economy.world.merchant_count{continue}
   if let Some(stock)=self.sim.economy.offices.get_mut(i){*stock=office.stock.clone()}
   if let Some(account)=self.sim.economy.accounts.offices.get_mut(i){*account=office.accounts}
   if let Some(world)=self.sim.economy.world.offices.get_mut(i){world.weapons.copy_from_slice(&office.stock.stock[20..24])}
  }
 }
 fn trade_center_daily(&mut self,c:u32){
  let populations=self.sim.economy.world.towns.iter().map(|town|town.i32(0x2d4) as u32).collect::<Vec<_>>();
  let tick=self.sim.idle.clock.tick;
  let before=self.sim.host.trade_centers.as_ref().and_then(|centers|centers.get(c as usize)).map(|center|crate::save_version(center));
  let result=self.sim.host.trade_centers.as_mut().and_then(|centers|centers.get_mut(c as usize)).ok_or("centro comercial ausente o no compatible").and_then(|center|p3_core::economy::trade_center::daily_serialized(center,&populations,tick).map(|_|()));
  if let Err(error)=result{self.error=Some(format!("centro comercial {c}: {error}"));}else if let Some(before)=before{let after=self.sim.host.trade_centers.as_ref().and_then(|centers|centers.get(c as usize)).map(|center|crate::save_version(center));if after!=Some(before){let i=c as usize;if self.sim.host.trade_center_versions.len()<=i{self.sim.host.trade_center_versions.resize_with(i+1,Default::default)}self.sim.host.trade_center_versions[i].insert(before);}}
 }
 fn siege_daily(&mut self,t:u32){let before=self.sim.host.siege_version(t as usize);if let Err(error)=self.sim.siege_step(t as usize,self.world_ready){let error=format!("asedio {t}: {error}; tick urbano parcialmente ejecutado");self.sim.host.navigation_error=Some(error.clone());self.error=Some(error)}else{self.sim.host.remember_siege_version(t as usize,before)}}
 fn ships_tick(&mut self,t:u32){self.sim.host.naval_expedition_countdown=self.sim.idle.clock.expedition_countdown;self.bridge_m(|b|b.ships_tick(t))}
 fn scheduled_tasks_tick(&mut self,t:u32){
  loop {let errors_before=self.sim.host.council_errors.len();self.bridge_m(|b|b.scheduled_tasks_tick(t));
   for (i,businesses,houses) in self.sim.host.burn_office_updates.drain(..) {
    if let Some(office)=self.sim.persisted.get_mut(i){office.set_building_heads(businesses,houses)}else{self.error=Some(format!("oficina de incendio {i} ausente"));}
   }
   if let Some(error)=self.sim.host.council_errors.get(errors_before){self.error=Some(format!("{error}; tick parcialmente ejecutado"));break}
   // La escena debe abrirse después de que el anfitrión capture la base
   // viva. El tick queda interrumpido antes de puerto y del gestor naval.
   if self.sim.host.pending_pirate_attack.is_some(){break}
   if let Some((index,expected))=self.sim.host.pending_pirate_approach.take(){
    if let Err(e)=self.sim.apply_pirate_mission_approach(index,expected){self.error=Some(format!("aproximación pirata: {e}; tick parcialmente ejecutado"));break}
    continue
   }
   if let Some((index,expected))=self.sim.host.pending_founding.take(){
    if let Err(e)=self.sim.start_assigned_founding_mission(index,expected){self.error=Some(format!("fundación programada: {e}; tick parcialmente ejecutado"));break}
    continue
   }
   let Some((owner,city,tick))=self.sim.host.pending_office.take() else{break};
   if let Err(e)=self.sim.create_live_office(owner,city,tick){self.error=Some(format!("creación de oficina: {e}; tick parcialmente ejecutado"));break}
  }
  if let Some(error)=&self.error{self.sim.host.clock_partial_error=Some(error.clone())}
 }
 fn harbor_scene_sync(&mut self){self.bridge(|b|b.harbor_scene_sync())}
 fn sea_battles_tick(&mut self){self.bridge(|b|b.sea_battles_tick())}
 fn siege_scene(&mut self,t:u32){if self.sim.siege_scene_idle(t as usize){return}let before=self.sim.host.siege_version(t as usize);if let Err(error)=self.sim.siege_scene_step(t as usize){let error=format!("escena de asedio {t}: {error}");self.sim.host.navigation_error=Some(error.clone());self.error=Some(error)}else{self.sim.host.remember_siege_version(t as usize,before)}}
 fn merchant_gold(&self,m:u32)->i32{self.sim.economy.merchants[m as usize].i32(0)}
 fn set_merchant_gold(&mut self,m:u32,g:i32){self.bridge_m(|b|b.set_merchant_gold(m,g));self.sim.rep_merchants[m as usize].set_i32(0,g)}
}
impl Sim{
 /// Avanza con la fase diaria real de comerciantes. El estado de red del
 /// original es una entrada explícita. No reanuda encuentros pendientes.
 pub fn advance_with_merchant_daily(&mut self,target:u32,world_ready:bool)->Result<p3_core::tick::AdvanceEnd,String>{
  self.advance_with_merchant_daily_inner(target,world_ready,false)
 }
 pub(crate) fn advance_with_merchant_daily_inner(&mut self,target:u32,world_ready:bool,allow_encounter:bool)->Result<p3_core::tick::AdvanceEnd,String>{
  self.require_resolved_saved_battle()?;
  self.require_no_active_town_attack_save()?;
  if let Some(error)=&self.host.clock_partial_error{return Err(format!("tick incompleto: {error}; hay que recargar"))}
  if !self.host.council_errors.is_empty(){return Err(self.host.council_errors.join("; "))}
  if self.host.clock_before_battle||(!allow_encounter&&self.host.encounter_config.is_some())||self.host.encounter_battle.is_some()||self.host.navigation_error.is_some(){return Err("encuentro o error de navegación pendiente".into())}
  if target>self.idle.clock.tick {self.reject_unsimulated_siege_scenes()?;}
  let mut bridge=DailyClock{sim:self,world_ready,error:None};
  let end=match p3_core::tick::advance_time_with_task_boundary(&mut bridge,target,0,|b|b.sim.host.pending_pirate_attack.is_some(),|b|b.error.is_some()||b.sim.host.town_attack_active.is_some()||p3_core::economy::world::TownHost::interrupt_before_sea_battle(&b.sim.host)){
   p3_core::tick::TaskBoundaryEnd::Complete(end)=>end,
   p3_core::tick::TaskBoundaryEnd::Pending(boundary)=>{
    bridge.sim.host.pending_pirate_boundary=Some(boundary);
    bridge.sim.host.pending_pirate_target=Some((target,world_ready));
    p3_core::tick::AdvanceEnd::InterruptedBeforeTask
   }
  };
  if let Some(error)=bridge.error{return Err(format!("fase diaria: {error}"))}
  if end==p3_core::tick::AdvanceEnd::InterruptedBeforeBattle{bridge.sim.host.clock_before_battle=true;if bridge.sim.host.town_attack_active.is_some(){bridge.sim.host.town_attack_tick_pending=true;bridge.sim.host.town_attack_clock_target=Some(target);bridge.sim.host.town_attack_manager_processed=false;}}
  if let Some(error)=&bridge.sim.host.navigation_error{return Err(format!("navegación: {error}"))}
  Ok(end)
 }
}
#[cfg(test)]
#[path = "founding_scheduler_tests.rs"]
mod founding_scheduler_tests;
#[cfg(test)]
#[path = "pirate_mission_approach_tests.rs"]
mod pirate_mission_approach_tests;
