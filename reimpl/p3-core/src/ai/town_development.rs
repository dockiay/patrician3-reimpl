//! Hitos de una ciudad fundada: tarea 0x2F (`0x004EA0D4`).
use super::reputation::Merchant;
use crate::{
    buildings::Site,
    scheduler::TaskRecord,
    town::{off, Town},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevelopmentEffect {
    None,
    CreateOffice { merchant: u8, city: u8 },
    AppointMayor { merchant: u8, city: u8 },
    PlaceLender { city: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DevelopmentResult {
    pub keep: bool,
    pub effect: DevelopmentEffect,
}

/// Las llamadas externas se devuelven explícitamente; el llamante debe aplicarlas
/// antes de publicar los cambios a la ciudad y la tarea.
pub fn tick(
    task: &mut TaskRecord,
    towns: &mut [Town],
    sites: &[Site],
    merchants: &[Merchant],
    merchant_count: u16,
    network: bool,
    tick: u32,
) -> Result<DevelopmentResult, &'static str> {
    let mut staged = *task;
    let city = staged.u32_at(0xc) as usize;
    if city >= towns.len() {
        return Ok(DevelopmentResult {
            keep: false,
            effect: DevelopmentEffect::None,
        });
    }
    let mut town = towns[city].clone();
    let flags = town.u32(off::MORE_FLAGS);
    let mut progress = staged.u32_at(0x10);
    let mut effect = DevelopmentEffect::None;
    let delta;
    if progress & 1 == 0 {
        if flags & 1 == 0 {
            delta = 0x1500;
        } else {
            let mayor = town.u8(0x6f1);
            if u16::from(mayor) < merchant_count {
                let m = merchants.get(usize::from(mayor)).ok_or("alcalde ausente")?;
                if m.u16(8) != 0 {
                    effect = DevelopmentEffect::CreateOffice {
                        merchant: mayor,
                        city: city as u8,
                    };
                }
                progress |= 1;
                staged.set_u32_at(0x10, progress);
                staged.set_u32_at(0x14, tick);
                delta = 0x100;
            } else {
                if !network {
                    if merchants.len() < usize::from(merchant_count) {
                        return Err("comerciantes incompletos");
                    }
                    let mut best = 0;
                    let mut chosen = None;
                    for i in (0..usize::from(merchant_count)).rev() {
                        let m = &merchants[i];
                        if m.u16(8) == 0 || towns.iter().any(|t| usize::from(t.u8(0x6f1)) == i) {
                            continue;
                        }
                        let home = usize::from(m.bytes[0x19]);
                        let rank = usize::from(m.bytes[0x39c + home]);
                        if rank > best {
                            best = rank;
                            chosen = Some(i as u8)
                        }
                    }
                    if let Some(merchant) = chosen {
                        effect = DevelopmentEffect::AppointMayor {
                            merchant,
                            city: city as u8,
                        };
                    }
                }
                delta = 0xe00;
            }
        }
    } else {
        if progress & 0x80 != 0 && town.u16(0x854) == 0 {
            town.set_u16(0x854, 1);
            town.set_u32(0x2e0, town.u32(0x2e0).wrapping_add(4));
            town.set_u32(0x2d4, town.u32(0x2d4).wrapping_add(4));
        }
        if progress & 0x100 == 0 {
            if flags & 0x100 != 0 {
                progress |= 0x100;
                staged.set_u32_at(0x10, progress);
                delta = 0x100;
            } else {
                let population = town.i32(0x2d4);
                if population < 500 {
                    delta = 0x1c00;
                } else if population < 1000 {
                    delta = (0x88dbi32.wrapping_sub(population.wrapping_mul(35))) as u32;
                } else {
                    let site = sites.get(city).ok_or("plano de ciudad ausente")?;
                    let count = usize::from(town.u16(off::BUILDING_RECORD_COUNT));
                    if site.records.len() < count * 8 {
                        return Err("registros de ciudad truncados");
                    }
                    let wall = usize::from(town.u16(off::WALL_HEAD));
                    if wall < count && site.rec(wall, 2) < 0xfe {
                        let mut head = usize::from(town.u16(off::DEFENSE_HEAD));
                        let mut seen = vec![false; count];
                        while head < count {
                            if seen[head] {
                                return Err("ciclo en defensas");
                            };
                            seen[head] = true;
                            head = usize::from(site.next(head));
                        }
                    }
                    let level = site.wall_expansion_level(&town);
                    if level == 1 || level == 2 {
                        town.set_u32(off::MORE_FLAGS, flags | 0x100);
                        delta = 5;
                    } else if level != 0 {
                        delta = 0x1c00;
                    } else {
                        let mut head = usize::from(town.u16(off::CONSTRUCTION_HEAD));
                        let mut seen = vec![false; count];
                        let mut waiting = false;
                        while head < count {
                            if seen[head] {
                                return Err("ciclo en obras");
                            };
                            seen[head] = true;
                            if site.rec(head, 3) >= 0x36 && site.rec(head, 2) == 0xff {
                                waiting = true;
                                break;
                            }
                            head = usize::from(site.next(head));
                        }
                        if waiting {
                            delta = 0xe00;
                        } else {
                            town.set_u32(off::MORE_FLAGS, flags | 0x100);
                            delta = 0x1c00;
                        }
                    }
                }
            }
        } else if progress & 0x800 == 0 {
            if flags & 0x800 != 0 {
                progress |= 0x800;
                staged.set_u32_at(0x10, progress);
                staged.set_u32_at(0x14, tick);
                delta = 0x100;
            } else if town.u32(0x2e0).wrapping_mul(3).wrapping_add(town.u32(0x2dc)) > 1500 {
                if !network {
                    effect = DevelopmentEffect::PlaceLender { city: city as u8 };
                }
                delta = 0x700;
            } else {
                delta = 0x1c00;
            }
        } else {
            delta = 0x1500;
        }
    }
    staged.due = staged.due.wrapping_add(delta);
    towns[city] = town;
    *task = staged;
    Ok(DevelopmentResult { keep: true, effect })
}
