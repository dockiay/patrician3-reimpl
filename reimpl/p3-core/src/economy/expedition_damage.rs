//! Desgaste y sucesos del retorno de expedición: tramo x86 4E6C78..4E7099.
//! Las operaciones de centros, cartas y fin de convoy se ejecutan por separado.
use crate::rng::LcgRandom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DamageOutcome {
    pub storm: bool,
    pub attack: bool,
    pub stolen: u32,
    pub rng: u32,
}

fn r16(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}
fn r32(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(b[i..i + 4].try_into().unwrap())
}
fn w32(b: &mut [u8], i: usize, v: u32) {
    b[i..i + 4].copy_from_slice(&v.to_le_bytes())
}
fn health_sub(b: &mut [u8], i: usize, damage: u32) {
    let v = r32(b, i).wrapping_sub(damage);
    w32(b, i, if (v as i32) < 0 { 0 } else { v })
}

/// Entrada y salida preparadas sobre copias. Una cadena cíclica o incompleta,
/// división nula o aritmética excepcional no altera el estado del llamante.
pub fn prepare(
    ships: &[u8],
    convoy: &[u8],
    due: u32,
    start: u32,
    tick: u32,
    difficulty: u8,
) -> Result<(Vec<u8>, Vec<u8>, DamageOutcome), String> {
    if ships.len() % 0x180 != 0 || ships.len() / 0x180 > usize::from(u16::MAX) || convoy.len() != 0x3c {
        return Err("flota de expedición truncada".into());
    }
    let mut ships = ships.to_vec();
    let mut convoy = convoy.to_vec();
    let count = ships.len() / 0x180;
    let mut indices = Vec::new();
    let mut seen = vec![false; count];
    let mut at = usize::from(r16(&convoy, 0xa));
    while at < count {
        if seen[at] {
            return Err("convoy de expedición cíclico".into());
        }
        seen[at] = true;
        indices.push(at);
        at = usize::from(r16(&ships, at * 0x180 + 6));
    }
    let mut elapsed = due.wrapping_sub(start);
    if r16(&convoy, 0x38) == 0 {
        elapsed = elapsed.wrapping_mul(100) / 150;
    }
    let wear = (elapsed >> 8).wrapping_mul(0x280);
    for &id in &indices {
        health_sub(&mut ships, id * 0x180 + 0x18, wear)
    }
    let mut rng = LcgRandom::seeded(tick);
    let roll = rng.next_below(11);
    let mut storm = false;
    let mut attack = false;
    let mut stolen = 0;
    if roll == 9 {
        storm = true;
        for &id in &indices {
            let p = id * 0x180;
            let (range, base) = if ships[p + 0xe] <= 1 {
                (0x3333, 0x3333)
            } else {
                (0x1999, 0x1999)
            };
            let q = rng.next_below(range).wrapping_add(base);
            let max = r32(&ships, p + 0x14) as i32;
            let damage = (i64::from(max) * i64::from(q) >> 16) as u32;
            health_sub(&mut ships, p + 0x18, damage)
        }
    } else if roll == 10 {
        attack = true;
        if indices.is_empty() {
            return Err("ataque a convoy sin barcos: división nula original".into());
        }
        let total = indices
            .iter()
            .fold(0u32, |a, &id| a.wrapping_add(r32(&ships, id * 0x180 + 0x120)));
        let average = total / indices.len() as u32;
        let power = u64::from(average.wrapping_sub(u32::from(difficulty))).wrapping_mul(0x3456700) / 5;
        let defense = ((power as f64).sqrt() as u32).wrapping_add(0xaf0);
        for &id in &indices {
            let p = id * 0x180;
            let q = rng.next_below(0x4ccc).wrapping_add(0x4ccc);
            let max = r32(&ships, p + 0x14) as i32;
            let raw = (i64::from(max) * i64::from(q) >> 16) - i64::from(defense);
            health_sub(&mut ships, p + 0x18, raw.max(0) as u32)
        }
        let second = rng.next_below(11);
        let money = r32(&convoy, 0x20);
        if (second == 10 || (second > 5 && average < 12)) && (money as i32) > 5000 {
            let n = (money as i32).wrapping_mul(2500);
            if n < 0 {
                return Err("raíz de dinero de expedición fuera del dominio contrastado".into());
            }
            let base = (f64::from(n).sqrt() as u32).wrapping_mul(rng.next_below(30) + 20) / 50;
            stolen = base.min(money);
        }
        w32(&mut convoy, 0x20, money.wrapping_sub(stolen));
    }
    Ok((
        ships,
        convoy,
        DamageOutcome {
            storm,
            attack,
            stolen,
            rng: rng.state,
        },
    ))
}
