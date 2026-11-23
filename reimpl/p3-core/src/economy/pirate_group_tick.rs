//! 0x00514000: actualización diaria de una cuadrilla pirata.
//! Memoria y callbacks son los del anfitrión; no se supone que reparar,
//! destruir, crear barcos o encolar una operación sean efectos vacíos.
use super::pirate_small::ship_set_pirate_band_index;
use crate::flatmem::Mem;

pub trait PirateGroupTickHost {
    fn remove_ship(&mut self, m: &mut dyn Mem, ship: u16) -> Result<(), String>;
    fn dissolve(&mut self, m: &mut dyn Mem, band: u32) -> Result<(), String>;
    fn destroy_band(&mut self, m: &mut dyn Mem, band: u32) -> Result<(), String>;
    fn create_ship(&mut self, m: &mut dyn Mem, band: u32, kind: u32, a: u32, size: u32) -> Result<u32, String>;
    fn upgrade(&mut self, m: &mut dyn Mem, ship: u32, level: u32) -> Result<(), String>;
    fn crew(&mut self, m: &mut dyn Mem, ship: u32) -> Result<u16, String>;
    fn create_captain(&mut self, m: &mut dyn Mem) -> Result<u16, String>;
    fn enqueue(&mut self, m: &mut dyn Mem, record: [u32; 5]) -> Result<(), String>;
}
fn ship(m: &mut dyn Mem, id: u16) -> u32 {
    m.r32(0x6dd7a4).wrapping_add(u32::from(id) * 0x180)
}
fn bounded(count: &mut u32) -> Result<(), String> {
    *count += 1;
    if *count > 65536 {
        Err("cadena pirata cíclica".into())
    } else {
        Ok(())
    }
}
/// El llamante debe proporcionar una copia transaccional de memoria y del
/// anfitrión; si falta una dependencia, no debe publicar efectos parciales.
pub fn tick(m: &mut dyn Mem, host: &mut dyn PirateGroupTickHost, band: u32, index: u32) -> Result<(), String> {
    let mut captain_count = 0i32;
    let mut repair_priority = false;
    let mut candidate = u32::MAX;
    if m.r8(band + 0x15) == 1 {
        let mut steps = 0;
        while m.r16(band + 0x12) < m.r16(0x6dd894) {
            bounded(&mut steps)?;
            let i = m.r16(band + 0x12);
            let s = ship(m, i);
            let n = m.r16(s + 6);
            m.w16(band + 0x12, n);
            host.remove_ship(m, i)?;
        }
        let mut at = m.r16(band + 0xc);
        let mut steps = 0;
        while at < m.r16(0x6dd894) {
            bounded(&mut steps)?;
            let s = ship(m, at);
            let cv = m.r16(s + 8);
            let cap = m.r16(0x6dd896);
            let mut active = false;
            let mut current = m.r16(0x6dd88c);
            let mut loops = 0;
            if cv < cap {
                while current < cap {
                    bounded(&mut loops)?;
                    if current == cv {
                        active = true;
                        break;
                    }
                    let ptr = m.r32(0x6dd7a8).wrapping_add(u32::from(current) * 0x3c);
                    current = m.r16(ptr + 8);
                }
            }
            if !active {
                m.w32(s + 0x18, 0)
            }
            at = m.r16(s + 4);
        }
        if m.r16(band + 0xc) >= m.r16(0x6dd894) {
            host.dissolve(m, band)?;
            let slot = 0x6dd7ac + index.wrapping_mul(4);
            if m.r32(slot) == band {
                host.destroy_band(m, band)?;
                m.w32(slot, 0)
            }
        }
        return Ok(());
    }
    let mut quality = 0i32;
    let mut steps = 0;
    let mut current = m.r16(band + 0xc);
    while current < m.r16(0x6dd894) {
        bounded(&mut steps)?;
        let s = ship(m, current);
        ship_set_pirate_band_index(m, s, index);
        if u16::from(m.r8(s + 0x15d)) >= m.r16(0x6de4aa) {
            quality += 1
        }
        if m.r16(s + 0x42) < m.r16(0x6dd892) {
            captain_count += 1
        }
        current = m.r16(s + 4)
    }
    if m.r16(band + 0x10) < m.r16(0x6dd896) {
        return Ok(());
    }
    let mut at = 0u16;
    let mut steps = 0;
    while at < m.r16(band + 8) {
        bounded(&mut steps)?;
        let e = m.r32(band + 4).wrapping_add(u32::from(at) * 8);
        let age = m.r8(e + 6);
        m.w8(e + 6, age.saturating_sub(4));
        at = m.r16(e + 4)
    }
    if m.r16(band + 0x12) >= m.r16(0x6dd894) {
        if m.r8(0x6de4a4) & 3 == 0 {
            let s = host.create_ship(m, band, 2, 0, 4000)?;
            let hp = m.r32(s + 0x14);
            m.w32(s + 0x18, hp);
            host.upgrade(m, s, 2)?;
            let crew = host.crew(m, s)?;
            m.w16(s + 0x40, crew)
        }
    } else {
        let mut id = m.r16(band + 0x12);
        let mut steps = 0;
        while id < m.r16(0x6dd894) {
            bounded(&mut steps)?;
            let s = ship(m, id);
            let maximum = m.r32(s + 0x14);
            let hp = m.r32(s + 0x18);
            if (maximum.wrapping_sub(hp) as i32) < 1000 {
                m.w32(s + 0x18, maximum);
                if u16::from(m.r8(s + 0x15d)) < m.r16(0x6de4aa) {
                    repair_priority = true;
                    candidate = u32::from(id)
                } else if m.r16(s + 0x42) < m.r16(0x6dd892) && !repair_priority {
                    candidate = u32::from(id)
                }
            } else {
                m.w32(s + 0x18, hp.wrapping_add(1000))
            }
            if u16::from(m.r8(s + 0x15d)) >= m.r16(0x6de4aa) {
                quality -= 1
            }
            id = m.r16(s + 6);
        }
    }
    let level = m.r8(band + 0x14);
    let threshold = i32::from(m.r8(0x673658 + u32::from(level))) + 3;
    if quality > threshold {
        let mut link = band + 0x12;
        let mut steps = 0;
        while m.r16(link) < m.r16(0x6dd894) {
            bounded(&mut steps)?;
            let id = m.r16(link);
            let s = ship(m, id);
            if m.r16(s + 0x42) >= m.r16(0x6dd892) && u16::from(m.r8(s + 0x15d)) >= m.r16(0x6de4aa) {
                let next = m.r16(s + 6);
                m.w16(link, next);
                m.w16(s + 6, 0xffff);
                host.remove_ship(m, id)?;
                quality = -1;
                break;
            }
            link = s + 6;
        }
    }
    if captain_count < 1 {
        let id = m.r16(band + 0x12);
        if id < m.r16(0x6dd894) {
            let s = ship(m, id);
            if m.r16(s + 0x42) >= m.r16(0x6dd892) {
                let c = host.create_captain(m)?;
                m.w16(s + 0x42, c)
            }
        }
    }
    if candidate < u32::from(m.r16(0x6dd894))
        && (quality <= 0 || repair_priority)
        && m.r32(0x6df31c) == 0
        && m.r8(0x6de52a) == 0
    {
        host.enqueue(
            m,
            [
                0xb5,
                index,
                u32::from(level) * 100 + 120,
                if repair_priority { 1 } else { u32::from(level) + 1 },
                0,
            ],
        )?;
    }
    Ok(())
}

/// `ship_get_target_crew` (0x005185C0), con comparación final sin signo.
pub fn target_crew(ship: &[u8]) -> Option<u16> {
    if ship.len() < 0x14 {
        return None;
    }
    let capacity = i32::from_le_bytes(ship[0x10..0x14].try_into().ok()?);
    let level = u32::from(ship[0xf]);
    let factor = [3u32, 5, 8, 10][usize::from(ship[0xe] & 3)];
    let result = ((capacity / 2000) as u32).wrapping_add(level.wrapping_mul(factor));
    Some(result.max(20) as u16)
}
