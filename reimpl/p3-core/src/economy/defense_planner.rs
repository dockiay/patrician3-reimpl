//! Decisión de defensas de town_daily_construction_planner, fase siete.
use crate::{
    buildings::Site,
    town::{off, Town},
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefenseAction {
    Upgrade,
    PlaceGate,
    QueuePitch,
    QueuePitchShoot,
}
fn contains(site: &Site, head: u16, count: u16, pred: impl Fn(u8) -> bool) -> Result<bool, &'static str> {
    let mut seen = vec![false; usize::from(count)];
    let mut at = head;
    while at < count {
        let i = usize::from(at);
        if seen[i] || i * 8 + 8 > site.records.len() {
            return Err("cadena de defensas inválida");
        }
        seen[i] = true;
        if pred(site.records[i * 8 + 3]) {
            return Ok(true);
        }
        at = site.next(i)
    }
    Ok(false)
}
/// `local_human` indica si un comerciante humano tiene aquí su ciudad natal.
pub fn plan(t: &Town, site: &Site, day: u16, local_human: bool) -> Result<Option<DefenseAction>, &'static str> {
    if day & 7 != 7 || day & 0x78 != u16::from(t.index() & 15) * 8 {
        return Ok(None);
    }
    let mut phase = t.i32(0x2e0) % 7;
    if t.flags() & 0x40 != 0 && local_human {
        phase = 0
    }
    let count = t.u16(off::BUILDING_RECORD_COUNT);
    let building = t.u16(off::CONSTRUCTION_HEAD);
    let defense = t.u16(off::DEFENSE_HEAD);
    let result = match phase {
        0 | 1 => {
            if t.flags() & 0x10 == 0 && !contains(site, building, count, |v| v == 0x32)? {
                Some(DefenseAction::Upgrade)
            } else {
                None
            }
        }
        2 => {
            if t.flags() & 0x10 == 0 && contains(site, defense, count, |v| v == 0x32)? {
                Some(DefenseAction::Upgrade)
            } else {
                None
            }
        }
        3 | 4 => {
            if !contains(site, building, count, |v| v == 0x30)? {
                Some(DefenseAction::PlaceGate)
            } else {
                None
            }
        }
        5 => {
            if !contains(site, defense, count, |v| v == 0x31)? {
                Some(DefenseAction::QueuePitch)
            } else {
                None
            }
        }
        6 => {
            if contains(site, defense, count, |v| v >= 0x37)? {
                Some(DefenseAction::QueuePitchShoot)
            } else {
                None
            }
        }
        _ => None,
    };
    Ok(result)
}
