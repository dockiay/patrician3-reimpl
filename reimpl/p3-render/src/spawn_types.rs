//! Métodos virtuales `+0x40` (aparición) de ocho tipos de la fábrica: decorados 5 y 6, soldados 9, 10, 11, 35 y 36 y el tipo 8 (escalera/guardia).
//! Traducción 1:1 del x86; el objeto es memoria plana ([`SceneMem`]) y los externos pasan por [`TypeEnv`].
use crate::ambience::SceneMem;

/// Tabla de conjuntos de sprites de un tipo: `count` (`DAT_…`), `sets` (puntero a `count` punteros) y `kinds` (un byte por conjunto).
pub struct Table<'a> {
    pub count: u32,
    pub sets: &'a [u32],
    pub kinds: &'a [u8],
}

impl Table<'_> {
    fn set(&self, i: u32) -> u32 {
        if i < self.count {
            self.sets[i as usize]
        } else {
            0
        }
    }
}

pub trait TypeEnv {
    fn rand(&mut self) -> u32;
    /// `0x557730(ecx = objeto+0x20, conjunto, n)`: devuelve la animación elegida.
    fn f557730(&mut self, set: u32, n: u32) -> u8;
    fn f559cf0(&mut self, set: u32);
    /// `0x559260(rectA, rectB, conjunto, objeto+8)`
    fn f559260(&mut self, set: u32) -> bool;
    /// `0x59B5B0(rect)`: `which` 0 = primer rectángulo, 1 = segundo.
    fn f59b5b0(&mut self, which: u32);
    fn f559810(&mut self, set: u32);
    /// `0x55E620(ecx = objeto, x, y, rumbo, conjunto, enlace, tipo, bandera, clase, 0)`
    fn f55e620(&mut self, args: [u32; 10]);
    /// Byte en `registro_del_lado + off` (`[DAT_006DF270 + 8 + lado*0x1024]`).
    fn rec_byte(&mut self, side: u8, off: u32) -> u8;
    fn rec_word(&mut self, side: u8, off: u32) -> u16;
    /// `0x55A290(ecx = objeto+0x2C, a, b, c, d)`
    fn f55a290(&mut self, a: [u32; 4]);
    /// `0x559130(a, conjunto, objeto+8)`
    fn f559130(&mut self, a: u32, set: u32);
    /// `escena[0xC58C] += 1`
    fn bump_c58c(&mut self);
    /// Virtual `+0x1C()` del objeto.
    fn v1c(&mut self);
    /// Constantes leídas de memoria: 0 = `*[0x6E1808]`, 1 = `[0x6E17F8]`, 2 = `[0x6DCCF8]`, 3 = byte `escena+0x598`, 4 = `**(conjunto+0x20)` del conjunto `arg`.
    fn g(&mut self, which: u32, arg: u32) -> u32;
    /// `*(i16*)(**[0x6E17B8] + 10) += d`
    fn shift_w(&mut self, d: i32);
    fn rd8(&mut self, a: u32) -> u8;
    fn rd16(&mut self, a: u32) -> u16;
    fn rd32(&mut self, a: u32) -> u32;
    fn wr8(&mut self, a: u32, v: u8);
    fn wr16(&mut self, a: u32, v: u16);
    fn wr32(&mut self, a: u32, v: u32);
    /// Campo de la escena global (`[0x6E51AC] + off`).
    fn scene_r32(&mut self, off: u32) -> u32;
    fn scene_r8(&mut self, off: u32) -> u8;
    fn scene_r16(&mut self, off: u32) -> u16;
    /// `0x559370(rectA, conjunto, objeto+8)`
    fn f559370(&mut self, set: u32);
    /// `0x59A260(objeto)` con `ecx = escena`.
    fn f59a260(&mut self);
    /// Ayudante de colocación (`0x565D70` en el tipo 21, `0x575560` en el 22): `ecx = objeto`, dos palabras; rellena los campos del objeto.
    fn f_place(&mut self, which: u32, o: &mut SceneMem, a: u32, b: u32);
    /// `0x557730(ecx = objeto+0x2C, conjunto, n)`
    fn f557730_2c(&mut self, set: u32, n: u32) -> u8;
    fn f55e250(&mut self, set: u32);
    fn f55e430(&mut self, set: u32);
    fn f571da0(&mut self);
    /// `0x58CFB0(&{a, b}, &salida1, &salida2)`
    fn f58cfb0(&mut self, a: u32, b: u32) -> (u32, u32);
    fn f563090(&mut self, o: &mut SceneMem);
    /// `0x57AF60(ecx = objeto, conjunto, bandera)`
    fn f57af60(&mut self, set: u32, flag: u32);
    /// `0x5986A0(ecx = escena, a, b)`: palabras bajas.
    fn f5986a0(&mut self, a: u16, b: u16);
    fn f570800(&mut self, o: &mut SceneMem);
    /// `push_back` del vector global `[0x6E2F14..0x6E2F1C]` (señales de aviso).
    fn registry_push(&mut self, obj: u32);
    fn f5709c0(&mut self, o: &SceneMem) -> u8;
    /// `0x4D8DB0(ecx = 0x6DD73C, índice)`: puntero a la entrada.
    fn f4d8db0(&mut self, idx: u32) -> u32;
    /// `0x559790(conjunto, objeto+8, 0)`
    fn f559790(&mut self, set: u32);
    /// `0x58D030(x, y, &salida)`: devuelve las dos palabras de salida.
    fn f58d030(&mut self, x: i32, y: i32) -> (u32, u32);
    /// `0x630CC0(ecx = registro del lado, a, b, tamaño)`
    fn f630cc0(&mut self, side: u8, a: u32, b: u32, size: u32);
    /// `0x58B740(&a, &b, dirección)`
    fn f58b740(&mut self, a: &mut u32, b: &mut u32, dir: u8);
    /// `0x56A240()` con `ecx = objeto` (escribe `objeto+0x3A`).
    fn f56a240(&mut self, o: &mut SceneMem);
}

pub struct SpawnArgs {
    pub x: u32,
    pub y: u32,
    pub heading: u32,
    pub link: u32,
    pub variant: u32,
}

/// `0x5721D0` (tipo 5) y `0x56E5D0` (tipo 6): decorados con variante `rand % count` si `variant == 0`.
pub fn spawn_decor(o: &mut SceneMem, a: &SpawnArgs, t: &Table, env: &mut dyn TypeEnv) {
    if a.variant == 0 {
        let r = env.rand();
        o.w8(0x2D, (r % t.count) as u8);
    } else {
        o.w8(0x2D, a.variant as u8);
    }
    let set = t.set(u32::from(o.r8(0x2D)));
    let anim = env.f557730(set, 5);
    o.w8(0x15, anim);
    o.w32(0x24, 0);
    env.f559cf0(set);
    o.w16(0x16, a.link as u16);
    o.w32(0xC, a.x);
    o.w32(0x10, a.y);
    o.w8(0x14, a.heading as u8);
    let moved = env.f559260(set);
    env.f59b5b0(0);
    if moved {
        env.f59b5b0(1);
    }
    env.f559810(set);
}

fn call_55e620(a: &SpawnArgs, set: u32, ty: u32, kind: u32, env: &mut dyn TypeEnv) {
    env.f55e620([a.x, a.y, a.heading, set, a.link, a.variant & 0xFFFF_FF7F, ty, (a.variant >> 7) & 0xFFFF_FF01, kind, 0]);
}

/// `0x5673E0`, `0x5623B0`, `0x572A90` (`ty = 1`): elige al azar entre los conjuntos cuyo `kind` coincide con `variant >> 16` (muestreo con reserva).
pub fn spawn_pick_kind(o: &mut SceneMem, a: &SpawnArgs, t: &Table, ty: u32, env: &mut dyn TypeEnv) {
    let mut idx: u8 = 0;
    let mut n: u8 = 0;
    o.w8(0x88, 0);
    if t.count != 0 {
        loop {
            if u16::from(t.kinds[usize::from(idx)]) == (a.variant >> 16) as u16 && (n == 0 || (env.rand() as i32) % (i32::from(n) + 1) == 0) {
                o.w8(0x88, idx);
            }
            if u16::from(t.kinds[usize::from(idx)]) == (a.variant >> 16) as u16 {
                n = n.wrapping_add(1);
            }
            idx = idx.wrapping_add(1);
            if u32::from(idx) >= t.count {
                break;
            }
        }
    }
    let i = u32::from(o.r8(0x88));
    let kind = u32::from(t.kinds[i as usize]);
    call_55e620(a, t.set(i), ty, kind, env);
}

/// `0x56A700` (tipo 35) y `0x56F0B0` (tipo 36): conjunto al azar sin filtro (`ty = 1`, clase 0).
pub fn spawn_pick_plain(o: &mut SceneMem, a: &SpawnArgs, t: &Table, env: &mut dyn TypeEnv) {
    let mut idx: u8 = 0;
    let mut n: u8 = 0;
    o.w8(0x88, 0);
    if t.count != 0 {
        loop {
            if n == 0 || (env.rand() as i32) % (i32::from(n) + 1) == 0 {
                o.w8(0x88, idx);
            }
            n = n.wrapping_add(1);
            idx = idx.wrapping_add(1);
            if u32::from(idx) >= t.count {
                break;
            }
        }
    }
    let set = t.set(u32::from(o.r8(0x88)));
    call_55e620(a, set, 1, 0, env);
}

/// `0x5777B0` (tipo 8): como `spawn_pick_kind` con `ty = 2`, pero el filtro admite un segundo intento con `kind 0` y guarda la clase de unidad en `+0x88/+0x89/+0x90`.
pub fn spawn_type8(o: &mut SceneMem, a: &SpawnArgs, t: &Table, env: &mut dyn TypeEnv) {
    let mut want = (a.variant >> 16) as u8;
    o.w8(0x90, 0xFF);
    if want < 3 {
        o.w8(0x88, 0);
    } else {
        o.w8(0x88, 1);
        o.w8(0x89, want.wrapping_sub(3));
        want = 0;
    }
    let mut guard = 0;
    loop {
        let mut idx: u8 = 0;
        let mut n: u8 = 0;
        if t.count != 0 {
            loop {
                if t.kinds[usize::from(idx)] == want {
                    if n == 0 || (env.rand() as i32) % (i32::from(n) + 1) == 0 {
                        o.w8(0x90, idx);
                    }
                    n = n.wrapping_add(1);
                }
                idx = idx.wrapping_add(1);
                if u32::from(idx) >= t.count {
                    break;
                }
            }
        }
        want = 0;
        guard += 1;
        if o.r8(0x90) != 0xFF || guard > 4 {
            break;
        }
    }
    let i = u32::from(o.r8(0x90));
    let kind = u32::from(t.kinds[i as usize]);
    call_55e620(a, t.set(i), 2, kind, env);
    if o.r8(0x88) == 0 {
        o.w32(0x8C, 0);
    } else {
        let side = o.r8(0x6A);
        let c = env.rec_byte(side, 0x74 + u32::from(o.r8(0x89) >> 1));
        o.w8(0x3C, c);
        let w = env.rec_word(side, 0x3A);
        o.w32(0x8C, 0);
        if w != 0 {
            o.w8(0x3C, c.wrapping_sub(0x80));
        }
    }
}

/// `0x57E690` (tipo 16), `0x57C090` (tipo 17) y `0x573730` (tipo 26): decorados con variante `variant - 1` y llamada previa a `0x55A290`.
pub fn spawn_decor2(o: &mut SceneMem, a: &SpawnArgs, t: &Table, p55: [u32; 4], bump: bool, env: &mut dyn TypeEnv) {
    if a.variant == 0 {
        let r = env.rand();
        o.w8(0x39, (r % t.count) as u8);
    } else {
        o.w8(0x39, (a.variant as u8).wrapping_sub(1));
    }
    env.f55a290(p55);
    let set = t.set(u32::from(o.r8(0x39)));
    let anim = env.f557730(set, 2);
    o.w8(0x15, anim);
    o.w32(0x24, 0);
    env.f559cf0(set);
    o.w16(0x16, a.link as u16);
    o.w32(0xC, a.x);
    o.w32(0x10, a.y);
    o.w8(0x14, a.heading as u8);
    let moved = env.f559260(set);
    env.f59b5b0(0);
    if moved {
        env.f59b5b0(1);
    }
    env.f559810(set);
    if bump {
        env.bump_c58c();
    }
}

/// `0x575A80` (tipo 33, lluvia): solo escribe campos.
pub fn spawn_t33(o: &mut SceneMem, a: &SpawnArgs) {
    o.w16(0x16, a.link as u16);
    o.w32(0x10, a.y << 16);
    o.w32(0xC, a.x << 16);
    o.w8(0x23, (a.variant >> 24) as u8);
    o.w8(0x14, 0);
    o.w8(0x15, 0);
    o.w8(0x21, (a.variant >> 16) as u8);
    o.w8(0x20, 0);
    o.w8(0x22, 0);
}

/// `0x5732B0` (tipo 28, niebla).
pub fn spawn_t28(o: &mut SceneMem, a: &SpawnArgs, t: &Table, env: &mut dyn TypeEnv) {
    let r1 = env.rand();
    o.w16(0x16, a.link as u16);
    o.w8(0x20, (r1 % t.count) as u8);
    let mut r2 = env.rand() & 0x8000_FFFF;
    if (r2 as i32) < 0 {
        r2 = ((r2.wrapping_sub(1)) | 0xFFFF_0000).wrapping_add(1);
    }
    o.w32(0xC, r2.wrapping_add(a.x.wrapping_mul(0x10000)));
    o.w8(0x14, a.heading as u8);
    o.w32(0x10, a.y << 16);
    let i = u32::from(o.r8(0x20));
    env.f559130(0, if t.count <= i { 0 } else { t.sets[i as usize] });
}

/// `0x576250` (tipo 29, nieve).
pub fn spawn_t29(o: &mut SceneMem, a: &SpawnArgs, t: &Table, env: &mut dyn TypeEnv) {
    let r1 = env.rand();
    o.w8(0x14, 0);
    o.w16(0x16, a.link as u16);
    o.w32(0xC, a.x << 16);
    o.w32(0x10, a.y << 16);
    let idx = (r1 % t.count) as u8;
    o.w8(0x24, idx);
    let set = t.set(u32::from(idx));
    let r2 = env.rand();
    let first = env.g(4, set);
    o.w8(0x25, (a.variant >> 16) as u8);
    o.w8(0x15, (r2 % first) as u8);
    o.w8(0x26, (a.variant >> 24) as u8);
}

/// `0x568DD0` (tipo 30, banderín).
pub fn spawn_t30(o: &mut SceneMem, a: &SpawnArgs, env: &mut dyn TypeEnv) {
    let r1 = env.rand();
    let u3 = env.g(0, 0);
    o.w32(0x20, 0);
    o.w32(0xC, a.x);
    o.w32(0x10, a.y);
    o.w8(0x15, (r1 % u3) as u8);
    o.w16(0x16, a.link as u16);
    let s598 = env.g(3, 0) as u8;
    o.w8(0x14, s598);
    let r2 = env.rand();
    let m = env.g(1, 0);
    let base = env.g(2, 0);
    o.w32(0x24, a.variant);
    o.w32(0x20, (r2 % m).wrapping_add(base));
    env.v1c();
}

/// `0x5687B0` (tipo 14, bala): usa el conjunto fijo `0x6E17A0`; `p6` (palabra) va en `+0x20`.
pub fn spawn_t14(o: &mut SceneMem, a: &SpawnArgs, env: &mut dyn TypeEnv) {
    const SET: u32 = 0x6E_17A0;
    env.f559cf0(SET);
    o.w16(0x16, a.link as u16);
    o.w16(0x20, a.variant as u16);
    o.w32(0xC, a.x);
    o.w32(0x10, a.y);
    o.w8(0x14, a.heading as u8);
    env.f559810(SET);
    env.shift_w(-i32::from(o.r16(0x20) as i16));
    env.f559130(0, SET);
    env.shift_w(i32::from(o.r16(0x20) as i16));
}

/// Fila de rejilla de un conjunto: `*(set+0x18)[((h * *(set+0x14) + 0x80) >> 8) & (*(set+0x14) - 1)]`.
fn row_of(env: &mut dyn TypeEnv, set: u32, heading: u8) -> u32 {
    let cnt = env.rd32(set.wrapping_add(0x14));
    let idx = (u32::from(heading).wrapping_mul(cnt).wrapping_add(0x80) >> 8) & cnt.wrapping_sub(1);
    let dirs = env.rd32(set.wrapping_add(0x18));
    env.rd32(dirs.wrapping_add(idx.wrapping_mul(4)))
}

fn adj16(env: &mut dyn TypeEnv, a: u32, d: i32) {
    let v = env.rd16(a);
    env.wr16(a, (i32::from(v as i16).wrapping_add(d)) as u16);
}

/// `0x563D50` (tipo 15, aves): `tbl` es la tabla de registros de 0x14 bytes `{byte base, u32 rango, …}`.
pub fn spawn_t15(o: &mut SceneMem, a: &SpawnArgs, t: &Table, tbl: u32, env: &mut dyn TypeEnv) {
    if a.variant == 0 {
        let r = env.rand();
        o.w8(0x2E, (r % t.count) as u8);
        let rec = tbl.wrapping_add(((r % t.count) & 0xFF).wrapping_mul(0x14));
        let r2 = env.rand();
        let range = env.rd32(rec.wrapping_add(4));
        let base = env.rd8(rec);
        o.w8(0x2C, ((r2 % range) as u8).wrapping_add(base));
    } else {
        o.w8(0x2C, a.variant as u8);
        o.w8(0x2E, (a.variant >> 16) as u8);
    }
    let set = t.set(u32::from(o.r8(0x2E)));
    let anim = env.f557730(set, 0);
    o.w16(0x16, a.link as u16);
    o.w32(0xC, a.x);
    o.w8(0x15, anim);
    o.w32(0x24, 0);
    o.w32(0x10, a.y);
    let h = a.heading as u8;
    o.w8(0x14, h);
    let row = row_of(env, set, h);
    let frame = row.wrapping_add(u32::from(anim) * 0xC);
    adj16(env, frame.wrapping_add(10), -i32::from(o.r8(0x2C)));
    let l = i64::from(o.r8(0x2C)) * -0x6666_6667_i64;
    let d = ((l >> 33) as i16).wrapping_sub((l >> 63) as i16);
    let frame8 = row.wrapping_add(u32::from(o.r8(0x15)) * 0xC).wrapping_add(8);
    adj16(env, frame8, i32::from(d));
    let moved = env.f559260(set);
    env.f59b5b0(0);
    if moved {
        env.f59b5b0(1);
    }
    let fr = row.wrapping_add(u32::from(o.r8(0x15)) * 0xC);
    adj16(env, fr.wrapping_add(10), i32::from(o.r8(0x2C)));
    adj16(env, fr.wrapping_add(8), i32::from(o.r8(0x2C) / 5));
}

/// `0x57B640` (tipo 19, cascotes): registros de 0x10 bytes `{byte, u32, byte, u32}`.
pub fn spawn_t19(o: &mut SceneMem, a: &SpawnArgs, t: &Table, tbl: u32, env: &mut dyn TypeEnv) {
    if a.variant == 0 {
        if env.scene_r8(0xC594) == 0 {
            o.w8(0x35, 0);
        } else {
            let r = env.rand();
            o.w8(0x35, (r % t.count) as u8);
        }
        let rec = tbl.wrapping_add(u32::from(o.r8(0x35)) * 0x10);
        let r = env.rand();
        let range = env.rd32(rec.wrapping_add(4));
        let base = env.rd8(rec);
        o.w8(0x2D, ((r % range) as u8).wrapping_add(base));
    } else {
        o.w8(0x2D, a.variant as u8);
        o.w8(0x35, (a.variant >> 16) as u8);
    }
    let idx = u32::from(o.r8(0x35));
    let rec = tbl.wrapping_add(idx * 0x10);
    let r = env.rand();
    let cb = env.rd8(rec.wrapping_add(8));
    let range = env.rd32(rec.wrapping_add(0xC));
    o.w32(0x30, 0);
    o.w8(0x34, ((r % range) as u8).wrapping_add(cb));
    let set = t.set(idx);
    let anim = env.f557730(set, 0);
    o.w8(0x15, anim);
    o.w32(0x24, 0);
    env.f559cf0(set);
    o.w16(0x16, a.link as u16);
    o.w32(0xC, a.x);
    o.w32(0x10, a.y);
    let h = a.heading as u8;
    o.w8(0x14, h);
    let row = row_of(env, set, h);
    let fr = row.wrapping_add(u32::from(o.r8(0x15)) * 0xC).wrapping_add(10);
    adj16(env, fr, -i32::from(o.r8(0x2C)));
    let moved = env.f559260(set);
    let fr = row.wrapping_add(u32::from(o.r8(0x15)) * 0xC).wrapping_add(10);
    adj16(env, fr, i32::from(o.r8(0x2C)));
    env.f59b5b0(0);
    if moved {
        env.f59b5b0(1);
    }
    env.f559810(set);
}

/// `0x569350` (tipo 27, peces).
pub fn spawn_t27(o: &mut SceneMem, a: &SpawnArgs, t: &Table, env: &mut dyn TypeEnv) {
    if a.variant == 0 {
        let r = env.rand();
        o.w8(0x39, (r % t.count) as u8);
    } else {
        o.w8(0x39, (a.variant as u8).wrapping_sub(1));
    }
    let r2 = env.rand() as i32;
    o.w8(0x3A, ((r2 % 3) as u8).wrapping_add(3));
    env.f55a290([0x14, 0, 0, 0]);
    let set = t.set(u32::from(o.r8(0x39)));
    let anim = env.f557730(set, 2);
    o.w8(0x15, anim);
    o.w32(0x24, 0);
    env.f559cf0(set);
    o.w32(0xC, a.x);
    o.w32(0x10, a.y);
    o.w16(0x16, a.link as u16);
    o.w8(0x14, a.heading as u8);
    o.w8(0x3B, 0);
    o.w8(0x3C, 1);
    env.f559370(set);
    env.f59b5b0(0);
    env.f59a260();
}

/// `0x569C20` (tipo 24, pescador): elige el puesto de pesca activo más cercano a `(x, y)` (en casillas).
pub fn spawn_t24(o: &mut SceneMem, a: &SpawnArgs, t: &Table, env: &mut dyn TypeEnv) {
    if a.variant == 0 {
        let r = env.rand();
        o.w8(0x2D, (r % t.count) as u8);
    } else {
        o.w8(0x2D, a.variant as u8);
    }
    o.w8(0x41, 0);
    let mut b: u8 = 0;
    let mut best: i32 = 0x7FFF_FFFF;
    let n = env.scene_r32(0xC668);
    let flags = env.scene_r32(0xC59C);
    let pts = env.scene_r32(0xC680);
    if n != 0 {
        loop {
            let u = u32::from(b);
            if env.rd8(flags.wrapping_add(u)) != 0 {
                let p = env.rd32(pts.wrapping_add(4).wrapping_add(u * 8));
                let dx = ((a.x >> 16) as i32).wrapping_sub(env.rd32(p) as i32);
                let dy = ((a.y >> 16) as i32).wrapping_sub(env.rd32(p.wrapping_add(4)) as i32);
                let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
                if d < best {
                    o.w8(0x41, b);
                    best = d;
                }
            }
            b = b.wrapping_add(1);
            if u32::from(b) >= n {
                break;
            }
        }
    }
    let set = t.set(u32::from(o.r8(0x2D)));
    let anim = env.f557730(set, 2);
    o.w8(0x15, anim);
    o.w32(0x24, 0);
    env.f559cf0(set);
    o.w16(0x16, a.link as u16);
    let k = u32::from(o.r8(0x41));
    let p = env.rd32(pts.wrapping_add(4).wrapping_add(k * 8));
    o.w32(0xC, env.rd32(p) << 16);
    o.w8(0x38, 0);
    o.w32(0x10, env.rd32(p.wrapping_add(4)) << 16);
    o.w8(0x39, 1);
    env.f56a240(o);
    let h = o.r8(0x3A);
    o.w8(0x14, h);
    let moved = env.f559260(set);
    env.f59b5b0(0);
    if moved {
        env.f59b5b0(1);
    }
    env.f559810(set);
    env.wr8(flags.wrapping_add(u32::from(o.r8(0x41))), 0);
}

/// Tipo 20 (`0x005803B0`, nubes): elige al azar un juego de la tabla cuyo tipo (`set+4`) encaja con el nivel `lvl_ptr+0x16C`,
/// reparte el centro y la desviación y registra el objeto. `lvl_ptr` es el puntero global `0x006CBDF0`.
pub fn spawn_t20(o: &mut SceneMem, a: &SpawnArgs, t: &Table, lvl_ptr: u32, env: &mut dyn TypeEnv) {
    let mut found: u8 = 0;
    o.w8(0x20, 0);
    let lvl = env.rd8(lvl_ptr.wrapping_add(0x16C)).wrapping_add(1);
    let mut i: u8 = 0;
    while u32::from(i) < t.count {
        let set = t.set(u32::from(i));
        let kind = env.rd32(set.wrapping_add(4));
        let ok = match kind {
            1 => lvl == 1,
            2 => lvl == 2,
            3 => lvl > 1,
            _ => false,
        };
        if ok {
            if found == 0 || (env.rand() as i32) % i32::from(found) == 0 {
                o.w8(0x20, i);
            }
            found = found.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let v = a.variant as i32;
    o.w32(0x2C, (v as i16) as i32 as u32);
    o.w32(0x30, (v.wrapping_add((v >> 31) & 0xFFFF) >> 16) as u32);
    let r = (env.rand() & 0x3F) as i32;
    o.w32(0x24, (r - 0x20).wrapping_add(o.r32(0x2C) as i32) as u32);
    let r2 = (env.rand() as i32) % 0x2C;
    o.w32(0x28, (r2.wrapping_add(o.r32(0x30) as i32) - 0x16) as u32);
    let set = t.set(u32::from(o.r8(0x20)));
    o.w16(0x16, a.link as u16);
    o.w32(0xC, a.x << 16);
    o.w32(0x10, a.y << 16);
    o.w8(0x14, a.heading as u8);
    env.f559260(set);
    env.f59b5b0(0);
}

/// Tipos 21 (`0x00564810`, lado de ariete/almacén) y 22 (`0x005740F0`): eligen un juego, se colocan sobre el último nodo del camino
/// `.nodes` y recorren cuatro pasos con `0x58B740`. `ty22` selecciona la variante de desplazamientos del tipo 22.
pub fn spawn_rail(o: &mut SceneMem, a: &SpawnArgs, t: &Table, ty22: bool, env: &mut dyn TypeEnv) {
    let (hi, lo, pick, st_a, st_b, bytes, size) = if ty22 { (0x48usize, 0x49usize, 0x35usize, 0x40usize, 0x41usize, 0x42usize, 0x10u32) } else { (0x44, 0x45, 0x31, 0x3C, 0x3D, 0x3E, 8) };
    let r = env.rand();
    o.w8(lo, a.variant as u8);
    o.w8(hi, (a.variant >> 8) as u8);
    let idx = r % t.count;
    o.w8(pick, idx as u8);
    let set = t.set(idx & 0xFF);
    let anim = env.f557730(set, 2);
    o.w8(0x15, anim);
    o.w32(0x24, 0);
    env.f559cf0(set);
    o.w16(0x16, a.link as u16);
    let i = u32::from(o.r8(hi)) * 8;
    let tab_b = env.scene_r32(0xC678);
    let tab_a = env.scene_r32(0xC674);
    let cnt = env.rd8(tab_b.wrapping_add(i));
    let q = env.rd32(tab_b.wrapping_add(i).wrapping_add(4));
    let j = u32::from(cnt.wrapping_sub(1)) * 8;
    let pn = env.rd32(tab_a.wrapping_add(i).wrapping_add(4));
    let px = env.rd32(pn);
    let py = env.rd32(pn.wrapping_add(4));
    let qx = env.rd32(q.wrapping_add(j));
    let qy = env.rd32(q.wrapping_add(j).wrapping_add(4));
    o.w32(0xC, px.wrapping_add(qx) << 15);
    o.w8(st_a, 0);
    o.w8(st_b, 1);
    o.w32(0x10, qy.wrapping_add(0x16).wrapping_add(py) << 15);
    let (wa, wb) = (u32::from(o.r16(0xE)), u32::from(o.r16(0x12)));
    env.f_place(u32::from(ty22), o, wa, wb);
    o.w8(0x14, o.r8(bytes));
    let moved = env.f559260(set);
    env.f59b5b0(0);
    if moved {
        env.f59b5b0(1);
    }
    if ty22 {
        env.f559810(set);
    } else {
        env.f559790(set);
    }
    let (mut ra, mut rb) = env.f58d030(i32::from(o.r16(0xE) as i16), i32::from(o.r16(0x12) as i16));
    o.w32(0x50, ra);
    o.w32(0x54, rb);
    let side = o.r8(lo);
    let (d_f, d_a) = (env.rd8(0x0067_792F), env.rd8(0x0067_792A));
    env.f630cc0(side, ra, rb, size);
    for d in [d_f, d_a, d_f] {
        env.f58b740(&mut ra, &mut rb, d);
        env.f630cc0(side, ra, rb, size);
    }
    let w = 0x3AB20u32 / env.rd32(set.wrapping_add(0xC));
    if ty22 {
        o.w16(0x4A, 0);
        o.w32(0x30, 0);
        o.w16(0x4C, w as u16);
        o.w8(0x58, 0);
        o.w16(0x2E, 0xFFFF);
        o.w32(0x5C, 0xFFFF_FFFF);
        o.w32(0x60, 0);
        o.w32(0x64, 0);
    } else {
        o.w16(0x4C, 0);
        o.w16(0x4E, w as u16);
        o.w32(0x60, 0);
        o.w8(0x6A, 0);
        o.w16(0x2E, 0xFFFF);
        o.w32(0x6C, 0xFFFF_FFFF);
        o.w32(0x70, 0);
        o.w32(0x74, 0);
    }
}

/// Tipo 32 (`0x00570F70`, soldado de escalera): elige un juego al azar, crea el soldado con `0x55E620` (clase 4) y, con el bit 8 de la variante, repasa la animación.
pub fn spawn_t32(o: &mut SceneMem, a: &SpawnArgs, t: &Table, env: &mut dyn TypeEnv) {
    let rem = env.rand() % t.count;
    o.w32(0x88, 0);
    o.w8(0x90, ((a.variant >> 7) & 1) as u8);
    o.w32(0x8C, 0);
    o.w8(0x91, rem as u8);
    let set = t.set(rem & 0xFF);
    env.f55e620([a.x, a.y, a.heading, set, a.link, (rem & !0xFF) | (a.variant & 0x7F), 4, 0, 0, 1]);
    o.w16(0x82, 0);
    o.w16(0x84, 0);
    if a.variant & 0x100 != 0 {
        env.f55e250(set);
        let b = env.rec_byte(o.r8(0x6A), 0x5D2);
        o.w8(0x3C, b);
        o.w8(0x14, b);
        let anim = env.f557730_2c(set, 0x18);
        o.w8(0x15, anim);
        env.f571da0();
        env.f55e430(set);
    }
}

/// Tipo 23 (`0x00576990`, proyectil con destino): elige una fila de la tabla (`tbl`, 12 bytes por fila) cuyo código encaja con `heading & 0xFF`,
/// calcula distancia y octante hacia el destino devuelto por `0x58CFB0` y desplaza tres campos del cuadro de animación antes y después de `0x559810`.
pub fn spawn_t23(o: &mut SceneMem, a: &SpawnArgs, t: &Table, tbl: u32, env: &mut dyn TypeEnv) {
    o.w8(0x36, 0);
    let mut found: u8 = 0;
    let mut i: u8 = 0;
    while u32::from(i) < t.count {
        if env.rd32(tbl.wrapping_add(u32::from(i) * 12)) == (a.heading & 0xFF) {
            if found == 0 || (env.rand() as i32) % (i32::from(found) + 1) == 0 {
                o.w8(0x36, i);
            }
            found = found.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    o.w16(0x38, (a.x >> 16) as u16);
    o.w16(0x3A, (a.y >> 16) as u16);
    let (out1, out2) = env.f58cfb0((a.variant >> 24) as u32, (a.variant >> 16) & 0xFF);
    o.w16(0x3C, out1 as u16);
    let dx = (out1 & 0xFFFF) as i32 - i32::from(o.r16(0x38));
    o.w16(0x3E, out2 as u16);
    let dy = ((((out2 & 0xFFFF) as i32) - i32::from(o.r16(0x3A))).wrapping_mul(0x22)) / 0x16;
    let d2 = f64::from(dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)));
    let mut len = crate::crack::ftol(d2.sqrt()) as u16;
    if len == 0 {
        len = 1;
    }
    o.w16(0x34, len);
    o.w32(0x30, 0);
    o.w8(0x40, (a.variant >> 8) as u8);
    o.w8(0x41, a.variant as u8);
    let e = tbl.wrapping_add(u32::from(o.r8(0x36)) * 12);
    let r = env.rand();
    let m = env.rd32(e.wrapping_add(8));
    o.w8(0x2E, ((r % m) as u8).wrapping_add(env.rd8(e.wrapping_add(4))));
    let k = crate::crack::ftol(f64::from(len).sqrt());
    o.w8(0x2E, (k.wrapping_mul(i32::from(o.r8(0x2E))) >> 5) as u8);
    let set = t.set(u32::from(o.r8(0x36)));
    let anim = env.f557730(set, 0);
    o.w8(0x15, anim);
    o.w32(0x24, 0);
    env.f559cf0(set);
    o.w16(0x16, a.link as u16);
    o.w32(0xC, a.x);
    o.w32(0x10, a.y);
    let g = |env: &mut dyn TypeEnv, k: u32| env.rd8(0x0067_7928 + k);
    let (ay, ax) = (dy.wrapping_abs(), dx.wrapping_abs());
    let dir = if ax.wrapping_mul(12) < ay.wrapping_mul(5) {
        if dy < 1 { g(env, 0) } else { g(env, 4) }
    } else if ay.wrapping_mul(12) < ax.wrapping_mul(5) {
        if dx < 1 { g(env, 6) } else { g(env, 2) }
    } else if dy < 1 {
        if dx < 1 { g(env, 7) } else { g(env, 1) }
    } else if dx < 1 {
        g(env, 5)
    } else {
        g(env, 3)
    };
    o.w8(0x14, dir);
    let cnt = env.rd32(set.wrapping_add(0x14));
    let sel = ((u32::from(dir) * cnt + 0x80) >> 8) & cnt.wrapping_sub(1);
    let arr = env.rd32(set.wrapping_add(0x18));
    let b = u32::from(o.r8(0x2D));
    let frame = |env: &mut dyn TypeEnv, o: &SceneMem| env.rd32(arr.wrapping_add(sel * 4)).wrapping_add(u32::from(o.r8(0x15)) * 12);
    let f = frame(env, o);
    let add = |env: &mut dyn TypeEnv, at: u32, d: u16| {
        let v = env.rd16(at).wrapping_add(d);
        env.wr16(at, v);
    };
    add(env, f + 6, b as u16);
    add(env, f + 4, (b / 5) as u16);
    add(env, f + 10, (b as u16).wrapping_neg());
    env.f559810(set);
    let f = frame(env, o);
    add(env, f + 6, (b as u16).wrapping_neg());
    let l = i64::from(b as i32) * -0x6666_6667;
    add(env, f + 4, ((l >> 0x21) as i16).wrapping_sub((l >> 0x3F) as i16) as u16);
    add(env, f + 10, b as u16);
}

/// Tipo 7 (`0x00562C10`, funcionario): elige un juego al azar y se coloca en el primer punto de la lista `escena+0xC67C`.
pub fn spawn_t7(o: &mut SceneMem, a: &SpawnArgs, t: &Table, env: &mut dyn TypeEnv) {
    let r = env.rand();
    o.w8(0x41, a.variant as u8);
    let idx = r % t.count;
    o.w8(0x2D, idx as u8);
    let set = t.set(idx & 0xFF);
    let anim = env.f557730(set, 2);
    o.w8(0x15, anim);
    o.w32(0x24, 0);
    env.f559cf0(set);
    o.w16(0x16, a.link as u16);
    let tb = env.scene_r32(0xC67C);
    let p = env.rd32(tb.wrapping_add(4));
    o.w32(0xC, env.rd32(p) << 16);
    let y = env.rd32(p.wrapping_add(4));
    o.w8(0x38, 0);
    o.w32(0x10, y << 16);
    o.w8(0x39, 1);
    env.f563090(o);
    o.w8(0x14, o.r8(0x3A));
    let moved = env.f559260(set);
    env.f59b5b0(0);
    if moved {
        env.f59b5b0(1);
    }
    env.f559810(set);
}

/// Tipo 18 (`0x0057A150`, efectos: humo, fuego, polvo y el humo del puerto): busca entre los juegos cuyo código (`bytes[i]`) coincide con
/// `variante & 0xFF` (y, en los códigos con subtipo, cuyo `set+4` coincide con `variante>>8`), elige uno por muestreo de depósito y lo dibuja.
pub fn spawn_t18(o: &mut SceneMem, a: &SpawnArgs, t: &Table, bytes: u32, env: &mut dyn TypeEnv) {
    let v = a.variant;
    let code = v as u8;
    let key = if matches!(code, 4 | 5 | 6 | 7 | 0x40 | 0x41 | 0x42 | 0x43 | 0x50 | 0x80 | 0xB0 | 199) { (v >> 8) & 0xFF } else { 0 };
    o.w8(0x2C, 0);
    let mut b6: u8 = 0;
    let mut b1: u8 = 0;
    let n = t.count;
    if n != 0 {
        loop {
            let idx = u32::from(b6);
            if env.rd8(bytes.wrapping_add(idx)) == code {
                let skip = if key as u8 != 0 {
                    let s = t.set(idx);
                    key != env.rd32(s.wrapping_add(4))
                } else {
                    false
                };
                if !skip {
                    b1 = b1.wrapping_add(1);
                    let r = env.rand() as i32;
                    if r % i32::from(b1) == 0 {
                        o.w8(0x2C, b6);
                        if env.scene_r8(0xC594) == 0 {
                            b6 = (n as u8).wrapping_sub(1);
                        }
                    }
                }
            }
            b6 = b6.wrapping_add(1);
            if u32::from(b6) >= n {
                break;
            }
        }
    }
    let sel = u32::from(o.r8(0x2C));
    let set = t.set(sel);
    let anim = env.f557730(set, 2);
    o.w8(0x15, anim);
    let d10 = env.rd32(set.wrapping_add(0x10));
    o.w32(0x24, d10.wrapping_add(env.g(2, 0)));
    env.f559cf0(set);
    o.w16(0x16, a.link as u16);
    o.w32(0xC, a.x);
    o.w32(0x10, a.y);
    o.w8(0x14, a.heading as u8);
    if matches!(code, 0x24 | 0x30 | 0x11 | 0xB0) {
        o.w32(0x10, a.y.wrapping_add(0x30_0000));
    } else if code == 0x81 {
        o.w32(0x10, a.y.wrapping_add(0xB_0000));
    }
    if env.rd8(bytes.wrapping_add(sel)) & 0x20 == 0 {
        o.w16(0x2E, 0);
    } else {
        o.w16(0x2E, (v >> 16) as u16);
    }
    if env.scene_r8(0xC594) != 0 || !(0x40..=0x43).contains(&code) {
        env.f57af60(set, 1);
        if v & 0x40 == 0 {
            env.f559790(set);
        } else {
            env.f559370(set);
            env.f59b5b0(0);
            env.f59a260();
        }
        env.f57af60(set, 0);
        if code == 0 || code == 2 {
            let (x, y) = (o.r16(0xE), o.r16(0x12));
            env.f5986a0(x, y);
            env.f5986a0(x.wrapping_sub(1), y);
            env.f5986a0(x, y.wrapping_sub(1));
            env.f5986a0(x.wrapping_sub(1), y.wrapping_sub(1));
        }
    }
}

/// Tipo 25 (`0x0057D380`, puerta): coloca la puerta en el primer juego, localiza el nodo de camino (`escena+0xC674`, tres listas) a menos de 5 casillas
/// y, si la animación es la `0xEE` («cerrada»), la cambia a `0xCE` cuando no hay ninguna entidad ligada a esa puerta (`+0x68`).
/// Después desplaza 11 píxeles el cuadro para las orientaciones 0x20/0xE0 (hacia abajo) o 0x60/0xA0 (hacia arriba) mientras dibuja.
pub fn spawn_t25(o: &mut SceneMem, a: &SpawnArgs, t: &Table, env: &mut dyn TypeEnv) {
    let v = a.variant;
    o.w8(0x2C, 0);
    o.w8(0x2D, v as u8);
    o.w8(0x2E, u8::from((v >> 8) & 0xFF != 0));
    let set = t.set(0);
    env.f559cf0(set);
    let anim = env.f557730(set, if v & 0xFF_0000 != 0 { 0xCE } else { 0xEE });
    o.w8(0x15, anim);
    o.w16(0x16, a.link as u16);
    o.w32(0xC, a.x);
    o.w32(0x10, a.y);
    o.w8(0x14, a.heading as u8);
    o.w32(0x24, 0);
    let (ox, oy) = env.f58d030(i32::from(o.r16(0xE) as i16), i32::from(o.r16(0x12) as i16));
    let mut found: Option<u8> = None;
    'outer: for g in 0u8..3 {
        let tab = env.scene_r32(0xC674);
        let cnt = env.rd32(tab.wrapping_add(u32::from(g) * 8));
        if cnt == 0 {
            continue;
        }
        let mut k: u8 = 0;
        loop {
            let tab = env.scene_r32(0xC674);
            let ptr = env.rd32(tab.wrapping_add(u32::from(g) * 8).wrapping_add(4));
            let px = env.rd32(ptr.wrapping_add(u32::from(k) * 8));
            let py = env.rd32(ptr.wrapping_add(u32::from(k) * 8).wrapping_add(4));
            let (nx, ny) = env.f58d030(px as i32, py as i32);
            let dx = (ox.wrapping_sub(nx) as i32).wrapping_mul(2);
            let dy = oy.wrapping_sub(ny) as i32;
            if dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)) <= 0x19 {
                found = Some(g);
                break 'outer;
            }
            k = k.wrapping_add(1);
            let tab = env.scene_r32(0xC674);
            if u32::from(k) >= env.rd32(tab.wrapping_add(u32::from(g) * 8)) {
                break;
            }
        }
    }
    // Busca en las cuatro listas de puertas una entidad con `+0x68 == want`; devuelve si hay alguna.
    let search = |env: &mut dyn TypeEnv, want: u8| -> bool {
        for q in 0u32..4 {
            let c = u32::from(env.scene_r16(0xC5C0 + q * 2));
            if c == 0 {
                continue;
            }
            let start = u32::from(env.scene_r16(0xC5B8 + q * 2));
            let end = c + start;
            let mut i = start;
            if i < end {
                let arr = env.scene_r32(0xC5A0 + q * 4);
                loop {
                    let ent = env.rd32(arr.wrapping_add(i * 4));
                    if env.rd8(ent.wrapping_add(0x68)) == want {
                        return true;
                    }
                    i = (i + 1) & 0xFFFF;
                    if i >= end {
                        break;
                    }
                }
            }
        }
        false
    };
    match found {
        Some(g) => {
            o.w8(0x2F, g);
            if o.r8(0x28) == 0xEE && !search(env, g) {
                let an = env.f557730(set, 0xCE);
                o.w8(0x15, an);
            }
        }
        None => {
            o.w8(0x2F, 0);
            if o.r8(0x28) == 0xEE && !search(env, 0) {
                let an = env.f557730(set, 0xCE);
                o.w8(0x15, an);
            }
        }
    }
    let cnt = env.rd32(set.wrapping_add(0x14));
    let dir = u32::from(o.r8(0x14));
    let sel = (cnt.wrapping_mul(dir).wrapping_add(0x80) >> 8) & cnt.wrapping_sub(1);
    let arr = env.rd32(set.wrapping_add(0x18));
    let frame = env.rd32(arr.wrapping_add(sel * 4)).wrapping_add(u32::from(o.r8(0x15)) * 12);
    let step = |env: &mut dyn TypeEnv, o: &mut SceneMem, down: bool| {
        let d: u16 = 0xB;
        let (a, b): (u16, u16) = if down { (d, d.wrapping_neg()) } else { (d.wrapping_neg(), d) };
        o.w16(0x12, o.r16(0x12).wrapping_add(a));
        let v = env.rd16(frame + 10).wrapping_add(b);
        env.wr16(frame + 10, v);
        env.f559790(set);
        o.w16(0x12, o.r16(0x12).wrapping_add(b));
        let v = env.rd16(frame + 10).wrapping_add(a);
        env.wr16(frame + 10, v);
    };
    match dir {
        0x20 | 0xE0 => step(env, o, true),
        0x60 | 0xA0 => step(env, o, false),
        _ => {}
    }
}

/// Tipo 31 (`0x0056FB20`, cochero): elige juego al azar, escoge el camino de salida entre tres, calcula el tramo inicial recorrido y deja el objeto en la
/// cabeza de la lista global `[0x6E6280]`. `obj_addr` es la dirección del objeto (se guarda en la lista). Si una división entre cero (`idiv`) detendría el
/// original, la función termina sin más escrituras (no es un caso comparado).
pub fn spawn_t31(o: &mut SceneMem, a: &SpawnArgs, t: &Table, obj_addr: u32, env: &mut dyn TypeEnv) {
    if t.count >= 1 {
        let r = env.rand();
        o.w8(0x2C, (r % t.count) as u8);
    }
    let set = t.set(u32::from(o.r8(0x2C)));
    let tab = env.scene_r32(0xC674);
    let p0 = env.rd32(tab.wrapping_add(4));
    let p1 = env.rd32(tab.wrapping_add(0xC));
    let p2 = env.rd32(tab.wrapping_add(0x14));
    let pt = |env: &mut dyn TypeEnv, p: u32| (env.rd32(p) as i32, env.rd32(p.wrapping_add(4)) as i32);
    let (x0, y0) = pt(env, p0);
    let (x1, y1) = pt(env, p1);
    let (x2, y2) = pt(env, p2);
    let sq = |dx: i32, dy: i32| dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    let mut bst: u8 = 2;
    let d01 = sq(x0.wrapping_sub(x1), y0.wrapping_sub(y1));
    let d02 = sq(x0.wrapping_sub(x2), y0.wrapping_sub(y2));
    let mut best = d01;
    if d01 < d02 {
        bst = 1;
        best = d02;
    }
    if best < sq(x1.wrapping_sub(x2), y1.wrapping_sub(y2)) {
        bst = 0;
    }
    o.w32(0x64, a.variant);
    let ent = env.f4d8db0(a.variant);
    let eb = env.rd8(ent.wrapping_add(9));
    let c324 = env.scene_r32(0xC324);
    let eq = u32::from(eb) == c324;
    o.w8(0x3A, u8::from(eq));
    if eq {
        o.w8(0x3C, bst);
    } else {
        let ia = u32::from(env.rd8(0x006D_E4B8 + u32::from(eb)));
        let ib = u32::from(env.rd8(0x006D_E4B8u32.wrapping_add(c324)));
        let i10 = (env.rd32(0x006D_DB90 + ia * 52) as i32).wrapping_sub(env.rd32(0x006D_DB90 + ib * 52) as i32);
        let i9 = (env.rd32(0x006D_DB94 + ia * 52) as i32).wrapping_sub(env.rd32(0x006D_DB94 + ib * 52) as i32);
        let len = crate::crack::ftol(f64::from(sq(i10, i9)).sqrt());
        if len == 0 {
            return;
        }
        let mut best2: i32 = 0;
        for k in 0u8..3 {
            if k == bst {
                continue;
            }
            let tab = env.scene_r32(0xC674);
            let ptr = env.rd32(tab.wrapping_add(4).wrapping_add(u32::from(k) * 8));
            let w = env.scene_r32(0xC31C) >> 1;
            let h = env.scene_r32(0xC320) >> 1;
            let i12 = (env.rd32(ptr) as i32).wrapping_sub(w as i32);
            let i6 = (env.rd32(ptr.wrapping_add(4)) as i32).wrapping_sub(h as i32);
            let l2 = crate::crack::ftol(f64::from(sq(i12, i6)).sqrt());
            if l2 == 0 {
                return;
            }
            let div = |n: i32, d: i32| n.checked_div(d);
            let (Some(q1), Some(q2), Some(q3), Some(q4)) = (div(i12.wrapping_mul(256), l2), div(i10.wrapping_mul(256), len), div(i6.wrapping_mul(256), l2), div(i9.wrapping_mul(256), len)) else {
                return;
            };
            let sc = sq(q1.wrapping_add(q2), q3.wrapping_add(q4));
            if best2 < sc {
                o.w8(0x3C, k);
                best2 = sc;
            }
        }
    }
    let i7 = u32::from(o.r8(0x3C)) * 8;
    let tab = env.scene_r32(0xC674);
    let tab2 = env.scene_r32(0xC678);
    let c = env.rd8(tab.wrapping_add(i7));
    o.w8(0x3B, c);
    o.w8(0x39, if o.r8(0x3A) == 0 { c.wrapping_sub(1) } else { 0 });
    o.w32(0x48, 0);
    o.w32(0x40, 0);
    let qa = env.rd32(tab2.wrapping_add(4).wrapping_add(i7));
    let pa = env.rd32(tab.wrapping_add(4).wrapping_add(i7));
    let mut prev = (env.rd32(qa.wrapping_sub(8).wrapping_add(u32::from(c) * 8)) as i32).wrapping_add(env.rd32(pa) as i32) >> 1;
    let b8 = o.r8(0x3B);
    if b8 > 1 {
        let mut s: u8 = 1;
        loop {
            let i9 = u32::from(o.r8(0x3C)) * 8;
            let qa = env.rd32(tab2.wrapping_add(4).wrapping_add(i9));
            let pa = env.rd32(tab.wrapping_add(4).wrapping_add(i9));
            let v = (env.rd32(qa.wrapping_sub(8).wrapping_add(u32::from(b8.wrapping_sub(s)) * 8)) as i32).wrapping_add(env.rd32(pa.wrapping_add(u32::from(s) * 8)) as i32) >> 1;
            let d = v.wrapping_sub(prev).wrapping_abs();
            o.w32(0x40, o.r32(0x40).wrapping_add(d as u32));
            s = s.wrapping_add(1);
            prev = v;
            if s >= o.r8(0x3B) {
                break;
            }
        }
    }
    o.w8(0x4C, 0);
    let g = env.g(2, 0);
    o.w32(0x44, g);
    let b8 = u32::from(o.r8(0x39));
    let i7 = u32::from(o.r8(0x3C)) * 8;
    let qa = env.rd32(tab2.wrapping_add(4).wrapping_add(i7));
    let pa = env.rd32(tab.wrapping_add(4).wrapping_add(i7));
    let rel = u32::from(o.r8(0x3B)).wrapping_sub(b8).wrapping_mul(8);
    o.w32(0xC, env.rd32(qa.wrapping_sub(8).wrapping_add(rel)).wrapping_add(env.rd32(pa.wrapping_add(b8 * 8))) << 15);
    o.w32(0x10, env.rd32(qa.wrapping_sub(4).wrapping_add(rel)).wrapping_add(env.rd32(pa.wrapping_add(4).wrapping_add(b8 * 8))) << 15);
    o.w8(0x38, if o.r8(0x3A) == 0 { (b8 as u8).wrapping_sub(1) } else { (b8 as u8).wrapping_add(1) });
    env.f570800(o);
    o.w8(0x14, o.r8(0x58));
    let mut r = env.f5709c0(o);
    if r == o.r8(0x58) {
        loop {
            let b = o.r8(0x38);
            if b == 0 || u32::from(o.r8(0x3B)).wrapping_sub(1) == u32::from(b) {
                break;
            }
            o.w8(0x38, if o.r8(0x3A) == 0 { b.wrapping_sub(1) } else { b.wrapping_add(1) });
            r = env.f5709c0(o);
            if r != o.r8(0x58) {
                break;
            }
        }
    }
    let _ = r;
    let ents = env.rd32(set.wrapping_add(0x2C));
    for k in 0u8..3 {
        match env.rd32(ents.wrapping_add(8).wrapping_add(u32::from(k) * 12)) {
            2 => o.w8(0x5D, k),
            9 => o.w8(0x5C, k),
            10 => o.w8(0x5B, k),
            _ => {}
        }
    }
    o.w16(0x16, a.link as u16);
    env.f559cf0(set);
    let cnt = env.rd32(set.wrapping_add(0x14));
    let u = (u32::from(o.r8(0x14)).wrapping_mul(cnt).wrapping_add(0x80) >> 8) & cnt.wrapping_sub(1);
    let ar18 = env.rd32(set.wrapping_add(0x18));
    let ar1c = env.rd32(set.wrapping_add(0x1C));
    let off = u32::from(o.r8(0x15)) * 12;
    let f18 = env.rd32(ar18.wrapping_add(u * 4)).wrapping_add(off).wrapping_add(10);
    let f1c = env.rd32(ar1c.wrapping_add(u * 4)).wrapping_add(off).wrapping_add(10);
    let add = |env: &mut dyn TypeEnv, at: u32, d: u16| {
        let v = env.rd16(at).wrapping_add(d);
        env.wr16(at, v);
    };
    add(env, f18, 0x21);
    add(env, f1c, 0x21);
    o.w16(0x12, o.r16(0x12).wrapping_sub(0x21));
    env.f559790(set);
    let f18 = env.rd32(ar18.wrapping_add(u * 4)).wrapping_add(u32::from(o.r8(0x15)) * 12).wrapping_add(10);
    add(env, f18, 0x21u16.wrapping_neg());
    let f1c = env.rd32(ar1c.wrapping_add(u * 4)).wrapping_add(u32::from(o.r8(0x15)) * 12).wrapping_add(10);
    add(env, f1c, 0x21u16.wrapping_neg());
    o.w16(0x12, o.r16(0x12).wrapping_add(0x21));
    let head = env.rd32(0x006E_6280);
    o.w32(0x60, head);
    env.wr32(0x006E_6280, obj_addr);
    o.w8(0x5A, 0);
    o.w8(0x5E, 0);
}

/// Tipo 34 (`0x0057FC60`, señal de aviso): coloca el objeto, fija su contador y lo añade al vector global de señales (`0x6E2F14`).
pub fn spawn_t34(o: &mut SceneMem, a: &SpawnArgs, obj_addr: u32, env: &mut dyn TypeEnv) {
    o.w16(0x16, a.link as u16);
    o.w8(0x14, a.heading as u8);
    o.w32(0xC, a.x << 16);
    o.w32(0x10, a.y << 16);
    o.w32(0x20, a.variant & 0xFFFF);
    o.w32(0x24, a.variant >> 16);
    let r = env.rand();
    let m = env.rd32(0x006E_2EE0);
    let g = env.g(2, 0);
    o.w32(0x2C, (r % m).wrapping_add(g));
    env.f559130(0, 0x006E_2ED0);
    env.registry_push(obj_addr);
}
