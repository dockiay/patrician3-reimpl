//! Resto del dibujo de la carta náutica (`nav_map_draw`, `0x0044AB70`) con paridad x86 (`oracle/x86_navmap_draw.py`,
//! fixtures `tests/fixtures/navmap_draw_*.json`):
//!
//! - [`draw_town_labels`]: `0x00452600`, nombres de las ciudades con contorno negro (`NamenXPos/YPos`, alineación
//!   `NameLinksOderRechtsBuendig`).
//! - [`draw_town_markers`]: `0x0044BCC0`, símbolo de cada ciudad (`StadtSymbolXPos/YPos`, textura `+0xD0`) según sea la ciudad
//!   natal del jugador o tenga oficina, resaltado bajo el cursor (`+0x2B04`) o elegido (`+0x2B06`); al final el símbolo de la
//!   ciudad `merchant+0xE1` del jugador.
//! - [`draw_town_status_icons`]: `0x0044BFD0`, iconos de estado de cada ciudad (`EreignisSymbolXPos/YPos`, texturas `+0xC4` y
//!   `+0xC8`) según los bits que devuelve `0x00535300` (historial de noticias conocidas de la ciudad).
//! - [`draw_iceberg_icons`]: `0x0044B630`, la misma animación que las tormentas ([`crate::navmap::draw_weather_icons`]) pero
//!   dibujando solo los icebergs (subtipo 2, textura por identificador `+0x2AFC`). El original anima las 16 entradas en las dos
//!   funciones y las llama seguidas en cada fotograma.
//!
//! Igual que en el x86, los rectángulos, manejadores y tamaños de fotograma que quedan sin iniciar cuando el índice cae fuera de
//! la textura se arrastran de una ciudad a la siguiente ([`Carry`], empiezan a cero).

use crate::navmap::{r16, r32, r8, NavHost, Tex, Textures};

/// Fuente de los rótulos de las ciudades (`0x006DCF20`) y la pequeña de cifras (`0x006DCD28`): objetos `SGLFont`; el original pasa
/// a `sgl_SetFont` su manejador (`0x004C1B90`), aquí identificados por la dirección del objeto.
pub const FONT_TOWN_LABELS: u32 = 0x006D_CF20;
pub const FONT_SMALL: u32 = 0x006D_CD28;

fn i16_at(m: &[u8], o: usize) -> i32 {
    i32::from(r16(m, o) as i16)
}
fn i32_at(m: &[u8], o: usize) -> i32 {
    r32(m, o) as i32
}

/// Entrada de la tabla de ciudades `0x006DDB90` (0x34 bytes por ciudad, cargada de `scripts/StadtDaten.ini` en `0x0051E480`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TownInfo {
    /// `+0x00` `StadtSymbolXPos/YPos`.
    pub symbol: (i32, i32),
    /// `+0x08` `NamenXPos/YPos`.
    pub label: (i32, i32),
    /// `+0x10` `EreignisSymbolXPos/YPos`.
    pub event: (i32, i32),
    /// `+0x18` `SchiffAufLandXPos/YPos`.
    pub harbor: (i32, i32),
    /// `+0x20` `AnfahrtXPos/YPos`.
    pub approach: (i32, i32),
    /// `+0x2F`: byte sin identificar de la tabla; distinto de 0 en una ciudad de las zonas fluviales, la ciudad no admite
    /// cascos de tipo mayor que 1 (`0x0045BCF0`).
    pub river: u8,
    /// `+0x33` primera letra de `NameLinksOderRechtsBuendig` (`c`, `l`, `r`).
    pub align: u8,
}

/// Ciudades de la partida: `DAT_006DE4B0` (número), `0x006DE4B8` (identificador de cada una), la tabla `0x006DDB90` por
/// identificador y los nombres `0x006DDA00` (por posición, no por identificador).
#[derive(Clone, Copy, Debug, Default)]
pub struct Towns<'a> {
    pub order: &'a [u8],
    pub info: &'a [TownInfo],
    pub names: &'a [String],
}
impl Towns<'_> {
    pub(crate) fn info(&self, i: usize) -> TownInfo {
        let id = usize::from(self.order.get(i).copied().unwrap_or(0));
        self.info.get(id).copied().unwrap_or_default()
    }
    pub(crate) fn info_by_id(&self, id: usize) -> TownInfo {
        self.info.get(id).copied().unwrap_or_default()
    }
}

/// Valores de pila que el original deja sin iniciar y conserva entre iteraciones: manejador, origen y tamaño del fotograma.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Carry {
    pub handle: u32,
    pub src: (i32, i32),
    pub size: (i32, i32),
}
impl Carry {
    /// `0x0045C080` en línea: el fotograma `i` existe si `i < número` (con signo) y el manejador no es nulo.
    fn take(&mut self, t: &Tex, i: i32) -> bool {
        match t.frame(i) {
            Some((h, f)) => {
                self.handle = h;
                self.src = (f[0], f[1]);
                self.size = (f[2], f[3]);
                true
            }
            None => false,
        }
    }
}

/// `0x00452600`.
pub fn draw_town_labels(obj: &[u8], towns: &Towns, host: &mut dyn NavHost) {
    host.font(FONT_TOWN_LABELS);
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    for i in 0..towns.order.len() {
        let t = towns.info(i);
        match t.align {
            b'c' => host.text_mode(3),
            b'l' => host.text_mode(1),
            b'r' => host.text_mode(2),
            _ => {}
        }
        let name = towns.names.get(i).map(String::as_str).unwrap_or("");
        let x = t.label.0.wrapping_sub(sx);
        let y = i32_at(obj, 0x18).wrapping_sub(sy).wrapping_add(t.label.1);
        host.color(0xFF00_0000);
        for (dx, dy) in [(-1, -1), (1, 1), (-1, 1), (1, -1)] {
            host.text(x.wrapping_add(dx), y.wrapping_add(dy), name);
        }
        host.color(0xFFFF_E288);
        host.text(x, y, name);
    }
}

fn view(obj: &[u8]) -> (i32, i32) {
    (i32_at(obj, 0x14).wrapping_add(i32_at(obj, 0x2C)), i32_at(obj, 0x30).wrapping_add(i32_at(obj, 0x18)))
}

/// `0x0044BCC0`. `has_office` es la tabla de bytes `+0x28CC` (por posición de ciudad); `merchant(m)` la imagen del comerciante
/// `m` (`0x005303C0`, se lee `+0x19` ciudad natal y `+0xE1`).
pub fn draw_town_markers(
    obj: &[u8],
    towns: &Towns,
    tex: &Tex,
    player: u32,
    has_office: &[u8],
    merchant: &dyn Fn(u32) -> Vec<u8>,
    carry: &mut Carry,
    host: &mut dyn NavHost,
) {
    host.color(0xFFFF_FFFF);
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let (right, bottom) = view(obj);
    let vy = i32_at(obj, 0x18);
    for i in 0..towns.order.len() {
        let t = towns.info(i);
        let x = t.symbol.0.wrapping_sub(sx);
        let y = t.symbol.1.wrapping_sub(sy);
        host.get_merchant(player);
        let m = merchant(player);
        let flag: i32 = if i == usize::from(m.get(0x19).copied().unwrap_or(0)) {
            1
        } else if has_office.get(i).copied().unwrap_or(0) != 0 {
            2
        } else {
            0
        };
        let idx = if i as u32 == u32::from(r16(obj, 0x2B06)) {
            flag * 3 + 2
        } else if i as u32 == u32::from(r16(obj, 0x2B04)) {
            flag * 3 + 1
        } else {
            flag * 3
        };
        carry.take(tex, idx);
        host.select(carry.handle);
        let (w, h) = carry.size;
        if x < right && y < bottom && y.wrapping_add(h) > 0 && x.wrapping_add(w) > 0 {
            host.blit(carry.src.0, carry.src.1, x, vy.wrapping_add(y), w, h);
        }
    }
    host.get_merchant(player);
    let target = merchant(player).get(0xE1).copied().unwrap_or(0xFF);
    if target >= 0x28 {
        return;
    }
    host.color(0xFF00_0000);
    let t = towns.info_by_id(usize::from(target));
    let x = t.symbol.0.wrapping_sub(sx);
    let y = t.symbol.1.wrapping_sub(sy);
    let idx = if r16(obj, 0x2B06) == 0x27 {
        5
    } else if r16(obj, 0x2B04) == 0x27 {
        4
    } else {
        3
    };
    carry.take(tex, idx);
    host.select(carry.handle);
    let (w, h) = carry.size;
    if x < right && y < bottom && y.wrapping_add(h) > 0 && w.wrapping_add(x) > 0 {
        host.blit(carry.src.0, carry.src.1, x, vy.wrapping_add(y), w, h);
    }
}

/// `0x0044BFD0`. `flags(i)` es `0x00535300(i)` sobre `0x006DE588` (por posición de ciudad); `icons` la textura `+0xC4` y
/// `levels` la `+0xC8` (fotograma = bits 18..22 de las banderas cuando no valen 0x1F).
pub fn draw_town_status_icons(
    obj: &[u8],
    towns: &Towns,
    icons: &Tex,
    levels: &Tex,
    flags: &mut dyn FnMut(u32) -> u32,
    carry: &mut Carry,
    host: &mut dyn NavHost,
) {
    host.color(0xFFFF_FFFF);
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let (right, bottom) = view(obj);
    let vy = i32_at(obj, 0x18);
    let mut draw = |carry: &Carry, x: i32, y: i32, host: &mut dyn NavHost| {
        host.select(carry.handle);
        let (w, h) = carry.size;
        if x < right && y < bottom && y.wrapping_add(h) > 0 && w.wrapping_add(x) > 0 {
            host.blit(carry.src.0, carry.src.1, x, y, w, h);
        }
    };
    for i in 0..towns.order.len() {
        let t = towns.info(i);
        let x = t.event.0.wrapping_sub(sx);
        let y = t.event.1.wrapping_sub(sy).wrapping_add(vy);
        let f = flags(i as u32);
        if f & 0x1000 != 0 {
            carry.take(icons, 6);
            draw(carry, x, y, host);
        } else if f & 0x7C_0000 != 0x7C_0000 {
            carry.take(levels, ((f as i32) >> 18) & 0x1F);
            draw(carry, x, y, host);
        } else if f & 0x1_0000 != 0 {
            carry.take(icons, 5);
            draw(carry, x, y, host);
        }
        let second = if f & 0x200 != 0 {
            Some(12)
        } else if f & 0x400_0000 != 0 {
            Some(14)
        } else if f & 0x400 != 0 {
            Some(13)
        } else if f & 0x10 != 0 {
            Some(8)
        } else if f & 0x8000 != 0 {
            Some(11)
        } else if f & 0x8 != 0 {
            Some(10)
        } else if f & 0x800 != 0 {
            Some(9)
        } else {
            None
        };
        if let Some(k) = second {
            carry.take(icons, k);
            draw(carry, x.wrapping_add(0x14), y, host);
        }
    }
}

/// `0x0044B630`: anima las 16 entidades de tiempo (como [`crate::navmap::draw_weather_icons`]) y dibuja los icebergs (subtipo 2,
/// textura `+0x2AFC`).
pub fn draw_iceberg_icons(obj: &mut [u8], tex: &Textures, clock: u32, carry: &mut Carry, host: &mut dyn NavHost) {
    let (right, bottom) = view(obj);
    for i in 0..16usize {
        let kind = r8(obj, 0x2A88 + i);
        if kind >= 0x10 {
            continue;
        }
        crate::navmap::animate_weather_entry(obj, i, clock, host);
        if kind != 2 {
            continue;
        }
        let id = r32(obj, 0x2AFC);
        host.lookup(id);
        let t = tex.lookup(id);
        carry.take(&t, i32::from(r8(obj, 0x2A78 + i)));
        let (w, h) = carry.size;
        let half = |n: i32| n.wrapping_sub(n >> 31) >> 1;
        let x = i16_at(obj, 0x2AB8 + 4 * i).wrapping_sub(i16_at(obj, 0x29E4)).wrapping_sub(half(w));
        let vy = i32_at(obj, 0x18);
        let y = i16_at(obj, 0x2ABA + 4 * i).wrapping_sub(i16_at(obj, 0x29E6)).wrapping_sub(half(h)).wrapping_add(vy);
        if !(x < right && y < bottom && h.wrapping_add(y) > vy && w.wrapping_add(x) > 0) {
            continue;
        }
        host.color(0xFFFF_FFFF);
        carry.take(&t, i32::from(r8(obj, 0x2A78 + i)));
        host.select(carry.handle);
        let (w, h) = carry.size;
        host.blit(carry.src.0, carry.src.1, x, y, w, h);
    }
}

/// Contactos perdidos que se desvanecen en la carta (`+0x28A8` barcos, `+0x28AC` opacidad, `+0x28B0` última marca de tiempo;
/// recuento `+0x2924`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Contacts {
    pub ids: Vec<u32>,
    pub alpha: Vec<i32>,
    pub last: Vec<u32>,
}

/// `0x00452F60(barco)`, llamada desde el tick naval `0x0050D2B0` cuando un barco deja de verse. `enabled` es la condición del
/// original: `[0x006CC3E8]+0xD` distinto de cero y `DAT_006DFC1C != DAT_00673CA8`. Si el barco no está ya, entra con opacidad
/// 255 y la hora `now` (`DAT_006DE4B4`).
pub fn add_lost_contact(c: &mut Contacts, ship: u32, now: u32, enabled: bool) {
    if !enabled || c.ids.contains(&ship) {
        return;
    }
    c.ids.push(ship);
    c.alpha.push(0xFF);
    c.last.push(now);
}

/// Opacidad que se pierde entre dos fotogramas: `(float)(now - last) / 10 * -255.0` truncado (`0x0066DE78`, `0x0066DB70`).
pub fn contact_fade(elapsed: u32) -> i32 {
    let e = elapsed as f32;
    crate::navmap::ftol(f64::from(e) / f64::from(10f32) * -255.0)
}

/// Estado de pila que `0x004530B0` reutiliza sin iniciar entre barcos y entre sus dos bucles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FadeCarry {
    pub hull: u32,
    pub rect: [i32; 4],
    pub info: [i32; 3],
}

/// `0x004530B0`. Primer bucle: cada contacto se dibuja (marcador del bando, número del convoy y casco) con su opacidad, que
/// baja según el tiempo pasado; sale de la lista al llegar bajo cero o cuando `ship+0x39 == ship+0x38`. Segundo bucle: los
/// contactos en estado `<= 0x10` que están en la selección (`0x004885A0`) llevan la flecha `0x7557` encima.
pub fn draw_fading_contacts(
    obj: &mut [u8],
    world: &crate::navmap::World,
    tex: &Textures,
    contacts: &mut Contacts,
    selection: &[u16],
    now: u32,
    carry: &mut FadeCarry,
    host: &mut dyn NavHost,
) {
    use crate::navmap::{half, w32};
    if contacts.ids.is_empty() {
        return;
    }
    host.lookup(crate::navmap::ARROW_TEXTURE);
    let arrow = tex.lookup(crate::navmap::ARROW_TEXTURE);
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let (vx, vy) = (i32_at(obj, 0x14), i32_at(obj, 0x18));
    let (vw, vh) = (i32_at(obj, 0x2C), i32_at(obj, 0x30));
    let mut i = 0usize;
    while i < contacts.ids.len() {
        let id = contacts.ids[i];
        let s = world.ship(id);
        let a = (contacts.alpha[i] as u32 & 0xFF) << 24;
        let x = i16_at(s, 0x1E).wrapping_sub(sx);
        let y = i16_at(s, 0x22).wrapping_sub(sy).wrapping_add(vy);
        if let Some((h, f)) = tex.hull(r8(s, 0xE)).and_then(|t| t.frame(i32::from(r8(s, 1)))) {
            carry.hull = h;
            carry.rect = f;
        }
        let x0 = x.wrapping_sub(half(carry.rect[2]));
        let y0 = y.wrapping_sub(half(carry.rect[3]));
        carry.info = host.texture_info(tex.marker.handle);
        host.select(carry.info[2] as u32);
        let (mw, mh) = (carry.info[0], carry.info[1]);
        let state = r16(s, 0x134);
        let owner = r8(s, 0);
        let other = carry.rect[2].wrapping_sub(mw).wrapping_add(x0);
        let (colour, text, mx) = if state == 0x12
            || u16::from(owner) >= world.merchant_count
            || (r8(s, 0x15C) & 1 != 0 && u32::from(owner) != world.player)
        {
            (a, a | 0xFF_FFFF, other)
        } else if state == 0x16 {
            (a | 0xFF_FF00, a, other)
        } else if u32::from(owner) != world.player {
            if host.merchant_word8(owner) != 0 {
                (a | 0xFF_FFFF, a, other)
            } else {
                (a | 0xFF_0000, a | 0xFF_FFFF, other)
            }
        } else {
            (a | 0xFF, a | 0xFF_FFFF, x0)
        };
        host.color(colour);
        host.blit(0, 0, mx, y0, mw, mh);
        let conv = r16(s, 8);
        if conv < world.convoy_count {
            let mut n: u8 = 0;
            let mut c = r16(world.convoy(u32::from(conv)), 0xA);
            while c < world.ship_count {
                n = n.wrapping_add(1);
                c = r16(world.ship(u32::from(c)), 6);
            }
            host.color(text);
            host.text(mx.wrapping_add(7), y0.wrapping_add(3), &n.to_string());
        }
        host.select(carry.hull);
        host.color(a | 0xFF_FFFF);
        let r = carry.rect;
        if y0 < vy.wrapping_add(vh) && x0 < vx.wrapping_add(vw) && r[3].wrapping_add(y0) > vy && x0.wrapping_add(r[2]) > 0 {
            host.blit(r[0], r[1], x0, y0, r[2], r[3]);
        }
        let d = contact_fade(now.wrapping_sub(contacts.last[i]));
        contacts.alpha[i] = contacts.alpha[i].wrapping_add(d);
        contacts.last[i] = now;
        if r8(s, 0x39) == r8(s, 0x38) || contacts.alpha[i] < 0 {
            contacts.ids.remove(i);
            contacts.alpha.remove(i);
            contacts.last.remove(i);
        } else {
            i += 1;
        }
    }
    w32(obj, 0x2924, contacts.ids.len() as u32);
    for i in 0..contacts.ids.len() {
        let id = contacts.ids[i];
        let s = world.ship(id);
        let state = r16(s, 0x134);
        if state > 0x10 {
            continue;
        }
        let a = (contacts.alpha[i] as u32 & 0xFF) << 24;
        let x = i16_at(s, 0x1E).wrapping_sub(sx);
        let mut y = i16_at(s, 0x22).wrapping_sub(sy).wrapping_add(vy);
        if let Some((_, f)) = tex.hull(r8(s, 0xE)).and_then(|t| t.frame(i32::from(r8(s, 1)))) {
            carry.rect = f;
        }
        y = y.wrapping_sub(half(carry.rect[3]));
        let owner = r8(s, 0);
        let colour = if u16::from(owner) >= world.merchant_count
            || state == 0x12
            || (r8(s, 0x15C) & 1 != 0 && u32::from(owner) != world.player)
        {
            a
        } else if state == 0x16 {
            a | 0xFF_FF00
        } else if u32::from(owner) != world.player {
            if host.merchant_word8(owner) != 0 {
                a | 0xFF_FFFF
            } else {
                a | 0xFF_0000
            }
        } else {
            a | 0xFF
        };
        host.color(colour);
        carry.info = host.texture_info(arrow.handle);
        let (aw, ah) = (carry.info[0], carry.info[1]);
        for &sel in selection {
            if u32::from(sel) == id {
                host.select(arrow.handle);
                let dy = y.wrapping_sub(i32::from(r16(obj, 0x2B0C))).wrapping_sub(ah);
                host.blit(0, 0, x.wrapping_sub(half(aw)), dy, aw, ah);
            }
        }
    }
}

/// `draw_geometry` (`0x004BD680`): una línea horizontal o vertical es un rectángulo de un píxel de ancho (`sgl_FillRect`), las
/// demás van a la línea en coma flotante.
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, host: &mut dyn NavHost) {
    let (dx, dy) = (x2.wrapping_sub(x1), y2.wrapping_sub(y1));
    if dx == 0 {
        if dy >= 0 {
            host.fill(x1, y1, 1, dy.wrapping_add(1));
        } else {
            host.fill(x2, y2, 1, 1i32.wrapping_sub(dy));
        }
    } else if dy == 0 {
        if dx >= 0 {
            host.fill(x1, y1, dx.wrapping_add(1), 1);
        } else {
            host.fill(x2, y2, 1i32.wrapping_sub(dx), 1);
        }
    } else {
        host.line(x1 as f32, y1 as f32, x2 as f32, y2 as f32);
    }
}

/// `0x00448D80`: ruta con las esquinas redondeadas ([`p3_core::route::planner::smooth_corner`] sobre cada terna, con el
/// primer punto delante y el último detrás). Con menos de tres puntos se copian los dos primeros.
pub fn smooth_route(points: &[(i16, i16)]) -> Vec<(i16, i16)> {
    use p3_core::route::planner::smooth_corner;
    let at = |k: usize| points.get(k).copied().unwrap_or((0, 0));
    let mut out = vec![at(0)];
    if points.len() < 3 {
        out.push(at(1));
        return out;
    }
    smooth_corner(at(0), at(1), at(2), false, &mut out);
    for j in 3..points.len() {
        smooth_corner(at(j - 2), at(j - 1), at(j), false, &mut out);
    }
    out.push(at(points.len() - 1));
    out
}

/// Estado que conserva el dibujo de rutas entre fotogramas: la caché de rutas suavizadas por barco (`+0xA4`), la ruta en
/// preparación (`+0xA8`, que si existe se dibuja en lugar de la de cada barco elegido) y el contador global `DAT_006CBDDC`
/// (tres por tramo dibujado; desde 200 las rutas nuevas se copian sin suavizar).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RouteCache {
    pub smoothed: std::collections::BTreeMap<u32, Vec<(i16, i16)>>,
    pub preview: Option<Vec<(i16, i16)>>,
    pub budget: u32,
}

/// `0x0044B330(origen)`. Para cada barco elegido (`0x004885A0`; con convoy, su capitán `convoy+0x10`) del jugador y estado
/// distinto de 0x11 marca `+0x2B27` y, si su ruta (`ship+0x24`, que da `route`) tiene más de un punto, dibuja la polilínea en
/// el color `+0x2920` con un filo a cada lado en `0x80315A8C` (desplazado en `y` si el tramo es más ancho que alto, en `x` si
/// no). `origin` es la esquina de la zona de dibujo.
pub fn draw_routes(
    obj: &mut [u8],
    world: &crate::navmap::World,
    selection: &[u16],
    route: &dyn Fn(u32) -> Option<Vec<(i16, i16)>>,
    cache: &mut RouteCache,
    origin: (i32, i32),
    host: &mut dyn NavHost,
) {
    for &sel in selection {
        let mut id = u32::from(sel);
        let mut s = world.ship(id);
        let conv = r16(s, 8);
        if conv < world.convoy_count {
            id = u32::from(r16(world.convoy(u32::from(conv)), 0x10));
            s = world.ship(id);
        }
        if u32::from(r8(s, 0)) != world.player || r16(s, 0x134) == 0x11 {
            continue;
        }
        obj[0x2B27] = 1;
        let Some(points) = route(id) else { continue };
        if points.len() <= 1 {
            continue;
        }
        if !cache.smoothed.contains_key(&id) {
            let r = if (cache.budget as i32) < 200 { smooth_route(&points) } else { points.clone() };
            cache.smoothed.insert(id, r);
        }
        cache.budget = 0;
        let r = cache.preview.as_ref().unwrap_or(&cache.smoothed[&id]).clone();
        if r.is_empty() {
            continue;
        }
        let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
        let colour = r32(obj, 0x2920);
        let at = |p: (i16, i16)| (i32::from(p.0).wrapping_sub(sx).wrapping_add(origin.0), i32::from(p.1).wrapping_sub(sy).wrapping_add(origin.1));
        for j in 0..r.len() {
            host.color(colour);
            if j + 1 == r.len() {
                break;
            }
            let (x1, y1) = at(r[j]);
            let (x2, y2) = at(r[j + 1]);
            draw_line(x1, y1, x2, y2, host);
            let (dx, dy) = (x2.wrapping_sub(x1), y2.wrapping_sub(y1));
            host.color(0x8031_5A8C);
            if f64::from(dy as f32).abs() < f64::from(dx as f32).abs() {
                draw_line(x1, y1.wrapping_add(1), x2, y2.wrapping_add(1), host);
                draw_line(x1, y1.wrapping_sub(1), x2, y2.wrapping_sub(1), host);
            } else {
                draw_line(x1.wrapping_add(1), y1, x2.wrapping_add(1), y2, host);
                draw_line(x1.wrapping_sub(1), y1, x2.wrapping_sub(1), y2, host);
            }
            cache.budget = cache.budget.wrapping_add(3);
        }
    }
}

/// Textura de los barcos en puerto (`0x753A`, por identificador).
pub const PORT_TEXTURE: u32 = 0x753A;

/// `0x0044C3B0`. Junto a cada ciudad (`SchiffAufLandXPos/YPos`) dibuja el barco más grande del jugador atracado en ella
/// (textura `0x753A`, fotograma = tipo mayor), cuántos barcos tiene allí (hasta 9, a la izquierda) y cuántos en convoy (a la
/// derecha); junto a `EreignisSymbolXPos/YPos` pone los iconos de la textura `+0xC4` de los barcos ajenos en las ciudades con
/// barco u oficina del jugador (3 o 4 según `merchant+8`; con convoy los fotogramas 0/1 a 20 píxeles).
///
/// Como el original, las tablas se rellenan por `ship+0x39` y se leen por **posición** de ciudad (`0x006DE4B8`), y el texto de
/// la derecha se alinea con el ancho del último fotograma dibujado ([`Carry`]). `first_ship(m)` es `merchant+0xE`.
pub fn draw_ships_in_port(
    obj: &[u8],
    world: &crate::navmap::World,
    towns: &Towns,
    port: &Tex,
    icons: &Tex,
    first_ship: &dyn Fn(u32) -> u16,
    carry: &mut Carry,
    host: &mut dyn NavHost,
) {
    let n = towns.order.len();
    let (mut here, mut convoy_mark, mut biggest) = (vec![0u8; n], vec![0u8; n], vec![0u8; n]);
    let (mut ships, mut convoys) = (vec![0i32; n], vec![0i32; n]);
    host.get_merchant(world.player);
    let mut i = u32::from(first_ship(world.player));
    while i < u32::from(world.ship_count) {
        let s = world.ship(i);
        let t = usize::from(r8(s, 0x39));
        if t < n && r16(s, 0x134) < 4 {
            ships[t] = ships[t].wrapping_add(1);
            here[t] = 1;
            let kind = r8(s, 0xE);
            if i32::from(kind) > i32::from(biggest[t] as i8) {
                biggest[t] = kind;
            }
            let conv = r16(s, 8);
            if conv < world.convoy_count {
                convoys[t] = convoys[t].wrapping_add(1);
                let lead = r16(world.convoy(u32::from(conv)), 0x10);
                if u32::from(r8(world.ship(u32::from(lead)), 0)) == world.player {
                    convoy_mark[t] = 1;
                }
            }
        }
        i = u32::from(r16(s, 4));
    }
    host.color(0xFFFF_FFFF);
    host.lookup(PORT_TEXTURE);
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let vy = i32_at(obj, 0x18);
    for p in 0..n {
        let t = towns.info(p);
        let x = t.harbor.0.wrapping_sub(sx);
        let y = t.harbor.1.wrapping_sub(sy).wrapping_add(vy);
        if here[p] != 0 {
            carry.take(port, i32::from(biggest[p] as i8));
            host.select(carry.handle);
            host.blit(carry.src.0, carry.src.1, x, y, carry.size.0, carry.size.1);
            biggest[p] = 0;
        }
    }
    let merchants = world.merchant_count;
    for m in 0..u32::from(merchants) {
        host.get_merchant(m);
        let mut i = u32::from(first_ship(m));
        while i < u32::from(world.ship_count) {
            let s = world.ship(i);
            let t = u32::from(r8(s, 0x39));
            let office = host.office(world.player & 0xFFFF, t);
            let tu = t as usize;
            if tu < n && (here[tu] != 0 || office) && m != world.player && r16(s, 0x134) < 4 {
                let conv = r16(s, 8);
                if conv > world.convoy_count {
                    here[tu] = 2 + u8::from(host.merchant_word8(m as u8) != 0);
                } else {
                    let c = world.convoy(u32::from(conv));
                    let lead_owner = r8(world.ship(u32::from(r16(c, 0x10))), 0);
                    if u32::from(lead_owner) == world.player {
                        convoy_mark[tu] = 1;
                    } else if r8(c, 0x14) & 2 == 0 {
                        convoy_mark[tu] = if host.merchant_word8(lead_owner) != 0 { 3 } else { 2 };
                    } else {
                        here[tu] = 2 + u8::from(host.merchant_word8(m as u8) != 0);
                    }
                }
            }
            i = u32::from(r16(s, 4));
        }
    }
    host.text_mode(3);
    host.font(FONT_SMALL);
    host.color(0xFF00_0000);
    for p in 0..n {
        let t = towns.info(p);
        let x = t.harbor.0.wrapping_sub(sx);
        let y = t.harbor.1.wrapping_sub(sy).wrapping_add(vy).wrapping_add(4);
        if ships[p] != 0 {
            ships[p] = ships[p].min(9);
            host.text(x.wrapping_add(10), y, &ships[p].to_string());
        }
        if convoys[p] != 0 {
            convoys[p] = convoys[p].min(9);
            let s = convoys[p].to_string();
            let w = host.text_width(FONT_SMALL, &s);
            host.text(x.wrapping_add(carry.size.0).wrapping_sub(w).wrapping_sub(10), y, &s);
        }
    }
    host.color(0xFFFF_FFFF);
    for p in 0..n {
        let t = towns.info(p);
        let x = t.event.0.wrapping_sub(sx);
        let y = t.event.1.wrapping_sub(sy).wrapping_add(vy).wrapping_add(0x14);
        let mut icon = |frame: i32, x: i32, carry: &mut Carry, host: &mut dyn NavHost| {
            carry.take(icons, frame);
            host.select(carry.handle);
            host.blit(carry.src.0, carry.src.1, x, y, carry.size.0, carry.size.1);
        };
        match here[p] as i8 {
            2 => icon(4, x, carry, host),
            3 => icon(3, x, carry, host),
            _ => {}
        }
        match convoy_mark[p] as i8 {
            2 => icon(1, x.wrapping_add(0x14), carry, host),
            3 => icon(0, x.wrapping_add(0x14), carry, host),
            _ => {}
        }
    }
}

/// Piezas de [`draw_nav_map`] con función propia portada.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavPart {
    /// [`draw_routes`] (`0x0044B330`, recibe la zona de dibujo).
    Routes,
    /// [`draw_town_markers`] (`0x0044BCC0`).
    Markers,
    /// [`draw_town_labels`] (`0x00452600`).
    Labels,
    /// [`draw_town_status_icons`] (`0x0044BFD0`).
    Status,
    /// [`draw_ships_in_port`] (`0x0044C3B0`).
    InPort,
    /// [`draw_iceberg_icons`] (`0x0044B630`).
    Icebergs,
    /// [`draw_fading_contacts`] (`0x004530B0`).
    Fading,
    /// [`crate::navmap::draw_ships_at_sea`] (`0x0044CBF0`).
    Ships,
    /// [`crate::navmap::draw_weather_icons`] (`0x0044B970`).
    Weather,
}

/// Argumentos y globales de [`draw_nav_map`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameArgs {
    /// Primer argumento: destino de dibujo (también lo reciben los controles).
    pub dest: u32,
    /// Desplazamiento de la ventana (segundo y tercer argumento) y el cuarto, que los controles reciben con ellos.
    pub dx: i32,
    pub dy: i32,
    pub extra: u32,
    /// `DAT_006DCCE0`/`DAT_006DCCE4` (tamaño de la pantalla).
    pub screen: (i32, i32),
    /// `DAT_006DCD20[DAT_006DCD04]`.
    pub clipper: u32,
    /// `[DAT_006CBB40]+0x7E4`: si no coincide con el primer valor del generador `LcgRandom` de semilla 0 (`0xD102D831`), el
    /// fondo se tiñe de un color al azar (`rand() << 16 | rand() & 0xFFFF`), como comprobación.
    pub check: u32,
}

/// Primer valor de `lcg_random_next(0x0046C350)` con semilla 0.
pub const LCG_FIRST: u32 = {
    let v = 0xA205_B063u32;
    if v & 1 != 0 {
        (v >> 1) | 0x8000_0000
    } else {
        v >> 1
    }
};

/// `nav_map_draw(0x0044AB70)`: el fotograma entero de la carta en el orden del original (fondo, lupa, controles, rutas,
/// símbolos de ciudad, marcas `0x7552`, rótulos, estado, barcos en puerto, recuadro de selección, puntos de ruta `0x7551`,
/// icebergs, contactos perdidos, barcos, controles, tiempo y restauración del estado gráfico). Las piezas con función propia
/// llegan por [`NavHost::part`]; `marks` son las posiciones `+0x28B4`/`+0x28B8` (recuento `+0x29DC`) y `waypoint_labels` los
/// textos `+0x2884` de los puntos de ruta `+0x28D4` (recuento `+0x291C`).
pub fn draw_nav_map(obj: &[u8], tex: &Textures, args: &FrameArgs, marks: &[(i32, i32)], waypoint_labels: &[String], host: &mut dyn NavHost) {
    use crate::navmap::half;
    let (vx, vy) = (i32_at(obj, 0x14), i32_at(obj, 0x18));
    let clip = [vx.wrapping_add(args.dx), vy.wrapping_add(args.dy), 0, 0];
    let clip = [clip[0], clip[1], clip[0].wrapping_add(i32_at(obj, 0x2C)), clip[1].wrapping_add(i32_at(obj, 0x30))];
    if !host.window_shown() || !host.clip_visible(clip) {
        return;
    }
    let clipper = host.get_clipper();
    let prev_clipper = host.set_clipper(clipper);
    host.copy_clipper(args.clipper);
    host.close_popups();
    host.sub_clip(0, 0, args.screen.0, vy);
    host.sub_clip(i32_at(obj, 0x2C), vy, args.screen.0, args.screen.1);
    let prev_dest = host.set_dest(args.dest);
    let prev_tex = host.set_texture(tex.background.handle);
    let tint = if args.check == LCG_FIRST {
        0xFFFF_FFFF
    } else {
        let hi = host.rand() << 16;
        hi.wrapping_add(host.rand() & 0xFFFF)
    };
    let prev_colour = host.set_color(tint);
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    host.blit(sx, sy, clip[0], clip[1], clip[2].wrapping_sub(clip[0]), clip[3].wrapping_sub(clip[1]));
    // Rectángulo de pila que la lupa llena con el cursor y que reutilizan los puntos de ruta y el modo de mezcla.
    let mut scratch = [0i32; 4];
    let mut scratch_handle = args.dest;
    if obj[0x2B23] != 0 {
        host.color(0x64FF_FFFF);
        let (cx, cy) = host.cursor();
        scratch[0] = cx;
        scratch[1] = cy;
        let lens = tex.lens.handle;
        let [w, h, _] = host.texture_info(lens);
        host.select(lens);
        let y = i16_at(obj, 0x28FA).wrapping_sub(half(h)).wrapping_sub(sy).wrapping_add(vy);
        let x = i16_at(obj, 0x28F8).wrapping_sub(half(w)).wrapping_sub(sx);
        host.blit(0, 0, x, y, w, h);
    }
    let wargs = [args.dx as u32, args.dy as u32, args.extra];
    let mut widgets = |host: &mut dyn NavHost, first: u32, n: u32| {
        for k in 0..n {
            let off = first + 0xD8 * k;
            if host.widget_running(off) {
                host.widget_draw(off, args.dest, wargs);
            }
        }
    };
    widgets(host, 0x288, 10);
    widgets(host, 0xE58, 3);
    host.part(NavPart::Routes, clip);
    widgets(host, 0x1950, 1);
    host.part(NavPart::Markers, clip);
    host.lookup(0x7552);
    let mark = tex.lookup(0x7552);
    host.color(0xFFFF_FFFF);
    let [mw, mh, _] = host.texture_info(mark.handle);
    host.select(mark.handle);
    for k in 0..(i32_at(obj, 0x29DC).max(0) as usize) {
        let (x, y) = marks.get(k).copied().unwrap_or((0, 0));
        let y = y.wrapping_sub(half(mh)).wrapping_sub(sy).wrapping_add(vy);
        let x = x.wrapping_sub(half(mw)).wrapping_sub(sx);
        host.blit(0, 0, x, y, mw, mh);
    }
    host.part(NavPart::Labels, clip);
    host.part(NavPart::Status, clip);
    host.part(NavPart::InPort, clip);
    host.color(0xFF64_6464);
    if i32_at(obj, 0x98) != -1 {
        let prev = host.blend_mode(3);
        let (x1, x2) = (i32_at(obj, 0x94).wrapping_sub(sx), i32_at(obj, 0x9C).wrapping_sub(sx));
        let (y1, y2) = (i32_at(obj, 0x98).wrapping_sub(sy), i32_at(obj, 0xA0).wrapping_sub(sy));
        draw_line(x1, y1, x1, y2, host);
        draw_line(x2, y1, x2, y2, host);
        draw_line(x1, y1, x2, y1, host);
        draw_line(x1, y2, x2, y2, host);
        // El original guarda el modo anterior en el byte bajo de ese rectángulo de pila y restaura la doble palabra entera.
        scratch[0] = (scratch[0] & !0xFF) | (prev & 0xFF);
        host.blend_mode(scratch[0]);
    }
    host.color(0xFFFF_FFFF);
    host.lookup(0x7551);
    let way = tex.lookup(0x7551);
    if let Some((h, f)) = way.frame(7) {
        scratch_handle = h;
        scratch = f;
    }
    host.select(scratch_handle);
    let count = i32_at(obj, 0x291C);
    for k in 0..count.max(0) {
        if host.widget_running(0x1950) && k == count - 1 {
            break;
        }
        let at = 0x28D4 + 4 * k as usize;
        let x = i16_at(obj, at).wrapping_sub(sx).wrapping_sub(9);
        let y = i16_at(obj, at + 2).wrapping_sub(sy).wrapping_add(vy).wrapping_sub(9);
        host.blit(scratch[0], scratch[1], x, y, scratch[2], scratch[3]);
        host.text_mode(3);
        host.font(FONT_SMALL);
        let label = waypoint_labels.get(k as usize).map(String::as_str).unwrap_or("");
        host.text(x.wrapping_add(5), y.wrapping_sub(5), label);
    }
    host.part(NavPart::Icebergs, clip);
    host.part(NavPart::Fading, clip);
    host.part(NavPart::Ships, clip);
    widgets(host, 0xD8, 2);
    widgets(host, 0x1A28, 3);
    widgets(host, 0xAF8, 4);
    for k in 0..8 {
        let off = 0x10E0 + 0xD8 * k;
        if host.widget_running(off) {
            host.widget_draw(off, args.dest, wargs);
            break;
        }
    }
    widgets(host, 0x17A0, 1);
    widgets(host, 0x1878, 1);
    host.part(NavPart::Weather, clip);
    host.color(prev_colour);
    host.set_dest(prev_dest);
    host.select(prev_tex);
    host.set_clipper(prev_clipper);
    host.free_clipper(clipper);
}

/// `scroll_map_scroll_blit(0x004538E0, dx, dy)`: al desplazar la carta mueve lo ya dibujado con `sgl_Blt` y marca para
/// redibujar la zona que queda al descubierto y lo que el desplazamiento arrastra (cursor, ventana flotante, objetos
/// superpuestos y arrastre). Con `+0x2B24`, `+0x2B22`, `+0x10 == 0` o `+0x2B2B == 0` no copia y pide el redibujo completo
/// (`+0x2B2B = 1`). Conserva las asimetrías del original: el cursor y los superpuestos se comparan con el **ancho** de la
/// vista (`+0x2C`) como si fuese su borde derecho, y la ventana flotante solo se desplaza en vertical.
pub fn scroll_blit(obj: &mut [u8], dx: i32, dy: i32, host: &mut dyn NavHost) {
    if obj[0x2B24] != 0 || obj[0x2B22] != 0 || r32(obj, 0x10) == 0 || obj[0x2B2B] == 0 {
        obj[0x2B2B] = 1;
        return;
    }
    if dx == 0 && dy == 0 {
        return;
    }
    let (vx, vy) = (i32_at(obj, 0x14), i32_at(obj, 0x18));
    let (mut w, mut h) = (i32_at(obj, 0x2C), i32_at(obj, 0x30));
    let (mut src_x, mut dst_x, mut src_y, mut dst_y) = (vx, vx, vy, vy);
    if dx > 0 {
        w = w.wrapping_sub(dx);
        src_x = vx.wrapping_add(dx);
    } else {
        w = w.wrapping_add(dx);
        dst_x = vx.wrapping_sub(dx);
    }
    if dy > 0 {
        h = h.wrapping_sub(dy);
        src_y = vy.wrapping_add(dy);
    } else {
        h = h.wrapping_add(dy);
        dst_y = vy.wrapping_sub(dy);
    }
    if w <= 0 || h <= 0 {
        return;
    }
    host.select(-3i32 as u32);
    host.set_dest(-3i32 as u32);
    if host.screen_blt(src_x, src_y, dst_x, dst_y, w, h, 4) != 0 {
        return;
    }
    host.add_dirty([dst_x, dst_y, dst_x.wrapping_add(w), dst_y.wrapping_add(h)]);
    let width = i32_at(obj, 0x2C);
    let shift = |r: [i32; 4]| [r[0].wrapping_sub(dx), r[1].wrapping_sub(dy), r[2].wrapping_sub(dx), r[3].wrapping_sub(dy)];
    let rect = |x: i32, y: i32, w: i32, h: i32| [x, y, x.wrapping_add(w), y.wrapping_add(h)];
    let (cx, cy) = host.game_cursor();
    let (cw, ch) = host.cursor_size();
    let r = rect(cx, cy, cw, ch);
    if r[3] >= vy && r[0] <= width {
        host.repaint(r);
    }
    let (lx, ly) = host.last_cursor();
    let r = shift(rect(lx, ly, cw, ch));
    if r[3] >= vy && r[0] <= width {
        host.repaint(r);
    }
    if let Some(t) = host.tooltip() {
        let r = rect(t[0], t[1], t[2], t[3]);
        host.repaint(r);
        host.repaint([r[0], r[1].wrapping_sub(dy), r[2], r[3].wrapping_sub(dy)]);
    }
    let (a, b) = host.overlays();
    for o in a.iter().chain(b.iter()) {
        let r = [o[0], o[1], o[0].wrapping_add(o[2]).wrapping_add(1), o[1].wrapping_add(o[3]).wrapping_add(1)];
        host.repaint(r);
        host.repaint(shift(r));
    }
    if let Some(r) = host.drag_rect() {
        if r[0] < width || r[3] > vy {
            host.repaint(shift(r));
        }
    }
}

/// Zonas (`x1, y1, x2, y2` en coordenadas del mapa) y pesos de las animaciones ambientales de la carta (`0x0066D8C8` y
/// `0x0066D918`, 10 zonas; `0x0066D7E8`/`0x0066D858` y `0x0066D968`/`0x0066D9D8`, 14 zonas). Los pesos suman 100.
pub const AMBIENT_ZONES_A: [([i16; 4], f64); 10] = [
    ([68, 68, 580, 189], 20.35),
    ([311, 190, 579, 903], 62.45),
    ([237, 375, 310, 439], 0.78),
    ([247, 591, 310, 705], 1.2),
    ([216, 706, 310, 825], 1.85),
    ([580, 540, 957, 636], 5.96),
    ([580, 637, 723, 700], 1.5),
    ([1440, 68, 1507, 180], 2.5),
    ([1560, 327, 1610, 474], 2.45),
    ([1611, 351, 1716, 378], 0.96),
];
pub const AMBIENT_ZONES_C: [([i16; 4], f64); 14] = [
    ([306, 68, 597, 543], 21.96),
    ([277, 544, 789, 726], 14.83),
    ([204, 727, 672, 903], 13.11),
    ([336, 904, 663, 1290], 20.05),
    ([334, 1291, 420, 1449], 2.19),
    ([792, 545, 978, 669], 3.69),
    ([930, 670, 1029, 828], 2.51),
    ([972, 829, 1068, 906], 1.2),
    ([1110, 1101, 1328, 1173], 2.53),
    ([1329, 1040, 1614, 1138], 4.47),
    ([1299, 937, 1590, 1041], 4.84),
    ([1440, 804, 1590, 936], 3.17),
    ([1560, 442, 1611, 798], 2.93),
    ([1560, 351, 1734, 441], 2.52),
];
pub const AMBIENT_ZONES_D: [([i16; 4], f64); 14] = [
    ([306, 68, 597, 543], 22.51),
    ([277, 544, 789, 726], 15.2),
    ([204, 727, 672, 903], 13.44),
    ([336, 904, 663, 1279], 19.97),
    ([334, 1280, 417, 1433], 2.09),
    ([792, 545, 978, 669], 3.79),
    ([930, 670, 1029, 828], 2.58),
    ([972, 829, 1068, 890], 0.97),
    ([1110, 1101, 1328, 1173], 2.59),
    ([1329, 1040, 1535, 1148], 3.65),
    ([1299, 937, 1590, 1041], 4.97),
    ([1440, 804, 1590, 936], 3.25),
    ([1560, 418, 1609, 798], 3.09),
    ([1560, 351, 1734, 417], 1.9),
];

/// `rand() / 32768.0` guardado en `float` (`0x0066CB34`).
fn unit(host: &mut dyn NavHost) -> f32 {
    (f64::from(host.rand() as i32 as f32) / 32768.0) as f32
}

/// Zona elegida por peso y punto al azar dentro (`x`, `y` con la vista `+0x18` sumada). `None` si el sorteo pasa del total.
fn ambient_place(zones: &[([i16; 4], f64)], vy: i32, host: &mut dyn NavHost) -> Option<(i32, i32)> {
    use crate::navmap::ftol;
    let p = (f64::from(unit(host)) * 100.0) as f32;
    let mut acc = 0f32;
    let mut pick = None;
    for (k, (_, w)) in zones.iter().enumerate() {
        acc = (f64::from(acc) + w) as f32;
        if p < acc {
            pick = Some(k);
            break;
        }
    }
    let z = zones[pick?].0;
    let f = |v: i32| f64::from(v as f32);
    let t = unit(host);
    let x = ftol(f(i32::from(z[2]) - i32::from(z[0])) * f64::from(t) + f(i32::from(z[0])));
    let t = unit(host);
    let y = ftol(f(i32::from(z[3]) - i32::from(z[1])) * f64::from(t) + f(vy) + f(i32::from(z[1])));
    Some((x, y))
}

fn rect_of_widget(host: &mut dyn NavHost, off: u32) -> [i32; 4] {
    let g = host.widget_geom(off);
    [g[0], g[1], g[0].wrapping_add(g[2]), g[1].wrapping_add(g[3])]
}

/// `IntersectRect`: la intersección, o ceros si no la hay (el original reutiliza el resultado vacío).
fn intersect_or_zero(a: &[i32; 4], b: &[i32; 4]) -> ([i32; 4], bool) {
    let r = [a[0].max(b[0]), a[1].max(b[1]), a[2].min(b[2]), a[3].min(b[3])];
    let empty = |r: &[i32; 4]| r[2] <= r[0] || r[3] <= r[1];
    if empty(a) || empty(b) || empty(&r) {
        ([0; 4], false)
    } else {
        (r, true)
    }
}

/// `scroll_map_update_ambient_animations(0x00458050)`: coloca, arranca y repinta las animaciones del mar de la carta
/// (controles `Animation2D` incrustados en el objeto). Grupos: dos en `+0xD8` (zonas A, una vez de cada cien, lejos de los
/// barcos visibles y de los del grupo `+0xAF8`), diez en `+0x288` (en cualquier punto navegable de la vista, una de cada
/// diez), tres en `+0xE58` (zonas C), cuatro en `+0xAF8` (zonas D, sin pisar `+0xD8`/`+0x1B0`), las ocho direcciones del
/// barco fantasma `+0x10E0` (uno visible, que cambia de dirección cuando vence `+0x2910` y avanza 0,2 por fotograma en
/// `+0x29CC`/`+0x29D0`), y tres fijos (`+0x17A0`, `+0x1878` y el marcador `+0x1950` en `+0x29D4`/`+0x29D8`).
/// `visible` es la lista de barcos visibles (`+0x28A4`, recuento `+0x2908`) y `clock` `DAT_006DCCF0`.
///
/// Quirks conservados: en el grupo `+0xAF8` se repinta el rectángulo vacío que deja `IntersectRect`, y al cambiar de
/// dirección el barco fantasma repinta el rectángulo del control **anterior** al nuevo (`+0x1008 + 0xD8·k`).
pub fn update_ambient_animations(obj: &mut [u8], clock: u32, visible: &[u32], world: &crate::navmap::World, host: &mut dyn NavHost) {
    use crate::navmap::{ftol, w32};
    let (vx, vy) = (i32_at(obj, 0x14), i32_at(obj, 0x18));
    let view = [vx, vy, vx.wrapping_add(i32_at(obj, 0x2C)), vy.wrapping_add(i32_at(obj, 0x30))];
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let show = |host: &mut dyn NavHost, off: u32| {
        let r = rect_of_widget(host, off);
        let (r, ok) = intersect_or_zero(&r, &view);
        if ok {
            host.repaint(r);
            host.dirty(r);
        }
    };
    let put = |obj: &mut [u8], at: usize, v: i32| w32(obj, at, v as u32);
    let place = |obj: &[u8], host: &mut dyn NavHost, off: u32, px: usize, py: usize| {
        host.widget_move(off, i32_at(obj, px).wrapping_sub(sx), i32_at(obj, py).wrapping_sub(sy), 100);
    };
    // Dos en +0xD8, zonas A.
    for k in 0..2u32 {
        let off = 0xD8 + 0xD8 * k;
        let (px, py) = (0x2934 + 4 * k as usize, 0x293C + 4 * k as usize);
        if host.widget_running(off) {
            place(obj, host, off, px, py);
            show(host, off);
            continue;
        }
        if let Some((x, y)) = ambient_place(&AMBIENT_ZONES_A, vy, host) {
            put(obj, px, x);
            put(obj, py, y);
        }
        place(obj, host, off, px, py);
        if !(f64::from(unit(host)) < 0.01) {
            continue;
        }
        let own = rect_of_widget(host, off);
        let (r, ok) = intersect_or_zero(&own, &view);
        if !ok {
            continue;
        }
        let mut clash = false;
        for j in 0..4u32 {
            let o = 0xAF8 + 0xD8 * j;
            if host.widget_running(o) {
                let q = rect_of_widget(host, o);
                if intersect_or_zero(&q, &own).1 {
                    clash = true;
                    break;
                }
            }
        }
        if clash {
            continue;
        }
        let (x0, y0) = (i32_at(obj, px), i32_at(obj, py));
        let n = r32(obj, 0x2908) as usize;
        let near = (0..n).any(|j| {
            let s = world.ship(visible.get(j).copied().unwrap_or(0));
            let dx = i16_at(s, 0x1E).wrapping_sub(x0);
            let dy = i16_at(s, 0x22).wrapping_add(vy).wrapping_sub(y0);
            dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < 0x9C40
        });
        if near {
            continue;
        }
        host.repaint(r);
        host.dirty(r);
        host.widget_timer(off, 1, 0);
    }
    // Diez en +0x288, en cualquier punto navegable de la vista.
    for k in 0..10u32 {
        let off = 0x288 + 0xD8 * k;
        let (px, py) = (0x2944 + 4 * k as usize, 0x296C + 4 * k as usize);
        if host.widget_running(off) {
            place(obj, host, off, px, py);
            show(host, off);
            continue;
        }
        let f = |v: i32| f64::from(v as f32);
        let t = unit(host);
        let x = ftol((f(i32_at(obj, 0x2C)) * f64::from(t) + f(sx)) + f64::from(68f32));
        put(obj, px, x);
        let t = unit(host);
        let y = ftol(((f(i32_at(obj, 0x30)) * f64::from(t) + f(vy)) + f(sy)) + f64::from(68f32));
        put(obj, py, y);
        let nx = (x as u16).wrapping_sub(0x44) as i16;
        let ny = (y as u16).wrapping_sub(vy as u16).wrapping_sub(0x44) as i16;
        let nav = host.navigable(nx, ny);
        place(obj, host, off, px, py);
        let go = f64::from(unit(host)) > 0.9;
        if nav && go {
            host.widget_timer(off, 1, 0);
        }
    }
    // Tres en +0xE58, zonas C.
    for k in 0..3u32 {
        let off = 0xE58 + 0xD8 * k;
        let (px, py) = (0x29B4 + 4 * k as usize, 0x29C0 + 4 * k as usize);
        if host.widget_running(off) {
            place(obj, host, off, px, py);
            show(host, off);
            continue;
        }
        if let Some((x, y)) = ambient_place(&AMBIENT_ZONES_C, vy, host) {
            put(obj, px, x);
            put(obj, py, y);
        }
        place(obj, host, off, px, py);
        if !(f64::from(unit(host)) > 0.9) {
            continue;
        }
        let (r, ok) = intersect_or_zero(&rect_of_widget(host, off), &view);
        if ok {
            host.repaint(r);
            host.dirty(r);
            host.widget_timer(off, 1, 0);
        }
    }
    // Cuatro en +0xAF8, zonas D, sin pisar +0xD8 ni +0x1B0.
    for k in 0..4u32 {
        let off = 0xAF8 + 0xD8 * k;
        let (px, py) = (0x2994 + 4 * k as usize, 0x29A4 + 4 * k as usize);
        if host.widget_running(off) {
            place(obj, host, off, px, py);
            show(host, off);
            continue;
        }
        if let Some((x, y)) = ambient_place(&AMBIENT_ZONES_D, vy, host) {
            put(obj, px, x);
            put(obj, py, y);
        }
        place(obj, host, off, px, py);
        if !(f64::from(unit(host)) < 0.01) {
            continue;
        }
        let (a, ok) = intersect_or_zero(&rect_of_widget(host, off), &view);
        if !ok {
            continue;
        }
        let own = rect_of_widget(host, off);
        let c = rect_of_widget(host, 0xD8);
        let d = rect_of_widget(host, 0x1B0);
        let (d, hit_d) = intersect_or_zero(&own, &d);
        let (_, hit_c) = intersect_or_zero(&own, &c);
        if !hit_d && !hit_c {
            host.repaint(d);
            host.dirty(a);
            host.widget_timer(off, 1, 0);
        }
    }
    // Barco fantasma: ocho controles por dirección en +0x10E0.
    let ghost = |k: i32| 0x10E0 + 0xD8 * k as u32;
    let first = |host: &mut dyn NavHost| (0..8).find(|&k| host.widget_running(ghost(k)));
    let read_f = |obj: &[u8], at: usize| f32::from_bits(r32(obj, at));
    if clock > r32(obj, 0x2910) {
        let n = ftol(f64::from(host.rand() as i32 as f32) / -32768.0 * 10000.0);
        w32(obj, 0x2910, clock.wrapping_sub(n as u32));
        if let Some(mut k) = first(host) {
            host.widget_timer(ghost(k), 0, 0);
            if f64::from(unit(host)) > 0.5 {
                k += 1;
                if k > 7 {
                    k = 0;
                }
            } else {
                k -= 1;
                if k < 0 {
                    k = 7;
                }
            }
            host.widget_timer(ghost(k), 1, 1);
            let y = ftol(f64::from(read_f(obj, 0x29D0)));
            let x = ftol(f64::from(read_f(obj, 0x29CC)));
            host.widget_move(ghost(k), x.wrapping_sub(sx), y.wrapping_sub(sy), 100);
            show(host, 0x1008 + 0xD8 * k as u32);
        }
    } else if let Some(k) = first(host) {
        let step = f64::from(0.2f32);
        let add = |obj: &mut [u8], at: usize, d: f64| {
            let v = (f64::from(f32::from_bits(r32(obj, at))) + d) as f32;
            w32(obj, at, v.to_bits());
        };
        let (dx, dy): (f64, f64) = match k {
            0 => (step, -step),
            1 => (step, 0.0),
            2 => (step, step),
            3 => (0.0, step),
            4 => (-step, step),
            5 => (-step, 0.0),
            6 => (-step, -step),
            _ => (0.0, -step),
        };
        if dx != 0.0 {
            add(obj, 0x29CC, dx);
        }
        if dy != 0.0 {
            add(obj, 0x29D0, dy);
        }
        let y = ftol(f64::from(read_f(obj, 0x29D0)));
        let x = ftol(f64::from(read_f(obj, 0x29CC)));
        host.widget_move(ghost(k), x.wrapping_sub(sx), y.wrapping_sub(sy), 100);
        show(host, ghost(k));
    }
    // Fijos: +0x17A0 y +0x1878 arrancan en cuanto paran; +0x1950 sigue a +0x29D4/+0x29D8 con margen de 5.
    for (off, x, y) in [(0x17A0u32, 0x66C, 0x661), (0x1878, 0x7C4, 0x262)] {
        let running = host.widget_running(off);
        host.widget_move(off, x - sx, y - sy, 100);
        if running {
            show(host, off);
        } else {
            host.widget_timer(off, 1, 1);
        }
    }
    if host.widget_running(0x1950) {
        host.widget_move(0x1950, i32_at(obj, 0x29D4).wrapping_sub(sx), i32_at(obj, 0x29D8).wrapping_sub(sy), 100);
        let g = host.widget_geom(0x1950);
        let (l, t) = (g[0] - 5, g[1] - 5);
        let r = [l, t, l.wrapping_add(g[2]).wrapping_add(10), t.wrapping_add(g[3]).wrapping_add(10)];
        let (r, ok) = intersect_or_zero(&r, &view);
        if ok {
            host.repaint(r);
            host.dirty(r);
        }
    }
}
