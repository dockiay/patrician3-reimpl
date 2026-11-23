//! Cálculo de déficits de `ai_decide_construction` (0x004DEBA0).
use crate::{economy::world::TickEntry, town::Town};
const WARE_BY_FACILITY: [u8; 24] = [
    6, 6, 6, 20, 9, 2, 3, 12, 5, 0, 1, 11, 8, 4, 16, 14, 7, 18, 19, 15, 17, 0, 0, 0,
];
const FACILITY_BY_WARE: [u8; 20] = [9, 10, 5, 6, 13, 8, 0, 16, 12, 4, 255, 11, 7, 10, 15, 19, 14, 20, 17, 18];
#[derive(Clone, Debug)]
pub struct OfficeDemand {
    pub next: u16,
    pub consumption: [i32; 24],
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DemandScores {
    pub production: [i32; 24],
    pub demand: [i32; 24],
    pub stock: [i32; 24],
    pub facilities: [i32; 21],
    pub fish: i32,
}

pub fn calculate(
    towns: &[Town],
    offices: &[OfficeDemand],
    entries: &[Vec<TickEntry>],
    month: u8,
) -> Result<DemandScores, String> {
    if towns.len() != entries.len() {
        return Err("entradas de suministro incompletas".into());
    }
    let mut out = DemandScores {
        production: [0; 24],
        demand: [0; 24],
        stock: [0; 24],
        facilities: [0; 21],
        fish: 0,
    };
    for (town, nodes) in towns.iter().zip(entries) {
        for w in 0..20 {
            out.demand[w] = out.demand[w]
                .wrapping_add(town.i32(0x310 + w * 4))
                .wrapping_add(town.i32(0x64 + w * 4));
            out.production[w] = out.production[w].wrapping_add(town.i32(0x490 + w * 4));
            out.stock[w] = out.stock[w].wrapping_add(town.i32(4 + w * 4))
        }
        let mut at = usize::from(town.u16(0x784));
        let mut seen = vec![false; offices.len()];
        while at < offices.len() {
            if seen[at] {
                return Err("cadena urbana de oficinas cíclica".into());
            }
            seen[at] = true;
            let office = &offices[at];
            for w in 0..20 {
                out.demand[w] = out.demand[w].wrapping_add(office.consumption[w])
            }
            at = usize::from(office.next)
        }
        for node in nodes {
            let good = usize::from(
                *WARE_BY_FACILITY
                    .get(usize::from(node.facility_type))
                    .ok_or("tipo de suministro inválido")?,
            );
            let value = if node.factor == 0 {
                0
            } else if node.factor >= 1024 {
                node.sum
            } else {
                ((i64::from(node.sum) * i64::from(node.factor)) >> 10) as i32
            };
            out.production[good] = out.production[good].wrapping_add(value)
        }
    }
    for w in [0, 17, 7, 5] {
        out.demand[w] = out.demand[w].wrapping_add(out.demand[w] >> 3)
    }
    if !(2..=10).contains(&month) {
        for w in [0, 17, 5] {
            out.demand[w] = 0
        }
    }
    for w in (0..20).rev() {
        let p = out.production[w];
        let minimum = p.wrapping_add(p >> 5).wrapping_add(p >> 4);
        let score = if out.demand[w] < (out.stock[w] >> 5) || out.demand[w] < minimum {
            0
        } else if p <= 100 {
            100
        } else {
            out.demand[w] / (p / 100)
        };
        if w == 10 {
            out.fish = score
        } else {
            out.facilities[usize::from(FACILITY_BY_WARE[w])] = score
        }
    }
    Ok(out)
}

#[derive(Clone, Copy, Debug)]
pub struct OfficeRank {
    pub next: u16,
    pub owner: u16,
    pub first_business: u16,
}
#[derive(Clone, Copy, Debug)]
pub struct BusinessRank {
    pub next: u16,
    pub employees: u16,
    pub maximum: u16,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildRanking {
    pub facilities: [i32; 21],
    pub best: [u32; 5],
    pub town_scores: Vec<i32>,
    pub town_order: Vec<u8>,
}

/// Ordena las ciudades y elimina instalaciones ya en construcción.
pub fn rank(
    scores: &DemandScores,
    towns: &[Town],
    sites: &[Vec<[u8; 8]>],
    offices: &[OfficeRank],
    businesses: &[BusinessRank],
    merchants: &[crate::ai::reputation::Merchant],
    rng: &mut crate::rng::CrtRand,
) -> Result<BuildRanking, String> {
    if towns.len() > 40 || towns.len() != sites.len() {
        return Err("ciudades para construcción IA incompatibles".into());
    }
    let mut random = *rng;
    let mut result = BuildRanking {
        facilities: scores.facilities,
        best: [0; 5],
        town_scores: Vec::with_capacity(towns.len()),
        town_order: (0..towns.len()).map(|i| i as u8).collect(),
    };
    for (town, records) in towns.iter().zip(sites) {
        let population = town.i32(0x2d4);
        let mut value = if population > 100000 {
            200000i32.wrapping_sub(population)
        } else {
            100000
        };
        value = value.wrapping_sub(random.rand());
        if town.u32(0x2c8) & 0xe0 != 0 {
            let count = usize::from(town.u16(0x776));
            if records.len() != count {
                return Err("registros de construcción incompletos".into());
            }
            let mut at = usize::from(town.u16(0x762));
            let mut seen = vec![false; count];
            while at < count {
                if seen[at] {
                    return Err("obras cíclicas".into());
                }
                seen[at] = true;
                let row = &records[at];
                if row[3] < 21 {
                    result.facilities[usize::from(row[3])] = 0
                }
                at = usize::from(u16::from_le_bytes([row[4], row[5]]));
            }
            let mut oi = usize::from(town.u16(0x784));
            let mut seen_offices = vec![false; offices.len()];
            let mut current = 0u32;
            let mut maximum = 0u32;
            while oi < offices.len() {
                if seen_offices[oi] {
                    return Err("oficinas cíclicas".into());
                }
                seen_offices[oi] = true;
                let office = offices[oi];
                let merchant = merchants
                    .get(usize::from(office.owner))
                    .ok_or("comerciante de oficina ausente")?;
                if merchant.bytes.len() < 10 {
                    return Err("comerciante de oficina truncado".into());
                }
                if merchant.u16(8) != 0 {
                    let mut bi = usize::from(office.first_business);
                    let mut seen = vec![false; businesses.len()];
                    while bi < businesses.len() {
                        if seen[bi] {
                            return Err("negocios cíclicos".into());
                        }
                        seen[bi] = true;
                        let b = businesses[bi];
                        current = current.wrapping_add(u32::from(b.employees));
                        maximum = maximum.wrapping_add(u32::from(b.maximum));
                        bi = usize::from(b.next)
                    }
                }
                oi = usize::from(office.next)
            }
            if maximum != 0 && current.wrapping_mul(20) < maximum.wrapping_mul(17) {
                value = 0
            }
        }
        result.town_scores.push(value)
    }
    result.facilities[0] = 0;
    let mut best = [0u32; 6];
    for facility in 4..21usize {
        for pos in (1..=5).rev() {
            if result.facilities[best[pos - 1] as usize] < result.facilities[facility] {
                best[pos] = best[pos - 1];
                best[pos - 1] = facility as u32
            } else {
                break;
            }
        }
    }
    result.best.copy_from_slice(&best[..5]);
    for i in 0..result.town_order.len() {
        for j in i + 1..result.town_order.len() {
            let a = usize::from(result.town_order[i]);
            let b = usize::from(result.town_order[j]);
            if result.town_scores[a] < result.town_scores[b] {
                result.town_order.swap(i, j)
            }
        }
    }
    *rng = random;
    Ok(result)
}

const WARE_FOR_FACILITY: [u8; 21] = [
    255, 255, 255, 12, 12, 4, 0, 16, 255, 255, 4, 255, 14, 11, 255, 255, 255, 11, 11, 11, 255,
];
#[derive(Clone, Debug)]
pub struct OfficeSelect {
    pub next: u16,
    pub owner: u16,
    pub stock: [i32; 24],
    pub target: [i32; 24],
}
pub trait Placement {
    fn near_office(
        &mut self,
        city: u8,
        office: usize,
        row: &mut u8,
        col: &mut u8,
        skip: u8,
        kind: u8,
    ) -> Result<(), String>;
    fn nearest(&mut self, city: u8, row: &mut u8, col: &mut u8, kind: u8, flag: bool) -> Result<bool, String>;
}

/// Selecciona una operación o ninguna; la búsqueda física queda en `Placement`.
pub fn select<P: Placement>(
    scores: &DemandScores,
    ranking: &BuildRanking,
    towns: &[Town],
    offices: &[OfficeSelect],
    merchants: &[crate::ai::reputation::Merchant],
    day: u8,
    rng: &mut crate::rng::CrtRand,
    placement: &mut P,
) -> Result<Option<crate::pump::Operation>, String> {
    if ranking.town_order.len() != towns.len()
        || ranking.town_scores.len() != towns.len()
        || ranking.best.iter().any(|&v| v >= 21)
    {
        return Err("orden de construcción IA inválido".into());
    }
    let mut random = *rng;
    let try_site = |city: u8,
                    owner: u8,
                    office: Option<usize>,
                    kind: u8,
                    random: &mut crate::rng::CrtRand,
                    placement: &mut P|
     -> Result<Option<crate::pump::Operation>, String> {
        let town = &towns[usize::from(city)];
        let mut row = town.u8(0x800).wrapping_add((random.rand() as u8) & 3);
        let mut col = town.u8(0x801).wrapping_add((random.rand() as u8) & 3);
        if let Some(office) = office {
            placement.near_office(city, office, &mut row, &mut col, day & 3, kind)?
        }
        if placement.nearest(city, &mut row, &mut col, kind, town.u32(0x2c8) & 0x10 != 0)? {
            Ok(Some(crate::pump::Operation([
                0x22,
                u32::from_le_bytes([owner, city, row, col]),
                u32::from(kind),
                0,
                0,
            ])))
        } else {
            Ok(None)
        }
    };
    if ranking.facilities[ranking.best[0] as usize] < scores.fish {
        for &city in &ranking.town_order {
            let town = towns.get(usize::from(city)).ok_or("ciudad de construcción ausente")?;
            if ranking.town_scores[usize::from(city)] < 1 {
                break;
            }
            if town.u32(0x2cc) > 899 {
                let mut at = usize::from(town.u16(0x784));
                let mut seen = vec![false; offices.len()];
                let mut office = None;
                while at < offices.len() {
                    if seen[at] {
                        return Err("oficinas cíclicas".into());
                    }
                    seen[at] = true;
                    if offices[at].owner == u16::from(city) {
                        office = Some(at);
                        break;
                    }
                    at = usize::from(offices[at].next)
                }
                if let Some(op) = try_site(city, city, office, 5, &mut random, placement)? {
                    *rng = random;
                    return Ok(Some(op));
                }
            }
        }
    }
    for &kind in &ranking.best {
        let kind = kind as usize;
        if kind == 0 || ranking.facilities[kind] <= 0 {
            break;
        }
        let ware = usize::from(WARE_FOR_FACILITY[kind]);
        for &city in &ranking.town_order {
            let town = towns.get(usize::from(city)).ok_or("ciudad de construcción ausente")?;
            if ranking.town_scores[usize::from(city)] < 1 {
                break;
            }
            if town.u16(0x848 + kind * 16) < 1000 {
                continue;
            }
            let mut chain = Vec::new();
            let mut at = usize::from(town.u16(0x784));
            let mut seen = vec![false; offices.len()];
            while at < offices.len() {
                if seen[at] {
                    return Err("oficinas cíclicas".into());
                }
                seen[at] = true;
                chain.push(at);
                at = usize::from(offices[at].next)
            }
            if ware < 24 && town.i32(4 + ware * 4) < town.i32(0x4f0 + ware * 16) {
                chain.clear()
            }
            for pass in 0..2 {
                if pass == 1 && chain.is_empty() {
                    let mut at = usize::from(town.u16(0x784));
                    let mut seen = vec![false; offices.len()];
                    while at < offices.len() {
                        if seen[at] {
                            return Err("oficinas cíclicas".into());
                        }
                        seen[at] = true;
                        chain.push(at);
                        at = usize::from(offices[at].next)
                    }
                }
                for &index in &chain {
                    let office = &offices[index];
                    if ware < 24 && office.target[ware] > 0 && office.stock[ware].wrapping_mul(2) < office.target[ware]
                    {
                        continue;
                    }
                    let merchant = merchants
                        .get(usize::from(office.owner))
                        .ok_or("comerciante de oficina ausente")?;
                    if merchant.bytes.len() < 10 {
                        return Err("comerciante de oficina truncado".into());
                    }
                    let money = merchant.i32(0);
                    let control = merchant.u16(8) & 7;
                    if if pass == 0 {
                        money >= 40000 && control == 1
                    } else {
                        money > 40000 && control > 1
                    } {
                        if let Some(mut op) = try_site(
                            city,
                            office.owner as u8,
                            Some(index),
                            kind as u8,
                            &mut random,
                            placement,
                        )? {
                            op.0[1] = (op.0[1] & !0xff00) | (u32::from(town.u8(0x2c0)) << 8);
                            *rng = random;
                            return Ok(Some(op));
                        }
                    }
                }
            }
        }
    }
    *rng = random;
    Ok(None)
}
