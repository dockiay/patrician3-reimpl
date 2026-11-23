//! `SeaMapWindow` (0x46CE70–0x46F6E0): la ventana del minimapa náutico (la carta pequeña del mar) con sus marcadores de ciudades, barcos
//! ajenos y pulsos de evento. Disposición del objeto: `+0x14/+0x18` posición y `+0x2c/+0x30` tamaño (clase base), `+0x41`/`+0x42`/`+0x48`
//! estado de la ventana, `+0x94` marca, `+0x98` textura del fondo, `+0x9c/+0xa0` tamaño del fondo, `+0xa4/+0xa6` contadores de
//! marcadores, `+0xa8/+0xac/+0xb0` tablas de barcos ajenos, `+0xb4..+0xba` rectángulo de la vista (4 palabras con signo), `+0xbc` vista activa,
//! `+0xc0` marcadores de ciudad (12 bytes cada uno), `+0xc4/+0xc8` pulsos de evento (12 bytes cada uno), `+0xd4` marcas de ciudad (un byte
//! por ciudad), `+0xd8..+0xdd` parpadeo, `+0xdf/+0xe0` constantes.

use p3_core::numeric::ftol_i32;

use super::{cdecl, free, memcpy, memset, new, vcall, Host, Mem, Ret};

const VTABLE: u32 = 0x0066_EAA8;
const BASE_CTOR: u32 = 0x004B_15F0;
const BASE_DTOR: u32 = 0x004B_1640;
const BASE_RESET: u32 = 0x004B_1650;
const BASE_SET_VISIBLE: u32 = 0x004B_21A0;
const FREE_TEXTURE: u32 = 0x004B_B490;
const INVALIDATE: u32 = 0x004B_9650;
const DIRTY: u32 = 0x006D_A5F0;
const SCOPE_GUARD: u32 = 0x0064_F253;
const STRING_INIT: u32 = 0x0064_F2C1;
const NAME_ARG: u32 = 0x006B_96D8;
const TOWN_COUNT: u32 = 0x006D_E4B0;
const TOWN_ORDER: u32 = 0x006D_E4B8;
const TOWN_POS: u32 = 0x006D_DB90;
const SCALE_X: f64 = 9.23077;
const SCALE_Y: f64 = 7.92952;
const RESET_STATE: u32 = 0x0046_D1E0;
const DTOR: u32 = 0x0046_CFA0;
const REBUILD_SHIP_MARKERS: u32 = 0x0046_EDF0;
const VISIBLE_CHECK: u32 = 0x004B_2200;
const OWN_TOWN: u32 = 0x006D_FC14;
const TABLE: u32 = 0x006D_E4A0;
const TABLE_ENTRY: u32 = 0x0053_03C0;
const SHIP_COUNT: u32 = 0x006D_D894;
const SHIP_TABLE: u32 = 0x006D_D7A4;
const LAST_EVENT: u32 = 0x006D_FC1C;
const NO_EVENT: u32 = 0x0067_3CA8;
const STRING_FORMAT: u32 = 0x0064_DF80;
const LOAD_DEFS: u32 = 0x004B_E060;
const LOAD_TEXTURE: u32 = 0x004B_CA00;
const SIZE_OF: u32 = 0x004B_BB20;

fn fz(v: u32, scale: f64) -> u32 {
    ftol_i32(f64::from(v as i32) / scale) as u32
}

/// `(int)(v / escala + 0.5)`.
fn fr(v: u32, scale: f64) -> u32 {
    ftol_i32(f64::from(v as i32) / scale + 0.5) as u32
}

fn free_nz(m: &mut dyn Mem, h: &mut dyn Host, p: u32) {
    if p != 0 {
        free(m, h, p);
    }
}

/// Parte común de los constructores: los campos a cero y `reset_state` (`0x46D1E0`).
fn zero_fields(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    m.w32(this + 0x98, 0);
    m.w16(this + 0xA6, 0);
    m.w16(this + 0xA4, 0);
    for off in [0xA8, 0xAC, 0xB0, 0xC4, 0xC0, 0xD4] {
        m.w32(this + off, 0);
    }
}

/// `0x46CE70` (constructor).
fn ctor(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    let arg = m.r32(NAME_ARG);
    h.call(m, BASE_CTOR, this, &[arg]);
    zero_fields(m, h, this);
    m.w32(this, VTABLE);
    m.w8(this + 0xDF, 0xFF);
    m.w8(this + 0xE0, 0xFF);
    h.call(m, RESET_STATE, this, &[]);
    this
}

/// `0x46CF10(nombre)` (constructor con nombre).
fn ctor_with_name(m: &mut dyn Mem, h: &mut dyn Host, this: u32, name: u32) -> u32 {
    h.call(m, BASE_CTOR, this, &[name]);
    m.w32(this, VTABLE);
    zero_fields(m, h, this);
    h.call(m, RESET_STATE, this, &[]);
    this
}

/// `0x46CFA0` (destructor).
fn dtor(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    m.w32(this, VTABLE);
    m.w32(this + 0x98, 0);
    m.w16(this + 0xA6, 0);
    m.w16(this + 0xA4, 0);
    h.call(m, RESET_STATE, this, &[]);
    h.call(m, BASE_DTOR, this, &[])
}

/// Posición de una ciudad en el minimapa (`+0` x, `+4` y del registro de 52 bytes de `0x6DDB90`) en palabras.
fn town_marker(m: &mut dyn Mem, index: u32, out: u32) {
    let rec = TOWN_POS.wrapping_add(index.wrapping_mul(52));
    let (x, y) = (m.r32(rec), m.r32(rec.wrapping_add(4)));
    m.w16(out, fr(x, SCALE_X) as u16);
    m.w16(out + 2, fr(y, SCALE_Y) as u16);
    m.w32(out + 4, 0xFFFF_FFF6);
    m.w8(out + 8, 0);
}

/// `0x46D010`: crea los marcadores de las ciudades (`+0xC0`, 12 bytes cada uno) y su tabla de marcas (`+0xD4`).
fn init_town_markers(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let old = m.r32(this + 0xC0);
    free_nz(m, h, old);
    let d4 = m.r32(this + 0xD4);
    m.w32(this + 0xC0, 0);
    free_nz(m, h, d4);
    m.w32(this + 0xD4, 0);
    let n = m.r32(TOWN_COUNT);
    let p = new(m, h, n.wrapping_mul(12));
    m.w32(this + 0xC0, p);
    let mut i = 0u32;
    let mut n = m.r32(TOWN_COUNT);
    while i < n {
        let index = u32::from(m.r8(TOWN_ORDER.wrapping_add(i)));
        town_marker(m, index, p.wrapping_add(i.wrapping_mul(12)));
        i += 1;
        n = m.r32(TOWN_COUNT);
    }
    let n = m.r32(TOWN_COUNT);
    let q = new(m, h, n);
    m.w32(this + 0xD4, q);
    let n = m.r32(TOWN_COUNT);
    memset(m, q, 0, n);
}

/// `0x46D140`: libera las tablas de marcadores.
fn free_marker_arrays(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    m.w16(this + 0xA6, 0);
    m.w16(this + 0xA4, 0);
    for off in [0xA8, 0xAC, 0xB0, 0xC0, 0xD4] {
        let p = m.r32(this + off);
        if p != 0 {
            free(m, h, p);
            m.w32(this + off, 0);
        }
    }
}

/// `0x46D1E0` (reset_state): vuelve al estado inicial y libera textura, tablas y pulsos.
fn reset_state(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    h.call(m, BASE_RESET, this, &[]);
    let tex = m.r32(this + 0x98);
    m.w8(this + 0x41, 1);
    if tex != 0 {
        cdecl(m, h, FREE_TEXTURE, &[tex]);
    }
    let a8 = m.r32(this + 0xA8);
    m.w32(this + 0x98, 0);
    m.w32(this + 0x9C, 0);
    m.w32(this + 0xA0, 0);
    m.w16(this + 0xA6, 0);
    m.w16(this + 0xA4, 0);
    m.w8(this + 0xBC, 0);
    if a8 != 0 {
        free(m, h, a8);
        m.w32(this + 0xA8, 0);
    }
    for off in [0xAC, 0xB0] {
        let p = m.r32(this + off);
        if p != 0 {
            free(m, h, p);
            m.w32(this + off, 0);
        }
    }
    let d4 = m.r32(this + 0xD4);
    m.w8(this + 0x94, 0);
    m.w8(this + 0xDC, 0);
    m.w8(this + 0xDD, 0x1E);
    m.w32(this + 0xD8, 0);
    free_nz(m, h, d4);
    let c4 = m.r32(this + 0xC4);
    m.w32(this + 0xD4, 0);
    free_nz(m, h, c4);
    let c0 = m.r32(this + 0xC0);
    m.w32(this + 0xC4, 0);
    m.w16(this + 0xC8, 0);
    free_nz(m, h, c0);
    m.w32(this + 0xC0, 0);
}

/// `0x46D940(nombre, número)` (`init`): carga la textura del fondo de la carta (`%s` con el número) y fija el tamaño de la ventana.
/// `nombre` es un `CString` pasado por valor (su puntero va en el primer argumento de pila).
fn init(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) -> u32 {
    let (name, number) = (a[0], a[1]);
    let s = esp - 0x1C;
    let v = m.r32(0x006C_7CCC);
    m.w32(s, v);
    cdecl(m, h, STRING_FORMAT, &[s, 0x006B_96DC, number]);
    let text = m.r32(s);
    let path = m.r32(0x006B_EEE0);
    h.call(m, LOAD_DEFS, 0x006D_D524, &[text, 0x006B_96E8, path, 0x006D_BB58, 0x1000, name]);
    let tex = cdecl(m, h, LOAD_TEXTURE, &[0x006D_BB58, 0]);
    m.w32(this + 0x98, tex);
    if tex == 0 {
        h.call(m, SCOPE_GUARD, s, &[]);
        h.call(m, SCOPE_GUARD, esp + 4, &[]);
        return 0;
    }
    let size = esp - 0x18;
    cdecl(m, h, SIZE_OF, &[tex, size]);
    let (w, hh) = (m.r32(size), m.r32(size + 4));
    let (w, hh) = (w.wrapping_add(0x13), hh.wrapping_add(0x17));
    m.w32(this + 0xA0, hh);
    m.w32(this + 0x30, hh);
    m.w32(this + 0x9C, w);
    m.w32(this + 0x2C, w);
    h.call(m, SCOPE_GUARD, s, &[]);
    h.call(m, SCOPE_GUARD, esp + 4, &[]);
    1
}

/// `0x46EB50(índice, x, y, valor)`: añade un barco ajeno (posición del mundo reducida a la carta).
fn add_foreign_ship_marker(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) -> u32 {
    let (index, wx, wy, value) = (a[0], a[1], a[2], a[3]);
    let x = fz(wx, SCALE_X);
    let y = fz(wy, SCALE_Y);
    let px = (x as i16 as i32 as u32).wrapping_add(m.r32(this + 0x14));
    let py = (y as i16 as i32 as u32).wrapping_add(m.r32(this + 0x18));
    let r = esp - 0x10;
    m.w32(r, px);
    m.w32(r + 8, px);
    m.w32(r + 4, py);
    m.w32(r + 0xC, py);
    let off = (index as u16 as i16 as i32 as u32) << 2;
    let t = m.r32(this + 0xAC);
    m.w16(t.wrapping_add(off), x as u16);
    let t = m.r32(this + 0xAC);
    m.w16(t.wrapping_add(off).wrapping_add(2), y as u16);
    let t = m.r32(this + 0xB0);
    m.w32(t.wrapping_add(off), value);
    let n = m.r16(this + 0xA6);
    m.w16(this + 0xA6, n.wrapping_add(1));
    h.call(m, INVALIDATE, DIRTY, &[r])
}

fn sx(m: &mut dyn Mem, a: u32) -> u32 {
    m.r16(a) as i16 as i32 as u32
}

/// Rectángulo (con margen de 10) de la vista, en pantalla, a partir de las palabras `+0xB4..+0xBA` y el origen `(ox, oy)`.
fn view_rect(m: &mut dyn Mem, this: u32, ox: u32, oy: u32, r: u32) {
    let left = sx(m, this + 0xB4).wrapping_add(ox).wrapping_sub(0xA);
    let right = sx(m, this + 0xB8).wrapping_add(ox).wrapping_add(0xA);
    let top = sx(m, this + 0xB6).wrapping_add(oy).wrapping_sub(0xA);
    let bottom = sx(m, this + 0xBA).wrapping_add(oy).wrapping_add(0xA);
    m.w32(r, left);
    m.w32(r + 8, right);
    m.w32(r + 4, top);
    m.w32(r + 0xC, bottom);
}

/// `0x46EBF0(izq, arriba, der, abajo)`: cambia el rectángulo de la vista e invalida el antiguo y el nuevo.
fn set_view_rect(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) -> u32 {
    let r = esp - 0x10;
    let (ox, oy) = (m.r32(this + 0x14), m.r32(this + 0x18));
    view_rect(m, this, ox, oy, r);
    h.call(m, INVALIDATE, DIRTY, &[r]);
    for (k, v) in a.iter().take(4).enumerate() {
        m.w16(this + 0xB4 + 2 * k as u32, *v as u16);
    }
    let ox = m.r32(this + 0x14);
    let oy = m.r32(this + 0x18);
    view_rect(m, this, ox, oy, r);
    let ret = h.call(m, INVALIDATE, DIRTY, &[r]);
    m.w8(this + 0xBC, 1);
    ret
}

/// `0x46ECD0`: borra el rectángulo de la vista (invalida el antiguo respecto de `+0x20/+0x24`).
fn clear_view_rect(m: &mut dyn Mem, h: &mut dyn Host, this: u32, esp: u32) {
    let r = esp - 0x10;
    let (ox, oy) = (m.r32(this + 0x20), m.r32(this + 0x24));
    view_rect(m, this, ox, oy, r);
    h.call(m, INVALIDATE, DIRTY, &[r]);
    m.w8(this + 0xBC, 1);
    for off in [0xBA, 0xB8, 0xB6, 0xB4] {
        m.w16(this + off, 0);
    }
}

/// `0x46F1B0(visible)`: al mostrar la carta marca las ciudades con barcos propios, rehace las tablas de barcos ajenos y la invalida.
fn set_visible(m: &mut dyn Mem, h: &mut dyn Host, this: u32, flag: u32, esp: u32) -> u32 {
    if flag as u8 != 0 {
        let own = m.r32(OWN_TOWN);
        let rec = h.call(m, TABLE_ENTRY, TABLE, &[own]);
        let count = m.r32(SHIP_COUNT) & 0xFFFF;
        let mut i = u32::from(m.r16(rec.wrapping_add(0xE)));
        if (i as i32) < count as i32 {
            loop {
                let base = m.r32(SHIP_TABLE);
                let ship = base.wrapping_add(i.wrapping_mul(0x180));
                let town = u32::from(m.r8(ship.wrapping_add(0x39)));
                let n = m.r32(TOWN_COUNT);
                let valid = u32::from(town < n);
                let mode = u32::from(m.r16(ship.wrapping_add(0x134)) < 4);
                if (mode & valid) != 0 {
                    let flags = m.r32(this + 0xD4);
                    if flags != 0 {
                        m.w8(flags.wrapping_add(town), 1);
                    }
                }
                i = u32::from(m.r16(ship.wrapping_add(4)));
                let count = m.r32(SHIP_COUNT) & 0xFFFF;
                if (i as i32) >= count as i32 {
                    break;
                }
            }
        }
        for off in [0xA8, 0xAC, 0xB0] {
            let p = m.r32(this + off);
            if p != 0 {
                free(m, h, p);
                m.w32(this + off, 0);
            }
        }
        m.w16(this + 0xA4, 0);
        m.w16(this + 0xA6, 0);
        for off in [0xA8, 0xAC, 0xB0] {
            let n = (m.r32(SHIP_COUNT) & 0xFFFF) << 2;
            let p = new(m, h, n);
            m.w32(this + off, p);
        }
        h.call(m, REBUILD_SHIP_MARKERS, this, &[]);
    }
    let cur = m.r8(this + 0x48);
    if flag as u8 != cur {
        let r = esp - 0x18 + 8;
        let (x, y) = (m.r32(this + 0x14), m.r32(this + 0x18));
        let (w, hh) = (m.r32(this + 0x2C), m.r32(this + 0x30));
        m.w32(r, x);
        m.w32(r + 4, y);
        m.w32(r + 8, w.wrapping_add(x));
        m.w32(r + 0xC, hh.wrapping_add(y));
        h.call(m, INVALIDATE, DIRTY, &[r]);
    }
    h.call(m, BASE_SET_VISIBLE, this, &[flag])
}

/// `0x46F340(x, y)`: añade un pulso de evento en esa posición del mundo (si la carta está visible y hay evento), salvo que ya exista.
fn add_event_pulse(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) {
    let vis = h.call(m, VISIBLE_CHECK, this, &[]);
    let last = m.r32(LAST_EVENT);
    let none = m.r32(NO_EVENT);
    let cl = u32::from(last != none);
    if (vis & cl) as u8 == 0 {
        return;
    }
    let old = m.r32(this + 0xC4);
    let x = fr(a[0] & 0xFFFF, SCALE_X);
    let y = fr(a[1] & 0xFFFF, SCALE_Y);
    let count = u32::from(m.r16(this + 0xC8));
    let mut i = 0u32;
    if (count as i32) > 0 {
        let mut p = old;
        loop {
            let same_y = m.r16(p.wrapping_add(2)) == y as u16;
            let same_x = m.r16(p) == x as u16;
            if same_x && same_y {
                break;
            }
            i += 1;
            p = p.wrapping_add(12);
            if (i as i32) >= count as i32 {
                break;
            }
        }
    }
    if i != count {
        return;
    }
    let size = count.wrapping_mul(3).wrapping_add(3) << 2;
    let p = new(m, h, size);
    let count = u32::from(m.r16(this + 0xC8));
    let src = m.r32(this + 0xC4);
    memcpy(m, p, src, count.wrapping_mul(12));
    let count = u32::from(m.r16(this + 0xC8));
    let rec = p.wrapping_add(count.wrapping_mul(12));
    m.w32(rec + 4, 0xFF);
    let count = u32::from(m.r16(this + 0xC8));
    m.w8(p.wrapping_add(count.wrapping_mul(12)) + 8, 1);
    let count = u32::from(m.r16(this + 0xC8));
    m.w16(p.wrapping_add(count.wrapping_mul(12)), x as u16);
    let count = u32::from(m.r16(this + 0xC8));
    m.w16(p.wrapping_add(count.wrapping_mul(12)) + 2, y as u16);
    let n = m.r16(this + 0xC8);
    m.w16(this + 0xC8, n.wrapping_add(1));
    m.w32(this + 0xC4, p);
    free_nz(m, h, old);
}

/// `0x46F4C0`: quita el primer pulso de evento.
fn remove_first_event_pulse(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let n = u32::from(m.r16(this + 0xC8));
    let old = m.r32(this + 0xC4);
    if n > 1 {
        let p = new(m, h, n.wrapping_mul(3).wrapping_mul(4).wrapping_sub(12));
        let src = m.r32(this + 0xC4).wrapping_add(12);
        let n = u32::from(m.r16(this + 0xC8));
        memcpy(m, p, src, n.wrapping_mul(3).wrapping_mul(4).wrapping_sub(12));
        let c = m.r16(this + 0xC8);
        m.w16(this + 0xC8, c.wrapping_sub(1));
        m.w32(this + 0xC4, p);
        free_nz(m, h, old);
        return;
    }
    m.w16(this + 0xC8, 0);
    free_nz(m, h, old);
    m.w32(this + 0xC4, 0);
}

/// `0x46F560`: añade al final el marcador de una ciudad (la última de la tabla de orden) copiando las tablas a bloques nuevos.
fn append_town_marker(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let old = m.r32(this + 0xC0);
    let n = m.r32(TOWN_COUNT);
    let p = new(m, h, n.wrapping_mul(12));
    let k = (m.r8(TOWN_COUNT).wrapping_sub(1)) as u32;
    memcpy(m, p, old, k.wrapping_mul(12));
    let index = u32::from(m.r8(TOWN_ORDER.wrapping_add(k)));
    town_marker(m, index, p.wrapping_add(k.wrapping_mul(12)));
    m.w32(this + 0xC0, p);
    free_nz(m, h, old);
    let n = m.r32(TOWN_COUNT);
    let flags = m.r32(this + 0xD4);
    let q = new(m, h, n);
    memcpy(m, q, flags, k);
    m.w8(q.wrapping_add(k), 0);
    m.w32(this + 0xD4, q);
    free_nz(m, h, flags);
}

const SET_COLOR: u32 = 0x004B_B870;
const LINE: u32 = 0x004B_D680;
const SCENE: u32 = 0x006E_51AC;
const PAGE: u32 = 0x006C_BDD8;
const SHIP_WINDOW: u32 = 0x006C_C940;
const CITY_COUNT16: u32 = 0x006D_E4AA;
const FAMILY: u32 = 0x006C_E6D0;
const FAMILY_LIST: u32 = 0x0048_85A0;
const TICK: u32 = 0x006D_CCF8;
const PAGE_REFRESH: u32 = 0x0045_A6B0;

/// Escribe el rectángulo `[izq, arriba, der, abajo]` en `at` y lo invalida (`0x4B9650` sobre `0x6DA5F0`).
fn inval(m: &mut dyn Mem, h: &mut dyn Host, at: u32, l: u32, t: u32, r: u32, b: u32) {
    m.w32(at, l);
    m.w32(at + 4, t);
    m.w32(at + 8, r);
    m.w32(at + 0xC, b);
    h.call(m, INVALIDATE, DIRTY, &[at]);
}

/// Rectángulo de 5x5 (o `n`x`n`) alrededor de un marcador `p` (x en `+0`, y en `+2`), desplazado por `-d`.
fn mark_rect(m: &mut dyn Mem, this: u32, p: u32, d: u32, n: u32) -> (u32, u32, u32, u32) {
    let l = m.r32(this + 0x14).wrapping_add(sx(m, p)).wrapping_sub(d);
    let t = sx(m, p + 2).wrapping_add(m.r32(this + 0x18)).wrapping_sub(d);
    (l, t, l.wrapping_add(n), t.wrapping_add(n))
}

/// `0x46EDF0`: reconstruye las tablas de barcos ajenos visibles.
fn rebuild_ship_markers(m: &mut dyn Mem, h: &mut dyn Host, this: u32, esp: u32) {
    let rect = esp.wrapping_sub(0x20);
    let rect2 = esp.wrapping_sub(0x10);
    let list_slot = esp.wrapping_sub(0x24);
    let vis = m.r32(SHIP_WINDOW);
    let vis = h.call(m, VISIBLE_CHECK, vis, &[]) as u8;
    m.w8(esp.wrapping_sub(0x31), vis);
    let fam = m.r32(FAMILY);
    let n_list = h.call(m, FAMILY_LIST, fam, &[list_slot, 0, 0]);
    let mut counter: u32 = 0;
    let mut first = 0u32;
    let town = m.r32(OWN_TOWN);
    if town < u32::from(m.r16(CITY_COUNT16)) {
        let ent = h.call(m, TABLE_ENTRY, TABLE, &[town]);
        first = u32::from(m.r16(ent + 0xE));
        let mut i = 0i32;
        while i < i32::from(m.r16(this + 0xA6) as i16) {
            let t = m.r32(this + 0xAC);
            let x = sx(m, t.wrapping_add((i as u32) << 2));
            let y = sx(m, t.wrapping_add((i as u32) << 2).wrapping_add(2));
            let (ox, oy) = (m.r32(this + 0x20), m.r32(this + 0x24));
            let l = x.wrapping_add(ox);
            let t2 = y.wrapping_add(oy);
            inval(m, h, rect, l.wrapping_sub(1), t2.wrapping_sub(3), l.wrapping_add(3), t2.wrapping_add(3));
            i += 1;
        }
        m.w16(this + 0xA6, 0);
        let mut i = 0i32;
        while i < i32::from(m.r16(this + 0xA4) as i16) {
            let t = m.r32(this + 0xA8);
            let x = sx(m, t.wrapping_add((i as u32) << 2));
            let y = sx(m, t.wrapping_add((i as u32) << 2).wrapping_add(2));
            let (ox, oy) = (m.r32(this + 0x20), m.r32(this + 0x24));
            let l = x.wrapping_add(ox);
            let t2 = y.wrapping_add(oy);
            inval(m, h, rect, l.wrapping_sub(1), t2.wrapping_sub(3), l.wrapping_add(3), t2.wrapping_add(3));
            i += 1;
        }
        m.w16(this + 0xA4, 0);
        let mut idx = first;
        if (idx as i32) < (m.r32(SHIP_COUNT) & 0xFFFF) as i32 {
            loop {
                let base = m.r32(SHIP_TABLE);
                let rec = idx.wrapping_mul(0x180).wrapping_add(base);
                if m.r16(rec + 0x134) == 0xF {
                    let p1 = m.r16(rec + 0x50);
                    let p2 = m.r16(rec + 0x52);
                    m.w16(rect, p2);
                    if p1 != 0xFFFF {
                        let other = u32::from(p1).wrapping_mul(0x180).wrapping_add(base);
                        let st = m.r16(other + 0x134);
                        if (st == 0x12 || st == 0xF) && vis != 0 {
                            let wx = sx(m, other + 0x1E);
                            let wy = sx(m, other + 0x22);
                            h.call(m, 0x0046_EB50, this, &[counter, wx, wy, u32::from(p1)]);
                            counter = counter.wrapping_add(1);
                        }
                    }
                    let p2 = u32::from(m.r16(rect));
                    if p2 as u16 != 0xFFFF {
                        let base = m.r32(SHIP_TABLE);
                        let other = p2.wrapping_mul(0x180).wrapping_add(base);
                        let st = m.r16(other + 0x134);
                        if (st == 0x12 || st == 0xF) && vis != 0 {
                            let wx = sx(m, other + 0x1E);
                            let wy = sx(m, other + 0x22);
                            h.call(m, 0x0046_EB50, this, &[counter, wx, wy, p2]);
                            counter = counter.wrapping_add(1);
                        }
                    }
                }
                idx = u32::from(m.r16(rec + 4));
                if (idx as i32) >= (m.r32(SHIP_COUNT) & 0xFFFF) as i32 {
                    break;
                }
            }
        }
    }
    let _ = counter;
    let v2 = h.call(m, VISIBLE_CHECK, this, &[]) as u8;
    if v2 == 0 {
        return;
    }
    if m.r32(OWN_TOWN) >= u32::from(m.r16(CITY_COUNT16)) {
        return;
    }
    let mut idx = first;
    if (idx as i32) >= (m.r32(SHIP_COUNT) & 0xFFFF) as i32 {
        return;
    }
    let mut n = 0u32;
    loop {
        let list = m.r32(list_slot);
        let rec = idx.wrapping_mul(0x180).wrapping_add(m.r32(SHIP_TABLE));
        let wx = sx(m, rec + 0x1E);
        let wy = sx(m, rec + 0x22);
        let mut found = false;
        let mut k = 0i32;
        while k < n_list as i32 {
            if u32::from(m.r16(list.wrapping_add((k as u32) << 1))) == idx {
                found = true;
                break;
            }
            k += 1;
        }
        let st = m.r16(rec + 0x134);
        if (st == 0x11 && found) || st == 0xF {
            let x = fr(wx, SCALE_X);
            let y = fr(wy, SCALE_Y);
            let l = (x as i16 as i32 as u32).wrapping_add(m.r32(this + 0x14));
            let t = (y as i16 as i32 as u32).wrapping_add(m.r32(this + 0x18));
            let tbl = m.r32(this + 0xA8);
            let off = (n as u16 as i16 as i32 as u32) << 2;
            m.w16(tbl.wrapping_add(off), x as u16);
            let tbl = m.r32(this + 0xA8);
            m.w16(tbl.wrapping_add(off).wrapping_add(2), y as u16);
            let c = m.r16(this + 0xA4);
            m.w16(this + 0xA4, c.wrapping_add(1));
            inval(m, h, rect2, l, t, l.wrapping_add(3), t.wrapping_add(3));
            n = n.wrapping_add(1);
        }
        idx = u32::from(m.r16(rec + 4));
        if (idx as i32) >= (m.r32(SHIP_COUNT) & 0xFFFF) as i32 {
            break;
        }
    }
}

/// `0x46E1D0`: dibuja las marcas de las ciudades.
fn draw_town_markers(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let scene = m.r32(SCENE);
    let selected = u32::from(m.r16(scene + 0xC324));
    let town = m.r32(OWN_TOWN);
    if town >= u32::from(m.r16(CITY_COUNT16)) {
        return;
    }
    h.call(m, TABLE_ENTRY, TABLE, &[town]);
    if m.r32(TOWN_COUNT) == 0 {
        return;
    }
    let mut i = 0u32;
    loop {
        cdecl(m, h, SET_COLOR, &[0xFFB4_0000]);
        let page = m.r32(PAGE);
        let marks = m.r32(page + 0x28CC);
        if m.r8(marks.wrapping_add(i)) != 0 {
            cdecl(m, h, SET_COLOR, &[0xFF00_00FF]);
        }
        if i == u32::from(m.r8(this + 0xDF)) {
            cdecl(m, h, SET_COLOR, &[0xFFFF_FF64]);
        }
        let p = m.r32(this + 0xC0).wrapping_add(i.wrapping_mul(12));
        let y = sx(m, p + 2).wrapping_add(m.r32(this + 0x18));
        let x = sx(m, p).wrapping_add(m.r32(this + 0x14));
        let w = |a: u32| a.wrapping_add(0);
        let _ = w;
        let ln = |m: &mut dyn Mem, h: &mut dyn Host, a: u32, b: u32, c: u32, d: u32| {
            cdecl(m, h, LINE, &[a, b, c, d]);
        };
        let (xm1, xp1, xm2, xp2, xm3, xp3) =
            (x.wrapping_sub(1), x.wrapping_add(1), x.wrapping_sub(2), x.wrapping_add(2), x.wrapping_sub(3), x.wrapping_add(3));
        ln(m, h, x, y.wrapping_sub(3), x, y.wrapping_sub(3));
        ln(m, h, xm1, y.wrapping_sub(2), xp1, y.wrapping_sub(2));
        ln(m, h, xm2, y.wrapping_sub(1), xp2, y.wrapping_sub(1));
        ln(m, h, xm3, y, xp3, y);
        ln(m, h, xm2, y.wrapping_add(1), xp2, y.wrapping_add(1));
        ln(m, h, xm1, y.wrapping_add(2), xp1, y.wrapping_add(2));
        ln(m, h, x, y.wrapping_add(3), x, y.wrapping_add(3));
        let flags = m.r32(this + 0xD4);
        if m.r8(flags.wrapping_add(i)) != 0 {
            cdecl(m, h, SET_COLOR, &[0xFFBF_BFFF]);
            ln(m, h, x, y.wrapping_sub(1), x, y.wrapping_add(1));
            ln(m, h, xm1, y, xp1, y);
        }
        let page = m.r32(PAGE);
        let vis = h.call(m, VISIBLE_CHECK, page, &[]) as u8;
        if vis == 0 && i == selected {
            cdecl(m, h, SET_COLOR, &[0xFFFF_FFFF]);
            let (ym4, yp4, xm4, xp4) = (y.wrapping_sub(4), y.wrapping_add(4), x.wrapping_sub(4), x.wrapping_add(4));
            ln(m, h, xm4, y, x, ym4);
            ln(m, h, x, ym4, xp4, y);
            ln(m, h, xp4, y, x, yp4);
            ln(m, h, x, yp4, xm4, y);
        }
        i = i.wrapping_add(1);
        if i >= m.r32(TOWN_COUNT) {
            break;
        }
    }
}

/// `0x46E3D0`: actualiza el minimapa náutico un paso.
fn update(m: &mut dyn Mem, h: &mut dyn Host, this: u32, esp: u32) {
    let rect = esp.wrapping_sub(0x20);
    let rect_b = esp.wrapping_sub(0x10);
    let list_slot = esp.wrapping_sub(0x24);
    let page = m.r32(PAGE);
    h.call(m, PAGE_REFRESH, page, &[]);
    for (cnt_off, tbl_off) in [(0xA4u32, 0xA8u32), (0xA6, 0xAC)] {
        let mut i = 0i32;
        while i < i32::from(m.r16(this + cnt_off) as i16) {
            let t = m.r32(this + tbl_off);
            let x = sx(m, t.wrapping_add((i as u32) << 2));
            let y = sx(m, t.wrapping_add((i as u32) << 2).wrapping_add(2));
            let l = m.r32(this + 0x14).wrapping_add(x);
            let tp = m.r32(this + 0x18).wrapping_add(y);
            inval(m, h, rect, l.wrapping_sub(5), tp.wrapping_sub(5), l.wrapping_add(5), tp.wrapping_add(5));
            i += 1;
        }
    }
    for off in [0xA8u32, 0xAC, 0xB0] {
        let p = m.r32(this + off);
        if p != 0 {
            free(m, h, p);
            m.w32(this + off, 0);
        }
    }
    m.w16(this + 0xA4, 0);
    m.w16(this + 0xA6, 0);
    for off in [0xA8u32, 0xAC, 0xB0] {
        let sz = (m.r32(SHIP_COUNT) & 0xFFFF) << 2;
        let p = new(m, h, sz);
        m.w32(this + off, p);
    }
    h.call(m, REBUILD_SHIP_MARKERS, this, &[]);
    // Parpadeo.
    let a = m.r32(this + 0xD8);
    if m.r32(TICK) > a {
        let a = a.wrapping_add(0x23);
        m.w32(this + 0xD8, a);
        let t = m.r32(TICK);
        if a < t {
            m.w32(this + 0xD8, t);
        }
        let step = m.r8(this + 0xDD);
        let val = m.r8(this + 0xDC);
        let sum = i32::from(val) + i32::from(step as i8);
        if sum > 0xFF {
            m.w8(this + 0xDC, 0xFF);
            m.w8(this + 0xDD, step.wrapping_neg());
        } else if sum < 0 {
            m.w8(this + 0xDC, 0);
            m.w8(this + 0xDD, step.wrapping_neg());
        } else {
            m.w8(this + 0xDC, val.wrapping_add(step));
        }
    }
    let nt = m.r32(TOWN_COUNT);
    let mut i = 0u32;
    while i < m.r32(TOWN_COUNT) {
        let p = m.r32(this + 0xC0).wrapping_add(i.wrapping_mul(12));
        if m.r8(p + 8) != 0 {
            if (m.r32(p + 4) as i32) < 0 {
                m.w8(p + 8, 0);
            }
            let p = m.r32(this + 0xC0).wrapping_add(i.wrapping_mul(12));
            let (l, t, r, b) = mark_rect(m, this, p, 2, 5);
            inval(m, h, rect, l, t, r, b);
        }
        i += 1;
    }
    let _ = nt;
    // Pulsos de evento.
    let mut i = 0i32;
    let mut off = 0u32;
    if m.r16(this + 0xC8) != 0 {
        loop {
            let tbl = m.r32(this + 0xC4);
            let p = tbl.wrapping_add(off);
            let value = m.r32(p + 4) as i32;
            let (l, t, r, b) = mark_rect(m, this, p, 2, 5);
            inval(m, h, rect, l, t, r, b);
            if value < 0 {
                h.call(m, 0x0046_F4C0, this, &[]);
                i -= 1;
                off = off.wrapping_sub(12);
            }
            i += 1;
            off = off.wrapping_add(12);
            if i >= i32::from(m.r16(this + 0xC8)) {
                break;
            }
        }
    }
    // Marcas de ciudad: se copian las anteriores, se recalculan y se invalida lo que cambió.
    let nt = m.r32(TOWN_COUNT);
    let old = new(m, h, nt);
    let marks = m.r32(this + 0xD4);
    let n = m.r32(TOWN_COUNT);
    memcpy(m, old, marks, n);
    let marks = m.r32(this + 0xD4);
    let n = m.r32(TOWN_COUNT);
    memset(m, marks, 0, n);
    let town = m.r32(OWN_TOWN);
    let ent = h.call(m, TABLE_ENTRY, TABLE, &[town]);
    let mut idx = u32::from(m.r16(ent + 0xE));
    if (idx as i32) < (m.r32(SHIP_COUNT) & 0xFFFF) as i32 {
        loop {
            let rec = idx.wrapping_mul(0x180).wrapping_add(m.r32(SHIP_TABLE));
            let tn = u32::from(m.r8(rec + 0x39));
            if tn < m.r32(TOWN_COUNT) && m.r16(rec + 0x134) < 4 {
                let marks = m.r32(this + 0xD4);
                m.w8(marks.wrapping_add(tn), 1);
            }
            idx = u32::from(m.r16(rec + 4));
            if (idx as i32) >= (m.r32(SHIP_COUNT) & 0xFFFF) as i32 {
                break;
            }
        }
    }
    let mut i = 0u32;
    let mut off = 0u32;
    while i < m.r32(TOWN_COUNT) {
        let marks = m.r32(this + 0xD4);
        let (was, now) = (m.r8(old.wrapping_add(i)), m.r8(marks.wrapping_add(i)));
        if was != now {
            if now != 0 {
                let k = (i as u16 as i16 as i32 as u32).wrapping_mul(12);
                let p = m.r32(this + 0xC0).wrapping_add(k);
                m.w32(p + 4, 0xFF);
                let p = m.r32(this + 0xC0).wrapping_add(k);
                m.w8(p + 8, 1);
                let p = m.r32(this + 0xC0).wrapping_add(k);
                let (l, t, r, b) = mark_rect(m, this, p, 2, 5);
                inval(m, h, rect_b, l, t, r, b);
            }
            let p = m.r32(this + 0xC0).wrapping_add(off);
            let (l, t, r, b) = mark_rect(m, this, p, 2, 5);
            inval(m, h, rect, l, t, r, b);
        }
        i += 1;
        off = off.wrapping_add(12);
    }
    if old != 0 {
        free(m, h, old);
    }
    let fam = m.r32(FAMILY);
    let n_list = h.call(m, FAMILY_LIST, fam, &[list_slot, 0, 0]) as i32;
    let mut k = 0i32;
    while k < n_list {
        let list = m.r32(list_slot);
        let id = u32::from(m.r16(list.wrapping_add((k as u32) << 1)));
        let rec = id.wrapping_mul(0x180).wrapping_add(m.r32(SHIP_TABLE));
        if u32::from(m.r8(rec)) == m.r32(OWN_TOWN) {
            let tn = u32::from(m.r8(rec + 0x39));
            if tn < m.r32(TOWN_COUNT) {
                let p = m.r32(this + 0xC0).wrapping_add(tn.wrapping_mul(12));
                let (l, t, r, b) = mark_rect(m, this, p, 2, 5);
                inval(m, h, rect, l, t, r, b);
                m.w8(this + 0xDE, 1);
            }
        }
        k += 1;
    }
    if m.r8(this + 0xDE) & u8::from(n_list == 0) != 0 {
        let mut i = 0u32;
        while i < m.r32(TOWN_COUNT) {
            let p = m.r32(this + 0xC0).wrapping_add(i.wrapping_mul(12));
            let (l, t, r, b) = mark_rect(m, this, p, 2, 5);
            inval(m, h, rect, l, t, r, b);
            i += 1;
        }
        m.w8(this + 0xDE, 0);
    }
    let page = m.r32(PAGE);
    let vis = h.call(m, VISIBLE_CHECK, page, &[]) as u8;
    m.w8(this + 0xBC, if vis != 0 { 1 } else { vis });
    let (e0, df) = (m.r8(this + 0xE0), m.r8(this + 0xDF));
    if e0 != df {
        if u32::from(e0) < m.r32(TOWN_COUNT) {
            let p = m.r32(this + 0xC0).wrapping_add(u32::from(e0) * 12);
            let (l, t, r, b) = mark_rect(m, this, p, 3, 7);
            inval(m, h, rect, l, t, r, b);
        }
        let df = m.r8(this + 0xDF);
        m.w8(this + 0xE0, df);
        if u32::from(df) < m.r32(TOWN_COUNT) {
            let p = m.r32(this + 0xC0).wrapping_add(u32::from(df) * 12);
            let (l, t, r, b) = mark_rect(m, this, p, 3, 7);
            inval(m, h, rect, l, t, r, b);
        }
    }
}

const RECT_VISIBLE: u32 = 0x004B_B7C0;
const SGL_TARGET_A: u32 = 0x004B_B9B0;
const SGL_TARGET_B: u32 = 0x004B_B9C0;
const SGL_FILL: u32 = 0x004B_B430;
const SGL_FRAME: u32 = 0x004B_B330;
const SGL_MODE: u32 = 0x004B_B7A0;
const DRAW_TOWN_MARKERS: u32 = 0x0046_E1D0;
const RAND_REF: u32 = 0x006C_BB40;

fn wf64(m: &mut dyn Mem, a: u32, v: f64) {
    let b = v.to_bits();
    m.w32(a, b as u32);
    m.w32(a + 4, (b >> 32) as u32);
}

/// Color `0xFF` + canal repetido, como lo arma el original con desplazamientos de 32 bits.
fn gray_pulse(a: u32) -> u32 {
    ((((a | 0xFFFF_FF00) << 8) | a) << 8) | a
}

fn line(m: &mut dyn Mem, h: &mut dyn Host, a: u32, b: u32, c: u32, d: u32) {
    cdecl(m, h, LINE, &[a, b, c, d]);
}

/// Las cinco líneas de un pulso alrededor de `(x, y)`.
fn pulse_lines(m: &mut dyn Mem, h: &mut dyn Host, x: u32, y: u32) {
    line(m, h, x, y.wrapping_sub(2), x, y.wrapping_sub(2));
    line(m, h, x.wrapping_sub(1), y.wrapping_sub(1), x.wrapping_add(1), y.wrapping_sub(1));
    line(m, h, x.wrapping_sub(2), y, x.wrapping_add(2), y);
    line(m, h, x.wrapping_sub(1), y.wrapping_add(1), x.wrapping_add(1), y.wrapping_add(1));
    line(m, h, x, y.wrapping_add(2), x, y.wrapping_add(2));
}

/// `0x46DA60`: dibuja la carta (SEH, `ret 0x10`).
fn draw(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) {
    let rect = esp.wrapping_sub(0x1C);
    let slot = esp.wrapping_sub(0x38);
    let slot8 = esp.wrapping_add(8);
    let fam = m.r32(FAMILY);
    let n_list = h.call(m, FAMILY_LIST, fam, &[slot, 0, 0]) as i32;
    let lx = m.r32(this + 0x14).wrapping_add(a[1]);
    let ty = m.r32(this + 0x18).wrapping_add(a[2]);
    let (w, hh) = (m.r32(this + 0x2C), m.r32(this + 0x30));
    m.w32(rect, lx);
    m.w32(rect + 4, ty);
    m.w32(rect + 8, lx.wrapping_add(w));
    m.w32(rect + 0xC, ty.wrapping_add(hh));
    if h.call(m, VISIBLE_CHECK, this, &[]) as u8 == 0 {
        return;
    }
    if cdecl(m, h, RECT_VISIBLE, &[rect]) == 0 {
        return;
    }
    let r1 = cdecl(m, h, SGL_TARGET_A, &[a[0]]);
    let tex = m.r32(this + 0x98);
    let r2 = cdecl(m, h, SGL_TARGET_B, &[tex]);
    cdecl(m, h, SET_COLOR, &[0xFFFF_FFFF]);
    let mut rng = p3_core::rng::LcgRandom::default();
    rng.set_seed(0);
    let v = rng.next_u32();
    let rr = m.r32(RAND_REF);
    if m.r32(rr.wrapping_add(0x7E4)) == v {
        let (x, y, ww, h2) = (m.r32(this + 0x14), m.r32(this + 0x18), m.r32(this + 0x2C), m.r32(this + 0x30));
        cdecl(m, h, SGL_FRAME, &[0, 0, x.wrapping_add(7), y.wrapping_add(8), ww.wrapping_sub(0x13), h2.wrapping_sub(0x17)]);
        h.call(m, DRAW_TOWN_MARKERS, this, &[]);
    }
    let (l, t) = (m.r32(rect), m.r32(rect + 4));
    // Barcos ajenos (azul/rojo/blanco según relación con la ciudad propia).
    let mut k = 0i32;
    while k < 2 * i32::from(m.r16(this + 0xA6) as i16) {
        let tbl = m.r32(this + 0xAC);
        let x = sx(m, tbl.wrapping_add((k as u32) << 1));
        let y = sx(m, tbl.wrapping_add((k as u32) << 1).wrapping_add(2));
        let px = x.wrapping_add(l).wrapping_sub(3);
        let py = y.wrapping_add(t).wrapping_sub(3);
        let ids = m.r32(this + 0xB0);
        let id = m.r32(ids.wrapping_add(((k >> 1) as u32) << 2));
        let rec = id.wrapping_mul(0x180).wrapping_add(m.r32(SHIP_TABLE));
        let owner = u32::from(m.r8(rec));
        let town = m.r32(OWN_TOWN);
        let flag = m.r8(rec + 0x15C);
        let col = if m.r16(rec + 0x134) == 0x12 || (flag & u8::from(owner != town)) != 0 {
            Some(0xFF00_0000u32)
        } else if owner == town {
            None
        } else {
            let ent = h.call(m, TABLE_ENTRY, TABLE, &[owner]);
            Some(if m.r16(ent + 8) != 0 { 0xFFFF_FFFF } else { 0xFFFF_0000 })
        };
        if let Some(c) = col {
            cdecl(m, h, SET_COLOR, &[c]);
        }
        cdecl(m, h, SGL_FILL, &[px, py, 5, 5]);
        k += 2;
    }
    cdecl(m, h, SET_COLOR, &[0xFF00_00FF]);
    let mut k = 0i32;
    while k < 2 * i32::from(m.r16(this + 0xA4) as i16) {
        let tbl = m.r32(this + 0xA8);
        let x = sx(m, tbl.wrapping_add((k as u32) << 1));
        let y = sx(m, tbl.wrapping_add((k as u32) << 1).wrapping_add(2));
        cdecl(m, h, SGL_FILL, &[x.wrapping_add(l).wrapping_sub(3), y.wrapping_add(t).wrapping_sub(3), 5, 5]);
        k += 2;
    }
    // Barcos de la ciudad propia en marcha (estado 0x14).
    let town = m.r32(OWN_TOWN);
    let ent = h.call(m, TABLE_ENTRY, TABLE, &[town]);
    let first = m.r16(ent + 0xE);
    if first < m.r16(SHIP_COUNT) {
        let mut rec = u32::from(first).wrapping_mul(0x180).wrapping_add(m.r32(SHIP_TABLE));
        loop {
            m.w32(slot8, rec);
            if m.r16(rec + 0x134) == 0x14 {
                let sxv = sx(m, rec + 0x1E);
                let x = fr(sxv, SCALE_X);
                let syv = sx(m, rec + 0x22);
                let y = fr(syv, SCALE_Y);
                let px = x.wrapping_add(l).wrapping_sub(3);
                let py = y.wrapping_add(t).wrapping_sub(3);
                line(m, h, px, py, px.wrapping_add(4), py.wrapping_add(4));
                line(m, h, px, py.wrapping_add(4), px.wrapping_add(4), py);
                let py1 = py.wrapping_add(1);
                line(m, h, px.wrapping_add(1), py.wrapping_add(4), px.wrapping_add(3), py1);
                line(m, h, px, py1, px.wrapping_add(3), py.wrapping_add(4));
                rec = m.r32(slot8);
            }
            let next = m.r16(rec + 4);
            if next >= m.r16(SHIP_COUNT) {
                break;
            }
            rec = u32::from(next).wrapping_mul(0x180).wrapping_add(m.r32(SHIP_TABLE));
        }
    }
    // Barcos de la lista de la familia.
    let mut i = 0i32;
    while i < n_list {
        let town = m.r32(OWN_TOWN);
        h.call(m, TABLE_ENTRY, TABLE, &[town]);
        let list = m.r32(slot);
        let id = u32::from(m.r16(list.wrapping_add((i as u32) << 1)));
        let rec = id.wrapping_mul(0x180).wrapping_add(m.r32(SHIP_TABLE));
        let town = m.r32(OWN_TOWN);
        if u32::from(m.r8(rec)) == town {
            let st = m.r16(rec + 0x134);
            if !(st > 0x14 || st == 0x13 || st == 0xE) {
                let c = u32::from(m.r8(this + 0xDC));
                let col = ((((c | 0xFFFF_8000) << 8) | c) << 8) | c;
                cdecl(m, h, SET_COLOR, &[col]);
                let tn = u32::from(m.r8(rec + 0x39));
                if tn >= m.r32(TOWN_COUNT) {
                    let sxv = sx(m, rec + 0x1E);
                    let x = fr(sxv, SCALE_X);
                    wf64(m, slot8, f64::from(sxv as i32));
                    let px = x.wrapping_add(l).wrapping_sub(3);
                    let syv = sx(m, rec + 0x22);
                    wf64(m, slot8, f64::from(syv as i32));
                    let y = fr(syv, SCALE_Y);
                    cdecl(m, h, SGL_FILL, &[px, y.wrapping_add(t).wrapping_sub(3), 5, 5]);
                } else {
                    let p = m.r32(this + 0xC0).wrapping_add(tn.wrapping_mul(12));
                    let x = sx(m, p).wrapping_add(m.r32(this + 0x14));
                    let y = sx(m, p + 2).wrapping_add(m.r32(this + 0x18));
                    pulse_lines(m, h, x, y);
                }
            }
        }
        i += 1;
    }
    // Pulsos de las ciudades (se apagan restando 10).
    let n = m.r32(TOWN_COUNT);
    if n != 0 {
        let mut i = 0u32;
        let mut off = 0u32;
        m.w32(slot8, 0);
        loop {
            let p = m.r32(this + 0xC0).wrapping_add(off);
            if m.r8(p + 8) != 0 {
                let x = sx(m, p).wrapping_add(l);
                let a8 = m.r32(p + 4) & 0xFF;
                let y = sx(m, p + 2).wrapping_add(t);
                cdecl(m, h, SET_COLOR, &[gray_pulse(a8)]);
                pulse_lines(m, h, x, y);
                let q = m.r32(this + 0xC0).wrapping_add(off).wrapping_add(4);
                let v = m.r32(q).wrapping_add(0xFFFF_FFF6);
                m.w32(q, v);
            }
            i += 1;
            off = off.wrapping_add(12);
            m.w32(slot8, off);
            if i >= m.r32(TOWN_COUNT) {
                break;
            }
        }
    }
    // Pulsos de evento.
    if m.r16(this + 0xC8) != 0 {
        let mut i = 0i32;
        let mut off = 0u32;
        loop {
            let p = m.r32(this + 0xC4).wrapping_add(off);
            let x = sx(m, p).wrapping_add(l);
            let a8 = m.r32(p + 4) & 0xFF;
            let y = sx(m, p + 2).wrapping_add(t);
            cdecl(m, h, SET_COLOR, &[gray_pulse(a8)]);
            m.w32(slot8, x.wrapping_add(1));
            pulse_lines(m, h, x, y);
            let q = m.r32(this + 0xC4).wrapping_add(off).wrapping_add(4);
            let v = m.r32(q).wrapping_add(0xFFFF_FFF6);
            m.w32(q, v);
            off = off.wrapping_add(12);
            i += 1;
            if i >= i32::from(m.r16(this + 0xC8)) {
                break;
            }
        }
    }
    // Rectángulo de la vista.
    if m.r8(this + 0xBC) != 0 {
        let page = m.r32(PAGE);
        if h.call(m, VISIBLE_CHECK, page, &[]) as u8 != 0 {
            let al = cdecl(m, h, SGL_MODE, &[3]) as u8;
            let esi = sx(m, this + 0xB4).wrapping_add(l);
            let ebx = sx(m, this + 0xB8).wrapping_add(l);
            let ebp = sx(m, this + 0xB6).wrapping_add(t);
            let edi = sx(m, this + 0xBA).wrapping_add(t);
            m.w8(slot8, al);
            cdecl(m, h, SET_COLOR, &[0xFF64_6464]);
            line(m, h, esi, ebp, ebx, ebp);
            let ebp = ebp.wrapping_add(1);
            let em1 = edi.wrapping_sub(1);
            line(m, h, esi, ebp, esi, em1);
            line(m, h, esi, edi, ebx, edi);
            line(m, h, ebx, ebp, ebx, em1);
            let prev = m.r32(slot8);
            cdecl(m, h, SGL_MODE, &[prev]);
        }
    }
    cdecl(m, h, SGL_TARGET_A, &[r1]);
    cdecl(m, h, SGL_TARGET_B, &[r2]);
}

const BASE_MOUSE: u32 = 0x004B_1720;
const FAMILY_ACTIVE: u32 = 0x0046_2310;
const FIND_OFFICE: u32 = 0x0053_08A0;
const CENTER_ON: u32 = 0x0044_F490;
const SELECT_TOWN: u32 = 0x0045_5B50;
const SCENE_SELECT: u32 = 0x0058_A4F0;
const TOWN_ACTION: u32 = 0x004A_9690;
const POS_TABLE: u32 = 0x006D_DB90;
const ACTION_ARG: u32 = 0x0066_FE90;
const ACTION_THIS: u32 = 0x006D_9584;

/// Los cuatro avisos al cierre de un clic: página `(a, b)` y escena `(c, d)` por `[vtable+0xcc]` y `[vtable+0x34]`.
fn notify(m: &mut dyn Mem, h: &mut dyn Host, page_args: u32, scene_args: u32) {
    let page = m.r32(PAGE);
    vcall(m, h, page, 0xCC, &[page_args]);
    let page = m.r32(PAGE);
    vcall(m, h, page, 0x34, &[page_args]);
    let scene = m.r32(SCENE);
    vcall(m, h, scene, 0xCC, &[scene_args]);
    let scene = m.r32(SCENE);
    vcall(m, h, scene, 0x34, &[scene_args]);
}

/// `0x46D2F0`: ratón sobre la carta (`ev` apunta a `(x, y)`; `code` 1 = pulsar, -2 = arrastrar, -3 = mover).
fn on_mouse_event(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) {
    let (ev, code) = (a[0], a[1]);
    h.call(m, BASE_MOUSE, this, &[ev, code]);
    m.w8(this + 0x94, 0);
    let nt = m.r32(TOWN_COUNT);
    let slot = esp.wrapping_sub(0x30);
    let fam = m.r32(FAMILY);
    let n = h.call(m, FAMILY_LIST, fam, &[slot, 0, 0]) as i32;
    if cdecl(m, h, FAMILY_ACTIVE, &[]) as u8 != 0 {
        return;
    }
    let mut found = 0u8;
    let mut i = 0i32;
    while i < n {
        let list = m.r32(slot);
        let id = u32::from(m.r16(list.wrapping_add((i as u32) << 1)));
        let rec = id.wrapping_mul(0x180).wrapping_add(m.r32(SHIP_TABLE));
        if u32::from(m.r8(rec)) == m.r32(OWN_TOWN) {
            found = 1;
            break;
        }
        i += 1;
    }
    let nt8 = m.r8(TOWN_COUNT);
    m.w8(this + 0xDF, nt8);
    let (mut idx, mut hit_town) = (nt, 0u32);
    if m.r32(TOWN_COUNT) != 0 {
        let (ex, ey) = (m.r32(ev) as i32, m.r32(ev + 4) as i32);
        let mut p = m.r32(this + 0xC0);
        let mut k = 0u32;
        loop {
            let t = u32::from(m.r8(TOWN_ORDER.wrapping_add(k)));
            let mx = sx(m, p).wrapping_add(m.r32(this + 0x14)) as i32;
            let my = sx(m, p + 2).wrapping_add(m.r32(this + 0x18)) as i32;
            if mx.wrapping_sub(4) < ex && mx.wrapping_add(4) > ex && my.wrapping_sub(4) < ey && my.wrapping_add(4) > ey {
                idx = k;
                hit_town = t;
                let page = m.r32(PAGE);
                let _ = sx(m, page + 0x29E4);
                let e = hit_town.wrapping_mul(52);
                let _ = m.r32(POS_TABLE.wrapping_add(e));
                let page = m.r32(PAGE);
                vcall(m, h, page, 0x70, &[]);
                let page = m.r32(PAGE);
                let _ = sx(m, page + 0x29E6);
                let _ = m.r32(POS_TABLE.wrapping_add(e).wrapping_add(4));
                break;
            }
            k += 1;
            p = p.wrapping_add(12);
            if k >= m.r32(TOWN_COUNT) {
                idx = nt;
                break;
            }
        }
    }
    let _ = hit_town;
    let mut second = 0u8;
    if idx < m.r32(TOWN_COUNT) {
        let town = m.r32(OWN_TOWN);
        let ent = h.call(m, TABLE_ENTRY, TABLE, &[town]);
        let mut di = m.r16(ent + 0xE);
        let arg0 = (TABLE & 0xFFFF_0000) | u32::from(m.r16(OWN_TOWN));
        let r = h.call(m, FIND_OFFICE, TABLE, &[arg0, idx, 0]);
        if r != 0 {
            second = 1;
        } else {
            let cnt = m.r16(SHIP_COUNT);
            if di < cnt {
                loop {
                    let rec = u32::from(di).wrapping_mul(0x180).wrapping_add(m.r32(SHIP_TABLE));
                    if u32::from(m.r8(rec + 0x39)) == idx {
                        second = 1;
                        break;
                    }
                    di = m.r16(rec + 4);
                    if di >= cnt {
                        break;
                    }
                }
            }
        }
        if found != 0 || second != 0 {
            m.w8(this + 0xDF, idx as u8);
        }
    }
    let rect = esp.wrapping_sub(0x10);
    let mouse_dx = |m: &mut dyn Mem, round: bool| -> (u32, u32) {
        let dy = f64::from((m.r32(ev + 4).wrapping_sub(m.r32(this + 0x18))) as i32);
        let dx = f64::from((m.r32(ev).wrapping_sub(m.r32(this + 0x14))) as i32);
        let (fx, fy) = if round { (dx * SCALE_X + 0.5, dy * SCALE_Y + 0.5) } else { (dx * SCALE_X, dy * SCALE_Y) };
        (ftol_i32(fx) as u32, ftol_i32(fy) as u32)
    };
    let center = |m: &mut dyn Mem, h: &mut dyn Host, round: bool| {
        let (x, y) = mouse_dx(m, round);
        let page = m.r32(PAGE);
        h.call(m, CENTER_ON, page, &[x, y]);
    };
    match code as i32 {
        1 => {
            if second != 0 {
                return;
            }
            let (l, t) = (m.r32(this + 0x14), m.r32(this + 0x18));
            let (w, hh) = (m.r32(this + 0x2C), m.r32(this + 0x30));
            inval(m, h, rect, l, t, w.wrapping_add(l), hh.wrapping_add(t));
            center(m, h, false);
            let page = m.r32(PAGE);
            if h.call(m, VISIBLE_CHECK, page, &[]) as u8 == 0 {
                notify(m, h, 1, 0);
            }
            m.w8(this + 0x94, 1);
        }
        -2 => {
            if second != 0 {
                let df = m.r8(this + 0xDF);
                if u32::from(df) >= m.r32(TOWN_COUNT) {
                    return;
                }
                let scene = m.r32(SCENE);
                let arg = (m.r32(TOWN_COUNT) & 0xFFFF_0000) | u32::from(df);
                h.call(m, SCENE_SELECT, scene, &[arg, 0, 1, 1, 1]);
                m.w8(this + 0xDF, 0xFF);
                let page = m.r32(PAGE);
                vcall(m, h, page, 0xCC, &[0]);
                let page = m.r32(PAGE);
                vcall(m, h, page, 0x34, &[0]);
                let scene = m.r32(SCENE);
                vcall(m, h, scene, 0xCC, &[1]);
                let scene = m.r32(SCENE);
                vcall(m, h, scene, 0x34, &[1]);
                return;
            }
            let page = m.r32(PAGE);
            if h.call(m, VISIBLE_CHECK, page, &[]) as u8 == 0 {
                notify(m, h, 1, 0);
            }
            center(m, h, false);
        }
        -3 => {
            let (ex, ey) = (m.r32(ev), m.r32(ev + 4));
            let dx = f64::from((ex.wrapping_sub(m.r32(this + 0x14))) as i32);
            let page = m.r32(PAGE);
            let px = ftol_i32(dx * SCALE_X + 0.5) as u32;
            let l18 = px.wrapping_sub(sx(m, page + 0x29E4));
            let v70 = vcall(m, h, page, 0x70, &[]);
            let dy = f64::from((ey.wrapping_sub(m.r32(this + 0x18))) as i32);
            let py = ftol_i32(dy * SCALE_Y + 0.5) as u32;
            let page = m.r32(PAGE);
            let mut l18 = l18;
            let mut l14 = v70.wrapping_add(py.wrapping_sub(sx(m, page + 0x29E6)));
            let hit = found != 0 && idx < m.r32(TOWN_COUNT);
            if hit {
                m.w16(page + 0x2B04, idx as u16);
                let t = u32::from(m.r8(TOWN_ORDER.wrapping_add(idx)));
                let e = t.wrapping_mul(52);
                let page = m.r32(PAGE);
                l18 = m.r32(POS_TABLE.wrapping_add(e).wrapping_add(0x20)).wrapping_sub(sx(m, page + 0x29E4));
                let v = vcall(m, h, page, 0x70, &[]);
                let page = m.r32(PAGE);
                let yy = m.r32(POS_TABLE.wrapping_add(e).wrapping_add(0x24)).wrapping_sub(sx(m, page + 0x29E6));
                l14 = v.wrapping_add(yy);
            }
            let q = esp.wrapping_sub(0x18);
            m.w32(q, l18);
            m.w32(q + 4, l14);
            let page = m.r32(PAGE);
            h.call(m, SELECT_TOWN, page, &[q]);
            if idx < m.r32(TOWN_COUNT) && found != 0 {
                let page = m.r32(PAGE);
                m.w16(page + 0x2B04, 0xFF);
            }
            if idx < m.r32(TOWN_COUNT) && found != 0 {
                let off = (idx as u16 as i16 as i32 as u32).wrapping_mul(12);
                let tbl = m.r32(this + 0xC0);
                m.w32(tbl.wrapping_add(off).wrapping_add(4), 0xFF);
                let tbl = m.r32(this + 0xC0);
                m.w8(tbl.wrapping_add(off).wrapping_add(8), 1);
                let p = m.r32(this + 0xC0).wrapping_add(off);
                let (l, t, r, b) = mark_rect(m, this, p, 2, 5);
                inval(m, h, esp.wrapping_sub(0x10), l, t, r, b);
            }
            if idx < m.r32(TOWN_COUNT) && found == 0 {
                let town = m.r32(OWN_TOWN);
                let arg0 = u32::from(m.r16(OWN_TOWN));
                let _ = town;
                if h.call(m, FIND_OFFICE, TABLE, &[arg0, idx, 0]) != 0 {
                    let town = m.r32(OWN_TOWN);
                    let a = m.r32(ACTION_ARG);
                    let t = m.r32(ACTION_THIS);
                    h.call(m, TOWN_ACTION, t, &[a, town, idx]);
                }
            }
        }
        _ => {}
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    Some(match f {
        0x0046_CE70 => Ret::U32(ctor(m, h, ecx)),
        0x0046_CF10 => Ret::U32(ctor_with_name(m, h, ecx, a[0])),
        0x0046_CFA0 => Ret::U32(dtor(m, h, ecx)),
        0x0046_D010 => {
            init_town_markers(m, h, ecx);
            Ret::None
        }
        0x0046_D140 => {
            free_marker_arrays(m, h, ecx);
            Ret::None
        }
        0x0046_D1E0 => {
            reset_state(m, h, ecx);
            Ret::None
        }
        0x0046_D930 => {
            h.call(m, SCOPE_GUARD, esp + 4, &[]);
            Ret::U8(0)
        }
        0x0046_D940 => Ret::U8(init(m, h, ecx, a, esp) as u8),
        0x0046_EB50 => Ret::U32(add_foreign_ship_marker(m, h, ecx, a, esp)),
        0x0046_EBF0 => Ret::U32(set_view_rect(m, h, ecx, a, esp)),
        0x0046_ECD0 => {
            clear_view_rect(m, h, ecx, esp);
            Ret::U32(0)
        }
        0x0046_EDE0 => Ret::None,
        0x0046_EDF0 => {
            rebuild_ship_markers(m, h, ecx, esp);
            Ret::None
        }
        0x0046_E1D0 => {
            draw_town_markers(m, h, ecx);
            Ret::None
        }
        0x0046_D2F0 => {
            on_mouse_event(m, h, ecx, a, esp);
            Ret::None
        }
        0x0046_DA60 => {
            draw(m, h, ecx, a, esp);
            Ret::None
        }
        0x0046_E3D0 => {
            update(m, h, ecx, esp);
            Ret::None
        }
        0x0046_F1B0 => Ret::U32(set_visible(m, h, ecx, a[0], esp)),
        0x0046_F340 => {
            add_event_pulse(m, h, ecx, a);
            Ret::None
        }
        0x0046_F4C0 => {
            remove_first_event_pulse(m, h, ecx);
            Ret::None
        }
        0x0046_F560 => {
            append_town_marker(m, h, ecx);
            Ret::None
        }
        0x0046_F6A0 => {
            m.w8(ecx + 0x94, 0);
            m.w8(ecx + 0x42, 0);
            Ret::U8(1)
        }
        0x0046_F6B0 => {
            let arg = m.r32(NAME_ARG);
            h.call(m, STRING_INIT, a[0], &[arg]);
            Ret::U32(a[0])
        }
        0x0046_F6E0 => {
            h.call(m, DTOR, ecx, &[]);
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        _ => return None,
    })
}

#[allow(dead_code)]
fn unused(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    vcall(m, h, this, 0, &[]);
}
