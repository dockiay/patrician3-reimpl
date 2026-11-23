//! Paso de los animales del pueblo (tipo 16): `0x57E840`.
//!
//! Misma estructura que el paso de perros y gato (`pets::step`) con otras constantes: velocidad ×2 en el estado 0x0B, huellas con divisor 0x77, efectos con
//! offset diagonal, destino fijado directamente y probabilidades distintas. Se reutilizan `PetState`, `PetCtx` y `PetEnv` (el campo `s29` no se usa).
use crate::pets::{PetCtx, PetEnv, PetState};

fn footprint(h: u8, r3: u32) -> (i32, i32) {
    let s = |m: i32, d: i32| (r3 as i32).wrapping_mul(m).wrapping_div(d);
    let u = |m: u32, d: u32| (r3.wrapping_mul(m) / d) as i32;
    match h {
        0x00 => (0, s(-0xb, 0x11)),
        0x20 => (u(5, 7), s(-0x37, 0x77)),
        0x40 => (r3 as i32, 0),
        0x60 => (u(5, 7), u(0x37, 0x77)),
        0x80 => (0, u(0xb, 0x11)),
        0xA0 => (s(-5, 7), u(0x37, 0x77)),
        0xC0 => ((r3 as i32).wrapping_neg(), 0),
        0xE0 => (s(-5, 7), s(-0x37, 0x77)),
        _ => (0, 0),
    }
}

/// `0x57E840`: avanza un paso del animal; devuelve `true` si el paso es válido. Los rumbos han de ser múltiplos de `0x20`.
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
            u = u.wrapping_mul(2);
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
            let (xo, yo) = footprint(p.heading, r3);
            let (a6, a8) = (xo >> 2, yo >> 2);
            let two = (r3.wrapping_mul(2)) as i32;
            let flat = row[2] == 0;
            let f = u32::from(p.frame);
            for i in 0..4 {
                if f == row[4 + i] {
                    let (x, y, k) = if flat {
                        (pxi.wrapping_add(xo), yo.wrapping_add(pyi).wrapping_add(two), 0xC5)
                    } else {
                        (pxi.wrapping_add(xo).wrapping_add(a8), pyi.wrapping_add(yo).wrapping_add(two.wrapping_sub(a6)), 0xC3)
                    };
                    env.effect(k, x, y, p.heading);
                }
                if f == row[8 + i] {
                    let (x, y, k) = if flat {
                        (pxi.wrapping_sub(xo), pyi.wrapping_add(two).wrapping_sub(yo), 0xC5)
                    } else {
                        (a8.wrapping_sub(xo).wrapping_add(pxi), two.wrapping_sub(a6).wrapping_add(pyi).wrapping_sub(yo), 0xC3)
                    };
                    env.effect(k, x, y, p.heading);
                }
                if f == row[12 + i] {
                    let (x, y, k) = if flat {
                        (pxi.wrapping_add(xo), yo.wrapping_add(pyi).wrapping_add(two), 0xC6)
                    } else {
                        (pxi.wrapping_sub(a8).wrapping_add(xo), pyi.wrapping_add(a6).wrapping_add(yo).wrapping_add(two), 0xC4)
                    };
                    env.effect(k, x, y, p.heading);
                }
                if f == row[16 + i] {
                    let (x, y, k) = if flat {
                        (pxi.wrapping_sub(xo), pyi.wrapping_add(two).wrapping_sub(yo), 0xC6)
                    } else {
                        (pxi.wrapping_sub(a8).wrapping_sub(xo), a6.wrapping_add(two).wrapping_add(pyi).wrapping_sub(yo), 0xC4)
                    };
                    env.effect(k, x, y, p.heading);
                }
            }
        }
    }
    let mut heading = p.heading;
    let mut blocked = false;
    if old_counter != p.counter {
        if !env.try_step(heading) {
            if p.state != 2 {
                frame = env.select();
            }
            env.set_target(2);
            p.target = 2;
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
            let mut b9 = 1u32;
            let cond = |env: &mut dyn PetEnv, id: u8| -> (u8, bool) {
                let i = env.find(id);
                (i, i != 0xFF && (c.rec)(i) < c.counts[dir])
            };
            match st {
                2 => {
                    let (_, a) = cond(env, 0x0B);
                    if a {
                        b9 = 2;
                    }
                    let (i3, bc) = cond(env, 3);
                    if bc {
                        b9 += 8;
                    }
                    let r = env.rand();
                    let v = r as i32 % b9 as i32;
                    let rc = (c.rec)(i3) < c.counts[dir];
                    if v == 0 || rc {
                        if v == 0 {
                            if p.counter & 1 != 0 && env.try_step(p.heading.wrapping_sub(0x20)) {
                                heading = heading.wrapping_sub(0x20);
                            } else if env.try_step(p.heading.wrapping_add(0x20)) {
                                heading = heading.wrapping_add(0x20);
                            }
                        } else if v != 9 {
                            p.target = 3;
                        } else {
                            p.target = 0x0B;
                        }
                    } else {
                        p.target = 0x0B;
                    }
                }
                3 | 0x43 | 0x63 | 0x85 | 0x8E | 0xC5 | 0xEE => {
                    let r = env.rand();
                    if r % 5 != 0 {
                        p.target = 3;
                    } else {
                        let (_, a) = cond(env, 0x0B);
                        if a {
                            b9 = 2;
                        }
                        let r = env.rand();
                        p.target = if r as i32 % b9 as i32 == 0 { 2 } else { 0x0B };
                    }
                }
                0x0B | 0x89 | 0x8A => {
                    let (_, a) = cond(env, 3);
                    if a {
                        b9 = 9;
                    }
                    let r = env.rand();
                    p.target = if r as i32 % b9 as i32 == 0 { 2 } else { 3 };
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
