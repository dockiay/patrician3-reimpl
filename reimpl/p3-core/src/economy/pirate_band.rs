//! Cuadrilla pirata (`PirateBand`): `pirate_band_alloc_slot` (0x00513BF0) y `pirate_band_setup` (0x00513D80).
//! Traducción 1:1 sobre la memoria del original ([`Mem`]); el asignador y el alta de barcos pasan por el anfitrión.
use crate::flatmem::Mem;

pub trait BandHost {
    /// `operator new` (0x0064F7B9): dirección del bloque (sin inicializar).
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    /// `operator delete` (0x0064F7E2).
    fn free(&mut self, addr: u32);
    /// `0x005159C0(ciudad)`: puerto libre cercano.
    fn free_port(&mut self, town: u32) -> u8;
    /// `0x00516170(banda, a, b, c)`: crea una pieza/barco y devuelve su dirección.
    fn create_ship(&mut self, m: &mut dyn Mem, band: u32, a: u32, b: u32, c: u32) -> u32;
}

fn memcpy(m: &mut dyn Mem, dst: u32, src: u32, n: u32) {
    for i in 0..n {
        let v = m.r8(src.wrapping_add(i));
        m.w8(dst.wrapping_add(i), v);
    }
}

/// `pirate_band_alloc_slot`: toma una ranura de 8 bytes de la lista libre (creciendo de 32 en 32) y la enlaza al final de la cadena.
pub fn alloc_slot(m: &mut dyn Mem, host: &mut dyn BandHost, band: u32) -> u32 {
    let mut head = m.r16(band + 0xa);
    let cap = m.r16(band + 8);
    if head == cap {
        let cap = cap.wrapping_add(0x20);
        m.w16(band + 8, cap);
        let new = host.alloc(m, u32::from(cap) << 3);
        let used = m.r16(band + 0xa);
        if used != 0 {
            let old = m.r32(band + 4);
            memcpy(m, new, old, u32::from(used) << 3);
        }
        while head < m.r16(band + 8) {
            let e = new.wrapping_add(u32::from(head) << 3);
            for k in 0..8 {
                m.w8(e + k, 0xff);
            }
            head = head.wrapping_add(1);
            m.w16(e + 4, head);
        }
        let old = m.r32(band + 4);
        host.free(old);
        head = m.r16(band + 0xa);
        m.w32(band + 4, new);
    }
    let table = m.r32(band + 4);
    let off = u32::from(head) << 3;
    let entry = table.wrapping_add(off);
    let next = m.r16(entry + 4);
    m.w16(band + 0xa, next);
    m.w16(entry + 4, 0xffff);
    if head != 0 {
        let cap = m.r16(band + 8);
        let mut last = 0u32;
        let mut link = m.r16(table + 4);
        if link < cap {
            loop {
                last = u32::from(link);
                link = m.r16(table.wrapping_add(u32::from(link) << 3) + 4);
                if link >= cap {
                    break;
                }
            }
        }
        m.w16(table.wrapping_add(last << 3) + 4, head);
        m.w8(entry + 3, 0x1b);
    } else {
        m.w8(entry + 3, 0x24);
    }
    m.w8(entry + 7, 0);
    m.w8(entry, 0xff);
    m.w8(entry + 1, 0xff);
    m.w8(entry + 2, 0xff);
    m.w8(entry + 6, 0);
    entry
}

/// `pirate_band_setup(ciudad, nivel, semilla)`; el cierre consulta el byte
/// de orientación marítima `0x006DDBC1 + 52·puerto` de la ciudad elegida.
pub fn setup(
    m: &mut dyn Mem,
    host: &mut dyn BandHost,
    band: u32,
    town: u32,
    level: u32,
    seed: u32,
    kind_of_port: &dyn Fn(u8) -> u8,
    bytes: (u8, u8),
) {
    let n = level & 3;
    m.w8(band + 0x14, n as u8);
    let port = host.free_port(town);
    m.w8(band + 0xe, port);
    let k = kind_of_port(port);
    if (3..=5).contains(&k) {
        m.w8(band + 0xf, 1);
    } else {
        let r = (seed % 3) as u8;
        m.w8(band + 0xf, if r == 0 { 0 } else { r + 1 });
    }
    m.w16(band + 0xa, 0);
    m.w16(band + 8, 0);
    let table = m.r32(band + 4);
    if table != 0 {
        host.free(table);
        m.w32(band + 4, 0);
    }
    alloc_slot(m, host, band);
    for _ in 0..n {
        let e = alloc_slot(m, host, band);
        m.w8(e + 3, 0x30);
    }
    for i in 0..3u32 {
        let e = alloc_slot(m, host, band);
        m.w8(e + 3, 0x1b);
        m.w8(e + 7, if i & 1 != 0 { bytes.0 } else { bytes.1 });
    }
    m.w16(band + 0x16, 0x3e8);
    let mut s = seed;
    let n = u32::from(m.r8(band + 0x14));
    for i in 0..=n {
        let size = n * 5 * 8 + 0x32;
        let ship = if i == 0 {
            host.create_ship(m, band, 1, 1, size)
        } else {
            let c = ((s >> 1) & 1) + (s & 1) + 1;
            let r = host.create_ship(m, band, c, 0, size);
            s >>= 2;
            r
        };
        m.w32(ship + 0x18, 1);
    }
    m.w16(band + 0x10, 0xffff);
    m.w8(band + 0x15, 0);
}

pub trait DissolveHost {
    /// `0x004F9930(plan)` y `0x004F9650(plan)`: reinicio y liberación de los datos del plan.
    fn plan_reset(&mut self, plan: u32);
    fn plan_free_data(&mut self, plan: u32);
    /// `operator delete`.
    fn free(&mut self, addr: u32);
    /// `0x00507FB0(barco)`: `remove_ship`.
    fn remove_ship(&mut self, ship: u32);
    /// `0x0050A3D0(convoy)`: `release_convoy`.
    fn release_convoy(&mut self, convoy: u32);
}

const SHIPS: u32 = 0x6DD7A4;
const SHIP_COUNT: u32 = 0x6DD894;
const CONVOY_COUNT: u32 = 0x6DD896;

/// `pirate_band_dissolve` (0x00513EC0): libera plan y tabla, marca los barcos de la cuadrilla, suelta el convoy, retira los barcos
/// y borra la referencia de los comerciantes a la cuadrilla.
pub fn dissolve(m: &mut dyn Mem, host: &mut dyn DissolveHost, band: u32) {
    let plan = m.r32(band);
    if plan != 0 {
        host.plan_reset(plan);
        let p = m.r32(band);
        if p != 0 {
            host.plan_free_data(p);
            host.free(p);
        }
        m.w32(band, 0);
    }
    let table = m.r32(band + 4);
    if table != 0 {
        host.free(table);
        m.w32(band + 4, 0);
    }
    m.w16(band + 8, 0);
    m.w16(band + 0xa, 0);
    m.w16(band + 0x16, 0);
    let ships = |m: &mut dyn Mem, i: u32| m.r32(SHIPS).wrapping_add(i.wrapping_mul(0x180));
    let mut i = u32::from(m.r16(band + 0xc));
    while i < u32::from(m.r16(SHIP_COUNT)) {
        let s = ships(m, i);
        i = u32::from(m.r16(s + 4));
        let f = m.r16(s + 0x3c) | 0x100;
        m.w16(s + 0x3c, f);
        m.w32(s + 0x18, 0);
    }
    let convoy = m.r16(band + 0x10);
    if convoy < m.r16(CONVOY_COUNT) {
        host.release_convoy(u32::from(convoy));
        m.w16(band + 0x10, 0xffff);
    }
    loop {
        let i = u32::from(m.r16(band + 0x12));
        if i >= m.r16(SHIP_COUNT) as u32 {
            break;
        }
        let s = ships(m, i);
        let next = m.r16(s + 6);
        m.w16(band + 0x12, next);
        m.w16(s + 6, 0xffff);
        host.remove_ship(i);
    }
    let count = u32::from(m.r16(0x6DE4AA));
    for k in 0..count {
        let merchant = m.r32(0x6DE4A0 + 0x78).wrapping_add(k.wrapping_mul(0x650));
        if m.r8(merchant + 0xe1) == m.r8(band + 0xe) {
            m.w8(merchant + 0xe1, 0xff);
            m.w8(merchant + 0xe0, 0xff);
        }
    }
    m.w8(band + 0xe, 0xff);
    m.w8(band + 0xf, 0xff);
    m.w8(band + 0x14, 0xff);
}

pub trait BandLoadHost {
    fn plan_reset(&mut self, plan: u32);
    fn plan_free_data(&mut self, plan: u32);
    fn free(&mut self, addr: u32);
    /// `operator new`; la memoria devuelta se declara al `Mem`.
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    /// `0x004F95C0`: construye un plan vacío en el bloque.
    fn plan_construct(&mut self, plan: u32) -> u32;
    /// `0x004F9770(plan, cursor, versión)`: lee el plan y devuelve el cursor siguiente.
    fn plan_deserialize(&mut self, m: &mut dyn Mem, plan: u32, cursor: u32, version: u32) -> u32;
}

/// `pirate_band_deserialize` (0x005138B0) para guardados de versión 0x7F o posterior; devuelve el cursor siguiente.
/// (Las versiones anteriores pasan por `pirate_band_deserialize_legacy`, 0x00513A40, no portada.)
pub fn deserialize(m: &mut dyn Mem, host: &mut dyn BandLoadHost, band: u32, cursor: u32, version: u32) -> u32 {
    assert!(version >= 0x7f, "versiones anteriores: 0x00513A40");
    let mut s = cursor;
    for (off, len) in [(8u32, 2u32), (0xa, 2), (0xc, 2)] {
        for k in 0..len {
            let v = m.r8(s + k);
            m.w8(band + off + k, v);
        }
        s += 2;
    }
    let v = m.r8(s);
    s += 1;
    m.w8(band + 0xe, v);
    let v = m.r8(s);
    s += 1;
    m.w8(band + 0xf, v);
    for off in [0x10u32, 0x12] {
        for k in 0..2 {
            let v = m.r8(s + k);
            m.w8(band + off + k, v);
        }
        s += 2;
    }
    let v = m.r8(s);
    s += 1;
    m.w8(band + 0x14, v);
    let flag = m.r8(s);
    s += 1;
    m.w8(band + 0x15, u8::from(flag != 0));
    for k in 0..2 {
        let v = m.r8(s + k);
        m.w8(band + 0x16 + k, v);
    }
    let has_plan = m.r8(s + 2);
    s += 3;
    if has_plan != 0 {
        let old = m.r32(band);
        if old != 0 {
            host.plan_reset(old);
            let p = m.r32(band);
            if p != 0 {
                host.plan_free_data(p);
                host.free(p);
            }
        }
        let block = host.alloc(m, 0x6c);
        let plan = if block != 0 { host.plan_construct(block) } else { 0 };
        m.w32(band, plan);
        s = host.plan_deserialize(m, plan, s, version);
    }
    let table = m.r32(band + 4);
    if table != 0 {
        host.free(table);
        m.w32(band + 4, 0);
    }
    let cap = u32::from(m.r16(band + 8));
    if cap > 0 {
        let new = host.alloc(m, cap << 3);
        m.w32(band + 4, new);
        for i in 0..cap << 3 {
            let v = m.r8(s + i);
            m.w8(new + i, v);
        }
        s += cap << 3;
    }
    s
}

/// `pirate_band_deserialize_legacy` (0x00513A40): formato antiguo, con tabla fija de 16 piezas y la bandera del plan como dword.
pub fn deserialize_legacy(m: &mut dyn Mem, host: &mut dyn BandLoadHost, band: u32, cursor: u32, version: u32) -> u32 {
    let old = m.r32(band);
    if old != 0 {
        host.plan_reset(old);
        let p = m.r32(band);
        if p != 0 {
            host.plan_free_data(p);
            host.free(p);
        }
        m.w32(band, 0);
    }
    let table = m.r32(band + 4);
    if table != 0 {
        host.free(table);
        m.w32(band + 4, 0);
    }
    m.w16(band + 8, 0x10);
    m.w16(band + 0xa, 0x10);
    let new = host.alloc(m, 0x80);
    m.w32(band + 4, new);
    let mut s = cursor;
    memcpy(m, new, s, 0x80);
    s += 0x80;
    let has_plan = [m.r8(s), m.r8(s + 1), m.r8(s + 2), m.r8(s + 3)];
    s += 4;
    memcpy(m, band + 0xc, s, 2);
    s += 2;
    memcpy(m, band + 0x16, s, 2);
    let v = m.r8(s + 2);
    m.w8(band + 0xe, v);
    s += 3;
    let v = m.r8(s);
    s += 1;
    m.w8(band + 0xf, v);
    memcpy(m, band + 0x10, s, 2);
    s += 2;
    memcpy(m, band + 0x12, s, 2);
    let v = m.r8(s + 2);
    m.w8(band + 0x14, v);
    s += 3;
    let v = m.r8(s);
    s += 1;
    m.w8(band + 0x15, v);
    if has_plan != [0; 4] {
        let block = host.alloc(m, 0x6c);
        let plan = if block != 0 { host.plan_construct(block) } else { 0 };
        m.w32(band, plan);
        s = host.plan_deserialize(m, plan, s, version);
    }
    s
}
