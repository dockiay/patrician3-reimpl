//! Apariciones ambientales pequeñas de la ciudad (J02): el funcionario (`0x5984E0`, tipo 7) y el pescador (`0x59A420`, tipo 0x18).
//! Paridad: `tests/x86_ambient.rs` frente al x86 original (`oracle/x86_ambient.py`).

use crate::manager::ManagerEnv;

/// Entradas de `0x5984E0`.
pub struct OfficialIn<'a> {
    pub plan: u32,
    pub plan_count: u32,
    /// Byte `ciudad+0x76C`.
    pub flags_76c: u8,
    /// Tabla `DAT_006DE4E0` (plan -> ciudad de referencia).
    pub plan_map: &'a [u8],
    /// Tabla de rutas por ciudad `DAT_00677958` (256 bytes en el ejecutable).
    pub route_table: &'a [u8],
    pub clock: u32,
    /// `escena+0xC664`.
    pub f664: u32,
}

/// `0x5984E0`: cuando vence el temporizador `+0xC598` y la ciudad tiene el bit 2 de `+0x76C`, sale un funcionario por la ruta de su ciudad.
/// Devuelve el nuevo valor de `+0xC598`. `fallback` es `0x515D20` (solo si el plan no está en la tabla; inalcanzable tras la primera comprobación).
pub fn spawn_official(i: &OfficialIn, f598: u32, env: &mut dyn ManagerEnv, fallback: &mut dyn FnMut() -> u32) -> u32 {
    if !(i.plan < i.plan_count && i.flags_76c & 2 != 0 && f598 <= i.clock && f598 != 0) {
        return f598;
    }
    if env.hanse() {
        return f598;
    }
    if i.f664 == 0 {
        return 0;
    }
    let idx = if i.plan == 0xFF {
        0xFF
    } else if i.plan & 0x7F < i.plan_count {
        u32::from(i.plan_map.get((i.plan & 0x7F) as usize).copied().unwrap_or(0)) | (i.plan & 0x80)
    } else {
        fallback() & 0xFF
    };
    let route = i.route_table.get(idx as usize).copied().unwrap_or(0);
    let obj = env.new_object(7);
    let link = env.next_link();
    env.register(obj, link);
    env.spawn(obj, 0, 0, 0x20, link, u32::from(route));
    0
}

/// `0x59A420(x, y)`: crea un pescador (tipo 0x18) en `(x, y)` si algún puesto de pesca activo está a menos de `0x1685B` (distancia al cuadrado).
/// `flags[i]` es el byte activo del puesto `i`, `spots[i]` su posición. Devuelve el objeto creado.
pub fn spawn_fisher(env: &mut dyn ManagerEnv, x: i32, y: i32, count: u32, flags: &[bool], spots: &[(i32, i32)]) -> Option<u32> {
    let mut i = 0u8;
    let mut u = 0u32;
    if count != 0 {
        loop {
            if flags.get(u as usize).copied().unwrap_or(false) {
                let (sx, sy) = spots.get(u as usize).copied().unwrap_or((0, 0));
                let dx = x.wrapping_sub(sx);
                let dy = y.wrapping_sub(sy);
                if dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)) < 0x1685B {
                    let obj = env.new_object(0x18);
                    let link = env.next_link();
                    env.register(obj, link);
                    env.spawn(obj, (x as u32) << 16, (y as u32) << 16, 0x20, link, 0);
                    return Some(obj);
                }
            }
            i = i.wrapping_add(1);
            u = u32::from(i);
            if u >= count {
                break;
            }
        }
    }
    None
}

/// `0x58A920`: veinte aves (tipo 15) en puntos aleatorios de la rejilla `(ancho, alto) = (+0xC31C, +0xC320)`, con rumbo `(rand & 7) * 32`, ordenadas
/// por el byte `objeto+0x2C` (selección ascendente). Devuelve los veinte objetos en el orden final.
pub fn spawn_birds(env: &mut dyn ManagerEnv, width: u32, height: u32, key: &mut dyn FnMut(u32) -> u8) -> [u32; 20] {
    let mut slots = [0u32; 20];
    for slot in &mut slots {
        let heading = ((env.rand() & 7) * 32) as u8;
        let ry = env.rand();
        let rx = env.rand();
        let obj = env.new_object(15);
        let link = env.next_link();
        env.register(obj, link);
        env.spawn(obj, (rx % width) << 16, (ry % height) << 16, heading, link, 0);
        *slot = obj;
    }
    for u in 1..=20usize {
        let pi = u - 1;
        let mut obj = slots[pi];
        let mut b = key(obj);
        if u < 20 {
            for q in pi + 1..20 {
                if key(slots[q]) < b {
                    slots[pi] = slots[q];
                    slots[q] = obj;
                    obj = slots[pi];
                    b = key(obj);
                }
            }
        }
    }
    slots
}

/// Resultado de `0x58B1B0`: diez perros/gatos (tipo 17) y veinte animales (tipo 16); `None` si no había casilla libre (la ranura queda como estaba).
pub struct Fauna {
    pub pets: [Option<u32>; 10],
    pub animals: [Option<u32>; 20],
}

/// `0x58B1B0`: solo si los tres contadores `+0xC688..` son distintos de cero. Los perros usan la clave de casilla `(1, 0, 0)` y los animales `(0, 0, 0)`.
pub fn spawn_fauna(env: &mut dyn ManagerEnv, geom: (i32, i32, i32, i32), counts: [u32; 3]) -> Option<Fauna> {
    if counts.contains(&0) {
        return None;
    }
    let mut out = Fauna { pets: [None; 10], animals: [None; 20] };
    for (kind, first, n) in [(0x11u8, 1u32, 10usize), (0x10, 0, 20)] {
        for i in 0..n {
            if let Some((cx, flag)) = env.free_cell(first, 0, 0) {
                let (x, y) = crate::manager::cell_position(geom, cx, flag);
                let heading = ((env.rand() & 7) * 32) as u8;
                let obj = env.new_object(kind);
                let link = env.next_link();
                env.register(obj, link);
                env.spawn(obj, x, y, heading, link, 0);
                if kind == 0x11 {
                    out.pets[i] = Some(obj);
                } else {
                    out.animals[i] = Some(obj);
                }
            }
        }
    }
    Some(out)
}

/// Entradas de `0x598280`.
pub struct MarketIn<'a> {
    pub geom: &'a crate::geom::Geom,
    /// `+0x510`, `+0x514`, `+0x4D0`, `+0x4D4`.
    pub g: (i32, i32, i32, i32),
    pub clock: u32,
    /// Tabla `+0xC6A4` (punto del malabarista: `(columna, bandera)`).
    pub juggler_points: &'a [(u32, u32)],
    /// Tabla `+0xC6A8` (puestos).
    pub stall_points: &'a [(u32, u32)],
}

/// `0x598280`: coloca un malabarista (tipo 6, rumbo `0x60`) en un punto al azar de la tabla y un puesto (tipo 5) por entrada de la segunda, orientado hacia la primera
/// casilla vecina (rumbos `0x20`, `0x60`, `0xA0`, `0xE0`) cuya ficha de capa 1 es de clase 6; sin vecina de esa clase no hay puesto. Devuelve el nuevo `+0xC598`.
/// `class_at(col, fila)` es la clase de la ficha de capa 1 (`None` si no hay objeto).
pub fn place_market(i: &MarketIn, env: &mut dyn ManagerEnv, class_at: &dyn Fn(u32, u32) -> Option<u32>) -> u32 {
    let (g510, g514, g4d0, g4d4) = i.g;
    if !i.juggler_points.is_empty() {
        let r = env.rand();
        let (cx, flag) = i.juggler_points[(r as usize) % i.juggler_points.len()];
        let (x, y) = crate::manager::cell_position(i.g, cx as i32, flag);
        let obj = env.new_object(6);
        let link = env.next_link();
        env.register(obj, link);
        env.spawn(obj, x, y, 0x60, link, 0);
    }
    for &(cx, flag) in i.stall_points {
        let mut dir = 0x20i32;
        loop {
            let (mut x, mut y) = (cx, flag);
            i.geom.step(&mut x, &mut y, dir as u8);
            if class_at(x, y) == Some(6) {
                break;
            }
            dir += 0x40;
            if dir >= 0xE1 {
                break;
            }
        }
        if dir < 0x100 && dir <= 0xE0 {
            let (px, py) = if flag & 1 == 0 {
                (g510.wrapping_mul(cx as i32).wrapping_add(g4d0), g514.wrapping_add(1).wrapping_mul((flag >> 1) as i32).wrapping_sub(g4d4))
            } else {
                (g510.wrapping_mul((cx as i32).wrapping_add(1)), g514.wrapping_add(1).wrapping_mul((flag >> 1) as i32))
            };
            let sx = if dir == 0x60 || dir == 0xE0 { px.wrapping_add(8) } else { px.wrapping_sub(8) };
            let obj = env.new_object(5);
            let link = env.next_link();
            env.register(obj, link);
            env.spawn(obj, (sx as u32) << 16, (py.wrapping_add(g514).wrapping_sub(5) as u32) << 16, dir as u8, link, 0);
        }
    }
    let r = env.rand();
    let t = i.clock.wrapping_add((r % 60000 + 60000) >> 1);
    if t == 0 {
        1
    } else {
        t
    }
}
