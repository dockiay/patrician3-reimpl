//! Retirada de un objeto del pueblo: método `+4` del subobjeto `escena+0x3F8` (`0x593D50`), con `0x597FB0` (quitar de una lista de población) y `0x634D80`
//! (liberar un objeto del puerto).
//!
//! Observado y probado en emulador (`oracle/x86_object_remove.py`, 3.000 casos con el x86 real y `0x597FB0`, `0x634D80` y `0x594430` reales). El resto de
//! llamadas (métodos virtuales, `0x557D10`, `0x568A70`, `0x568B20`, `0x568BD0`, `0x59A4F0`, `0x4B9650`, `rand`) se sustituyen por [`LifeEnv`] y se
//! registran. Los desplazamientos de los campos son los del `this` del método (`escena + 0x3F8`).

/// Datos del objeto que se retira.
#[derive(Clone, Copy, Debug)]
pub struct Obj {
    pub handle: u32,
    /// Resultado del método virtual `+0x20`.
    pub kind: u8,
    /// `+0x16`
    pub link: u16,
    /// `+0x2C`, `+0x2E`, `+0x39`
    pub b2c: u8,
    pub b2e: u8,
    pub b39: u8,
}

/// Ranura de enlace de interfaz (`this + 0x24 + i * 0x24`).
#[derive(Clone, Copy, Debug, Default)]
pub struct UiSlot {
    pub f24: i32,
    pub f28: i32,
    pub f2c: i32,
    pub f30: i32,
    /// `+0x40`
    pub link: u16,
}

/// Lista de población de una clase: `[A][B][C]` seguidos de un centinela (`0x597FB0`).
#[derive(Clone, Debug, Default)]
pub struct PopList {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub list: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Life {
    pub ui: [UiSlot; 4],
    /// `+0xC258`, `+0xC25C`
    pub c258: i32,
    pub c25c: i32,
    /// Listas de las clases 0..3 (tipos 4, 0, 1, 2).
    pub pop: [PopList; 4],
    /// `+0xC208` y `+0xC20E`
    pub list3: Vec<u32>,
    pub count3: u16,
    /// `+0xC0CC` (aves), `+0xC11C` (animales), `+0xC16C` (perros/gato)
    pub birds: [u32; 20],
    pub animals: [u32; 20],
    pub pets: [u32; 10],
    /// `+0xC194`, `+0xC198`
    pub c194: i32,
    pub c198: i32,
    /// `+0xC19C`
    pub pop_flag: bool,
    /// `+0xC2F8` (50 ranuras), `+0xC3C4` y `+0xC3D4`
    pub weather: [u32; 50],
    pub weather_count: i32,
    pub weather_limit: i32,
}

/// Registro del puerto de la ciudad (`DAT_006DF270 + plan * 0x1024`).
#[derive(Clone, Debug, Default)]
pub struct HarbourLists {
    pub head: [u32; 2],
    /// `+0x100E`, `+0x100F`
    pub head_count: [u8; 2],
    /// Cuatro listas (`+0x14`, `+0x410`, `+0x80C`, `+0xC08`) con su contador (`+0x1004`..`+0x100A`).
    pub lists: [Vec<u32>; 4],
    pub counts: [u16; 4],
}

/// Escena: cuántas ciudades existen, la geometría de casillas (`escena+0x4D0`, `+0x4D4`, `+0x510`, `+0x514`) y el tamaño y la vista.
pub struct SceneGeom {
    pub plan: u32,
    pub town_count: u32,
    /// `+0xC31C`, `+0xC320`
    pub size: (u32, u32),
    /// `+0xC650`, `+0xC654`, `+0x2C`, `+0x30`
    pub view: (u32, u32, u32, u32),
    pub x0: i32,
    pub y0: i32,
    pub cw: i32,
    pub ch: i32,
}

pub trait LifeEnv {
    fn rand(&mut self) -> u32;
    /// `0x4B9650`: invalida el rectángulo de interfaz `(izq, arr, der, aba)`.
    fn invalidate(&mut self, l: i32, t: i32, r: i32, b: i32);
    /// Marca de objeto del puerto: bandera `+0x59`, siguiente `+0x5C`.
    fn flag59(&mut self, h: u32) -> bool;
    fn next5c(&mut self, h: u32) -> u32;
    fn set_next5c(&mut self, h: u32, v: u32);
    /// `0x568BD0(enlace)`
    fn unlink(&mut self, link: u16);
    /// `0x594430` lo hace el módulo; `0x59A4F0(&celda, a, b, 0)`: devuelve la casilla `(x, y)` si hay.
    fn pick(&mut self, a: u32, b: u32) -> (bool, i32, u32);
    /// `0x557D10(tipo)`: objeto nuevo.
    fn make(&mut self, ty: u32) -> u32;
    /// `0x568A70(1)`
    fn new_link(&mut self) -> u16;
    /// `0x568B20(objeto, enlace)`
    fn register(&mut self, h: u32, link: u16);
    /// Método virtual `+0x40(x, y, rumbo, enlace, modo)`.
    fn spawn(&mut self, h: u32, x: u32, y: u32, heading: u8, link: u16, mode: u32);
    /// Método virtual `+0` con el argumento dado (destructor).
    fn destroy(&mut self, h: u32, flag: u32);
}

/// `0x634D80(objeto, índice)`: saca un objeto del puerto de las cadenas de cabeza y de la lista `idx` (0..3; los demás índices solo tocan las cadenas).
pub fn harbour_release(hb: &mut HarbourLists, obj: u32, idx: u8, env: &mut dyn LifeEnv) {
    if env.flag59(obj) {
        if obj == hb.head[0] {
            hb.head[0] = env.next5c(obj);
            hb.head_count[0] = hb.head_count[0].wrapping_sub(1);
        } else if obj == hb.head[1] {
            hb.head[1] = env.next5c(obj);
            hb.head_count[1] = hb.head_count[1].wrapping_sub(1);
        } else {
            let mut b = 0usize;
            loop {
                let mut cur = hb.head[b];
                let mut found = false;
                loop {
                    let prev = cur;
                    if prev == 0 {
                        break;
                    }
                    cur = env.next5c(prev);
                    if cur == 0 {
                        break;
                    }
                    if cur == obj {
                        let n = env.next5c(obj);
                        env.set_next5c(prev, n);
                        hb.head_count[b] = hb.head_count[b].wrapping_sub(1);
                        found = true;
                        break;
                    }
                }
                b += 1;
                if found || b >= 2 {
                    break;
                }
            }
        }
    }
    if idx < 4 {
        let i = usize::from(idx);
        let n = hb.counts[i];
        let mut k = 0u8;
        while u16::from(k) < n {
            if hb.lists[i][usize::from(k)] == obj {
                hb.counts[i] = n - 1;
                hb.lists[i][usize::from(k)] = hb.lists[i][usize::from(n - 1)];
                hb.lists[i][usize::from(n - 1)] = 0;
                return;
            }
            k = k.wrapping_add(1);
        }
    }
}

/// `0x597FB0(clase, índice)`: quita la entrada `i` de la lista `[A][B][C]` de la clase.
pub fn pop_remove(p: &mut PopList, i: u32) {
    let a = u32::from(p.a);
    let sentinel = |p: &mut PopList| {
        let n = usize::from(p.a) + usize::from(p.b) + usize::from(p.c);
        p.list[n] = 0;
    };
    if i < a {
        p.a -= 1;
        if i != u32::from(p.a) {
            p.list[i as usize] = p.list[usize::from(p.a)];
        }
        if p.b != 0 {
            p.list[usize::from(p.a)] = p.list[usize::from(p.b) + usize::from(p.a)];
        }
    } else {
        let b = u32::from(p.b);
        if b + a <= i {
            p.c -= 1;
            let last = a + u32::from(p.c) + b;
            if i != last {
                p.list[i as usize] = p.list[last as usize];
            }
            sentinel(p);
            return;
        }
        p.b = (b - 1) as u16;
        let last = a + (b - 1);
        if i != last {
            p.list[i as usize] = p.list[last as usize];
        }
    }
    if p.c != 0 {
        p.list[usize::from(p.b) + usize::from(p.a)] = p.list[usize::from(p.c) + usize::from(p.b) + usize::from(p.a)];
    }
    sentinel(p);
}

fn cell_origin(g: &SceneGeom, cx: i32, cy: u32) -> (i32, i32) {
    if cy & 1 == 0 {
        (g.cw.wrapping_mul(cx).wrapping_add(g.x0), ((cy >> 1) as i32).wrapping_mul(g.ch + 1).wrapping_sub(g.y0))
    } else {
        (g.cw.wrapping_mul(cx + 1), ((cy >> 1) as i32).wrapping_mul(g.ch + 1))
    }
}

/// `0x594430`: punto al azar del mapa, desplazado fuera de la vista si cae dentro (con margen `m`).
fn far_point(g: &SceneGeom, mx: u32, my: u32, env: &mut dyn LifeEnv) -> (u32, u32) {
    let (w, h) = g.size;
    let x = env.rand() % w;
    let y = env.rand() % h;
    let (v650, v654, s2c, s30) = g.view;
    if v650.wrapping_sub(mx) <= x && x <= s2c.wrapping_add(v650).wrapping_add(mx) && v654.wrapping_sub(my) <= y && y <= s30.wrapping_add(v654).wrapping_add(my) {
        let half = w >> 1;
        let mut nx = w >> 2;
        if v650 <= half {
            nx += half;
        }
        let half_h = h >> 1;
        let qh = h >> 2;
        let ny = if half_h < v654 { qh } else { qh + half_h };
        return (nx, ny);
    }
    (x, y)
}

fn pick_cell(env: &mut dyn LifeEnv, a: u32, b: u32) -> (i32, u32) {
    let (ok, x, y) = env.pick(a, b);
    if ok {
        return (x, y);
    }
    let (_, x, y) = env.pick(a, 0);
    (x, y)
}

/// `0x593D50`: retira `o` de las estructuras del pueblo y, para aves, animales y perros/gato, crea un sustituto.
pub fn remove_object(l: &mut Life, hb: &mut HarbourLists, g: &SceneGeom, o: &Obj, env: &mut dyn LifeEnv) {
    for s in l.ui.iter_mut() {
        if s.link == o.link {
            let top = s.f28.wrapping_sub(l.c25c);
            let left = s.f24.wrapping_sub(l.c258);
            let right = s.f2c.wrapping_add(left);
            let bottom = s.f30.wrapping_add(top);
            env.invalidate(left, top, right, bottom);
            s.link = 0;
            break;
        }
    }
    let kind = o.kind;
    let mut mode = 0u32;
    let mut release: Option<u8> = None;
    match kind {
        0x08 => release = Some(0),
        0x09 => release = Some(1),
        0x0A => release = Some(2),
        0x0B => release = Some(3),
        0x0F => mode = (u32::from(o.b2e) << 16) | u32::from(o.b2c),
        0x10 | 0x11 => mode = u32::from(o.b39) + 1,
        0x15 => release = Some(4),
        0x16 => release = Some(5),
        0x1A => l.c194 = l.c194.wrapping_sub(1),
        0x1B => l.c198 = l.c198.wrapping_sub(1),
        0x20 => release = Some(6),
        0x23 => release = Some(7),
        0x24 => release = Some(8),
        _ => {}
    }
    if let Some(i) = release {
        harbour_release(hb, o.handle, i, env);
    }
    if o.link != 0 {
        env.unlink(o.link);
    }
    let mut rand_heading = |env: &mut dyn LifeEnv| -> u8 {
        let r = (env.rand() & 7) as u8;
        r << 5
    };
    match kind {
        0 | 1 | 2 | 4 => {
            let u: usize = match kind {
                4 => 0,
                0 => 1,
                1 => 2,
                _ => 3,
            };
            let p = &mut l.pop[u];
            let total = u32::from(p.a) + u32::from(p.b) + u32::from(p.c);
            for i in 0..total {
                if p.list[i as usize] == o.handle {
                    pop_remove(p, i);
                    return;
                }
            }
        }
        3 => {
            for i in 0..u32::from(l.count3) {
                if l.list3[i as usize] == o.handle {
                    l.count3 -= 1;
                    if i != u32::from(l.count3) {
                        l.list3[i as usize] = l.list3[usize::from(l.count3)];
                    }
                    l.list3[usize::from(l.count3)] = 0;
                    return;
                }
            }
        }
        0x0F if l.pop_flag => {
            if let Some(i) = l.birds.iter().position(|&h| h == o.handle) {
                let (x, y) = far_point(g, 0x32, 0x32, env);
                let heading = rand_heading(env);
                let nh = env.make(0x0F);
                let link = if o.link == 0 { env.new_link() } else { o.link };
                env.register(nh, link);
                env.spawn(nh, x << 16, y << 16, heading, link, mode);
                l.birds[i] = nh;
            }
        }
        0x10 if l.pop_flag && g.plan < g.town_count => {
            if let Some(i) = l.animals.iter().position(|&h| h == o.handle) {
                let (cx, cy) = pick_cell(env, 0, 1);
                let (x, y) = cell_origin(g, cx, cy);
                let heading = rand_heading(env);
                let nh = env.make(0x10);
                let link = if o.link == 0 { env.new_link() } else { o.link };
                env.register(nh, link);
                env.spawn(nh, (x as u32) << 16, ((y.wrapping_add(g.ch)) as u32) << 16, heading, link, mode);
                l.animals[i] = nh;
            }
        }
        0x11 if l.pop_flag && g.plan < g.town_count => {
            if let Some(i) = l.pets.iter().position(|&h| h == o.handle) {
                let (cx, cy) = pick_cell(env, 1, 1);
                let (x, y) = cell_origin(g, cx, cy);
                let heading = rand_heading(env);
                let nh = env.make(0x11);
                let link = env.new_link();
                env.register(nh, link);
                env.spawn(nh, (x as u32) << 16, ((y.wrapping_add(g.ch)) as u32) << 16, heading, link, 0);
                l.pets[i] = nh;
            }
        }
        0x1D | 0x21 => {
            if let Some(i) = l.weather.iter().position(|&h| h == o.handle) {
                let last = (l.weather_count - 1) as usize;
                l.weather[i] = l.weather[last];
                l.weather[last] = o.handle;
                l.weather_count -= 1;
                if l.weather_limit <= l.weather_count {
                    let p = l.weather[l.weather_count as usize];
                    if p != 0 {
                        env.destroy(p, 3);
                    }
                    l.weather[l.weather_count as usize] = 0;
                }
            }
        }
        _ => {}
    }
}

/// Campos propios de `0x58B090` (clima) en el marco de la escena.
#[derive(Clone, Debug, Default)]
pub struct WeatherExtra {
    /// `escena+0xC7C0`
    pub current: i32,
    /// `escena+0xC7C4`: reparto de la cuota entre las partículas.
    pub rate: u32,
}

/// Operaciones externas de `0x58B090`.
pub trait WeatherEnv {
    /// `0x557D10(tipo)`: objeto nuevo.
    fn make(&mut self, ty: u32) -> u32;
    /// Método `+4` del subobjeto `escena+0x3F8` (retirada; debe sacarlo de `l.weather`).
    fn remove(&mut self, l: &mut Life, h: u32);
    /// Destructor virtual `+0(1)`.
    fn destroy(&mut self, h: u32, flag: u32);
}

/// `0x58B090(cantidad, inmediato)`: fija el número de partículas de clima (hasta 50), crea las que faltan con la fábrica y, con `immediate`, retira o
/// destruye las que sobran. `town_flag4` es el bit `4` de `registro_ciudad+0x9C4`: activo da el tipo `0x1D` (cuota 9000); inactivo, `0x21` (cuota 0x9C4).
pub fn set_weather(l: &mut Life, w: &mut WeatherExtra, count: i32, immediate: bool, town_flag4: bool, env: &mut dyn WeatherEnv) {
    l.weather_limit = count.min(0x32);
    if immediate {
        w.current = l.weather_limit;
    }
    let cnt = l.weather_limit;
    if cnt == 0 {
        w.rate = 0;
    } else {
        let ty = if town_flag4 { 0x1D } else { 0x21 };
        let quota: u32 = if town_flag4 { 9000 } else { 0x9C4 };
        w.rate = quota / cnt as u32;
        for i in 0..cnt.max(0) as usize {
            if l.weather[i] == 0 {
                l.weather[i] = env.make(ty);
            }
        }
    }
    if immediate && l.weather_limit < 0x32 {
        let mut i = l.weather_limit;
        while i < 0x32 {
            let h = l.weather[i as usize];
            if h != 0 {
                if i < l.weather_count {
                    env.remove(l, h);
                    i -= 1;
                } else {
                    env.destroy(h, 1);
                    l.weather[i as usize] = 0;
                }
            }
            i += 1;
        }
    }
}
