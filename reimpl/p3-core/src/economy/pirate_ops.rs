//! Operaciones sobre cuadrillas y barcos piratas: `op_create_pirate_band` (0x00542E40) y `op_hand_ship_to_pirate` (0x005386C0).
//! Traducción 1:1 sobre la memoria del original ([`Mem`]).
use super::pirate_small;
use crate::flatmem::Mem;

const SHIPS: u32 = 0x6DD7A4;
const SHIP_COUNT: u32 = 0x6DD894;
const CONVOYS: u32 = 0x6DD7A8;
const BANDS: u32 = 0x6DD7AC;

pub trait OpsHost {
    /// `operator new` (0x0064F7B9).
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    /// `pirate_band_dissolve` (0x00513EC0).
    fn dissolve(&mut self, band: u32);
    /// `pirate_band_setup` (0x00513D80): (cuadrilla, a, b, c).
    fn setup(&mut self, band: u32, a: u32, b: u32, c: u32);
}

/// `op_create_pirate_band`: `this+4` = ranura (0..4), `+8`, `+0xC`, `+0x10` = parámetros de la preparación.
pub fn op_create_pirate_band(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let slot = m.r32(this + 4);
    if slot >= 5 {
        return;
    }
    let existing = m.r32(BANDS + slot * 4);
    if existing == 0 {
        let block = host.alloc(m, 0x18);
        let band = if block != 0 {
            pirate_small::band_init(m, block);
            block
        } else {
            0
        };
        m.w32(BANDS + slot * 4, band);
    } else {
        host.dissolve(existing);
    }
    let (a, b, c) = (m.r32(this + 0xc), m.r32(this + 8), m.r32(this + 0x10));
    let slot2 = m.r32(this + 4);
    let band = m.r32(BANDS + slot2 * 4);
    host.setup(band, a, b, c);
}

/// `op_hand_ship_to_pirate`: `this+4` = barco, `this+8` = bandera (entregar la escuadra / solo marcar).
pub fn op_hand_ship_to_pirate(m: &mut dyn Mem, this: u32) {
    let s = u32::from(m.r16(SHIP_COUNT));
    let idx = m.r32(this + 4);
    if idx >= s {
        return;
    }
    let ship = m.r32(SHIPS).wrapping_add(idx.wrapping_mul(0x180));
    let cidx = m.r16(ship + 8);
    if u32::from(cidx) >= s {
        if m.r16(ship + 0x134) != 0xf {
            return;
        }
        if m.r32(this + 8) != 0 {
            let f = m.r8(ship + 0x3d) | 0x18;
            m.w8(ship + 0x3d, f);
            m.w8(ship + 0x15c, 1);
            m.w8(ship + 0x136, 0);
        } else {
            let f = m.r8(ship + 0x3d) | 0x10;
            m.w8(ship + 0x3d, f);
            m.w8(ship + 0x15c, 0);
        }
        return;
    }
    let conv = m.r32(CONVOYS).wrapping_add(u32::from(cidx) * 0x3c);
    let f = m.r32(conv + 0x18) | 0x1000;
    m.w32(conv + 0x18, f);
    let mark = m.r32(this + 8) != 0;
    if mark {
        if m.r8(conv) != m.r8(ship) || m.r16(conv + 0x12) != 0xf || m.r8(conv + 0x14) & 2 == 0 {
            return;
        }
    }
    let mut cur = u32::from(m.r16(conv + 0xa));
    if cur >= u32::from(m.r16(SHIP_COUNT)) {
        return;
    }
    loop {
        let sh = m.r32(SHIPS).wrapping_add(cur * 0x180);
        let next = u32::from(m.r16(sh + 6));
        let f = m.r16(sh + 0x3c) | 0x1000;
        m.w16(sh + 0x3c, f);
        m.w8(sh + 0x15c, u8::from(mark));
        cur = next;
        if cur >= u32::from(m.r16(SHIP_COUNT)) {
            break;
        }
    }
}

pub trait MissionShipHost {
    /// `spawn_pirate_ship_near_ship` (0x005063C0), thiscall con ECX = 0x6DD7A0: (arg0, arg1) -> AX.
    fn spawn(&mut self, arg0: u32, arg1: u32) -> u32;
    /// `0x0051A820` sobre el barco, con un byte de configuración.
    fn ship_adjust(&mut self, ship: u32, value: u32);
    /// `0x004D8DB0` sobre la tabla `0x6DD73C`: puntero del objeto `index`.
    fn object_at(&mut self, index: u32) -> u32;
}

/// `op_spawn_mission_pirate_ship` (0x0053FB00). `this+4` barco de referencia, `+6/+7` datos, `+8` objeto de misión,
/// `+0xA` ranura, `+0xB` modo, `+0xC` límite.
pub fn op_spawn_mission_pirate_ship(m: &mut dyn Mem, host: &mut dyn MissionShipHost, this: u32) {
    let cx = u32::from(m.r16(this + 4));
    let ship_count = m.r16(SHIP_COUNT);
    if (cx as u16) < ship_count {
        let base = m.r32(SHIPS);
        let kind = m.r16(base.wrapping_add(cx * 0x180).wrapping_add(0x134));
        if kind != 0xf && kind != 0x10 {
            return;
        }
    }
    // ECX conserva la parte alta de `this` (ECX = this al entrar).
    let arg0 = (this & 0xffff_0000) | cx;
    let mode = u32::from(m.r8(this + 0xb));
    let ax = host.spawn(arg0, mode) as u16;
    let mut edi = 0u32;
    if ax < m.r16(SHIP_COUNT) {
        edi = m.r32(SHIPS).wrapping_add(u32::from(ax) * 0x180);
        let t = m.r16(edi + 0x42);
        if t < m.r16(0x6DD892) {
            let tr = m.r32(0x6DD7A0).wrapping_add(u32::from(t) << 4);
            let (a, b) = (m.r8(this + 6), m.r8(this + 7));
            m.w8(tr + 2, a);
            m.w8(tr + 3, b);
        }
        let lim = u32::from(m.r8(this + 0xc));
        if m.r32(0x6DE4B0) > lim {
            host.ship_adjust(edi, lim);
        }
        m.w8(edi + 0x136, 0x80);
    }
    let o = u32::from(m.r16(this + 8));
    if o >= u32::from(m.r16(0x6DD748)) {
        return;
    }
    let obj = host.object_at(o);
    if m.r16(obj + 6) != 0x1b {
        return;
    }
    let slot = u32::from(m.r8(this + 0xa));
    if slot >= u32::from(m.r16(obj + 0x12)) {
        return;
    }
    let cell = m.r32(obj + 0xc).wrapping_add(slot * 4);
    if edi == 0 {
        m.w32(cell, 0xffff_fffe);
        return;
    }
    let a = u32::from(m.r8(this + 6));
    m.w32(cell, a);
    let v = m.r32(cell) << 8;
    m.w32(cell, v);
    let v = m.r32(cell).wrapping_add(u32::from(m.r8(this + 7)));
    m.w32(cell, v);
    let v = m.r32(cell) << 16;
    m.w32(cell, v);
    let v = m.r32(cell).wrapping_add(u32::from(m.r16(edi + 0x42)));
    m.w32(cell, v);
}
