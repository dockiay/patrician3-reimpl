//! Clases de edificios de la ciudad derivadas de `TownSprite` (`0x5535F0`–`0x557330`).
//!
//! Doce clases simples solo cambian vtables, el bit de tipo de edificio que las activa y la hoja estática que devuelven. Las demás
//! añaden animaciones propias:
//! - `bit09` y `bit14`: estado agrupado (`+0x20`/`+0x24`), lista de variantes y dos (o una) tablas de conjuntos de animación estáticos,
//!   con una fase (`+0x34`/`+0x1C`) que avanza con el reloj del mundo (`0x6DE4B4`) cada `periodo << 8`.
//! - `bit0e`, `bit05` y `overlay_pair`/`bit0b`: hoja estática y conjuntos estáticos, con uno o dos animadores de fotograma
//!   (fotograma, estado, instante) que avanzan con el reloj de animación (`0x6DCCF8`); el estado usa las constantes `0x673D98`..`0x673D9B`
//!   (en espera, en marcha, terminado y reiniciar; el bit 7 indica marcha atrás).
//! - `two_variant`: dos hojas estáticas (`0x6E1290` + variante·`0x3C`) elegidas por el código de variante del fichero.

use super::sheet::{
    ANIM_BASE_CTOR, ANIM_BASE_DTOR, ANIM_BASE_RESET, ANIM_CLOCK, ANIM_SET_LOAD, ANIM_SET_RESET, ARCHIVE_CLOSE, ARCHIVE_CTOR,
    ARCHIVE_DTOR, ARCHIVE_OPEN, BURN_ANIMATE, BURN_DRAW_FRAME, BURN_DRAW_MASKED, CRT_RAND, GROUPED_STATE_DTOR, HIGHLIGHT_COLOR,
    QUEUE_BUILDING_SOUND, READER_LOAD_SHEET, READ_ARRAY_AT_24, SGL_DRAW_BITMAP_RECT, SGL_DRAW_BITMAP_RECT_ALPHA,
    SGL_GET_CONSTANT_COLOR, SGL_SET_CONSTANT_COLOR, SGL_SET_RENDER_SOURCE, SHEET_DRAW_LAYERS, SHEET_RESET, TOWN_SPRITE_CTOR,
    TOWN_SPRITE_DTOR, TOWN_VARIANT_DTOR,
};
use super::sprite::{deleting_dtor, CRT_FREE, CRT_REALLOC, GROUPED_ANIMATOR_DTOR, UI};
use super::{at32, cdecl, free, new, vcall, Host, Mem, Ret};

pub const TOWN_VARIANT_CTOR: u32 = 0x0055_3110;
pub const GROUPED_STATE_CTOR: u32 = 0x0055_2470;
pub const GROUPED_ANIMATOR_CTOR: u32 = 0x0054_EDC0;
pub const BURN_RESET: u32 = 0x0055_0490;
pub const MERCHANT_HAS_FLAG: u32 = 0x0054_F710;
pub const ANIM_SET_INVALIDATE: u32 = 0x0058_54D0;
pub const ANIM_SET_DRAW_CLIPPED: u32 = 0x0058_5550;
pub const ANIM_SET_APPEND_LOAD: u32 = 0x0058_5640;
pub const ANIM_SET_ARRAY_FREE: u32 = 0x0058_5710;
pub const ANIM_SET_LIST_RESIZE: u32 = 0x0055_3F10;
pub const ANIMATION_CTOR: u32 = 0x0058_5220;
pub const SPAWN_WALKER: u32 = 0x0059_A420;
/// Reloj del mundo (`0x6DE4B4`) y jugador local (`0x6DFC14`).
pub const WORLD_CLOCK: u32 = 0x006D_E4B4;
pub const LOCAL_PLAYER: u32 = 0x006D_FC14;
/// Códigos de las dos variantes de `two_variant` (`0x67792B`, `0x67792D`).
pub const VARIANT_CODE_A: u32 = 0x0067_792B;
pub const VARIANT_CODE_B: u32 = 0x0067_792D;
pub const TWO_VARIANT_SHEETS: u32 = 0x006E_1290;
/// Estados del animador de fotogramas: en espera, en marcha, terminado y reiniciar.
const ST_WAIT: u32 = 0x0067_3D98;
const ST_RUN: u32 = 0x0067_3D99;
const ST_DONE: u32 = 0x0067_3D9A;
const ST_RESTART: u32 = 0x0067_3D9B;

/// Clase simple: constructor y destructor sobre la base, `is_active` por bit de tipo de edificio y hoja estática.
pub struct Simple {
    pub ctor: u32,
    pub base_ctor: u32,
    pub vt0: u32,
    pub vt4: u32,
    pub dtor: u32,
    pub base_dtor: u32,
    pub is_active: u32,
    pub bit: u32,
    pub get_resource: u32,
    pub resource: u32,
    pub deleting_dtor: u32,
    pub thunk: u32,
}

#[rustfmt::skip]
pub const SIMPLE: [Simple; 12] = [
    Simple { ctor: 0x5548b0, base_ctor: 0x553110, vt0: 0x6741b8, vt4: 0x6741b4, dtor: 0x5548d0, base_dtor: 0x553130, is_active: 0x5548f0, bit: 0xf, get_resource: 0x554910, resource: 0x6e126c, deleting_dtor: 0x554920, thunk: 0x554940 },
    Simple { ctor: 0x5549f0, base_ctor: 0x553110, vt0: 0x674250, vt4: 0x67424c, dtor: 0x554a10, base_dtor: 0x553130, is_active: 0x554a30, bit: 0xa, get_resource: 0x554a50, resource: 0x6e127c, deleting_dtor: 0x554a60, thunk: 0x554a80 },
    Simple { ctor: 0x554d40, base_ctor: 0x552b50, vt0: 0x674378, vt4: 0x674374, dtor: 0x554d60, base_dtor: 0x552bb0, is_active: 0x554d80, bit: 0x6, get_resource: 0x554da0, resource: 0x6e130c, deleting_dtor: 0x554db0, thunk: 0x554dd0 },
    Simple { ctor: 0x556080, base_ctor: 0x553110, vt0: 0x674528, vt4: 0x674524, dtor: 0x5560a0, base_dtor: 0x553130, is_active: 0x5560c0, bit: 0x8, get_resource: 0x5560e0, resource: 0x6e1424, deleting_dtor: 0x5560f0, thunk: 0x556110 },
    Simple { ctor: 0x556160, base_ctor: 0x552b50, vt0: 0x6745c0, vt4: 0x6745bc, dtor: 0x556180, base_dtor: 0x552bb0, is_active: 0x5561a0, bit: 0x4, get_resource: 0x5561c0, resource: 0x6e1434, deleting_dtor: 0x5561d0, thunk: 0x5561f0 },
    Simple { ctor: 0x556240, base_ctor: 0x552b50, vt0: 0x674650, vt4: 0x67464c, dtor: 0x556260, base_dtor: 0x552bb0, is_active: 0x556280, bit: 0x13, get_resource: 0x5562a0, resource: 0x6e1474, deleting_dtor: 0x5562b0, thunk: 0x5562d0 },
    Simple { ctor: 0x556de0, base_ctor: 0x552b50, vt0: 0x674770, vt4: 0x67476c, dtor: 0x556e00, base_dtor: 0x552bb0, is_active: 0x556e20, bit: 0xd, get_resource: 0x556e40, resource: 0x6e1584, deleting_dtor: 0x556e50, thunk: 0x556e70 },
    Simple { ctor: 0x556ec0, base_ctor: 0x552b50, vt0: 0x674800, vt4: 0x6747fc, dtor: 0x556ee0, base_dtor: 0x552bb0, is_active: 0x556f00, bit: 0x11, get_resource: 0x556f20, resource: 0x6e15c4, deleting_dtor: 0x556f30, thunk: 0x556f50 },
    Simple { ctor: 0x556fa0, base_ctor: 0x552b50, vt0: 0x674890, vt4: 0x67488c, dtor: 0x556fc0, base_dtor: 0x552bb0, is_active: 0x556fe0, bit: 0xc, get_resource: 0x557000, resource: 0x6e1604, deleting_dtor: 0x557010, thunk: 0x557030 },
    Simple { ctor: 0x5570e0, base_ctor: 0x553110, vt0: 0x674920, vt4: 0x67491c, dtor: 0x557100, base_dtor: 0x553130, is_active: 0x557120, bit: 0x10, get_resource: 0x557140, resource: 0x6e1644, deleting_dtor: 0x557150, thunk: 0x557170 },
    Simple { ctor: 0x5571c0, base_ctor: 0x552b50, vt0: 0x6749b8, vt4: 0x6749b4, dtor: 0x5571e0, base_dtor: 0x552bb0, is_active: 0x557200, bit: 0x7, get_resource: 0x557220, resource: 0x6e1654, deleting_dtor: 0x557230, thunk: 0x557250 },
    Simple { ctor: 0x5572a0, base_ctor: 0x552b50, vt0: 0x674a48, vt4: 0x674a44, dtor: 0x5572c0, base_dtor: 0x552bb0, is_active: 0x5572e0, bit: 0x12, get_resource: 0x557300, resource: 0x6e1694, deleting_dtor: 0x557310, thunk: 0x557330 },
];

/// Datos estáticos de `bit09` y `bit14`: lista de variantes de hoja (número, tabla), lista de conjuntos por variante, tablas de
/// conjuntos de animación (tabla, número) y periodo.
struct Grouped {
    sheets_n: u32,
    sheets: u32,
    sets_list: u32,
    sets_n: u32,
    sets: u32,
    arrays: &'static [(u32, u32)],
    period: u32,
}

const BIT09: Grouped = Grouped {
    sheets_n: 0x006E_122C,
    sheets: 0x006E_1230,
    sets_list: 0x006E_1218,
    sets_n: 0x006E_121C,
    sets: 0x006E_1220,
    arrays: &[(0x006E_1234, 0x006E_123C), (0x006E_1238, 0x006E_1240)],
    period: 0x006C_3B18,
};
const BIT14: Grouped = Grouped {
    sheets_n: 0x006E_1258,
    sheets: 0x006E_125C,
    sets_list: 0x006E_1244,
    sets_n: 0x006E_1248,
    sets: 0x006E_124C,
    arrays: &[(0x006E_1260, 0x006E_1264)],
    period: 0x006C_3B4C,
};

/// `0x54F710` (cdecl): el comerciante bajo el cursor tiene el bit de tipo de edificio.
fn has_flag(m: &mut dyn Mem, h: &mut dyn Host, bit: u32) -> u32 {
    cdecl(m, h, MERCHANT_HAS_FLAG, &[bit])
}

/// Conjunto de animación de la variante `v` (0 si `v` queda fuera de la lista).
fn set_of(m: &mut dyn Mem, d: &Grouped, v: u32) -> u32 {
    if v < m.r32(d.sets_n) {
        at32(m, d.sets, v * 4)
    } else {
        0
    }
}

/// Vacía las hojas y conjuntos estáticos de `bit09`/`bit14` y vuelve al periodo 1.
fn grouped_release(m: &mut dyn Mem, h: &mut dyn Host, d: &Grouped) {
    let mut i = 0;
    while i < m.r32(d.sheets_n) {
        let s = at32(m, d.sheets, i * 4);
        h.call(m, SHEET_RESET, s, &[]);
        i += 1;
    }
    destroy_table(m, h, d.sheets, d.sheets_n, 0);
    let mut i = 0;
    while i < m.r32(d.sets_n) {
        let s = at32(m, d.sets, i * 4);
        vcall(m, h, s, 0, &[]);
        i += 1;
    }
    destroy_table(m, h, d.sets, d.sets_n, 4);
    for &(arr, n) in d.arrays {
        cdecl(m, h, ANIM_SET_ARRAY_FREE, &[arr, n]);
    }
    m.w32(d.period, 1);
}

/// Destruye cada objeto de la tabla `[arr]` (`count` entradas) con `vtable+slot(1)`, la libera y deja la lista vacía.
fn destroy_table(m: &mut dyn Mem, h: &mut dyn Host, arr: u32, count: u32, slot: u32) {
    if m.r32(arr) != 0 {
        let mut i = 0;
        while i < m.r32(count) {
            let o = at32(m, arr, i * 4);
            if o != 0 {
                vcall(m, h, o, slot, &[1]);
            }
            i += 1;
        }
        let t = m.r32(arr);
        cdecl(m, h, CRT_FREE, &[t]);
        m.w32(arr, 0);
    }
    m.w32(count, 0);
}

/// `0x553F10` (thiscall: número): ajusta la lista de conjuntos de animación (`+4` número, `+8` tabla), destruyendo los sobrantes con
/// `vtable+4(3)` y creando conjuntos nuevos de `0x18` bytes.
pub fn anim_set_list_resize(m: &mut dyn Mem, h: &mut dyn Host, list: u32, n: u32) {
    let count = m.r32(list + 4);
    if n == count {
        return;
    }
    if n < count {
        let mut i = n;
        while i < m.r32(list + 4) {
            let o = at32(m, list + 8, i * 4);
            if o != 0 {
                vcall(m, h, o, 4, &[3]);
            }
            i += 1;
        }
    }
    let t = m.r32(list + 8);
    let t = cdecl(m, h, CRT_REALLOC, &[t, n.wrapping_mul(4)]);
    let mut i = m.r32(list + 4);
    m.w32(list + 8, t);
    while i < n {
        let p = new(m, h, 0x18);
        let o = if p != 0 { h.call(m, ANIMATION_CTOR, p, &[]) } else { 0 };
        i += 1;
        let t = m.r32(list + 8);
        m.w32(t.wrapping_add(i.wrapping_mul(4)).wrapping_sub(4), o);
    }
    m.w32(list + 4, n);
}

/// `0x553FC0` (thiscall: banderas): destruye los conjuntos de la lista y, con el bit 0, libera la lista.
pub fn anim_set_list_deleting_dtor(m: &mut dyn Mem, h: &mut dyn Host, this: u32, flags: u32) -> u32 {
    m.w32(this, 0x0067_407C);
    destroy_table(m, h, this + 8, this + 4, 0);
    if flags & 1 != 0 {
        free(m, h, this);
    }
    this
}

/// Fase inicial aleatoria y próximo instante de `bit09` (cuerpo común de constructor, destructor y reinicio).
fn bit09_reset(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    vcall(m, h, this + 0x20, 4, &[]);
    m.w8(this + 0x34, 0);
    let v = u32::from(m.r8(this + 0x18));
    if v < m.r32(BIT09.sets_n) {
        let set = at32(m, BIT09.sets, v * 4);
        let r = cdecl(m, h, CRT_RAND, &[]) as i32;
        let c = i32::from(m.r16(set + 0x14)) + 1;
        m.w8(this + 0x34, (r.wrapping_rem(c) as u8).wrapping_sub(1));
    }
    let clock = m.r32(WORLD_CLOCK);
    let r = cdecl(m, h, CRT_RAND, &[]);
    let rem = r % m.r32(BIT09.period);
    m.w32(this + 0x1C, (rem << 8).wrapping_add(clock));
}

/// Avanza un animador de fotogramas (`frame` byte, `state` byte, `timer` instante) del conjunto `set` con el reloj de animación.
/// Devuelve el estado que deja el original en `AL` (0 = nada que redibujar).
fn step(m: &mut dyn Mem, frame: u32, state: u32, timer: u32, set: u32) -> u8 {
    let st = m.r8(state);
    if st == m.r8(ST_RESTART) {
        let v = m.r8(ST_RUN);
        m.w8(state, v);
        return m.r8(ST_RUN);
    }
    let t = m.r32(timer);
    let clock = m.r32(ANIM_CLOCK);
    if clock < t {
        let v = (st & 0x80) | m.r8(ST_WAIT);
        m.w8(state, v);
        return v;
    }
    let info = m.r32(set + 8);
    let period = m.r32(info);
    let n = ((clock - t) / period).wrapping_add(1);
    m.w32(timer, clock.wrapping_add(period));
    let fr = m.r8(frame);
    if st & 0x80 != 0 {
        if (u32::from(fr).wrapping_sub(n) as i32) < 0 {
            m.w8(frame, 0);
            let v = m.r8(ST_DONE);
            m.w8(state, v);
            return m.r8(ST_DONE);
        }
        m.w8(frame, fr.wrapping_sub(n as u8));
        let v = m.r8(ST_RUN) | 0x80;
        m.w8(state, v);
        return v;
    }
    let count = m.r16(set + 0x14);
    if u32::from(fr).wrapping_add(n) >= u32::from(count) {
        m.w8(frame, (count as u8).wrapping_sub(1));
        if m.r32(info + 4) != 0 {
            let v = m.r8(ST_RUN) | 0x80;
            m.w8(state, v);
            return v;
        }
        let v = m.r8(ST_DONE);
        m.w8(state, v);
        return m.r8(ST_DONE);
    }
    m.w8(frame, fr.wrapping_add(n as u8));
    let v = m.r8(ST_RUN);
    m.w8(state, v);
    m.r8(ST_RUN)
}

/// Si el animador terminó, vuelve al fotograma 0 con el primer intervalo de `set` y lo marca para reiniciar.
fn rewind_if_done(m: &mut dyn Mem, frame: u32, state: u32, timer: u32, set: impl Fn(&mut dyn Mem) -> u32) {
    if m.r8(state) & 0x7F == m.r8(ST_DONE) {
        m.w8(frame, 0);
        let s = set(m);
        let p = at32(m, s + 8, 0);
        let clock = m.r32(ANIM_CLOCK);
        m.w32(timer, p.wrapping_add(clock));
        let v = m.r8(ST_RESTART);
        m.w8(state, v);
    }
}

fn invalidate(m: &mut dyn Mem, h: &mut dyn Host, set: u32, a: &[u32], rect: u32, frame: u32) {
    h.call(m, ANIM_SET_INVALIDATE, set, &[a[0], a[1], rect, frame]);
}

fn draw_clipped(m: &mut dyn Mem, h: &mut dyn Host, set: u32, x: u32, y: u32, rect: u32, frame: u32) {
    h.call(m, ANIM_SET_DRAW_CLIPPED, set, &[x, y, rect, frame]);
}

/// Rectángulo del fotograma `f` en la hoja.
fn rect_of(m: &mut dyn Mem, sheet: u32, f: u32) -> u32 {
    m.r32(sheet + 0x10).wrapping_add((f & 0xFF) * 8)
}

/// Comienzo común del dibujo de un fotograma: posición `x` (más el ancho para los fotogramas extra) en `xslot`, color constante
/// guardado, color de selección si la casilla bajo el cursor es de otro jugador, incendio y, si el incendio no dibujó, el mapa de bits.
/// Devuelve el rectángulo y el color guardado; `None` si dibujó el incendio (ya restauró el color).
fn frame_start(
    m: &mut dyn Mem,
    h: &mut dyn Host,
    this: u32,
    sheet: u32,
    a: &[u32],
    xslot: u32,
    yslot: u32,
    masked: bool,
) -> Option<(u32, u32)> {
    let rect = rect_of(m, sheet, a[2]);
    let x = if (a[2] as u8) < m.r8(sheet + 0x2A) { a[0] } else { u32::from(m.r16(rect + 4)).wrapping_add(a[0]) };
    m.w32(xslot, x);
    let color = cdecl(m, h, SGL_GET_CONSTANT_COLOR, &[]);
    let ui = m.r32(UI);
    if m.r8(ui + 0x564) != 0 {
        let cell = (m.r32(ui + 0x55C).wrapping_sub(m.r32(ui + 0x560)) as i32 >> 2) as u32;
        let owners = m.r32(ui + 0xC310);
        let owner = m.r8(owners.wrapping_add(cell.wrapping_mul(4)).wrapping_add(3));
        if u32::from(owner) != m.r32(LOCAL_PLAYER) {
            let c = m.r32(HIGHLIGHT_COLOR);
            cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[c]);
        }
    }
    let burn = if masked { BURN_DRAW_MASKED } else { BURN_DRAW_FRAME };
    if h.call(m, burn, this + 4, &[sheet, rect, xslot, yslot]) as u8 != 0 {
        cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color]);
        return None;
    }
    let src = m.r32(sheet + 0xC);
    cdecl(m, h, SGL_SET_RENDER_SOURCE, &[src]);
    let (y, x) = (m.r32(yslot), m.r32(xslot));
    let hh = u32::from(m.r16(rect + 6));
    let w = u32::from(m.r16(rect + 4));
    let (rx, ry) = (u32::from(m.r16(rect)), u32::from(m.r16(rect + 2)));
    if masked {
        let mask = m.r32(sheet + 4);
        cdecl(m, h, SGL_DRAW_BITMAP_RECT_ALPHA, &[mask, rx, ry, x, y.wrapping_sub(hh), w, hh]);
    } else {
        cdecl(m, h, SGL_DRAW_BITMAP_RECT, &[rx, ry, x, y.wrapping_sub(hh), w, hh]);
    }
    Some((rect, color))
}

/// `0x5538D0` / `0x5543A0` (thiscall: x, y, fotograma): fotograma de la hoja de la variante y, si el edificio está activo, la
/// animación de la variante y la del estado agrupado.
fn grouped_draw_frame(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32, b14: bool) {
    let s = vcall(m, h, this, 0x74, &[]);
    let (xs, ys) = (esp - 4, esp + 8);
    let Some((rect, color)) = frame_start(m, h, this, s, a, xs, ys, false) else { return };
    let d = if b14 { &BIT14 } else { &BIT09 };
    if b14 {
        if m.r32(0x006E_1264) == 0 {
            return;
        }
    } else if m.r32(0x006E_1240) == 0 || m.r32(0x006E_123C) == 0 {
        return;
    }
    let v = u32::from(m.r8(this + 0x18));
    let set = if b14 {
        set_of(m, d, v)
    } else {
        if v >= m.r32(d.sets_n) {
            return;
        }
        at32(m, d.sets, v * 4)
    };
    if m.r32(set.wrapping_add(4)) == 0 {
        return;
    }
    let (phase, state, bit) = if b14 { (0x1C, 0x30, 0x14) } else { (0x34, 0x2C, 9) };
    let ui = m.r32(UI);
    if m.r32(ui + 0x4F0) != 0 && has_flag(m, h, bit) as u8 != 0 {
        let ph = m.r8(this + phase);
        if ph != 0xFF {
            let v = u32::from(m.r8(this + 0x18));
            let set = set_of(m, d, v);
            let (x, y) = (m.r32(xs), m.r32(ys));
            draw_clipped(m, h, set, x, y, rect, ph as i8 as u32);
        }
        let st = m.r8(this + state);
        if st & 0x40 == 0 {
            let idx = u32::from(st & 0x3F);
            let (frame, tab) = if b14 {
                (u32::from(m.r8(this + 0x2C)), 0x006E_1260)
            } else {
                let tab = if (m.r8(this + 0x34) as i8) < 0 { 0x006E_1234 } else { 0x006E_1238 };
                (u32::from(m.r8(this + 0x28)), tab)
            };
            let (x, y) = (m.r32(xs), m.r32(ys));
            let set = at32(m, tab, idx * 4);
            draw_clipped(m, h, set, x, y, rect, frame);
        }
    }
    cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color]);
}

/// Instante siguiente de la fase de `bit09`/`bit14`: `periodo << 8` sobre el reloj del mundo.
fn next_phase_time(m: &mut dyn Mem, this: u32, timer: u32, period: u32) {
    let p = m.r32(period);
    let clock = m.r32(WORLD_CLOCK);
    m.w32(this + timer, (p << 8).wrapping_add(clock));
}

/// `0x553AB0` (thiscall: x, y, fotograma): incendio y, si el edificio está activo, avance de la fase y del estado agrupado de `bit09`.
fn bit09_draw_animated(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) {
    let s = vcall(m, h, this, 0x74, &[]);
    if h.call(m, BURN_ANIMATE, this + 4, &[a[0], a[1], a[2], s]) as u8 != 0 {
        return;
    }
    let d = &BIT09;
    if m.r32(0x006E_1240) == 0 || m.r32(0x006E_123C) == 0 {
        return;
    }
    let v = u32::from(m.r8(this + 0x18));
    if v >= m.r32(d.sets_n) {
        return;
    }
    let set = at32(m, d.sets, v * 4);
    if m.r32(set.wrapping_add(4)) == 0 {
        return;
    }
    let rect = rect_of(m, s, a[2]);
    let group = this + 0x20;
    let ui = m.r32(UI);
    if m.r32(ui + 0x4F0) == 0 || has_flag(m, h, 9) as u8 == 0 {
        if m.r8(this + 0x2C) & 0x40 != 0 && m.r8(this + 0x34) == 0xFF {
            return;
        }
        let tab = if (m.r8(this + 0x34) as i8) < 0 { m.r32(0x006E_1234) } else { m.r32(0x006E_1238) };
        vcall(m, h, group, 8, &[tab, a[0], a[1], a[2], s]);
        let first = if m.r32(d.sets_n) > 0 { at32(m, d.sets, 0) } else { 0 };
        invalidate(m, h, first, a, rect, 0);
        m.w8(this + 0x34, 0xFF);
        return;
    }
    let v = m.r8(this + 0x18);
    let set = set_of(m, d, u32::from(v));
    let ph = m.r8(this + 0x34);
    if v != 0 {
        if ph != 0xFF && m.r32(WORLD_CLOCK) < m.r32(this + 0x1C) {
            return;
        }
        m.w8(this + 0x34, ph.wrapping_add(1));
        invalidate(m, h, set, a, rect, 0);
        if i32::from(m.r8(this + 0x34) as i8) >= i32::from(m.r16(set + 0x14)) {
            m.w8(this + 0x34, 0);
        }
        next_phase_time(m, this, 0x1C, d.period);
        return;
    }
    if ph == 0xFF {
        let (t0, t1) = (m.r32(0x006E_1234), m.r32(0x006E_123C));
        vcall(m, h, group, 0xC, &[t0, t1, a[0], a[1], a[2], s, 1]);
        let st = m.r8(this + 0x2C);
        if (st as i8) >= 0 || u32::from(st & 0x3F) != m.r32(0x006E_123C).wrapping_sub(2) {
            return;
        }
        m.w8(this + 0x34, 0);
        invalidate(m, h, set, a, rect, 0);
        next_phase_time(m, this, 0x1C, d.period);
        vcall(m, h, group, 4, &[]);
        return;
    }
    if i32::from(ph as i8) == i32::from(m.r16(set + 0x14)) - 2 {
        if m.r32(WORLD_CLOCK) < m.r32(this + 0x1C) {
            return;
        }
        let (t0, t1) = (m.r32(0x006E_1238), m.r32(0x006E_1240));
        vcall(m, h, group, 0xC, &[t0, t1, a[0], a[1], a[2], s, 1]);
        if u32::from(m.r8(this + 0x2C) & 0x3F) != m.r32(0x006E_1240).wrapping_sub(1) {
            return;
        }
        let ph = m.r8(this + 0x34).wrapping_add(1);
        m.w8(this + 0x34, ph);
        invalidate(m, h, set, a, rect, ph as i8 as u32);
        next_phase_time(m, this, 0x1C, d.period);
        return;
    }
    if m.r8(this + 0x2C) & 0x40 == 0 {
        let (t0, t1) = (m.r32(0x006E_1238), m.r32(0x006E_1240));
        vcall(m, h, group, 0xC, &[t0, t1, a[0], a[1], a[2], s, 1]);
        if (m.r8(this + 0x2C) as i8) < 0 {
            vcall(m, h, group, 4, &[]);
        }
    }
    if m.r32(WORLD_CLOCK) < m.r32(this + 0x1C) {
        return;
    }
    let ph = m.r8(this + 0x34).wrapping_add(1);
    m.w8(this + 0x34, ph);
    invalidate(m, h, set, a, rect, 0);
    if i32::from(m.r8(this + 0x34) as i8) >= i32::from(m.r16(set + 0x14)) {
        m.w8(this + 0x34, 0xFF);
    }
    next_phase_time(m, this, 0x1C, d.period);
}

/// `0x554560` (thiscall: x, y, fotograma): incendio y avance de la fase de `bit14`.
fn bit14_draw_animated(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) {
    let s = vcall(m, h, this, 0x74, &[]);
    if h.call(m, BURN_ANIMATE, this + 4, &[a[0], a[1], a[2], s]) as u8 != 0 {
        return;
    }
    let d = &BIT14;
    if m.r32(0x006E_1264) == 0 {
        return;
    }
    let v = u32::from(m.r8(this + 0x18));
    let set = set_of(m, d, v);
    if m.r32(set.wrapping_add(4)) == 0 {
        return;
    }
    let rect = rect_of(m, s, a[2]);
    let group = this + 0x24;
    let ui = m.r32(UI);
    if m.r32(ui + 0x4F0) == 0 || has_flag(m, h, 0x14) as u8 == 0 {
        if m.r8(this + 0x30) & 0x40 != 0 {
            return;
        }
        let t0 = m.r32(0x006E_1260);
        vcall(m, h, group, 8, &[t0, a[0], a[1], a[2], s]);
        let v = u32::from(m.r8(this + 0x18));
        let set = set_of(m, d, v);
        let ph = m.r8(this + 0x1C) as i8 as u32;
        invalidate(m, h, set, a, rect, ph);
        m.w8(this + 0x1C, 0xFF);
        return;
    }
    if m.r8(this + 0x18) == 0 {
        let (t0, t1) = (m.r32(0x006E_1260), m.r32(0x006E_1264));
        vcall(m, h, group, 0xC, &[t0, t1, a[0], a[1], a[2], s, 1]);
    }
    if m.r32(WORLD_CLOCK) < m.r32(this + 0x20) && m.r8(this + 0x1C) != 0xFF {
        return;
    }
    let ph = m.r8(this + 0x1C).wrapping_add(1);
    m.w8(this + 0x1C, ph);
    let v = u32::from(m.r8(this + 0x18));
    let set = set_of(m, d, v);
    if i32::from(ph as i8) >= i32::from(m.r16(set.wrapping_add(0x14))) {
        m.w8(this + 0x1C, 0);
    }
    let ph = m.r8(this + 0x1C) as i8 as u32;
    invalidate(m, h, set, a, rect, ph);
    next_phase_time(m, this, 0x20, d.period);
}

/// `parse_extra_chunk` de `bit09`/`bit14` (thiscall: cursor, contexto): etiqueta 3 = lista de conjuntos y animación de la variante,
/// 4 y 5 = tablas de conjuntos.
fn grouped_parse_extra(m: &mut dyn Mem, h: &mut dyn Host, this: u32, cur: u32, ctx: u32, b14: bool) -> u8 {
    let d = if b14 { &BIT14 } else { &BIT09 };
    let p = m.r32(cur);
    let tag = m.r32(p);
    m.w32(cur, p.wrapping_add(4));
    let tables: &[(u32, u32)] = if b14 { &[(4, 0)] } else { &[(5, 1), (4, 0)] };
    if tag == 3 {
        let n = m.r32(d.sheets_n);
        h.call(m, ANIM_SET_LIST_RESIZE, d.sets_list, &[n]);
        let v = u32::from(m.r8(this + 0x18));
        let set = set_of(m, d, v);
        let file = m.r32(ctx + 0xC);
        h.call(m, ANIM_SET_LOAD, set, &[cur, file, u32::MAX]);
        return 1;
    }
    for &(t, k) in tables {
        if tag == t {
            let (arr, n) = d.arrays[k as usize];
            let file = m.r32(ctx + 0xC);
            cdecl(m, h, ANIM_SET_APPEND_LOAD, &[arr, n, cur, file]);
            return 1;
        }
    }
    0
}

/// `parse_config_chunk` de `bit09`/`bit14` (cursor, estado agrupado): bandera de bucle y periodo.
fn grouped_parse_config(m: &mut dyn Mem, cur: u32, group: u32, period: u32) -> u8 {
    let p = m.r32(cur).wrapping_add(4);
    m.w32(cur, p);
    let flag = u8::from(m.r32(p) != 0);
    m.w8(group + 0x2C, flag);
    let p = m.r32(cur).wrapping_add(4);
    m.w32(cur, p);
    let v = m.r32(p);
    m.w32(period, v);
    let p = m.r32(cur).wrapping_add(8);
    m.w32(cur, p);
    1
}

/// `0x554B70` (thiscall: ruta): lee el código de variante del fichero y carga la hoja de esa variante.
fn two_variant_load(m: &mut dyn Mem, h: &mut dyn Host, this: u32, path: u32, esp: u32) -> u8 {
    let file = esp - 0x14;
    h.call(m, ARCHIVE_CTOR, file, &[]);
    if h.call(m, ARCHIVE_OPEN, file, &[path, 0x8000]) == 0 {
        h.call(m, ARCHIVE_DTOR, file, &[]);
        return 0;
    }
    let (arrp, countp) = (esp - 0x1C, esp - 0x18);
    m.w32(arrp, 0);
    m.w32(countp, 0);
    cdecl(m, h, READ_ARRAY_AT_24, &[file, arrp, countp]);
    h.call(m, ARCHIVE_CLOSE, file, &[]);
    let arr = m.r32(arrp);
    let v = u8::from(m.r8(arr.wrapping_add(4)) != m.r8(VARIANT_CODE_A));
    m.w8(this + 0x18, v);
    free(m, h, arr);
    let sheet = TWO_VARIANT_SHEETS + u32::from(m.r8(this + 0x18)) * 0x3C;
    let ok = h.call(m, READER_LOAD_SHEET, this, &[path, sheet]) as u8;
    h.call(m, ARCHIVE_DTOR, file, &[]);
    ok
}

/// `bit0e`: hoja `0x6E1364`, tablas de conjuntos `0x6E13A0` (número `0x6E13A8`) y `0x6E13A4` (`0x6E13AC`), conjunto `0x6E1348`.
const SHEET_0E: u32 = 0x006E_1364;
const SET_0E: u32 = 0x006E_1348;

fn bit0e_reset(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    h.call(m, ANIM_BASE_RESET, this + 0x18, &[]);
    m.w8(this + 0x24, 0);
    h.call(m, ANIM_BASE_RESET, this + 0x3C, &[]);
    vcall(m, h, this + 0x28, 4, &[])
}

fn bit0e_draw_frame(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) {
    let (xs, ys) = (esp + 4, esp + 8);
    let Some((rect, color)) = frame_start(m, h, this, SHEET_0E, a, xs, ys, false) else { return };
    if m.r32(0x006E_13AC) == 0 || m.r32(0x006E_13A8) == 0 || m.r32(0x006E_134C) == 0 {
        return;
    }
    if m.r8(this + 0x34) & 0x40 == 0 {
        let (x, y) = (m.r32(xs), m.r32(ys));
        let f = u32::from(m.r8(this + 0x20));
        let set = { let i = u32::from(m.r8(this + 0x24)) * 4; at32(m, 0x006E_13A4, i) };
        draw_clipped(m, h, set, x, y, rect, f);
        let (x, y) = (m.r32(xs), m.r32(ys));
        let f = u32::from(m.r8(this + 0x30));
        let set = { let i = u32::from(m.r8(this + 0x34) & 0x3F) * 4; at32(m, 0x006E_13A0, i) };
        draw_clipped(m, h, set, x, y, rect, f);
        let (x, y) = (m.r32(xs), m.r32(ys));
        let f = u32::from(m.r8(this + 0x44));
        draw_clipped(m, h, SET_0E, x, y, rect, f);
    }
    cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color]);
}

fn bit0e_draw_animated(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) {
    if h.call(m, BURN_ANIMATE, this + 4, &[a[0], a[1], a[2], SHEET_0E]) as u8 != 0 {
        return;
    }
    if m.r32(0x006E_13AC) == 0 || m.r32(0x006E_13A8) == 0 || m.r32(0x006E_134C) == 0 {
        return;
    }
    let fl = has_flag(m, h, 0xE) as u8;
    if fl != 0 {
        let snd = vcall(m, h, this, 0x50, &[]);
        let ui = m.r32(UI);
        h.call(m, QUEUE_BUILDING_SOUND, ui, &[a[0], a[1], snd]);
    }
    let rect_at = |m: &mut dyn Mem| m.r32(0x006E_1374).wrapping_add((a[2] & 0xFF) * 8);
    let ui = m.r32(UI);
    if m.r32(ui + 0x4F0) == 0 || fl == 0 {
        if m.r8(this + 0x34) & 0x40 != 0 {
            return;
        }
        let t = m.r32(0x006E_13A0);
        vcall(m, h, this + 0x28, 8, &[t, a[0], a[1], a[2], SHEET_0E]);
        let f = u32::from(m.r8(this + 0x20));
        let rect = rect_at(m);
        let set = { let i = u32::from(m.r8(this + 0x24)) * 4; at32(m, 0x006E_13A4, i) };
        invalidate(m, h, set, a, rect, f);
        let f = u32::from(m.r8(this + 0x44));
        let rect = rect_at(m);
        invalidate(m, h, SET_0E, a, rect, f);
        return;
    }
    let (t0, t1) = (m.r32(0x006E_13A0), m.r32(0x006E_13A8));
    vcall(m, h, this + 0x28, 0xC, &[t0, t1, a[0], a[1], a[2], SHEET_0E, 1]);
    if m.r8(this + 0x34) & 0x3F == 5 && m.r8(this + 0x30) == 0 {
        let f = u32::from(m.r8(this + 0x20));
        let rect = rect_at(m);
        let set = { let i = u32::from(m.r8(this + 0x24)) * 4; at32(m, 0x006E_13A4, i) };
        invalidate(m, h, set, a, rect, f);
        m.w8(this + 0x24, 1);
        let rect = rect_at(m);
        let f = u32::from(m.r8(this + 0x20));
        let set = at32(m, 0x006E_13A4, 4);
        invalidate(m, h, set, a, rect, f);
        let set = { let i = u32::from(m.r8(this + 0x24)) * 4; at32(m, 0x006E_13A4, i) };
        m.w8(this + 0x20, 0);
        let p = at32(m, set + 8, 0);
        let clock = m.r32(ANIM_CLOCK);
        m.w32(this + 0x1C, p.wrapping_add(clock));
        let v = m.r8(ST_RESTART);
        m.w8(this + 0x21, v);
    } else {
        let set = { let i = u32::from(m.r8(this + 0x24)) * 4; at32(m, 0x006E_13A4, i) };
        if step(m, this + 0x20, this + 0x21, this + 0x1C, set) != 0 {
            if m.r8(this + 0x21) & 0x7F == m.r8(ST_DONE) {
                m.w8(this + 0x24, 0);
                let set0 = at32(m, 0x006E_13A4, 0);
                m.w8(this + 0x20, 0);
                let p = at32(m, set0 + 8, 0);
                let clock = m.r32(ANIM_CLOCK);
                m.w32(this + 0x1C, p.wrapping_add(clock));
                let v = m.r8(ST_RESTART);
                m.w8(this + 0x21, v);
            }
            let f = u32::from(m.r8(this + 0x20));
            let rect = rect_at(m);
            let set = { let i = u32::from(m.r8(this + 0x24)) * 4; at32(m, 0x006E_13A4, i) };
            invalidate(m, h, set, a, rect, f);
        }
    }
    if step(m, this + 0x44, this + 0x45, this + 0x40, SET_0E) != 0 {
        rewind_if_done(m, this + 0x44, this + 0x45, this + 0x40, |_| SET_0E);
        let f = u32::from(m.r8(this + 0x44));
        let rect = rect_at(m);
        invalidate(m, h, SET_0E, a, rect, f);
    }
}

/// `parse_extra_chunk` de los conjuntos estáticos con `anim_set_load` (cursor, contexto): `(etiqueta, conjunto)`.
fn static_parse_extra(m: &mut dyn Mem, h: &mut dyn Host, cur: u32, ctx: u32, loads: &[(u32, u32)], appends: &[(u32, u32, u32)]) -> u8 {
    let p = m.r32(cur);
    let tag = m.r32(p);
    m.w32(cur, p.wrapping_add(4));
    for &(t, set) in loads {
        if tag == t {
            let file = m.r32(ctx + 0xC);
            h.call(m, ANIM_SET_LOAD, set, &[cur, file, u32::MAX]);
            return 1;
        }
    }
    for &(t, arr, n) in appends {
        if tag == t {
            let file = m.r32(ctx + 0xC);
            cdecl(m, h, ANIM_SET_APPEND_LOAD, &[arr, n, cur, file]);
            return 1;
        }
    }
    0
}

/// `redraw_region` (thiscall: x, y): todas las capas de la hoja estática y, si `show`, el último fotograma del conjunto.
fn redraw_region(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], sheet: u32, set: u32, frame: u32, show: bool) {
    cdecl(m, h, SHEET_DRAW_LAYERS, &[a[0], a[1], sheet, 0]);
    if show {
        let last = u32::from(m.r8(sheet + 0x2A));
        let rect = m.r32(sheet + 0x10).wrapping_add(last * 8).wrapping_sub(8);
        h.call(m, ANIM_SET_INVALIDATE, set, &[a[0], a[1], rect, frame]);
    }
}

/// `bit05`: hoja `0x6E13E4`, conjuntos `0x6E13B0` (efecto) y `0x6E13C8`, figura de la interfaz en `+0x30`.
const SHEET_05: u32 = 0x006E_13E4;
const SET_05A: u32 = 0x006E_13B0;
const SET_05B: u32 = 0x006E_13C8;

fn bit05_drop_walker(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let w = m.r32(this + 0x30);
    if w != 0 {
        let ui = m.r32(UI) + 0x3F8;
        vcall(m, h, ui, 4, &[w]);
        m.w32(this + 0x30, 0);
    }
}

fn bit05_draw_frame(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) {
    let (xs, ys) = (esp + 4, esp + 8);
    let Some((rect, color)) = frame_start(m, h, this, SHEET_05, a, xs, ys, false) else { return };
    if m.r32(SET_05A + 4) == 0 || m.r32(SET_05B + 4) == 0 {
        return;
    }
    if m.r8(this + 0x21) & 0x7F != m.r8(ST_DONE) {
        let (x, y) = (m.r32(xs), m.r32(ys));
        let f = u32::from(m.r8(this + 0x20));
        draw_clipped(m, h, SET_05A, x, y, rect, f);
        let (x, y) = (m.r32(xs), m.r32(ys));
        let f = u32::from(m.r8(this + 0x2C));
        draw_clipped(m, h, SET_05B, x, y, rect, f);
    }
    cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color]);
}

fn bit05_draw_animated(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) {
    if h.call(m, BURN_ANIMATE, this + 4, &[a[0], a[1], a[2], SHEET_05]) as u8 != 0 {
        return;
    }
    let fl = has_flag(m, h, 5) as u8;
    if fl != 0 {
        let snd = vcall(m, h, this, 0x50, &[]);
        let ui = m.r32(UI);
        h.call(m, QUEUE_BUILDING_SOUND, ui, &[a[0], a[1], snd]);
    }
    if m.r32(SET_05A + 4) == 0 || m.r32(SET_05B + 4) == 0 {
        return;
    }
    let rect_at = |m: &mut dyn Mem| m.r32(SHEET_05 + 0x10).wrapping_add((a[2] & 0xFF) * 8);
    let ui = m.r32(UI);
    if m.r32(ui + 0x4F0) == 0 || fl == 0 {
        if m.r8(this + 0x21) & 0x7F == m.r8(ST_DONE) {
            return;
        }
        let f = u32::from(m.r8(this + 0x20));
        let rect = rect_at(m);
        invalidate(m, h, SET_05A, a, rect, f);
        m.w32(this + 0x1C, 0);
        let last = m.r8(SET_05A + 0x14).wrapping_sub(1);
        m.w8(this + 0x20, last);
        let v = m.r8(ST_DONE);
        m.w8(this + 0x21, v);
        let f = u32::from(m.r8(this + 0x2C));
        let rect = rect_at(m);
        invalidate(m, h, SET_05B, a, rect, f);
        bit05_drop_walker(m, h, this);
        return;
    }
    if m.r32(this + 0x30) == 0 && m.r8(this + 0x21) & 0x7F == m.r8(ST_DONE) {
        let ui = m.r32(UI);
        let (px, py) = (m.r32(ui + 0xC650), m.r32(ui + 0xC654));
        let r = vcall(m, h, ui, 0x70, &[]);
        let y = py.wrapping_add(a[1]).wrapping_sub(r);
        let x = px.wrapping_add(a[0]);
        let ui = m.r32(UI);
        let w = h.call(m, SPAWN_WALKER, ui, &[x, y]);
        m.w32(this + 0x30, w);
    }
    if step(m, this + 0x20, this + 0x21, this + 0x1C, SET_05A) != 0 {
        rewind_if_done(m, this + 0x20, this + 0x21, this + 0x1C, |_| SET_05A);
        let f = u32::from(m.r8(this + 0x20));
        let rect = rect_at(m);
        invalidate(m, h, SET_05A, a, rect, f);
    }
    if step(m, this + 0x2C, this + 0x2D, this + 0x28, SET_05B) != 0 {
        rewind_if_done(m, this + 0x2C, this + 0x2D, this + 0x28, |_| SET_05B);
        let f = u32::from(m.r8(this + 0x2C));
        let rect = rect_at(m);
        invalidate(m, h, SET_05B, a, rect, f);
    }
}

/// `overlay_pair`/`bit0b`: hoja `0x6E1544` y seis conjuntos de `0x18` bytes desde `0x6E14B0`; `+0x30`/`+0x31` eligen el conjunto de
/// cada animador (`0xFF` = ninguno).
const SHEET_0B: u32 = 0x006E_1544;
const SETS_0B: u32 = 0x006E_14B0;

fn set_0b(k: u8) -> u32 {
    SETS_0B + u32::from(k) * 0x18
}

fn overlay_draw_frame(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) {
    let (xs, ys) = (esp + 4, esp + 8);
    let Some((rect, color)) = frame_start(m, h, this, SHEET_0B, a, xs, ys, false) else { return };
    if m.r32(set_0b(3) + 4) == 0 {
        return;
    }
    let k = m.r8(this + 0x30);
    if k != 0xFF {
        let (x, y) = (m.r32(xs), m.r32(ys));
        let f = u32::from(m.r8(this + 0x20));
        draw_clipped(m, h, set_0b(k), x, y, rect, f);
    }
    let k = m.r8(this + 0x31);
    let (x, y) = (m.r32(xs), m.r32(ys));
    if k != 0xFF {
        let f = u32::from(m.r8(this + 0x2C));
        draw_clipped(m, h, set_0b(k), x, y, rect, f);
    } else {
        let last = u32::from(m.r16(set_0b(3) + 0x14)).wrapping_sub(1);
        draw_clipped(m, h, set_0b(3), x, y, rect, last);
    }
    cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color]);
}

fn bit0b_draw_frame_masked(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) {
    if let Some((_, color)) = frame_start(m, h, this, SHEET_0B, a, esp + 4, esp + 8, true) {
        cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color]);
    }
}

/// Programa el animador (`frame`, `state`, `timer`) para empezar el conjunto `k` en el próximo intervalo.
fn start_set(m: &mut dyn Mem, frame: u32, state: u32, timer: u32, k: u8) {
    m.w8(frame, 0);
    let p = at32(m, set_0b(k) + 8, 0);
    let clock = m.r32(ANIM_CLOCK);
    m.w32(timer, p.wrapping_add(clock));
    let v = m.r8(ST_RESTART);
    m.w8(state, v);
}

fn bit0b_draw_animated(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) {
    if h.call(m, BURN_ANIMATE, this + 4, &[a[0], a[1], a[2], SHEET_0B]) as u8 != 0 {
        return;
    }
    if m.r32(SETS_0B + 4) == 0 {
        return;
    }
    let fl = has_flag(m, h, 0xB) as u8;
    if fl != 0 {
        let snd = vcall(m, h, this, 0x50, &[]);
        let ui = m.r32(UI);
        h.call(m, QUEUE_BUILDING_SOUND, ui, &[a[0], a[1], snd]);
    }
    let rect_at = |m: &mut dyn Mem| m.r32(SHEET_0B + 0x10).wrapping_add((a[2] & 0xFF) * 8);
    let ui = m.r32(UI);
    if m.r32(ui + 0x4F0) != 0 && fl != 0 {
        if m.r8(this + 0x30) == 0xFF && m.r32(this + 0x1C) <= m.r32(ANIM_CLOCK) {
            let r = (cdecl(m, h, CRT_RAND, &[]) as i32).wrapping_rem(20);
            if r < 1 {
                m.w8(this + 0x30, 0);
                start_set(m, this + 0x20, this + 0x21, this + 0x1C, 0);
            } else if r < 2 && m.r8(this + 0x31) == 5 {
                m.w8(this + 0x30, 1);
                start_set(m, this + 0x20, this + 0x21, this + 0x1C, 1);
            } else {
                let p = at32(m, set_0b(0) + 8, 0);
                let clock = m.r32(ANIM_CLOCK);
                m.w32(this + 0x1C, clock.wrapping_add(p.wrapping_mul(20)));
            }
        }
        if m.r8(this + 0x31) == 0xFF && m.r32(this + 0x28) <= m.r32(ANIM_CLOCK) {
            let r = (cdecl(m, h, CRT_RAND, &[]) as i32).wrapping_rem(10);
            if r < 8 {
                m.w8(this + 0x31, 2);
                start_set(m, this + 0x2C, this + 0x2D, this + 0x28, 2);
            } else {
                let p = at32(m, set_0b(2) + 8, 0);
                let clock = m.r32(ANIM_CLOCK);
                m.w32(this + 0x28, clock.wrapping_add(p.wrapping_mul(20)));
            }
        }
    } else {
        for (sel, frame) in [(0x30, 0x20), (0x31, 0x2C)] {
            let k = m.r8(this + sel);
            if k != 0xFF {
                let f = u32::from(m.r8(this + frame));
                let rect = rect_at(m);
                invalidate(m, h, set_0b(k), a, rect, f);
                m.w8(this + sel, 0xFF);
            }
        }
    }
    let k = m.r8(this + 0x30);
    if k != 0xFF && step(m, this + 0x20, this + 0x21, this + 0x1C, set_0b(k)) != 0 {
        let f = u32::from(m.r8(this + 0x20));
        let rect = rect_at(m);
        let k = m.r8(this + 0x30);
        invalidate(m, h, set_0b(k), a, rect, f);
        if m.r8(this + 0x21) & 0x7F == m.r8(ST_DONE) {
            m.w8(this + 0x30, 0xFF);
        }
    }
    let k = m.r8(this + 0x31);
    if k == 0xFF || step(m, this + 0x2C, this + 0x2D, this + 0x28, set_0b(k)) == 0 {
        return;
    }
    let f = u32::from(m.r8(this + 0x2C));
    let rect = rect_at(m);
    let k = m.r8(this + 0x31);
    invalidate(m, h, set_0b(k), a, rect, f);
    if m.r8(this + 0x2D) & 0x7F != m.r8(ST_DONE) {
        return;
    }
    match m.r8(this + 0x31) {
        2 | 5 => m.w8(this + 0x31, 4),
        3 => m.w8(this + 0x31, 0xFF),
        4 => {
            if m.r8(this + 0x30) == 1 {
                m.w8(this + 0x31, 3);
            } else {
                let r = (cdecl(m, h, CRT_RAND, &[]) as i32).wrapping_rem(12);
                if r < 1 {
                    m.w8(this + 0x31, 3);
                } else if r > 10 {
                    m.w8(this + 0x31, 5);
                }
            }
        }
        _ => {}
    }
    let k = m.r8(this + 0x31);
    if k != 0xFF {
        start_set(m, this + 0x2C, this + 0x2D, this + 0x28, k);
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    for s in &SIMPLE {
        let r = if f == s.ctor {
            h.call(m, s.base_ctor, ecx, &[]);
            m.w32(ecx, s.vt0);
            m.w32(ecx + 4, s.vt4);
            Ret::U32(ecx)
        } else if f == s.dtor {
            m.w32(ecx, s.vt0);
            m.w32(ecx + 4, s.vt4);
            Ret::U32(h.call(m, s.base_dtor, ecx, &[]))
        } else if f == s.is_active {
            let r = has_flag(m, h, s.bit);
            let ui = m.r32(UI);
            let v = m.r32(ui + 0x4F0);
            Ret::U32(r & ((v & !0xFF) | u32::from(v != 0)))
        } else if f == s.get_resource {
            Ret::U32(s.resource)
        } else if f == s.deleting_dtor {
            Ret::U32(deleting_dtor(m, h, ecx, s.dtor, a[0]))
        } else if f == s.thunk {
            Ret::U32(h.call(m, s.deleting_dtor, ecx.wrapping_sub(4), &[a[0]]))
        } else {
            continue;
        };
        return Some(r);
    }
    let r = match f {
        // bit09
        0x5535F0 => {
            h.call(m, TOWN_VARIANT_CTOR, ecx, &[]);
            h.call(m, GROUPED_STATE_CTOR, ecx + 0x20, &[]);
            m.w32(ecx, 0x0067_4088);
            m.w32(ecx + 4, 0x0067_4080);
            bit09_reset(m, h, ecx);
            Ret::U32(ecx)
        }
        0x5536A0 => {
            m.w32(ecx, 0x0067_4088);
            m.w32(ecx + 4, 0x0067_4080);
            bit09_reset(m, h, ecx);
            h.call(m, GROUPED_STATE_DTOR, ecx + 0x20, &[]);
            Ret::U32(h.call(m, TOWN_VARIANT_DTOR, ecx, &[]))
        }
        0x553760 => {
            bit09_reset(m, h, ecx);
            Ret::None
        }
        0x5537C0 => {
            grouped_release(m, h, &BIT09);
            Ret::None
        }
        0x5538D0 => {
            grouped_draw_frame(m, h, ecx, a, esp, false);
            Ret::None
        }
        0x553AB0 => {
            bit09_draw_animated(m, h, ecx, a);
            Ret::None
        }
        0x553DF0 => Ret::U8(grouped_parse_extra(m, h, ecx, a[0], a[1], false)),
        0x553EA0 => Ret::U8(grouped_parse_config(m, a[0], a[1], BIT09.period)),
        0x553EE0 => Ret::U32(0x006E_1228),
        0x553EF0 => Ret::U32(deleting_dtor(m, h, ecx, 0x5536A0, a[0])),
        0x553F10 => {
            anim_set_list_resize(m, h, ecx, a[0]);
            Ret::None
        }
        0x553FC0 => Ret::U32(anim_set_list_deleting_dtor(m, h, ecx, a[0])),
        0x554030 => Ret::U32(h.call(m, 0x553EF0, ecx.wrapping_sub(4), &[a[0]])),
        // bit14
        0x554190 => {
            h.call(m, TOWN_VARIANT_CTOR, ecx, &[]);
            h.call(m, GROUPED_STATE_CTOR, ecx + 0x24, &[]);
            m.w32(ecx, 0x0067_4120);
            m.w32(ecx + 4, 0x0067_411C);
            m.w8(ecx + 0x1C, 0xFF);
            let c = m.r32(WORLD_CLOCK);
            m.w32(ecx + 0x20, c);
            vcall(m, h, ecx + 0x24, 4, &[]);
            Ret::U32(ecx)
        }
        0x554200 => {
            m.w32(ecx, 0x0067_4120);
            m.w32(ecx + 4, 0x0067_411C);
            m.w8(ecx + 0x1C, 0xFF);
            let c = m.r32(WORLD_CLOCK);
            m.w32(ecx + 0x20, c);
            vcall(m, h, ecx + 0x24, 4, &[]);
            h.call(m, GROUPED_STATE_DTOR, ecx + 0x24, &[]);
            Ret::U32(h.call(m, TOWN_VARIANT_DTOR, ecx, &[]))
        }
        0x5542A0 => {
            grouped_release(m, h, &BIT14);
            Ret::None
        }
        0x5543A0 => {
            grouped_draw_frame(m, h, ecx, a, esp, true);
            Ret::None
        }
        0x554560 => {
            bit14_draw_animated(m, h, ecx, a);
            Ret::None
        }
        0x554700 => Ret::U8(grouped_parse_extra(m, h, ecx, a[0], a[1], true)),
        0x554790 => Ret::U8(grouped_parse_config(m, a[0], a[1], BIT14.period)),
        0x5547D0 => Ret::U32(0x006E_1254),
        0x5547E0 => Ret::U32(deleting_dtor(m, h, ecx, 0x554200, a[0])),
        0x554800 => Ret::U32(h.call(m, 0x5547E0, ecx.wrapping_sub(4), &[a[0]])),
        // two_variant
        0x554AF0 => {
            h.call(m, TOWN_SPRITE_CTOR, ecx, &[]);
            m.w32(ecx, 0x0067_42E8);
            m.w32(ecx + 4, 0x0067_42E4);
            m.w8(ecx + 0x18, 0);
            Ret::U32(ecx)
        }
        0x554B10 => {
            m.w32(ecx, 0x0067_42E8);
            m.w32(ecx + 4, 0x0067_42E4);
            h.call(m, BURN_RESET, ecx + 4, &[]);
            Ret::U32(h.call(m, TOWN_SPRITE_DTOR, ecx, &[]))
        }
        0x554B70 => Ret::U8(two_variant_load(m, h, ecx, a[0], esp)),
        0x554C70 => {
            let v = u8::from(a[0] as u8 != m.r8(VARIANT_CODE_A));
            m.w8(ecx + 0x18, v);
            Ret::None
        }
        0x554C90 => Ret::U8(if m.r8(ecx + 0x18) == 0 { m.r8(VARIANT_CODE_A) } else { m.r8(VARIANT_CODE_B) }),
        0x554CB0 => Ret::U32(TWO_VARIANT_SHEETS + u32::from(m.r8(ecx + 0x18)) * 0x3C),
        0x554CD0 => Ret::U32(deleting_dtor(m, h, ecx, 0x554B10, a[0])),
        0x554CF0 => Ret::U32(h.call(m, 0x554CD0, ecx.wrapping_sub(4), &[a[0]])),
        // bit0e
        0x554E60 => {
            h.call(m, TOWN_SPRITE_CTOR, ecx, &[]);
            h.call(m, ANIM_BASE_CTOR, ecx + 0x18, &[]);
            h.call(m, GROUPED_ANIMATOR_CTOR, ecx + 0x28, &[]);
            h.call(m, ANIM_BASE_CTOR, ecx + 0x3C, &[]);
            m.w32(ecx, 0x0067_4408);
            m.w32(ecx + 4, 0x0067_4404);
            bit0e_reset(m, h, ecx);
            Ret::U32(ecx)
        }
        0x554F00 => {
            m.w32(ecx, 0x0067_4408);
            m.w32(ecx + 4, 0x0067_4404);
            bit0e_reset(m, h, ecx);
            h.call(m, ANIM_BASE_DTOR, ecx + 0x3C, &[]);
            h.call(m, GROUPED_ANIMATOR_DTOR, ecx + 0x28, &[]);
            h.call(m, ANIM_BASE_DTOR, ecx + 0x18, &[]);
            Ret::U32(h.call(m, TOWN_SPRITE_DTOR, ecx, &[]))
        }
        0x554FA0 => Ret::U32(bit0e_reset(m, h, ecx)),
        0x554FD0 => {
            h.call(m, SHEET_RESET, SHEET_0E, &[]);
            h.call(m, ANIM_SET_RESET, SET_0E, &[]);
            cdecl(m, h, ANIM_SET_ARRAY_FREE, &[0x006E_13A0, 0x006E_13A8]);
            cdecl(m, h, ANIM_SET_ARRAY_FREE, &[0x006E_13A4, 0x006E_13AC]);
            Ret::None
        }
        0x555010 => {
            bit0e_draw_frame(m, h, ecx, a, esp);
            Ret::None
        }
        0x555190 => {
            bit0e_draw_animated(m, h, ecx, a);
            Ret::None
        }
        0x555610 => Ret::U8(static_parse_extra(m, h, a[0], a[1], &[(5, SET_0E)], &[(3, 0x006E_13A4, 0x006E_13AC), (4, 0x006E_13A0, 0x006E_13A8)])),
        0x555690 => {
            let show = m.r8(ecx + 0x34) & 0x40 == 0;
            let f = if show { u32::from(m.r8(ecx + 0x44)) } else { 0 };
            redraw_region(m, h, a, SHEET_0E, SET_0E, f, show);
            Ret::None
        }
        0x5556E0 => Ret::U32(SHEET_0E),
        0x5556F0 => Ret::U32(deleting_dtor(m, h, ecx, 0x554F00, a[0])),
        0x555710 => Ret::U32(h.call(m, 0x5556F0, ecx.wrapping_sub(4), &[a[0]])),
        // bit05
        0x5557E0 => {
            h.call(m, TOWN_SPRITE_CTOR, ecx, &[]);
            h.call(m, ANIM_BASE_CTOR, ecx + 0x18, &[]);
            h.call(m, ANIM_BASE_CTOR, ecx + 0x24, &[]);
            m.w32(ecx, 0x0067_4498);
            m.w32(ecx + 4, 0x0067_4494);
            m.w32(ecx + 0x30, 0);
            h.call(m, ANIM_BASE_RESET, ecx + 0x18, &[]);
            h.call(m, ANIM_BASE_RESET, ecx + 0x24, &[]);
            bit05_drop_walker(m, h, ecx);
            m.w32(ecx + 0x1C, 0);
            let last = m.r8(SET_05A + 0x14).wrapping_sub(1);
            m.w8(ecx + 0x20, last);
            let v = m.r8(ST_DONE);
            m.w8(ecx + 0x21, v);
            Ret::U32(ecx)
        }
        0x555890 => {
            m.w32(ecx, 0x0067_4498);
            m.w32(ecx + 4, 0x0067_4494);
            h.call(m, ANIM_BASE_RESET, ecx + 0x18, &[]);
            h.call(m, ANIM_BASE_RESET, ecx + 0x24, &[]);
            bit05_drop_walker(m, h, ecx);
            h.call(m, ANIM_BASE_DTOR, ecx + 0x24, &[]);
            h.call(m, ANIM_BASE_DTOR, ecx + 0x18, &[]);
            Ret::U32(h.call(m, TOWN_SPRITE_DTOR, ecx, &[]))
        }
        0x555940 => {
            h.call(m, ANIM_BASE_RESET, ecx + 0x18, &[]);
            h.call(m, ANIM_BASE_RESET, ecx + 0x24, &[]);
            bit05_drop_walker(m, h, ecx);
            Ret::None
        }
        0x555980 => {
            h.call(m, SHEET_RESET, SHEET_05, &[]);
            h.call(m, ANIM_SET_RESET, SET_05B, &[]);
            Ret::U32(h.call(m, ANIM_SET_RESET, SET_05A, &[]))
        }
        0x5559A0 => {
            bit05_draw_frame(m, h, ecx, a, esp);
            Ret::None
        }
        0x555B00 => {
            bit05_draw_animated(m, h, ecx, a);
            Ret::None
        }
        0x555EF0 => Ret::U8(static_parse_extra(m, h, a[0], a[1], &[(3, SET_05A), (4, SET_05B)], &[])),
        0x555F40 => {
            let show = m.r8(ecx + 0x21) & 0x7F != m.r8(ST_DONE);
            let f = if show { u32::from(m.r8(ecx + 0x2C)) } else { 0 };
            redraw_region(m, h, a, SHEET_05, SET_05B, f, show);
            Ret::None
        }
        0x555FA0 => Ret::U32(SHEET_05),
        0x555FB0 => Ret::U32(deleting_dtor(m, h, ecx, 0x555890, a[0])),
        0x555FD0 => Ret::U32(h.call(m, 0x555FB0, ecx.wrapping_sub(4), &[a[0]])),
        // overlay_pair / bit0b
        0x556380 => {
            h.call(m, TOWN_SPRITE_CTOR, ecx, &[]);
            h.call(m, ANIM_BASE_CTOR, ecx + 0x18, &[]);
            h.call(m, ANIM_BASE_CTOR, ecx + 0x24, &[]);
            m.w32(ecx, 0x0067_46E0);
            m.w32(ecx + 4, 0x0067_46DC);
            m.w8(ecx + 0x30, 0xFF);
            m.w8(ecx + 0x31, 0xFF);
            h.call(m, ANIM_BASE_RESET, ecx + 0x18, &[]);
            h.call(m, ANIM_BASE_RESET, ecx + 0x24, &[]);
            Ret::U32(ecx)
        }
        0x556410 => {
            m.w32(ecx, 0x0067_46E0);
            m.w32(ecx + 4, 0x0067_46DC);
            m.w8(ecx + 0x30, 0xFF);
            m.w8(ecx + 0x31, 0xFF);
            h.call(m, ANIM_BASE_RESET, ecx + 0x18, &[]);
            h.call(m, ANIM_BASE_RESET, ecx + 0x24, &[]);
            h.call(m, ANIM_BASE_DTOR, ecx + 0x24, &[]);
            h.call(m, ANIM_BASE_DTOR, ecx + 0x18, &[]);
            Ret::U32(h.call(m, TOWN_SPRITE_DTOR, ecx, &[]))
        }
        0x5564A0 => {
            m.w8(ecx + 0x30, 0xFF);
            m.w8(ecx + 0x31, 0xFF);
            h.call(m, ANIM_BASE_RESET, ecx + 0x18, &[]);
            Ret::U32(h.call(m, ANIM_BASE_RESET, ecx + 0x24, &[]))
        }
        0x5564C0 => {
            h.call(m, SHEET_RESET, SHEET_0B, &[]);
            for k in 0..6 {
                h.call(m, ANIM_SET_RESET, set_0b(k), &[]);
            }
            Ret::None
        }
        0x5564F0 => {
            overlay_draw_frame(m, h, ecx, a, esp);
            Ret::None
        }
        0x556680 => {
            bit0b_draw_frame_masked(m, h, ecx, a, esp);
            Ret::None
        }
        0x556780 => Ret::U8(static_parse_extra(
            m,
            h,
            a[0],
            a[1],
            &[(3, set_0b(0)), (4, set_0b(1)), (5, set_0b(2)), (6, set_0b(4)), (7, set_0b(5)), (8, set_0b(3))],
            &[],
        )),
        0x556860 => {
            bit0b_draw_animated(m, h, ecx, a);
            Ret::None
        }
        0x556D60 => Ret::U32(SHEET_0B),
        0x556D70 => Ret::U32(deleting_dtor(m, h, ecx, 0x556410, a[0])),
        0x556D90 => Ret::U32(h.call(m, 0x556D70, ecx.wrapping_sub(4), &[a[0]])),
        _ => return None,
    };
    Some(r)
}
