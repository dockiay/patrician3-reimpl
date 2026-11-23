use super::*;
use p3_core::{economy::captain_operations::retirement_tick, scheduler::TaskRecord, ships::captains::CaptainPool};
use std::io::Write;

fn hex(b:&[u8])->String{b.iter().map(|x|format!("{x:02x}")).collect()}
fn bytes(s:&str)->Vec<u8>{s.as_bytes().chunks_exact(2).map(|v|u8::from_str_radix(std::str::from_utf8(v).unwrap(),16).unwrap()).collect()}
fn put(b:&mut[u8],i:usize,n:u16){b[i..i+2].copy_from_slice(&n.to_le_bytes())}
fn raw(t:&TaskRecord)->Vec<u8>{t.due.to_le_bytes().into_iter().chain(t.next.to_le_bytes()).chain(t.kind.to_le_bytes()).chain(t.payload).collect()}

#[test]
fn retirement_disbands_docked_convoys_and_updates_reputation_like_x86(){
    let (Ok(exe),Ok(python))=(std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY")) else{return};
    let mut cases=Vec::new();let mut inputs=Vec::new();
    for state in [0u16,1] {for convoy_state in [0u16,1,4,5] {for special in [false,true] {for foreign in [false,true] {for count in 1..=3usize {for leader in 0..count {
        let mut manager=vec![0u8;0x110];for at in [0xe0,0xe4,0xe8,0xea,0xec]{put(&mut manager,at,0xffff)}put(&mut manager,0xe0,0);put(&mut manager,0xe4,1);put(&mut manager,0xec,0);put(&mut manager,0xf2,2);put(&mut manager,0xf4,count as u16);put(&mut manager,0xf6,2);
        let mut ships=vec![0;count*0x180];for i in 0..count{let a=i*0x180;ships[a]=u8::from(foreign&&i!=leader);put(&mut ships,a+6,if i+1<count{(i+1) as u16}else{0xffff});put(&mut ships,a+8,0);put(&mut ships,a+0x42,if i==leader{1}else{0xffff});put(&mut ships,a+0x134,state)}
        let mut convoys=vec![0;2*0x3c];put(&mut convoys,8,0xffff);put(&mut convoys,0xa,0);put(&mut convoys,0x10,leader as u16);put(&mut convoys,0x12,convoy_state);convoys[0x14]=if special{2}else{0};put(&mut convoys,0x3c+8,0xffff);put(&mut convoys,0x3c+0x12,0xff);
        let mut pool=CaptainPool{entries:vec![0;32],capacity:2,free_head:0};put(&mut pool.entries,0,0xffff);put(&mut pool.entries,16,0xffff);
        let towns=vec![Town::default()];let mut merchants=vec![RepMerchant{bytes:vec![0;0x650]};2];merchants[0].bytes[0x464..0x468].copy_from_slice(&1.25f32.to_le_bytes());merchants[0].bytes[0x120..0x124].copy_from_slice(&0.5f32.to_le_bytes());
        let mut task=TaskRecord::default();task.kind=0x27;task.due=42;task.set_u32_at(8,leader as u32);task.set_u32_at(0xc,1);
        cases.push(serde_json::json!({"manager":hex(&manager),"task":hex(&raw(&task)),"ships":hex(&ships),"convoys":hex(&convoys),"towns":hex(&towns[0].0[..]),"captains":hex(&pool.entries),"free":pool.free_head,"merchants":merchants.iter().map(|m|hex(&m.bytes)).collect::<Vec<_>>()}));
        inputs.push((manager,ships,convoys,towns,pool,merchants,task));
    }}}}}}
    let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_retirement_tick.py");
    let mut child=std::process::Command::new(python).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"binary":exe,"cases":cases,"disband_real":true}).to_string().as_bytes()).unwrap();
    let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let result:serde_json::Value=serde_json::from_slice(&out.stdout).unwrap();
    for (i,(manager,mut ships,mut convoys,mut towns,mut pool,merchants,mut task)) in inputs.into_iter().enumerate(){
        let mut host=RetirementDisband{manager,merchants:merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect(),merchant_count:merchants.len() as u16,tick:42,player:0,distances:None,notices:Vec::new()};
        let keep=retirement_tick(&mut task,&mut ships,&mut convoys,&mut towns,&mut pool,&merchants,&mut host).unwrap();let x=&result[i];
        assert_eq!(u64::from(keep),x["keep"].as_u64().unwrap(),"case {i}");assert_eq!(raw(&task),bytes(x["task"].as_str().unwrap()),"task {i}");assert_eq!(ships,bytes(x["ships"].as_str().unwrap()),"ships {i}");assert_eq!(convoys,bytes(x["convoys"].as_str().unwrap()),"convoys {i}");assert_eq!(pool.entries,bytes(x["captains"].as_str().unwrap()),"captains {i}");
        let expected_merchants=x["merchants"].as_array().unwrap().iter().flat_map(|m|bytes(m.as_str().unwrap())).collect::<Vec<_>>();assert_eq!(host.merchants,expected_merchants,"merchants {i}");
        let expected=bytes(x["manager"].as_str().unwrap());assert_eq!(&host.manager[0xe4..0xee],&expected[0xe4..0xee],"heads {i}");assert_eq!(u64::from(pool.free_head),x["free"].as_u64().unwrap(),"free {i}");
    }
}

#[test]
fn unsupported_and_cyclic_convoys_are_rejected_before_mutation(){
 use p3_core::economy::captain_operations::RetirementHost;
 let mut manager=vec![0;0x110];put(&mut manager,0xf4,1);put(&mut manager,0xf6,1);put(&mut manager,0xec,0);put(&mut manager,0xe4,0xffff);
 let mut host=RetirementDisband{manager:manager.clone(),merchants:vec![0;0x650],merchant_count:1,tick:0,player:0,distances:None,notices:Vec::new()};
 let mut ships=vec![0;0x180];put(&mut ships,6,0xffff);put(&mut ships,8,0);let mut convoys=vec![0;0x3c];put(&mut convoys,8,0xffff);put(&mut convoys,0xa,0);put(&mut convoys,0x10,0);convoys[0x1e]=1;let mut towns=vec![Town::default()];let mut pool=CaptainPool{entries:vec![],capacity:0,free_head:0};
 let before=(ships.clone(),convoys.clone());assert!(host.disband_convoy(0,&mut ships,&mut convoys,&mut towns,&mut pool).unwrap_err().contains("distancias"));assert_eq!((ships.clone(),convoys.clone()),before);assert_eq!(host.manager,manager);
 convoys[0x14]=2;put(&mut ships,6,0);let before=(ships.clone(),convoys.clone());assert!(host.disband_convoy(0,&mut ships,&mut convoys,&mut towns,&mut pool).unwrap_err().contains("cíclico"));assert_eq!((ships,convoys),before);assert_eq!(host.manager,manager);
}

#[test]
fn retirement_refunds_and_emits_convoy_notices_like_x86(){
    let (Ok(exe),Ok(python))=(std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY")) else{return};
    let mut cases=Vec::new();let mut inputs=Vec::new();
    for distance in [1u8,19,93,255] {for count in [2usize,3] {for leader in 0..count {for value in [50u32,100,12345,1_000_000] {for human in 0..4 {
        let mut manager=vec![0;0x110];for at in [0xe0,0xe4,0xe8,0xea,0xec]{put(&mut manager,at,0xffff)}put(&mut manager,0xe0,0);put(&mut manager,0xe4,1);put(&mut manager,0xec,0);put(&mut manager,0xf2,2);put(&mut manager,0xf4,count as u16);put(&mut manager,0xf6,2);
        let mut ships=vec![0;count*0x180];for i in 0..count{let a=i*0x180;ships[a]=u8::from(i!=leader);put(&mut ships,a+6,if i+1<count{(i+1) as u16}else{0xffff});put(&mut ships,a+8,0);put(&mut ships,a+0x42,if i==leader{1}else{0xffff});put(&mut ships,a+0x15e,0x2345+i as u16);ships[a+0x114..a+0x118].copy_from_slice(&value.to_le_bytes())}
        let mut convoys=vec![0;2*0x3c];put(&mut convoys,8,0xffff);put(&mut convoys,0xa,0);put(&mut convoys,0x10,leader as u16);put(&mut convoys,0x1c,1);convoys[0x1e]=0;convoys[0x1f]=1;put(&mut convoys,0x3c+8,0xffff);put(&mut convoys,0x3c+0x12,0xff);
        let mut pool=CaptainPool{entries:vec![0;32],capacity:2,free_head:0};put(&mut pool.entries,0,0xffff);put(&mut pool.entries,16,0xffff);let towns=vec![Town::default();2];let mut merchants=vec![RepMerchant{bytes:vec![0;0x650]};2];for (i,m) in merchants.iter_mut().enumerate(){put(&mut m.bytes,8,if human&(1<<i)!=0{0}else{1});m.bytes[..4].copy_from_slice(&(100_000i32+(i as i32)*50_000).to_le_bytes());m.bytes[0x19]=0;m.bytes[0x1a]=0x12;m.bytes[0x1c]=0x34;m.bytes[0x1d]=0x56;m.bytes[0x39c]=0x78;m.bytes[0x464..0x468].copy_from_slice(&1.25f32.to_le_bytes())}
        let mut task=TaskRecord::default();task.kind=0x27;task.due=42;task.set_u32_at(8,leader as u32);task.set_u32_at(0xc,1);let matrix=vec![0,distance,distance,0];
        cases.push(serde_json::json!({"manager":hex(&manager),"task":hex(&raw(&task)),"ships":hex(&ships),"convoys":hex(&convoys),"towns":hex(&towns.iter().flat_map(|t|t.0.iter().copied()).collect::<Vec<_>>()),"captains":hex(&pool.entries),"free":pool.free_head,"merchants":merchants.iter().map(|m|hex(&m.bytes)).collect::<Vec<_>>(),"matrix":hex(&matrix)}));inputs.push((manager,ships,convoys,towns,pool,merchants,task,matrix));
    }}}}}
    let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_retirement_tick.py");let mut child=std::process::Command::new(python).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"binary":exe,"cases":cases,"disband_real":true,"capture_messages":true}).to_string().as_bytes()).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let result:serde_json::Value=serde_json::from_slice(&out.stdout).unwrap();
    for (i,(manager,mut ships,mut convoys,mut towns,mut pool,merchants,mut task,matrix)) in inputs.into_iter().enumerate(){let mut host=RetirementDisband{manager,merchants:merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect(),merchant_count:2,tick:42,player:0,distances:Some(p3_core::ships::ai_dock::AiDockData{distances:matrix,size:2,..Default::default()}),notices:Vec::new()};let keep=retirement_tick(&mut task,&mut ships,&mut convoys,&mut towns,&mut pool,&merchants,&mut host).unwrap();let x=&result[i];assert_eq!(u64::from(keep),x["keep"].as_u64().unwrap(),"case {i}");assert_eq!(ships,bytes(x["ships"].as_str().unwrap()),"ships {i}");assert_eq!(convoys,bytes(x["convoys"].as_str().unwrap()),"convoys {i}");let expected=x["merchants"].as_array().unwrap().iter().flat_map(|m|bytes(m.as_str().unwrap())).collect::<Vec<_>>();assert_eq!(host.merchants,expected,"merchants {i}");assert_eq!(pool.entries,bytes(x["captains"].as_str().unwrap()),"captains {i}");let events=host.notices.iter().map(|(owner,town,kind,words)|std::iter::once(u64::from(*owner)).chain(std::iter::once(u64::from(*kind))).chain(std::iter::once(u64::from(*town))).chain(words.iter().flat_map(|w|w.to_le_bytes()).map(u64::from)).collect::<Vec<_>>()).collect::<Vec<_>>();let expected:Vec<Vec<u64>>=serde_json::from_value(x["events"].clone()).unwrap();assert_eq!(events,expected,"notices {i}");}
}
