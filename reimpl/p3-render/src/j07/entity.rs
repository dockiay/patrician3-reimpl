//! Entidades con sprite de la ciudad, sus conjuntos de sprites, la caché de texturas y los enlaces por casilla (`0x5576C0`–`0x559D20`).
//!
//! `SpriteEntity` (`0x20` bytes, vtable `0x674AE8`): `+0` enlaces por casilla (`EntityCellLinks`, `+4` tabla de enlaces de 4 bytes:
//! identificador `u16` y ranura siguiente `u8`), `+8` instancia (`SpriteInstance`, vtable `0x674B38`: `+0xC` dirección, `+0xD` animación,
//! `+0xE`/`+0x12` posición, `+0x10` bandera de máscara pendiente) y `+0x1C` cargador (vtable `0x674AD8`). La vtable `+0x4C` da el
//! `EntitySpriteSet`: `+0x14` direcciones, `+0x18`/`+0x1C` tablas de fotogramas y de máscaras por dirección, `+0x20` número de
//! animaciones por dirección, `+0x24` animaciones (12 bytes: número de partes, parte, partes), `+0x28`/`+0x2C` claves de animación,
//! `+0x30` propiedad de las partes y `+0x34`/`+0x38` texturas.
//!
//! Las llamadas a `UnionRect`, `CopyRect`, `IntersectRect` e `IsRectEmpty` pasan por las importaciones del ejecutable (el anfitrión
//! responde con la semántica de Windows) y `_stricmp` por su dirección del CRT; `strlen`, `strcpy` y `memset` se portan.

use super::sheet::{ARCHIVE_CLOSE, ARCHIVE_CTOR, ARCHIVE_DTOR, ARCHIVE_OPEN, ARCHIVE_READ, ARCHIVE_SEEK, CLIP_INTERSECTION,
    SGL_DRAW_BITMAP_RECT, SGL_SET_RENDER_SOURCE};
use super::sprite::{deleting_dtor, CRT_FREE, CRT_REALLOC, SHEET_FLAG_ALT, UI};
use super::{at32, cdecl, free, memset, new, vcall, Host, Mem, Ret};

pub const UNION_RECT: u32 = 0x0066_A4CC;
pub const COPY_RECT: u32 = 0x0066_A4A4;
pub const INTERSECT_RECT: u32 = 0x0066_A4E8;
pub const IS_RECT_EMPTY: u32 = 0x0066_A4D0;
pub const CRT_STRICMP: u32 = 0x0064_3DB0;
pub const SGL_FREE_MEMORY_TEXTURE: u32 = 0x004B_B490;
pub const SGL_SET_COLOR_KEY: u32 = 0x004B_B850;
pub const SGL2D_LOAD_MEMORY_TEXTURE: u32 = 0x004B_CA00;
pub const GET_FRAME_RECTS: u32 = 0x0055_9260;
pub const UNREGISTER_FROM_CELLS: u32 = 0x0055_96C0;
pub const PIXEL_TO_CELL: u32 = 0x0058_D030;
pub const REMOVE_ENTITY_FROM_CELL: u32 = 0x0059_3540;
pub const ANIM_SET_SKIP: u32 = 0x0058_5490;
/// Caché de texturas: tabla de pares (textura, nombre) y número.
pub const TEX_CACHE: u32 = 0x006E_16D0;
pub const TEX_CACHE_N: u32 = 0x006E_16D4;
/// Variante de dirección alternativa (`0x67792C`).
pub const DIR_ALT: u32 = 0x0067_792C;

/// Llamada a una función importada de `user32` a través de su entrada en la tabla de importaciones.
fn import(m: &mut dyn Mem, h: &mut dyn Host, iat: u32, args: &[u32]) -> u32 {
    let f = m.r32(iat);
    cdecl(m, h, f, args)
}

/// Índice de dirección: `((v * n + 0x80) >> 8) & (n - 1)`.
pub(super) fn dir_index(v: u8, n: u32) -> u32 {
    (u32::from(v).wrapping_mul(n).wrapping_add(0x80) >> 8) & n.wrapping_sub(1)
}

pub(super) fn sx16(m: &mut dyn Mem, a: u32) -> u32 {
    m.r16(a) as i16 as i32 as u32
}

/// Animación actual de la instancia en la tabla `tab` del conjunto (12 bytes por animación).
fn anim_entry(m: &mut dyn Mem, set: u32, tab: u32, inst: u32) -> u32 {
    let n = m.r32(set + 0x14);
    let i = dir_index(m.r8(inst + 0xC), n);
    let t = m.r32(set + tab);
    let e = m.r32(t.wrapping_add(i.wrapping_mul(4)));
    e.wrapping_add(u32::from(m.r8(inst + 0xD)) * 12)
}

/// `0x5576C0` (stdcall: conjunto, clave): índice de la animación con esa clave (5 bits) o `0xFF`.
fn find_animation_index(m: &mut dyn Mem, set: u32, key: u32) -> u8 {
    let n = m.r32(set + 0x28);
    if n == 0 {
        return 0xFF;
    }
    let tab = m.r32(set + 0x2C);
    let k = key & 0x1F;
    let mut i: u8 = 0;
    loop {
        if m.r32(tab.wrapping_add(u32::from(i) * 12 + 8)) == k {
            return i;
        }
        i = i.wrapping_add(1);
        if u32::from(i) >= n {
            return 0xFF;
        }
    }
}

fn strlen(m: &mut dyn Mem, s: u32) -> u32 {
    let mut n = 0u32;
    while m.r8(s.wrapping_add(n)) != 0 {
        n += 1;
    }
    n
}

fn strcpy(m: &mut dyn Mem, dst: u32, src: u32) {
    let mut i = 0u32;
    loop {
        let b = m.r8(src.wrapping_add(i));
        m.w8(dst.wrapping_add(i), b);
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// `0x557AF0`: libera todas las texturas de la caché y sus nombres.
fn tex_cache_free_all(m: &mut dyn Mem, h: &mut dyn Host) {
    if m.r32(TEX_CACHE) == 0 {
        return;
    }
    let mut i = 0;
    while i < m.r32(TEX_CACHE_N) {
        let t = at32(m, TEX_CACHE, i * 8);
        cdecl(m, h, SGL_FREE_MEMORY_TEXTURE, &[t]);
        let name = at32(m, TEX_CACHE, i * 8 + 4);
        free(m, h, name);
        i += 1;
    }
    let t = m.r32(TEX_CACHE);
    cdecl(m, h, CRT_FREE, &[t]);
    m.w32(TEX_CACHE, 0);
    m.w32(TEX_CACHE_N, 0);
}

/// `0x557BB0` (cdecl: nombre, salida, clave de color, banderas): textura de la caché con ese nombre o recién cargada.
fn tex_cache_get_or_load(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32]) -> u8 {
    let mut tex = 0;
    let mut i = 0;
    while i < m.r32(TEX_CACHE_N) {
        let name = at32(m, TEX_CACHE, i * 8 + 4);
        if cdecl(m, h, CRT_STRICMP, &[name, a[0]]) == 0 {
            tex = at32(m, TEX_CACHE, i * 8);
            break;
        }
        i += 1;
    }
    m.w32(a[1], tex);
    if tex != 0 {
        return 1;
    }
    let t = cdecl(m, h, 0x0055_7C20, &[a[0], a[2], a[3]]);
    m.w32(a[1], t);
    u8::from(t != 0)
}

/// Entrada `k` (desde el final: 1 = última) de la caché: `+0` textura, `+4` nombre.
fn tex_slot(m: &mut dyn Mem, back: u32, off: u32) -> u32 {
    let t = m.r32(TEX_CACHE);
    let n = m.r32(TEX_CACHE_N);
    t.wrapping_add(n.wrapping_mul(8)).wrapping_sub(back * 8).wrapping_add(off)
}

/// `0x557C20` (cdecl: nombre, clave de color, banderas): añade a la caché la textura cargada del archivo (o la retira si falla).
fn tex_cache_load(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32]) -> u32 {
    let n = m.r32(TEX_CACHE_N).wrapping_add(1);
    m.w32(TEX_CACHE_N, n);
    let t = m.r32(TEX_CACHE);
    let t = cdecl(m, h, CRT_REALLOC, &[t, n << 3]);
    m.w32(TEX_CACHE, t);
    let len = strlen(m, a[0]);
    let p = new(m, h, len + 1);
    let s = tex_slot(m, 1, 4);
    m.w32(s, p);
    let s = tex_slot(m, 1, 4);
    let dst = m.r32(s);
    strcpy(m, dst, a[0]);
    let s = tex_slot(m, 1, 4);
    let dst = m.r32(s);
    m.w8(dst.wrapping_add(len), 0);
    cdecl(m, h, SGL_SET_COLOR_KEY, &[a[1]]);
    let flags = (u32::from(a[1] != 0) * 4 + 3) | a[2];
    let tex = cdecl(m, h, SGL2D_LOAD_MEMORY_TEXTURE, &[a[0], flags]);
    let s = tex_slot(m, 1, 0);
    m.w32(s, tex);
    let s = tex_slot(m, 1, 0);
    if m.r32(s) != 0 {
        return m.r32(s);
    }
    let s = tex_slot(m, 1, 4);
    let name = m.r32(s);
    free(m, h, name);
    let n = m.r32(TEX_CACHE_N).wrapping_sub(1);
    let t = m.r32(TEX_CACHE);
    m.w32(TEX_CACHE_N, n);
    m.r32(t.wrapping_add(n.wrapping_mul(8)).wrapping_sub(8))
}

/// `0x558DB0` (cdecl: ruta, tabla de salida, número de salida): lee del archivo la tabla de `u32` contada en `0x1D`.
fn read_counted_at_1d(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) -> u8 {
    let file = esp - 0x14;
    h.call(m, ARCHIVE_CTOR, file, &[]);
    if h.call(m, ARCHIVE_OPEN, file, &[a[0], 0x8000]) == 0 {
        h.call(m, ARCHIVE_DTOR, file, &[]);
        return 0;
    }
    h.call(m, ARCHIVE_SEEK, file, &[0x1D, 0]);
    h.call(m, ARCHIVE_READ, file, &[a[2], 4]);
    let n = m.r32(a[2]);
    if n != 0 {
        let p = new(m, h, n.wrapping_mul(4));
        m.w32(a[1], p);
        let n = m.r32(a[2]);
        h.call(m, ARCHIVE_READ, file, &[p, n << 2]);
    }
    h.call(m, ARCHIVE_CLOSE, file, &[]);
    h.call(m, ARCHIVE_DTOR, file, &[]);
    1
}

/// `0x559000`: libera tablas de fotogramas, máscaras y animaciones (las partes solo si `+0x30`) y deja el conjunto vacío.
fn sprite_set_clear(m: &mut dyn Mem, h: &mut dyn Host, s: u32) {
    for tab in [0x18, 0x1C] {
        if m.r32(s + tab) != 0 {
            let mut i = 0;
            while i < m.r32(s + 0x14) {
                let p = at32(m, s + tab, i * 4);
                if p != 0 {
                    free(m, h, p);
                }
                i += 1;
            }
            let t = m.r32(s + tab);
            free(m, h, t);
            m.w32(s + tab, 0);
        }
    }
    if m.r32(s + 0x24) != 0 {
        if m.r8(s + 0x30) != 0 {
            let mut i = 0;
            while i < m.r32(s + 0x14) {
                let mut j = 0;
                while j < at32(m, s + 0x20, i * 4) {
                    let e = at32(m, s + 0x24, i * 4);
                    let p = m.r32(e.wrapping_add(j * 12 + 8));
                    free(m, h, p);
                    j += 1;
                }
                let e = at32(m, s + 0x24, i * 4);
                free(m, h, e);
                i += 1;
            }
        }
        let t = m.r32(s + 0x24);
        free(m, h, t);
        m.w32(s + 0x24, 0);
    }
    let p = m.r32(s + 0x2C);
    if p != 0 {
        free(m, h, p);
    }
    let p = m.r32(s + 0x20);
    m.w32(s + 0x2C, 0);
    if p != 0 {
        free(m, h, p);
    }
    for off in [0x20, 0x14, 4, 0x34, 0x38, 0x10, 8, 0xC, 0x3C] {
        m.w32(s + off, 0);
    }
    m.w8(s + 0x30, 0);
    m.w32(s + 0x28, 0);
}

/// `0x559130` (cdecl: rectángulo extra, conjunto, instancia): repinta la unión de los rectángulos del fotograma (y del extra).
fn set_invalidate(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) {
    let (r, r1, r2) = (esp - 0x30, esp - 0x20, esp - 0x10);
    if cdecl(m, h, GET_FRAME_RECTS, &[r1, r2, a[1], a[2]]) as u8 != 0 {
        import(m, h, UNION_RECT, &[r, r1, r2]);
    } else {
        import(m, h, COPY_RECT, &[r, r1]);
    }
    if a[0] != 0 {
        import(m, h, UNION_RECT, &[r, r, a[0]]);
    }
    let ui = m.r32(UI);
    h.call(m, CLIP_INTERSECTION, ui, &[r]);
}

/// `0x5591B0` (cdecl: conjunto, instancia): profundidad (y del pie) de la parte actual.
fn set_depth_y(m: &mut dyn Mem, a: &[u32]) -> u32 {
    let e = anim_entry(m, a[0], 0x24, a[1]);
    let k = m.r32(e + 4);
    let rec = m.r32(e + 8).wrapping_add(k.wrapping_mul(12));
    let base = u32::from(m.r16(rec + 6));
    let dy = sx16(m, rec + 0xA);
    let y = sx16(m, a[1] + 0xA);
    base.wrapping_add(dy).wrapping_add(y)
}

/// `0x559210` (cdecl: conjunto): desplazamiento superior del primer fotograma de la dirección por defecto.
fn set_top_offset(m: &mut dyn Mem, set: u32) -> u32 {
    let n = m.r32(set + 0x14);
    let mut i = dir_index(m.r8(SHEET_FLAG_ALT), n);
    if at32(m, set + 0x20, i * 4) == 0 {
        let n = m.r32(set + 0x14);
        i = dir_index(m.r8(DIR_ALT), n);
    }
    let f = at32(m, set + 0x18, i * 4);
    sx16(m, f + 0xA).wrapping_neg()
}

/// `0x559370` (cdecl: salida, conjunto, instancia): rectángulo que cubre el fotograma actual.
fn set_frame_bounds(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) -> u32 {
    let (r1, r2) = (esp - 0x20, esp - 0x10);
    if cdecl(m, h, GET_FRAME_RECTS, &[r1, r2, a[1], a[2]]) as u8 != 0 {
        import(m, h, UNION_RECT, &[a[0], r1, r2])
    } else {
        import(m, h, COPY_RECT, &[a[0], r1])
    }
}

/// `0x5593C0` (cdecl: conjunto, instancia): 1 si el fotograma se ve en la vista (rectángulo de la vista desplazado por `UI+0x18`).
fn set_is_on_screen(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) -> u32 {
    let (view, r, r1, r2) = (esp - 0x40, esp - 0x30, esp - 0x20, esp - 0x10);
    if cdecl(m, h, GET_FRAME_RECTS, &[r1, r2, a[0], a[1]]) as u8 != 0 {
        import(m, h, UNION_RECT, &[r, r1, r2]);
    } else {
        import(m, h, COPY_RECT, &[r, r1]);
    }
    let ui = m.r32(UI);
    vcall(m, h, ui, 0x3C, &[view]);
    let ui = m.r32(UI);
    let d = m.r32(ui + 0x18);
    let v = m.r32(view + 4).wrapping_add(d);
    m.w32(view + 4, v);
    let d = m.r32(ui + 0x18);
    let v = m.r32(view + 0xC).wrapping_add(d);
    m.w32(view + 0xC, v);
    import(m, h, INTERSECT_RECT, &[r, r, view]);
    u32::from(import(m, h, IS_RECT_EMPTY, &[r]) == 0)
}

/// `0x559470` (cdecl: conjunto, instancia): dibuja el fotograma actual en la posición de la instancia relativa a la vista.
fn set_draw_frame(m: &mut dyn Mem, h: &mut dyn Host, set: u32, inst: u32) {
    let tex = m.r32(set + 0x34);
    let fr = anim_entry(m, set, 0x18, inst);
    let ui = m.r32(UI);
    let (ox, oy) = (m.r32(ui + 0xC650), m.r32(ui + 0xC654));
    cdecl(m, h, SGL_SET_RENDER_SOURCE, &[tex]);
    let (hh, w) = (u32::from(m.r16(fr + 6)), u32::from(m.r16(fr + 4)));
    let ui = m.r32(UI);
    let r = vcall(m, h, ui, 0x70, &[]);
    let y = r.wrapping_add(sx16(m, fr + 0xA).wrapping_add(sx16(m, inst + 0xA)).wrapping_sub(oy));
    let ui = m.r32(UI);
    let r = vcall(m, h, ui, 0x6C, &[]);
    let x = r.wrapping_add(sx16(m, fr + 8).wrapping_add(sx16(m, inst + 6)).wrapping_sub(ox));
    let (rx, ry) = (u32::from(m.r16(fr)), u32::from(m.r16(fr + 2)));
    cdecl(m, h, SGL_DRAW_BITMAP_RECT, &[rx, ry, x, y, w, hh]);
}

/// `0x559520` (cdecl: conjunto, instancia): dibuja la máscara pendiente (`+0x10`) del fotograma actual y la da por dibujada.
fn set_draw_mask_frame(m: &mut dyn Mem, h: &mut dyn Host, set: u32, inst: u32) {
    if m.r8(inst + 0x10) == 0 || m.r32(set + 0x1C) == 0 {
        return;
    }
    let n = m.r32(set + 0x14);
    let i = dir_index(m.r8(inst + 0xC), n);
    let ui = m.r32(UI);
    let (ox, oy) = (m.r32(ui + 0xC650), m.r32(ui + 0xC654));
    let tex = m.r32(set + 0x38);
    cdecl(m, h, SGL_SET_RENDER_SOURCE, &[tex]);
    let e = at32(m, set + 0x1C, i * 4);
    let fr = e.wrapping_add(u32::from(m.r8(inst + 0xD)) * 12);
    let (hh, w) = (u32::from(m.r16(fr + 6)), u32::from(m.r16(fr + 4)));
    let ui = m.r32(UI);
    let y = sx16(m, fr + 0xA).wrapping_add(sx16(m, inst + 0xA)).wrapping_add(m.r32(ui + 0x18)).wrapping_sub(oy);
    let x = sx16(m, fr + 8).wrapping_add(sx16(m, inst + 6)).wrapping_add(m.r32(ui + 0x14)).wrapping_sub(ox);
    let (rx, ry) = (u32::from(m.r16(fr)), u32::from(m.r16(fr + 2)));
    cdecl(m, h, SGL_DRAW_BITMAP_RECT, &[rx, ry, x, y, w, hh]);
    m.w8(inst + 0x10, 0);
}

/// `0x5595F0` (cdecl: conjunto, instancia, salida): rectángulo del fotograma (en la salida o repintado) y baja de las casillas.
fn set_erase(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) -> u32 {
    let (r0, r1, r2) = (esp - 0x30, esp - 0x20, esp - 0x10);
    if a[2] != 0 {
        if cdecl(m, h, GET_FRAME_RECTS, &[r0, r1, a[0], a[1]]) as u8 != 0 {
            import(m, h, UNION_RECT, &[a[2], r0, r1]);
        } else {
            import(m, h, COPY_RECT, &[a[2], r0]);
        }
    } else {
        if cdecl(m, h, GET_FRAME_RECTS, &[r0, r2, a[0], a[1]]) as u8 != 0 {
            import(m, h, UNION_RECT, &[r1, r0, r2]);
        } else {
            import(m, h, COPY_RECT, &[r1, r0]);
        }
        let ui = m.r32(UI);
        h.call(m, CLIP_INTERSECTION, ui, &[r1]);
    }
    cdecl(m, h, UNREGISTER_FROM_CELLS, &[a[0], a[1]])
}

/// `0x5596C0` (cdecl: conjunto, instancia): quita la entidad de la casilla de cada parte de la animación, de la última a la primera.
fn set_unregister(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) {
    let (set, inst) = (a[0], a[1]);
    let e = anim_entry(m, set, 0x24, inst);
    let cnt = m.r32(e);
    let ox = sx16(m, inst + 6);
    let oy = sx16(m, inst + 0xA);
    let mut k = cnt.wrapping_sub(1) as i32;
    let (cell, idx) = (esp - 8, esp + 4);
    while k >= 0 {
        let rec = m.r32(e + 8).wrapping_add((k as u32).wrapping_mul(12));
        let px = sx16(m, rec + 8).wrapping_add(ox.wrapping_add(u32::from(m.r16(rec + 4)) >> 1));
        let dy = sx16(m, rec + 0xA);
        let py = u32::from(m.r16(rec + 6)).wrapping_add(oy.wrapping_add(dy));
        let ui = m.r32(UI);
        h.call(m, PIXEL_TO_CELL, ui, &[px, py, cell]);
        m.w8(idx, k as u8);
        let ui = m.r32(UI);
        h.call(m, REMOVE_ENTITY_FROM_CELL, ui, &[cell, cell + 4, inst + 0xE, idx]);
        k -= 1;
    }
}

/// Entidad `id` de la tabla de la interfaz (`UI+0x3FC`).
fn entity(m: &mut dyn Mem, id: u32) -> u32 {
    let ui = m.r32(UI);
    at32(m, ui + 0x3FC, id * 4)
}

/// `0x5599D0`/`0x559A10` (thiscall: ranura): dibuja la siguiente entidad de la casilla (`vtable+slot`) con su ranura siguiente.
fn links_draw_next(m: &mut dyn Mem, h: &mut dyn Host, this: u32, slot_arg: u32, vslot: u32) -> u32 {
    let l = m.r32(this + 4).wrapping_add((slot_arg & 0xFF) * 4);
    let id = u32::from(m.r16(l));
    if id == 0 {
        return 0;
    }
    let next = (l & 0xFFFF_FF00) | u32::from(m.r8(l + 2));
    let o = entity(m, id);
    vcall(m, h, o, vslot, &[next])
}

/// `0x559BB0` (cdecl: cabeza, identificador, ranura): quita de la cadena de la casilla el enlace de esa entidad y ranura.
fn links_remove(m: &mut dyn Mem, head: u32, id: u32, slot: u32) -> u8 {
    let ents = {
        let ui = m.r32(UI);
        m.r32(ui + 0x3FC)
    };
    let h0 = u32::from(m.r16(head));
    let mut e = m.r32(ents.wrapping_add(h0 * 4));
    let mut cur = m.r8(head + 2);
    let l0 = m.r32(e + 4).wrapping_add(u32::from(cur) * 4);
    if m.r16(l0) == 0 {
        m.w16(head, 0);
        m.w8(head + 2, 0);
        return 0;
    }
    let mut prev = 0u32;
    let mut prev_slot = 0u8;
    // El original recorre la cadena sin tope; una cadena cíclica (enlaces de entidades sin registrar) lo colgaría. Tope holgado:
    // la cadena de una casilla nunca pasa de unas decenas de enlaces.
    let mut steps = 0u32;
    loop {
        steps += 1;
        if steps > 65_536 {
            return 1;
        }
        if m.r16(e + 0x16) == id as u16 && cur == slot as u8 {
            break;
        }
        prev_slot = cur;
        prev = e;
        let l = m.r32(e + 4).wrapping_add(u32::from(cur) * 4);
        let nid = m.r16(l);
        if nid == 0 {
            return 1;
        }
        cur = m.r8(l + 2);
        e = m.r32(ents.wrapping_add(u32::from(nid) * 4));
        if e == 0 {
            return 1;
        }
    }
    let off = u32::from(cur) * 4;
    if prev != 0 {
        let po = u32::from(prev_slot) * 4;
        let src = m.r32(e + 4).wrapping_add(off);
        let v = m.r16(src);
        let d = m.r32(prev + 4).wrapping_add(po);
        m.w16(d, v);
        let src = m.r32(e + 4).wrapping_add(off + 2);
        let v = m.r8(src);
        let d = m.r32(prev + 4).wrapping_add(po + 2);
        m.w8(d, v);
    } else {
        let src = m.r32(e + 4).wrapping_add(off);
        let v = m.r16(src);
        m.w16(head, v);
        let src = m.r32(e + 4).wrapping_add(off + 2);
        let v = m.r8(src);
        m.w8(head + 2, v);
    }
    let l = m.r32(e + 4).wrapping_add(off);
    m.w8(l + 2, 0);
    let l = m.r32(e + 4).wrapping_add(off);
    m.w16(l, 0);
    1
}

fn links_free(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let p = m.r32(this + 4);
    if p != 0 {
        free(m, h, p);
    }
    m.w32(this + 4, 0);
}

fn instance_reset(m: &mut dyn Mem, this: u32) {
    m.w32(this, 0x0067_4B54);
    m.w32(this + 4, 0);
    m.w32(this + 8, 0);
    let v = m.r8(0x0067_792B);
    m.w8(this + 0xC, v);
    m.w8(this + 0xD, 0);
    m.w16(this + 0xE, 0);
    m.w8(this + 0x10, 1);
}

/// Conjunto de sprites de la entidad (`vtable+0x4C`).
fn set_of(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    vcall(m, h, this, 0x4C, &[])
}

fn inst_of(this: u32) -> u32 {
    if this != 0 {
        this + 8
    } else {
        0
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    let r = match f {
        0x5576C0 => Ret::U8(find_animation_index(m, a[0], a[1])),
        0x557A00 => {
            h.call(m, 0x0055_9910, ecx, &[]);
            h.call(m, 0x0055_8F00, ecx + 8, &[]);
            h.call(m, 0x0055_8750, ecx + 0x1C, &[]);
            m.w32(ecx + 8, 0x0067_4B38);
            m.w32(ecx, 0x0067_4AE8);
            m.w32(ecx + 0x1C, 0x0067_4AD8);
            Ret::U32(ecx)
        }
        0x557A70 => {
            m.w32(ecx, 0x0067_4AE8);
            m.w32(ecx + 8, 0x0067_4B38);
            m.w32(ecx + 0x1C, 0x0067_4AD8);
            h.call(m, 0x0055_8760, ecx + 0x1C, &[]);
            h.call(m, 0x0055_8F30, ecx + 8, &[]);
            Ret::U32(h.call(m, 0x0055_9920, ecx, &[]))
        }
        0x557AF0 => {
            tex_cache_free_all(m, h);
            Ret::None
        }
        0x557BB0 => Ret::U8(tex_cache_get_or_load(m, h, a)),
        0x557C20 => Ret::U32(tex_cache_load(m, h, a)),
        0x558550 => {
            let s = set_of(m, h, ecx);
            Ret::U32(h.call(m, 0x0055_9000, s, &[]))
        }
        0x558560 | 0x558590 => {
            let inst = inst_of(ecx);
            let s = set_of(m, h, ecx);
            let (draw, next) = if f == 0x558560 { (0x0055_9470, 0x0055_99D0) } else { (0x0055_9520, 0x0055_9A10) };
            cdecl(m, h, draw, &[s, inst]);
            Ret::U32(h.call(m, next, ecx, &[a[0]]))
        }
        0x5585C0 => {
            let s = set_of(m, h, ecx);
            Ret::U32(h.call(m, 0x0055_8770, ecx + 0x1C, &[a[0], s, 0]))
        }
        0x5585E0 => {
            let inst = inst_of(ecx);
            let s = set_of(m, h, ecx);
            Ret::U32(cdecl(m, h, 0x0055_93C0, &[s, inst]))
        }
        0x558610 => {
            let inst = inst_of(ecx);
            let s = set_of(m, h, ecx);
            Ret::U32(cdecl(m, h, 0x0055_9130, &[0, s, inst]))
        }
        0x558640 => {
            let s = set_of(m, h, ecx);
            Ret::U8(m.r8(s + 8))
        }
        0x558650 => {
            let s = set_of(m, h, ecx);
            Ret::U32(m.r32(s + 0x34))
        }
        0x558660 => {
            let s = set_of(m, h, ecx);
            let n = m.r32(s + 0x14);
            let i = dir_index(m.r8(ecx + 0x14), n);
            let s = set_of(m, h, ecx);
            let t = m.r32(s + 0x18);
            let e = m.r32(t.wrapping_add(i * 4));
            Ret::U32(e.wrapping_add(u32::from(m.r8(ecx + 0x15)) * 12))
        }
        0x5586B0 => {
            let x = sx16(m, ecx + 0xE);
            m.w32(a[0], x);
            let y = sx16(m, ecx + 0x12);
            m.w32(a[0] + 4, y);
            Ret::None
        }
        0x5586D0 => {
            let inst = inst_of(ecx);
            let s = set_of(m, h, ecx);
            Ret::U32(cdecl(m, h, 0x0055_91B0, &[s, inst]))
        }
        0x558700 => {
            let s = set_of(m, h, ecx);
            Ret::U32(cdecl(m, h, 0x0055_9210, &[s]))
        }
        0x558710 => Ret::U32(deleting_dtor(m, h, ecx, 0x0055_7A70, a[0])),
        0x558730 => Ret::U32(h.call(m, 0x0055_8710, ecx.wrapping_sub(8), &[a[0]])),
        0x558740 => Ret::U32(h.call(m, 0x0055_8710, ecx.wrapping_sub(0x1C), &[a[0]])),
        0x558750 => {
            m.w32(ecx, 0x0067_4B44);
            Ret::U32(ecx)
        }
        0x558760 => {
            m.w32(ecx, 0x0067_4B44);
            Ret::None
        }
        0x558D00 => {
            h.call(m, ARCHIVE_SEEK, a[0], &[4, 0]);
            h.call(m, ARCHIVE_READ, a[0], &[a[1], 4]);
            Ret::U8(1)
        }
        0x558DB0 => Ret::U8(read_counted_at_1d(m, h, a, esp)),
        0x558E90 => {
            let p = m.r32(a[0]);
            let n = m.r32(p);
            m.w32(a[0], p.wrapping_add(n.wrapping_mul(4)).wrapping_add(4));
            Ret::U8(1)
        }
        0x558EB0 => {
            let p = m.r32(a[0]);
            let n = m.r32(p);
            m.w32(a[0], p.wrapping_add(4));
            for _ in 0..n {
                cdecl(m, h, ANIM_SET_SKIP, &[a[0]]);
            }
            Ret::U8(1)
        }
        0x558EE0 => {
            m.w32(ecx, 0x0067_4B44);
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        0x558F00 => {
            instance_reset(m, ecx);
            Ret::U32(ecx)
        }
        0x558F30 => {
            instance_reset(m, ecx);
            Ret::None
        }
        0x558F80 => {
            instance_reset(m, ecx);
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        0x558FC0 => {
            m.w32(ecx, 0x0067_4B5C);
            for off in [0x1C, 0x18, 0x20, 0x24, 0x2C] {
                m.w32(ecx + off, 0);
            }
            h.call(m, 0x0055_9000, ecx, &[]);
            Ret::U32(ecx)
        }
        0x559000 => {
            sprite_set_clear(m, h, ecx);
            Ret::None
        }
        0x559130 => {
            set_invalidate(m, h, a, esp);
            Ret::None
        }
        0x5591B0 => Ret::U32(set_depth_y(m, a)),
        0x559210 => Ret::U32(set_top_offset(m, a[0])),
        0x559370 => Ret::U32(set_frame_bounds(m, h, a, esp)),
        0x5593C0 => Ret::U32(set_is_on_screen(m, h, a, esp)),
        0x559470 => {
            set_draw_frame(m, h, a[0], a[1]);
            Ret::None
        }
        0x559520 => {
            set_draw_mask_frame(m, h, a[0], a[1]);
            Ret::None
        }
        0x5595F0 => {
            set_erase(m, h, a, esp);
            Ret::None
        }
        0x5596C0 => {
            set_unregister(m, h, a, esp);
            Ret::None
        }
        0x5598E0 => {
            m.w32(ecx, 0x0067_4B5C);
            h.call(m, 0x0055_9000, ecx, &[]);
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        0x559910 => {
            m.w32(ecx, 0x0067_4B64);
            m.w32(ecx + 4, 0);
            Ret::U32(ecx)
        }
        0x559920 => {
            m.w32(ecx, 0x0067_4B64);
            links_free(m, h, ecx);
            Ret::None
        }
        0x559950 => {
            links_free(m, h, ecx);
            Ret::None
        }
        0x559970 => {
            let l = m.r32(ecx + 4).wrapping_add(u32::from(m.r8(a[0])) * 4);
            let id = u32::from(m.r16(l));
            if id == 0 {
                Ret::U32(0)
            } else {
                let s = m.r8(l + 2);
                m.w8(a[0], s);
                Ret::U32(entity(m, id))
            }
        }
        0x5599D0 => Ret::U32(links_draw_next(m, h, ecx, a[0], 8)),
        0x559A10 => Ret::U32(links_draw_next(m, h, ecx, a[0], 0xC)),
        0x559BB0 => Ret::U8(links_remove(m, a[0], a[1], a[2])),
        0x559CC0 => {
            let n = a[0].wrapping_mul(4);
            let p = new(m, h, n);
            m.w32(ecx + 4, p);
            memset(m, p, 0, n);
            Ret::U32(p)
        }
        0x559D20 => {
            m.w32(ecx, 0x0067_4B64);
            links_free(m, h, ecx);
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        _ => return None,
    };
    Some(r)
}
