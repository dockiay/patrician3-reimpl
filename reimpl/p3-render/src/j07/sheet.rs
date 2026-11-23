//! Hojas de sprites, objetos del mapa, incendio de edificios y sprites de ciudad (`0x54F710`–`0x553490`).
//!
//! `SpriteSheet` (`0x3C` bytes, vtable `0x673E94`): `+4` máscara alfa, `+0xC` textura, `+0x10` rectángulos de fotograma (8 bytes: x, y,
//! ancho, alto), `+0x14` animación de capa, `+0x18` animación secundaria, `+0x1C` variantes de textura, `+0x20` entradas, `+0x22..+0x28`
//! huella (x, y, ancho, alto), `+0x2A`/`+0x2B` fotogramas base/extra, `+0x2C`/`+0x2D` banderas y `+0x30` lista de grupos de animación
//! (vtable `0x673E98`, `+4` número, `+8` tabla). `SpriteSheetMapObject` añade en `+0x3C` la interfaz de recurso (vtable `0x673EC8`): sus
//! métodos reciben `this = hoja + 0x3C`.
//!
//! `BuildingBurnState` (`+4` progreso, `+6` total, `+8` instante de la próxima llama, `+0xC..+0xF` fase de cada foco) vive en `+4` de
//! los sprites de ciudad (`TownSprite`, vtables `0x673F50`/`0x673FD8`).

use super::sprite::{deleting_dtor, CRT_FREE, CRT_REALLOC, SHEET_FLAG_ALT, UI};
use super::{at16, at32, at8, cdecl, free, memset, new, rf32, vcall, Host, Mem, Ret};

pub const GET_MERCHANT: u32 = 0x0053_03C0;
pub const WORLD: u32 = 0x006D_E4A0;
pub const SGL_DRAW_BITMAP_RECT: u32 = 0x004B_B330;
pub const SGL_DRAW_BITMAP_RECT_ALPHA: u32 = 0x004B_B340;
pub const SGL_SET_CONSTANT_COLOR: u32 = 0x004B_B870;
pub const SGL_GET_CONSTANT_COLOR: u32 = 0x004B_B880;
pub const SGL_SET_RENDER_SOURCE: u32 = 0x004B_B9C0;
pub const SGL_GET_COLOR_KEY: u32 = 0x004B_B860;
pub const CRT_RAND: u32 = 0x0063_9C03;
pub const SPAWN_TOWN_ENTITY: u32 = 0x0059_0610;
pub const QUEUE_BUILDING_SOUND: u32 = 0x0059_4C00;
pub const CLIP_INTERSECTION: u32 = 0x0059_B5B0;
pub const REFRESH_FLAME_AREAS: u32 = 0x0055_06D0;
pub const ANIM_DRAW_MASKED_CLIPPED: u32 = 0x0058_5800;
pub const ANIM_INVALIDATE_FRAME: u32 = 0x0058_5770;
pub const ANIM_STATES_UPDATE_DRAW: u32 = 0x0058_5900;
pub const ANIM_STATES_DRAW: u32 = 0x0058_5970;
pub const ANIM_SET_RESET: u32 = 0x0058_53E0;
pub const ANIM_SET_DTOR: u32 = 0x0058_5250;
pub const ANIM_SET_SKIP: u32 = 0x0058_5490;
pub const ANIM_SET_LOAD: u32 = 0x0058_52A0;
pub const ANIMATION_CTOR: u32 = 0x0058_5220;
pub const ARC_CREATE_MEM_MAPPED: u32 = 0x0063_89EE;
pub const ARC_RELEASE_MEM_MAPPED: u32 = 0x0063_89F4;
pub const ARCHIVE_GLOBAL: u32 = 0x006D_CCC8;
pub const ARCHIVE_CTOR: u32 = 0x0040_2740;
pub const ARCHIVE_DTOR: u32 = 0x0040_2750;
pub const ARCHIVE_OPEN: u32 = 0x0040_2760;
pub const ARCHIVE_CLOSE: u32 = 0x0040_2780;
pub const ARCHIVE_READ: u32 = 0x0040_27A0;
pub const ARCHIVE_SEEK: u32 = 0x0040_27D0;
/// `CString` de MFC: cadena vacía, asignación, `ReverseFind`, `Insert` y destructor.
pub const AFX_EMPTY_STRING: u32 = 0x006C_7CCC;
pub const CSTRING_ASSIGN: u32 = 0x0064_F390;
pub const CSTRING_REVERSE_FIND: u32 = 0x0064_D360;
pub const CSTRING_INSERT: u32 = 0x0064_CD70;
pub const CSTRING_DTOR: u32 = 0x0064_F253;
pub const TEXTURE_GET_OR_LOAD: u32 = 0x0054_F290;
pub const READER_LOAD_SHEET: u32 = 0x0055_1000;
pub const READ_EXTRA_CHUNKS: u32 = 0x0055_13F0;
pub const READ_ARRAY_AT_24: u32 = 0x0055_1330;
pub const GROUP_LIST_RESIZE: u32 = 0x0055_1610;
pub const GROUP_APPEND: u32 = 0x0055_16C0;
pub const SHEET_CTOR: u32 = 0x0055_1A80;
pub const SHEET_DTOR: u32 = 0x0055_1AE0;
pub const SHEET_RESET: u32 = 0x0055_1B40;
pub const SHEET_DRAW_FRAME: u32 = 0x0055_1C80;
pub const SHEET_FOOTPRINT: u32 = 0x0055_1C20;
pub const SHEET_DRAW_LAYERS: u32 = 0x0055_1CF0;
pub const SHEET_CLIP_FRAME: u32 = 0x0055_1E60;
pub const SHEET_CLIP_OVERLAY: u32 = 0x0055_1ED0;
pub const SHEET_OVERLAY: u32 = 0x0055_1F70;
pub const SHEET_DRAW_MASKED: u32 = 0x0055_2010;
pub const SPRITE_BASE_CTOR: u32 = 0x0054_F0E0;
pub const SPRITE_BASE_DTOR: u32 = 0x0054_F100;
pub const ANIM_BASE_CTOR: u32 = 0x0054_EB10;
pub const ANIM_BASE_DTOR: u32 = 0x0054_EB30;
pub const ANIM_BASE_RESET: u32 = 0x0054_EB40;
pub const ANIM_STATE_CTOR: u32 = 0x0055_17B0;
pub const ANIM_STATE_DTOR: u32 = 0x0055_1800;
pub const ANIM_STATE_RESET: u32 = 0x0055_1810;
pub const BURN_CTOR: u32 = 0x0055_0430;
pub const BURN_CTOR_ALT: u32 = 0x0055_0460;
pub const BURN_UPDATE: u32 = 0x0055_0550;
pub const BURN_DRAW_FRAME: u32 = 0x0055_0770;
pub const BURN_ANIMATE: u32 = 0x0055_0A70;
pub const BURN_DRAW_MASKED: u32 = 0x0055_0CE0;
pub const SHEET_MAP_OBJECT_CTOR: u32 = 0x0055_2820;
pub const SHEET_MAP_OBJECT_DTOR: u32 = 0x0055_2890;
pub const TOWN_SPRITE_CTOR: u32 = 0x0055_2B50;
pub const TOWN_SPRITE_DTOR: u32 = 0x0055_2BB0;
pub const SPRITE_RESOURCE_SDD: u32 = 0x0055_2B20;
pub const TOWN_SPRITE_SDD: u32 = 0x0055_30E0;
pub const TOWN_VARIANT_DTOR: u32 = 0x0055_3130;
pub const RANDOM_STATE_DTOR: u32 = 0x0055_21F0;
pub const GROUPED_STATE_DTOR: u32 = 0x0055_24D0;
pub const ANIMATOR_FACTORY: u32 = 0x0055_2EF0;
/// Conjuntos de animación de las llamas (`0x6E11E8`, `0x6E1200`; `0x18` bytes cada uno).
pub const FLAME_SETS: u32 = 0x006E_11E8;
/// Reloj de animación (`0x6DCCF8`) y opciones (`0x6CBDF0`, `+0x16C` nivel de detalle).
pub const ANIM_CLOCK: u32 = 0x006D_CCF8;
pub const OPTIONS: u32 = 0x006C_BDF0;
/// Color constante de selección (`0x677940`).
pub const HIGHLIGHT_COLOR: u32 = 0x0067_7940;

/// Destruye la tabla de objetos `[arr]` (`count` entradas) con `vtable+slot(flag)` y la libera con `free` del CRT.
fn destroy_table(m: &mut dyn Mem, h: &mut dyn Host, arr: u32, count: u32, slot: u32, flag: u32) {
    if m.r32(arr) != 0 {
        let mut i = 0;
        while i < m.r32(count) {
            let t = m.r32(arr);
            let o = m.r32(t + i * 4);
            if o != 0 {
                vcall(m, h, o, slot, &[flag]);
            }
            i += 1;
        }
        let t = m.r32(arr);
        cdecl(m, h, CRT_FREE, &[t]);
        m.w32(arr, 0);
    }
    m.w32(count, 0);
}

/// `0x54F710`: 1 si el comerciante dueño de la casilla bajo el cursor tiene el bit `bit` de su tabla de edificios en la ciudad actual
/// y, cuando la ciudad tiene la bandera `0x10` en `+0x2C8`, la casilla del mapa de ocupación es mayor que `0x7F`.
pub fn merchant_has_building_type_flag(m: &mut dyn Mem, h: &mut dyn Host, bit: u32) -> u8 {
    let ui = m.r32(UI);
    let cell = (m.r32(ui + 0x55C).wrapping_sub(m.r32(ui + 0x560)) as i32 >> 2) as u32;
    let owners = m.r32(ui + 0xC310);
    let owner = m.r8(owners.wrapping_add(cell.wrapping_mul(4)).wrapping_add(3));
    if u16::from(owner) >= m.r16(WORLD + 0xA) {
        return 0;
    }
    let merchant = h.call(m, GET_MERCHANT, WORLD, &[u32::from(owner)]);
    let mask = 1u32 << (bit & 31);
    let ui = m.r32(UI);
    let town_idx = m.r32(ui + 0xC324) & 0x7F;
    let has = m.r32(merchant.wrapping_add(town_idx * 4 + 0x40)) & mask == mask;
    let town = m.r32(0x006D_E508).wrapping_add(town_idx * 0x9F8);
    if !has || m.r8(town + 0x2C8) & 0x10 == 0 {
        return u8::from(has);
    }
    let cell = (m.r32(ui + 0x55C).wrapping_sub(m.r32(ui + 0x560)) as i32 >> 2) as u32;
    let w = m.r32(ui + 0xC314);
    let row = cell / w;
    let col = cell - w * row;
    let p = col.wrapping_mul(m.r32(town + 0x7AC)).wrapping_add(m.r32(town + 0x80C));
    u8::from(m.r8(p.wrapping_add(row)) > 0x7F)
}

/// Constructor de cada tipo de `map_object_factory` (`0x54F7E0`).
#[derive(Clone, Copy)]
enum Make {
    /// `new(size)` + constructor y llamada `vtable+0x88(code)` si el argumento 3 es 0.
    Init(u32, u32),
    /// `new(size)` + constructor.
    Plain(u32, u32),
    /// Como `Plain` y después `+0x18 = byte`.
    Byte(u32, u32, ByteFrom),
    /// Solo con el argumento 3 no nulo: `new(size)` + constructor; devuelve el objeto `+0x3C` sin más llamadas.
    Resource(u32, u32),
    Sprite,
    TwoVariant,
    Type29,
    Type2F,
    Type2E,
}

#[derive(Clone, Copy)]
enum ByteFrom {
    UiVariant,
    UiVariantTimes4PlusCode,
    Code,
    CodeNotAlt,
}

fn factory_make(kind: u32) -> Make {
    use ByteFrom::*;
    use Make::*;
    match kind {
        0x2A | 0x50 | 0x51 => Sprite,
        0x5F => TwoVariant,
        0x56 => Init(0x1C, 0x0055_49F0),
        0x53 => Init(0x38, 0x0055_35F0),
        0x54 => Init(0x38, 0x0055_4190),
        0x59 => Init(0x1C, 0x0055_6080),
        0x55 => Init(0x1C, 0x0055_48B0),
        0x5C => Init(0x1C, 0x0055_70E0),
        0x29 => Type29,
        0x65 => Plain(0x18, 0x0058_13F0),
        0x5D => Plain(0x18, 0x0055_6160),
        0x5B => Plain(0x18, 0x0055_71C0),
        0x5A => Plain(0x18, 0x0055_6FA0),
        0x58 => Plain(0x18, 0x0055_4D40),
        0x63 => Plain(0x18, 0x0055_6EC0),
        0x64 => Plain(0x34, 0x0055_6380),
        0x62 => Plain(0x18, 0x0055_72A0),
        0x60 => Plain(0x48, 0x0055_4E60),
        0x5E => Plain(0x18, 0x0055_6DE0),
        0x57 => Plain(0x34, 0x0055_57E0),
        0x61 => Plain(0x18, 0x0055_6240),
        0x2F => Type2F,
        0x2E => Type2E,
        0x04 => Byte(0x1C, 0x0058_4B10, UiVariantTimes4PlusCode),
        0x0B => Plain(0x18, 0x0058_3660),
        0x01 => Plain(0x18, 0x0058_4EC0),
        0x1B => Plain(0x1C, 0x0058_4210),
        0x02 => Plain(0x18, 0x0058_3D20),
        0x2C => Byte(0x1C, 0x0058_4920, Code),
        0x15 => Plain(0x18, 0x0058_3A20),
        0x09 => Plain(0x18, 0x0058_3720),
        0x00 => Plain(0x18, 0x0058_4CE0),
        0x2D => Plain(0x18, 0x0058_38A0),
        0x07 => Plain(0x18, 0x0058_3960),
        0x05 => Plain(0x18, 0x0058_37E0),
        0x08 => Plain(0x18, 0x0058_4E00),
        0x33 => Plain(0x18, 0x0058_4840),
        0x67 => Plain(0x18, 0x0058_2220),
        0x68 => Plain(0x18, 0x0058_2390),
        0x66 => Byte(0x1C, 0x0058_1D50, CodeNotAlt),
        0x03 => Byte(0x1C, 0x0058_4040, UiVariant),
        0x69 => Byte(0x1C, 0x0058_14D0, UiVariant),
        0x1A => Resource(0x40, 0x0058_1650),
        0x06 | 0x0A | 0x37 => Resource(0x44, 0x0058_1300),
        0x12 => Resource(0x44, 0x0058_22A0),
        0x14 => Resource(0x6C, 0x0058_2F20),
        0x17 => Resource(0x44, 0x0058_2E50),
        _ => Resource(0x40, SHEET_MAP_OBJECT_CTOR),
    }
}

fn construct(m: &mut dyn Mem, h: &mut dyn Host, size: u32, ctor: u32) -> u32 {
    let p = new(m, h, size);
    if p != 0 {
        h.call(m, ctor, p, &[])
    } else {
        0
    }
}

/// `0x54F7E0` (cdecl: tipo, código, bandera): crea el objeto del mapa del tipo dado. Al final, si el objeto tiene hoja
/// (`vtable+0x74`), la pasa a `vtable+0x78`. `ecx` es el ECX de entrada (el original lo empuja en el byte bajo de un argumento cuando
/// `new` falla en el tipo `0x29`).
pub fn map_object_factory(m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, kind: u32, code: u32, flag: u32) -> u32 {
    let ui_variant = |m: &mut dyn Mem| {
        let ui = m.r32(UI);
        m.r32(ui + 0x570)
    };
    let obj = match factory_make(kind) {
        Make::Sprite => {
            let o = construct(m, h, 0x1C, 0x0054_E2D0);
            h.call(m, 0x0054_E520, o, &[kind, code, code >> 16]);
            o
        }
        Make::TwoVariant => {
            let o = construct(m, h, 0x1C, 0x0055_4AF0);
            h.call(m, 0x0055_4C70, o, &[code]);
            o
        }
        Make::Init(size, ctor) => {
            let o = construct(m, h, size, ctor);
            if flag as u8 == 0 {
                vcall(m, h, o, 0x88, &[code]);
            }
            o
        }
        Make::Type29 => {
            let p = new(m, h, 0x38);
            // ECX al empujar el byte de variante: el objeto tras el constructor, o el ECX de entrada si `new` falló.
            let (o, reg) = if p != 0 { (h.call(m, 0x0058_0990, p, &[]), p) } else { (0, ecx) };
            let b = ui_variant(m) & 0xFF;
            h.call(m, 0x0058_0F80, o, &[(reg & !0xFF) | b, code]);
            if o == 0 {
                return 0;
            }
            o + 4
        }
        Make::Type2F => {
            let o = construct(m, h, 0x1C, 0x0058_2470);
            let v = ui_variant(m);
            h.call(m, 0x0058_25F0, o, &[code, code >> 16, v, (code >> 8) & 0xFF]);
            o
        }
        Make::Type2E => {
            let o = construct(m, h, 0x1C, 0x0058_1FB0);
            let v = ui_variant(m);
            h.call(m, 0x0058_2130, o, &[code, v]);
            o
        }
        Make::Plain(size, ctor) => construct(m, h, size, ctor),
        Make::Byte(size, ctor, from) => {
            let o = construct(m, h, size, ctor);
            let b = match from {
                ByteFrom::UiVariant => ui_variant(m) as u8,
                ByteFrom::UiVariantTimes4PlusCode => (ui_variant(m) as u8).wrapping_shl(2).wrapping_add(code as u8),
                ByteFrom::Code => code as u8,
                ByteFrom::CodeNotAlt => u8::from(code as u8 != m.r8(SHEET_FLAG_ALT)),
            };
            m.w8(o.wrapping_add(0x18), b);
            o
        }
        Make::Resource(size, ctor) => {
            if flag as u8 == 0 {
                return 0;
            }
            let o = construct(m, h, size, ctor);
            return if o != 0 { o + 0x3C } else { 0 };
        }
    };
    if obj != 0 && vcall(m, h, obj, 0x74, &[]) != 0 {
        let vt = m.r32(obj);
        let f = m.r32(vt + 0x74);
        let s = h.call(m, f, obj, &[]);
        let f = m.r32(vt + 0x78);
        h.call(m, f, obj, &[s]);
    }
    obj
}

/// `0x5502C0` (cdecl): `obj->vtable[0x7C](obj->vtable[0x74]())`, destruye el objeto y pone el puntero a 0.
pub fn map_object_destroy_and_clear(m: &mut dyn Mem, h: &mut dyn Host, pp: u32) {
    let o = m.r32(pp);
    let vt = m.r32(o);
    let f = m.r32(vt + 0x74);
    let r = h.call(m, f, o, &[]);
    let o = m.r32(pp);
    let f = m.r32(vt + 0x7C);
    h.call(m, f, o, &[r]);
    let o = m.r32(pp);
    if o != 0 {
        vcall(m, h, o, 0, &[1]);
    }
    m.w32(pp, 0);
}

/// `0x550430`/`0x550460`/`0x550490`: estado de incendio a cero y fases `0xFE`. Devuelve el `memset` (fases) salvo el constructor.
pub fn burn_reset(m: &mut dyn Mem, this: u32, vtable: bool) -> u32 {
    if vtable {
        m.w32(this, 0x0067_3E5C);
    }
    m.w16(this + 6, 0);
    m.w16(this + 4, 0);
    m.w32(this + 8, 0);
    memset(m, this + 0xC, 0xFE, 4);
    this + 0xC
}

/// `0x5504C0` (stdcall: hoja, salida): copia el rectángulo y el desplazamiento de la animación secundaria (`+0x18`); devuelve su textura.
pub fn secondary_sprite_info(m: &mut dyn Mem, sheet: u32, out: u32) -> u32 {
    let a = m.r32(sheet + 0x18);
    if a == 0 {
        return 0;
    }
    if out != 0 {
        for k in 0..4 {
            let r = m.r32(a + 4);
            let v = m.r16(r + 2 * k);
            m.w16(out + 2 * k, v);
        }
        let v = m.r16(a + 0x10);
        m.w16(out + 8, v);
        let v = m.r16(a + 0x12);
        m.w16(out + 0xA, v);
    }
    m.r32(a + 0xC)
}

/// `0x550520`: altura de la hoja (`+0x28`) por el progreso del incendio (división entera con signo).
pub fn burnt_height(m: &mut dyn Mem, this: u32, sheet: u32) -> u32 {
    let hgt = i32::from(m.r16(sheet + 0x28));
    let p = i32::from(m.r16(this + 4));
    let t = i32::from(m.r16(this + 6));
    hgt.wrapping_mul(p).wrapping_div(t) as u32
}

/// Resto con signo de `idiv` (`a % b` truncado hacia cero).
fn irem(a: u32, b: u32) -> u32 {
    (a as i32).wrapping_rem(b as i32) as u32
}

/// `0x550A70` (thiscall, 4 argumentos: x, y, fotograma, hoja): avanza las fases de los cuatro focos y lanza llamas y humo cuando toca;
/// devuelve 0 si el edificio no arde.
pub fn burn_animate(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) -> u8 {
    let (x, y, frame, sheet) = (a[0], a[1], a[2], a[3]);
    let phase = m.r8(this + 0xE) as i8;
    if m.r16(this + 4) == m.r16(this + 6) && phase < -1 {
        return 0;
    }
    if phase < 0 {
        return 1;
    }
    if m.r32(ANIM_CLOCK) > m.r32(this + 8) {
        let anim = m.r32(sheet + 0x18);
        let mut p = m.r32(anim + 8);
        for k in 0..4u32 {
            if m.r16(p + 2) != 0 || m.r16(p) != 0 {
                let set = if k < 2 { FLAME_SETS } else { FLAME_SETS + 0x18 };
                let first = at32(m, set + 8, 4);
                let ph = m.r8(this + 0xC + k).wrapping_add(1);
                m.w8(this + 0xC + k, ph);
                let ph = ph as i8 as i32;
                if ph == first.wrapping_sub(1) as i32 {
                    let r = cdecl(m, h, CRT_RAND, &[]);
                    let span = u32::from(m.r16(set + 0x14)).wrapping_sub(first);
                    m.w8(this + 0xC + k, ((r % span) as u8).wrapping_add(first as u8));
                } else if ph >= i32::from(m.r16(set + 0x14)) {
                    m.w8(this + 0xC + k, first as u8);
                }
                let r = cdecl(m, h, CRT_RAND, &[]);
                let div = 13u32.wrapping_sub(irem(r, 4));
                let ph = m.r8(this + 0xC + k) as i8 as i32 as u32;
                if irem(ph, div) == 0 {
                    let frames = m.r32(set + 4);
                    let fr = frames.wrapping_add(first.wrapping_mul(8));
                    let wd = u32::from(m.r16(fr + 4));
                    let rects = m.r32(sheet + 0x10);
                    let rect = rects.wrapping_add((frame & 0xFF) * 8);
                    let anim = m.r32(sheet + 0x18);
                    let ui = m.r32(UI);
                    let dx = irem(r, wd >> 2)
                        .wrapping_sub(wd >> 3)
                        .wrapping_add(u32::from(m.r16(anim + 0x10)))
                        .wrapping_add(m.r32(ui + 0xC650))
                        .wrapping_sub(u32::from(m.r16(rect)));
                    let ex = u32::from(m.r16(p + 2)).wrapping_add(x.wrapping_add(dx));
                    let ht = u32::from(m.r16(fr + 6));
                    let dy = irem(r, ((ht as i32) / 10) as u32);
                    let ey0 = u32::from(m.r16(anim + 0x12))
                        .wrapping_sub(dy)
                        .wrapping_sub(u32::from(m.r16(rect + 6)))
                        .wrapping_sub(u32::from(m.r16(rect + 2)))
                        .wrapping_add(m.r32(ui + 0xC654));
                    let edx = y.wrapping_add(ey0);
                    let ey = u32::from(m.r16(p)).wrapping_add(edx);
                    let off = vcall(m, h, ui, 0x70, &[]);
                    let ey = ey.wrapping_sub(off);
                    let opts = m.r32(OPTIONS);
                    if m.r32(opts + 0x16C) as i32 >= 2 {
                        // El byte de la variante va en DL; los bits altos son los de EDX antes de la llamada virtual.
                        let alt = (edx & !0xFF) | u32::from(m.r8(SHEET_FLAG_ALT));
                        let kind = ((ht & 0xFFFC) << 14) | 0x24;
                        h.call(m, SPAWN_TOWN_ENTITY, ui, &[ex << 16, ey << 16, 0x12, kind, alt, 0, 0]);
                    }
                }
            }
            p += 4;
        }
        let step = at32(m, 0x006E_11F0, 0);
        let next = step.wrapping_add(m.r32(ANIM_CLOCK));
        m.w32(this + 8, next);
        h.call(m, REFRESH_FLAME_AREAS, this, &[x, y, frame, sheet]);
    }
    let ui = m.r32(UI);
    h.call(m, QUEUE_BUILDING_SOUND, ui, &[x, y, 0]);
    1
}

/// `0x550CE0` (thiscall, 4 argumentos: hoja, rectángulo, &x, &y): dibuja la parte quemada y la franja de brasas que se desvanece
/// línea a línea (alfa en coma flotante de 32 bits, como el original). `esp` es la pila de entrada (el original reutiliza el hueco del
/// primer argumento para la coordenada y).
pub fn burn_draw_frame_masked(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) -> u8 {
    let (sheet, rect, px, py) = (a[0], a[1], a[2], a[3]);
    let prog = m.r16(this + 4);
    let total = m.r16(this + 6);
    if prog == total && (m.r8(this + 0xE) as i8) < -1 {
        return 0;
    }
    let hgt = u32::from(m.r16(sheet + 0x28));
    let q = (u32::from(prog).wrapping_mul(hgt) as i32).wrapping_div(i32::from(total)) as u32;
    let anim = m.r32(sheet + 0x18);
    let mut cut = q
        .wrapping_sub(u32::from(m.r16(sheet + 0x24)))
        .wrapping_add(u32::from(m.r16(rect + 6)))
        .wrapping_sub(hgt)
        .wrapping_add(u32::from(m.r16(rect + 2)));
    let tex = m.r32(anim + 0xC);
    cdecl(m, h, SGL_SET_RENDER_SOURCE, &[tex]);
    let draw = |m: &mut dyn Mem, h: &mut dyn Host| {
        let mask = m.r32(sheet + 8);
        let yy = m.r32(py);
        let xx = m.r32(px);
        h.call(m, ANIM_DRAW_MASKED_CLIPPED, anim, &[xx, yy, rect, 0, mask]);
    };
    if (m.r8(this + 0xE) as i8) >= -1 {
        let color = cdecl(m, h, SGL_GET_CONSTANT_COLOR, &[]);
        cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[(color >> 24) << 24 | 0x28_2828]);
        draw(m, h);
        cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color]);
    } else {
        draw(m, h);
    }
    if m.r8(this + 0xE) == 0xFF || (cut.wrapping_add(0x16) as i32) <= 0 {
        return 1;
    }
    let rh = u32::from(m.r16(rect + 6));
    if cut as i32 > rh as i32 {
        cut = rh;
    }
    let mut row = u32::from(m.r16(rect + 2)).wrapping_sub(cut).wrapping_add(rh);
    let mut yy = m.r32(py).wrapping_sub(cut);
    let mask_tex = m.r32(sheet + 0xC);
    cdecl(m, h, SGL_SET_RENDER_SOURCE, &[mask_tex]);
    let src = m.r32(sheet + 4);
    if cut as i32 > 0 {
        let w = u32::from(m.r16(rect + 4));
        let x = m.r32(px);
        let rx = u32::from(m.r16(rect));
        cdecl(m, h, SGL_DRAW_BITMAP_RECT_ALPHA, &[src, rx, row, x, yy, w, cut]);
    }
    let color = cdecl(m, h, SGL_GET_CONSTANT_COLOR, &[]);
    let alpha = ((color >> 24) / 5 * 4) & 0xFF;
    let mut cur = alpha as i32 as f32;
    let step = cur / rf32(m, 0x0067_3E60);
    cur -= step;
    let one = rf32(m, 0x0066_BC80);
    let (b0, b1, b2) = (color & 0xFF, (color >> 8) & 0xFF, (color >> 16) & 0xFF);
    while cur >= one {
        row = row.wrapping_sub(1);
        let top = u32::from(m.r16(rect + 2));
        if (row as i32) < (top as i32) {
            break;
        }
        yy = yy.wrapping_sub(1);
        let bottom = u32::from(m.r16(rect + 6)).wrapping_add(top);
        if (row as i32) < (bottom as i32) {
            let ia = (cur as i32 as u32) & 0xFF;
            cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[ia << 24 | b2 << 16 | b1 << 8 | b0]);
            let w = u32::from(m.r16(rect + 4));
            let x = m.r32(px);
            let rx = u32::from(m.r16(rect));
            let src = m.r32(sheet + 4);
            cdecl(m, h, SGL_DRAW_BITMAP_RECT_ALPHA, &[src, rx, row, x, yy, w, 1]);
        }
        cur -= step;
    }
    cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color]);
    1
}

/// `0x551000` (thiscall: nombre, hoja): lee la cabecera de la hoja (`0x00 'R'`), sus rectángulos y texturas y los trozos extra. `esp` es
/// la pila de entrada (cursor en el hueco del primer argumento, `CString` del nombre de la máscara en `esp-0x20`).
pub fn reader_load_sheet(m: &mut dyn Mem, h: &mut dyn Host, this: u32, name: u32, sheet: u32, esp: u32) -> u8 {
    let cursor = esp + 4;
    let size = esp - 0x10;
    let archive = m.r32(ARCHIVE_GLOBAL);
    let handle = cdecl(m, h, ARC_CREATE_MEM_MAPPED, &[archive, name, cursor, size]);
    if handle == 0 {
        return 0;
    }
    let mut c = m.r32(cursor);
    let b0 = m.r8(c);
    let b1 = m.r8(c + 1);
    c += 2;
    m.w32(cursor, c);
    if b0 != 0 || b1 != 0x52 {
        let archive = m.r32(ARCHIVE_GLOBAL);
        cdecl(m, h, ARC_RELEASE_MEM_MAPPED, &[archive, handle]);
        return 0;
    }
    let v = m.r16(c);
    m.w16(sheet + 0x20, v);
    c += 2;
    let v = m.r8(c);
    m.w8(sheet + 0x2D, v);
    c += 4;
    let key = m.r32(c);
    c += 4;
    for off in [0x22, 0x24, 0x26, 0x28] {
        let v = m.r16(c);
        m.w16(sheet + off, v);
        c += 4;
    }
    for off in [0x2A, 0x2B] {
        let v = m.r8(c);
        m.w8(sheet + off, v);
        c += 4;
    }
    m.w32(cursor, c);
    if vcall(m, h, this, 4, &[cursor, sheet]) as u8 == 0 {
        let archive = m.r32(ARCHIVE_GLOBAL);
        cdecl(m, h, ARC_RELEASE_MEM_MAPPED, &[archive, handle]);
        return 0;
    }
    let n = u32::from(m.r8(sheet + 0x2B)) + u32::from(m.r8(sheet + 0x2A));
    let rects = new(m, h, n.wrapping_mul(8).wrapping_sub(8));
    m.w32(sheet + 0x10, rects);
    let mut i = 0u32;
    while (i as i32) < (u32::from(m.r8(sheet + 0x2B)) + u32::from(m.r8(sheet + 0x2A))) as i32 - 1 {
        for k in 0..4 {
            let c = m.r32(cursor);
            let v = m.r16(c);
            let r = m.r32(sheet + 0x10);
            m.w16(r + i * 8 + 2 * k, v);
            m.w32(cursor, c + 4);
        }
        i += 1;
    }
    let c = m.r32(cursor);
    let len = m.r32(c);
    let data = c + 4;
    m.w32(cursor, data.wrapping_add(len));
    let flag = u32::from(m.r8(sheet + 0x2C));
    let ok = cdecl(m, h, TEXTURE_GET_OR_LOAD, &[data, sheet + 0xC, key, flag, u32::MAX]) as u8;
    let s = esp - 0x20;
    let empty = m.r32(AFX_EMPTY_STRING);
    m.w32(s, empty);
    h.call(m, CSTRING_ASSIGN, s, &[data]);
    let at = h.call(m, CSTRING_REVERSE_FIND, s, &[0x2F]);
    h.call(m, CSTRING_INSERT, s, &[at.wrapping_add(1), 0x006C_3A8C]);
    let path = m.r32(s);
    cdecl(m, h, TEXTURE_GET_OR_LOAD, &[path, sheet + 4, key, 0, u32::MAX]);
    h.call(m, READ_EXTRA_CHUNKS, this, &[cursor, sheet]);
    let archive = m.r32(ARCHIVE_GLOBAL);
    cdecl(m, h, ARC_RELEASE_MEM_MAPPED, &[archive, handle]);
    let variants = m.r32(sheet + 0x1C);
    if variants != 0 {
        let t = m.r32(sheet + 0xC);
        m.w32(variants, t);
    }
    h.call(m, CSTRING_DTOR, s, &[]);
    ok
}

/// `0x5512A0` (stdcall: &cursor, hoja): trozo de banderas de cabecera (`+0x2C`).
pub fn read_header_flag_chunk(m: &mut dyn Mem, cursor: u32, sheet: u32) -> u8 {
    let c = m.r32(cursor);
    let n = m.r32(c);
    let c = c + 4;
    m.w32(cursor, c);
    if n != 0 {
        let v = m.r32(c);
        m.w8(sheet + 0x2C, u8::from(v != 0));
        let c = m.r32(cursor);
        m.w32(cursor, c.wrapping_add(n << 2));
    }
    1
}

/// `0x5512D0`/`0x551300` (cdecl: fichero, salida): lee `n` bytes en el desplazamiento `n`.
pub fn read_at(m: &mut dyn Mem, h: &mut dyn Host, file: u32, out: u32, n: u32) -> u8 {
    h.call(m, ARCHIVE_SEEK, file, &[n, 0]);
    h.call(m, ARCHIVE_READ, file, &[out, n]);
    1
}

/// `0x551330` (cdecl: fichero, &tabla, &número): número en `0x24` y tabla de `u32` a continuación.
pub fn read_counted_array(m: &mut dyn Mem, h: &mut dyn Host, file: u32, arr: u32, count: u32) -> u8 {
    h.call(m, ARCHIVE_SEEK, file, &[0x24, 0]);
    h.call(m, ARCHIVE_READ, file, &[count, 4]);
    let n = m.r32(count);
    if n != 0 {
        let p = new(m, h, n << 2);
        m.w32(arr, p);
        let n = m.r32(count);
        h.call(m, ARCHIVE_READ, file, &[p, n << 2]);
    }
    1
}

/// `0x551380` (stdcall: &cursor, hoja): trozo de animación de un grupo (`índice - 3` en la lista `+0x30`).
pub fn read_animation_chunk(m: &mut dyn Mem, h: &mut dyn Host, cursor: u32, sheet: u32) -> u8 {
    let list = sheet + 0x30;
    let c = m.r32(cursor);
    let idx = m.r32(c).wrapping_sub(3);
    m.w32(cursor, c + 4);
    let group = if idx < m.r32(list + 4) {
        at32(m, list + 8, idx * 4)
    } else {
        h.call(m, GROUP_LIST_RESIZE, list, &[idx.wrapping_add(1)]);
        if idx >= m.r32(list + 4) {
            0
        } else {
            at32(m, list + 8, idx * 4)
        }
    };
    let n = h.call(m, GROUP_APPEND, group, &[]).wrapping_sub(1);
    let anim = if n >= m.r32(group + 4) { 0 } else { at32(m, group + 8, n * 4) };
    let tex = m.r32(sheet + 0xC);
    h.call(m, ANIM_SET_LOAD, anim, &[cursor, tex, u32::MAX]);
    1
}

/// `0x5513F0` (thiscall: &cursor, hoja): trozos extra (0 capa, 1 secundaria con su máscara, 2 variante de textura; el resto, al lector
/// virtual `vtable+8` o se saltan). `esp` es la pila de entrada.
pub fn read_extra_chunks(m: &mut dyn Mem, h: &mut dyn Host, this: u32, cursor: u32, sheet: u32, esp: u32) -> u8 {
    let c = m.r32(cursor);
    let mut count = m.r32(c);
    m.w32(cursor, c + 4);
    if count as i32 <= 0 {
        return 1;
    }
    loop {
        let c = m.r32(cursor);
        let tag = m.r32(c);
        let c = c + 4;
        m.w32(cursor, c);
        match tag {
            0 => {
                let a = construct(m, h, 0x18, ANIMATION_CTOR);
                let tex = m.r32(sheet + 0xC);
                m.w32(sheet + 0x14, a);
                h.call(m, ANIM_SET_LOAD, a, &[cursor, tex, u32::MAX]);
            }
            1 => {
                let s = esp + 4;
                let empty = m.r32(AFX_EMPTY_STRING);
                m.w32(s, empty);
                let a = construct(m, h, 0x18, ANIMATION_CTOR);
                let tex = m.r32(sheet + 0xC);
                m.w32(sheet + 0x18, a);
                let name = h.call(m, ANIM_SET_LOAD, a, &[cursor, tex, u32::MAX]);
                h.call(m, CSTRING_ASSIGN, s, &[name]);
                let at = h.call(m, CSTRING_REVERSE_FIND, s, &[0x2F]);
                h.call(m, CSTRING_INSERT, s, &[at.wrapping_add(1), 0x006C_3A98]);
                let a = m.r32(sheet + 0x18);
                let path = m.r32(s);
                let t = m.r32(a + 0xC);
                let key = cdecl(m, h, SGL_GET_COLOR_KEY, &[t]);
                cdecl(m, h, TEXTURE_GET_OR_LOAD, &[path, sheet + 8, key, 0, u32::MAX]);
                h.call(m, CSTRING_DTOR, s, &[]);
            }
            2 => {
                if m.r32(sheet + 0x1C) == 0 {
                    let p = new(m, h, 0x14);
                    m.w32(sheet + 0x1C, p);
                    memset(m, p, 0, 0x14);
                }
                let local = esp - 0x24;
                h.call(m, ANIMATION_CTOR, local, &[]);
                let tex = m.r32(sheet + 0xC);
                h.call(m, ANIM_SET_LOAD, local, &[cursor, tex, u32::MAX]);
                let idx = at32(m, local + 8, 0);
                let v = m.r32(local + 0xC);
                let t = m.r32(sheet + 0x1C);
                m.w32(t.wrapping_add(idx.wrapping_mul(4)), v);
                h.call(m, ANIM_SET_DTOR, local, &[]);
            }
            _ => {
                m.w32(cursor, c - 4);
                if vcall(m, h, this, 8, &[cursor, sheet]) as u8 == 0 {
                    let c = m.r32(cursor);
                    m.w32(cursor, c.wrapping_sub(4));
                    cdecl(m, h, ANIM_SET_SKIP, &[cursor]);
                }
            }
        }
        count = count.wrapping_sub(1);
        if count == 0 {
            return 1;
        }
    }
}

/// `0x551610`: cambia el número de grupos de animación (destruye los sobrantes con `vtable[0](3)` y crea grupos vacíos).
pub fn group_list_resize(m: &mut dyn Mem, h: &mut dyn Host, this: u32, n: u32) {
    let count = m.r32(this + 4);
    if n == count {
        return;
    }
    if n < count {
        let mut i = n;
        while i < m.r32(this + 4) {
            let o = at32(m, this + 8, i * 4);
            if o != 0 {
                vcall(m, h, o, 0, &[3]);
            }
            i += 1;
        }
    }
    let arr = m.r32(this + 8);
    let arr = cdecl(m, h, CRT_REALLOC, &[arr, n.wrapping_mul(4)]);
    m.w32(this + 8, arr);
    let mut i = m.r32(this + 4);
    while i < n {
        let p = new(m, h, 0xC);
        if p != 0 {
            m.w32(p, 0x0067_3E78);
            m.w32(p + 8, 0);
            m.w32(p + 4, 0);
        }
        i += 1;
        let t = m.r32(this + 8);
        m.w32(t + (i - 1) * 4, p);
    }
    m.w32(this + 4, n);
}

/// `0x5516C0`: añade una animación vacía al grupo; devuelve el nuevo número.
pub fn group_append(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    let n = m.r32(this + 4).wrapping_add(1);
    m.w32(this + 4, n);
    let arr = m.r32(this + 8);
    let arr = cdecl(m, h, CRT_REALLOC, &[arr, n << 2]);
    m.w32(this + 8, arr);
    let a = construct(m, h, 0x18, ANIMATION_CTOR);
    let n = m.r32(this + 4);
    let t = m.r32(this + 8);
    m.w32(t.wrapping_add(n.wrapping_mul(4)).wrapping_sub(4), a);
    m.r32(this + 4)
}

/// `0x551820` (thiscall, 5 argumentos): invalida el rectángulo del fotograma actual y marca el estado como dibujado.
pub fn anim_state_draw_current(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) -> u8 {
    let fi = u32::from(m.r8(this + 0xC) & 0x3F);
    let frame = m.r32(a[0] + fi * 4);
    let b8 = u32::from(m.r8(this + 8));
    cdecl(m, h, ANIM_INVALIDATE_FRAME, &[a[1], a[2], a[3], frame, a[4], b8]);
    let v = m.r8(this + 0xC) | 0xC0;
    m.w8(this + 0xC, v);
    v
}

/// `0x551B40`.
pub fn sheet_reset(m: &mut dyn Mem, h: &mut dyn Host, s: u32) {
    m.w32(s + 8, 0);
    m.w32(s + 4, 0);
    m.w32(s + 0xC, 0);
    m.w16(s + 0x20, 0);
    m.w8(s + 0x2A, 0);
    m.w8(s + 0x2B, 0);
    let r = m.r32(s + 0x10);
    if r != 0 {
        free(m, h, r);
    }
    m.w32(s + 0x10, 0);
    m.w8(s + 0x2D, 0);
    for off in [0x22, 0x24, 0x26, 0x28] {
        m.w16(s + off, 0);
    }
    m.w8(s + 0x2C, 0);
    destroy_table(m, h, s + 0x38, s + 0x34, 0, 1);
    let v = m.r32(s + 0x1C);
    if v != 0 {
        free(m, h, v);
    }
    m.w32(s + 0x1C, 0);
    for off in [0x14, 0x18] {
        let o = m.r32(s + off);
        if o != 0 {
            vcall(m, h, o, 4, &[3]);
        }
        m.w32(s + off, 0);
    }
}

/// `0x551C20` (cdecl: x, y, color, hoja): rectángulo de la huella.
pub fn sheet_draw_footprint(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32]) -> u32 {
    let (x, y, color, s) = (a[0], a[1], a[2], a[3]);
    cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color]);
    let t = m.r32(s + 0xC);
    cdecl(m, h, SGL_SET_RENDER_SOURCE, &[t]);
    let hh = u32::from(m.r16(s + 0x28));
    let w = u32::from(m.r16(s + 0x26));
    let fw = u32::from(at16(m, s + 0x10, 4));
    let n = u32::from(m.r8(s + 0x2A)).wrapping_sub(1);
    let dx = x.wrapping_sub(n.wrapping_mul(fw));
    let (sx, sy) = (u32::from(m.r16(s + 0x22)), u32::from(m.r16(s + 0x24)));
    cdecl(m, h, SGL_DRAW_BITMAP_RECT, &[sx, sy, dx, y.wrapping_sub(hh), w, hh])
}

/// `0x551CF0` (cdecl: x, y, hoja, estados): recorta la huella con la vista e invalida el fotograma de cada grupo animado.
pub fn sheet_draw_all_layers(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) {
    let (x, y, s, states) = (a[0], a[1], a[2], a[3]);
    let ui = m.r32(UI);
    let step = m.r32(ui + 0x4D0);
    let n = u32::from(m.r8(s + 0x2A));
    let last = n.wrapping_sub(1);
    let mut left = x.wrapping_sub(step.wrapping_mul(last));
    if last >= n {
        left = left.wrapping_sub(m.r32(ui + 0x4D0));
    }
    let rect = esp - 0x10;
    m.w32(rect, left);
    let right = u32::from(m.r16(s + 0x26)).wrapping_add(left);
    m.w32(rect + 8, right);
    let e = m.r32(s + 0x10).wrapping_add(last.wrapping_mul(8));
    let top = u32::from(m.r16(s + 0x24))
        .wrapping_sub(u32::from(m.r16(e + 6)))
        .wrapping_sub(u32::from(m.r16(e + 2)))
        .wrapping_add(y);
    m.w32(rect + 4, top);
    let bottom = u32::from(m.r16(s + 0x28)).wrapping_add(top);
    m.w32(rect + 0xC, bottom);
    h.call(m, CLIP_INTERSECTION, ui, &[rect]);
    if states == 0 {
        return;
    }
    let mut i = 0;
    while i < m.r32(s + 0x34) {
        let st = m.r32(states + i * 4);
        if st != 0 {
            let group = if i < m.r32(s + 0x34) { at32(m, s + 0x38, i * 4) } else { 0 };
            let fi = u32::from(m.r8(st + 0xC) & 0x3F);
            let frame = if fi < m.r32(group + 4) { at32(m, group + 8, fi * 4) } else { 0 };
            let b8 = u32::from(m.r8(st + 8));
            // El original empuja ECX con solo el byte bajo cambiado (fotogramas base menos uno).
            let nb = (frame & !0xFF) | u32::from(m.r8(s + 0x2A).wrapping_sub(1));
            cdecl(m, h, ANIM_INVALIDATE_FRAME, &[x, y, nb, frame, s, b8]);
        }
        i += 1;
    }
}

/// `0x551DE0` (cdecl: salida, fotograma, x, y, hoja): rectángulo del fotograma en pantalla.
pub fn sheet_frame_screen_rect(m: &mut dyn Mem, a: &[u32]) -> u32 {
    let (out, frame, x, y, s) = (a[0], a[1], a[2], a[3], a[4]);
    let ui = m.r32(UI);
    let mut left = x.wrapping_sub(m.r32(ui + 0x4D0).wrapping_mul(frame));
    m.w32(out, left);
    if frame >= u32::from(m.r8(s + 0x2A)) {
        let ui = m.r32(UI);
        left = left.wrapping_sub(m.r32(ui + 0x4D0));
        m.w32(out, left);
    }
    let left = m.r32(out);
    let v = u32::from(m.r16(s + 0x26)).wrapping_add(left);
    m.w32(out + 8, v);
    let e = m.r32(s + 0x10).wrapping_add(frame.wrapping_mul(8));
    let top = u32::from(m.r16(s + 0x24))
        .wrapping_sub(u32::from(m.r16(e + 6)))
        .wrapping_sub(u32::from(m.r16(e + 2)))
        .wrapping_add(y);
    m.w32(out + 4, top);
    let v = u32::from(m.r16(s + 0x28)).wrapping_add(top);
    m.w32(out + 0xC, v);
    out
}

/// `0x551E60` (cdecl: fotograma, x, y, hoja): recorta el rectángulo del fotograma con la vista.
pub fn sheet_clip_frame(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) -> u32 {
    let (frame, x, y, s) = (a[0], a[1], a[2], a[3]);
    let rect = esp - 0x10;
    let mut left = x;
    m.w32(rect, left);
    if frame >= u32::from(m.r8(s + 0x2A)) {
        left = u32::from(at16(m, s + 0x10, frame.wrapping_mul(8).wrapping_add(4))).wrapping_add(x);
        m.w32(rect, left);
    }
    let e = m.r32(s + 0x10).wrapping_add(frame.wrapping_mul(8));
    let v = u32::from(m.r16(e + 4)).wrapping_add(left);
    m.w32(rect + 8, v);
    m.w32(rect + 0xC, y);
    let v = y.wrapping_sub(u32::from(m.r16(e + 6)));
    m.w32(rect + 4, v);
    let ui = m.r32(UI);
    h.call(m, CLIP_INTERSECTION, ui, &[rect])
}

/// Origen del sprite de capa (`+0x14`) para el fotograma: (x, y) de pantalla.
fn overlay_origin(m: &mut dyn Mem, s: u32, x: u32, y: u32, frame: u32) -> (u32, u32, u32) {
    let e = m.r32(s + 0x10).wrapping_add(frame.wrapping_mul(8));
    let ov = m.r32(s + 0x14);
    let top = u32::from(m.r16(ov + 0x12))
        .wrapping_sub(u32::from(m.r16(e + 6)))
        .wrapping_sub(u32::from(m.r16(e + 2)))
        .wrapping_add(y);
    let mut left = u32::from(m.r16(ov + 0x10)).wrapping_sub(u32::from(m.r16(e))).wrapping_add(x);
    if frame >= u32::from(m.r8(s + 0x2A)) {
        left = left.wrapping_add(u32::from(m.r16(e + 4)));
    }
    (left, top, ov)
}

/// `0x551ED0` (cdecl: x, y, fotograma, hoja): recorta el rectángulo del sprite de capa.
pub fn sheet_clip_overlay(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) -> u32 {
    let (left, top, ov) = overlay_origin(m, a[3], a[0], a[1], a[2]);
    let rect = esp - 0x10;
    m.w32(rect + 4, top);
    m.w32(rect, left);
    let r = m.r32(ov + 4);
    let v = u32::from(m.r16(r + 6)).wrapping_add(top);
    m.w32(rect + 0xC, v);
    let r = m.r32(ov + 4);
    let v = u32::from(m.r16(r + 4)).wrapping_add(left);
    m.w32(rect + 8, v);
    let ui = m.r32(UI);
    h.call(m, CLIP_INTERSECTION, ui, &[rect])
}

/// `0x551F70` (cdecl: x, y, fotograma, hoja): dibuja el sprite de capa.
pub fn sheet_draw_overlay(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32]) -> u32 {
    let s = a[3];
    let ov = m.r32(s + 0x14);
    let t = m.r32(ov + 0xC);
    cdecl(m, h, SGL_SET_RENDER_SOURCE, &[t]);
    let (left, top, ov) = overlay_origin(m, s, a[0], a[1], a[2]);
    let r = m.r32(ov + 4);
    let v: Vec<u32> = (0..4).map(|k| u32::from(m.r16(r + 2 * k))).collect();
    cdecl(m, h, SGL_DRAW_BITMAP_RECT, &[v[0], v[1], left, top, v[2], v[3]])
}

/// `0x552010` (cdecl: x, y, fotograma, hoja): dibuja el fotograma con la máscara alfa.
pub fn sheet_draw_frame_masked(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32]) -> u32 {
    let (x, y, frame, s) = (a[0], a[1], a[2], a[3]);
    if m.r32(s + 4) == 0 {
        return 0;
    }
    let e = m.r32(s + 0x10).wrapping_add((frame & 0xFF) * 8);
    let dx = if frame as u8 >= m.r8(s + 0x2A) { u32::from(m.r16(e + 4)).wrapping_add(x) } else { x };
    let t = m.r32(s + 0xC);
    cdecl(m, h, SGL_SET_RENDER_SOURCE, &[t]);
    let v: Vec<u32> = (0..4).map(|k| u32::from(m.r16(e + 2 * k))).collect();
    let mask = m.r32(s + 4);
    cdecl(m, h, SGL_DRAW_BITMAP_RECT_ALPHA, &[mask, v[0], v[1], dx, y.wrapping_sub(v[3]), v[2], v[3]])
}

/// `0x552090` (cdecl: hoja, variante): elige la textura de la variante o la anterior no nula.
pub fn sheet_select_variant(m: &mut dyn Mem, s: u32, idx: u32) -> u32 {
    let t = m.r32(s + 0x1C);
    if t == 0 {
        return 0;
    }
    let mut p = t.wrapping_add(idx.wrapping_mul(4));
    loop {
        let v = m.r32(p);
        m.w32(s + 0xC, v);
        if v != 0 {
            return v;
        }
        p = p.wrapping_sub(4);
    }
}

/// `0x552900` (thiscall en la interfaz `+0x3C`, 4 argumentos: dibujante, x, y, fotograma): `dibujante->vtable[0x18]`.
pub fn map_object_draw_via_renderer(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) -> u32 {
    let (r, x, y, frame) = (a[0], a[1], a[2], a[3]);
    let e = m.r32(this - 0x2C).wrapping_add((frame & 0xFF) * 8);
    let dx = if frame as u8 >= m.r8(this - 0x12) { u32::from(m.r16(e + 4)).wrapping_add(x) } else { x };
    let v: Vec<u32> = (0..4).map(|k| u32::from(m.r16(e + 2 * k))).collect();
    let t = m.r32(this - 0x30);
    vcall(m, h, r, 0x18, &[t, dx, y.wrapping_sub(v[3]), v[0], v[1], v[2], v[3]])
}

/// `0x552C10` (thiscall, 3 argumentos: x, y, fotograma): incendio, estados animados y sonido del edificio.
pub fn town_sprite_draw_full(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) -> u32 {
    let (x, y, frame) = (a[0], a[1], a[2]);
    let s = vcall(m, h, this, 0x74, &[]);
    let r = h.call(m, BURN_ANIMATE, this + 4, &[x, y, frame, s]);
    if r as u8 != 0 {
        return r;
    }
    let bl = vcall(m, h, this, 0x80, &[]) as u8;
    let states = m.r32(this + 0x14);
    // El original guarda el byte en el hueco del segundo argumento y empuja la palabra entera.
    let r = cdecl(m, h, ANIM_STATES_UPDATE_DRAW, &[x, y, frame, s, states, (y & !0xFF) | u32::from(bl)]);
    if bl == 0 {
        return r;
    }
    let kind = vcall(m, h, this, 0x50, &[]);
    let ui = m.r32(UI);
    h.call(m, QUEUE_BUILDING_SOUND, ui, &[x, y, kind])
}

/// `0x552C90` y `0x552D90` (thiscall, 3 argumentos: x, y, fotograma): dibujo con incendio o normal (con máscara alfa si `masked`).
pub fn town_sprite_draw_frame(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32, masked: bool) -> u32 {
    let frame = a[2];
    let s = vcall(m, h, this, 0x74, &[]);
    if masked && m.r32(s + 4) == 0 {
        return 0;
    }
    let e = m.r32(s + 0x10).wrapping_add((frame & 0xFF) * 8);
    let px = esp - 4;
    let py = esp + 8;
    let x = if frame as u8 >= m.r8(s + 0x2A) { u32::from(m.r16(e + 4)).wrapping_add(a[0]) } else { a[0] };
    m.w32(px, x);
    let color = cdecl(m, h, SGL_GET_CONSTANT_COLOR, &[]);
    if vcall(m, h, this, 0x84, &[]) as u8 != 0 {
        let c = m.r32(HIGHLIGHT_COLOR);
        cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[c]);
    }
    let burn = if masked { BURN_DRAW_MASKED } else { BURN_DRAW_FRAME };
    if h.call(m, burn, this + 4, &[s, e, px, py]) as u8 != 0 {
        return cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color]);
    }
    let t = m.r32(s + 0xC);
    cdecl(m, h, SGL_SET_RENDER_SOURCE, &[t]);
    let v: Vec<u32> = (0..4).map(|k| u32::from(m.r16(e + 2 * k))).collect();
    let y = m.r32(py);
    let x = m.r32(px);
    if masked {
        let mask = m.r32(s + 4);
        cdecl(m, h, SGL_DRAW_BITMAP_RECT_ALPHA, &[mask, v[0], v[1], x, y.wrapping_sub(v[3]), v[2], v[3]]);
    } else {
        cdecl(m, h, SGL_DRAW_BITMAP_RECT, &[v[0], v[1], x, y.wrapping_sub(v[3]), v[2], v[3]]);
        let states = m.r32(this + 0x14);
        let y = m.r32(py);
        let x = m.r32(px);
        cdecl(m, h, ANIM_STATES_DRAW, &[x, y, frame, s, states]);
    }
    cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[color])
}

/// `0x552E80` (cdecl: hoja, &tabla): un estado de animación por grupo (fábrica `0x552EF0` con la primera animación del grupo).
pub fn create_animators(m: &mut dyn Mem, h: &mut dyn Host, s: u32, out: u32) {
    let n = m.r32(s + 0x34);
    if n == 0 {
        return;
    }
    let p = new(m, h, n * 4);
    m.w32(out, p);
    for i in 0..n {
        let g = if i < m.r32(s + 0x34) { at32(m, s + 0x38, i * 4) } else { 0 };
        let first = if m.r32(g + 4) > 0 { at32(m, g + 8, 0) } else { 0 };
        let t = m.r32(out);
        cdecl(m, h, ANIMATOR_FACTORY, &[t + i * 4, first]);
    }
}

/// `0x552EF0` (cdecl: &salida, animación): estado de animación según el tipo (`+0x16`) de la animación.
pub fn animator_factory(m: &mut dyn Mem, h: &mut dyn Host, out: u32, anim: u32) {
    m.w32(out, 0);
    if anim == 0 {
        return;
    }
    let (size, ctor) = match m.r16(anim + 0x16) {
        2 => (0x10, ANIM_STATE_CTOR),
        4 => (0x10, 0x0054_EB70),
        5 => (0x10, 0x0055_21A0),
        7 => (0x14, 0x0054_EDC0),
        9 => (0x14, 0x0055_2470),
        _ => return,
    };
    let o = construct(m, h, size, ctor);
    m.w32(out, o);
}

/// `0x553070` (cdecl: hoja, &tabla): destruye los estados de animación (del último al primero) y la tabla.
pub fn destroy_animators(m: &mut dyn Mem, h: &mut dyn Host, s: u32, arr: u32) {
    if m.r32(arr) == 0 {
        return;
    }
    let mut i = m.r32(s + 0x34).wrapping_sub(1) as i32;
    while i >= 0 {
        let o = at32(m, arr, i as u32 * 4);
        if o != 0 {
            vcall(m, h, o, 0, &[1]);
        }
        i -= 1;
    }
    let t = m.r32(arr);
    free(m, h, t);
    m.w32(arr, 0);
}

/// `0x553150`: variante del sprite según el número de casilla (`1 + n % (variantes - 1)`; 0 fuera del mapa o sin variantes).
pub fn town_variant_pick(m: &mut dyn Mem, h: &mut dyn Host, this: u32, n: u32) {
    let ui = m.r32(UI);
    let cells = m.r32(ui + 0x514).wrapping_mul(m.r32(ui + 0x510));
    if n < cells {
        let l = vcall(m, h, this, 0x8C, &[]);
        if m.r32(l + 4) > 1 {
            let l = vcall(m, h, this, 0x8C, &[]);
            let k = m.r32(l + 4) - 1;
            m.w8(this + 0x18, ((n % k) as u8).wrapping_add(1));
            return;
        }
    }
    m.w8(this + 0x18, 0);
}

/// `0x5531B0` (thiscall: ruta): lee del fichero la tabla de variantes, asegura las hojas de la lista (`vtable+0x8C`) y carga la hoja.
/// `esp` es la pila de entrada (`ArchiveFile` en `esp-0x14`, tabla y número en `esp-0x24`/`esp-0x20`).
pub fn town_variant_load(m: &mut dyn Mem, h: &mut dyn Host, this: u32, path: u32, esp: u32) -> u8 {
    let file = esp - 0x14;
    h.call(m, ARCHIVE_CTOR, file, &[]);
    if h.call(m, ARCHIVE_OPEN, file, &[path, 0x8000]) == 0 {
        h.call(m, ARCHIVE_DTOR, file, &[]);
        return 0;
    }
    let (arrp, countp) = (esp - 0x24, esp - 0x20);
    m.w32(arrp, 0);
    m.w32(countp, 0);
    cdecl(m, h, READ_ARRAY_AT_24, &[file, arrp, countp]);
    h.call(m, ARCHIVE_CLOSE, file, &[]);
    let list = vcall(m, h, this, 0x8C, &[]);
    let arr = m.r32(arrp);
    if m.r32(arr.wrapping_add(8)) == 0 {
        m.w8(this + 0x18, 0);
        if m.r32(list + 4) == 0 {
            let n = m.r32(list + 4) + 1;
            m.w32(list + 4, n);
            let t = m.r32(list + 8);
            let t = cdecl(m, h, CRT_REALLOC, &[t, n * 4]);
            m.w32(list + 8, t);
            let s = construct(m, h, 0x3C, SHEET_CTOR);
            let n = m.r32(list + 4);
            let t = m.r32(list + 8);
            m.w32(t.wrapping_add(n.wrapping_mul(4)).wrapping_sub(4), s);
        }
    } else {
        let n = m.r32(list + 4);
        let want = n.wrapping_add(if n != 0 { 1 } else { 2 });
        let t = m.r32(list + 8);
        let t = cdecl(m, h, CRT_REALLOC, &[t, want.wrapping_mul(4)]);
        let mut i = m.r32(list + 4);
        m.w32(list + 8, t);
        while i < want {
            let s = construct(m, h, 0x3C, SHEET_CTOR);
            i += 1;
            let t = m.r32(list + 8);
            m.w32(t + (i - 1) * 4, s);
        }
        m.w32(list + 4, want);
        let n = m.r32(list + 4) as u8;
        m.w8(this + 0x18, n.wrapping_sub(1));
    }
    let arr = m.r32(arrp);
    if arr != 0 {
        free(m, h, arr);
    }
    let s = vcall(m, h, this, 0x74, &[]);
    let ok = h.call(m, READER_LOAD_SHEET, this, &[path, s]) as u8;
    h.call(m, ARCHIVE_DTOR, file, &[]);
    ok
}

/// `0x5533B0`: vacía la hoja actual y, si ninguna hoja de la lista tiene entradas, destruye la lista.
pub fn town_variant_release_unused(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let s = vcall(m, h, this, 0x74, &[]);
    h.call(m, SHEET_RESET, s, &[]);
    let l = vcall(m, h, this, 0x8C, &[]);
    let n = m.r32(l + 4);
    let mut i = 0;
    while i < n {
        let s = at32(m, l + 8, i * 4);
        if m.r16(s + 0x20) != 0 {
            break;
        }
        i += 1;
    }
    if i == n {
        destroy_table(m, h, l + 8, l + 4, 0, 1);
    }
}

/// `0x553440`: hoja de la variante actual (0 si no hay).
pub fn town_variant_resource(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    let v = m.r8(this + 0x18);
    if v == 0xFF {
        return 0;
    }
    let l = vcall(m, h, this, 0x8C, &[]);
    if u32::from(v) >= m.r32(l + 4) {
        return 0;
    }
    at32(m, l + 8, u32::from(v) * 4)
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    let r = match f {
        0x54F710 => Ret::U8(merchant_has_building_type_flag(m, h, a[0])),
        0x54F7E0 => Ret::U32(map_object_factory(m, h, ecx, a[0], a[1], a[2])),
        0x5502C0 => {
            map_object_destroy_and_clear(m, h, a[0]);
            Ret::None
        }
        0x5502F0 => {
            let s = vcall(m, h, ecx, 0x74, &[]);
            Ret::U32(h.call(m, READER_LOAD_SHEET, ecx, &[a[0], s]))
        }
        0x550310 | 0x550330 => {
            let s = vcall(m, h, ecx, 0x74, &[]);
            let t = if f == 0x550310 { SHEET_DRAW_FRAME } else { SHEET_DRAW_MASKED };
            Ret::U32(cdecl(m, h, t, &[a[0], a[1], a[2], s]))
        }
        0x550350 => {
            let s = vcall(m, h, ecx, 0x74, &[]);
            Ret::U32(cdecl(m, h, SHEET_DRAW_LAYERS, &[a[0], a[1], s, 0]))
        }
        0x550370 | 0x550390 => Ret::None,
        0x550380 => Ret::U32(0),
        0x5503A0 => Ret::U32(vcall(m, h, ecx, 0x44, a)),
        0x5503B0 => Ret::U32(deleting_dtor(m, h, ecx, SPRITE_BASE_DTOR, a[0])),
        0x550430 => {
            burn_reset(m, ecx, true);
            Ret::U32(ecx)
        }
        0x550460 => Ret::U32(burn_reset(m, ecx, true)),
        0x550490 => Ret::U32(burn_reset(m, ecx, false)),
        0x5504C0 => Ret::U32(secondary_sprite_info(m, a[0], a[1])),
        0x550520 => Ret::U32(burnt_height(m, ecx, a[0])),
        0x550A70 => Ret::U8(burn_animate(m, h, ecx, a)),
        0x550CE0 => Ret::U8(burn_draw_frame_masked(m, h, ecx, a)),
        0x550FA0 => {
            burn_reset(m, ecx, true);
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        0x550FE0 => {
            m.w32(ecx, 0x0067_3E6C);
            Ret::U32(ecx)
        }
        0x550FF0 => {
            m.w32(ecx, 0x0067_3E6C);
            Ret::None
        }
        0x551000 => Ret::U8(reader_load_sheet(m, h, ecx, a[0], a[1], esp)),
        0x5512A0 => Ret::U8(read_header_flag_chunk(m, a[0], a[1])),
        0x5512D0 => Ret::U8(read_at(m, h, a[0], a[1], 4)),
        0x551300 => Ret::U8(read_at(m, h, a[0], a[1], 2)),
        0x551330 => Ret::U8(read_counted_array(m, h, a[0], a[1], a[2])),
        0x551380 => Ret::U8(read_animation_chunk(m, h, a[0], a[1])),
        0x5513F0 => Ret::U8(read_extra_chunks(m, h, ecx, a[0], a[1], esp)),
        0x5515F0 => {
            m.w32(ecx, 0x0067_3E6C);
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        0x551610 => {
            group_list_resize(m, h, ecx, a[0]);
            Ret::None
        }
        0x5516C0 => Ret::U32(group_append(m, h, ecx)),
        0x551740 => {
            m.w32(ecx, 0x0067_3E78);
            destroy_table(m, h, ecx + 8, ecx + 4, 4, 1);
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        0x5517B0 => {
            h.call(m, ANIM_BASE_CTOR, ecx, &[]);
            m.w32(ecx, 0x0067_3E80);
            h.call(m, ANIM_BASE_RESET, ecx, &[]);
            m.w8(ecx + 0xC, 0xC0);
            Ret::U32(ecx)
        }
        0x551800 => {
            m.w32(ecx, 0x0067_3E80);
            Ret::U32(h.call(m, ANIM_BASE_DTOR, ecx, &[]))
        }
        0x551810 => {
            h.call(m, ANIM_BASE_RESET, ecx, &[]);
            m.w8(ecx + 0xC, 0xC0);
            Ret::None
        }
        0x551820 => Ret::U8(anim_state_draw_current(m, h, ecx, a)),
        0x551A00 => Ret::U32(deleting_dtor(m, h, ecx, ANIM_STATE_DTOR, a[0])),
        0x551A80 => {
            m.w32(ecx + 0x30, 0x0067_3E98);
            m.w32(ecx + 0x38, 0);
            m.w32(ecx + 0x34, 0);
            m.w32(ecx, 0x0067_3E94);
            for off in [0x10, 0x14, 0x18, 0x1C] {
                m.w32(ecx + off, 0);
            }
            h.call(m, SHEET_RESET, ecx, &[]);
            Ret::U32(ecx)
        }
        0x551AE0 => {
            m.w32(ecx, 0x0067_3E94);
            m.w32(ecx + 0x30, 0x0067_3E98);
            destroy_table(m, h, ecx + 0x38, ecx + 0x34, 0, 1);
            Ret::None
        }
        0x551B40 => {
            sheet_reset(m, h, ecx);
            Ret::None
        }
        0x551C00 => {
            h.call(m, ANIM_SET_RESET, FLAME_SETS, &[]);
            h.call(m, ANIM_SET_RESET, FLAME_SETS + 0x18, &[]);
            Ret::None
        }
        0x551C20 => Ret::U32(sheet_draw_footprint(m, h, a)),
        0x551CF0 => {
            sheet_draw_all_layers(m, h, a, esp);
            Ret::None
        }
        0x551DE0 => Ret::U32(sheet_frame_screen_rect(m, a)),
        0x551E60 => Ret::U32(sheet_clip_frame(m, h, a, esp)),
        0x551ED0 => Ret::U32(sheet_clip_overlay(m, h, a, esp)),
        0x551F70 => Ret::U32(sheet_draw_overlay(m, h, a)),
        0x552010 => Ret::U32(sheet_draw_frame_masked(m, h, a)),
        0x552090 => Ret::U32(sheet_select_variant(m, a[0], a[1])),
        0x5520C0 => Ret::U32(deleting_dtor(m, h, ecx, SHEET_DTOR, a[0])),
        0x5520E0 => {
            m.w32(ecx, 0x0067_3E98);
            destroy_table(m, h, ecx + 8, ecx + 4, 0, 1);
            Ret::None
        }
        0x552130 => {
            m.w32(ecx, 0x0067_3E98);
            destroy_table(m, h, ecx + 8, ecx + 4, 0, 1);
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        0x5521A0 | 0x552470 => {
            h.call(m, ANIM_STATE_CTOR, ecx, &[]);
            m.w32(ecx, if f == 0x5521A0 { 0x0067_3EA0 } else { 0x0067_3EB4 });
            h.call(m, ANIM_STATE_RESET, ecx, &[]);
            if f == 0x552470 {
                m.w8(ecx + 0x10, 0);
                m.w8(ecx + 0x11, 1);
            }
            Ret::U32(ecx)
        }
        0x5521F0 | 0x5524D0 => {
            m.w32(ecx, if f == 0x5521F0 { 0x0067_3EA0 } else { 0x0067_3EB4 });
            Ret::U32(h.call(m, ANIM_STATE_DTOR, ecx, &[]))
        }
        0x552450 => Ret::U32(deleting_dtor(m, h, ecx, RANDOM_STATE_DTOR, a[0])),
        0x5524E0 => {
            h.call(m, ANIM_STATE_RESET, ecx, &[]);
            m.w8(ecx + 0x10, 0);
            m.w8(ecx + 0x11, 1);
            Ret::None
        }
        0x552800 => Ret::U32(deleting_dtor(m, h, ecx, GROUPED_STATE_DTOR, a[0])),
        0x552820 => {
            h.call(m, SHEET_CTOR, ecx, &[]);
            h.call(m, SPRITE_BASE_CTOR, ecx + 0x3C, &[]);
            m.w32(ecx, 0x0067_3F48);
            m.w32(ecx + 0x3C, 0x0067_3EC8);
            for off in [0x10, 0x14, 0x18, 0x1C] {
                m.w32(ecx + off, 0);
            }
            h.call(m, SHEET_RESET, ecx, &[]);
            Ret::U32(ecx)
        }
        0x552890 => {
            m.w32(ecx, 0x0067_3F48);
            m.w32(ecx + 0x3C, 0x0067_3EC8);
            h.call(m, SHEET_RESET, ecx, &[]);
            h.call(m, SPRITE_BASE_DTOR, ecx + 0x3C, &[]);
            Ret::U32(h.call(m, SHEET_DTOR, ecx, &[]))
        }
        0x552900 => Ret::U32(map_object_draw_via_renderer(m, h, ecx, a)),
        0x552970 => Ret::U32(h.call(m, SHEET_RESET, ecx.wrapping_sub(0x3C), &[])),
        0x552980 | 0x5529A0 | 0x5529C0 | 0x5529E0 | 0x552A30 | 0x552A50 => {
            let t = match f {
                0x552980 => SHEET_DRAW_FRAME,
                0x5529A0 => SHEET_DRAW_MASKED,
                0x5529C0 => SHEET_FOOTPRINT,
                0x5529E0 => SHEET_CLIP_OVERLAY,
                0x552A30 => SHEET_CLIP_FRAME,
                _ => SHEET_OVERLAY,
            };
            Ret::U32(cdecl(m, h, t, &[a[0], a[1], a[2], ecx.wrapping_sub(0x3C)]))
        }
        0x552A00 => Ret::U32(m.r32(ecx.wrapping_sub(0x28))),
        0x552A10 => Ret::U32(cdecl(m, h, SHEET_DRAW_LAYERS, &[a[0], a[1], ecx.wrapping_sub(0x3C), 0])),
        0x552A70 => Ret::U32(h.call(m, READER_LOAD_SHEET, ecx, &[a[0], ecx.wrapping_sub(0x3C)])),
        0x552A90 => Ret::U32(u32::from(m.r16(ecx.wrapping_sub(0x16)))),
        0x552AA0 => Ret::U32(u32::from(m.r16(ecx.wrapping_sub(0x14)))),
        0x552AB0 => Ret::U32(u32::from(m.r8(ecx.wrapping_sub(0x12)))),
        0x552AC0 => Ret::U32(u32::from(m.r8(ecx.wrapping_sub(0x11)))),
        0x552AD0 => Ret::U32(u32::from(m.r8(ecx.wrapping_sub(0xF)))),
        0x552AE0 => Ret::U32(m.r32(ecx.wrapping_sub(0x2C))),
        0x552AF0 => Ret::U16(m.r16(ecx.wrapping_sub(0x1C))),
        0x552B00 => Ret::U32(m.r32(ecx.wrapping_sub(0x30))),
        0x552B10 => Ret::U32(ecx.wrapping_sub(0x3C)),
        0x552B20 => Ret::U32(deleting_dtor(m, h, ecx, SHEET_MAP_OBJECT_DTOR, a[0])),
        0x552B40 => Ret::U32(h.call(m, SPRITE_RESOURCE_SDD, ecx.wrapping_sub(0x3C), &[a[0]])),
        0x552B50 => {
            h.call(m, SPRITE_BASE_CTOR, ecx, &[]);
            h.call(m, BURN_CTOR, ecx + 4, &[]);
            m.w32(ecx + 4, 0x0067_3FD8);
            m.w32(ecx, 0x0067_3F50);
            m.w32(ecx + 0x14, 0);
            Ret::U32(ecx)
        }
        0x552BB0 => {
            m.w32(ecx, 0x0067_3F50);
            m.w32(ecx + 4, 0x0067_3FD8);
            h.call(m, BURN_CTOR_ALT, ecx + 4, &[]);
            Ret::U32(h.call(m, SPRITE_BASE_DTOR, ecx, &[]))
        }
        0x552C10 => Ret::U32(town_sprite_draw_full(m, h, ecx, a)),
        0x552C90 => Ret::U32(town_sprite_draw_frame(m, h, ecx, a, esp, false)),
        0x552D90 => Ret::U32(town_sprite_draw_frame(m, h, ecx, a, esp, true)),
        0x552E80 => {
            create_animators(m, h, a[0], a[1]);
            Ret::None
        }
        0x552EF0 => {
            animator_factory(m, h, a[0], a[1]);
            Ret::None
        }
        0x553070 => {
            destroy_animators(m, h, a[0], a[1]);
            Ret::None
        }
        0x5530B0 => {
            let s = vcall(m, h, ecx, 0x74, &[]);
            Ret::U32(h.call(m, BURN_UPDATE, ecx + 4, &[esp + 4, esp + 8, esp + 0xC, esp + 0x10, a[4], ecx, s]))
        }
        0x5530E0 => Ret::U32(deleting_dtor(m, h, ecx, TOWN_SPRITE_DTOR, a[0])),
        0x553100 => Ret::U32(h.call(m, TOWN_SPRITE_SDD, ecx.wrapping_sub(4), &[a[0]])),
        0x553110 => {
            h.call(m, TOWN_SPRITE_CTOR, ecx, &[]);
            m.w32(ecx, 0x0067_3FE8);
            m.w32(ecx + 4, 0x0067_3FE0);
            m.w8(ecx + 0x18, 0xFF);
            Ret::U32(ecx)
        }
        0x553130 => {
            m.w32(ecx, 0x0067_3FE8);
            m.w32(ecx + 4, 0x0067_3FE0);
            Ret::U32(h.call(m, TOWN_SPRITE_DTOR, ecx, &[]))
        }
        0x553150 => {
            town_variant_pick(m, h, ecx, a[0]);
            Ret::None
        }
        0x5531B0 => Ret::U8(town_variant_load(m, h, ecx, a[0], esp)),
        0x5533B0 => {
            town_variant_release_unused(m, h, ecx);
            Ret::None
        }
        0x553440 => Ret::U32(town_variant_resource(m, h, ecx)),
        0x553470 => Ret::U32(deleting_dtor(m, h, ecx, TOWN_VARIANT_DTOR, a[0])),
        _ => return None,
    };
    Some(r)
}
