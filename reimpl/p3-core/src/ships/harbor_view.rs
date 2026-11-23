//! Vista del barco local del puerto: `local_ship_update_view_entities` (`0x62018B`).
//!
//! Observado y probado en emulador (`oracle/x86_harbor_view.py`, ~3.000 casos con el x86 real). Según la marca `+0x12A`:
//! - **bit 4 (hundiéndose o ardiendo)**: crea dos emisores (`+0x15C` humo, `+0x158` llamas; códigos `dirección << 8 | 4..7` y
//!   `| 0x40..0x43` según el casco), los recoloca mientras su fotograma sea < 0x14, y en cada fotograma nuevo del segundo emisor
//!   dibuja anillos de píxeles (`0x5986A0`) o lanza restos (fotograma 0xC); hasta el fotograma 0xA suelta dos humos al viento.
//!   Solo un barco a la vez (`DAT_006E59CD`) reserva el aviso `+0x160`.
//! - **bits 2/1 sin bit 4**: da de baja la entidad de vista `+0x134`.
//! - **resto**: sonido del barco ajeno (bit 0x20, lista 60100 de `sound.ini`, `0x595B00`), alta de la entidad (`0x590610`, clase 0xD)
//!   o su pose (`0x56B140`, con histéresis de 3 en el rumbo) y, con la opción `escena+0xC594` y vida <= 2/3, humo de daño al viento
//!   cada `vida * 1200 / máx + 200` ms.
//!
//! Las llamadas externas se sustituyen por [`ViewHost`]. Las coordenadas verticales se reducen a la vista con `* 11 / 17` (o `22 / 34`)
//! exactamente como el compilador (multiplicación por `0x78787879`).

use super::harbor_list::sin_t;

/// Campos del barco local que lee y escribe `0x62018B`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocalShipView {
    /// `+4`: barco del mundo.
    pub world: u32,
    /// `+8`, `+0xC`: posición 16.16; `+0xA`, `+0xE` son sus partes altas.
    pub x: u32,
    pub y: u32,
    /// `+0x11`
    pub heading: u8,
    /// `+0x26`
    pub sails: u8,
    /// `+0x12A`
    pub flag: u8,
    /// `+0x134`: entidad de vista (0 = ninguna).
    pub entity: u32,
    /// `+0x13D`: índice en la tabla de propietarios `+0x35A` del objeto `+0x12C`.
    pub owner_slot: u8,
    /// `+0x14B`: el sonido del barco ya sonó.
    pub sounded: u8,
    /// `+0x14C`, `+0x150`: próximo humo de daño y el anterior.
    pub next_smoke: u32,
    pub last_smoke: u32,
    /// `+0x154`: último fotograma visto del emisor de llamas (0xFF tras crearlo).
    pub phase: u8,
    /// `+0x158`, `+0x15C`: emisores de llamas y humo del hundimiento.
    pub flames: u32,
    pub smoke: u32,
    /// `+0x160`: este barco tiene el aviso de hundimiento.
    pub warned: u8,
}

impl LocalShipView {
    fn px(&self) -> i32 {
        i32::from((self.x >> 16) as i16)
    }
    fn py(&self) -> i32 {
        i32::from((self.y >> 16) as i16)
    }
}

/// Datos del mundo y de la escena.
pub struct ViewCtx<'a> {
    /// `DAT_006DCCF8`
    pub clock: u32,
    /// `DAT_006DFC14`
    pub player: u32,
    /// Barco del mundo: tipo de casco `+0xE`, vida máxima `+0x14` y actual `+0x18`.
    pub hull: u8,
    pub max_health: u32,
    pub health: u32,
    /// Tabla de propietarios `objeto(+0x12C) + 0x35A` (palabras).
    pub owners: &'a [u16],
    /// Viento del objeto `+0x12C`: dirección `+0x670` y fuerza `+0x671`.
    pub wind_dir: u8,
    pub wind: u8,
    /// `escena+0xC594`: humo de daño activo.
    pub smoke_option: bool,
    /// `DAT_00677929`: último argumento de los emisores (32 en el EXE).
    pub emitter_arg: u8,
}

/// Lo que la escena hace por el barco. Las entidades se identifican por el valor que devolvió `create`.
pub trait ViewHost {
    fn rand(&mut self) -> i32;
    /// `0x590610(x, y, clase, código, argumento, 0, 0)` de la escena.
    fn create(&mut self, x: i32, y: i32, class: u32, code: u32, arg: u8) -> u32;
    /// Escribe `+0x30` (byte) de la entidad creada.
    fn set_alpha(&mut self, entity: u32, v: u8);
    /// Escribe `+0x40`/`+0x44` (deriva al viento) de la entidad creada.
    fn set_drift(&mut self, entity: u32, dx: i32, dy: i32);
    /// Fotograma `(entidad + 8) + 0xD` del emisor.
    fn frame(&mut self, entity: u32) -> u8;
    /// Rumbo `+0x14` de la entidad de vista.
    fn entity_heading(&mut self, entity: u32) -> u8;
    /// Método +0x48 del emisor: `(x, y, 0)`.
    fn emit(&mut self, entity: u32, x: i32, y: i32);
    /// Baja de la entidad (`escena+0x3F8`, método +4).
    fn release(&mut self, entity: u32);
    /// `0x56B140(x, y, rumbo, vela)` sobre la entidad.
    fn pose(&mut self, entity: u32, x: i16, y: i32, heading: u8, sails: u8);
    /// `0x595B00(x, y, muestra)` de la escena; devuelve si se aceptó.
    fn sound(&mut self, x: i32, y: i32, sample: i32) -> u8;
    /// `0x443690(lista)` del catálogo `[0x6CBDB8]`: número de muestras.
    fn list_count(&mut self, list: u32) -> i32;
    /// `0x5986A0(x, y)`
    fn pixel(&mut self, x: i16, y: i16);
}

/// Sonidos del barco ajeno en el puerto (`ShipSoundList`, 60100 en `sound.ini`).
pub const SHIP_SOUND_LIST: u32 = 0xEAC4;

/// `n / 17` con la multiplicación mágica `0x78787879` y `sar 3`.
fn div17(n: i32) -> i32 {
    let hi = ((i64::from(n) * 0x7878_7879) >> 32) as i32 >> 3;
    hi + ((hi as u32) >> 31) as i32
}

/// `n / 34` (`0x78787879`, `sar 4`).
fn div34(n: i32) -> i32 {
    let hi = ((i64::from(n) * 0x7878_7879) >> 32) as i32 >> 4;
    hi + ((hi as u32) >> 31) as i32
}

/// `n / 1000` (`0x10624DD3`, `sar 6`).
fn div1000(n: i32) -> i32 {
    let hi = ((i64::from(n) * 0x1062_4DD3) >> 32) as i32 >> 6;
    hi + ((hi as u32) >> 31) as i32
}

/// División con signo entre `1 << k` truncando hacia cero (`cdq; and edx, m; add; sar`).
fn sdiv_pow2(n: i32, k: u32) -> i32 {
    (n + ((n >> 31) & ((1 << k) - 1))) >> k
}

/// Divisor del tercer anillo (`0xC003000D`, `add`, `sar 13`).
fn div_ring3(n: i32) -> i32 {
    let hi = ((i64::from(n) * i64::from(0xC003_000Du32 as i32)) >> 32) as i32;
    let t = hi.wrapping_add(n) >> 13;
    t + ((t as u32) >> 31) as i32
}

/// Deriva de los humos al viento: `(fuerza·sen + 500) / 1000` y `(500 − fuerza·cos) / 1000` (con `doble` en el hundimiento).
fn drift(c: &ViewCtx, double: bool) -> (i32, i32) {
    let s = sin_t(c.wind_dir);
    let co = sin_t(c.wind_dir.wrapping_add(0x40));
    let w = i32::from(c.wind);
    if double {
        (div1000((w * s) * 2 + 500), div1000((250 - w * co) * 2))
    } else {
        (div1000(w * s + 500), div1000(500 - w * co))
    }
}

/// `0x62018B`.
pub fn update_view(o: &mut LocalShipView, c: &ViewCtx, lock: &mut u8, h: &mut dyn ViewHost) {
    let (px, py) = (o.px(), o.py());
    let view_y = div17(py * 11);
    if o.flag & 4 != 0 {
        if o.heading.wrapping_sub(0x10) & 0x18 == 0 {
            if o.entity == 0 && o.warned == 0 {
                return;
            }
            if o.smoke == 0 {
                let dir = (((u32::from(o.heading) + 0x10) >> 5) as u8 & 7).wrapping_add(1);
                let base = u32::from(dir) << 8;
                let (mut a, mut b, mut alpha) = (base + 4, base + 0x40, 0x80u8);
                match c.hull {
                    1 => (a, b) = (base + 5, base + 0x41),
                    2 => (a, b) = (base + 6, base + 0x42),
                    3 => (a, b, alpha) = (base + 7, base + 0x43, 0xFF),
                    _ => {}
                }
                if o.entity != 0 {
                    h.release(o.entity);
                    o.entity = 0;
                }
                o.smoke = h.create(px << 16, view_y << 16, 0x12, a, c.emitter_arg);
                o.flames = h.create(px << 16, view_y << 16, 0x12, b, c.emitter_arg);
                h.set_alpha(o.flames, alpha);
                o.phase = 0xFF;
            } else {
                if o.phase >= 0x21 {
                    if o.warned != 0 {
                        *lock = 0;
                        o.warned = 0;
                    }
                    return;
                }
                if h.frame(o.flames) < 0x14 {
                    h.emit(o.smoke, px, view_y);
                    h.emit(o.flames, px, view_y);
                }
            }
            let f = h.frame(o.flames);
            if f != o.phase {
                o.phase = f;
                sinking_frame(o, c, h, f, px, py);
                if o.phase < 0xA {
                    for _ in 0..2 {
                        let ry = h.rand() % 20;
                        let y = div17((ry + py - 10) << 16) * 11;
                        let rx = h.rand() % 20;
                        let e = h.create((rx + px - 10) << 16, y, 0x12, 0x24, c.emitter_arg);
                        h.set_alpha(e, 0x19);
                        let (dx, dy) = drift(c, true);
                        h.set_drift(e, dx, dy);
                    }
                }
            }
        }
        if *lock == 0 {
            *lock = 1;
            o.warned = 1;
        }
        if o.entity == 0 {
            return;
        }
    } else if o.flag & 6 != 0 {
        if o.entity != 0 {
            h.release(o.entity);
            o.entity = 0;
        }
        return;
    } else if o.flag & 0x20 != 0 {
        if o.sounded == 0 {
            let owner = c.owners.get(usize::from(o.owner_slot)).copied().unwrap_or(0);
            if u32::from(owner) != c.player && h.list_count(SHIP_SOUND_LIST) != 0 {
                let n = h.list_count(SHIP_SOUND_LIST);
                let sample = h.rand() % n - 0x153C;
                o.sounded = h.sound(px, div34(py * 22), sample);
            }
        }
    } else {
        o.sounded = 0;
    }
    // Entidad de vista (clase 0xD): alta o pose.
    if o.entity == 0 {
        let code = u32::from(c.hull) << 16 | u32::from(o.sails) << 24 | (o.world & 0xFFFF);
        o.entity = h.create(o.x as i32, div34(o.y as i32) * 22, 0xD, code, o.heading);
    } else {
        let eh = h.entity_heading(o.entity);
        let (a, b) = (i32::from(o.heading), i32::from(eh));
        let heading = if a - b < 3 && b - a < 3 { eh } else { o.heading };
        h.pose(o.entity, px as i16, div34(py * 22), heading, o.sails);
    }
    // Humo de casco dañado.
    if !c.smoke_option || c.health.wrapping_mul(3) > c.max_health.wrapping_mul(2) {
        return;
    }
    let due = (c.health.wrapping_mul(1200) / c.max_health).wrapping_add(c.clock).wrapping_add(200);
    if c.clock > o.next_smoke {
        o.last_smoke = o.next_smoke;
        let ry = h.rand() % 20;
        let y = div34((ry + py - 10) << 16) * 22;
        let rx = h.rand() % 20;
        let e = h.create((rx + px - 10) << 16, y, 0x12, 0x24, c.emitter_arg);
        o.next_smoke = due;
        h.set_alpha(e, (c.health.wrapping_mul(345) / c.max_health).wrapping_add(0x19) as u8);
        let (dx, dy) = drift(c, false);
        h.set_drift(e, dx, dy);
    } else if due < o.next_smoke {
        o.next_smoke = due;
    }
}

/// Lo que dibuja cada fotograma nuevo del emisor de llamas (tabla de saltos `0x620ED4` por `0x620EF0`).
fn sinking_frame(o: &LocalShipView, c: &ViewCtx, h: &mut dyn ViewHost, frame: u8, px: i32, py: i32) {
    let ring = |h: &mut dyn ViewHost, n: u32, step: u32, div: &dyn Fn(i32) -> i32| {
        let mut a = 0x40u32;
        for _ in 0..n {
            let s = sin_t(a as u8);
            let co = sin_t((a - 0x40) as u8);
            let y = div17((py - div(s)) * 11);
            let x = (px as i16).wrapping_add(div(co) as i16);
            h.pixel(x, y as i16);
            a += step;
        }
    };
    match frame {
        0 => h.pixel(px as i16, div17(py * 11) as i16),
        1 => ring(h, 16, 0x10, &|v| sdiv_pow2(v, 15)),
        2 => ring(h, 32, 8, &|v| sdiv_pow2(v, 14)),
        3 => ring(h, 64, 4, &div_ring3),
        4 => ring(h, 64, 4, &|v| sdiv_pow2(v, 13)),
        0xC => {
            let n = u32::from(c.hull) * 2 + 4;
            for _ in 0..n {
                let ry = h.rand() % 128;
                let y = div17((ry + py - 0x40) * 11) << 16;
                let rx = h.rand() % 128;
                h.create((rx + px - 0x40) << 16, y, 0x12, 0xC0, c.emitter_arg);
            }
        }
        _ => {}
    }
    let _ = o;
}
