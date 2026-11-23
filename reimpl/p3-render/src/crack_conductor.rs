//! Conductor de grietas `0x59BAF0` y la valoración de edificios `0x4D3B40` que usa.
//! Traducción 1:1 sobre memoria plana (`Mem`, direcciones absolutas) con paridad en `tests/x86_crack_conductor.rs` frente al x86 original
//! (`oracle/x86_crack_conductor.py`, que ejecuta también `0x5308A0`, `0x5303B0`, `0x51F1F0` y `0x639C28` del original).
//!
//! Alcance: la consulta de oficina de `0x5308A0` (tercer argumento 0, la que usa el conductor), no su rama de alta (reserva memoria y llama a la interfaz).
//! Las tablas del ejecutable (`0x6729D0`, `0x672A00`, `0x672A15`, `0x672C14`) están copiadas del EXE inglés; índices fuera de ellas devuelven 0 (el original leería
//! datos vecinos) y la división entre cero del original (`0x4D3B40` con tipo 0x15..0x17) se convierte en pánico.
use crate::scene_math::Mem;

const MGR: u32 = 0x006D_E4A0;
const TOWNS: u32 = 0x006D_E508;
const PLAYER: u32 = 0x006D_FC14;
const TOWN_COUNT: u32 = 0x006D_E4B0;
const TYPE_COUNT: u32 = 0x006D_E4A6;
const TOWN_SIZE: u32 = 0x9F8;
const OFFICE_SIZE: u32 = 0x44C;

/// `[0x6729D0 + tipo]`: divisor de la superficie.
pub const T1: [u8; 0x30] = [50, 40, 30, 15, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 15, 15, 30, 0, 0, 0, 30, 0, 36, 26, 56, 30, 16, 36, 16, 6, 39, 6, 33, 9, 36, 36, 36, 26, 3, 0, 6, 0, 0, 0];
/// `[0x672A00 + tipo]` (con signo).
pub const T2: [i8; 0x30] = [30, 0, 36, 26, 68, 32, 22, 48, 17, 7, 45, 8, 40, 9, 44, 44, 38, 32, 3, 3, 7, 24, 0, 0, 80, 0, -116, 0, 24, 1, 0, 0, 20, 0, 50, 0, 80, 0, 35, 0, 100, 0, -116, 0, 70, 0, -56, 0];
/// `[0x672A15]` (con signo).
pub const T3: i32 = 24;
/// `[0x672C14 + mercancía]`: 0 para las que se cuentan sobre 200 y 1 sobre 20 (rama del umbral).
pub const T4: [u8; 0x18] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];

fn t1(i: u8) -> i32 {
    i32::from(*T1.get(usize::from(i)).unwrap_or(&0))
}
fn t2(i: u8) -> i32 {
    i32::from(*T2.get(usize::from(i)).unwrap_or(&0))
}
fn t4(i: u8) -> u8 {
    *T4.get(usize::from(i)).unwrap_or(&0)
}

/// Escribe un byte con el efecto de `0x4D3B40` sobre el registro de salida.
fn d(a: i32, b: i32) -> i32 {
    a.checked_div(b).expect("0x4D3B40: división entre cero (la excepción del x86)")
}

/// `0x4D3B40(rec, out, oficina)`: evalúa el tipo de edificio `rec` (registro de 0x14 bytes de `0x5303B0`) contra la oficina del comerciante y rellena `out` (0x30 bytes).
pub fn building_eval(m: &mut dyn Mem, rec: u32, out: u32, office: u32) {
    let r0 = m.r32(rec) as i32;
    let r4 = i32::from(m.r16(rec + 4));
    let ty = m.r8(rec + 6);
    let town = m.r8(rec + 7);
    let b0 = i32::from(m.r16(rec + 0xC));
    let f = f32::from_bits(m.r32(rec + 0x10));
    let townrec = m.r32(TOWNS).wrapping_add(u32::from(town) * TOWN_SIZE);
    let mut l = [0i32; 2];
    m.w16(out, r4 as u16);
    m.w16(out + 2, b0 as u16);
    // Porcentaje: 0 si f <= 0, 100 si f >= 1, trunc(f*100 + 0.5) entre medias (x87 sin redondeo intermedio: producto y suma exactos).
    let pct = if !(f < 1.0) {
        100
    } else if f > 0.0 {
        ((f64::from(f) * 100.0 + 0.5) as i64) as u8
    } else {
        0
    };
    m.w8(out + 4, pct);
    let b = b0;
    let v1 = d(b, t1(ty));
    let v2 = i32::from(m.r16(townrec + 0x2C6)).wrapping_mul(v1).wrapping_mul(7);
    let v3 = t2(ty).wrapping_mul(r4).wrapping_mul(7) / 3;
    m.w32(out + 0x1C, v3.wrapping_add(v2) as u32);
    let flags = m.r32(townrec + 0x2C8);
    let o = |m: &mut dyn Mem, off: u32| m.r32(office + off) as i32;
    let (mut a, mut b) = (b, b);
    let mul = |x: i32, y: i32| x.wrapping_mul(y);
    if (4..=0x14).contains(&ty) {
        let rb = mul(r0, b);
        match ty {
            4 => {
                m.w8(out + 6, 9);
                m.w8(out + 0x18, 0xC);
                m.w8(out + 0x19, 0x11);
                a = mul(rb, 20) / 3 >> 10;
                l = [mul(o(m, 0xE8), 7) / 20; 2];
            }
            5 => {
                m.w8(out + 6, 2);
                a = mul(rb, 200) / 3 >> 10;
                let t2cc = m.r32(townrec + 0x2CC);
                if t2cc != 0 {
                    m.w8(out + 7, 10);
                    let add = mul(r4, T3).wrapping_mul(7) / 3;
                    let cur = m.r32(out + 0x1C) as i32;
                    m.w32(out + 0x1C, cur.wrapping_add(add) as u32);
                    b = mul(mul(t2cc as i32, b), 27) >> 10;
                }
                m.w8(out + 0x18, 4);
                m.w8(out + 0x19, 0x11);
                let z = mul(o(m, 0xCC), 7);
                l = [z / 5, z / 10];
            }
            6 => {
                m.w8(out + 6, 3);
                m.w8(out + 0x18, 0);
                m.w8(out + 0x19, 0xB);
                let z = mul(o(m, 0xD0), 7);
                a = mul(rb, 47) >> 10;
                l = [z / 4, z / 10];
            }
            7 => {
                m.w8(out + 6, 0xC);
                m.w8(out + 0x18, 0x10); // `mov [edi+0x18], dl` con dl = 0x10 (restos del salto de la tabla)
                m.w8(out + 0x19, 0xB);
                a = mul(rb, 20) >> 10;
                l = [mul(o(m, 0xF4), 7); 2];
            }
            8 => {
                m.w8(out + 6, 5);
                let mut e = if flags & 2 == 2 { mul(b, 2) } else { mul(b, 4) };
                if flags & 0x2000 != 0 {
                    e >>= 1;
                } else if flags & 0x4000 != 0 {
                    e = mul(e, 3);
                }
                a = mul(mul(r0, e), 40) / 3 >> 12;
            }
            9 => {
                m.w8(out + 6, 0);
                let mut s = mul(b, 3);
                if flags & 2 == 0 {
                    s = mul(s, 2);
                }
                if flags & 0x2000 != 0 {
                    s /= 3;
                } else if flags & 0x4000 != 0 {
                    s = mul(s, 2);
                }
                let e = mul(r0, s);
                a = if e > 0x00A3_D70A { d(mul(e >> 8, 200) / 3, 24) } else { mul(e, 200) / 3 / 6144 };
            }
            10 => {
                m.w8(out + 6, 1);
                m.w8(out + 7, 0xD);
                m.w8(out + 0x18, 4);
                m.w8(out + 0x19, 0xB);
                a = mul(rb, 50) / 3 >> 10;
                b = mul(a, 40) / 100;
                let e = mul(o(m, 0xF8), 7);
                let p = mul(o(m, 0xC8), 7) / 4;
                l = [e / 10 + p, e / 5];
            }
            0xB => {
                m.w8(out + 6, 0xB);
                a = if rb > 0x3F_FFFF { mul(rb >> 10, 400) / 3 } else { mul(rb, 400) / 3 >> 10 };
            }
            0xC => {
                m.w8(out + 6, 8);
                m.w8(out + 0x18, 0xE);
                a = mul(rb, 20) >> 10;
                l[0] = mul(o(m, 0xE4), 7);
            }
            0xD => {
                m.w8(out + 6, 4);
                m.w8(out + 0x18, 0xB);
                a = mul(rb, 167) / 5 >> 10;
                l[0] = mul(o(m, 0xD4), 7);
            }
            0xE => {
                m.w8(out + 6, 0x10);
                a = mul(rb, 100) / 3 >> 10;
            }
            0xF => {
                m.w8(out + 6, 0xE);
                a = mul(rb, 167) / 5 >> 10;
            }
            0x10 => {
                m.w8(out + 6, 7);
                let mut e = mul(b, 2);
                if flags & 2 == 0 {
                    e = mul(b, 4);
                }
                if flags & 0x2000 != 0 {
                    e >>= 1;
                } else if flags & 0x4000 != 0 {
                    e = mul(e, 2);
                }
                a = mul(mul(r0, e), 67) / 5 >> 12;
            }
            0x11 => {
                m.w8(out + 6, 0x12);
                m.w8(out + 0x18, 0xB);
                a = mul(rb, 67) / 5 >> 10;
                l[0] = mul(o(m, 0x10C), 7);
            }
            0x12 => {
                m.w8(out + 6, 0x13);
                m.w8(out + 0x18, 0xB);
                a = mul(rb, 134) >> 10;
                l[0] = mul(o(m, 0x110), 7) >> 1;
            }
            0x13 => {
                m.w8(out + 6, 0xF);
                m.w8(out + 0x18, 0xB);
                a = mul(rb, 67) / 5 >> 10;
                l[0] = mul(o(m, 0x100), 7);
            }
            _ => {
                m.w8(out + 6, 0x11);
                let e = if flags & 2 != 0 {
                    mul(b, 3)
                } else if flags & 0x2000 != 0 {
                    mul(b, 4)
                } else if flags & 0x4000 != 0 {
                    mul(b, 9)
                } else {
                    mul(b, 6)
                };
                a = mul(mul(r0, e), 50) / 3 / 6144;
            }
        }
    }
    // Cierre: relación existencias/capacidad en +5.
    let (o6, o7) = (m.r8(out + 6), m.r8(out + 7));
    let c4 = |m: &mut dyn Mem, i: u8| m.r32(office + 0xC4 + 4 * u32::from(i)) as i32;
    let (num, den) = if o7 < 0x18 {
        let s = mul(c4(m, o6).wrapping_add(c4(m, o7)), 100);
        (s, a.wrapping_add(b))
    } else if o6 < 0x18 {
        (mul(c4(m, o6), 100), a)
    } else {
        (0, 1)
    };
    if den < 1 {
        m.w8(out + 5, 0);
    } else {
        let q = d(num.wrapping_add(den >> 1), den) as u8;
        m.w8(out + 5, if q > 100 { 100 } else { q });
    }
    for k in 0..2u32 {
        let t = m.r8(out + 0x18 + k);
        if t < 0x18 {
            let stock = m.r32(office + 4 + 4 * u32::from(t)) as i32;
            let cap = m.r32(office + 0x64 + 4 * u32::from(t)) as i32;
            if stock < cap {
                m.w8(out + 0x1A + k, 1);
                if stock <= 0 {
                    m.w8(out + 5, 0);
                }
            } else {
                m.w8(out + 0x1A + k, 0);
            }
            let x = mul(l[k as usize], 10);
            let v = if t4(t) == 0 { x / 2000 } else { x / 200 };
            m.w32(out + 0x10 + 4 * k, (v as u32) & 0xFFFF);
        } else {
            m.w32(out + 0x10 + 4 * k, 0);
        }
        let u = m.r8(out + 6 + k);
        if u < 0x18 {
            let p = m.r32(office + 0xC4 + 4 * u32::from(u)) as i32;
            let x = mul(p, 7);
            m.w32(out + 8 + 4 * k, if t4(u) == 0 { x / 200 } else { x / 20 } as u32);
        } else {
            m.w32(out + 8 + 4 * k, 0);
        }
    }
}

/// `0x5308A0(jugador, ciudad, 0)`: oficina del comerciante `player` en `town` (dirección absoluta) o `None`.
pub fn find_office_flat(m: &mut dyn Mem, player: u32, town: u32) -> Option<u32> {
    let town = town & 0xFFFF;
    if town >= m.r32(MGR + 0x10) || (player & 0xFFFF) >= u32::from(m.r16(MGR + 0xA)) {
        return None;
    }
    let player = player & 0xFFFF;
    let cap = m.r16(MGR + 8);
    let pool = m.r32(MGR + 0x74);
    let towns = m.r32(MGR + 0x68);
    let mut at = m.r16(towns.wrapping_add(town * TOWN_SIZE) + 0x784);
    while at < cap {
        let node = pool.wrapping_add(u32::from(at) * OFFICE_SIZE);
        if u32::from(m.r16(node + 0x2C4)) == player {
            return Some(node);
        }
        at = m.r16(node + 0x2CA);
    }
    None
}

/// Efectos externos del conductor.
pub trait ConductorEnv {
    /// `0x59BC90(x, y, p)`: coloca una grieta.
    fn crack(&mut self, x: u32, y: u32, p: u32);
    /// `0x57FFB0`: barrido de grietas.
    fn sweep(&mut self);
}

/// `0x59BAF0(escena)`. `scratch` es el registro de salida local (0x30 bytes) de `0x4D3B40`.
pub fn conduct(m: &mut dyn Mem, scene: u32, scratch: u32, env: &mut dyn ConductorEnv) {
    let town = m.r32(scene + 0xC324);
    if town >= m.r32(TOWN_COUNT) {
        return;
    }
    let player = m.r32(PLAYER);
    let Some(office) = find_office_flat(m, player, town) else { return };
    let townrec = m.r32(TOWNS).wrapping_add(town * TOWN_SIZE);
    let types = m.r32(MGR + 0x70);
    let n_types = m.r16(TYPE_COUNT);
    let mut ti = m.r16(office + 0x2CC);
    while ti < n_types {
        let rec = types.wrapping_add(u32::from(ti) * 0x14);
        let ty = m.r8(rec + 6);
        if ty >= 4 {
            for i in 0..0x30 {
                m.w8(scratch + i, 0);
            }
            m.w16(scratch + 6, 0xFFFF);
            m.w16(scratch + 0x18, 0xFFFF);
            m.w8(scratch + 0x22, ty);
            m.w8(scratch + 0x23, player as u8);
            building_eval(m, rec, scratch, office);
            let (lvl, cur) = (m.r8(scratch + 4), m.r8(scratch + 5));
            let count = m.r16(townrec + 0x776);
            let mut bi = m.r16(office + 0x2CE);
            if cur < lvl && bi < count {
                loop {
                    let b = m.r32(townrec + 0x75C).wrapping_add(u32::from(bi) * 8);
                    if u32::from(m.r8(b + 2)) == player && m.r8(b + 3) == ty {
                        let p = (i32::from(cur) * 255) / i32::from(lvl);
                        env.crack(u32::from(m.r8(b)), u32::from(m.r8(b + 1)), p as u32);
                    }
                    bi = m.r16(b + 4);
                    if bi >= count {
                        break;
                    }
                }
            }
        }
        ti = m.r16(rec + 8);
    }
    env.sweep();
}
