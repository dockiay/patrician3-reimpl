//! Planificación de la reposición de capitanes de taberna, tarea 0x08.
use p3_core::scheduler::TaskRecord;

/// Devuelve, en orden, (ciudad, capitán pirata), sin modificar el mundo.
/// Los bits 0/1 indican disponibilidad normal/pirata en cada ciudad.
pub(crate) fn plan(
    task: &mut TaskRecord,
    availability: &[u8],
    ships: [u32; 2],
    captains: [u32; 2],
) -> Result<Vec<(usize, bool)>, String> {
    if task.kind != 8 || availability.is_empty() || availability.len() > 40 {
        return Err("ciudades de reposición inválidas".into());
    }
    let mut bits = [0xffu8; 40];
    for (dst, src) in bits.iter_mut().zip(availability) {
        *dst = *src & 3
    }
    let normals = availability.iter().filter(|&&v| v & 1 != 0).count() as i32;
    let pirates = availability.iter().filter(|&&v| v & 2 != 0).count();
    let mut next = *task;
    let mut actions = Vec::new();
    if pirates < 3 {
        let step = next.u32_at(0x14).wrapping_add(3) % 23 + 1;
        next.set_u32_at(0x14, step);
        let mut city = next.u32_at(0x10);
        let mut found = None;
        for _ in 0..40 {
            city = city.wrapping_add(step) % 37;
            next.set_u32_at(0x10, city);
            if bits[city as usize] & 2 == 0 {
                found = Some(city as usize);
                break;
            }
        }
        let city = found.ok_or("no hay ciudad pirata accesible")?;
        if city >= availability.len() {
            return Err("ciudad pirata fuera de rango".into());
        }
        actions.push((city, true));
    }
    if normals > 7 {
        next.due = next.due.wrapping_add(0x700)
    } else {
        if ships.iter().chain(captains.iter()).any(|&n| n > i32::MAX as u32) {
            return Err("conteo de flota excesivo".into());
        }
        let shortage = ((ships[1] as i32) / 5)
            .wrapping_add((ships[0] as i32).wrapping_add(1) >> 1)
            .wrapping_sub(captains[1] as i32)
            .wrapping_sub(captains[0] as i32);
        if shortage < normals && normals > 1 {
            next.due = next.due.wrapping_add(0x400)
        } else {
            let mut city = next.u32_at(8) % availability.len() as u32;
            let raw = next.u32_at(12).wrapping_add(3) % 29;
            let step = if raw == 0 {
                3
            } else if raw == 20 {
                2
            } else {
                raw
            };
            next.set_u32_at(12, step);
            let mut found = None;
            for _ in 0..37 {
                city = city.wrapping_add(step) % 37;
                if bits[city as usize] & 1 == 0 {
                    found = Some(city as usize);
                    break;
                }
            }
            let city = found.ok_or("no hay ciudad normal accesible")?;
            if city >= availability.len() {
                return Err("ciudad normal fuera de rango".into());
            }
            next.set_u32_at(8, city as u32);
            actions.push((city, false));
            next.due = next.due.wrapping_add(0x200)
        }
    }
    *task = next;
    Ok(actions)
}

/// Consulta las cadenas de taberna igual que los dos buscadores originales.
/// Rechaza ciclos: los buscadores x86 no terminan con esos datos corruptos.
pub(crate) fn availability(
    towns: &[p3_core::town::Town],
    captains: &[[u8; 16]],
    merchant_count: u16,
    player: u32,
) -> Result<Vec<u8>, String> {
    if captains.len() > usize::from(u16::MAX) {
        return Err("pool de capitanes excesivo".into());
    }
    let mut result = Vec::with_capacity(towns.len());
    for town in towns {
        let mut at = usize::from(town.u16(0x82e));
        let mut seen = vec![false; captains.len()];
        // El llamante comprueba <= capacidad para la búsqueda pirata.
        let (mut normal, mut pirate) = (false, captains.len() == usize::from(u16::MAX));
        while at < captains.len() {
            if seen[at] {
                return Err("cadena de capitanes cíclica".into());
            }
            seen[at] = true;
            let cap = &captains[at];
            if cap[8] <= 0x20 && (u16::from(cap[15]) == merchant_count || cap[15] == 0xff) {
                normal = true;
            }
            if cap[8] > 0x20 && (u32::from(cap[15]) == player || cap[15] == 0xff) {
                pirate = true;
            }
            at = usize::from(u16::from_le_bytes([cap[0], cap[1]]));
        }
        result.push(u8::from(normal) | (u8::from(pirate) << 1));
    }
    Ok(result)
}

/// Cuenta barcos en las cadenas comerciales y los que tienen capitán.
pub(crate) fn fleet_counts(
    merchants: &[p3_core::ai::reputation::Merchant],
    ships: &[Option<Vec<u8>>],
    captain_capacity: usize,
) -> Result<([u32; 2], [u32; 2]), String> {
    let mut ship_counts = [0u32; 2];
    let mut captain_counts = [0u32; 2];
    for merchant in merchants {
        if merchant.bytes.len() < 0x10 {
            return Err("comerciante truncado".into());
        }
        let side = usize::from(merchant.u16(8) != 0);
        let mut at = usize::from(merchant.u16(0xe));
        let mut seen = vec![false; ships.len()];
        while at < ships.len() {
            if seen[at] {
                return Err("cadena de barcos cíclica".into());
            }
            seen[at] = true;
            let ship = ships[at].as_deref().ok_or("barco ausente")?;
            if ship.len() < 0x136 {
                return Err("barco truncado".into());
            }
            let word = |i| u16::from_le_bytes([ship[i], ship[i + 1]]);
            if word(0x134) != 0x11 {
                ship_counts[side] = ship_counts[side].wrapping_add(1);
                if usize::from(word(0x42)) < captain_capacity {
                    captain_counts[side] = captain_counts[side].wrapping_add(1);
                }
            }
            at = usize::from(word(4));
        }
    }
    Ok((ship_counts, captain_counts))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn hex(b: &[u8]) -> String {
        b.iter().map(|v| format!("{v:02x}")).collect()
    }
    #[test]
    fn census_rejects_cyclic_chains() {
        let mut town = p3_core::town::Town::default();
        town.set_u16(0x82e, 0);
        let mut captain = [0u8; 16];
        captain[..2].copy_from_slice(&0u16.to_le_bytes());
        assert!(availability(&[town], &[captain], 1, 0).is_err());
        let mut merchant = p3_core::ai::reputation::Merchant { bytes: vec![0; 0x650] };
        merchant.bytes[0xe..0x10].copy_from_slice(&0u16.to_le_bytes());
        let mut ship = vec![0; 0x180];
        ship[4..6].copy_from_slice(&0u16.to_le_bytes());
        assert!(fleet_counts(&[merchant], &[Some(ship)], 1).is_err());
    }

    #[test]
    fn census_matches_original() {
        let (Ok(exe), Ok(python)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else {
            return;
        };
        let mut cases = Vec::new();
        let mut actual = Vec::new();
        for i in 0..128u32 {
            let mut towns = vec![p3_core::town::Town::default(); 8];
            let mut caps = vec![[0u8; 16]; 32];
            for (j, town) in towns.iter_mut().enumerate() {
                town.set_u16(0x82e, (j * 4) as u16);
                for k in 0..4 {
                    let id = j * 4 + k;
                    let cap = &mut caps[id];
                    cap[..2].copy_from_slice(&(if k == 3 { u16::MAX } else { (id + 1) as u16 }).to_le_bytes());
                    cap[8] = if (id as u32 + i) % 3 == 0 { 0x21 } else { 0x20 };
                    cap[15] = match (id as u32 + i) % 5 {
                        0 => 0xff,
                        1 => 4,
                        2 => 1,
                        _ => 3,
                    };
                }
            }
            let mut merchants = Vec::new();
            let mut ships = Vec::new();
            for j in 0..4usize {
                let mut m = p3_core::ai::reputation::Merchant { bytes: vec![0; 0x650] };
                m.bytes[8..10].copy_from_slice(&(u16::from(j % 2 == 1)).to_le_bytes());
                m.bytes[0xe..0x10].copy_from_slice(&((j * 3) as u16).to_le_bytes());
                merchants.push(m);
                for k in 0..3usize {
                    let id = j * 3 + k;
                    let mut ship = vec![0u8; 0x180];
                    ship[4..6].copy_from_slice(&(if k == 2 { u16::MAX } else { (id + 1) as u16 }).to_le_bytes());
                    ship[0x42..0x44]
                        .copy_from_slice(&(if (i + id as u32) % 3 == 0 { u16::MAX } else { id as u16 }).to_le_bytes());
                    ship[0x134..0x136]
                        .copy_from_slice(&(if (i + id as u32) % 4 == 0 { 0x11u16 } else { 3u16 }).to_le_bytes());
                    ships.push(Some(ship));
                }
            }
            let flags = availability(&towns, &caps, 4, 1).unwrap();
            let (ships_count, captains_count) = fleet_counts(&merchants, &ships, caps.len()).unwrap();
            let towns_raw = towns.iter().flat_map(|x| x.0.iter().copied()).collect::<Vec<_>>();
            let caps_raw = caps.iter().flatten().copied().collect::<Vec<_>>();
            let merchants_raw = merchants
                .iter()
                .flat_map(|x| x.bytes.iter().copied())
                .collect::<Vec<_>>();
            let ships_raw = ships
                .iter()
                .flat_map(|x| x.as_ref().unwrap().iter().copied())
                .collect::<Vec<_>>();
            cases.push(serde_json::json!({"towns":hex(&towns_raw),"captains":hex(&caps_raw),"merchants":hex(&merchants_raw),"ships":hex(&ships_raw),"town_count":towns.len(),"captain_count":caps.len(),"merchant_count":merchants.len(),"ship_count":ships.len(),"player":1}));
            actual.push(serde_json::json!({"flags":flags,"ships":ships_count,"captains":captains_count}));
        }
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_captain_census.py");
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
        assert_eq!(actual, expected);
    }

    #[test]
    fn refill_planning_matches_original() {
        let (Ok(exe), Ok(python)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else {
            return;
        };
        let mut cases = Vec::new();
        let mut inputs = Vec::new();
        for i in 0..512u32 {
            let n = if i & 1 == 0 { 24 } else { 37 };
            let mut flags = vec![0u8; n];
            for (j, flag) in flags.iter_mut().enumerate() {
                let j = j as u32;
                *flag = match i % 4 {
                    0 => u8::from(j < 12) | 2 * u8::from(j < 4),
                    1 => u8::from(j < 3) | 2 * u8::from(j < 4),
                    2 => u8::from(j < 3) | 2 * u8::from(j < 2),
                    _ => 2 * u8::from(j < 2),
                }
            }
            let ships = [i % 80, (i * 7) % 100];
            let captains = [i % 20, (i * 3) % 24];
            let mut task = TaskRecord {
                due: 0x123400 + i,
                next: 0xffff,
                kind: 8,
                ..Default::default()
            };
            task.set_u32_at(8, i * 31);
            task.set_u32_at(12, i * 19);
            task.set_u32_at(0x10, i * 11);
            task.set_u32_at(0x14, i * 7);
            let mut raw = Vec::new();
            raw.extend_from_slice(&task.due.to_le_bytes());
            raw.extend_from_slice(&task.next.to_le_bytes());
            raw.extend_from_slice(&task.kind.to_le_bytes());
            raw.extend_from_slice(&task.payload);
            cases.push(serde_json::json!({"task":hex(&raw),"flags":flags,"ships":ships,"captains":captains}));
            inputs.push((task, flags, ships, captains));
        }
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_captain_refill.py");
        let mut child = std::process::Command::new(&python)
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
        for (i, ((mut task, flags, ships, captains), want)) in inputs.into_iter().zip(expected).enumerate() {
            let actions = plan(&mut task, &flags, ships, captains).unwrap();
            let mut raw = Vec::new();
            raw.extend_from_slice(&task.due.to_le_bytes());
            raw.extend_from_slice(&task.next.to_le_bytes());
            raw.extend_from_slice(&task.kind.to_le_bytes());
            raw.extend_from_slice(&task.payload);
            assert_eq!(hex(&raw), want["task"].as_str().unwrap(), "task {i}");
            assert_eq!(serde_json::json!(actions), want["actions"], "actions {i}");
        }
    }
}
