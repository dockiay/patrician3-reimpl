//! Disponibilidad de materiales para pedidos navales de comerciantes IA.
use p3_core::{ai::reputation::Merchant, economy::trade::OfficeStock, town::Town};
const WARES: [usize; 5] = [11, 8, 12, 17, 15];
const UNITS: [i32; 5] = [2000, 200, 200, 2000, 200];
const REQUIREMENTS: [[[i32; 5]; 3]; 4] = [
    [[7, 3, 3, 3, 20], [9, 3, 3, 3, 20], [11, 3, 3, 3, 20]],
    [[12, 5, 5, 5, 30], [14, 5, 5, 5, 30], [16, 5, 5, 5, 30]],
    [[18, 3, 4, 4, 40], [20, 3, 4, 4, 40], [22, 3, 4, 4, 40]],
    [[30, 10, 10, 8, 50], [33, 10, 10, 8, 50], [36, 10, 10, 8, 50]],
];
/// Parte booleana de 0x0052B2C0 para un comerciante IA; no calcula el presupuesto.
pub(crate) fn possible(
    town: &Town,
    kind: u8,
    merchant: &Merchant,
    office: Option<&OfficeStock>,
    ships: &[Option<Vec<u8>>],
) -> Result<bool, String> {
    if merchant.bytes.len() < 0x10 {
        return Err("comerciante de astillero truncado".into());
    }
    if merchant.u16(8) == 0 {
        return Err("coste de astillero humano no conectado".into());
    }
    if kind >= 4 {
        return Ok(false);
    }
    let level = town.u8(0x824 + usize::from(kind)) as i8;
    if level < 0 || town.more_flags() & 0x20000 == 0 || town.flags() & 1 != 0 && kind & 2 != 0 {
        return Ok(false);
    }
    if town.u16(0x854) == 0 {
        return Err("divisor de astillero nulo".into());
    }
    let requirement = REQUIREMENTS[usize::from(kind)][usize::from((level as u8).min(2))];
    let mut stock = office.map_or([0i32; 5], |o| std::array::from_fn(|i| o.stock[WARES[i]]));
    let mut at = usize::from(merchant.u16(0xe));
    let mut seen = vec![false; ships.len()];
    while at < ships.len() {
        if seen[at] {
            return Err("cadena de barcos de astillero cíclica".into());
        }
        seen[at] = true;
        let ship = ships[at].as_deref().ok_or("barco de astillero ausente")?;
        if ship.len() < 0x138 {
            return Err("barco de astillero truncado".into());
        }
        let word = |i| u16::from_le_bytes([ship[i], ship[i + 1]]);
        if ship[0x39] == town.u8(0x2c0) && word(0x134) < 4 && ship[0x136] & 1 == 0 {
            for i in 0..5 {
                let j = 0x54 + 4 * WARES[i];
                stock[i] = stock[i].wrapping_add(i32::from_le_bytes(ship[j..j + 4].try_into().unwrap()));
            }
        }
        at = usize::from(word(4));
    }
    Ok((0..5).all(|i| {
        let owned = stock[i] / UNITS[i];
        owned >= requirement[i] || town.stock(WARES[i]) / UNITS[i] >= requirement[i].wrapping_sub(owned)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn hex(b: &[u8]) -> String {
        b.iter().map(|v| format!("{v:02x}")).collect()
    }
    #[test]
    fn ai_cost_availability_matches_original() {
        let (Ok(exe), Ok(python)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else {
            return;
        };
        let (mut cases, mut actual) = (Vec::new(), Vec::new());
        for n in 0..1024usize {
            let kind = (n % 4) as u8;
            let mut t = Town::default();
            t.set_u8(0x2c0, 0);
            t.set_u8(
                0x824 + kind as usize,
                if n % 13 == 0 { 255 } else { ((n / 4) % 4) as u8 },
            );
            t.set_u16(0x854, 7);
            t.set_u32(0x76c, if n % 17 == 0 { 0 } else { 0x20000 });
            t.set_flags(if n % 7 == 0 { 1 } else { 0 });
            for (i, &w) in WARES.iter().enumerate() {
                t.set_i32(
                    4 + 4 * w,
                    ((((n * 13 + i * 31) % 80) as i32) - 5) * UNITS[i] + (n % 173) as i32,
                );
            }
            let mut m = Merchant { bytes: vec![0; 0x650] };
            m.set_u16(8, 1);
            m.set_u16(0xe, 0);
            let mut ships = Vec::new();
            for j in 0..4 {
                let mut ship = vec![0u8; 0x180];
                ship[4..6].copy_from_slice(&(if j == 3 { u16::MAX } else { (j + 1) as u16 }).to_le_bytes());
                ship[0x39] = u8::from((n + j) % 5 == 0);
                ship[0x134..0x136].copy_from_slice(&(if (n + j) % 7 == 0 { 4u16 } else { 1u16 }).to_le_bytes());
                ship[0x136] = u8::from((n + j) % 11 == 0);
                for (i, &w) in WARES.iter().enumerate() {
                    let v = ((((n + i * 5 + j * 7) % 22) as i32) - 2) * UNITS[i] + (n % 137) as i32;
                    ship[0x54 + w * 4..0x58 + w * 4].copy_from_slice(&v.to_le_bytes());
                }
                ships.push(Some(ship))
            }
            let mut office = OfficeStock::default();
            for (i, &w) in WARES.iter().enumerate() {
                office.stock[w] = ((((n * 3 + i * 13) % 40) as i32) - 3) * UNITS[i] + (n % 113) as i32
            }
            let has_office = n % 3 != 0;
            let mut office_raw = vec![0u8; 0x44c];
            for i in 0..24 {
                office_raw[4 + i * 4..8 + i * 4].copy_from_slice(&office.stock[i].to_le_bytes())
            }
            let raw = ships
                .iter()
                .flat_map(|s| s.as_ref().unwrap().iter().copied())
                .collect::<Vec<_>>();
            cases.push(serde_json::json!({"town":hex(t.0.as_slice()),"merchant":hex(&m.bytes),"ships":hex(&raw),"office":hex(&office_raw),"has_office":has_office,"ship_count":4,"kind":kind}));
            actual.push(possible(&t, kind, &m, has_office.then_some(&office), &ships).unwrap());
        }
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_ai_ship_cost.py");
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
        let expected: Vec<bool> = serde_json::from_slice(&out.stdout).unwrap();
        assert!(expected.iter().any(|&v| v));
        assert!(expected.iter().any(|&v| !v));
        for (i, (a, b)) in actual.iter().zip(expected.iter()).enumerate() {
            assert_eq!(a, b, "caso {i}")
        }
    }
}
