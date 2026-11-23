//! Diagnostic snapshots around the first encounter. No game window.
use p3_mvp::Session;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn dump(session: &Session, out: &Path, label: &str) -> Result<(), String> {
    let nav = &session.sim.host.navigation.as_ref().ok_or("missing fleet")?.snapshot;
    for (name, data) in [
        ("ships", nav.ships.as_slice()),
        ("convoys", nav.convoys.as_slice()),
        ("captains", nav.captains.as_slice()),
        ("manager", nav.manager.as_slice()),
    ] {
        fs::write(out.join(format!("{label}.{name}.bin")), data).map_err(|e| e.to_string())?;
    }
    if let Some(active) = session.battle_diagnostic_snapshot() {
        let b = &active.state;
        fs::write(out.join(format!("{label}.battle.bin")), &b.object).map_err(|e| e.to_string())?;
        for (i, s) in b.local_ships.iter().enumerate() {
            fs::write(out.join(format!("{label}.local-{i}.bin")), s).map_err(|e| e.to_string())?;
        }
    } else if let Some(b) = session.sim.host.encounter_battle.as_ref() {
        fs::write(out.join(format!("{label}.battle.bin")), &b.object).map_err(|e| e.to_string())?;
        for (i, s) in b.local_ships.iter().enumerate() {
            fs::write(out.join(format!("{label}.local-{i}.bin")), s).map_err(|e| e.to_string())?;
        }
    }
    println!(
        "{}",
        serde_json::json!({"label":label,"tick":session.sim.idle.clock.tick,"encounter":session.sim.host.encounter_battle.is_some(),"rng":session.sim.host.investigation_rng.map(|v|v.state)})
    );
    Ok(())
}
fn run() -> Result<(), String> {
    let a: Vec<_> = env::args().skip(1).collect();
    if !(6..=7).contains(&a.len()) {
        return Err("usage: c10_encounter_probe SAVE CPR EXE OUT LOW_OFFSET HIGH_OFFSET [BATTLE_STEPS]".into());
    }
    let battle_steps: usize = a
        .get(6)
        .map(|s| s.parse().map_err(|e: std::num::ParseIntError| e.to_string()))
        .transpose()?
        .unwrap_or(0);
    let low: u32 = a[4].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
    let high: u32 = a[5].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
    if low > high {
        return Err("invalid interval".into());
    }
    let mut session = Session::open(
        &fs::read(&a[0]).map_err(|e| e.to_string())?,
        fs::read(&a[1]).map_err(|e| e.to_string())?,
    )?;
    let exe = fs::read(&a[2]).map_err(|e| e.to_string())?;
    let out = PathBuf::from(&a[3]);
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    session.sim.configure_crt_state(123);
    session.sim.host.celebration_state = Some(p3_core::economy::celebration::CelebrationState { next: 0, town: 255 });
    session.sim.host.investigation_local_mode = true;
    session.sim.host.siege_headless_march = true;
    session.configure_battle_texts(&exe)?;
    session.sim.configure_election_texts(&exe)?;
    session.sim.configure_hunt_texts(&exe)?;
    session.enable_full_fleet()?;
    let start = session.sim.idle.clock.tick;
    let mut targets = vec![1];
    targets.extend((1..=low / 256).map(|d| d * 256));
    targets.push(low);
    for offset in targets {
        let current = session.sim.idle.clock.tick - start;
        if offset > current {
            p3_mvp::headless::advance_ticks(&mut session, offset - current)?;
        }
    }
    dump(&session, &out, &format!("tick-{}", session.sim.idle.clock.tick))?;
    for offset in low + 1..=high {
        let result = session.advance_until_encounter(start + offset, Vec::new())?;
        if result["reached"] != true {
            session.complete_encounter_tick()?;
            dump(&session, &out, &format!("tick-{}", session.sim.idle.clock.tick))?;
            if battle_steps > 0 {
                session.activate_encounter_battle()?;
                dump(&session, &out, "battle-0000")?;
                for step in 1..=battle_steps {
                    let r = session.advance_paused_battle(16)?;
                    dump(&session, &out, &format!("battle-{step:04}"))?;
                    if r["battle_destroyed"] == true {
                        break;
                    }
                }
            }
            return Ok(());
        }
        dump(&session, &out, &format!("tick-{}", session.sim.idle.clock.tick))?;
    }
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
