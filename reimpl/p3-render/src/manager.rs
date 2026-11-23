//! Gestor de las figuras de la ciudad (`Class22` método `0x00596EE0`, J02).
//!
//! Reparte las cuatro poblaciones (clases 4, 0, 1 y 2 del fabricante `0x557D10`), los niños, los trabajadores del puerto y los soldados
//! de milicia (`0x1A`). Cada llamada hace como mucho una aparición en la entrada de cada temporizador: los dos primeros bucles de cuatro
//! clases terminan en la primera clase vencida (`break` del original). Las listas de punteros son `[A ocupadas][B entrantes][C reasignadas]`.
//! Los objetos son identificadores opacos; todo lo exterior (aleatorio, fábrica, aparición virtual, rutas, casillas libres) entra por [`ManagerEnv`].
//! Paridad: `tests/x86_manager.rs` frente al x86 original (`oracle/x86_manager.py`).

/// Cuatro clases de población, en el orden de las listas: tipos de entidad 4, 0, 1 y 2.
pub const CLASS_TYPES: [u8; 4] = [4, 0, 1, 2];

pub trait ManagerEnv {
    /// `0x633AE0` (modo hansa/partida especial).
    fn hanse(&mut self) -> bool;
    /// `rand` del CRT.
    fn rand(&mut self) -> u32;
    /// Fábrica `0x557D10(tipo)`.
    fn new_object(&mut self, kind: u8) -> u32;
    /// `0x568A70(1)`: primer enlace libre.
    fn next_link(&mut self) -> u32;
    /// `0x568B20(objeto, enlace)`.
    fn register(&mut self, obj: u32, link: u32);
    /// Método virtual `+0x40` (aparición): `x`, `y` en 16.16, rumbo, enlace, `modo << 16 | ruta`.
    fn spawn(&mut self, obj: u32, x: u32, y: u32, heading: u8, link: u32, mode_route: u32);
    /// Método virtual `+0x2C`: posición entera (x, y) del objeto.
    fn position(&mut self, obj: u32) -> (u32, u32);
    /// Campo `+0x48` del objeto (edad).
    fn age(&mut self, obj: u32) -> u32;
    /// Campo `+0x78` del objeto (índice de tipo en la tabla de rutas).
    fn type_index(&mut self, obj: u32) -> u8;
    /// `0x55B1D0(tabla, objeto, ruta, 0)`: asigna una ruta; `table` es el puntero de la tabla (0 si no hay).
    fn assign_route(&mut self, table: u32, obj: u32, route: u32) -> bool;
    /// Método `+4` del subobjeto `+0x3F8`: retirar un objeto.
    fn remove(&mut self, obj: u32);
    /// `0x59A4F0(&salida, a, b, c)`: casilla libre `(x, bandera)` si hay.
    fn free_cell(&mut self, a: u32, b: u32, c: u32) -> Option<(i32, u32)>;
    /// `realloc` de una lista de punteros a `bytes` bytes.
    fn realloc(&mut self, bytes: u32);
    /// `0x59A6A0(1)`.
    fn after(&mut self, arg: u32);
}

/// Una lista de punteros con capacidad (como el bloque de `realloc`): el contenido más allá de lo usado no se lee.
#[derive(Clone, Debug, Default)]
pub struct PtrList {
    pub cap: u16,
    pub items: Vec<u32>,
}

impl PtrList {
    fn get(&self, i: usize) -> u32 {
        self.items.get(i).copied().unwrap_or(0)
    }
    fn set(&mut self, i: usize, v: u32) {
        if self.items.len() <= i {
            self.items.resize(i + 1, 0);
        }
        self.items[i] = v;
    }
    /// `if usado == cap { cap += 100; realloc }`.
    fn grow_if_full(&mut self, used: usize) -> bool {
        if used == usize::from(self.cap) {
            self.cap = self.cap.wrapping_add(100);
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Population {
    pub list: PtrList,
    /// Ocupadas (`0xC5B8`), entrantes (`0xC5C0`) y reasignadas (`0xC5C8`).
    pub a: u16,
    pub b: u16,
    pub c: u16,
    /// Temporizadores `0xC5D0` y `0xC5E0`.
    pub t1: u32,
    pub t2: u32,
}

/// Tablas de rutas por clase: `(cantidad, punteros)`.
#[derive(Clone, Debug, Default)]
pub struct RouteTable {
    pub count: u32,
    pub ptrs: Vec<u32>,
}

/// Registro de la ciudad `DAT_006DE508 + plan * 0x9F8` (bytes crudos).
#[derive(Clone, Debug, Default)]
pub struct TownBytes(pub Vec<u8>);

impl TownBytes {
    fn u8(&self, o: usize) -> u8 {
        self.0[o]
    }
    fn i16(&self, o: usize) -> i16 {
        i16::from_le_bytes([self.0[o], self.0[o + 1]])
    }
    fn i32(&self, o: usize) -> i32 {
        i32::from_le_bytes([self.0[o], self.0[o + 1], self.0[o + 2], self.0[o + 3]])
    }
    /// Entero `0x2E4 - 4k`: población objetivo de la clase `k`.
    fn pop(&self, k: usize) -> i32 {
        self.i32(0x2E4 - 4 * k)
    }
    /// Entero `0x2F4 - 4k`.
    fn pop2(&self, k: usize) -> i32 {
        self.i32(0x2F4 - 4 * k)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Harbour {
    pub n: u16,
    pub list: Vec<u32>,
    /// `*(short *)(*(ptr + 8) + 0x3C) != 0`.
    pub flag: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Manager {
    pub plan: u32,
    pub plan_count: u32,
    pub clock: u32,
    pub gate: (u32, u32, u32),
    pub next: u32,
    pub view: (u32, u32, u32, u32),
    /// `+0x510`, `+0x514`, `+0x4D0`, `+0x4D4`.
    pub geom: (i32, i32, i32, i32),
    pub f58c: u32,
    pub pops: [Population; 4],
    pub kids: PtrList,
    pub kids_n: u16,
    pub tables: [RouteTable; 4],
    pub town: TownBytes,
    pub harbour: Harbour,
}

/// Posición 16.16 de una casilla `(x, bandera)` con la geometría `(+0x510, +0x514, +0x4D0, +0x4D4)`.
pub fn cell_position(geom: (i32, i32, i32, i32), cx: i32, flag: u32) -> (u32, u32) {
    let (g510, g514, g4d0, g4d4) = geom;
    let (x, y) = if flag & 1 == 0 {
        (g510.wrapping_mul(cx).wrapping_add(g4d0), ((flag >> 1) as i32).wrapping_mul(g514.wrapping_add(1)).wrapping_sub(g4d4))
    } else {
        (cx.wrapping_add(1).wrapping_mul(g510), ((flag >> 1) as i32).wrapping_mul(g514.wrapping_add(1)))
    };
    ((x as u32) << 16, (y.wrapping_add(g514) as u32) << 16)
}

fn div_u(a: u64, b: u64) -> u32 {
    (a / b.max(1)) as u32
}

impl Manager {
    /// Quita un objeto de las listas tras su retirada. Las poblaciones usan `0x597FB0` ([`crate::lifecycle::pop_remove`]: el último de la región ocupa el
    /// hueco, con paridad x86 en `tests/x86_object_remove.rs`); niños y trabajadores del puerto conservan una aproximación que mantiene el orden (sin oráculo).
    pub fn forget(&mut self, obj: u32) {
        for p in &mut self.pops {
            let total = usize::from(p.a) + usize::from(p.b) + usize::from(p.c);
            if let Some(i) = p.list.items.iter().take(total).position(|&o| o == obj) {
                let mut list = p.list.items.clone();
                list.resize(total + 2, 0);
                let mut pl = crate::lifecycle::PopList { a: p.a, b: p.b, c: p.c, list };
                crate::lifecycle::pop_remove(&mut pl, i as u32);
                p.a = pl.a;
                p.b = pl.b;
                p.c = pl.c;
                let keep = p.list.items.len();
                p.list.items = pl.list;
                p.list.items.truncate(keep.max(usize::from(p.a) + usize::from(p.b) + usize::from(p.c)));
                return;
            }
        }
        if let Some(i) = self.kids.items.iter().take(usize::from(self.kids_n)).position(|&o| o == obj) {
            self.kids.items.remove(i);
            self.kids_n -= 1;
        } else if let Some(i) = self.harbour.list.iter().take(usize::from(self.harbour.n)).position(|&o| o == obj) {
            self.harbour.list.remove(i);
            self.harbour.n -= 1;
        }
    }

    /// Posición en 16.16 de una casilla libre: `(x, y)` ya multiplicados.
    fn cell_pos(&self, cx: i32, flag: u32) -> (u32, u32) {
        cell_position(self.geom, cx, flag)
    }

    fn outside(&self, (x, y): (u32, u32)) -> bool {
        let (v650, v654, s2c, s30) = self.view;
        let (x, y) = (x & 0xFFFF, y & 0xFFFF);
        x + 0x32 < v650 || s2c.wrapping_add(0x32).wrapping_add(v650) < x || y + 0x32 < v654 || s30.wrapping_add(0x32).wrapping_add(v654) < y
    }

    fn town_flags(&self) -> u8 {
        self.town.u8(0x2C8)
    }

    /// Objetivo de niños (`ceil(Σ pop_i · peso_i / 10000)`) según la ciudad.
    fn kids_target(&self, env: &mut dyn ManagerEnv) -> u16 {
        if env.hanse() || self.town_flags() & 8 != 0 {
            return 0;
        }
        let mut sum: i32 = 0;
        for j in 0..4 {
            sum = sum.wrapping_add(i32::from(self.town.i16(0x300 + 2 * j)).wrapping_mul(self.town.i32(0x2D8 + 4 * j)));
        }
        if sum < 0 {
            sum = 0;
        }
        (sum.wrapping_add(9999) / 10000) as u16
    }

    /// Un paso del gestor (`0x596EE0`).
    pub fn update(&mut self, env: &mut dyn ManagerEnv) {
        if !(self.plan < self.plan_count && self.gate.0 == 3 && self.gate.1 == 3 && self.gate.2 != 0 && self.next < self.clock) {
            return;
        }
        self.next = self.clock.wrapping_add(0x616);
        // 1. Aparición por entrada (modo 1): como mucho una clase por llamada.
        for k in 0..4 {
            if self.pops[k].t1 < self.clock {
                if !env.hanse() {
                    let kind = CLASS_TYPES[k];
                    let p = &mut self.pops[k];
                    let (a, b, c) = (usize::from(p.a), usize::from(p.b), usize::from(p.c));
                    if p.list.grow_if_full(a + b + c) {
                        env.realloc(u32::from(p.list.cap) << 2);
                    }
                    if c != 0 {
                        let v = p.list.get(a + b);
                        p.list.set(a + b + c, v);
                    }
                    let r = env.rand();
                    let obj = env.new_object(kind);
                    let link = env.next_link();
                    env.register(obj, link);
                    env.spawn(obj, 0x100, 0x100, 0x20, link, (r % 3) + 0x10000);
                    p.list.set(a + b, obj);
                    p.b += 1;
                }
                let d = (self.town.pop(k) - self.town.pop2(k)).wrapping_add(10).max(10) as u32;
                self.pops[k].t1 = div_u(2_000_000, u64::from(d)).wrapping_add(self.clock);
                break;
            }
        }
        // 2. Reasignar a rutas una figura ocupada cuando vence t2.
        let mut local10 = [0u32; 4];
        for k in 0..4 {
            let d = self.town.pop2(k).wrapping_sub(self.town.pop(k)).wrapping_add(10);
            local10[k] = if d < 10 { 10 } else { d as u32 };
            if self.pops[k].t2 < self.clock && !env.hanse() {
                let mut j = 0usize;
                let a = usize::from(self.pops[k].a);
                if a != 0 {
                    loop {
                        let obj = self.pops[k].list.get(j);
                        let ti = env.type_index(obj);
                        let t = &self.tables[k];
                        let table = if u32::from(ti) < t.count { t.ptrs.get(usize::from(ti)).copied().unwrap_or(0) } else { 0 };
                        let mut found = false;
                        for route in 0..3 {
                            if env.assign_route(table, obj, route) {
                                found = true;
                                break;
                            }
                        }
                        if found {
                            break;
                        }
                        j += 1;
                        if a <= j {
                            break;
                        }
                    }
                }
                if j < a {
                    let p = &mut self.pops[k];
                    let b = usize::from(p.b);
                    let na = a - 1;
                    p.a = na as u16;
                    let tmp = p.list.get(j);
                    let v = p.list.get(na);
                    p.list.set(j, v);
                    let v = p.list.get(b + na);
                    p.list.set(na, v);
                    p.list.set(na + b, tmp);
                    p.c += 1;
                    p.t2 = div_u(2_000_000, u64::from(local10[k])).wrapping_add(self.clock);
                }
            }
        }
        // 3. Aparición fuera de pantalla (modo 2): como mucho una clase por llamada.
        for k in 0..4 {
            let due = div_u(8_000_000, u64::from(local10[k].wrapping_mul(3))).wrapping_add(self.pops[k].t2);
            if due < self.clock {
                if !env.hanse() {
                    let kind = CLASS_TYPES[k];
                    let p = &mut self.pops[k];
                    let (a, b, c) = (usize::from(p.a), usize::from(p.b), usize::from(p.c));
                    if p.list.grow_if_full(a + b + c) {
                        env.realloc(u32::from(p.list.cap) << 2);
                    }
                    let r = env.rand();
                    let obj = env.new_object(kind);
                    let link = env.next_link();
                    env.register(obj, link);
                    env.spawn(obj, 5000, 5000, 0, link, (r % 3) + 0x20000);
                    p.list.set(b + a + c, obj);
                    p.c += 1;
                }
                self.pops[k].t2 = div_u(2_000_000, u64::from(local10[k] << 2)).wrapping_add(self.clock);
                break;
            }
        }
        // 4. Población libre: crear o retirar según la cuota.
        let quota: i32 = if !env.hanse() {
            if self.town_flags() & 8 != 0 { 100 } else { 50 }
        } else if self.harbour.flag {
            1000
        } else {
            200
        };
        for k in 0..4 {
            let pop = i32::from(self.pops[k].a);
            let target = self.town.pop(k);
            if quota.wrapping_mul(pop) < target {
                if let Some((cx, flag)) = env.free_cell(1, 1, 1) {
                    let (x, y) = self.cell_pos(cx, flag);
                    let kind = CLASS_TYPES[k];
                    let p = &mut self.pops[k];
                    let (a, b, c) = (usize::from(p.a), usize::from(p.b), usize::from(p.c));
                    if p.list.grow_if_full(b + a + c) {
                        env.realloc(u32::from(p.list.cap) << 2);
                    }
                    if c != 0 {
                        let v = p.list.get(a + b);
                        p.list.set(c + b + a, v);
                    }
                    if b != 0 {
                        let v = p.list.get(a);
                        p.list.set(b + a, v);
                    }
                    let r = env.rand() & 3;
                    let obj = env.new_object(kind);
                    let link = env.next_link();
                    env.register(obj, link);
                    env.spawn(obj, x, y, (r * 64 + 0x20) as u8, link, 0);
                    p.list.set(a, obj);
                    p.a += 1;
                }
            } else {
                let limit = (pop - 1).wrapping_mul(quota);
                if limit > target {
                    let mut best_age = 0u32;
                    let mut count = 0i32;
                    let mut chosen = pop;
                    for i in 0..pop {
                        let obj = self.pops[k].list.get(i as usize);
                        let pos = env.position(obj);
                        if self.outside(pos) {
                            let age = env.age(obj);
                            if best_age < age {
                                count = 1;
                                best_age = age;
                                chosen = i;
                            } else if age == best_age {
                                count += 1;
                                if env.rand() as i32 % count == 0 {
                                    chosen = i;
                                }
                            }
                        }
                    }
                    if chosen < pop {
                        env.remove(self.pops[k].list.get(chosen as usize));
                    }
                }
            }
        }
        // 5. Niños.
        let target = self.kids_target(env);
        if self.kids_n < target {
            if let Some((cx, flag)) = env.free_cell(1, 1, 1) {
                let (x, y) = self.cell_pos(cx, flag);
                if self.kids_n == self.kids.cap {
                    self.kids.cap = self.kids.cap.wrapping_add(100);
                    env.realloc(u32::from(self.kids.cap) << 2);
                }
                let r = env.rand() & 3;
                let obj = env.new_object(3);
                let link = env.next_link();
                env.register(obj, link);
                env.spawn(obj, x, y, (r * 64 + 0x20) as u8, link, 0);
                self.kids.set(usize::from(self.kids_n), obj);
                self.kids_n += 1;
            }
        } else {
            let target = self.kids_target(env);
            if target < self.kids_n {
                let mut i = 0usize;
                if self.kids_n != 0 {
                    loop {
                        let pos = env.position(self.kids.get(i));
                        if self.outside(pos) {
                            break;
                        }
                        i += 1;
                        if usize::from(self.kids_n) <= i {
                            break;
                        }
                    }
                }
                if i < usize::from(self.kids_n) {
                    env.remove(self.kids.get(i));
                }
            }
        }
        // 6. Trabajadores del puerto y cierre.
        if !env.hanse() {
            let want = u16::from(self.town.u8(0x998));
            if self.harbour.n < want {
                if let Some((cx, flag)) = env.free_cell(1, 1, 1) {
                    let (x, y) = self.cell_pos(cx, flag);
                    let obj = env.new_object(8);
                    let link = env.next_link();
                    env.register(obj, link);
                    env.spawn(obj, x, y, 0x20, link, self.plan + 0x10000);
                    let n = usize::from(self.harbour.n);
                    if self.harbour.list.len() <= n {
                        self.harbour.list.resize(n + 1, 0);
                    }
                    self.harbour.list[n] = obj;
                    self.harbour.n += 1;
                }
            } else if want < self.harbour.n {
                let mut i = 0usize;
                if self.harbour.n != 0 {
                    loop {
                        let pos = env.position(self.harbour.list[i]);
                        if self.outside(pos) {
                            break;
                        }
                        i += 1;
                        if usize::from(self.harbour.n) <= i {
                            break;
                        }
                    }
                }
                if i < usize::from(self.harbour.n) {
                    env.remove(self.harbour.list[i]);
                }
            }
            env.after(1);
        }
        // 7. Soldados de milicia (0x1A), solo con la bandera 8 de la ciudad.
        if self.town_flags() & 8 != 0 && self.f58c < (self.town.i32(0x2D4) / 0x32) as u32 {
            if let Some((cx, flag)) = env.free_cell(1, 1, 0) {
                let (x, y) = self.cell_pos(cx, flag);
                let r = env.rand() & 3;
                let obj = env.new_object(0x1A);
                let link = env.next_link();
                env.register(obj, link);
                env.spawn(obj, x, y, (r * 64 + 0x20) as u8, link, 0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fake {
        n: u32,
        removed: Vec<u32>,
    }
    impl ManagerEnv for Fake {
        fn hanse(&mut self) -> bool {
            false
        }
        fn rand(&mut self) -> u32 {
            1
        }
        fn new_object(&mut self, _k: u8) -> u32 {
            self.n += 1;
            self.n
        }
        fn next_link(&mut self) -> u32 {
            1
        }
        fn register(&mut self, _o: u32, _l: u32) {}
        fn spawn(&mut self, _o: u32, _x: u32, _y: u32, _h: u8, _l: u32, _m: u32) {}
        fn position(&mut self, _o: u32) -> (u32, u32) {
            (0xFFFF, 0xFFFF)
        }
        fn age(&mut self, _o: u32) -> u32 {
            0
        }
        fn type_index(&mut self, _o: u32) -> u8 {
            0
        }
        fn assign_route(&mut self, _t: u32, _o: u32, _r: u32) -> bool {
            false
        }
        fn remove(&mut self, o: u32) {
            self.removed.push(o);
        }
        fn free_cell(&mut self, _a: u32, _b: u32, _c: u32) -> Option<(i32, u32)> {
            Some((3, 4))
        }
        fn realloc(&mut self, _b: u32) {}
        fn after(&mut self, _a: u32) {}
    }

    fn base() -> Manager {
        let mut town = vec![0u8; 0x9F8];
        // objetivo de la clase 0 (0x2E4): 400 habitantes; cuota 50 por figura -> 8 figuras
        town[0x2E4..0x2E8].copy_from_slice(&400i32.to_le_bytes());
        let mut m = Manager { plan: 0, plan_count: 1, clock: 1000, gate: (3, 3, 1), next: 0, view: (0, 0, 100, 100), geom: (34, 10, 0, 0), town: TownBytes(town), ..Default::default() };
        for p in &mut m.pops {
            p.t1 = u32::MAX;
            p.t2 = u32::MAX / 2;
            p.list.cap = 4;
        }
        m
    }

    #[test]
    fn free_population_grows_to_quota_then_forgets() {
        let mut m = base();
        let mut env = Fake { n: 0, removed: vec![] };
        for step in 0..12 {
            m.clock = 1000 + step * 0x700;
            m.update(&mut env);
        }
        assert_eq!(m.pops[0].a, 8, "400 / 50 figuras");
        let first = m.pops[0].list.items[0];
        m.forget(first);
        assert_eq!(m.pops[0].a, 7);
        assert!(!m.pops[0].list.items.contains(&first));
    }
}
