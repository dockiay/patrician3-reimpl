//! `0x550550`: actualización del estado de daño/incendio de un edificio (objeto de estado de 0x10 bytes: `+4` valor actual, `+6` objetivo, `+0xC..0xF` marcas, `+0xE` signo).
//! Traducción 1:1 sobre memoria plana (`Mem`) con paridad en `tests/x86_fire_state.rs` frente al x86 original (`oracle/x86_fire_state.py`).
//! Externos como ganchos: `0x5506D0` (escribe en las tablas de objetos de dibujo), `_rand`, y el virtual `+0x20` del objeto de escena (repintado).
use crate::scene_math::Mem;

pub trait FireEnv {
    /// `0x5506D0(x, y, k - 1, p8)` (stdcall, 4 argumentos).
    fn f5506d0(&mut self, x: u32, y: u32, k: u8, p8: u32);
    /// `_rand()` (`0x639C03`).
    fn rand(&mut self) -> u32;
    /// Virtual `+0x20` de `param_7` con `(*param_4, *param_5)`.
    fn repaint(&mut self, x: u32, y: u32);
}

/// `0x550550(this = obj, &a, &b, &x, &y, flag, obj7, p8)`: `a`, `b`, `x`, `y` son direcciones de los valores de entrada (los lee como el original).
#[allow(clippy::too_many_arguments)]
pub fn update(m: &mut dyn Mem, obj: u32, a: u32, b: u32, x: u32, y: u32, flag: u8, p8: u32, env: &mut dyn FireEnv) {
    let (va, vb) = (m.r16(a), m.r16(b));
    if m.r16(obj + 4) == va && m.r16(obj + 6) == vb {
        return;
    }
    m.w16(obj + 6, vb);
    m.w16(obj + 4, va);
    let s0e = m.r8(obj + 0xE) as i8;
    let (px, py) = (m.r32(x), m.r32(y));
    let k = m.r8(p8 + 0x2A).wrapping_sub(1);
    if flag != 0 {
        if s0e < -1 {
            env.f5506d0(px, py, k, p8);
            if va < 0x32 {
                for i in 0..4u32 {
                    let base = if i < 2 { 0x006E_11E8 } else { 0x006E_1200 };
                    let tbl = m.r32(base + 8);
                    let low = m.r32(tbl + 4);
                    let span = u32::from(m.r16(base + 0x14)).wrapping_sub(low);
                    let r = env.rand();
                    m.w8(obj + 0xC + i, ((r % span) as u8).wrapping_add(low as u8));
                }
            } else {
                for i in 0..4 {
                    m.w8(obj + 0xC + i, 0);
                }
                let cur = m.r16(obj + 4);
                if cur > 0x32 {
                    m.w16(obj + 4, cur - 0x32);
                }
            }
        } else if m.r32(a) == 0 {
            env.f5506d0(px, py, k, p8);
            for i in 0..4 {
                m.w8(obj + 0xC + i, 0xFF);
            }
        }
    } else {
        let cur = m.r16(obj + 4);
        let tgt = m.r16(obj + 6);
        if u8::from(cur != tgt) + u8::from(s0e >= -1) != 0 {
            let d = (i32::from(tgt) / 10) as u16;
            m.w16(obj + 4, if d < cur { cur - d } else { 0 });
        }
    }
    env.repaint(m.r32(x), m.r32(y));
}

const TABLE_A: u32 = 0x006E_11E8;
const TABLE_B: u32 = 0x006E_1200;

pub trait PlaceEnv {
    /// `0x59B5B0(escena, &rect)` (recorte de ventana y marca; ya portada en `precip_step::clip_window`).
    fn clip(&mut self, rect: [i32; 4]);
}

/// `0x5854D0(tabla, x, y, p, idx)`: rectángulo de la pieza `idx` de la tabla colocada en `(x, y)` y marca de ventana.
pub fn place_piece(m: &mut dyn Mem, table: u32, x: i32, y: i32, p: u32, idx: u32, env: &mut dyn PlaceEnv) {
    let top = i32::from(m.r16(table + 0x12) as i16) - i32::from(m.r16(p + 6)) - i32::from(m.r16(p + 2) as i16) + y;
    let items = m.r32(table + 4).wrapping_add(idx.wrapping_mul(8));
    let bottom = i32::from(m.r16(items + 6)) + top;
    let left = i32::from(m.r16(table + 0x10) as i16) - i32::from(m.r16(p) as i16) + x;
    let right = i32::from(m.r16(items + 4)) + left;
    env.clip([left, top, right, bottom]);
}

/// `0x5506D0(x, y, k, p8)`: coloca las cuatro piezas de la pieza `k` del edificio en las dos tablas de dibujo (0 y 1 en la primera, 2 y 3 en la segunda).
pub fn place_tables(m: &mut dyn Mem, x: u32, y: u32, k: u8, p8: u32, env: &mut dyn PlaceEnv) {
    let items = m.r32(p8 + 0x10).wrapping_add(u32::from(k) * 8);
    let origin = m.r32(p8 + 0x18);
    let recs = m.r32(origin + 8);
    for i in 0..4u32 {
        let r = recs + 4 * i;
        let (ry, rx) = (m.r16(r), m.r16(r + 2));
        // `[+2]` es la fila y `[+0]` la columna del registro; el original los lee como (psVar4[1], psVar4[0]).
        if ry == 0 && rx == 0 {
            continue;
        }
        let table = if i < 2 { TABLE_A } else { TABLE_B };
        let piece = m.r32(table + 4);
        let w = m.r16(piece + 4) >> 1;
        let h = m.r16(piece + 6);
        let ox = m.r16(origin + 0x10).wrapping_sub(w).wrapping_add(rx);
        let oy = m.r16(origin + 0x12).wrapping_sub(h).wrapping_add(ry);
        m.w16(table + 0x10, ox);
        m.w16(table + 0x12, oy);
        place_piece(m, table, x as i32, y as i32, items, 0, env);
    }
}

/// Primitivas gráficas del juego a las que llaman `0x585550` y `0x550770` (saltos indirectos por `0x6DA8A8`, `0x6DA8E8`, `0x6DA8EC`, `0x6DAA28`).
pub trait Gfx {
    /// `0x4BB9C0(superficie)`.
    fn select(&mut self, surface: u32);
    /// `0x4BB880()`: color actual.
    fn color(&mut self) -> u32;
    /// `0x4BB870(color)`.
    fn set_color(&mut self, c: u32);
    /// `0x4BB330(sx, sy, dx, dy, ancho, alto)`.
    fn blit(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32);
}

fn s16(v: u16) -> i32 {
    i32::from(v as i16)
}

/// `0x585550(pack, x, y, rect, idx)`: vuelca la pieza `idx` del paquete recortada al rectángulo.
pub fn draw_piece(m: &mut dyn Mem, pack: u32, x: i32, y: i32, rect: u32, idx: i32, g: &mut dyn Gfx) {
    let items = m.r32(pack + 4).wrapping_add((idx as u32).wrapping_mul(8));
    let px = m.r16(pack + 0x10);
    let r0 = m.r16(rect);
    let (iw, ih) = (m.r16(items + 4), m.r16(items + 6));
    let right = (iw.wrapping_add(px)) as i16;
    if !((r0 as i16) < right) {
        return;
    }
    let r4 = m.r16(rect + 4);
    if !((px as i16) < (r4.wrapping_add(r0)) as i16) {
        return;
    }
    let surf = m.r32(pack + 0xC);
    g.select(surf);
    let dxs = s16(px.wrapping_sub(r0));
    let mut w = s16(iw);
    let mut off = 0;
    let mut left = dxs.wrapping_add(x);
    if left < x {
        off = -dxs;
        w = w.wrapping_add(dxs);
        left = x;
    }
    if (i32::from(r4 as i16)).wrapping_add(x) < w.wrapping_add(left) {
        w = i32::from(r4).wrapping_sub(left).wrapping_add(x);
    }
    let dy = s16(m.r16(pack + 0x12)).wrapping_sub(i32::from(m.r16(rect + 6))).wrapping_sub(s16(m.r16(rect + 2))).wrapping_add(y);
    let sx = i32::from(m.r16(items)).wrapping_add(off);
    let sy = i32::from(m.r16(items + 2));
    g.blit(sx, sy, left, dy, w, i32::from(ih));
}

/// `0x550770(estado, objeto, rect, &x, &y)`: dibuja el edificio dañado/incendiado. Devuelve verdadero si dibujó.
pub fn draw_fire(m: &mut dyn Mem, state: u32, obj: u32, rect: u32, px: u32, py: u32, g: &mut dyn Gfx) -> bool {
    let (cur, tgt) = (m.r16(state + 4), m.r16(state + 6));
    let sign = m.r8(state + 0xE) as i8;
    if u8::from(cur != tgt) + u8::from(sign >= -1) == 0 {
        return false;
    }
    let pack = m.r32(obj + 0x18);
    if pack == 0 {
        return false;
    }
    let h = i32::from(m.r16(obj + 0x28));
    let quot = (i32::from(cur) * h).checked_div(i32::from(tgt)).expect("0x550770: división entre cero (la excepción del x86)");
    let mut edi = quot
        .wrapping_sub(i32::from(m.r16(obj + 0x24)))
        .wrapping_add(i32::from(m.r16(rect + 6)))
        .wrapping_sub(h)
        .wrapping_add(i32::from(m.r16(rect + 2)));
    let (x, y) = (m.r32(px) as i32, m.r32(py) as i32);
    g.select(m.r32(pack + 0xC));
    if sign >= -1 {
        let c = g.color();
        g.set_color(((c >> 24) << 24) | 0x0028_2828);
        draw_piece(m, pack, x, y, rect, 0, g);
        g.set_color(c);
    } else {
        draw_piece(m, pack, x, y, rect, 0, g);
    }
    if sign != -1 && edi.wrapping_add(0x16) > 0 {
        let rh = i32::from(m.r16(rect + 6));
        if edi > rh {
            edi = rh;
        }
        let mut ebx = i32::from(m.r16(rect + 2)).wrapping_sub(edi).wrapping_add(rh);
        let mut ebp = (m.r32(py) as i32).wrapping_sub(edi);
        g.select(m.r32(obj + 0xC));
        if edi > 0 {
            g.blit(i32::from(m.r16(rect)), ebx, x, ebp, i32::from(m.r16(rect + 4)), edi);
        }
        let c = g.color();
        let a = (c >> 24) as f32;
        let step = a / 22.0f32;
        let mut f = a - step;
        let (r2, r6) = (i32::from(m.r16(rect + 2)), i32::from(m.r16(rect + 6)));
        while !(f < 1.0) {
            ebx -= 1;
            if ebx < r2 {
                break;
            }
            ebp -= 1;
            if ebx < r6 + r2 {
                let al = (f as i64) as u8;
                g.set_color((u32::from(al) << 24) | (c & 0x00FF_FFFF));
                g.blit(i32::from(m.r16(rect)), ebx, x, ebp, i32::from(m.r16(rect + 4)), 1);
            }
            f -= step;
        }
        g.set_color(c);
    }
    if sign >= 0 {
        let recs = m.r32(pack + 8);
        for i in 0..4u32 {
            let r = recs + 4 * i;
            let (ry, rx) = (m.r16(r + 2), m.r16(r));
            if ry == 0 && rx == 0 {
                continue;
            }
            let table = if i < 2 { TABLE_A } else { TABLE_B };
            let piece = m.r32(table + 4);
            let ox = m.r16(pack + 0x10).wrapping_sub(m.r16(piece + 4) >> 1).wrapping_add(ry);
            let oy = m.r16(pack + 0x12).wrapping_sub(m.r16(piece + 6)).wrapping_add(rx);
            m.w16(table + 0x10, ox);
            m.w16(table + 0x12, oy);
            let mark = m.r8(state + 0xC + i) as i8;
            draw_piece(m, table, x, y, rect, i32::from(mark), g); // `this` = la tabla (0x6E11E8/0x6E1200), no el paquete
        }
    }
    true
}
