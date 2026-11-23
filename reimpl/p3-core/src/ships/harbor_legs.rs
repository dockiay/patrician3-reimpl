//! J05 paso 4: tramos del recorrido de los barcos del puerto (`0x6148D0`, `0x614B50`, `0x615100`, `0x615400`, ...), verificados frente al x86
//! original con `oracle/x86_harbor_legs.py`. Cada tramo escribe en el barco local `+8 = 0`, `+0xA` (x), `+0xC = 0`, `+0xE` (y de salida), `+0x11`
//! (rumbo) y `+0x26` (etapa de velas) y devuelve en los 16 bits bajos `0xFFFF` cuando el barco queda colocado.
use super::harbor_list::{heading, sin_t, ShipList, SoundHost};

/// `0x65536 * (velocidad + 100) / 0x7C0` de la ranura (la longitud de ruta que `0x613B20` deja en `+0x2B40`).
fn unit(l: &ShipList, slot: u8) -> i32 {
    let v = l.speeds.get(usize::from(slot)).copied().unwrap_or(0);
    v.wrapping_add(100).wrapping_mul(0x10000) / 0x7C0
}

/// `(sin * d) >> 16` como palabra de 16 bits (`sar` del producto de 32 bits).
fn hi16(a: i32, b: i32) -> u16 {
    ((a.wrapping_mul(b)) >> 16) as u16
}

fn div_pow2(v: i32, bits: u32) -> i32 {
    let mask = (1i32 << bits) - 1;
    v.wrapping_add((v >> 31) & mask) >> bits
}

/// `(v + ((v >> 31) & 0x7F)) >> bits`: el redondeo de seis pasos que el compilador deja tras dividir por 128 (sesgo fijo `0x7F`).
fn rsh7(v: i32, bits: u32) -> i32 {
    v.wrapping_add((v >> 31) & 0x7F) >> bits
}

fn put(l: &mut ShipList, slot: u8, x: u16, y: u16, h: u8, sails: Option<u8>) {
    if let Some(Some(o)) = l.ships.get_mut(usize::from(slot)) {
        o[8..10].copy_from_slice(&0u16.to_le_bytes());
        o[0xA..0xC].copy_from_slice(&x.to_le_bytes());
        o[0xC..0xE].copy_from_slice(&0u16.to_le_bytes());
        o[0xE..0x10].copy_from_slice(&y.to_le_bytes());
        o[0x11] = h;
        if let Some(s) = sails {
            o[0x26] = s;
        }
    }
}

fn set_sails(l: &mut ShipList, slot: u8, s: u8) {
    if let Some(Some(o)) = l.ships.get_mut(usize::from(slot)) {
        o[0x26] = s;
    }
}

fn sails(l: &ShipList, slot: u8) -> u8 {
    l.ships
        .get(usize::from(slot))
        .and_then(|o| o.as_ref())
        .map_or(0, |o| o[0x26])
}

fn d(a: u16, b: u16) -> i32 {
    i32::from(a as i16) - i32::from(b as i16)
}

/// Desplazamiento y etapa de velas del tramo final `t < 0x80` (cuadrático) o `t >= 0x80` (lineal).
fn end_run(l: &ShipList, slot: u8, t: i16) -> (i32, u8) {
    let u = unit(l, slot);
    if t < 0x80 {
        let v = i32::from(t);
        let dist = rsh7(u.wrapping_mul(v).wrapping_mul(v), 24);
        let sails = (div_pow2(v.wrapping_mul(4).wrapping_add(0x7F), 7)) as u8;
        (dist, sails)
    } else {
        (u.wrapping_mul(i32::from(t) - 0x40) >> 16, 4)
    }
}

/// Corrección de rumbo de los tramos curvos: fracción `w / 256` del giro de `h1` a `h2` (truncada hacia cero).
fn turn(h1: u8, h2: u8, w: i32) -> u8 {
    let diff = h2.wrapping_sub(h1);
    if diff < 0x80 {
        div_pow2(i32::from(diff).wrapping_mul(w), 8) as u8
    } else {
        (div_pow2(i32::from(h1.wrapping_sub(h2)).wrapping_mul(w), 8) as u8).wrapping_neg()
    }
}

/// Pesos cuadráticos `((t + 0x80)^2 >> 2) * u >> 23` y `((0x80 - t)^2 >> 2) * u >> 23` de los tramos curvos.
fn weights(l: &ShipList, slot: u8, t: i16) -> (i32, i32, i32) {
    let u = unit(l, slot);
    let a = i32::from(t) + 0x80;
    let b = 0x80 - i32::from(t);
    let wa = rsh7((a.wrapping_mul(a) >> 2).wrapping_mul(u), 23);
    let wb = rsh7((b.wrapping_mul(b) >> 2).wrapping_mul(u), 23);
    (wa, wb, b)
}

/// `0x6148D0`: coloca el barco en el último punto, avanzando `t` desde `(x0, y0)` hacia `(x1, y1)`.
pub fn leg_final(l: &mut ShipList, slot: u8, x0: u16, y0: u16, x1: u16, y1: u16, t: u16) -> u32 {
    let h = heading(d(x1, x0), d(y1, y0));
    let (dist, s) = end_run(l, slot, t as i16);
    set_sails(l, slot, s);
    let x = hi16(sin_t(h), dist).wrapping_add(x0);
    let y = y0.wrapping_sub(hi16(sin_t(h.wrapping_add(0x40)), dist));
    put(l, slot, x, y, h, None);
    0xFFFF
}

/// `0x614B50`: salida del último tramo (curva con el segmento anterior mientras `t < 0x80`).
pub fn leg_depart_curve(l: &mut ShipList, slot: u8, p: [u16; 6], t: u16) -> u32 {
    let [x0, y0, x1, y1, x2, y2] = p;
    let h1 = heading(d(x1, x0), d(y1, y0));
    let t = t as i16;
    if t > 0x7F {
        let dist = unit(l, slot).wrapping_mul(i32::from(t)) >> 16;
        let x = hi16(sin_t(h1), dist).wrapping_add(x0);
        let y = y0.wrapping_sub(hi16(sin_t(h1.wrapping_add(0x40)), dist));
        put(l, slot, x, y, h1, Some(4));
        return 0xFFFF;
    }
    let (wa, wb, b) = weights(l, slot, t);
    let h2 = heading(d(x0, x2), d(y0, y2));
    let adj = turn(h1, h2, b);
    let x = hi16(sin_t(h1), wa).wrapping_sub(hi16(sin_t(h2), wb)).wrapping_add(x0);
    let y = hi16(sin_t(h2.wrapping_add(0x40)), wb)
        .wrapping_sub(hi16(sin_t(h1.wrapping_add(0x40)), wa))
        .wrapping_add(y0);
    put(l, slot, x, y, adj.wrapping_add(h1), Some(4));
    0xFFFF
}

/// `0x615100`: llegada al último punto desde `(x0, y0)` hacia `(x1, y1)` (recorre hacia atrás desde `(x1, y1)`); suena al pasar a la etapa 3 de velas.
pub fn leg_arrive(
    l: &mut ShipList,
    snd: &mut dyn SoundHost,
    slot: u8,
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16,
    t: u16,
) -> u32 {
    let h = heading(d(x1, x0), d(y1, y0));
    let t = t as i16;
    let u = unit(l, slot);
    let dist;
    if t < 0x80 {
        let v = i32::from(t);
        dist = rsh7(u.wrapping_mul(v).wrapping_mul(v), 24);
        let s = div_pow2(v.wrapping_mul(4), 7) as u8;
        if s == 3 {
            if let Some(Some(o)) = l.ships.get(usize::from(slot)) {
                if o[0x26] == 4 {
                    let x = i32::from(i16::from_le_bytes([o[0xA], o[0xB]]));
                    let y = i32::from(i16::from_le_bytes([o[0xE], o[0xF]]));
                    snd.ship_sound(x, y.wrapping_mul(0x16) / 0x22, 1);
                }
            }
        }
        set_sails(l, slot, s);
    } else {
        dist = u.wrapping_mul(i32::from(t) - 0x40) >> 16;
        set_sails(l, slot, 4);
    }
    let x = x1.wrapping_sub(hi16(sin_t(h), dist));
    let y = hi16(sin_t(h.wrapping_add(0x40)), dist).wrapping_add(y1);
    put(l, slot, x, y, h, None);
    0xFFFF
}

/// `0x615400`: llegada con curva (rumbo mezclado con el segmento siguiente mientras `t < 0x80`).
pub fn leg_arrive_curve(l: &mut ShipList, slot: u8, p: [u16; 6], t: u16) -> u32 {
    let [x0, y0, x1, y1, x2, y2] = p;
    let h1 = heading(d(x1, x0), d(y1, y0));
    let t = t as i16;
    if t >= 0x80 {
        let dist = unit(l, slot).wrapping_mul(i32::from(t)) >> 16;
        let x = x1.wrapping_sub(hi16(sin_t(h1), dist));
        let y = hi16(sin_t(h1.wrapping_add(0x40)), dist).wrapping_add(y1);
        put(l, slot, x, y, h1, Some(4));
        return 0xFFFF;
    }
    let (wa, wb, b) = weights(l, slot, t);
    let h2 = heading(d(x2, x1), d(y2, y1));
    let adj = turn(h1, h2, b);
    let x = hi16(sin_t(h2), wb).wrapping_sub(hi16(sin_t(h1), wa)).wrapping_add(x1);
    let y = hi16(sin_t(h1.wrapping_add(0x40)), wa)
        .wrapping_sub(hi16(sin_t(h2.wrapping_add(0x40)), wb))
        .wrapping_add(y1);
    put(l, slot, x, y, adj.wrapping_add(h1), Some(4));
    0xFFFF
}

/// Longitud del tramo `(x0, y0) -> (x1, y1)` (`ftol(sqrt(dx² + dy²))`, con la suma en 32 bits como el original) y unidad de la ranura.
fn span(l: &ShipList, slot: u8, x0: u16, y0: u16, x1: u16, y1: u16) -> (i32, i32) {
    let dx = d(x1, x0);
    let dy = d(y1, y0);
    let sum = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    let r = f64::from(sum).sqrt();
    let len = if r.is_nan() || r >= 2_147_483_648.0 {
        i32::MIN
    } else {
        r as i32
    };
    (len, unit(l, slot))
}

fn div(a: i32, b: i32) -> i32 {
    a.checked_div(b).unwrap_or(0)
}

/// Colocación recta desde `(x0, y0)` con rumbo `h` y distancia `dist`.
fn straight(l: &mut ShipList, slot: u8, x0: u16, y0: u16, h: u8, dist: i32, s: Option<u8>) {
    let x = hi16(sin_t(h), dist).wrapping_add(x0);
    let y = y0.wrapping_sub(hi16(sin_t(h.wrapping_add(0x40)), dist));
    put(l, slot, x, y, h, s);
}

/// Curva alrededor de `(cx, cy)`: `h1` entra, `h2` sale; `back` invierte los signos (curva de salida frente a llegada).
fn bend(l: &mut ShipList, slot: u8, cx: u16, cy: u16, h1: u8, h2: u8, wa: i32, wb: i32, w: i32, arrive: bool) {
    let adj = turn(h1, h2, w);
    let (x, y) = if arrive {
        (
            hi16(sin_t(h2), wb).wrapping_sub(hi16(sin_t(h1), wa)).wrapping_add(cx),
            hi16(sin_t(h1.wrapping_add(0x40)), wa)
                .wrapping_sub(hi16(sin_t(h2.wrapping_add(0x40)), wb))
                .wrapping_add(cy),
        )
    } else {
        (
            hi16(sin_t(h1), wa).wrapping_sub(hi16(sin_t(h2), wb)).wrapping_add(cx),
            hi16(sin_t(h2.wrapping_add(0x40)), wb)
                .wrapping_sub(hi16(sin_t(h1.wrapping_add(0x40)), wa))
                .wrapping_add(cy),
        )
    };
    put(l, slot, x, y, adj.wrapping_add(h1), Some(4));
}

/// Pesos cuadráticos de `t` (en palabras de 16 bits) y complemento `0x80 - t`.
fn quad(u: i32, t: i32) -> (i32, i32, i32) {
    let a = t + 0x80;
    let b = 0x80 - t;
    (
        rsh7((a.wrapping_mul(a) >> 2).wrapping_mul(u), 23),
        rsh7((b.wrapping_mul(b) >> 2).wrapping_mul(u), 23),
        b,
    )
}

/// `0x6159A0`: tramo recto de `(x0, y0)` a `(x1, y1)`; si el barco aún no llega devuelve lo que sobra (`t - longitud`), sin colocarlo.
pub fn leg_straight(l: &mut ShipList, slot: u8, x0: u16, y0: u16, x1: u16, y1: u16, t: u16) -> u32 {
    let h = heading(d(x1, x0), d(y1, y0));
    let (len, u) = span(l, slot, x0, y0, x1, y1);
    let t = i32::from(t as i16);
    let total = div(len.wrapping_shl(16), u).wrapping_add(0x80);
    if total < t {
        return (t - total) as u32;
    }
    let (dist, s);
    if t < 0x80 {
        dist = rsh7(t.wrapping_mul(t).wrapping_mul(u), 24);
        s = rsh7(t.wrapping_mul(4).wrapping_add(0x7F), 7) as u8;
    } else if total - t < 0x80 {
        dist = len.wrapping_sub(rsh7(t.wrapping_mul(t).wrapping_mul(u), 24));
        s = rsh7((total - t).wrapping_mul(4), 7) as u8;
    } else {
        dist = (t - 0x40).wrapping_mul(u) >> 16;
        s = 4;
    }
    straight(l, slot, x0, y0, h, dist, Some(s));
    0xFFFF
}

/// `0x615CD0`: arco que termina en `(x1, y1)` y sigue hacia `(x2, y2)`; `forward` (marca 1) recorre desde el principio y `0` desde el final.
pub fn leg_arc_in(l: &mut ShipList, slot: u8, p: [u16; 6], t: u32, forward: bool) -> u32 {
    let [x0, y0, x1, y1, x2, y2] = p;
    let (len, u) = span(l, slot, x0, y0, x1, y1);
    let total = div(len.wrapping_shl(16), u).wrapping_add(0x40);
    let l16 = total & 0xFFFF;
    if l16 < i32::from(t as i16) {
        return (i32::from(t as i16) - l16) as u32;
    }
    let h1 = heading(d(x1, x0), d(y1, y0));
    let t = if forward { t } else { (total as u32).wrapping_sub(t) };
    let tt = (t & 0xFFFF) as i32;
    let rem = l16 - tt;
    if rem < 0x80 {
        let (wa, wb, b) = quad(u, rem);
        let h2 = heading(d(x2, x1), d(y2, y1));
        bend(l, slot, x1, y1, h1, h2, wa, wb, b, true);
        return 0xFFFF;
    }
    let (dist, s);
    if tt < 0x80 {
        dist = rsh7(tt.wrapping_mul(tt).wrapping_mul(u), 24);
        s = (tt.wrapping_mul(4).wrapping_add(0x7F) >> 7) as u8;
    } else {
        dist = (tt - 0x40).wrapping_mul(u) >> 16;
        s = 4;
    }
    straight(l, slot, x0, y0, h1, dist, Some(s));
    0xFFFF
}

/// `0x616330`: arco de salida desde `(x0, y0)` con un giro hacia `(x2, y2)` (el segmento anterior); suena al pasar a la etapa 3 de velas.
pub fn leg_arc_out(l: &mut ShipList, snd: &mut dyn SoundHost, slot: u8, p: [u16; 6], t: u32, forward: bool) -> u32 {
    let [x0, y0, x1, y1, x2, y2] = p;
    let (len, u) = span(l, slot, x0, y0, x1, y1);
    let total = div(len.wrapping_shl(16), u).wrapping_add(0x40);
    let l16 = total & 0xFFFF;
    if l16 < i32::from(t as i16) {
        return (i32::from(t as i16) - l16) as u32;
    }
    let h1 = heading(d(x1, x0), d(y1, y0));
    let t = if forward { t } else { (total as u32).wrapping_sub(t) };
    let tt = (t & 0xFFFF) as i32;
    if tt >= 0x80 {
        let rem = l16 - tt;
        let dist;
        if rem < 0x80 {
            dist = ((l16 - 0x40).wrapping_mul(u) >> 16).wrapping_sub(rsh7(u.wrapping_mul(rem).wrapping_mul(rem), 24));
            let s = (rsh7(rem.wrapping_mul(4), 7)) as u8;
            if s == 3 {
                if let Some(Some(o)) = l.ships.get(usize::from(slot)) {
                    if o[0x26] == 4 {
                        let x = i32::from(i16::from_le_bytes([o[0xA], o[0xB]]));
                        let y = i32::from(i16::from_le_bytes([o[0xE], o[0xF]]));
                        snd.ship_sound(x, y.wrapping_mul(0x16) / 0x22, 1);
                    }
                }
            }
            set_sails(l, slot, s);
        } else {
            dist = u.wrapping_mul(tt) >> 16;
            set_sails(l, slot, 4);
        }
        straight(l, slot, x0, y0, h1, dist, None);
        return 0xFFFF;
    }
    let (wa, wb, b) = quad(u, tt);
    let h2 = heading(d(x0, x2), d(y0, y2));
    bend(l, slot, x0, y0, h1, h2, wa, wb, b, false);
    0xFFFF
}

/// `0x6169E0`: tramo con curva en ambos extremos: giro inicial hacia el punto anterior `(x2, y2)` y giro final hacia `(x3, y3)`.
pub fn leg_both(l: &mut ShipList, slot: u8, p: [u16; 8], t: u32, forward: bool) -> u32 {
    let [x0, y0, x1, y1, x2, y2, x3, y3] = p;
    let (len, u) = span(l, slot, x0, y0, x1, y1);
    let total = div(len.wrapping_shl(16), u);
    let l16 = total & 0xFFFF;
    if l16 < i32::from(t as i16) {
        return (i32::from(t as i16) - l16) as u32;
    }
    let h1 = heading(d(x1, x0), d(y1, y0));
    let t = if forward { t } else { (total as u32).wrapping_sub(t) };
    let tt = (t & 0xFFFF) as i32;
    set_sails(l, slot, 4);
    if tt < 0x80 {
        let (wa, wb, b) = quad(u, tt);
        let h2 = heading(d(x0, x2), d(y0, y2));
        bend(l, slot, x0, y0, h1, h2, wa, wb, b, false);
        return 0xFFFF;
    }
    let rem = l16 - tt;
    if rem < 0x80 {
        let (wa, wb, b) = quad(u, rem);
        let h2 = heading(d(x3, x1), d(y3, y1));
        bend(l, slot, x1, y1, h1, h2, wa, wb, b, true);
        return 0xFFFF;
    }
    straight(l, slot, x0, y0, h1, u.wrapping_mul(tt) >> 16, None);
    0xFFFF
}

/// `0x617380` (`local_map_route_place_ship_at_distance`): recorre la ruta de la ranura (`xs`, `ys` y marcas `fl` de `n` puntos) hasta `distance` y deja la
/// posición del barco. Marca 2 = punto de curva, 1 = extremo. `reverse` recorre la ruta hacia atrás. `0x6148C0` (que solo devuelve `t`) no se porta.
pub fn place_on_route(
    l: &mut ShipList,
    snd: &mut dyn SoundHost,
    slot: u8,
    xs: &[u16],
    ys: &[u16],
    fl: &[u8],
    n: u8,
    distance: u16,
    reverse: bool,
) {
    let n = usize::from(n);
    let mut t = u32::from(distance);
    let neg = |t: u32| (t as u16 as i16) < 0;
    let (x, y) = (|k: usize| xs[k], |k: usize| ys[k]);
    if n >= 2 + 2 {
        let last = n - 2;
        let mut i = 1usize;
        loop {
            let (c, nx) = (fl[i], fl[i + 1]);
            t = if c == 2 {
                if nx == 2 {
                    if i == 1 && neg(t) {
                        return;
                    }
                    if !reverse {
                        leg_straight(l, slot, x(i + 1), y(i + 1), x(i), y(i), t as u16)
                    } else {
                        leg_straight(l, slot, x(i), y(i), x(i + 1), y(i + 1), t as u16)
                    }
                } else {
                    if neg(t) {
                        return;
                    }
                    if !reverse {
                        leg_arc_out(
                            l,
                            snd,
                            slot,
                            [x(i + 1), y(i + 1), x(i), y(i), x(i + 2), y(i + 2)],
                            t,
                            false,
                        )
                    } else {
                        leg_arc_in(l, slot, [x(i), y(i), x(i + 1), y(i + 1), x(i + 2), y(i + 2)], t, true)
                    }
                }
            } else if nx == 2 {
                if !reverse {
                    leg_arc_in(l, slot, [x(i + 1), y(i + 1), x(i), y(i), x(i - 1), y(i - 1)], t, false)
                } else {
                    leg_arc_out(
                        l,
                        snd,
                        slot,
                        [x(i), y(i), x(i + 1), y(i + 1), x(i - 1), y(i - 1)],
                        t,
                        true,
                    )
                }
            } else if !reverse {
                leg_both(
                    l,
                    slot,
                    [x(i + 1), y(i + 1), x(i), y(i), x(i + 2), y(i + 2), x(i - 1), y(i - 1)],
                    t,
                    false,
                )
            } else {
                leg_both(
                    l,
                    slot,
                    [x(i), y(i), x(i + 1), y(i + 1), x(i - 1), y(i - 1), x(i + 2), y(i + 2)],
                    t,
                    true,
                )
            };
            if neg(t) {
                return;
            }
            i += 1;
            if i >= last {
                break;
            }
        }
    }
    if n >= 1 && fl[n - 1] == 1 {
        let (a, b) = (n - 1, n - 2);
        if !reverse {
            if n < 4 {
                if neg(t) {
                    return;
                }
                leg_arrive(l, snd, slot, x(a), y(a), x(b), y(b), t as u16);
            } else {
                leg_arrive_curve(l, slot, [x(a), y(a), x(b), y(b), x(n - 3), y(n - 3)], t as u16);
            }
        } else if n >= 4 {
            leg_depart_curve(l, slot, [x(b), y(b), x(a), y(a), x(n - 3), y(n - 3)], t as u16);
        } else {
            if neg(t) {
                return;
            }
            leg_final(l, slot, x(b), y(b), x(a), y(a), t as u16);
        }
    }
}
