//! Funciones pequeñas de la cuadrilla pirata y de sus barcos, traducidas 1:1 sobre la memoria del original ([`Mem`]).
use crate::flatmem::Mem;

pub trait HideoutHost {
    /// `0x004F9670(escondite)`: tamaño serializado.
    fn hideout_size(&mut self, hideout: u32) -> u32;
    /// `0x004F9930(escondite)`.
    fn hideout_clear(&mut self, hideout: u32);
    /// `0x004F9650(escondite)`: destructor.
    fn hideout_destroy(&mut self, hideout: u32);
    /// `operator delete` (0x0064F7E2).
    fn free(&mut self, addr: u32);
    /// `0x00515F00(grupo, índice de barco)`.
    fn group_remove(&mut self, group: u32, ship: u32);
}

/// `pirate_band_init` (0x00513720).
pub fn band_init(m: &mut dyn Mem, band: u32) {
    m.w32(band, 0);
    m.w32(band + 4, 0);
    m.w16(band + 8, 0);
    m.w16(band + 0xa, 0);
    m.w16(band + 0xc, 0xffff);
    m.w8(band + 0xe, 0xff);
    m.w8(band + 0xf, 0xff);
    m.w16(band + 0x10, 0xffff);
    m.w16(band + 0x12, 0xffff);
    m.w8(band + 0x14, 0xff);
    m.w8(band + 0x15, 0);
    m.w16(band + 0x16, 0);
}

/// `pirate_band_get_serialized_size` (0x005137B0).
pub fn band_serialized_size(m: &mut dyn Mem, host: &mut dyn HideoutHost, band: u32) -> u32 {
    let mut size = 0x11u32;
    let hideout = m.r32(band);
    if hideout != 0 {
        size = host.hideout_size(hideout).wrapping_add(0x11);
    }
    size.wrapping_add(u32::from(m.r16(band + 8)) * 8)
}

/// `pirate_band_free_hideout_data` (0x00513760) y su duplicado idéntico `0x00513BA0`.
pub fn band_free_hideout_data(m: &mut dyn Mem, host: &mut dyn HideoutHost, band: u32) {
    let hideout = m.r32(band);
    if hideout != 0 {
        host.hideout_clear(hideout);
        let h = m.r32(band);
        if h != 0 {
            host.hideout_destroy(h);
            host.free(h);
        }
        m.w32(band, 0);
    }
    let p = m.r32(band + 4);
    if p != 0 {
        host.free(p);
        m.w32(band + 4, 0);
    }
    m.w16(band + 8, 0);
    m.w16(band + 0xa, 0);
    m.w16(band + 0x16, 0);
}

/// `pirate_band_is_away_or_disbanded` (0x00515D00).
pub fn band_is_away_or_disbanded(m: &mut dyn Mem, band: u32) -> bool {
    m.r8(band + 0x15) == 1 || m.r16(band + 0x10) < m.r16(0x6DD896)
}

/// `pirate_band_get_piece_plunder_value` (0x00515D30).
pub fn band_piece_plunder_value(m: &mut dyn Mem, band: u32, idx: u32) -> u32 {
    if idx >= u32::from(m.r16(band + 8)) {
        return 0;
    }
    let piece = m.r32(band + 4).wrapping_add(idx.wrapping_mul(8));
    let age = u32::from(m.r8(piece + 6));
    if age >= 0xc8 {
        return 0;
    }
    let base: u32 = match m.r8(piece + 3) {
        0x30 => 0x2fda0,
        0x31 => 0x77a10,
        _ => 0,
    };
    let t = (0xc8 - age).wrapping_mul(base);
    ((u64::from(t) * 0x51eb_851f) >> 32) as u32 >> 6
}

/// `point_in_pirate_zone_box` (0x0050E340): tres cajas de 8 bytes desde `0x00673564` (x mín., y mín., x máx., y máx.) con signo.
pub fn point_in_pirate_zone_box(m: &mut dyn Mem, x: u16, y: u16) -> bool {
    let (x, y) = (x as i16, y as i16);
    let mut p = 0x673568u32;
    while p < 0x673580 {
        let (x0, y0, x1, y1) = (
            m.r16(p - 4) as i16,
            m.r16(p - 2) as i16,
            m.r16(p) as i16,
            m.r16(p + 2) as i16,
        );
        if !(x < x0 || x > x1 || y < y0 || y > y1) {
            return true;
        }
        p += 8;
    }
    false
}

/// `ship_set_pirate_patrol_town` (0x0051A820).
pub fn ship_set_pirate_patrol_town(m: &mut dyn Mem, ship: u32, town: u32) {
    if m.r32(0x6DE4B0) > town && u16::from(m.r8(ship)) >= m.r16(0x6DE4AA) {
        m.w8(ship + 0x159, town as u8);
    } else {
        m.w8(ship + 0x159, 0xff);
    }
}

/// `ship_set_pirate_band_index` (0x0051A4C0).
pub fn ship_set_pirate_band_index(m: &mut dyn Mem, ship: u32, band: u32) {
    if band >= 5 || m.r32(0x6DD7AC + band * 4) == 0 {
        return;
    }
    m.w8(ship + 0x158, band as u8);
}

/// `remove_ship_from_pirate_groups` (0x005066F0): quita el barco de los cinco grupos.
pub fn remove_ship_from_pirate_groups(m: &mut dyn Mem, host: &mut dyn HideoutHost, this: u32, ship: u32) {
    for i in 0..5 {
        let g = m.r32(this + 0xc + 4 * i);
        if g != 0 {
            host.group_remove(g, ship);
        }
    }
}

/// `pirate_band_append_ship` (0x00515EA0): añade el barco al final de la cadena (enlace `barco+4`).
pub fn band_append_ship(m: &mut dyn Mem, band: u32, idx: u32) {
    let s = m.r16(0x6DD894);
    if (idx as u16) >= s {
        return;
    }
    let mut at = band + 0xc;
    let mut cur = m.r16(at);
    if cur < s {
        loop {
            let ship = m.r32(0x6DD7A4).wrapping_add(u32::from(cur) * 0x180);
            at = ship + 4;
            cur = m.r16(at);
            if cur >= s {
                break;
            }
        }
    }
    m.w16(at, idx as u16);
    let last = m.r32(0x6DD7A4).wrapping_add((idx & 0xffff) * 0x180);
    m.w16(last + 4, 0xffff);
}
