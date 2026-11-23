//! Gestor diario de estadísticas C10 y representación del `.pst`.
use p3_core::{ai::reputation::Merchant,economy::{merchant_statistics,town_statistics},town::Town};
use p3_save::model::StatisticsModel;
const MONTH_DAYS:[usize;12]=[31,31,28,31,30,31,30,31,31,30,31,30];

/// Estado vacío de `game_statistics_clear` (0x0054DDF0). La ausencia de un
/// `.pst` no desactiva la captura: también limpia acumuladores del mundo.
fn empty_statistics()->StatisticsModel{
 StatisticsModel{header:[0;6],chains:std::array::from_fn(|_|None),items:Vec::new(),slack:0}
}

fn push_chain(stats:&mut StatisticsModel,index:usize,record:Vec<u8>,limit:Option<usize>){
 let chain=stats.chains[index].get_or_insert_with(Vec::new);chain.insert(0,((record.len()/0x204) as u32,record));if let Some(max)=limit{chain.truncate(max)}
}

#[cfg(test)]
mod live_tests {
 use std::io::Write;
 use std::process::{Command,Stdio};

 fn hex(bytes:&[u8])->String{bytes.iter().map(|b|format!("{b:02x}")).collect()}
 fn unhex(s:&str)->Vec<u8>{s.as_bytes().chunks_exact(2).map(|v|u8::from_str_radix(std::str::from_utf8(v).unwrap(),16).unwrap()).collect()}
 fn same(actual:&[u8],expected:&[u8],what:&str){
  assert_eq!(actual.len(),expected.len(),"longitud {what}");
  if let Some((offset,(a,b)))=actual.iter().zip(expected).enumerate().find(|(_, (a,b))|a!=b){panic!("{what} +{offset:x}: {a:02x} != {b:02x}")}
 }
 /// `game_statistics_clear` (`0x0054DDF0`) contra el x86 con 400 objetos aleatorios: el original deja a cero las cinco palabras de
 /// cabecera y los cinco punteros (destruyendo y liberando cada cadena no nula, la quinta con otro destructor); `empty_statistics`
 /// es ese estado vacío.
 #[test]
 fn empty_statistics_matches_x86_clear(){
  let (Ok(binary),Ok(python))=(std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY")) else{return};
  let mut seed=0x54ddf0u64;let mut next=move|n:u32|{seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);((seed>>33) as u32)%n};
  let mut cases=Vec::new();let mut objects=Vec::new();
  for _ in 0..400{
   let mut o:Vec<u8>=(0..0x20).map(|_|next(256) as u8).collect();
   for slot in [0xc,0x10,0x14,0x18,0x1c]{let p=if next(2)==0{0}else{0x1000_0000+next(0x1000)*0x10};o[slot..slot+4].copy_from_slice(&u32::to_le_bytes(p))}
   cases.push(serde_json::json!({"object":hex(&o)}));objects.push(o);
  }
  let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_small.py");
  let mut child=Command::new(python).arg(script).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
  child.stdin.take().unwrap().write_all(serde_json::json!({"binary":binary,"fn":"stats_clear","cases":cases}).to_string().as_bytes()).unwrap();
  let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));
  let orig:Vec<serde_json::Value>=serde_json::from_slice(&out.stdout).unwrap();
  let empty=super::empty_statistics();
  assert!(empty.header.iter().all(|&h|h==0)&&empty.chains.iter().all(Option::is_none)&&empty.items.is_empty()&&empty.slack==0);
  for (i,(o,input)) in orig.iter().zip(&objects).enumerate(){
   let after=unhex(o["object"].as_str().unwrap());
   assert!(after[..10].iter().all(|&b|b==0),"cabecera {i}");
   assert_eq!(&after[10..0xc],&input[10..0xc],"relleno {i}");
   let mut expected=Vec::new();
   for (slot,kind) in [(0xc,"chain"),(0x10,"chain"),(0x14,"chain"),(0x18,"chain"),(0x1c,"items")]{
    let p=u32::from_le_bytes(input[slot..slot+4].try_into().unwrap());
    assert_eq!(&after[slot..slot+4],&[0u8;4],"puntero {slot:#x} caso {i}");
    if p!=0{expected.push(serde_json::json!([kind,p]));expected.push(serde_json::json!(["delete",p]))}
   }
   assert_eq!(o["calls"].as_array().unwrap(),&expected,"llamadas {i}");
  }
 }
 #[test]
 fn real_save_statistics_step_matches_x86(){
  compare_real_save(false);
 }
 #[test]
 fn absent_pst_still_captures_and_clears_world_counters_like_x86(){
  compare_real_save(true);
 }
 fn compare_real_save(without_pst:bool){
  let (Ok(root),Ok(binary),Ok(python))=(std::env::var("P3_SAVES"),std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY")) else{return};
  let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
  let compressed=if without_pst{p3_core::route::compress::compress(&super::empty_statistics().write())}else{std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pst")).unwrap()};
  let (pst,_)=p3_core::route::rou::decompress(&compressed).unwrap();
  for (day,month,day_of_year,tick) in [(12,3,101,256*101),(1,5,152,256*154),(1,0,0,256*7)]{
   let mut sim=crate::Sim::from_save_bytes(&raw).unwrap();if !without_pst{sim.configure_statistics(&compressed).unwrap();}
   sim.idle.clock.tick=tick;sim.idle.clock.calendar.day=day;sim.idle.clock.calendar.month=month;sim.idle.clock.calendar.day_of_year=day_of_year;
   let pool=sim.host.office_pool.as_ref().unwrap();
   let mut offices=Vec::new();for(i,persisted)in sim.persisted.iter().enumerate(){let mut image=p3_save::office::decode_office(&persisted.to_save_v85());image[0x2c4..0x2ca].copy_from_slice(&pool.records[i][0x2c4..0x2ca]);image[0x2ca..0x2cc].copy_from_slice(&sim.economy.world.offices[i].next.to_le_bytes());offices.extend_from_slice(&image);}
   let towns=sim.economy.world.towns.iter().flat_map(|v|v.0.iter().copied()).collect::<Vec<_>>();
   let merchants=sim.rep_merchants[..usize::from(sim.idle.clock.merchant_count)].iter().flat_map(|v|v.bytes.iter().copied()).collect::<Vec<_>>();
   let facilities=sim.host.employment_businesses.iter().flat_map(|v|v.iter().copied()).collect::<Vec<_>>();
   let input=serde_json::json!({"binary":binary,"pst":hex(&pst),"towns":hex(&towns),"merchants":hex(&merchants),"offices":hex(&offices),"facilities":hex(&facilities),"captains":sim.host.expedition_factory.as_ref().unwrap().captains.len(),"day":day,"month":month,"year":sim.idle.clock.calendar.year,"day_of_year":day_of_year,"tick":tick});
   let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_daily_statistics_live.py");
   let mut child=Command::new(&python).arg(script).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
   child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"oráculo: {}",String::from_utf8_lossy(&output.stderr));
   let expected:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();sim.statistics_step().unwrap();
   same(&sim.host.statistics.as_ref().unwrap().write(),&unhex(expected["pst"].as_str().unwrap()),"pst");
   same(&sim.economy.world.towns.iter().flat_map(|v|v.0.iter().copied()).collect::<Vec<_>>(),&unhex(expected["towns"].as_str().unwrap()),"ciudades");
   same(&sim.rep_merchants[..usize::from(sim.idle.clock.merchant_count)].iter().flat_map(|v|v.bytes.iter().copied()).collect::<Vec<_>>(),&unhex(expected["merchants"].as_str().unwrap()),"comerciantes");
  }
 }
}
fn mean_chain(stats:&StatisticsModel,index:usize,limit:usize)->Result<Vec<u8>,String>{
 let nodes=stats.chains[index].as_ref().ok_or("cadena de estadísticas ausente")?.iter().map(|(_,v)|v.clone()).collect::<Vec<_>>();
 town_statistics::average(&nodes,limit).map_err(str::to_string)?.ok_or("cadena de estadísticas vacía".into())
}

/// Ejecuta `0x0054DE90` sin presentación. Reputaciones y datos de oficinas y
/// negocios son los mismos que verá la captura del original. Es transaccional.
pub fn daily(stats:&mut StatisticsModel,towns:&mut Vec<Town>,merchants:&mut Vec<Merchant>,offices:&[Vec<u8>],facilities:&[[u8;20]],captain_count:u16,reputations:&[(i16,i16)],tick:u32,day:u8,month:u8,day_of_year:u16)->Result<(),String>{
 if month>=12||merchants.len()!=reputations.len()||merchants.len()>u16::MAX as usize{return Err("entorno de estadísticas incompatible".into())}
 let mut next=stats.clone();let mut town_next=towns.clone();let mut merchant_next=merchants.clone();
 let sample=town_statistics::capture(&mut town_next,offices).map_err(str::to_string)?;push_chain(&mut next,0,sample,Some(32));
 let month_end=day==1;let year_end=month_end&&day_of_year==0;let weekday=(tick>>8)%7;
 if month_end{
  let monthly=mean_chain(&next,0,MONTH_DAYS[usize::from(month)])?;push_chain(&mut next,2,monthly,Some(25));
  if year_end{let yearly=mean_chain(&next,2,12)?;push_chain(&mut next,3,yearly,None);next.header[4]=next.header[4].wrapping_add(1);}
  if next.header[2]<24{next.header[2]+=1;}
 }
 if weekday==6{let weekly=mean_chain(&next,0,7)?;push_chain(&mut next,1,weekly,Some(25));if next.header[1]<24{next.header[1]+=1;}}
 if next.items.is_empty(){
  for owner in (0..merchant_next.len()).rev(){if merchant_next[owner].u16(8)&4!=0{continue}
   let (home,total)=reputations[owner];let initial=merchant_statistics::capture(&mut merchant_next[owner].bytes,offices,facilities,captain_count,home,total).map_err(str::to_string)?;
   next.items.insert(0,merchant_statistics::new_history(owner as u16,&initial));
  }
 }
 for item in &mut next.items{
  if item.len()<2{return Err("histórico de comerciante truncado".into())}let owner=usize::from(u16::from_le_bytes([item[0],item[1]]));
  let merchant=merchant_next.get_mut(owner).ok_or("comerciante de estadísticas ausente")?;let(home,total)=reputations[owner];
  let current=merchant_statistics::capture(&mut merchant.bytes,offices,facilities,captain_count,home,total).map_err(str::to_string)?;
  merchant_statistics::advance_history(item,&current,weekday==0,month_end,year_end,month).map_err(str::to_string)?;
 }
 next.header[0]=next.header[0].wrapping_add(1)&0xfff;
 *stats=next;*towns=town_next;*merchants=merchant_next;Ok(())
}

impl crate::Sim{
 /// Carga el fichero `.pst` asociado; el `.pat` no incluye estas estadísticas.
 pub fn configure_statistics(&mut self,bytes:&[u8])->Result<(),String>{
  let (data,_)=p3_core::route::rou::decompress(bytes).ok_or("PST comprimido inválido")?;
  let stats=StatisticsModel::parse(&data).map_err(|e|format!("estadísticas PST: {e:?}"))?;
  self.host.statistics=Some(stats);Ok(())
 }
 /// Serializa el `.pst` actual; no modifica el archivo de entrada.
 pub fn statistics_bytes(&self)->Option<Vec<u8>>{self.host.statistics.as_ref().map(|stats|p3_core::route::compress::compress(&stats.write()))}
 pub fn statistics_step(&mut self)->Result<(),String>{
  let mut stats=self.host.statistics.clone().unwrap_or_else(empty_statistics);
  // 0x0054DE90 recorre `0x006DE4AA` (comerciantes del mundo), no el recuento de despacho del reloj.
  let n=self.economy.world.towns.len();let m=usize::from(self.economy.world.merchant_count);
  if n>40||m>self.rep_merchants.len()||m>self.economy.merchants.len(){return Err("dimensiones de estadísticas incompatibles".into())}
  let pool=self.host.office_pool.as_ref().ok_or("pool de oficinas ausente")?;
  if pool.records.len()!=self.persisted.len()||pool.records.len()!=self.economy.world.offices.len(){return Err("oficinas de estadísticas incompatibles".into())}
  let mut offices=Vec::with_capacity(pool.records.len());
  for(i,persisted)in self.persisted.iter().enumerate(){let record=&pool.records[i];if record.len()!=0x44c{return Err("oficina truncada".into())}let mut image=p3_save::office::decode_office(&persisted.to_save_v85());image[0x2c4..0x2ca].copy_from_slice(&record[0x2c4..0x2ca]);image[0x2ca..0x2cc].copy_from_slice(&self.economy.world.offices[i].next.to_le_bytes());offices.push(image.to_vec());}
  let captains=if let Some(nav)=&self.host.navigation{nav.snapshot.captains.len()/16}else{self.host.expedition_factory.as_ref().ok_or("pool de capitanes ausente")?.captains.len()};
  let captain_count=u16::try_from(captains).map_err(|_|"demasiados capitanes")?;
  let mut reps=Vec::with_capacity(m);
  for merchant in &self.rep_merchants[..m]{
   let home=usize::from(merchant.bytes[0x19]);let local=if home>=n{0}else{let value=f32::from_bits(u32::from_le_bytes(merchant.bytes[0x2fc+4*home..0x300+4*home].try_into().unwrap()));if value>32758.0{0x7fff}else{p3_core::numeric::ftol_i32(value as f64) as u16}};
   let total=merchant.calc_total_reputation_score(n as i32,&self.economy.world.towns) as i16;reps.push((local as i16,total));
  }
  let mut towns=self.economy.world.towns.clone();let mut merchants=self.rep_merchants[..m].to_vec();let c=self.idle.clock.calendar;
  daily(&mut stats,&mut towns,&mut merchants,&offices,&self.host.employment_businesses,captain_count,&reps,self.idle.clock.tick,c.day,c.month,c.day_of_year)?;
  self.economy.world.towns=towns;
  for(i,merchant)in merchants.into_iter().enumerate(){self.economy.world.merchants[i].income=merchant.i32(0x4c8);self.economy.merchants[i]=merchant.clone();self.rep_merchants[i]=merchant;}
  self.host.statistics=Some(stats);Ok(())
 }
}
