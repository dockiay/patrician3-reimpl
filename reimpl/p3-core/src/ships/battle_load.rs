//! Carga de un combate naval guardado, `SeaBattle::deserialize` (`0x00604600`), sobre la imagen de memoria del original.
//!
//! Lee el flujo con la versión del guardado (`ver`; las ramas `<= 0x5C` conservan los formatos antiguos), reconstruye listas de
//! barcos y proyectiles y deja los vectores asignados en el objeto. Las demás funciones del original van por [`OpsHost`] con su
//! dirección: reserva (`0x64F7B9`, o `0x63A29F` con vista), constructor de barco local `0x61A4E0`, lector de barco `0x61A880`,
//! registro en la rejilla espacial `0x619450`, constructor `0x602A90` y lector `0x602EF0` de proyectil, y la virtual `+0x28`
//! (¿hay vista?). Los argumentos de `0x619450` que el original lee como byte o palabra se pasan enmascarados (el resto de bits es
//! residuo de registro).

use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

const ALLOC: u32 = 0x0064_f7b9;
const ALLOC_VIEW: u32 = 0x0063_a29f;
const SHIPS: u32 = 0x006d_d7a4;
const TOWNS_PTR: u32 = 0x006d_d7a8;
const LETTERS: u32 = 0x006d_d7a0;
const LETTER_COUNT: u32 = 0x006d_d892;
const WARE_COUNT: u32 = 0x0067_2cc0;
const WARE_VALUE: u32 = 0x0067_2cd0;
const WARE_WEIGHT: u32 = 0x0067_2ce4;

fn cp(m: &mut dyn Mem, dst: u32, src: u32, n: u32) {
    for i in 0..n {
        let b = m.r8(src.wrapping_add(i));
        m.w8(dst.wrapping_add(i), b);
    }
}

fn register(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: [u32; 6]) -> u32 {
    host.call(m, 0x619450, this, &[a[0] & 0xff, a[1] & 0xff, a[2] & 0xffff, a[3] & 0xffff, a[4] & 0xffff, a[5] & 0xffff])
}

/// Barcos de una cadena (`+0x10` o `+0x65C`): crea el barco local, lo lee y lo registra en la rejilla; fija la marca `+0x13C` del
/// barco del mapa (`1` si `301·A > 605·B` con las sumas por ranura de la carga).
fn read_chain(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, head: u32, ver: u32, mut p: u32, first: bool) -> u32 {
    let mut id = head;
    while id != 0xffff {
        let view = has_view(m, host, this);
        let ship;
        if view {
            let q = host.call(m, ALLOC, 0, &[0x1168]);
            if q != 0 {
                let idx = if first { id & 0xff } else { id & 0xff };
                host.call(m, 0x61a4e0, q, &[0, this, idx, 0]);
                m.w32(q, 0x67aad0);
                m.w16(q + 0x1164, 0xffff);
                m.w16(q + 0x1166, 0xffff);
                m.w8(q + 0x127, 0);
                ship = q;
            } else {
                ship = 0;
            }
        } else {
            let q = host.call(m, ALLOC, 0, &[0x164]);
            ship = if q != 0 { host.call(m, 0x61a4e0, q, &[0, this, id, 0]) } else { 0 };
        }
        let arr = m.r32(this + 4);
        m.w32(arr + id * 4, ship);
        let obj = rind32(m, this + 4, id * 4);
        p = host.call(m, 0x61a880, obj, &[p, ver, this]);
        let r = register(m, host, this, [0, id, 0, 0, 0, 0]);
        let a74 = m.r32(this + 0xa74);
        m.w16(a74 + id * 2, r as u16);
        let r = register(m, host, this, [6, id, 0, 0, 0, 0]);
        let a78 = m.r32(this + 0xa78);
        m.w16(a78 + id * 2, r as u16);
        let ship = rind32(m, this + 4, id * 4);
        let (mut sum_a, mut sum_b) = (0i32, 0i32);
        for j in 0..0x18u32 {
            let rec = m.r32(ship + 4).wrapping_mul(0x180).wrapping_add(m.r32(SHIPS));
            let t = u32::from(m.r8(rec.wrapping_add(j).wrapping_add(0x13c)));
            if t < 6 {
                let c = (u32::from(m.r8(WARE_VALUE + t)) as i32).wrapping_mul(u32::from(m.r8(WARE_COUNT + t)) as i32);
                sum_a = sum_a.wrapping_add(c);
                let d = (u32::from(m.r16(WARE_WEIGHT + t * 2)) as i32).wrapping_mul(6);
                let num = c.wrapping_mul(15).wrapping_neg().wrapping_mul(32);
                let q = num.wrapping_div(d);
                sum_b = sum_b.wrapping_add(c.wrapping_add(q));
            }
        }
        let big = sum_a.wrapping_mul(301) > sum_b.wrapping_mul(605);
        m.w8(ship + 0x13c, u8::from(big));
        let next = m.r32(this + 0xc);
        id = u32::from(m.r16(next + id * 2));
    }
    p
}

/// `0x00604600` (ret 8: búfer, versión): devuelve el cursor tras leer el combate.
pub fn seabattle_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, src: u32, ver: u32) -> u32 {
    let mut p = src;
    m.w8(this + 0x5c, 0);
    cp(m, this + 0x558, p, 1);
    p += 1;
    if ver <= 0x5b {
        for dst in [0x658u32, 0x65a, 0x10, 0x65c] {
            let b = m.r8(p);
            m.w16(this + dst, u16::from(b));
            p += 1;
        }
        // El original lee un quinto byte y lo descarta.
        p += 1;
        if m.r16(this + 0x10) == 0xff {
            m.w16(this + 0x10, 0xffff);
        }
        if m.r16(this + 0x65c) == 0xff {
            m.w16(this + 0x65c, 0xffff);
        }
    } else {
        for dst in [0x658u32, 0x65a, 0x10, 0x65c] {
            cp(m, this + dst, p, 2);
            p += 2;
        }
    }
    m.w16(this + 0x12, 0xffff);
    for dst in [0x65eu32, 0x660, 0xad4, 0xad6, 0xad8, 0xada] {
        cp(m, this + dst, p, 2);
        p += 2;
    }
    cp(m, this + 0xadc, p, 4);
    p += 4;
    if ver > 0x22 {
        cp(m, this + 0x12e8, p, 1);
        p += 1;
    } else {
        m.w8(this + 0x12e8, 0);
    }
    if ver > 0x27 {
        cp(m, this + 0x130c, p, 2);
        p += 2;
        cp(m, this + 0x130e, p, 1);
        p += 1;
    } else {
        m.w16(this + 0x130c, 0);
        m.w8(this + 0x130e, 1);
    }
    let players = u32::from(m.r8(this + 0x558));
    for i in 0..players {
        m.w16(this + 0x15c + 2 * i, 0);
    }
    if ver > 0x29 {
        cp(m, this + 0x5c, p, 1);
        p += 1;
        if m.r8(this + 0x5c) != 0 {
            m.w16(this + 0x15c, 1);
        }
    } else {
        m.w8(this + 0x5c, 1);
    }
    if ver > 0x30 {
        cp(m, this + 0x670, p, 1);
        p += 1;
        cp(m, this + 0x671, p, 1);
        p += 1;
    } else {
        m.w8(this + 0x670, 0x40);
        m.w8(this + 0x671, 5);
    }
    if ver > 0x49 {
        cp(m, this + 0x1e90, p, 1);
        p += 1;
        cp(m, this + 0x1e91, p, 1);
        p += 1;
    }
    if ver > 0x56 {
        cp(m, this + 0xaca, p, 1);
        p += 1;
        cp(m, this + 0xacb, p, 1);
        p += 1;
    }
    if ver > 0x5c {
        cp(m, this + 0x1e94, p, 2);
        p += 2;
        cp(m, this + 0xad2, p, 1);
        p += 1;
    } else {
        let side0_set = m.r16(this + 0x35a) != 0xff;
        m.w16(this + 0x1e94, 0x800);
        m.w8(this + 0xad2, u8::from(side0_set) + 1);
    }
    let n = m.r16(this + 0x65a).wrapping_add(m.r16(this + 0x658));
    m.w16(this + 0x14, n);
    let n = u32::from(n);
    let view = has_view(m, host, this);
    let (alloc, sizes): (u32, [u32; 5]) = (if view { ALLOC_VIEW } else { ALLOC }, [n << 2, n * 2, n * 2, n * 2, n * 2]);
    let mut blocks = [0u32; 5];
    for (k, size) in sizes.iter().enumerate() {
        blocks[k] = if view { host.call(m, alloc, 0, &[0, *size]) } else { host.call(m, alloc, 0, &[*size]) };
    }
    m.w32(this + 4, blocks[0]);
    m.w32(this + 0xa74, blocks[1]);
    m.w32(this + 0xa78, blocks[2]);
    m.w32(this + 0xc, blocks[3]);
    m.w32(this + 8, blocks[4]);
    m.w8(this + 0xac8, 1);
    for i in 0..players {
        cp(m, this + 0x35a + 2 * i, p, 2);
        p += 2;
        cp(m, this + 0x5d + i, p, 1);
        p += 1;
        cp(m, this + 0x559 + i, p, 1);
        p += 1;
        if i < 2 {
            if ver > 0x20 {
                cp(m, this + 0xa84 + 4 * i, p, 4);
                p += 4;
            } else {
                m.w32(this + 0xa84 + 4 * i, 0xffff_ffff);
            }
        }
        if m.r8(this + 0x5d + i) == 0 {
            m.w8(this + 0xac8, 0);
        }
    }
    if ver <= 0x49 {
        let idx = if m.r16(this + 0x35a) == 0xff {
            Some(u32::from(m.r16(this + 0x65e)))
        } else if m.r16(this + 0x35c) == 0xff {
            Some(u32::from(m.r16(this + 0x660)))
        } else {
            None
        };
        let town = match idx {
            Some(i) => {
                let t = u32::from(rind16(m, TOWNS_PTR, i.wrapping_mul(15) * 4 + 0x10));
                {
                    let base = m.r32(SHIPS);
                    u32::from(m.r16((t * 3 << 7).wrapping_add(base).wrapping_add(0x42)))
                }
            }
            None => 0xffff,
        };
        if (town as u16) < m.r16(LETTER_COUNT) {
            let o = town << 4;
            let lt = m.r32(LETTERS);
            let b2 = m.r8(o.wrapping_add(lt).wrapping_add(2));
            m.w8(this + 0x1e90, b2);
            let lt = m.r32(LETTERS);
            let b3 = m.r8(o.wrapping_add(lt).wrapping_add(3));
            m.w8(this + 0x1e91, b3);
        } else {
            m.w8(this + 0x1e90, 0xff);
            m.w8(this + 0x1e91, 0xff);
        }
    }
    if ver > 0x5b {
        cp(m, this + 0xace, p, 2);
        p += 2;
    } else {
        m.w16(this + 0xace, 8);
    }
    let words = u32::from(m.r16(this + 0xace));
    m.w16(this + 0xacc, (words << 5) as u16);
    let bm = host.call(m, ALLOC_VIEW, 0, &[0, words << 2]);
    m.w32(this + 0x664, bm);
    let slots = u32::from(m.r16(this + 0xacc));
    let pr = host.call(m, ALLOC_VIEW, 0, &[0, slots << 2]);
    m.w32(this + 0x668, pr);
    for k in 0..words {
        let dst = m.r32(this + 0x664) + 4 * k;
        cp(m, dst, p, 4);
        p += 4;
    }
    let count = u32::from(m.r16(this + 0x14));
    if ver > 0x5b {
        for k in 0..count {
            let dst = m.r32(this + 0xc) + 2 * k;
            cp(m, dst, p, 2);
            p += 2;
        }
        for k in 0..count {
            let dst = m.r32(this + 8) + 2 * k;
            cp(m, dst, p, 2);
            p += 2;
        }
    } else {
        for base in [0xcu32, 0x8] {
            for k in 0..0xffu32 {
                let b = u32::from(m.r8(p));
                p += 1;
                if k < count {
                    let dst = m.r32(this + base) + 2 * k;
                    m.w16(dst, if b < count { b as u16 } else { 0xffff });
                }
            }
        }
    }
    // Rejilla espacial de la zona: 0x20 (0x40 desde la versión 0x47) entradas de 4 palabras por grupo.
    let n_entries: u32 = if ver > 0x46 { 0x40 } else { 0x20 };
    for (base, ty_a, ty_b) in [(0x672u32, 3u32, 5u32), (0x872, 2, 4)] {
        for i in 0..0x40u32 {
            let e = this + base + 8 * i;
            if i < n_entries {
                for k in 0..4 {
                    cp(m, e + 2 * k, p, 2);
                    p += 2;
                }
            } else {
                m.w16(e, 0xffff);
            }
            let cx = u32::from(m.r16(e));
            if cx != 0xffff {
                let ax = u32::from(m.r16(e + 2));
                let w2 = u32::from(m.r16(e + 4));
                let w4 = u32::from(m.r16(e + 6));
                let ecx = (ax & 1).wrapping_add(cx.wrapping_mul(2));
                let a5 = ax.wrapping_add(1).wrapping_mul(17);
                let a4 = ecx.wrapping_add(w4).wrapping_add(1).wrapping_mul(17);
                let a3 = ax.wrapping_sub(w2).wrapping_sub(w4).wrapping_add(1).wrapping_mul(17);
                let a2 = ecx.wrapping_sub(w2).wrapping_add(1).wrapping_mul(17);
                let r = register(m, host, this, [ty_a, i, a2, a3, a4, a5]);
                let off = (r & 0xffff) << 2;
                let v38 = m.r32(this + 0x38).wrapping_add(off);
                let v28 = m.r32(this + 0x28).wrapping_add(off);
                let a_ = m.r16(v38 + 2).wrapping_add(0xc8);
                let c_ = m.r16(v38).wrapping_sub(0xc8);
                let b_ = m.r16(v28 + 2).wrapping_add(0xc8);
                let d_ = m.r16(v28).wrapping_sub(0xc8);
                register(m, host, this, [ty_b, i, u32::from(d_), u32::from(c_), u32::from(b_), u32::from(a_)]);
            }
        }
    }
    let head0 = u32::from(m.r16(this + 0x10));
    p = read_chain(m, host, this, head0, ver, p, true);
    let head1 = u32::from(m.r16(this + 0x65c));
    p = read_chain(m, host, this, head1, ver, p, false);
    let slots = u32::from(m.r16(this + 0xacc));
    for k in 0..slots {
        let word = rind32(m, this + 0x664, (k >> 5) * 4);
        if word & (1u32 << (k & 31)) != 0 {
            let q = host.call(m, ALLOC, 0, &[0x38]);
            let obj = if q != 0 { host.call(m, 0x602a90, q, &[k, 1, 0, 0, 0, 0, 0xc, this, 0, 0xff, 8]) } else { 0 };
            let tab = m.r32(this + 0x668);
            m.w32(tab + k * 4, obj);
            let o = rind32(m, this + 0x668, k * 4);
            p = host.call(m, 0x602ef0, o, &[p, ver, this, k]);
        } else {
            let tab = m.r32(this + 0x668);
            m.w32(tab + k * 4, 0);
        }
    }
    m.w8(this + 0x1e92, 1);
    m.w8(this + 0xab0, 1);
    let flag = if m.r16(this + 0x35c) == 0xff && m.r16(this + 0x35a) == 0xfe { 2 } else { 1 };
    m.w8(this + 0xab1, flag);
    p
}

fn has_view(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> bool {
    let vt = m.r32(this);
    let f = m.r32(vt + 0x28);
    host.call(m, f, this, &[]) as u8 != 0
}

fn rind8(m: &mut dyn Mem, ptr: u32, off: u32) -> u8 {
    let b = m.r32(ptr);
    m.r8(b.wrapping_add(off))
}
fn rind16(m: &mut dyn Mem, ptr: u32, off: u32) -> u16 {
    let b = m.r32(ptr);
    m.r16(b.wrapping_add(off))
}
fn rind32(m: &mut dyn Mem, ptr: u32, off: u32) -> u32 {
    let b = m.r32(ptr);
    m.r32(b.wrapping_add(off))
}
