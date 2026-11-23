//! Diagnostic save: original town event 0x1E, phase 3, immediately before reward.
//! No game text or pointer is injected. Refuses to overwrite any destination.
use p3_save::{model::SaveModel, tail::ScheduledTasks, world::World, Save};
use sha2::{Digest,Sha256};

fn word(b:&[u8],a:usize)->u16 {u16::from_le_bytes(b[a..a+2].try_into().unwrap())}
fn dword(b:&[u8],a:usize)->u32 {u32::from_le_bytes(b[a..a+4].try_into().unwrap())}
fn hex(b:&[u8])->String {b.iter().map(|v|format!("{v:02x}")).collect()}
fn main() {
    let a:Vec<_>=std::env::args().collect();
    assert_eq!(a.len(),4,"letter42_lab ORIGINAL.pat OUTPUT.pat EVIDENCE.json");
    for p in [&a[2],&a[3]] {assert!(!std::path::Path::new(p).exists(),"destination exists: {p}");}
    assert_ne!(std::path::Path::new(&a[1]).canonicalize().unwrap(),
               std::path::Path::new(&a[2]).parent().unwrap().canonicalize().unwrap().join(std::path::Path::new(&a[2]).file_name().unwrap()));
    let raw=std::fs::read(&a[1]).unwrap();let save=Save::parse(&raw).unwrap();
    let world=World::parse(save.block_named("game_world").unwrap()).unwrap();
    let owner=(0..world.merchant_slots()).find(|&i|word(&world.merchant_image(i),8)==0).expect("no human merchant");
    assert!(owner<256);let mut merchant=world.merchant_image(owner);
    let city=usize::from(merchant[0x19]);assert!(city<world.town_count());
    // Serialized GameWorld+0x18 -> live GameWorld+0x14 (0x6DE4B4).
    // Serialized +0x14 is start_time_raw, not the running clock.
    let tick=dword(world.header(),0x18);let mut model=SaveModel::from_save(&save).unwrap();
    let parsed=ScheduledTasks::parse(&model.scheduled_tasks).unwrap();
    let free=usize::from(parsed.field_0a);assert!(free<usize::from(parsed.count),"no free task slot");
    let range=parsed.tasks[free].range.clone();assert_eq!(parsed.tasks[free].kind,0xffff);
    let next_free=word(&model.scheduled_tasks,range.start+4);
    // Put the diagnostic event first. Its due tick is in the past, so the
    // original scheduler dispatches it as soon as the loaded game advances.
    let head=parsed.field_08;
    let mut task=[0u8;24];task[0..4].copy_from_slice(&tick.saturating_sub(1).to_le_bytes());
    task[4..6].copy_from_slice(&head.to_le_bytes());task[6..8].copy_from_slice(&0x1eu16.to_le_bytes());
    for at in [8,12] {task[at..at+4].copy_from_slice(&tick.saturating_add(0x1000).to_le_bytes());}
    task[16]=3;task[17]=city as u8;task[18..20].fill(255);task[20]=20;task[21]=1;task[22]=owner as u8;
    // The offer stores its baseline at +0x17. Four subsequent victories are
    // the original payment condition. This counter is synthetic lab input.
    task[23]=0;merchant[0x1b]=4;model.world.merchants[owner][0x1b]=4;
    model.scheduled_tasks[range].copy_from_slice(&task);
    model.scheduled_tasks[0..2].copy_from_slice(&(free as u16).to_le_bytes());
    model.scheduled_tasks[2..4].copy_from_slice(&next_free.to_le_bytes());
    ScheduledTasks::parse(&model.scheduled_tasks).unwrap();
    let out=model.write();let reparsed=SaveModel::parse(&out).unwrap();assert_eq!(reparsed,model);
    for block in ["ships","message_pool","route_stop_pool","sea_battles","name_bank"] {
        assert_eq!(Save::parse(&out).unwrap().block_named(block),save.block_named(block));
    }
    let evidence=serde_json::json!({"schema":1,"method":"synthetic event in a copy; original game must dispatch it",
        "original":std::fs::canonicalize(&a[1]).unwrap(),"original_sha256":format!("{:x}",Sha256::digest(&raw)),
        "output":a[2],"output_sha256":format!("{:x}",Sha256::digest(&out)),"owner":owner,"city":city,"tick":tick,
        "task_slot":free,"task":hex(&task),"town":hex(&world.town(city).0[..]),"merchant":hex(&merchant),
        "human_edit":"merchant+0x1B=4; task baseline=0, phase=3; no pointer/text injection"});
    std::fs::write(&a[2],out).unwrap();std::fs::write(&a[3],serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
    assert_eq!(format!("{:x}",Sha256::digest(std::fs::read(&a[1]).unwrap())),evidence["original_sha256"].as_str().unwrap());
    println!("owner={owner} city={city} task={free}; output {}",a[2]);
}
