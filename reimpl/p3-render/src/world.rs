//! Objetos de la escena con sus enlaces: fábrica, registro de enlaces (`0x568A70`, `0x568B20`) y los métodos virtuales de nieve (29) y lluvia (33) que usa el
//! reparto de precipitación `0x58AD30` (`precip::fill_rect`). `World` implementa [`PrecipEnv`] con objetos reales (bytes del constructor) en lugar de ganchos;
//! `oracle/x86_weather_world.py` ejecuta el x86 completo (fábrica, registro, virtuales y conjuntos reales) y compara el estado final.
//!
//! La invalidación de pantalla de los virtuales `+0x1C` (`0x559130`, `0x575D10`) no existe aquí: el renderizador repinta la escena entera.

use crate::ambience::SceneMem;
use crate::factory::{self, CRACK, RAIN, SNOW};
use crate::precip::{PrecipEnv, SpriteInfo};
use crate::crack::CrackEnv;
use crate::spawn_types::{spawn_t29, spawn_t33, spawn_t34, SpawnArgs, Table, TypeEnv};
use p3_assets::lcr::Lcr;
use std::collections::BTreeMap;

/// Registro de enlaces (`escena+0x3F8`): `table` es la tabla (`+4`, capacidad `+0x14` = longitud) y `max` el mayor enlace usado (`+0xC`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Registry {
    pub table: Vec<u32>,
    pub max: u32,
}

impl Registry {
    /// `0x568A70(inicio)`: primer enlace libre desde `inicio`; si no hay, `1` (tabla vacía) o la capacidad.
    pub fn find_free(&self, mut start: u32) -> u32 {
        let cap = self.table.len() as u32;
        while start < cap {
            if self.table[start as usize] == 0 {
                return start;
            }
            start += 1;
        }
        if cap == 0 {
            1
        } else {
            cap & 0xFFFF
        }
    }

    /// `0x568B20(objeto, enlace)`: asigna el objeto al enlace, ampliando la tabla a `enlace + 100` entradas si no cabe.
    pub fn register(&mut self, obj: u32, link: u32) {
        let link = link & 0xFFFF;
        if self.table.len() as u32 <= link {
            let mut t = vec![0u32; (link + 100) as usize];
            if self.max != 0 {
                t[..=self.max as usize].copy_from_slice(&self.table[..=self.max as usize]);
            }
            self.table = t;
        }
        if self.max < link {
            self.max = link;
        }
        self.table[link as usize] = obj;
    }
}

pub struct Object {
    pub ty: u32,
    pub mem: SceneMem,
}

/// Objetos de la escena y sus tablas de clase.
pub struct World {
    pub objects: BTreeMap<u32, Object>,
    next: u32,
    pub registry: Registry,
    /// Conjuntos de la clase 29 (nieve), en el orden en que se cargaron.
    pub snow_sets: Vec<Lcr>,
    /// `[0x67792B]`: rumbo inicial de los objetos nuevos (dato del ejecutable).
    pub dir_byte: u8,
    pub rand: Box<dyn FnMut() -> i32>,
    /// Vector global de señales de aviso `[0x6E2F14..0x6E2F18]` (grietas): asas en el orden en que se añadieron.
    pub cracks: Vec<u32>,
    /// `[0x6DCCF8]`: reloj de la escena (ms).
    pub clock: u32,
    /// `[0x6E2EE0]`: intervalo (ms) del conjunto de la clase 34 (`0x6E2ED0 + 0x10`).
    pub crack_interval: u32,
    /// Proyección de la escena `(+0x510, +0x514, +0x4D0, +0x4D4)`; la usa `0x58CFB0`. `None` hasta que se configure.
    pub projection: Option<[i32; 4]>,
}

impl World {
    pub fn new(dir_byte: u8, rand: Box<dyn FnMut() -> i32>) -> Self {
        World { objects: BTreeMap::new(), next: 0x1000_0000, registry: Registry::default(), snow_sets: vec![], dir_byte, rand, cracks: vec![], clock: 0, crack_interval: 0, projection: None }
    }

    /// `0x557D10(tipo)`: crea el objeto y devuelve su asa.
    pub fn create(&mut self, ty: u32) -> u32 {
        let bytes = factory::construct(ty, self.dir_byte).unwrap_or_else(|| panic!("tipo {ty} sin constructor reproducido"));
        let h = self.next;
        self.next += 0x100;
        self.objects.insert(h, Object { ty, mem: SceneMem(bytes) });
        h
    }

    fn obj(&mut self, h: u32) -> &mut Object {
        self.objects.get_mut(&h).expect("objeto desconocido")
    }
}

struct SnowTypes<'a> {
    projection: Option<[i32; 4]>,
    rand: &'a mut dyn FnMut() -> i32,
    first_frames: &'a [u32],
}

impl TypeEnv for SnowTypes<'_> {
    fn rand(&mut self) -> u32 {
        (self.rand)() as u32
    }
    fn f557730(&mut self, set: u32, n: u32) -> u8 {
        unreachable!("f557730: no se usa en nieve ni lluvia")
    }
    fn f559cf0(&mut self, set: u32) {
        unreachable!("f559cf0: no se usa en nieve ni lluvia")
    }
    fn f559260(&mut self, set: u32) -> bool {
        unreachable!("f559260: no se usa en nieve ni lluvia")
    }
    fn f59b5b0(&mut self, which: u32) {
        unreachable!("f59b5b0: no se usa en nieve ni lluvia")
    }
    fn f559810(&mut self, set: u32) {
        unreachable!("f559810: no se usa en nieve ni lluvia")
    }
    fn f55e620(&mut self, args: [u32; 10]) {
        unreachable!("f55e620: no se usa en nieve ni lluvia")
    }
    fn rec_byte(&mut self, side: u8, off: u32) -> u8 {
        unreachable!("rec_byte: no se usa en nieve ni lluvia")
    }
    fn rec_word(&mut self, side: u8, off: u32) -> u16 {
        unreachable!("rec_word: no se usa en nieve ni lluvia")
    }
    fn f55a290(&mut self, a: [u32; 4]) {
        unreachable!("f55a290: no se usa en nieve ni lluvia")
    }
    fn f559130(&mut self, a: u32, set: u32) {
        unreachable!("f559130: no se usa en nieve ni lluvia")
    }
    fn bump_c58c(&mut self) {
        unreachable!("bump_c58c: no se usa en nieve ni lluvia")
    }
    fn v1c(&mut self) {
        unreachable!("v1c: no se usa en nieve ni lluvia")
    }
    fn g(&mut self, which: u32, arg: u32) -> u32 {
        assert_eq!(which, 4, "solo la nieve usa constantes");
        self.first_frames[arg as usize - 1]
    }
    fn shift_w(&mut self, d: i32) {
        unreachable!("shift_w: no se usa en nieve ni lluvia")
    }
    fn rd8(&mut self, a: u32) -> u8 {
        unreachable!("rd8: no se usa en nieve ni lluvia")
    }
    fn rd16(&mut self, a: u32) -> u16 {
        unreachable!("rd16: no se usa en nieve ni lluvia")
    }
    fn rd32(&mut self, a: u32) -> u32 {
        unreachable!("rd32: no se usa en nieve ni lluvia")
    }
    fn wr8(&mut self, a: u32, v: u8) {
        unreachable!("wr8: no se usa en nieve ni lluvia")
    }
    fn wr16(&mut self, a: u32, v: u16) {
        unreachable!("wr16: no se usa en nieve ni lluvia")
    }
    fn wr32(&mut self, a: u32, v: u32) {
        unreachable!("wr32: no se usa en nieve ni lluvia")
    }
    fn scene_r32(&mut self, off: u32) -> u32 {
        unreachable!("scene_r32: no se usa en nieve ni lluvia")
    }
    fn scene_r8(&mut self, off: u32) -> u8 {
        unreachable!("scene_r8: no se usa en nieve ni lluvia")
    }
    fn scene_r16(&mut self, off: u32) -> u16 {
        unreachable!("scene_r16: no se usa en nieve ni lluvia")
    }
    fn f559370(&mut self, set: u32) {
        unreachable!("f559370: no se usa en nieve ni lluvia")
    }
    fn f59a260(&mut self) {
        unreachable!("f59a260: no se usa en nieve ni lluvia")
    }
    fn f_place(&mut self, which: u32, o: &mut SceneMem, a: u32, b: u32) {
        unreachable!("f_place: no se usa en nieve ni lluvia")
    }
    fn f557730_2c(&mut self, set: u32, n: u32) -> u8 {
        unreachable!("f557730_2c: no se usa en nieve ni lluvia")
    }
    fn f55e250(&mut self, set: u32) {
        unreachable!("f55e250: no se usa en nieve ni lluvia")
    }
    fn f55e430(&mut self, set: u32) {
        unreachable!("f55e430: no se usa en nieve ni lluvia")
    }
    fn f571da0(&mut self) {
        unreachable!("f571da0: no se usa en nieve ni lluvia")
    }
    /// `0x58CFB0` con la proyección de `World::projection` (portada, con oráculo en `hooks6`).
    fn f58cfb0(&mut self, a: u32, b: u32) -> (u32, u32) {
        let [c510, c514, c4d0, c4d4] = self.projection.expect("0x58CFB0: falta World::projection (proyección de la escena)");
        crate::hooks6::cell_to_pixel_alt_pure(a as i32, b, c510, c514, c4d0, c4d4)
    }
    fn f563090(&mut self, _o: &mut SceneMem) {
        panic!("0x563090 (hooks6::aim_at_row) necesita la tabla de filas de la escena (+0xC67C), que World no modela; solo la usan tipos que ni nieve, lluvia ni grietas crean")
    }
    fn f57af60(&mut self, _set: u32, _flag: u32) {
        panic!("0x57AF60 (hooks6::shift_frame) necesita la memoria del conjunto de fotogramas, que World no modela; solo la usan tipos que ni nieve, lluvia ni grietas crean")
    }
    fn f5986a0(&mut self, _a: u16, _b: u16) {
        panic!("0x5986A0 (hooks6::mark_pixel) necesita la vista y la máscara de la escena, que World no modela; solo la usan tipos que ni nieve, lluvia ni grietas crean")
    }
    fn f570800(&mut self, o: &mut SceneMem) {
        unreachable!("f570800: no se usa en nieve ni lluvia")
    }
    fn registry_push(&mut self, obj: u32) {
        unreachable!("registry_push: no se usa en nieve ni lluvia")
    }
    fn f5709c0(&mut self, _o: &SceneMem) -> u8 {
        unreachable!("f5709c0: no se usa en nieve ni lluvia")
    }
    fn f4d8db0(&mut self, idx: u32) -> u32 {
        unreachable!("f4d8db0: no se usa en nieve ni lluvia")
    }
    fn f559790(&mut self, set: u32) {
        unreachable!("f559790: no se usa en nieve ni lluvia")
    }
    fn f58d030(&mut self, x: i32, y: i32) -> (u32, u32) {
        unreachable!("f58d030: no se usa en nieve ni lluvia")
    }
    fn f630cc0(&mut self, side: u8, a: u32, b: u32, size: u32) {
        unreachable!("f630cc0: no se usa en nieve ni lluvia")
    }
    fn f58b740(&mut self, a: &mut u32, b: &mut u32, dir: u8) {
        unreachable!("f58b740: no se usa en nieve ni lluvia")
    }
    fn f56a240(&mut self, o: &mut SceneMem) {
        unreachable!("f56a240: no se usa en nieve ni lluvia")
    }
}

struct CrackTypes<'a> {
    projection: Option<[i32; 4]>,
    rand: &'a mut dyn FnMut() -> i32,
    interval: u32,
    clock: u32,
    cracks: &'a mut Vec<u32>,
}

impl TypeEnv for CrackTypes<'_> {
    fn rand(&mut self) -> u32 {
        (self.rand)() as u32
    }
    fn f557730(&mut self, set: u32, n: u32) -> u8 {
        unreachable!("f557730: no se usa en nieve ni lluvia")
    }
    fn f559cf0(&mut self, set: u32) {
        unreachable!("f559cf0: no se usa en nieve ni lluvia")
    }
    fn f559260(&mut self, set: u32) -> bool {
        unreachable!("f559260: no se usa en nieve ni lluvia")
    }
    fn f59b5b0(&mut self, which: u32) {
        unreachable!("f59b5b0: no se usa en nieve ni lluvia")
    }
    fn f559810(&mut self, set: u32) {
        unreachable!("f559810: no se usa en nieve ni lluvia")
    }
    fn f55e620(&mut self, args: [u32; 10]) {
        unreachable!("f55e620: no se usa en nieve ni lluvia")
    }
    fn rec_byte(&mut self, side: u8, off: u32) -> u8 {
        unreachable!("rec_byte: no se usa en nieve ni lluvia")
    }
    fn rec_word(&mut self, side: u8, off: u32) -> u16 {
        unreachable!("rec_word: no se usa en nieve ni lluvia")
    }
    fn f55a290(&mut self, a: [u32; 4]) {
        unreachable!("f55a290: no se usa en nieve ni lluvia")
    }
    /// Invalidación de pantalla: el renderizador repinta todo.
    fn f559130(&mut self, _a: u32, _set: u32) {}
    fn bump_c58c(&mut self) {
        unreachable!("bump_c58c: no se usa en nieve ni lluvia")
    }
    fn v1c(&mut self) {
        unreachable!("v1c: no se usa en nieve ni lluvia")
    }
    fn g(&mut self, which: u32, _arg: u32) -> u32 {
        assert_eq!(which, 2, "la grieta solo lee el reloj");
        self.clock
    }
    fn shift_w(&mut self, d: i32) {
        unreachable!("shift_w: no se usa en nieve ni lluvia")
    }
    fn rd8(&mut self, a: u32) -> u8 {
        unreachable!("rd8: no se usa en nieve ni lluvia")
    }
    fn rd16(&mut self, a: u32) -> u16 {
        unreachable!("rd16: no se usa en nieve ni lluvia")
    }
    fn rd32(&mut self, a: u32) -> u32 {
        assert_eq!(a, 0x006E_2EE0, "la grieta solo lee el intervalo de su conjunto");
        self.interval
    }
    fn wr8(&mut self, a: u32, v: u8) {
        unreachable!("wr8: no se usa en nieve ni lluvia")
    }
    fn wr16(&mut self, a: u32, v: u16) {
        unreachable!("wr16: no se usa en nieve ni lluvia")
    }
    fn wr32(&mut self, a: u32, v: u32) {
        unreachable!("wr32: no se usa en nieve ni lluvia")
    }
    fn scene_r32(&mut self, off: u32) -> u32 {
        unreachable!("scene_r32: no se usa en nieve ni lluvia")
    }
    fn scene_r8(&mut self, off: u32) -> u8 {
        unreachable!("scene_r8: no se usa en nieve ni lluvia")
    }
    fn scene_r16(&mut self, off: u32) -> u16 {
        unreachable!("scene_r16: no se usa en nieve ni lluvia")
    }
    fn f559370(&mut self, set: u32) {
        unreachable!("f559370: no se usa en nieve ni lluvia")
    }
    fn f59a260(&mut self) {
        unreachable!("f59a260: no se usa en nieve ni lluvia")
    }
    fn f_place(&mut self, which: u32, o: &mut SceneMem, a: u32, b: u32) {
        unreachable!("f_place: no se usa en nieve ni lluvia")
    }
    fn f557730_2c(&mut self, set: u32, n: u32) -> u8 {
        unreachable!("f557730_2c: no se usa en nieve ni lluvia")
    }
    fn f55e250(&mut self, set: u32) {
        unreachable!("f55e250: no se usa en nieve ni lluvia")
    }
    fn f55e430(&mut self, set: u32) {
        unreachable!("f55e430: no se usa en nieve ni lluvia")
    }
    fn f571da0(&mut self) {
        unreachable!("f571da0: no se usa en nieve ni lluvia")
    }
    /// `0x58CFB0` con la proyección de `World::projection` (portada, con oráculo en `hooks6`).
    fn f58cfb0(&mut self, a: u32, b: u32) -> (u32, u32) {
        let [c510, c514, c4d0, c4d4] = self.projection.expect("0x58CFB0: falta World::projection (proyección de la escena)");
        crate::hooks6::cell_to_pixel_alt_pure(a as i32, b, c510, c514, c4d0, c4d4)
    }
    fn f563090(&mut self, _o: &mut SceneMem) {
        panic!("0x563090 (hooks6::aim_at_row) necesita la tabla de filas de la escena (+0xC67C), que World no modela; solo la usan tipos que ni nieve, lluvia ni grietas crean")
    }
    fn f57af60(&mut self, _set: u32, _flag: u32) {
        panic!("0x57AF60 (hooks6::shift_frame) necesita la memoria del conjunto de fotogramas, que World no modela; solo la usan tipos que ni nieve, lluvia ni grietas crean")
    }
    fn f5986a0(&mut self, _a: u16, _b: u16) {
        panic!("0x5986A0 (hooks6::mark_pixel) necesita la vista y la máscara de la escena, que World no modela; solo la usan tipos que ni nieve, lluvia ni grietas crean")
    }
    fn f570800(&mut self, o: &mut SceneMem) {
        unreachable!("f570800: no se usa en nieve ni lluvia")
    }
    fn registry_push(&mut self, obj: u32) {
        self.cracks.push(obj);
    }
    fn f5709c0(&mut self, _o: &SceneMem) -> u8 {
        unreachable!("f5709c0: no se usa en nieve ni lluvia")
    }
    fn f4d8db0(&mut self, idx: u32) -> u32 {
        unreachable!("f4d8db0: no se usa en nieve ni lluvia")
    }
    fn f559790(&mut self, set: u32) {
        unreachable!("f559790: no se usa en nieve ni lluvia")
    }
    fn f58d030(&mut self, x: i32, y: i32) -> (u32, u32) {
        unreachable!("f58d030: no se usa en nieve ni lluvia")
    }
    fn f630cc0(&mut self, side: u8, a: u32, b: u32, size: u32) {
        unreachable!("f630cc0: no se usa en nieve ni lluvia")
    }
    fn f58b740(&mut self, a: &mut u32, b: &mut u32, dir: u8) {
        unreachable!("f58b740: no se usa en nieve ni lluvia")
    }
    fn f56a240(&mut self, o: &mut SceneMem) {
        unreachable!("f56a240: no se usa en nieve ni lluvia")
    }
}

impl PrecipEnv for World {
    fn rand(&mut self) -> i32 {
        (self.rand)()
    }
    /// Virtual `+0x20`: nieve y grieta devuelven la clase de su conjunto (`0x558640`), la lluvia `0x21` (`0x575F00`); los tres coinciden con el tipo de la fábrica.
    fn obj_type(&mut self, h: u32) -> u8 {
        self.objects.get(&h).map_or(0, |o| o.ty as u8)
    }
    /// Destructor virtual con borrado: quita el objeto (los enlaces del registro que lo apuntan no se tocan).
    fn destroy(&mut self, h: u32) {
        self.objects.remove(&h);
    }
    fn make(&mut self, ty: u32) -> u32 {
        self.create(ty)
    }
    fn new_link(&mut self) -> u16 {
        self.registry.find_free(1) as u16
    }
    fn register(&mut self, h: u32, link: u16) {
        self.registry.register(h, u32::from(link));
    }
    /// Virtual `+0x40`: `0x576250` (nieve) o `0x575A80` (lluvia).
    fn spawn(&mut self, h: u32, x: u32, y: u32, a: u32, link: u32, v: u32) {
        let args = SpawnArgs { x, y, heading: a, link, variant: v };
        let ty = self.obj(h).ty;
        match ty {
            RAIN => spawn_t33(&mut self.obj(h).mem, &args),
            SNOW => {
                let first: Vec<u32> = self.snow_sets.iter().map(|l| l.dirs[0].len() as u32).collect();
                let handles: Vec<u32> = (1..=first.len() as u32).collect();
                let table = Table { count: first.len() as u32, sets: &handles, kinds: &[] };
                let mut mem = std::mem::replace(&mut self.obj(h).mem, SceneMem(vec![]));
                let mut env = SnowTypes { projection: self.projection, rand: &mut *self.rand, first_frames: &first };
                spawn_t29(&mut mem, &args, &table, &mut env);
                self.obj(h).mem = mem;
            }
            CRACK => {
                let mut mem = std::mem::replace(&mut self.obj(h).mem, SceneMem(vec![]));
                let mut env = CrackTypes { projection: self.projection, rand: &mut *self.rand, interval: self.crack_interval, clock: self.clock, cracks: &mut self.cracks };
                spawn_t34(&mut mem, &args, h, &mut env);
                self.obj(h).mem = mem;
            }
            _ => unimplemented!("aparición del tipo {ty}"),
        }
    }
    /// Virtual `+0x28`: rectángulo del fotograma actual (`0x558660`) o, en la lluvia, la caja de la gota (`0x575DA0`).
    fn info(&mut self, h: u32) -> SpriteInfo {
        let (ty, heading, frame, c, d) = {
            let o = self.obj(h);
            (o.ty, o.mem.r8(0x14), o.mem.r8(0x15), o.mem.r8(0x21) as i8, o.mem.r8(0x23) as i8)
        };
        match ty {
            RAIN => SpriteInfo { w4: c.unsigned_abs() as u16, w6: d.unsigned_abs() as u16, s8: i16::from(c.min(0)), sa: i16::from(d.min(0)) },
            SNOW => {
                let idx = self.obj(h).mem.r8(0x24) as usize;
                let l = &self.snow_sets[idx];
                let r = &l.dirs[l.direction_of(heading)][frame as usize].rect;
                SpriteInfo { w4: r.w, w6: r.h, s8: r.dx, sa: r.dy }
            }
            _ => unimplemented!("información del tipo {ty}"),
        }
    }
    /// Virtual `+0x48(x, y, 0)`: nieve y lluvia guardan la posición en punto fijo 16.16.
    fn place(&mut self, h: u32, x: u32, y: u32) {
        let o = self.obj(h);
        o.mem.w32(0xC, x << 16);
        o.mem.w32(0x10, y << 16);
    }
    fn refresh(&mut self, _h: u32) {}
}


/// Objetos del mapa que consulta la grieta (`0x59BC90`): la casilla de la rejilla de objetos y el tamaño de la huella de cada uno (virtuales `+0x48` y `+0x4C`).
/// Las clases de esos objetos (los `.let` de la escena) aún no están modeladas: quien llama aporta los valores.
pub trait MapObjects {
    /// Identificador del objeto en la casilla `indice = c314 * y + x` de `escena+0xC30C`.
    fn cell(&mut self, idx: u32) -> u16;
    fn width(&mut self, h: u16) -> i32;
    fn height(&mut self, h: u16) -> i32;
}

/// Entorno de `0x59BC90`: el mundo, la escena (solo lectura) y los objetos del mapa.
pub struct CrackCtx<'a> {
    pub world: &'a mut World,
    pub scene: &'a SceneMem,
    pub map: &'a mut dyn MapObjects,
}

impl CrackEnv for CrackCtx<'_> {
    /// `0x57FF20(x, y, p)`: si ya hay una grieta en `(x, y)` la reutiliza (cambia su byte `+0x29`, pone a cero `+0x28`) y cancela.
    fn skip(&mut self, x: u32, y: u32, p: u32) -> bool {
        let found = self.world.cracks.iter().copied().find(|h| {
            let o = &self.world.objects[h].mem;
            o.r32(0x20) == x && o.r32(0x24) == y
        });
        let Some(h) = found else { return false };
        let o = &mut self.world.obj(h).mem;
        // Si el byte difiere, el original llama antes al virtual +0x1C (0x558610: invalidar el rectángulo en pantalla); el renderizador repinta siempre.
        if o.r8(0x29) != p as u8 {
            o.w8(0x29, p as u8);
        }
        o.w8(0x28, 0);
        true
    }
    fn cell(&mut self, idx: u32) -> u16 {
        self.map.cell(idx)
    }
    /// `0x58DAC0(&(x, y), &salida)`: píxel del centro de la casilla `(x, y)` (las filas impares van desplazadas media casilla).
    fn pair(&mut self, x: u32, y: u32) -> (u32, u32) {
        let m = self.scene;
        let (c510, c514, c4d0, c4d4) = (m.r32(0x510) as i32, m.r32(0x514) as i32, m.r32(0x4D0) as i32, m.r32(0x4D4) as i32);
        let (ox, oy) = (m.r32(0x14) as i32, m.r32(0x18) as i32);
        let (a, b) = if y & 1 == 0 {
            ((x as i32).wrapping_sub(1).wrapping_mul(c510).wrapping_add(c4d0), c514.wrapping_add(1).wrapping_mul((y >> 1) as i32).wrapping_add(c4d4))
        } else {
            (c510.wrapping_mul(x as i32), ((y >> 1) as i32).wrapping_mul(c514.wrapping_add(1)).wrapping_add(1).wrapping_add(c514))
        };
        (a.wrapping_add(ox) as u32, b.wrapping_add(oy) as u32)
    }
    fn obj_w(&mut self, h: u16) -> i32 {
        self.map.width(h)
    }
    fn obj_h(&mut self, h: u16) -> i32 {
        self.map.height(h)
    }
    fn make(&mut self, ty: u32) -> u32 {
        self.world.create(ty)
    }
    fn new_link(&mut self) -> u16 {
        self.world.registry.find_free(1) as u16
    }
    fn register(&mut self, h: u32, link: u16) {
        self.world.registry.register(h, u32::from(link));
    }
    fn spawn(&mut self, h: u32, x: i32, y: i32, a: u32, link: u32, packed: u32) {
        PrecipEnv::spawn(self.world, h, x as u32, y as u32, a, link, packed);
    }
    fn set_byte29(&mut self, h: u32, v: u8) {
        self.world.obj(h).mem.w8(0x29, v);
    }
}
