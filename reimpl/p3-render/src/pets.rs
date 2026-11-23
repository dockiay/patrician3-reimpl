//! Perros y gato de la ciudad (tipo 17, J02). Paridad x86 por función en `tests/x86_pet_anim.rs`.

/// `0x57CE90`: transición cuando termina la animación (`estado` = `objeto+0x28`, `destino` = `objeto+0x3A`). `find(id)` es la búsqueda `0x5576C0` en el juego de
/// sprites de la variante (`true` si el estado `id` existe). Devuelve el nuevo destino que fija `0x5576B0`, si lo hay.
pub fn anim_done(state: u8, target: u8, find: &mut dyn FnMut(u8) -> bool) -> Option<u8> {
    // 0x57CEDE: de 0x89 si existe, si no 0xB
    fn cede(find: &mut dyn FnMut(u8) -> bool) -> u8 {
        if find(0x89) {
            0x89
        } else {
            0x0B
        }
    }
    // 0x57CEBC: 0x85 si existe, si no 3
    fn cebc(find: &mut dyn FnMut(u8) -> bool) -> u8 {
        if find(0x85) {
            0x85
        } else {
            3
        }
    }
    match state {
        2 => match target {
            3 => Some(cebc(find)),
            0x0B => Some(cede(find)),
            _ => None,
        },
        3 => {
            if target == 2 {
                return Some(if find(0x87) { 0x87 } else { 2 });
            }
            if target != 0x0B {
                return None;
            }
            if find(0x87) {
                return Some(0x87);
            }
            Some(cede(find))
        }
        0x0B => {
            if target == 2 {
                return Some(if find(0x8A) { 0x8A } else { 2 });
            }
            if target != 3 {
                return None;
            }
            if find(0x8A) {
                return Some(0x8A);
            }
            Some(cebc(find))
        }
        0x85 => Some(3),
        0x87 => match target {
            2 => Some(2),
            3 => Some(0x85),
            _ => Some(cede(find)),
        },
        0x89 => Some(0x0B),
        0x8A => match target {
            2 => Some(2),
            3 => Some(cebc(find)),
            _ => None,
        },
        _ => None,
    }
}

/// Estado del perro o gato que lee y escribe `0x57C240`.
#[derive(Clone, Debug, Default)]
pub struct PetState {
    pub x: u32,
    pub y: u32,
    pub px: i16,
    pub py: i16,
    pub heading: u8,
    pub frame: u8,
    /// `+0x28`
    pub state: u8,
    /// `+0x29`
    pub s29: u8,
    /// `+0x38`
    pub counter: u8,
    /// `+0x3A`
    pub target: u8,
}

/// Datos fijos del paso: escena, juego de sprites y fila de la tabla por variante (`DAT_006E2E9C + variante * 0x50`, 20 palabras).
pub struct PetCtx<'a> {
    /// `escena+0xC594`
    pub flag: bool,
    /// `escena+0x414`
    pub budget: u32,
    /// `+0xC650`, `+0xC654`, `+0x2C`, `+0x30`
    pub view: (u32, u32, u32, u32),
    /// `+0xC31C`, `+0xC320`
    pub size: (u32, u32),
    /// `juego+0xC`
    pub speed: u32,
    pub row: &'a [u32; 20],
    /// `juego+0x20`: recuento de fotogramas por dirección.
    pub counts: &'a [u32],
    /// `juego+0x2C`: primer campo del registro de estado `i`.
    pub rec: &'a dyn Fn(u8) -> u32,
}

/// Ayudantes del paso, cubiertos por otras pruebas.
pub trait PetEnv {
    /// `0x55A2C0(figura, rumbo, 1, 1)`.
    fn try_step(&mut self, heading: u8) -> bool;
    /// `0x557340`: devuelve `(fotograma, contador +0x38)`.
    fn advance(&mut self) -> (u8, u8);
    /// `0x557730(juego, 2)`: fotograma inicial.
    fn select(&mut self) -> u8;
    /// `0x5576B0(destino)`.
    fn set_target(&mut self, t: u8);
    /// `0x5576C0(juego, id)`: índice de estado o `0xFF`.
    fn find(&mut self, id: u8) -> u8;
    /// `0x55B030(clase, x, y, rumbo, 0)`: efecto de pisada.
    fn effect(&mut self, kind: u32, x: i32, y: i32, heading: u8);
    fn rand(&mut self) -> u32;
    /// `0x5595F0` y `0x559790`: sacar de la rejilla de ocupación y volver a meter.
    fn remove(&mut self);
    fn add(&mut self);
}

fn footprint(h: u8, r3: u32) -> (i32, i32) {
    let s = |m: i32, d: i32| (r3 as i32).wrapping_mul(m).wrapping_div(d);
    let u = |m: u32, d: u32| (r3.wrapping_mul(m) / d) as i32;
    match h {
        0x00 => (0, s(-0xb, 0x11)),
        0x20 => (u(5, 7), s(-0x37, 0x4d)),
        0x40 => (r3 as i32, 0),
        0x60 => (u(5, 7), u(0x37, 0x4d)),
        0x80 => (0, u(0xb, 0x11)),
        0xA0 => (s(-5, 7), u(0x37, 0x4d)),
        0xC0 => ((r3 as i32).wrapping_neg(), 0),
        0xE0 => (s(-5, 7), s(-0x37, 0x4d)),
        _ => (0, 0),
    }
}

/// `0x57C240`: avanza un paso del perro o gato; devuelve `true` si el paso es válido. Los rumbos han de ser múltiplos de `0x20`.
pub fn step(p: &mut PetState, c: &PetCtx, env: &mut dyn PetEnv) -> bool {
    if !c.flag {
        return false;
    }
    let (mut nx, mut ny) = (p.x, p.y);
    let st = p.state;
    if matches!(st, 2 | 0x0B | 0x89 | 0x8A) && env.try_step(p.heading) {
        let mut u = c.budget.min(1000);
        u = c.speed.wrapping_mul(u).wrapping_mul(0x1000) / 0x271;
        if st == 0x0B {
            u = u.wrapping_mul(3);
        }
        let quarter = |i: i32| (i.wrapping_mul(2)).wrapping_div(0x22) as u32;
        match p.heading {
            0x00 => ny = ny.wrapping_add(quarter((u as i32).wrapping_mul(-0xb))),
            0x80 => ny = ny.wrapping_add(u.wrapping_mul(0x16) / 0x22),
            0x40 => nx = nx.wrapping_add(u),
            0xC0 => nx = nx.wrapping_sub(u),
            h => {
                let i6 = (u.wrapping_mul(0xb5) as i32) >> 8;
                match h {
                    0x20 | 0xE0 => {
                        let i7 = if h == 0x20 { i6 } else { i6.wrapping_neg() };
                        nx = nx.wrapping_add(i7 as u32);
                        ny = ny.wrapping_add(i6.wrapping_mul(-0x16).wrapping_div(0x22) as u32);
                    }
                    0x60 | 0xA0 => {
                        nx = if h == 0x60 { nx.wrapping_add(i6 as u32) } else { nx.wrapping_sub(i6 as u32) };
                        ny = ny.wrapping_add(quarter(i6.wrapping_mul(0xb)));
                    }
                    _ => {}
                }
            }
        }
    }
    let old_counter = p.counter;
    let (new_frame, counter) = env.advance();
    p.counter = counter;
    let mut frame = new_frame;
    let row = c.row;
    let (pxi, pyi) = (i32::from(p.px), i32::from(p.py));
    if frame != p.frame {
        let (v650, v654, s2c, s30) = c.view;
        let (pxu, pyu) = (pxi as u32, pyi as u32);
        if v650 <= pxu.wrapping_add(200) && v654 <= pyu.wrapping_add(200) && pxu <= v650.wrapping_add(200).wrapping_add(s2c) && pyu <= s30.wrapping_add(200).wrapping_add(v654) {
            let r3 = row[3];
            let (ox, oy) = footprint(p.heading, r3);
            let base_y = (r3 as i32).wrapping_add(pyi);
            let (ka, kb) = if row[2] == 0 { (0xC5, 0xC6) } else { (0xC3, 0xC4) };
            for i in 0..4 {
                let f = u32::from(p.frame);
                if f == row[4 + i] {
                    env.effect(ka, pxi.wrapping_add(ox), base_y.wrapping_add(oy), p.heading);
                }
                if f == row[8 + i] {
                    env.effect(ka, pxi.wrapping_sub(ox), base_y.wrapping_sub(oy), p.heading);
                }
                if f == row[12 + i] {
                    env.effect(kb, pxi.wrapping_add(ox), base_y.wrapping_add(oy), p.heading);
                }
                if f == row[16 + i] {
                    env.effect(kb, pxi.wrapping_sub(ox), base_y.wrapping_sub(oy), p.heading);
                }
            }
        }
    }
    let mut heading = p.heading;
    let mut blocked = false;
    if old_counter != p.counter {
        if !env.try_step(heading) {
            if p.state != 2 && p.state != 0x0B {
                frame = env.select();
            }
            if p.s29 != 0x0B {
                env.set_target(2);
            }
            if p.target != 0x0B {
                p.target = 2;
            }
            let r = env.rand();
            p.counter = (row[0] as u8).wrapping_sub((r % row[1].max(1)) as u8);
            let mut b = 0x20u8;
            loop {
                if env.try_step(p.heading.wrapping_add(b)) || env.try_step(p.heading.wrapping_sub(b)) {
                    break;
                }
                b = b.wrapping_add(0x20);
                if b >= 0x81 {
                    blocked = true;
                    break;
                }
            }
            if !env.try_step(p.heading.wrapping_add(b)) {
                heading = heading.wrapping_sub(0x20);
            } else if env.try_step(p.heading.wrapping_sub(b)) {
                let r = env.rand() & 1;
                heading = heading.wrapping_add((r as u8).wrapping_mul(0x40)).wrapping_sub(0x20);
            } else {
                heading = heading.wrapping_add(0x20);
            }
        } else if u32::from(p.counter) > row[0] {
            let st = p.state;
            let r = env.rand();
            p.counter = (row[0] as u8).wrapping_sub((r % row[1].max(1)) as u8);
            let dir = usize::from(p.heading >> 5);
            let mut b8 = 1u32;
            let cond = |env: &mut dyn PetEnv, id: u8| -> (u8, bool) {
                let i = env.find(id);
                (i, i != 0xFF && (c.rec)(i) < c.counts[dir])
            };
            let mut set_end = false;
            match st {
                2 => {
                    let (_, a) = cond(env, 0x0B);
                    if a {
                        b8 = 2;
                    }
                    let (i3, bcond) = cond(env, 3);
                    if bcond {
                        b8 += 2;
                    }
                    let r = env.rand();
                    let v = (r as i32 % b8 as i32) as u8;
                    if v == 0 || (c.rec)(i3) < c.counts[dir] {
                        if v == 0 {
                            if p.counter & 1 != 0 && env.try_step(p.heading.wrapping_sub(0x20)) {
                                heading = heading.wrapping_sub(0x20);
                            } else if !env.try_step(p.heading.wrapping_add(0x20)) {
                                // sin cambio
                            } else {
                                heading = heading.wrapping_add(0x20);
                            }
                            set_end = true;
                        } else if v != 3 {
                            p.target = 3;
                            set_end = true;
                        }
                    }
                    if !set_end {
                        p.target = 0x0B;
                    }
                }
                3 | 0x85 | 0x87 => {
                    let (_, a) = cond(env, 0x0B);
                    if a {
                        b8 = 2;
                    }
                    let r = env.rand();
                    if (r as i32 % b8 as i32) as u8 == 0 {
                        p.target = 2;
                    } else {
                        p.target = 0x0B;
                    }
                }
                0x0B | 0x89 | 0x8A => {
                    let (_, a) = cond(env, 3);
                    if a {
                        b8 = 3;
                    }
                    let r = env.rand();
                    p.target = if (r as i32 % b8 as i32) as u8 == 0 { 2 } else { 3 };
                }
                _ => {}
            }
        }
    }
    let (nxs, nys) = (nx >> 16, ny >> 16);
    if nxs == pxi as u32 && nys == pyi as u32 && frame == p.frame && heading == p.heading {
        p.x = nx;
        p.y = ny;
        p.px = (nx >> 16) as i16;
        p.py = (ny >> 16) as i16;
        return true;
    }
    if nxs <= c.size.0 && nys <= c.size.1 && !blocked {
        env.remove();
        p.x = nx;
        p.y = ny;
        p.px = (nx >> 16) as i16;
        p.py = (ny >> 16) as i16;
        p.frame = frame;
        p.heading = heading;
        env.add();
        return true;
    }
    false
}
