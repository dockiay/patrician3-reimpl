//! Sprites de edificio (`Sprite`, vtables `0x673D10`/`0x673D0C`), animadores base y caché de texturas (`0x54E2D0`–`0x54F4F0`).
//!
//! `Sprite`: `+0` vtable, `+4` estado de incendio (`BuildingBurnState`, vtable secundaria `0x673D0C`), `+0x14` recurso, `+0x18` índice de
//! hoja en la tabla global `0x6DFCC0` (entradas de `0x3C` bytes; `+0x10` textura, `+0x18` animación de reserva, `+0x28` altura). Las
//! llamadas virtuales de los portes leen la vtable real del objeto; las ranuras usadas son `+0x20` (dibujo), `+0x50` (tipo de hoja) y
//! `+0x74` (hoja).
//!
//! Caché de texturas: global `0x6E11D8` (tabla de entradas de 16 bytes: textura, banderas, paleta copiada de `0x400` bytes y nombre) y
//! `0x6E11DC` (número de entradas).

use super::{cdecl, free, memcpy, new, vcall, Host, Mem, Ret};

pub const SHEETS: u32 = 0x006D_FCC0;
pub const TEXTURES: u32 = 0x006E_11D8;
pub const TEXTURE_COUNT: u32 = 0x006E_11DC;
/// Byte global que compara el tipo de hoja (`0x67792F`) y su alternativa (`0x677929`).
pub const SHEET_FLAG: u32 = 0x0067_792F;
pub const SHEET_FLAG_ALT: u32 = 0x0067_7929;
/// Puntero global a la interfaz (`0x6E51AC`): `+0x570` variante de hoja, `+0x4F0` bandera.
pub const UI: u32 = 0x006E_51AC;

pub const TOWN_SPRITE_CTOR: u32 = 0x0055_2B50;
pub const TOWN_SPRITE_DTOR: u32 = 0x0055_2BB0;
pub const ANIM_SET_LOAD: u32 = 0x0058_52A0;
pub const ARCHIVE_CTOR: u32 = 0x0040_2740;
pub const ARCHIVE_DTOR: u32 = 0x0040_2750;
pub const ARCHIVE_OPEN: u32 = 0x0040_2760;
pub const ARCHIVE_CLOSE: u32 = 0x0040_2780;
pub const READ_U32_AT_4: u32 = 0x0055_12D0;
pub const READ_ARRAY_AT_24: u32 = 0x0055_1330;
pub const READER_LOAD_SHEET: u32 = 0x0055_1000;
pub const ANIMATION_CTOR: u32 = 0x0058_5220;
pub const BURN_UPDATE: u32 = 0x0055_0550;
pub const SHEET_RESET: u32 = 0x0055_1B40;
pub const SHEET_FOOTPRINT: u32 = 0x0055_1C20;
pub const SHEET_OVERLAY: u32 = 0x0055_1F70;
pub const SHEET_CLIP_OVERLAY: u32 = 0x0055_1ED0;
pub const SHEET_CLIP_FRAME: u32 = 0x0055_1E60;
pub const SHEET_SELECT_VARIANT: u32 = 0x0055_2090;
pub const SECONDARY_INFO: u32 = 0x0055_04C0;
pub const BURNT_HEIGHT: u32 = 0x0055_0520;
pub const SHEET_DRAW_LAYERS: u32 = 0x0055_1CF0;
pub const RESOURCE_CREATE_ANIMATORS: u32 = 0x0055_2E80;
pub const RESOURCE_DESTROY_ANIMATORS: u32 = 0x0055_3070;
pub const SPRITE_DTOR: u32 = 0x0054_E2F0;
pub const SPRITE_SDD: u32 = 0x0054_EAE0;
pub const ANIM_STATE_CTOR: u32 = 0x0055_17B0;
pub const ANIM_STATE_DTOR: u32 = 0x0055_1800;
pub const ANIM_STATE_RESET: u32 = 0x0055_1810;
pub const SPRITE_ANIMATOR_DTOR: u32 = 0x0054_EBC0;
pub const GROUPED_ANIMATOR_DTOR: u32 = 0x0054_EE10;
pub const READER_CTOR: u32 = 0x0055_0FE0;
pub const READER_CTOR_ALT: u32 = 0x0055_0FF0;
pub const SGL_FREE_MEMORY_TEXTURE: u32 = 0x004B_B490;
pub const CRT_FREE: u32 = 0x0063_A1B6;
pub const CRT_REALLOC: u32 = 0x0063_A29F;
pub const CRT_STRLEN: u32 = 0x0063_ABE0;
pub const CRT_STRCPY: u32 = 0x0063_ADD0;
pub const CRT_STRICMP: u32 = 0x0064_3DB0;
pub const CLEAR_FLAMES: u32 = 0x0055_1C00;
pub const LOAD_TEXTURE: u32 = 0x0054_F330;
pub const SGL_SET_COLOR_KEY: u32 = 0x004B_B850;
pub const SGL_SET_ARCHIVE: u32 = 0x004B_B790;
pub const SGL_LOAD_MEMORY_TEXTURE: u32 = 0x004B_B5C0;
pub const SGL_LOCK_TEXTURE: u32 = 0x004B_BB00;
pub const SGL_UNLOCK_TEXTURE: u32 = 0x004B_BB10;
pub const SGL_CHANGE_PALETTE: u32 = 0x004B_B210;
/// Archivo de recursos activo (`0x6DCCC8`).
pub const ARCHIVE_GLOBAL: u32 = 0x006D_CCC8;

/// Tipo de hoja (`vtable+0x50`): `0x2A`, `0x50` o `0x51` desplazan el índice 0, 3 o 6.
fn kind_offset(kind: u32) -> Option<u8> {
    match kind {
        0x2A => Some(0),
        0x50 => Some(3),
        0x51 => Some(6),
        _ => None,
    }
}

/// `0x54E2D0`.
pub fn ctor(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    h.call(m, TOWN_SPRITE_CTOR, this, &[]);
    m.w32(this, 0x0067_3D10);
    m.w32(this + 4, 0x0067_3D0C);
    m.w8(this + 0x18, 0);
    this
}

/// `0x54E2F0`.
pub fn dtor(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    m.w32(this, 0x0067_3D10);
    m.w32(this + 4, 0x0067_3D0C);
    h.call(m, TOWN_SPRITE_DTOR, this, &[])
}

/// `0x54E310` (stdcall): trozo del fichero de hoja; los tipos 3 y 4 cargan un conjunto de animaciones (`0x6E11E8`/`0x6E1200`).
pub fn parse_chunk(m: &mut dyn Mem, h: &mut dyn Host, cursor: u32, obj: u32) -> u8 {
    let at = m.r32(cursor);
    let tag = m.r32(at);
    m.w32(cursor, at.wrapping_add(4));
    let set = match tag {
        3 => 0x006E_11E8,
        4 => 0x006E_1200,
        _ => return 0,
    };
    let arg = m.r32(obj + 0xC);
    h.call(m, ANIM_SET_LOAD, set, &[cursor, arg, 0x0300_0000]);
    1
}

/// `0x54E370`: lee del fichero el tipo y la variante de la hoja (dos lectores de `0x5512D0`/`0x551330` sobre un `ArchiveFile` en la
/// pila) y la carga con `0x551000`. `esp` es la pila de entrada.
pub fn load_sheet_from_file(m: &mut dyn Mem, h: &mut dyn Host, this: u32, path: u32, esp: u32) -> u8 {
    let p = esp.wrapping_sub(0x2C);
    let file = p + 0x18;
    h.call(m, ARCHIVE_CTOR, file, &[]);
    if h.call(m, ARCHIVE_OPEN, file, &[path, 0x8000]) == 0 {
        h.call(m, ARCHIVE_DTOR, file, &[]);
        return 0;
    }
    m.w32(p + 0xC, 0);
    m.w32(p + 0x10, 0);
    m.w32(p + 0x14, 0);
    cdecl(m, h, READ_U32_AT_4, &[file, p + 0xC]);
    cdecl(m, h, READ_ARRAY_AT_24, &[file, p + 0x10, p + 0x14]);
    h.call(m, ARCHIVE_CLOSE, file, &[]);
    if let Some(o) = kind_offset(m.r32(p + 0xC)) {
        m.w8(this + 0x18, o);
    }
    let arr = m.r32(p + 0x10);
    let mut idx = m.r8(this + 0x18).wrapping_add(m.r8(arr + 4));
    m.w8(this + 0x18, idx);
    if m.r32(arr + 8) == u32::from(m.r8(SHEET_FLAG)) {
        idx = idx.wrapping_add(9);
        m.w8(this + 0x18, idx);
    }
    let variant = if m.r32(p + 0x14) >= 4 { m.r32(arr + 0xC) as u8 } else { 2 };
    let idx = m.r8(this + 0x18).wrapping_add(variant.wrapping_mul(0x12));
    m.w8(this + 0x18, idx);
    free(m, h, arr);
    let sheet = SHEETS + u32::from(m.r8(this + 0x18)) * 0x3C;
    let ok = h.call(m, READER_LOAD_SHEET, this, &[path, sheet]) as u8;
    h.call(m, ARCHIVE_DTOR, file, &[]);
    ok
}

/// `0x54E4C0`: variante dentro del tipo (`índice % 9` menos 0, 3 o 6; `0xFF` si el tipo es otro).
pub fn get_variant_in_kind(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u8 {
    let v = m.r8(this + 0x18) % 9;
    match kind_offset(vcall(m, h, this, 0x50, &[])) {
        Some(o) => v.wrapping_sub(o),
        None => 0xFF,
    }
}

/// `0x54E520` (stdcall).
pub fn set_sheet_index(m: &mut dyn Mem, this: u32, kind: u32, index: u32, flag: u32) {
    if let Some(o) = kind_offset(kind) {
        m.w8(this + 0x18, (index as u8).wrapping_add(o));
    }
    if flag as u8 == m.r8(SHEET_FLAG) {
        let v = m.r8(this + 0x18).wrapping_add(9);
        m.w8(this + 0x18, v);
    }
    let ui = m.r32(UI);
    let mut variant = m.r32(ui + 0x570);
    let idx = m.r8(this + 0x18);
    let entry = (u32::from(idx).wrapping_add(variant.wrapping_mul(18))).wrapping_mul(0x3C).wrapping_add(SHEETS + 0x10);
    if m.r32(entry) == 0 {
        variant = 2;
    }
    m.w8(this + 0x18, (variant as u8).wrapping_mul(0x12).wrapping_add(idx));
}

/// `0x54E5A0`.
pub fn get_format_flag(m: &mut dyn Mem, this: u32) -> u8 {
    let idx = m.r8(this + 0x18) % 18;
    if idx < 9 {
        m.r8(SHEET_FLAG_ALT)
    } else {
        m.r8(SHEET_FLAG)
    }
}

fn sheet_of(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    vcall(m, h, this, 0x74, &[])
}

/// `0x54E850`.
pub fn release_sheet_data(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    let s = sheet_of(m, h, this);
    h.call(m, SHEET_RESET, s, &[])
}

/// `0x54E860`, `0x54E880`, `0x54E8A0`, `0x54E8C0`: reenvían a la hoja (cdecl con la hoja como cuarto argumento).
pub fn forward_sheet(m: &mut dyn Mem, h: &mut dyn Host, this: u32, target: u32, a: &[u32]) -> u32 {
    let s = sheet_of(m, h, this);
    cdecl(m, h, target, &[a[0], a[1], a[2], s])
}

/// `0x54E8F0`..`0x54E970`: campos de la hoja.
pub fn sheet_field(m: &mut dyn Mem, h: &mut dyn Host, this: u32, f: u32) -> Ret {
    let s = sheet_of(m, h, this);
    match f {
        0x54E8F0 => Ret::U32(u32::from(m.r16(s + 0x26))),
        0x54E900 => Ret::U32(u32::from(m.r16(s + 0x28))),
        0x54E910 => Ret::U32(u32::from(m.r8(s + 0x2A))),
        0x54E920 => Ret::U32(u32::from(m.r8(s + 0x2B))),
        0x54E930 => Ret::U32(u32::from(m.r8(s + 0x2D))),
        0x54E940 => Ret::U32(m.r32(s + 0x14)),
        0x54E950 => Ret::U32(m.r32(s + 0x10)),
        0x54E960 => Ret::U16(m.r16(s + 0x20)),
        0x54E970 => Ret::U32(m.r32(s + 0xC)),
        _ => unreachable!(),
    }
}

/// `0x54E9C0`: 1 si cambió el fotograma (`+8 != +0xA`) más 1 si `+0x12` (con signo) es al menos −1.
pub fn get_redraw_level(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r16(this + 8) != m.r16(this + 0xA)) + u32::from(m.r8(this + 0x12) as i8 >= -1)
}

/// `0x54E5E0` (5 argumentos: tipo, ?, x, y, bandera): con la bandera, si la hoja no tiene animación de reserva (`+0x18` de la entrada) la copia de la variante base; después,
/// para tipos hasta `0x2C` con variante no nula, dibuja la variante base (`vtable+0x20`) y vuelve al índice original; por último actualiza
/// el estado de incendio (`0x550550`) con punteros a sus propios argumentos.
pub fn draw_with_fallback(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) {
    let (kind, x, y, flag) = (a[0], a[2], a[3], a[4]);
    if flag as u8 != 0 {
        let idx = m.r8(this + 0x18);
        if m.r32(SHEETS + 0x18 + u32::from(idx) * 0x3C) == 0 {
            let base = idx % 9;
            let v = match kind_offset(vcall(m, h, this, 0x50, &[])) {
                Some(o) => base.wrapping_sub(o),
                None => 0xFF,
            };
            let src_idx = u32::from(m.r8(this + 0x18)).wrapping_sub(u32::from(v));
            let src_sheet = SHEETS.wrapping_add(src_idx.wrapping_mul(0x3C));
            let src = m.r32(src_sheet.wrapping_add(0x18));
            let anim = new(m, h, 0x18);
            let anim = if anim != 0 { h.call(m, ANIMATION_CTOR, anim, &[]) } else { 0 };
            let idx = u32::from(m.r8(this + 0x18));
            m.w32(SHEETS + 0x18 + idx * 0x3C, anim);
            let w = m.r16(src + 0x14);
            m.w16(anim + 0x14, w);
            let n = m.r16(src + 0x16);
            m.w16(anim + 0x16, n);
            let c = m.r32(src + 0xC);
            m.w32(anim + 0xC, c);
            let n4 = u32::from(m.r16(src + 0x16)) << 2;
            let p = new(m, h, n4);
            m.w32(anim + 8, p);
            let n4 = u32::from(m.r16(src + 0x16)) << 2;
            let from = m.r32(src + 8);
            memcpy(m, p, from, n4);
            let n8 = u32::from(m.r16(src + 0x14)) << 3;
            let q = new(m, h, n8);
            m.w32(anim + 4, q);
            let n8 = u32::from(m.r16(src + 0x14)) << 3;
            let from = m.r32(src + 4);
            memcpy(m, q, from, n8);
            let v10 = m.r16(src + 0x10);
            m.w16(anim + 0x10, v10);
            let idx = u32::from(m.r8(this + 0x18));
            let hh = m.r16(SHEETS + 0x28 + idx * 0x3C).wrapping_sub(m.r16(src_sheet.wrapping_add(0x28))).wrapping_add(m.r16(src + 0x12));
            m.w16(anim + 0x12, hh);
        }
        if kind <= 0x2C {
            let base = m.r8(this + 0x18) % 9;
            let k = vcall(m, h, this, 0x50, &[]);
            let v = match kind_offset(k) {
                Some(o) => Some(base.wrapping_sub(o)),
                None => None,
            };
            if v != Some(0) {
                vcall(m, h, this, 0x20, &[x, y]);
                let base = m.r8(this + 0x18) % 9;
                let v = match kind_offset(vcall(m, h, this, 0x50, &[])) {
                    Some(o) => base.wrapping_sub(o),
                    None => 0xFF,
                };
                let idx = m.r8(this + 0x18).wrapping_sub(v);
                m.w8(this + 0x18, idx);
            }
        }
    }
    // Llamada final con punteros a los argumentos de la pila: (&kind, &x, &y, out, this, hoja) más `&flag`.
    let sheet = SHEETS + u32::from(m.r8(this + 0x18)) * 0x3C;
    h.call(m, BURN_UPDATE, this + 4, &[esp + 4, esp + 8, esp + 0xC, esp + 0x10, flag, this, sheet]);
}

/// `0x54E980`.
pub fn select_texture_level(m: &mut dyn Mem, h: &mut dyn Host, this: u32, level: u32) -> u32 {
    let s = sheet_of(m, h, this);
    cdecl(m, h, SHEET_SELECT_VARIANT, &[s, level])
}

/// `0x54E9A0`.
pub fn get_image_b_rect(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: u32) -> u32 {
    let s = sheet_of(m, h, this);
    h.call(m, SECONDARY_INFO, this + 4, &[s, a])
}

/// `0x54E9E0`.
pub fn get_scaled_height(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    let s = sheet_of(m, h, this);
    h.call(m, BURNT_HEIGHT, this + 4, &[s])
}

/// `0x54EA00`.
pub fn clip_and_register_rect(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: u32, b: u32) -> u32 {
    let res = m.r32(this + 0x14);
    let s = sheet_of(m, h, this);
    cdecl(m, h, SHEET_DRAW_LAYERS, &[a, b, s, res])
}

/// `0x54EA20`.
pub fn is_ui_flag_4f0_set(m: &mut dyn Mem) -> u8 {
    let ui = m.r32(UI);
    u8::from(m.r32(ui + 0x4F0) != 0)
}

/// `0x54EAC0`.
pub fn get_sheet(m: &mut dyn Mem, this: u32) -> u32 {
    SHEETS + u32::from(m.r8(this + 0x18)) * 0x3C
}

/// Destructor con borrado (`scalar deleting destructor`): llama al destructor `dtor` y libera si `flags & 1`.
pub fn deleting_dtor(m: &mut dyn Mem, h: &mut dyn Host, this: u32, dtor: u32, flags: u32) -> u32 {
    h.call(m, dtor, this, &[]);
    if flags & 1 != 0 {
        free(m, h, this);
    }
    this
}

/// `0x54EB10`.
pub fn anim_base_ctor(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this, 0x0067_3D9C);
    m.w32(this + 4, 0);
    m.w8(this + 8, 0);
    m.w8(this + 9, 0);
    this
}

/// `0x54EB40`.
pub fn anim_base_reset(m: &mut dyn Mem, this: u32) {
    m.w32(this + 4, 0);
    m.w8(this + 8, 0);
    m.w8(this + 9, 0);
}

/// `0x54EB50`.
pub fn anim_base_deleting_dtor(m: &mut dyn Mem, h: &mut dyn Host, this: u32, flags: u32) -> u32 {
    m.w32(this, 0x0067_3D9C);
    if flags & 1 != 0 {
        free(m, h, this);
    }
    this
}

/// `0x54EB70` (`grouped` = `0x54EDC0`): estado de animación con la vtable de la clase y reiniciado.
pub fn animator_ctor(m: &mut dyn Mem, h: &mut dyn Host, this: u32, grouped: bool) -> u32 {
    h.call(m, ANIM_STATE_CTOR, this, &[]);
    m.w32(this, if grouped { 0x0067_3DB8 } else { 0x0067_3DA4 });
    h.call(m, ANIM_STATE_RESET, this, &[]);
    if grouped {
        m.w8(this + 0x10, 0);
    }
    this
}

/// `0x54EE20`.
pub fn grouped_animator_reset(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    h.call(m, ANIM_STATE_RESET, this, &[]);
    m.w8(this + 0x10, 0);
}

/// `0x54F110`: libera paletas, texturas SGL y nombres; después limpia las llamas (`0x551C00`).
pub fn texture_cache_free_all(m: &mut dyn Mem, h: &mut dyn Host) -> u32 {
    if m.r32(TEXTURES) != 0 {
        let mut i = 0;
        while i < m.r32(TEXTURE_COUNT) {
            let e = m.r32(TEXTURES) + i * 0x10;
            let pal = m.r32(e + 8);
            if pal != 0 {
                free(m, h, pal);
            }
            let e = m.r32(TEXTURES) + i * 0x10;
            let tex = m.r32(e);
            cdecl(m, h, SGL_FREE_MEMORY_TEXTURE, &[tex]);
            let e = m.r32(TEXTURES) + i * 0x10;
            let name = m.r32(e + 0xC);
            free(m, h, name);
            i += 1;
        }
        let arr = m.r32(TEXTURES);
        cdecl(m, h, CRT_FREE, &[arr]);
        m.w32(TEXTURES, 0);
        m.w32(TEXTURE_COUNT, 0);
    }
    cdecl(m, h, CLEAR_FLAMES, &[])
}

/// `0x54F250`: nombre de la textura `id` (0 si no está).
pub fn texture_cache_get_name(m: &mut dyn Mem, id: u32) -> u32 {
    let n = m.r32(TEXTURE_COUNT);
    let arr = m.r32(TEXTURES);
    for i in 0..n {
        if m.r32(arr + i * 0x10) == id {
            return m.r32(arr + i * 0x10 + 0xC);
        }
    }
    0
}

/// `0x54F290` (cdecl): busca la textura por nombre (`_stricmp`); si está, suma `flags` a sus banderas; si no, la carga.
pub fn texture_cache_get_or_load(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32]) -> u8 {
    let (path, out, key, flags, mode) = (a[0], a[1], a[2], a[3], a[4]);
    let mut found = None;
    let mut i = 0;
    while i < m.r32(TEXTURE_COUNT) {
        let arr = m.r32(TEXTURES);
        let name = m.r32(arr + i * 0x10 + 0xC);
        if cdecl(m, h, CRT_STRICMP, &[name, path]) == 0 {
            let arr = m.r32(TEXTURES);
            found = Some((i, m.r32(arr + i * 0x10)));
            break;
        }
        i += 1;
    }
    match found {
        Some((i, tex)) if tex != 0 => {
            m.w32(out, tex);
            let e = m.r32(TEXTURES) + (i << 4) + 4;
            let v = m.r32(e) | flags;
            m.w32(e, v);
            1
        }
        _ => {
            m.w32(out, 0);
            let tex = cdecl(m, h, LOAD_TEXTURE, &[path, key, flags, mode]);
            m.w32(out, tex);
            u8::from(tex != 0)
        }
    }
}

/// `0x54F330` (cdecl): carga la textura con SGL y la añade a la caché. Con banderas, guarda los 4 primeros bytes de la paleta que da
/// `sgl_LockTexture` (`+4` de la estructura de bloqueo, en `esp-0x14`; el original copia solo 4 de los 0x400 reservados).
pub fn texture_cache_load_texture(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) -> u32 {
    let (path, key, flags, mut mode) = (a[0], a[1], a[2], a[3]);
    if mode == u32::MAX {
        mode = u32::from(key != 0) * 4 + 3;
    }
    cdecl(m, h, SGL_SET_COLOR_KEY, &[key]);
    let archive = m.r32(ARCHIVE_GLOBAL);
    let old = cdecl(m, h, SGL_SET_ARCHIVE, &[archive]);
    let tex = cdecl(m, h, SGL_LOAD_MEMORY_TEXTURE, &[path, mode]);
    cdecl(m, h, SGL_SET_ARCHIVE, &[old]);
    if tex == 0 {
        return 0;
    }
    let count = m.r32(TEXTURE_COUNT).wrapping_add(1);
    m.w32(TEXTURE_COUNT, count);
    let arr = m.r32(TEXTURES);
    let arr = cdecl(m, h, CRT_REALLOC, &[arr, count << 4]);
    m.w32(TEXTURES, arr);
    let len = cdecl(m, h, CRT_STRLEN, &[path]);
    let name = new(m, h, len.wrapping_add(1));
    let e = |m: &mut dyn Mem| m.r32(TEXTURES).wrapping_add(m.r32(TEXTURE_COUNT) << 4).wrapping_sub(0x10);
    let ent = e(m);
    m.w32(ent + 0xC, name);
    let ent = e(m);
    let dst = m.r32(ent + 0xC);
    cdecl(m, h, CRT_STRCPY, &[dst, path]);
    let ent = e(m);
    let nm = m.r32(ent + 0xC);
    m.w8(nm.wrapping_add(len), 0);
    let ent = e(m);
    m.w32(ent + 4, flags);
    let ent = e(m);
    m.w32(ent + 8, 0);
    let ent = e(m);
    m.w32(ent, tex);
    if flags != 0 {
        let lock = esp.wrapping_sub(0x14);
        let ent = e(m);
        let t = m.r32(ent);
        cdecl(m, h, SGL_LOCK_TEXTURE, &[t, lock, 1]);
        let pal_src = m.r32(lock + 4);
        if pal_src != 0 {
            let pal = new(m, h, 0x400);
            let ent = e(m);
            m.w32(ent + 8, pal);
            let ent = e(m);
            let pal = m.r32(ent + 8);
            memcpy(m, pal, pal_src, 4);
        }
        let ent = e(m);
        let t = m.r32(ent);
        cdecl(m, h, SGL_UNLOCK_TEXTURE, &[t]);
    }
    let ent = e(m);
    m.r32(ent)
}

/// `0x54F4F0`: aclara las paletas guardadas en `amount` (×0,7 en las entradas `0x380..0x398`, ×1,8 en `0x3C0..0x3DC`) sin pasar del
/// máximo del color más 20, y las aplica con `sgl_ChangePalette` desde una copia en la pila (`esp - 0x400`).
pub fn texture_cache_adjust_palettes(m: &mut dyn Mem, h: &mut dyn Host, amount: u32, esp: u32) {
    let s = esp.wrapping_sub(0x440);
    let lock = s + 0x2C;
    let buf = s + 0x40;
    let f = amount as u64 as f32;
    let low = (f64::from(f) * f64::from(0.7f32)).trunc() as i64 as u32;
    let high = (f64::from(f) * f64::from(1.8f32)).trunc() as i64 as u32;
    let mut i = 0;
    while i < m.r32(TEXTURE_COUNT) {
        let e = m.r32(TEXTURES) + i * 0x10;
        if m.r32(e + 4) != 0 {
            let tex = m.r32(e);
            cdecl(m, h, SGL_LOCK_TEXTURE, &[tex, lock, 1]);
            let e = m.r32(TEXTURES) + i * 0x10;
            let tex = m.r32(e);
            cdecl(m, h, SGL_UNLOCK_TEXTURE, &[tex]);
            if m.r32(lock + 4) != 0 {
                let e = m.r32(TEXTURES) + i * 0x10;
                let pal = m.r32(e + 8);
                memcpy(m, buf, pal, 0x400);
                let mut off = 0x380u32;
                while off < 0x400 {
                    let delta = if off < 0x398 {
                        low
                    } else if off > 0x3BC && off < 0x3DC {
                        high
                    } else {
                        amount
                    };
                    let t = m.r32(TEXTURES);
                        let pal = m.r32(t + i * 0x10 + 8);
                    let b2 = u32::from(m.r8(pal + off + 2));
                    let b1 = u32::from(m.r8(pal + off + 1));
                    let b0 = u32::from(m.r8(pal + off));
                    let mut lim = b1.max(b2).max(b0);
                    if lim < 0xEB {
                        lim = (lim + 0x14).min(0xFF);
                    }
                    for k in [2u32, 1, 0] {
                        let t = m.r32(TEXTURES);
                        let pal = m.r32(t + i * 0x10 + 8);
                        let b = u32::from(m.r8(pal + off + k));
                        if lim != b {
                            let v = if b.wrapping_add(delta) > lim { lim as u8 } else { (b as u8).wrapping_add(delta as u8) };
                            m.w8(buf + off + k, v);
                        }
                    }
                    off += 4;
                }
                let e = m.r32(TEXTURES) + i * 0x10;
                let tex = m.r32(e);
                cdecl(m, h, SGL_CHANGE_PALETTE, &[tex, buf]);
            }
        }
        i += 1;
    }
}

/// Tabla de portes de este módulo.
pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    let r = match f {
        0x54E2D0 => Ret::U32(ctor(m, h, ecx)),
        0x54E2F0 => Ret::U32(dtor(m, h, ecx)),
        0x54E310 => Ret::U8(parse_chunk(m, h, a[0], a[1])),
        0x54E370 => Ret::U8(load_sheet_from_file(m, h, ecx, a[0], esp)),
        0x54E4C0 => Ret::U8(get_variant_in_kind(m, h, ecx)),
        0x54E520 => {
            set_sheet_index(m, ecx, a[0], a[1], a[2]);
            Ret::None
        }
        0x54E5A0 => Ret::U8(get_format_flag(m, ecx)),
        0x54E5E0 => {
            draw_with_fallback(m, h, ecx, a, esp);
            Ret::None
        }
        0x54E850 => Ret::U32(release_sheet_data(m, h, ecx)),
        0x54E860 => Ret::U32(forward_sheet(m, h, ecx, SHEET_FOOTPRINT, a)),
        0x54E880 => Ret::U32(forward_sheet(m, h, ecx, SHEET_OVERLAY, a)),
        0x54E8A0 => Ret::U32(forward_sheet(m, h, ecx, SHEET_CLIP_OVERLAY, a)),
        0x54E8C0 => Ret::U32(forward_sheet(m, h, ecx, SHEET_CLIP_FRAME, a)),
        0x54E8E0 => Ret::U32(sheet_of(m, h, ecx)),
        0x54E8F0 | 0x54E900 | 0x54E910 | 0x54E920 | 0x54E930 | 0x54E940 | 0x54E950 | 0x54E960 | 0x54E970 => sheet_field(m, h, ecx, f),
        0x54E980 => Ret::U32(select_texture_level(m, h, ecx, a[0])),
        0x54E9A0 => Ret::U32(get_image_b_rect(m, h, ecx, a[0])),
        0x54E9C0 => Ret::U32(get_redraw_level(m, ecx)),
        0x54E9E0 => Ret::U32(get_scaled_height(m, h, ecx)),
        0x54EA00 => Ret::U32(clip_and_register_rect(m, h, ecx, a[0], a[1])),
        0x54EA20 => Ret::U8(is_ui_flag_4f0_set(m)),
        0x54EA80 => Ret::U32(cdecl(m, h, RESOURCE_CREATE_ANIMATORS, &[a[0], ecx + 0x14])),
        0x54EAA0 => Ret::U32(cdecl(m, h, RESOURCE_DESTROY_ANIMATORS, &[a[0], ecx + 0x14])),
        0x54EAC0 => Ret::U32(get_sheet(m, ecx)),
        0x54EAE0 => Ret::U32(deleting_dtor(m, h, ecx, SPRITE_DTOR, a[0])),
        0x54EB00 => Ret::U32(h.call(m, SPRITE_SDD, ecx.wrapping_sub(4), &[a[0]])),
        0x54EB10 => Ret::U32(anim_base_ctor(m, ecx)),
        0x54EB30 => {
            m.w32(ecx, 0x0067_3D9C);
            Ret::None
        }
        0x54EB40 => {
            anim_base_reset(m, ecx);
            Ret::U32(0)
        }
        0x54EB50 => Ret::U32(anim_base_deleting_dtor(m, h, ecx, a[0])),
        0x54EB70 => Ret::U32(animator_ctor(m, h, ecx, false)),
        0x54EBC0 => {
            m.w32(ecx, 0x0067_3DA4);
            Ret::U32(h.call(m, ANIM_STATE_DTOR, ecx, &[]))
        }
        0x54EDA0 => Ret::U32(deleting_dtor(m, h, ecx, SPRITE_ANIMATOR_DTOR, a[0])),
        0x54EDC0 => Ret::U32(animator_ctor(m, h, ecx, true)),
        0x54EE10 => {
            m.w32(ecx, 0x0067_3DB8);
            Ret::U32(h.call(m, ANIM_STATE_DTOR, ecx, &[]))
        }
        0x54EE20 => {
            grouped_animator_reset(m, h, ecx);
            Ret::None
        }
        0x54F0C0 => Ret::U32(deleting_dtor(m, h, ecx, GROUPED_ANIMATOR_DTOR, a[0])),
        0x54F0E0 => {
            h.call(m, READER_CTOR, ecx, &[]);
            m.w32(ecx, 0x0067_3DD0);
            Ret::U32(ecx)
        }
        0x54F100 => {
            m.w32(ecx, 0x0067_3DD0);
            Ret::U32(h.call(m, READER_CTOR_ALT, ecx, &[]))
        }
        0x54F110 => Ret::U32(texture_cache_free_all(m, h)),
        0x54F250 => Ret::U32(texture_cache_get_name(m, a[0])),
        0x54F290 => Ret::U8(texture_cache_get_or_load(m, h, a)),
        0x54F330 => Ret::U32(texture_cache_load_texture(m, h, a, esp)),
        0x54F4F0 => {
            texture_cache_adjust_palettes(m, h, a[0], esp);
            Ret::None
        }
        _ => return None,
    };
    Some(r)
}
