//! Tarea de dedicación de estatua de alcalde 0x004ECB14.
use p3_core::{
    ai::reputation::Merchant,
    buildings::Site,
    scheduler::{Scheduler, TaskRecord},
    town::Town,
};
#[derive(Debug)]
pub(crate) struct Outcome {
    pub keep: bool,
    pub letter: Option<[u8; 16]>,
    pub visual: Option<(u8, u8, u8, u8)>,
}
/// Prepara los efectos en el plano clonado; las cartas se insertan aparte.
pub(crate) fn advance(
    task: &mut TaskRecord,
    scheduler: &Scheduler,
    towns: &[Town],
    sites: &mut [Site],
    merchants: &[Merchant],
    sea_x: Option<u32>,
    tick: u32,
) -> Result<Outcome, String> {
    let none = || Outcome {
        keep: false,
        letter: None,
        visual: None,
    };
    if task.kind != 0x38 {
        return Err("tarea de estatua inválida".into());
    }
    let city = usize::from(task.u8_at(8));
    let Some(town) = towns.get(city) else { return Ok(none()) };
    let owner = usize::from(task.u8_at(9));
    let Some(merchant) = merchants.get(owner) else {
        return Ok(none());
    };
    if usize::from(town.u8(0x6f1)) != owner {
        return Ok(none());
    }
    if merchant.bytes.len() < 0x470 {
        return Err("alcalde de estatua truncado".into());
    }
    let mut at = task.next;
    let mut seen = vec![false; usize::from(scheduler.capacity)];
    while at < scheduler.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("cola de estatua cíclica".into());
        }
        seen[i] = true;
        let other = scheduler.records.get(i).ok_or("cola de estatua truncada")?;
        if other.kind == 0x38 && usize::from(other.u8_at(9)) == owner {
            return Ok(none());
        }
        at = other.next;
    }
    let site = sites.get_mut(city).ok_or("plano de estatua ausente")?;
    let count = usize::from(town.u16(0x776));
    let mut at = usize::from(town.u16(0x764));
    let mut seen = vec![false; count];
    let mut found = None;
    while at < count {
        if seen[at] {
            return Err("lista de edificios de estatua cíclica".into());
        }
        seen[at] = true;
        let record = site
            .records
            .get(at * 8..at * 8 + 8)
            .ok_or("edificio de estatua truncado")?;
        if record[3] == 0x2d {
            if usize::from(record[2]) >= merchants.len() {
                found = Some(at)
            }
            break;
        }
        at = usize::from(u16::from_le_bytes([record[4], record[5]]));
    }
    let Some(id) = found else { return Ok(none()) };
    let delta = sea_x.map_or(0, |x| ((x & 0x3ff) * 0x1f00) >> 9);
    task.due = task.due.wrapping_add(0x3e00).wrapping_add(delta);
    if Site::is_plan_point_valid(town, 1)
        || (Site::is_plan_point_valid(town, 2) && usize::from(town.u16(0x766)) < count)
    {
        return Ok(Outcome {
            keep: true,
            letter: None,
            visual: None,
        });
    }
    let record = &mut site.records[id * 8..id * 8 + 8];
    record[2] = owner as u8;
    let mut variant = ((owner as u32).wrapping_add(tick).wrapping_add(city as u32) % 3) as u8;
    let epithet = merchant.get_merchant_epithet_index();
    if epithet > 7 {
        variant += 3
    }
    record[7] = variant;
    let mut letter = [0u8; 16];
    letter[1] = 0x80;
    letter[2..4].fill(0xff);
    letter[4] = 0x7b;
    letter[5] = city as u8;
    letter[8..10].copy_from_slice(&u16::from(epithet).to_le_bytes());
    letter[10..12].copy_from_slice(&u16::from(merchant.bytes[0x1c]).to_le_bytes());
    letter[12..14].copy_from_slice(&u16::from(merchant.bytes[0x1d]).to_le_bytes());
    Ok(Outcome {
        keep: false,
        letter: Some(letter),
        visual: Some((record[0], record[1], variant, owner as u8)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn hex(b: &[u8]) -> String {
        b.iter().map(|v| format!("{v:02x}")).collect()
    }
    fn encode(t: &TaskRecord) -> Vec<u8> {
        [
            t.due.to_le_bytes().as_slice(),
            t.next.to_le_bytes().as_slice(),
            t.kind.to_le_bytes().as_slice(),
            t.payload.as_slice(),
        ]
        .concat()
    }
    #[test]
    fn statue_matches_original() {
        let (Ok(exe), Ok(python)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else {
            return;
        };
        let (mut cases, mut actual) = (Vec::new(), Vec::new());
        for n in 0..256u32 {
            let mut town = Town::default();
            town.set_u8(0x6f1, if n % 7 == 0 { 1 } else { 0 });
            town.set_u16(0x776, 2);
            town.set_u16(0x764, 0);
            town.set_u16(0x766, if n % 5 == 0 { 0 } else { u16::MAX });
            town.set_u32(0x7a8, if n % 3 == 0 { 10 } else { 0 });
            town.set_u32(0x7ac, if n % 3 == 0 { 10 } else { 0 });
            for i in 0..8 {
                town.set_u8(0x7b0 + i, if n % 3 == 0 { 0 } else { 0xff })
            }
            let mut site = Site::default();
            site.records = vec![0, 0, 0xff, 0x2d, 1, 0, 0, 0, 0, 0, 0xff, 0, 0xff, 0xff, 0, 0];
            if n % 11 == 0 {
                site.records[2] = 0
            }
            if n % 13 == 0 {
                site.records[3] = 0x2c
            }
            let mut merchant = Merchant { bytes: vec![0; 0x650] };
            merchant.bytes[0x1c] = 12;
            merchant.bytes[0x1d] = 33;
            merchant.set_i32(0x46c, (n as i32) * 100000);
            let mut scheduler = Scheduler::new();
            let a = scheduler.add_task(0x100000 + n);
            let b = scheduler.add_task(0x200000 + n);
            scheduler.records[a as usize].kind = 0x38;
            scheduler.records[a as usize].set_u8_at(8, 0);
            scheduler.records[a as usize].set_u8_at(9, 0);
            scheduler.records[b as usize].kind = if n % 17 == 0 { 0x38 } else { 0xffff };
            scheduler.records[b as usize].set_u8_at(9, 0);
            scheduler.capacity = 2;
            scheduler.records.truncate(2);
            scheduler.records[b as usize].next = u16::MAX;
            let mut task = scheduler.records[a as usize];
            let tick = 0x100000 + n;
            let x = n * 13;
            let sea = if n % 2 == 0 { Some(x) } else { None };
            let mut ship = vec![0u8; 0x180];
            ship[0x1c..0x20].copy_from_slice(&x.to_le_bytes());
            let raw = [encode(&scheduler.records[0]), encode(&scheduler.records[1])].concat();
            cases.push(serde_json::json!({"town":hex(town.0.as_slice()),"merchant":hex(&merchant.bytes),"records":hex(&site.records),"tasks":hex(&raw),"ship":hex(&ship),"sea_head":if sea.is_some(){0}else{1},"tick":tick}));
            let out = advance(
                &mut task,
                &scheduler,
                &[town],
                std::slice::from_mut(&mut site),
                &[merchant],
                sea,
                tick,
            )
            .unwrap();
            let letters = out.letter.map_or_else(Vec::new, |letter| {
                vec![
                    serde_json::json!([4294967294u32, hex(&letter)]),
                    serde_json::json!([4294967295u32, hex(&letter)]),
                ]
            });
            actual.push(serde_json::json!({"task":hex(&encode(&task)),"records":hex(&site.records),"keep":u8::from(out.keep),"letters":letters}));
        }
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_mayor_statue.py");
        let mut child = std::process::Command::new(python)
            .arg(script)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(serde_json::json!({"binary":exe,"cases":cases}).to_string().as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let expected: Vec<serde_json::Value> = serde_json::from_slice(&out.stdout).unwrap();
        for (i, (a, b)) in actual.iter().zip(expected.iter()).enumerate() {
            assert_eq!(a, b, "caso {i}")
        }
    }
}
