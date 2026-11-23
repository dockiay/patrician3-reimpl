//! Final ranking (`004220D0`), including original human selection and stable ties.
use crate::{ai::reputation::Merchant, town::Town};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub merchant: u8,
    pub score: i32,
    pub reputation: i32,
    pub value: i32,
    pub ships: u16,
    pub offices: u8,
    pub buildings: i32,
    pub employees: i32,
    pub non_human: u8,
    pub selected: bool,
}
fn word(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([b[p], b[p + 1]])
}
/// Rebuilds the eight entries used by `004220B0`; the rest retain their order.
pub fn build(
    merchants: &[Merchant],
    towns: &[Town],
    offices: &[Vec<u8>],
    count: u16,
    score_flag: u8,
) -> Result<Vec<Row>, String> {
    if !(28..=255).contains(&count)
        || usize::from(count) > merchants.len()
        || towns.len() > 40
        || merchants
            .iter()
            .take(usize::from(count))
            .any(|m| m.bytes.len() != 0x650)
        || offices.iter().any(|o| o.len() != 0x44c)
    {
        return Err("datos de clasificacion incompletos o el original lee fuera de la tabla".into());
    }
    let mut rows = Vec::with_capacity(usize::from(count) - 20);
    let mut human_count = 0usize;
    for id in 20..usize::from(count) {
        let m = &merchants[id];
        let ctl = m.u16(8);
        let non_human = u8::from(ctl != 0 && ctl & 0x8000 == 0);
        if non_human == 0 {
            human_count += 1
        }
        let mut next = usize::from(m.u16(0xc));
        let mut seen = vec![false; offices.len()];
        let (mut n, mut buildings, mut employees) = (0u8, 0i32, 0i32);
        while next < offices.len() {
            if seen[next] {
                return Err("cadena ciclica en clasificacion".into());
            }
            seen[next] = true;
            let o = &offices[next];
            n = n.wrapping_add(1);
            buildings = buildings
                .wrapping_add(i32::from(word(o, 0x2d2)))
                .wrapping_add(i32::from(word(o, 0x2d4)));
            employees = employees.wrapping_add(i32::from(word(o, 0x448)));
            next = usize::from(word(o, 0x2c8));
        }
        let home = usize::from(m.bytes[0x19]);
        let rank = u32::from(m.bytes[0x39c + home]);
        let mut epithet = u32::from(m.get_merchant_epithet_index());
        if epithet > 7 {
            epithet -= 7
        }
        let reputation = i32::from(m.calc_total_reputation_score(towns.len() as i32, towns) as i16);
        let value = m.i32(0x46c);
        let base = (epithet + rank * 2).wrapping_mul(3) as i32;
        let mut score = base.wrapping_add(reputation.wrapping_mul(2));
        if score_flag != 0 {
            score = score.wrapping_add(reputation)
        }
        let amount = (i64::from(value).unsigned_abs() / 100) as f64;
        let root = amount.sqrt() as i32;
        score = if value < 0 {
            score.wrapping_sub(root)
        } else {
            score.wrapping_add(root)
        };
        if score < 0 {
            score = 0
        }
        rows.push(Row {
            merchant: id as u8,
            score,
            reputation,
            value,
            ships: m.u16(0x478),
            offices: n,
            buildings,
            employees,
            non_human,
            selected: non_human == 0,
        });
    }
    if human_count < 8 {
        for _ in human_count..8 {
            let pos = rows
                .iter()
                .enumerate()
                .filter(|(_, r)| !r.selected)
                .max_by_key(|(i, r)| (r.score, std::cmp::Reverse(*i)))
                .map(|(i, _)| i)
                .ok_or("el original selecciona fuera de la tabla")?;
            rows[pos].selected = true
        }
    }
    for i in 0..8 {
        let pos = (i..rows.len())
            .filter(|&j| rows[j].selected)
            .max_by_key(|&j| (rows[j].score, std::cmp::Reverse(j)))
            .ok_or("el original ordena fuera de la tabla")?;
        rows.swap(i, pos)
    }
    Ok(rows)
}
/// If the uninitialised screen flag is unknown, accept only an invariant winner.
pub fn winner(
    merchants: &[Merchant],
    towns: &[Town],
    offices: &[Vec<u8>],
    count: u16,
    score_flag: Option<u8>,
) -> Result<u16, String> {
    if let Some(flag) = score_flag {
        return Ok(u16::from(build(merchants, towns, offices, count, flag)?[0].merchant));
    }
    let a = build(merchants, towns, offices, count, 0)?[0].merchant;
    let b = build(merchants, towns, offices, count, 1)?[0].merchant;
    if a != b {
        return Err("la clasificacion depende del byte de pantalla no disponible".into());
    }
    Ok(u16::from(a))
}
