//! `pirate_band_prepare_hideout_and_convoy` (0x00514530): crea los datos del escondite de la cuadrilla, coloca sus piezas y
//! prepara el barco y el convoy. Traducción 1:1 sobre la memoria del original ([`Mem`]).
use crate::flatmem::Mem;

const SHIPS: u32 = 0x6DD7A4;
const SHIP_COUNT: u32 = 0x6DD894;
const CONVOYS: u32 = 0x6DD7A8;

pub trait PrepareHost {
    /// `operator new` (0x0064F7B9).
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    /// Constructor de los datos del escondite (0x004F95C0): devuelve el objeto.
    fn hideout_ctor(&mut self, block: u32) -> u32;
    /// `0x004F9B40` (ECX = datos): inicializa con `0xFE - (+0xF & 3)`.
    fn hideout_init(&mut self, data: u32, value: u32);
    /// `0x004F9C80` (ECX = datos): (pieza, pieza+1, tipo, a, b) -> AL.
    fn piece_check(&mut self, data: u32, a: [u32; 5]) -> bool;
    /// `0x004FB9C0` (ECX = datos): cinco argumentos; el quinto (tipo 0x1B) conserva basura de ECX en los 24 bits altos.
    fn piece_place(&mut self, data: u32, a: [u32; 5]);
    /// `0x004FBDB0` (ECX = datos): (x, y).
    fn piece_place2(&mut self, data: u32, x: u32, y: u32);
    /// `0x00516170` (ECX = cuadrilla): crea el barco; puede reescribir `+0x12`. El tercer argumento conserva en el original
    /// los 16 bits altos de EAX (resto de la llamada anterior); solo los 16 bajos (`40*nivel+100`) son deducibles.
    fn ship_create(&mut self, m: &mut dyn Mem, band: u32, a: u32, b: u32, c: u32);
    /// `0x005062F0` (ECX = 0x6DD7A0): crea el convoy; escribe el índice en `out`.
    fn convoy_create(&mut self, m: &mut dyn Mem, out: u32, ship: u32);
    /// `0x0052BB00` (ECX = ciudad).
    fn town_52bb00(&mut self, town: u32) -> u32;
    /// `0x0051A4E0` (ECX = barco): (valor, resultado de 0x52BB00).
    fn ship_51a4e0(&mut self, ship: u32, a: u32, b: u32);
}

/// El original no avanza el índice con tipos de pieza distintos de 0x24/0x30/0x31/0x1B y se queda en un bucle infinito:
/// aquí se detecta y se hace `panic!` en vez de colgar.
pub fn band_prepare_hideout_and_convoy(m: &mut dyn Mem, host: &mut dyn PrepareHost, band: u32) {
    let tick = m.r32(0x6DE4B4);
    let mut flag = false;
    if m.r32(band) == 0 {
        let block = host.alloc(m, 0x6c);
        let data = if block != 0 { host.hideout_ctor(block) } else { 0 };
        let low = u32::from(m.r8(band + 0xf) & 3);
        m.w32(band, data);
        host.hideout_init(data, 0xfe - low);
        let mut i = 0u32;
        let mut guard = 0u32;
        while m.r16(band + 8) > 0 && i < u32::from(m.r16(band + 8)) {
            guard += 1;
            assert!(guard < 1_000_000, "el original no avanza con este tipo de pieza");
            let pieces = m.r32(band + 4);
            let p = pieces.wrapping_add(i * 8);
            m.w8(p + 6, 0);
            let kind = m.r8(p + 3);
            let data = m.r32(band);
            if kind == 0x24 {
                m.w8(p + 7, 0);
                let ok = host.piece_check(data, [p, p + 1, 0x24, 0, 0]);
                if ok {
                    let p = m.r32(band + 4).wrapping_add(i * 8);
                    let (x, y) = (u32::from(m.r8(p)), u32::from(m.r8(p + 1)));
                    host.piece_place(m.r32(band), [x, y, 0, 0x24, 0]);
                }
                let p = m.r32(band + 4).wrapping_add(i * 8);
                i = u32::from(m.r16(p + 4));
            } else if kind == 0x30 || kind == 0x31 {
                let ok = host.piece_check(data, [p, p + 1, 0x30, 0, 0]);
                let pieces = m.r32(band + 4);
                if ok {
                    let p = pieces.wrapping_add(i * 8);
                    let (x, y) = (u32::from(m.r8(p)), u32::from(m.r8(p + 1)));
                    host.piece_place2(m.r32(band), x, y);
                    let p = m.r32(band + 4).wrapping_add(i * 8);
                    i = u32::from(m.r16(p + 4));
                } else {
                    m.w8(pieces.wrapping_add(i * 8) + 3, 0x1b);
                }
            } else if kind == 0x1b {
                let v = if i & 1 != 0 { m.r8(0x67792f) } else { m.r8(0x677929) };
                m.w8(p + 7, v);
                let d = m.r32(band);
                let bl = i as u8;
                let dl = (tick as u8).wrapping_add(bl) & 7;
                let cl = m.r8(d + 0x5c).wrapping_sub(dl).wrapping_add(4);
                let pieces = m.r32(band + 4);
                m.w8(pieces.wrapping_add(i * 8), cl);
                let d = m.r32(band);
                let al = (((tick as i32) >> 4) as u8).wrapping_add(bl) & 7;
                let dl = m.r8(d + 0x5d).wrapping_sub(al).wrapping_add(4);
                let pieces = m.r32(band + 4);
                m.w8(pieces.wrapping_add(i * 8) + 1, dl);
                let pieces = m.r32(band + 4);
                let p = pieces.wrapping_add(i * 8);
                // EDX conserva los 24 bits altos del puntero de piezas al empujar el argumento.
                let v7 = (pieces & 0xffff_ff00) | u32::from(m.r8(p + 7));
                let ok = host.piece_check(m.r32(band), [p, p + 1, 0x1b, v7, 0]);
                if ok {
                    let pieces = m.r32(band + 4);
                    let p = pieces.wrapping_add(i * 8);
                    let (x, y, v7) = (u32::from(m.r8(p)), u32::from(m.r8(p + 1)), u32::from(m.r8(p + 7)));
                    host.piece_place(m.r32(band), [x, y, 0, 0x1b, v7]);
                    let pieces = m.r32(band + 4);
                    i = u32::from(m.r16(pieces.wrapping_add(i * 8) + 4));
                } else {
                    let head = m.r32(band + 4);
                    let off = (i & 0xffff) << 3;
                    let me = i as u16;
                    let mut ax = m.r16(head + 4);
                    let mut edx = head + 4;
                    let next = u32::from(m.r16(off.wrapping_add(head) + 4));
                    let mut found = me == ax;
                    if !found {
                        loop {
                            if ax >= m.r16(band + 8) {
                                break;
                            }
                            edx = head.wrapping_add(u32::from(ax) * 8 + 4);
                            ax = m.r16(edx);
                            if me == ax {
                                found = true;
                                break;
                            }
                        }
                    }
                    if found {
                        let n = m.r16(off.wrapping_add(head) + 4);
                        m.w16(edx, n);
                        let pieces = m.r32(band + 4);
                        let free = m.r16(band + 0xa);
                        m.w16(off.wrapping_add(pieces) + 4, free);
                        m.w16(band + 0xa, me);
                    }
                    i = next;
                }
            } else {
                // sin avance en el original
                continue;
            }
        }
    }
    // Barco y convoy.
    let conv_ptr = band + 0x10;
    if m.r16(conv_ptr) >= m.r16(0x6DD896) {
        if m.r16(band + 0x12) >= m.r16(SHIP_COUNT) {
            let lvl = u32::from(m.r8(band + 0x14));
            host.ship_create(m, band, 0, 1, lvl * 40 + 0x64);
            flag = true;
        }
        let bx = u32::from(m.r16(band + 0x12));
        let ship = m.r32(SHIPS).wrapping_add(bx * 0x180);
        let next = m.r16(ship + 6);
        m.w16(band + 0x12, next);
        m.w16(ship + 6, 0xffff);
        let kind = u32::from(m.r8(band + 0xe));
        let a = m.r16(0x6DDBB0 + kind * 0x34);
        m.w16(ship + 0x1e, a);
        let b = m.r16(0x6DDBB4 + kind * 0x34);
        m.w16(ship + 0x22, b);
        host.convoy_create(m, conv_ptr, bx);
        if flag {
            let v = (m.r32(ship + 0x14) as i32) >> 2;
            m.w32(ship + 0x18, v as u32);
            let c = u32::from(m.r16(conv_ptr));
            m.w16(band + 0x12, bx as u16);
            let conv = m.r32(CONVOYS).wrapping_add(c * 0x3c);
            m.w16(conv + 0xa, 0xffff);
            m.w16(ship + 8, 0xffff);
        }
    }
    let c = u32::from(m.r16(conv_ptr));
    if c >= u32::from(m.r16(0x6DD896)) || flag {
        return;
    }
    let mut cx = u32::from(m.r16(band + 0x12));
    let conv = m.r32(CONVOYS).wrapping_add(c * 0x3c);
    if cx as u16 >= m.r16(SHIP_COUNT) {
        return;
    }
    loop {
        let ship = m.r32(SHIPS).wrapping_add(cx * 0x180);
        let next = m.r16(ship + 6);
        m.w16(band + 0x12, next);
        let head = m.r16(conv + 0xa);
        m.w16(ship + 6, head);
        m.w16(conv + 0xa, cx as u16);
        let cv = m.r16(band + 0x10);
        m.w16(ship + 8, cv);
        let town = m.r32(0x6DE508).wrapping_add(u32::from(m.r8(0x6DE52E)) * 0x9f8);
        let r = host.town_52bb00(town);
        let lvl = u32::from(m.r8(band + 0x14));
        let k = u32::from(m.r8(ship + 0xe));
        let v = (lvl + 1).wrapping_mul(k).wrapping_mul(20).wrapping_add(0x32);
        host.ship_51a4e0(ship, v, r);
        cx = u32::from(m.r16(band + 0x12));
        if cx as u16 >= m.r16(SHIP_COUNT) {
            break;
        }
    }
}
