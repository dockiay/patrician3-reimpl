//! Constructor de escena `0x589360`: ventanas auxiliares, cargas de `files.lst` (objetos de la escena y objetos de entorno) y objetos finales.
//! Traducción 1:1 del flujo del x86. Las funciones de cadenas, iteradores, fábrica y los métodos virtuales se reciben por [`InitEnv`];
//! la memoria de la escena es plana ([`SceneMem`]). Los dos recorridos de lista tienen dos ramas: el fichero `files.lst` mapeado (nombres
//! terminados en cero) o, si no existe, el iterador del directorio.
use crate::ambience::SceneMem;
use crate::scene_order::order_objects;

pub struct Globals {
    /// `[0x6DCCE0]` y `[0x6DCCE4]`: tamaño de la ventana principal.
    pub width: u32,
    pub height: u32,
}

pub trait InitEnv {
    fn ini(&mut self);
    /// Virtual `+8` de la escena.
    fn vcall8(&mut self) -> u8;
    fn alloc(&mut self, size: u32) -> u32;
    fn ctor_window(&mut self, o: u32);
    /// `0x4B4E30` con `ecx = [0x6CBB40]`.
    fn reg_window(&mut self, o: u32);
    /// Virtual `+0x68(&{x, y, z})` de una ventana.
    fn v68(&mut self, w: u32, x: i32, y: i32, z: i32);
    /// `0x4A63C0(ventana, bandera)`
    fn setflag(&mut self, w: u32, v: u32);
    fn f4bbb00(&mut self, a: u32, b: u32, c: u32);
    fn f4bb520(&mut self) -> u32;
    /// `arc_CreateMemMapped("files.lst")`: `None` si el fichero no existe.
    fn map_file(&mut self, which: u32) -> Option<Vec<u8>>;
    fn unmap(&mut self, which: u32);
    fn match_name(&mut self, name: &[u8]) -> bool;
    fn p551300(&mut self) -> u32;
    fn p5512d0(&mut self) -> u32;
    fn f54f7e0(&mut self, a: u32, b: u32, c: u32) -> u32;
    fn f586d90(&mut self, h: u32, idx: u32);
    fn v34(&mut self, idx: u32, name: &[u8]) -> bool;
    fn f586e50(&mut self, idx: u32, flag: u32);
    fn build_iter(&mut self, which: u32);
    fn iter_open(&mut self) -> bool;
    fn iter_next(&mut self) -> bool;
    fn iter_get(&mut self) -> Vec<u8>;
    fn iter_close(&mut self);
    fn f586e40(&mut self);
    fn list_get(&mut self, n: usize) -> Vec<u16>;
    fn list_set(&mut self, list: &[u16]);
    fn obj_key(&mut self, h: u16) -> u8;
    fn p558d00(&mut self) -> u32;
    fn make(&mut self, ty: u32) -> u32;
    fn f568c40(&mut self, o: u32);
    fn v10(&mut self, o: u32, name: &[u8]);
    fn set_ratio(&mut self, r: f32);
    fn f59da40(&mut self, a: u32, b: u32);
    fn ctor_48(&mut self, o: u32);
    fn ctor_0c(&mut self, o: u32);
    fn v0c(&mut self, o: u32, a: u32, b: u32, c: u32, d: u32);
    fn ctor_1c(&mut self, o: u32);
}

/// Un recorrido de `files.lst`: `which` 1 = objetos del mapa (`escena+0x4B8`/`+0x4CC`), 2 = objetos de entorno (`+0x4BC`/`+0x4C0`).
fn walk_list(m: &mut SceneMem, env: &mut dyn InitEnv, which: u32, mut each: impl FnMut(&mut SceneMem, &mut dyn InitEnv, &[u8], bool)) {
    match env.map_file(which) {
        Some(buf) => {
            let mut at = 0usize;
            let mut guard = 0;
            while at < buf.len() {
                let end = buf[at..].iter().position(|&b| b == 0).map_or(buf.len(), |p| at + p);
                let name = buf[at..end].to_vec();
                let before = at;
                each(m, env, &name, false);
                // El original avanza solo cuando el nombre se acepta; el avance lo decide `each` con `accepted`.
                at = end + 1;
                guard += 1;
                if at <= before || guard > 100_000 {
                    break;
                }
            }
            env.unmap(which);
        }
        None => {
            if which == 1 {
                m.w32(0x9C, 0);
            }
            env.build_iter(which);
            if env.iter_open() {
                loop {
                    let more = env.iter_next();
                    let name = env.iter_get();
                    each(m, env, &name, true);
                    if !more {
                        break;
                    }
                }
            }
            env.iter_close();
        }
    }
}

/// Devuelve `virtual +8 & 1`. El `AL` real al salir NO es ese valor: `0x586E40` retira 8 bytes del marco (`ret 8`) y el epílogo lee `[esp+0x37]`
/// desplazado, así que es un residuo de pila que ningún llamador debe usar; no se compara con el oráculo.
pub fn init_scene(m: &mut SceneMem, g: &Globals, env: &mut dyn InitEnv) -> u8 {
    env.ini();
    let flag = env.vcall8() & 1;
    for k in 0..8 {
        let p = env.alloc(0x98);
        let o = if p != 0 {
            env.ctor_window(p);
            p
        } else {
            0
        };
        m.w32(0xC608 + 4 * k, o);
        env.reg_window(o);
    }
    if m.r32(0xC608) != 0 {
        let w0 = m.r32(0xC608);
        env.v68(w0, g.width.wrapping_sub(0x14) as i32, 0x1E, 0);
        let mut x = g.width.wrapping_sub(0x34) as i32;
        for k in 0..6 {
            let w = m.r32(0xC60C + 4 * k);
            env.v68(w, x, 0x1E, 0);
            x = x.wrapping_sub(0x27);
        }
        let w7 = m.r32(0xC624);
        env.v68(w7, x.wrapping_add(0x27), 0x1E, 0);
    }
    for (off, v) in [(0xC608, 0x800u32), (0xC60C, 0x8000), (0xC610, 8), (0xC614, 0x10), (0xC618, 0x1000), (0xC61C, 0x400_0000), (0xC620, 0x400), (0xC624, 0x200)] {
        let w = m.r32(off);
        env.setflag(w, v);
    }
    let a = m.r32(0xC6EC);
    env.f4bbb00(a, 0xC6D8, 1);
    let v = env.f4bb520();
    m.w32(0x4FC, v);
    // Lista 1: objetos del mapa.
    walk_list(m, env, 1, |m, env, name, iter| {
        if env.match_name(name) {
            let idx = env.p551300();
            let id2 = env.p5512d0();
            let h = env.f54f7e0(id2, 0, 1);
            env.f586d90(h, idx);
            // Con el iterador el nombre se vuelve a pedir antes de cargar el objeto.
            let name = if iter { env.iter_get() } else { name.to_vec() };
            if !env.v34(idx & 0xFFFF, &name) {
                env.f586e50(idx, 1);
            }
        }
        let _ = m;
    });
    env.f586e40();
    let n = m.r32(0x104) as usize;
    let mut list = env.list_get(n);
    let keys: Vec<(u16, u8)> = list.iter().map(|&h| (h, env.obj_key(h))).collect();
    let key = |h: u16| keys.iter().find(|(x, _)| *x == h).map_or(0, |(_, k)| *k);
    let mut hist = m.0[0x574..0x684].to_vec();
    order_objects(&mut list, &key, &mut hist);
    m.0[0x574..0x684].copy_from_slice(&hist);
    env.list_set(&list);
    // Lista 2: objetos de entorno.
    walk_list(m, env, 2, |_, env, name, iter| {
        if env.match_name(name) {
            let ty = env.p558d00();
            let o = env.make(ty);
            env.f568c40(o);
            let name = if iter { env.iter_get() } else { name.to_vec() };
            env.v10(o, &name);
        }
    });
    let ratio = (m.r32(0x514).wrapping_add(1) as i32 as f32) / (m.r32(0x510) as i32 as f32);
    env.set_ratio(ratio);
    if m.r8(0x524) != 0 {
        env.f59da40(g.width, g.height);
    }
    let big = m.r8(0xC328) != 0;
    let p = env.alloc(if big { 0x48 } else { 0xC });
    let o = if p != 0 {
        if big {
            env.ctor_48(p);
        } else {
            env.ctor_0c(p);
        }
        p
    } else {
        0
    };
    m.w32(0x530, o);
    env.v0c(o, m.r32(0x2C), m.r32(0x30), m.r32(0x14), m.r32(0x18));
    for k in 0..4 {
        let p = env.alloc(0x1C);
        let o = if p != 0 {
            env.ctor_1c(p);
            p
        } else {
            0
        };
        m.w32(0x418 + 0x24 * k, o);
    }
    flag
}
