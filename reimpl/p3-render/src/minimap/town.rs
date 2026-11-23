//! `TownMiniMap` (0x59C200–0x59D8xx): el minimapa de la ciudad. Disposición del objeto: `+0x14/+0x18` posición y `+0x2c/+0x30` tamaño de
//! la ventana (clase base), `+0x48` visible, `+0x94` textura del terreno, `+0x98/+0x9c` origen del dibujo, `+0xa4/+0xa8` tamaño de la
//! textura, `+0xac/+0xb0/+0xb4/+0xb8` rectángulo de la vista, `+0xbc` arrastre, `+0xc0` textura de la zona resaltada, `+0xc4` tabla de
//! 0x100 bytes con el degradado de paleta, `+0x1c4` reloj, `+0x1c8/+0x1c9` valor y paso del parpadeo.
//! `[0x6E51AC]` es la escena (plano de la ciudad): `+0xc314/+0xc318` ancho y alto de la rejilla, `+0xc310` celdas de 4 bytes.

use super::{cdecl, vcall, Host, Mem, Ret, CDECL};

const SCENE: u32 = 0x006E_51AC;
const VTABLE: u32 = 0x0067_7AC8;
const BASE_CTOR: u32 = 0x004B_15F0;
const BASE_DTOR: u32 = 0x004B_1640;
const BASE_RESET: u32 = 0x004B_1650;
const BASE_MOUSE: u32 = 0x004B_1720;
const BASE_SET_VISIBLE: u32 = 0x004B_21A0;
const FREE_TEXTURE: u32 = 0x004B_B490;
const INVALIDATE: u32 = 0x004B_9650;
const DIRTY: u32 = 0x006D_A5F0;
const SCOPE_GUARD: u32 = 0x0064_F253;
const STRING_INIT: u32 = 0x0064_F2C1;
const MOUSE_BUSY: u32 = 0x0046_2310;
const SCENE_WIDTH_UNITS: u32 = 0x0059_3BF0;
const SCENE_HEIGHT_UNITS: u32 = 0x0059_3B70;
const SCENE_SET_CENTER: u32 = 0x0059_0AE0;
const SCENE_PROJECT: u32 = 0x0058_D030;
const SET_RECT: u32 = 0x0066_A4D8;
const FREE: u32 = 0x0064_F7E2;
const CLASS_NAME_ARG: u32 = 0x006C_4FA8;
const RETIRE_UNITS: u32 = 0x006D_E4B0;
const OWN_ID: u32 = 0x006D_FC14;
const TABLE_LIMIT: u32 = 0x006D_E4AA;
const TABLE: u32 = 0x006D_E4A0;
const TABLE_ENTRY: u32 = 0x0053_03C0;

/// Tabla de casos de `0x59D8C8` (la misma en `draw`, `render_layer_to_texture` y `get_cell_color`): clase de objeto de celda → caso.
pub const CELL_CASE: [u8; 0x6A] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 11, 1, 2, 3, 4, 5, 6, 7, 0, 0, 3, 3, 11,
    4, 0, 4, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 4, 0, 8, 11, 9, 10, 0, 0, 0, 0, 4, 8,
    11, 11, 11, 0, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 8, 8, 0, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8,
];

/// Caso de la tabla para una clase (`> 0x69` cae en el caso por defecto, el 11).
pub fn cell_case(class: u32) -> u8 {
    if class > 0x69 {
        11
    } else {
        CELL_CASE[class as usize]
    }
}

/// Campo `off` de la escena viva (`[[0x6E51AC] + off]`).
fn scene_at(m: &mut dyn Mem, off: u32) -> u32 {
    let s = m.r32(SCENE);
    m.r32(s.wrapping_add(off))
}

/// `sbb eax, eax` tras `cmp a, b` (a < b sin signo): `0xFFFFFFFF` o 0.
fn sbb_lt(a: u32, b: u32) -> u32 {
    if a < b {
        u32::MAX
    } else {
        0
    }
}

/// Color de un terreno por su byte de celda (caso 8 de la tabla): `byte` es el índice en la tabla de tipos de terreno.
fn terrain_color(m: &mut dyn Mem, h: &mut dyn Host, byte: u8) -> u32 {
    if u16::from(byte) < m.r16(TABLE_LIMIT) {
        if u32::from(byte) == m.r32(OWN_ID) {
            return 0xFF0A_0099;
        }
        let e = h.call(m, TABLE_ENTRY, TABLE, &[u32::from(byte)]);
        let v = m.r16(e.wrapping_add(8));
        let mask = if v != 0 { u32::MAX } else { 0 };
        return (mask & 0x006C_FFF5).wrapping_add(0xFF93_000A);
    }
    0xFF39_342D
}

/// Color de una celda según su clase (`0x59D780`, y el mismo bloque en línea de `0x59CA00`): `fixed` es el color ya calculado de la
/// celda anterior (caso 2), `cell` el índice `fila * ancho + columna` de la rejilla.
pub fn color_for_class(
    m: &mut dyn Mem,
    h: &mut dyn Host,
    class: u32,
    fixed: &dyn Fn(&mut dyn Mem) -> u32,
    cell: &dyn Fn(&mut dyn Mem) -> u32,
) -> u32 {
    let scene = m.r32(SCENE);
    match cell_case(class) {
        0 => 0xFF00_4841,
        1 => 0xFF22_6192,
        2 => fixed(m),
        3 => 0xFFAB_9F8C,
        5 => {
            let t = sbb_lt(m.r32(scene.wrapping_add(0xC324)), m.r32(RETIRE_UNITS));
            (t & 0xFF9F_DF10).wrapping_add(0xFF82_8282)
        }
        6 => {
            let t = sbb_lt(m.r32(scene.wrapping_add(0xC324)), m.r32(RETIRE_UNITS));
            (t & 0xFF22_6292).wrapping_add(0xFFFF_FF00)
        }
        7 => 0xFF1C_5D19,
        8 => {
            let cell = cell(m);
            let cells = m.r32(scene.wrapping_add(0xC310));
            let byte = m.r8(cells.wrapping_add(cell.wrapping_mul(4)).wrapping_add(3));
            terrain_color(m, h, byte)
        }
        9 => 0xFF0A_0099,
        10 => 0xFF93_0012,
        _ => 0xFF5A_832D,
    }
}

/// `0x59D780(&clase, &color_fijo, &columna, &fila)` (stdcall, 4 argumentos): color de una celda del minimapa.
fn get_cell_color(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32]) -> u32 {
    let class = m.r32(a[0]);
    let fixed = |m: &mut dyn Mem| m.r32(a[1]);
    let cell = |m: &mut dyn Mem| {
        let scene = m.r32(SCENE);
        let width = m.r32(scene.wrapping_add(0xC314));
        let row = m.r32(a[3]);
        width.wrapping_mul(row).wrapping_add(m.r32(a[2]))
    };
    color_for_class(m, h, class, &fixed, &cell)
}

/// Cuerpo común de constructores y `reset`: el objeto queda sin datos (tamaño 0, vista vacía) y sin textura de la zona resaltada.
fn reset_fields(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    m.w32(this + 0xA4, 0);
    m.w32(this + 0xA8, 0);
    m.w32(this + 0xB8, u32::MAX);
    m.w32(this + 0x98, 0);
    m.w32(this + 0x9C, 0);
    m.w32(this + 0xA0, 0);
    h.call(m, BASE_RESET, this, &[]);
    let over = m.r32(this + 0xC0);
    m.w8(this + 0xBC, 0);
    m.w8(this + 0x1C8, 0);
    m.w8(this + 0x1C9, 0x1E);
    m.w32(this + 0x1C4, 0);
    if over != 0 {
        cdecl(m, h, FREE_TEXTURE, &[over]);
        m.w32(this + 0xC0, 0);
    }
}

/// `0x59C200` (constructor): tabla de degradado de `+0xC4` (0x100 bytes) y estado inicial.
fn ctor(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    let arg = m.r32(CLASS_NAME_ARG);
    h.call(m, BASE_CTOR, this, &[arg]);
    m.w32(this, VTABLE);
    m.w32(this + 0xC0, 0);
    m.w32(this + 0x94, 0);
    for i in 0u32..0x100 {
        let q = (i / 13) as u8;
        let v = if i < 0x5D {
            0x5Du8.wrapping_sub(q)
        } else if i < 0x99 {
            i as u8
        } else {
            q.wrapping_sub(0x67)
        };
        m.w8(this + 0xC4 + i, v);
    }
    reset_fields(m, h, this);
    this
}

/// `0x59C340(nombre)` (constructor con nombre).
fn ctor_with_name(m: &mut dyn Mem, h: &mut dyn Host, this: u32, name: u32) -> u32 {
    h.call(m, BASE_CTOR, this, &[name]);
    m.w32(this, VTABLE);
    m.w32(this + 0xC0, 0);
    m.w32(this + 0x94, 0);
    reset_fields(m, h, this);
    this
}

/// `0x59C400` (destructor): restaura la tabla virtual y sigue en el de la clase base.
fn dtor(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    m.w32(this, VTABLE);
    h.call(m, BASE_DTOR, this, &[])
}

/// `0x59C410` (reset): libera la textura y deja el objeto sin datos.
fn reset(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let tex = m.r32(this + 0x94);
    if tex != 0 {
        cdecl(m, h, FREE_TEXTURE, &[tex]);
        m.w32(this + 0x94, 0);
    }
    reset_fields(m, h, this);
}

/// `0x59C4A0(&punto, evento)`: ratón; pulsar (1) centra la vista de la escena donde se pulsa y empieza el arrastre, `-2` lo termina.
fn on_mouse_event(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) -> u32 {
    let (pt, ev) = (a[0], a[1]);
    h.call(m, BASE_MOUSE, this, &[pt, ev]);
    let busy = h.call(m, MOUSE_BUSY, CDECL, &[]);
    if busy as u8 != 0 {
        return busy;
    }
    if ev == 1 {
        let scene = m.r32(SCENE);
        let x = m.r32(pt).wrapping_sub(m.r32(this + 0x98)) as i32;
        let x = (x.wrapping_sub(x >> 31) >> 1).wrapping_mul(m.r32(scene.wrapping_add(0xC314)).wrapping_mul(2).wrapping_add(1) as i32);
        let x = x.wrapping_div(m.r32(this + 0xA4) as i32);
        let y = (m.r32(pt + 4).wrapping_sub(m.r32(this + 0x9C)) as i32).wrapping_mul(m.r32(scene.wrapping_add(0xC318)).wrapping_sub(1) as i32);
        let y = y.wrapping_div(m.r32(this + 0xA8) as i32);
        let w = h.call(m, SCENE_WIDTH_UNITS, scene, &[]) as i32;
        let x = x.wrapping_add(-((w - (w >> 31)) >> 1));
        let hh = h.call(m, SCENE_HEIGHT_UNITS, scene, &[]) as i32;
        let y = y.wrapping_add(-((hh - (hh >> 31)) >> 1));
        let r = h.call(m, SCENE_SET_CENTER, scene, &[x as u32, y as u32]);
        m.w8(this + 0xBC, 1);
        return r;
    }
    if ev == (-2i32) as u32 {
        m.w8(this + 0xBC, 0);
    }
    busy
}

/// `0x59D2F0(visible)`: al mostrarlo invalida su rectángulo y deja la clase base hacer el resto.
fn set_visible(m: &mut dyn Mem, h: &mut dyn Host, this: u32, flag: u32, esp: u32) -> u32 {
    if flag as u8 != 0 && m.r8(this + 0x48) == 0 {
        let r = esp - 0x10;
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

/// `0x59D350(x, y, z)`: llama al `slot 0x68` (poner posición) con un vector de tres palabras.
fn set_position_xyz(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) -> u32 {
    let p = esp - 0xC;
    m.w32(p, a[0]);
    m.w32(p + 8, a[2]);
    m.w32(p + 4, a[1]);
    vcall(m, h, this, 0x68, &[p])
}

/// `0x59D380(&punto)`: coloca el origen del dibujo (centrado si la textura es menor que la ventana) y actualiza el rectángulo de la vista.
fn set_position(m: &mut dyn Mem, h: &mut dyn Host, this: u32, pt: u32) -> Ret {
    let x = m.r32(pt);
    m.w32(this + 0x98, x);
    let y = m.r32(pt + 4);
    m.w32(this + 0x9C, y);
    let (tw, ww) = (m.r32(this + 0xA4), m.r32(this + 0x2C));
    if tw < ww {
        let v = m.r32(this + 0x98).wrapping_add(ww.wrapping_sub(tw) >> 1);
        m.w32(this + 0x98, v);
    }
    let (th, wh) = (m.r32(this + 0xA8), m.r32(this + 0x30));
    if th < wh {
        let v = m.r32(this + 0x9C).wrapping_add(wh.wrapping_sub(th) >> 1);
        m.w32(this + 0x9C, v);
    }
    h.call(m, 0x0059_D400, this, &[]);
    for k in 0..3 {
        let v = m.r32(pt + 4 * k);
        m.w32(this + 0x14 + 4 * k, v);
    }
    Ret::U8(1)
}

/// Posición, en píxeles del minimapa, de un punto del mundo (`0x58D030` devuelve la casilla en dos enteros; ver `0x59C930`): el valor se
/// duplica, el segundo se redondea a par y los dos se escalan al tamaño de la textura.
fn scale_to_texture(m: &mut dyn Mem, this: u32, scene: u32, c0: u32, c1: u32) -> (u32, u32) {
    let c0 = c0.wrapping_mul(2);
    let c1 = if c1 & 1 != 0 { c1.wrapping_add(1) } else { c1 };
    let wd = m.r32(scene.wrapping_add(0xC314)).wrapping_mul(2).wrapping_add(1);
    let x = m.r32(this + 0xA4).wrapping_mul(c0) / wd;
    let hd = m.r32(scene.wrapping_add(0xC318)).wrapping_sub(1);
    let y = m.r32(this + 0xA8).wrapping_mul(c1) / hd;
    (x.wrapping_add(m.r32(this + 0x98)).wrapping_add(1), y.wrapping_add(m.r32(this + 0x9C)).wrapping_add(1))
}

/// `0x59C930(barco, &rect)`: rectángulo de 3×3 píxeles del punto de un barco.
fn get_ship_dot_rect(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) -> u32 {
    let (ship, out) = (a[0], a[1]);
    let pt = esp - 8;
    vcall(m, h, ship + 4, 0x2C, &[pt]);
    let scene = m.r32(SCENE);
    let o = esp - 0x10;
    let (px, py) = (m.r32(esp - 8), m.r32(esp - 4));
    h.call(m, SCENE_PROJECT, scene, &[px, py, o]);
    let (c0, c1) = (m.r32(o), m.r32(o + 4));
    let (l, t) = scale_to_texture(m, this, scene, c0, c1);
    m.w32(out, l);
    m.w32(out + 4, t);
    m.w32(out + 8, l.wrapping_add(3));
    m.w32(out + 0xC, t.wrapping_add(3));
    out
}

/// `0x59D540(&rect)` (cdecl): invalida los cuatro bordes de un rectángulo (líneas de un píxel).
fn invalidate_rect_outline(m: &mut dyn Mem, h: &mut dyn Host, rect: u32, esp: u32) {
    let r = esp - 0x10;
    let (x0, y0, x1, y1) = (m.r32(rect), m.r32(rect + 4), m.r32(rect + 8), m.r32(rect + 0xC));
    // Borde superior (x0, y0, x1+1, y0+1).
    m.w32(r + 4, y0);
    m.w32(r + 0xC, y0.wrapping_add(1));
    m.w32(r, x0);
    m.w32(r + 8, x1.wrapping_add(1));
    h.call(m, INVALIDATE, DIRTY, &[r]);
    // Borde inferior (x0, y1, x1+1, y1+1).
    m.w32(r + 4, y1);
    m.w32(r + 0xC, y1.wrapping_add(1));
    h.call(m, INVALIDATE, DIRTY, &[r]);
    // Borde izquierdo (x0, y0, x0+1, y1).
    m.w32(r, x0);
    m.w32(r + 4, y0);
    m.w32(r + 8, x0.wrapping_add(1));
    m.w32(r + 0xC, y1);
    h.call(m, INVALIDATE, DIRTY, &[r]);
    // Borde derecho (x1, y0, x1+1, y1).
    m.w32(r, x1);
    m.w32(r + 8, x1.wrapping_add(1));
    h.call(m, INVALIDATE, DIRTY, &[r]);
}

/// `0x59D400`: rectángulo de la vista de la escena sobre la textura; si cambió, invalida el antiguo y el nuevo.
fn update_view_rect(m: &mut dyn Mem, h: &mut dyn Host, this: u32, esp: u32) -> u32 {
    let scene = m.r32(SCENE);
    let t = m.r32(scene.wrapping_add(0xC640)).wrapping_mul(m.r32(this + 0xA8)) / m.r32(scene.wrapping_add(0xC318)).wrapping_sub(1);
    let top = t.wrapping_add(m.r32(this + 0x9C));
    let wd = m.r32(scene.wrapping_add(0xC314)).wrapping_mul(2).wrapping_add(1);
    let l = m.r32(scene.wrapping_add(0xC644)).wrapping_mul(m.r32(this + 0xA4)).wrapping_mul(2) / wd;
    let left = l.wrapping_add(m.r32(this + 0x98));
    let old_left = m.r32(this + 0xAC);
    if left == old_left && top == m.r32(this + 0xB0) {
        return old_left;
    }
    let r = esp - 0x10;
    let (b0, b4, b8) = (m.r32(this + 0xB0), m.r32(this + 0xB4), m.r32(this + 0xB8));
    m.w32(r + 4, b0);
    m.w32(r, old_left);
    m.w32(r + 8, b4);
    m.w32(r + 0xC, b8);
    cdecl(m, h, 0x0059_D540, &[r]);
    m.w32(this + 0xB0, top);
    m.w32(this + 0xAC, left);
    let w = h.call(m, SCENE_WIDTH_UNITS, scene, &[]);
    let right = w.wrapping_mul(m.r32(this + 0xA4)).wrapping_mul(2) / m.r32(scene.wrapping_add(0xC314)).wrapping_mul(2).wrapping_add(1);
    let right = right.wrapping_add(m.r32(this + 0xAC));
    m.w32(this + 0xB4, right);
    let hh = h.call(m, SCENE_HEIGHT_UNITS, scene, &[]).wrapping_sub(1);
    let bottom = hh.wrapping_mul(m.r32(this + 0xA8)) / m.r32(scene.wrapping_add(0xC318)).wrapping_sub(1);
    let bottom = bottom.wrapping_add(top);
    m.w32(this + 0xB8, bottom);
    let (l2, t2, r2) = (m.r32(this + 0xAC), top, m.r32(this + 0xB4));
    let f = m.r32(SET_RECT);
    cdecl(m, h, f, &[r, l2, t2, r2, bottom]);
    cdecl(m, h, 0x0059_D540, &[r])
}


const SGL_NEW_TEXTURE: u32 = 0x004B_B2A0;
const SGL_LOCK: u32 = 0x004B_BB00;
const SGL_UNLOCK: u32 = 0x004B_BB10;
const SGL_SIZE_OF: u32 = 0x004B_BB20;
const SGL_FREE: u32 = 0x004B_B490;
const SGL_SET_TARGET: u32 = 0x004B_B9B0;
const SGL_SELECT_TEXTURE: u32 = 0x004B_B9C0;
const SGL_SET_COLOR: u32 = 0x004B_B870;
const SGL_SET_MODE: u32 = 0x004B_B760;
const SGL_SET_ALPHA_MODE: u32 = 0x004B_B7A0;
const SGL_BLIT: u32 = 0x004B_B330;
const SGL_STRETCH: u32 = 0x004B_BA80;
const SGL_FILL_RECT: u32 = 0x004B_B430;
const SGL_LINE: u32 = 0x004B_D680;
const SGL_CLIP_GET: u32 = 0x004B_B7C0;
const SGL_CLIP_SET: u32 = 0x004B_B820;
const SGL_RENDER_TARGET: u32 = 0x004B_B220;
const LOCAL_OBJ_CTOR: u32 = 0x0046_C320;
const LOCAL_OBJ_DTOR: u32 = 0x0046_C330;
const LOCAL_OBJ_SET: u32 = 0x0046_C340;
const LOCAL_OBJ_GET: u32 = 0x0046_C350;
const VISIBLE_CHECK: u32 = 0x004B_2200;
const SHIP_LIST: u32 = 0x006E_2D50;
const TIMER: u32 = 0x006D_CCF8;
const RENDER_LAYER: u32 = 0x0059_CA00;
const GET_CELL_COLOR: u32 = 0x0059_D780;
const SGL_CURRENT: u32 = 0x006C_BB40;

/// `0x59CA00(capa)` (this = minimapa): pinta la textura del terreno bloqueándola (SGL) y recorriendo las celdas de la capa. Las filas impares
/// empiezan con un texel suelto, las pares terminan con otro; cada celda ocupa dos texels. El color sale de la clase del objeto de la celda.
fn render_layer(m: &mut dyn Mem, h: &mut dyn Host, this: u32, layer: u32, esp: u32) -> u32 {
    let lock = esp - 0x14;
    let tex = m.r32(this + 0x94);
    cdecl(m, h, SGL_LOCK, &[tex, lock, 3]);
    let scene = m.r32(SCENE);
    let mut pix = m.r32(lock);
    let objs = m.r32(scene.wrapping_add(0x98));
    let mut row = m.r32(scene.wrapping_add(layer.wrapping_mul(4)).wrapping_add(0xC308));
    let height = m.r32(scene.wrapping_add(0xC318));
    let s = esp - 0x34;
    let mut y = 0u32;
    while y < height {
        let width = m.r32(scene.wrapping_add(0xC314));
        if y & 1 != 0 {
            let id = u32::from(m.r16(row));
            if id != 0 {
                let obj = m.r32(objs.wrapping_add(id.wrapping_mul(4)));
                let class = vcall(m, h, obj, 0x50, &[]);
                let esi = pix;
                let fixed = move |m: &mut dyn Mem| m.r32(esi);
                let cell = move |_: &mut dyn Mem| width.wrapping_mul(y);
                let c = color_for_class(m, h, class, &fixed, &cell);
                m.w32(pix, c);
            }
            pix = pix.wrapping_add(4);
        }
        let mut col = 0u32;
        while col < width {
            let id = u32::from(m.r16(row));
            if id != 0 {
                let obj = m.r32(objs.wrapping_add(id.wrapping_mul(4)));
                let class = vcall(m, h, obj, 0x50, &[]);
                let esi = pix;
                let fixed = move |m: &mut dyn Mem| m.r32(esi);
                let cell = move |_: &mut dyn Mem| width.wrapping_mul(y).wrapping_add(col);
                let c = color_for_class(m, h, class, &fixed, &cell);
                m.w32(pix, c);
                pix = pix.wrapping_add(4);
                m.w32(pix, c);
                pix = pix.wrapping_add(4);
            } else {
                pix = pix.wrapping_add(8);
            }
            row = row.wrapping_add(4);
            col += 1;
        }
        if y & 1 == 0 {
            let id = u32::from(m.r16(row.wrapping_sub(4)));
            if id != 0 {
                let obj = m.r32(objs.wrapping_add(id.wrapping_mul(4)));
                let class = vcall(m, h, obj, 0x50, &[]);
                m.w32(s + 0x1C, class);
                m.w32(s + 0x38, col);
                m.w32(s + 0x10, y);
                let c = h.call(m, GET_CELL_COLOR, this, &[s + 0x1C, pix, s + 0x38, s + 0x10]);
                m.w32(pix, c);
            }
            pix = pix.wrapping_add(4);
        }
        let pitch = m.r32(s + 0x30);
        let step = pitch.wrapping_sub(m.r32(this + 0xA4) << 2);
        pix = pix.wrapping_add(step.wrapping_mul(4));
        y += 1;
        let height = scene_at(m, 0xC318);
        if y >= height {
            break;
        }
    }
    let tex = m.r32(this + 0x94);
    cdecl(m, h, SGL_UNLOCK, &[tex])
}

/// `0x59CEB0` (reconstruye la textura): crea la textura del tamaño de la rejilla, pinta el terreno (capa 0), mezcla la imagen del mapa de
/// fondo de la escena (reescalada y pasada por la tabla de degradado de `+0xC4`), pinta la capa 1 y ajusta la textura a la ventana.
fn rebuild_texture(m: &mut dyn Mem, h: &mut dyn Host, this: u32, esp: u32) -> u32 {
    let tex = m.r32(this + 0x94);
    if tex != 0 {
        cdecl(m, h, SGL_FREE, &[tex]);
        m.w32(this + 0x94, 0);
    }
    let scene = m.r32(SCENE);
    let w2 = m.r32(scene.wrapping_add(0xC314)).wrapping_mul(2).wrapping_add(1);
    m.w32(this + 0xA4, w2);
    let scene = m.r32(SCENE);
    let hh = m.r32(scene.wrapping_add(0xC318));
    m.w32(this + 0xA8, hh);
    let t = cdecl(m, h, SGL_NEW_TEXTURE, &[w2, hh, 0]);
    m.w32(this + 0x94, t);
    h.call(m, RENDER_LAYER, this, &[0]);
    let scene = m.r32(SCENE);
    let back = m.r32(scene.wrapping_add(0xC32C));
    if back != 0 {
        let ebp = cdecl(m, h, SGL_SET_MODE, &[0]);
        let buf = esp - 0x24;
        cdecl(m, h, SGL_SIZE_OF, &[back, buf]);
        let back = scene_at(m, 0xC32C);
        cdecl(m, h, SGL_SELECT_TEXTURE, &[back]);
        cdecl(m, h, SGL_SET_COLOR, &[u32::MAX]);
        let (a4, a8) = (m.r32(this + 0xA4), m.r32(this + 0xA8));
        let edi = cdecl(m, h, SGL_NEW_TEXTURE, &[a4, a8, 0x0500_0010]);
        cdecl(m, h, SGL_SET_TARGET, &[edi]);
        let back = scene_at(m, 0xC32C);
        let ebx = cdecl(m, h, SGL_RENDER_TARGET, &[back, 0]);
        let (a4, a8) = (m.r32(this + 0xA4), m.r32(this + 0xA8));
        let (b0, b1) = (m.r32(buf), m.r32(buf + 4));
        cdecl(m, h, SGL_STRETCH, &[0, 0, 0, 0, b0, b1, a4, a8]);
        let back = scene_at(m, 0xC32C);
        cdecl(m, h, SGL_RENDER_TARGET, &[back, ebx]);
        let lock = esp - 0x14;
        cdecl(m, h, SGL_LOCK, &[edi, lock, 3]);
        let mut p = m.r32(lock);
        let end = m.r32(this + 0xA8).wrapping_mul(m.r32(lock + 0x10)).wrapping_add(p);
        if p < end {
            loop {
                let v = m.r8(p);
                p = p.wrapping_add(1);
                let mapped = m.r8(this.wrapping_add(u32::from(v)).wrapping_add(0xC4));
                m.w8(p.wrapping_sub(1), mapped);
                if p >= end {
                    break;
                }
            }
        }
        cdecl(m, h, SGL_UNLOCK, &[edi]);
        cdecl(m, h, SGL_SELECT_TEXTURE, &[edi]);
        let t = m.r32(this + 0x94);
        cdecl(m, h, SGL_SET_TARGET, &[t]);
        let (a4, a8) = (m.r32(this + 0xA4), m.r32(this + 0xA8));
        cdecl(m, h, SGL_BLIT, &[0, 0, 0, 0, a4, a8]);
        cdecl(m, h, SGL_SET_MODE, &[ebp]);
        cdecl(m, h, SGL_FREE, &[edi]);
    }
    h.call(m, RENDER_LAYER, this, &[1]);
    let ebx = m.r32(this + 0x2C);
    let a4 = m.r32(this + 0xA4);
    let a8 = m.r32(this + 0xA8);
    if a4 > ebx || a8 > m.r32(this + 0x30) {
        let edi = m.r32(this + 0x30);
        let ebp = cdecl(m, h, SGL_NEW_TEXTURE, &[ebx, edi, 0]);
        let old = cdecl(m, h, SGL_SET_MODE, &[0]);
        m.w32(esp - 0x28, old);
        let t = m.r32(this + 0x94);
        cdecl(m, h, SGL_SELECT_TEXTURE, &[t]);
        cdecl(m, h, SGL_SET_TARGET, &[ebp]);
        let (a8, a4) = (m.r32(this + 0xA8), m.r32(this + 0xA4));
        cdecl(m, h, SGL_STRETCH, &[0, 0, 0, 0, a4, a8, ebx, edi]);
        let old = m.r32(esp - 0x28);
        cdecl(m, h, SGL_SET_MODE, &[old]);
        let t = m.r32(this + 0x94);
        cdecl(m, h, SGL_FREE, &[t]);
        m.w32(this + 0x94, ebp);
        m.w32(this + 0xA4, ebx);
        m.w32(this + 0xA8, edi);
    }
    let pos = this + 0x14;
    vcall(m, h, this, 0x68, &[pos]);
    let x = m.r32(pos);
    let y = m.r32(this + 0x18);
    let (w, hgt) = (m.r32(this + 0x2C), m.r32(this + 0x30));
    let r = esp - 0x24;
    m.w32(r + 4, y);
    m.w32(r + 8, w.wrapping_add(x));
    m.w32(r, x);
    m.w32(r + 0xC, hgt.wrapping_add(y));
    h.call(m, INVALIDATE, DIRTY, &[r])
}

/// Posición del punto de un barco en el minimapa: `0x58D030` sobre la posición del barco (dos palabras en `pt`), escalada a la textura.
fn ship_texel(m: &mut dyn Mem, h: &mut dyn Host, this: u32, ship: u32, pt: u32, out: u32) -> (u32, u32) {
    vcall(m, h, ship + 4, 0x2C, &[pt]);
    let scene = m.r32(SCENE);
    let (x, y) = (m.r32(pt), m.r32(pt + 4));
    h.call(m, SCENE_PROJECT, scene, &[x, y, out]);
    let (c0, c1) = (m.r32(out), m.r32(out + 4));
    scale_to_texture(m, this, scene, c0, c1)
}

/// `0x59D180`: avanza el parpadeo del marcador de barcos y marca como sucios los puntos de los barcos visibles.
fn update_frame(m: &mut dyn Mem, h: &mut dyn Host, this: u32, esp: u32) {
    h.call(m, 0x0059_D400, this, &[]);
    let mut t = m.r32(this + 0x1C4);
    if m.r32(TIMER) <= t {
        return;
    }
    t = t.wrapping_add(0x23);
    m.w32(this + 0x1C4, t);
    let now = m.r32(TIMER);
    if t < now {
        m.w32(this + 0x1C4, now);
    }
    let step = m.r8(this + 0x1C9) as i8;
    let cur = m.r8(this + 0x1C8);
    let sum = i32::from(cur) + i32::from(step);
    if sum > 0xFF {
        m.w8(this + 0x1C8, 0xFF);
        m.w8(this + 0x1C9, (step as u8).wrapping_neg());
    } else if sum < 0 {
        m.w8(this + 0x1C8, 0);
        m.w8(this + 0x1C9, (step as u8).wrapping_neg());
    } else {
        m.w8(this + 0x1C8, cur.wrapping_add(step as u8));
    }
    let mut ship = m.r32(SHIP_LIST);
    while ship != 0 {
        if m.r8(ship + 0x3A) != 0 {
            let (px, py) = ship_texel(m, h, this, ship, esp - 0x18, esp - 0x20);
            let r = esp - 0x10;
            m.w32(r, px);
            m.w32(r + 4, py);
            m.w32(r + 8, px.wrapping_add(3));
            m.w32(r + 0xC, py.wrapping_add(3));
            h.call(m, INVALIDATE, DIRTY, &[r]);
        }
        ship = m.r32(ship + 0x30);
    }
}

/// `0x59C590(destino, x, y, _)`: dibuja el minimapa: la textura del terreno, la zona resaltada, el rectángulo de la vista y un punto por barco.
fn draw(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) {
    let (ox, oy) = (a[1], a[2]);
    let rect = esp - 0x1C;
    let left = m.r32(this + 0x14).wrapping_add(ox);
    let top = m.r32(this + 0x18).wrapping_add(oy);
    m.w32(rect + 4, top);
    m.w32(rect, left);
    let (w, hg) = (m.r32(this + 0x2C), m.r32(this + 0x30));
    m.w32(rect + 0xC, top.wrapping_add(hg));
    m.w32(rect + 8, left.wrapping_add(w));
    if h.call(m, VISIBLE_CHECK, this, &[]) as u8 == 0 {
        return;
    }
    if cdecl(m, h, SGL_CLIP_GET, &[rect]) == 0 {
        return;
    }
    let r = cdecl(m, h, SGL_SET_MODE, &[3]);
    m.w32(esp + 8, r);
    cdecl(m, h, SGL_CLIP_SET, &[rect]);
    let old = m.r32(esp + 4);
    let r = cdecl(m, h, SGL_SET_TARGET, &[old]);
    m.w32(esp + 4, r);
    cdecl(m, h, SGL_SET_COLOR, &[u32::MAX]);
    let t = m.r32(this + 0x94);
    cdecl(m, h, SGL_SELECT_TEXTURE, &[t]);
    let obj = esp - 0x2C;
    h.call(m, LOCAL_OBJ_CTOR, obj, &[]);
    h.call(m, LOCAL_OBJ_SET, obj, &[0]);
    let v = h.call(m, LOCAL_OBJ_GET, obj, &[]);
    let cur = m.r32(SGL_CURRENT);
    if m.r32(cur.wrapping_add(0x7E4)) == v {
        let (p0, p1, a4, a8) = (m.r32(this + 0x98), m.r32(this + 0x9C), m.r32(this + 0xA4), m.r32(this + 0xA8));
        cdecl(m, h, SGL_BLIT, &[0, 0, p0, p1, a4, a8]);
        let ov = m.r32(this + 0xC0);
        if ov != 0 {
            cdecl(m, h, SGL_SELECT_TEXTURE, &[ov]);
            let (p0, p1, a4, a8) = (m.r32(this + 0x98), m.r32(this + 0x9C), m.r32(this + 0xA4), m.r32(this + 0xA8));
            cdecl(m, h, SGL_BLIT, &[0, 0, p0, p1, a4, a8]);
        }
    }
    let mode = cdecl(m, h, SGL_SET_ALPHA_MODE, &[3]) & 0xFF;
    cdecl(m, h, SGL_SET_COLOR, &[0xFF64_6464]);
    let (ac, b0, b4, b8) = (m.r32(this + 0xAC), m.r32(this + 0xB0), m.r32(this + 0xB4), m.r32(this + 0xB8));
    cdecl(m, h, SGL_LINE, &[ac, b0, b4, b0]);
    cdecl(m, h, SGL_LINE, &[b4, b0, b4, b8]);
    cdecl(m, h, SGL_LINE, &[b4, b8, ac, b8]);
    cdecl(m, h, SGL_LINE, &[ac, b8, ac, b0]);
    cdecl(m, h, SGL_SET_ALPHA_MODE, &[mode]);
    let mut ship = m.r32(SHIP_LIST);
    while ship != 0 {
        let (px, py) = ship_texel(m, h, this, ship, esp - 0x24, esp - 0x34);
        let color = m.r32(ship + 0x34);
        cdecl(m, h, SGL_SET_COLOR, &[color]);
        cdecl(m, h, SGL_FILL_RECT, &[px, py, 3, 3]);
        if m.r8(ship + 0x3A) != 0 {
            let v = u32::from(m.r8(this + 0x1C8));
            let c = (((v | 0xFFFF_8000) << 8 | v) << 8) | v;
            cdecl(m, h, SGL_SET_COLOR, &[c]);
            cdecl(m, h, SGL_FILL_RECT, &[px, py, 3, 3]);
        }
        ship = m.r32(ship + 0x30);
    }
    let t = m.r32(esp + 4);
    cdecl(m, h, SGL_SET_TARGET, &[t]);
    let t = m.r32(esp + 8);
    cdecl(m, h, SGL_SET_MODE, &[t]);
    h.call(m, LOCAL_OBJ_DTOR, obj, &[]);
}

/// `0x59D5E0(origen)`: crea la textura de la zona resaltada (2 bits por celda en un byte: 0 transparente, 1 el color de selección).
fn set_site_overlay(m: &mut dyn Mem, h: &mut dyn Host, this: u32, src: u32, esp: u32) -> u32 {
    let ov = m.r32(this + 0xC0);
    if ov != 0 {
        cdecl(m, h, SGL_FREE, &[ov]);
        m.w32(this + 0xC0, 0);
    }
    let (x, y) = (m.r32(this + 0x14), m.r32(this + 0x18));
    let (w, hgt) = (m.r32(this + 0x2C), m.r32(this + 0x30));
    let r = esp - 0x24;
    m.w32(r, x);
    m.w32(r + 4, y);
    m.w32(r + 8, w.wrapping_add(x));
    m.w32(r + 0xC, hgt.wrapping_add(y));
    let ret = h.call(m, INVALIDATE, DIRTY, &[r]);
    if src == 0 {
        return ret;
    }
    let (a4, a8) = (m.r32(this + 0xA4), m.r32(this + 0xA8));
    let tex = cdecl(m, h, SGL_NEW_TEXTURE, &[a4, a8, 3]);
    m.w32(this + 0xC0, tex);
    let lock = esp - 0x14;
    cdecl(m, h, SGL_LOCK, &[tex, lock, 2]);
    let mut pix = m.r32(lock);
    let scene = m.r32(SCENE);
    let width = m.r32(scene.wrapping_add(0xC314));
    let height = m.r32(scene.wrapping_add(0xC318));
    let color = m.r32(scene.wrapping_add(0x558));
    let table = esp - 0x24;
    m.w32(table, 0);
    m.w32(table + 4, color);
    let mut src = src;
    let mut row = 0u32;
    while row < height {
        let odd = row & 1;
        if odd != 0 {
            pix = pix.wrapping_add(4);
        }
        for _ in 0..width {
            let idx = u32::from(m.r8(src));
            let v = m.r32(table.wrapping_add(4 * idx));
            m.w32(pix, v);
            let idx = u32::from(m.r8(src));
            let v = m.r32(table.wrapping_add(4 * idx));
            m.w32(pix.wrapping_add(4), v);
            pix = pix.wrapping_add(8);
            src = src.wrapping_add(1);
        }
        if odd == 0 {
            pix = pix.wrapping_add(4);
        }
        row += 1;
    }
    let tex = m.r32(this + 0xC0);
    cdecl(m, h, SGL_UNLOCK, &[tex])
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    Some(match f {
        0x0059_C200 => Ret::U32(ctor(m, h, ecx)),
        0x0059_C340 => Ret::U32(ctor_with_name(m, h, ecx, a[0])),
        0x0059_C400 => Ret::U32(dtor(m, h, ecx)),
        0x0059_C410 => {
            reset(m, h, ecx);
            Ret::None
        }
        0x0059_C4A0 => Ret::U32(on_mouse_event(m, h, ecx, a)),
        0x0059_C570 => {
            h.call(m, SCOPE_GUARD, esp + 4, &[]);
            Ret::U8(0)
        }
        0x0059_C580 => {
            h.call(m, SCOPE_GUARD, esp + 4, &[]);
            Ret::U8(1)
        }
        0x0059_C930 => {
            Ret::U32(get_ship_dot_rect(m, h, ecx, a, esp))
        }
        0x0059_C590 => {
            draw(m, h, ecx, a, esp);
            Ret::None
        }
        0x0059_CA00 => Ret::U32(render_layer(m, h, ecx, a[0], esp)),
        0x0059_CEB0 => Ret::U32(rebuild_texture(m, h, ecx, esp)),
        0x0059_D180 => {
            update_frame(m, h, ecx, esp);
            Ret::None
        }
        0x0059_D5E0 => Ret::U32(set_site_overlay(m, h, ecx, a[0], esp)),
        0x0059_C9F0 | 0x0059_D170 => Ret::None,
        0x0059_D2F0 => Ret::U32(set_visible(m, h, ecx, a[0], esp)),
        0x0059_D350 => Ret::U32(set_position_xyz(m, h, ecx, a, esp)),
        0x0059_D380 => set_position(m, h, ecx, a[0]),
        0x0059_D400 => Ret::U32(update_view_rect(m, h, ecx, esp)),
        0x0059_D540 => {
            invalidate_rect_outline(m, h, a[0], esp);
            Ret::None
        }
        0x0059_D720 => {
            m.w8(ecx + 0xBC, 0);
            m.w8(ecx + 0x42, 0);
            Ret::U8(1)
        }
        0x0059_D730 => {
            let arg = m.r32(CLASS_NAME_ARG);
            h.call(m, STRING_INIT, a[0], &[arg]);
            Ret::U32(a[0])
        }
        0x0059_D760 => {
            dtor_call(m, h, ecx);
            if a[0] & 1 != 0 {
                cdecl(m, h, FREE, &[ecx]);
            }
            Ret::U32(ecx)
        }
        0x0059_D780 => Ret::U32(get_cell_color(m, h, a)),
        _ => return None,
    })
}

fn dtor_call(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    h.call(m, 0x0059_C400, this, &[]);
}
