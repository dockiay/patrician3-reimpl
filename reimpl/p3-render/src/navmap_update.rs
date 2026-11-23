//! Fotograma de la carta náutica (`nav_map_update`, `0x0044DCE0`) entero, con paridad x86 (`oracle/x86_navmap_draw.py
//! --only update`, fixture `tests/fixtures/navmap_draw_update.json`).
//!
//! Orden del original:
//!
//! 1. Repinta y libera los rectángulos pendientes (`+0x28D0`, `+0x2928`), pide la selección (`0x004885A0`) y el ratón.
//! 2. Con un único barco elegido, comprueba si el cursor está sobre su ruta: primero lee el píxel del casco bajo el cursor
//!    (`0x004BBB00`, textura de 256 píxeles de ancho, fotogramas de 52 × 56 en filas de cuatro); si el casco no está debajo,
//!    pregunta a `0x00446A00` y cambia el color de la ruta (`+0x2920`, `0xFF687888` normal, `0xFFA6B9C7` resaltada). Con el
//!    cursor fuera de la vista cancela la vista previa de ruta (`+0x2B23`, `+0xA8`).
//! 3. Desplazamiento ([`crate::navmap::scroll_frame`]), aviso de tiempo de viaje (`0x00451230`), copia incremental
//!    (`0x004538E0`), minimapa y rectángulo de arrastre (`+0x94..+0xA0`).
//! 4. Ciudad bajo el cursor (`0x00446D50` en línea, `+0x2B04`; la ciudad natal del jugador da `0x27`) y repintado de su símbolo.
//! 5. Iconos, barcos visibles, vista previa (cada 100 ms), animaciones y tiempo.
//! 6. Recuento de barcos propios en puerto por ciudad (`+0x28C4` solos, `+0x28C8` en convoy; solo estados `< 4`) y repintado de
//!    los rótulos que cambian; repintado de los barcos de la lista `+0x28A8`.
//! 7. Flecha parpadeante sobre los barcos elegidos (`+0x2B0C` sube y baja hasta `+0x2B08` cada `+0x2B0A` ms) y sobre el barco
//!    señalado `+0x2B14` (se desvanece con `+0x2B1C`/`+0x2B1D` cada 35 ms del reloj `0x006DCCF8`).
//!
//! Las funciones ya portadas aparte se piden al anfitrión como [`UpdateStep`]. Igual que en el original, el rectángulo de
//! fotograma, el rectángulo auxiliar y la textura de casco que quedan sin iniciar se arrastran ([`UpdateCarry`]).

use crate::navmap::{
    ftol, minimap_rect, r16, r32, r8, scroll_frame, view_rect, w16, w32, w8, NavHost, ScrollInput, Textures, World, ARROW_TEXTURE,
};
use crate::navmap_draw::Towns;

fn i16_at(m: &[u8], o: usize) -> i32 {
    i32::from(r16(m, o) as i16)
}
fn i32_at(m: &[u8], o: usize) -> i32 {
    r32(m, o) as i32
}

/// Color de la ruta del barco elegido (`+0x2920`).
pub const ROUTE_COLOR: u32 = 0xFF68_7888;
pub const ROUTE_COLOR_HOVER: u32 = 0xFFA6_B9C7;

/// Funciones de la carta que `0x0044DCE0` llama sin usar su resultado.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateStep {
    /// `0x00621750` (vacía).
    Begin,
    /// `0x0045A470`: posiciones de los sonidos ambientales.
    Sounds,
    /// `0x0045A6B0`: confirma la ruta por puntos de los barcos elegidos.
    Waypoint,
    /// `0x00451230(x, y)`: aviso de tiempo de viaje bajo el cursor.
    TravelHint(i32, i32),
    /// `0x004538E0(dx, dy)` ([`crate::navmap_draw::scroll_blit`]).
    ScrollBlit(i32, i32),
    /// `0x0044D400`: selección por rectángulo de arrastre.
    Drag,
    /// `0x00453EE0`: iconos de estado de las ciudades.
    Status,
    /// `0x00454430`: iconos de presencia de barcos en las ciudades.
    Presence,
    /// `0x004516B0` ([`crate::navmap::visible_ships`]).
    Visible,
    /// `0x00451EB0`: vista previa de la ruta.
    Preview,
    /// `0x00458050` ([`crate::navmap_draw::update_ambient_animations`]).
    Ambient,
    /// `0x00459380`: tormentas y calmas.
    Weather,
}

/// Lo que `0x0044DCE0` lee fuera de la imagen de la carta.
#[derive(Clone, Copy, Debug, Default)]
pub struct UpdateInput<'a> {
    /// Rectángulos pendientes de `+0x28D0` (el original repinta `+0x2928` elementos).
    pub dirty_rects: &'a [[i32; 4]],
    /// Puntero de la selección que devuelve `0x004885A0` (se compara con `+0x28C0`) y sus barcos.
    pub selection_ptr: u32,
    pub selection: &'a [u16],
    /// Ratón (`0x004B77F0`) y resolución (`0x006DCCE0`).
    pub mouse: (i32, i32),
    pub screen: (i32, i32),
    /// `DAT_006DD5DE` (diálogo modal), `0x00462310` (ventana emergente) y `+0x94` de `0x006CC940` (arrastre del minimapa).
    pub modal: u8,
    pub popup: bool,
    pub minimap_drag: u8,
    /// `DAT_006DCCF0` y `DAT_006DCCF8`.
    pub clock: u32,
    pub clock2: u32,
    /// Manejador de la textura de fondo (`+0xBC`).
    pub background: u32,
    /// `merchant+0xE1` del jugador.
    pub home_town: u8,
    /// Las dos dobles palabras del puntero `ship+0x24` del barco elegido (las pasa a `0x00446A00`).
    pub route_head: [u32; 2],
    /// `*(+0xA8)` (el original libera ese puntero y luego `+0xA8`).
    pub preview_inner: u32,
    /// Barcos de la lista `+0x28A8` (`+0x2924` elementos).
    pub frame_list: &'a [u32],
}

/// Valores de pila del original sin iniciar: rectángulo de fotograma `[esp+0x28]`, rectángulo auxiliar `[esp+0x4C]` y
/// manejador de la textura de casco `[esp+0x78]` (tipos de casco mayores que 3).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UpdateCarry {
    pub rect: [i32; 4],
    pub scratch: [i32; 4],
    pub tex: u32,
}

fn intersect(a: &[i32; 4], b: &[i32; 4]) -> Option<[i32; 4]> {
    let r = [a[0].max(b[0]), a[1].max(b[1]), a[2].min(b[2]), a[3].min(b[3])];
    let empty = |r: &[i32; 4]| r[2] <= r[0] || r[3] <= r[1];
    if empty(a) || empty(b) || empty(&r) {
        None
    } else {
        Some(r)
    }
}

fn hull_frame(tex: &Textures, kind: u8, frame: i32, carry: &mut [i32; 4]) -> bool {
    match tex.hull(kind).and_then(|t| t.frame(frame)) {
        Some((_, f)) => {
            *carry = f;
            true
        }
        None => false,
    }
}

fn repaint_dirty(r: [i32; 4], view: &[i32; 4], host: &mut dyn NavHost) {
    if let Some(r) = intersect(&r, view) {
        host.repaint(r);
        host.dirty(r);
    }
}

/// `0x0044DCE0`. `ports` son las tablas `+0x28C4` (barcos solos) y `+0x28C8` (en convoy), una casilla por posición de ciudad.
#[allow(clippy::too_many_arguments)]
pub fn nav_map_update(
    obj: &mut [u8],
    world: &World,
    tex: &Textures,
    towns: &Towns,
    input: &UpdateInput,
    keys: &mut [u8; 4],
    ports: &mut [Vec<u8>; 2],
    carry: &mut UpdateCarry,
    host: &mut dyn NavHost,
) {
    host.update_step(UpdateStep::Begin);
    w8(obj, 0x2B25, 0);
    host.update_step(UpdateStep::Sounds);
    w8(obj, 0x2B28, 0);
    for k in 0..i32_at(obj, 0x2928).max(0) {
        host.repaint(input.dirty_rects.get(k as usize).copied().unwrap_or_default());
    }
    let p = r32(obj, 0x28D0);
    if p != 0 {
        host.free(p);
    }
    for o in [0x28D0, 0x2928, 0x58, 0x50, 0x54, 0x4C] {
        w32(obj, o, 0);
    }
    let sel = input.selection;
    let count = sel.len() as i32;
    let (mx, my) = input.mouse;
    let mut dirtied = false;
    host.get_merchant(world.player);
    if r32(obj, 0x28C0) != input.selection_ptr {
        carry.scratch = view_rect(obj);
        host.repaint(carry.scratch);
        w32(obj, 0x28C0, input.selection_ptr);
    }
    if r8(obj, 0x2B27) & 1 != 0 && count == 0 {
        carry.scratch = view_rect(obj);
        host.repaint(carry.scratch);
        w8(obj, 0x2B27, 0);
    }
    let active = !(input.popup || input.modal != 0);
    w8(obj, 0x2B21, u8::from(active));
    let mut scroll = crate::navmap::ScrollOut::default();
    let mut si = ScrollInput { active: false, mouse: input.mouse, screen: input.screen, clock: input.clock, keys: *keys, background: input.background };
    if active && input.minimap_drag == 0 {
        if count == 1 {
            dirtied |= hover_route(obj, world, tex, input, carry, host);
        }
        host.update_step(UpdateStep::Waypoint);
        si.active = true;
        scroll = scroll_frame(obj, &mut si, host);
        host.update_step(UpdateStep::TravelHint(mx, my));
    } else {
        scroll_frame(obj, &mut si, host);
        w16(obj, 0x2B04, 0xFF);
    }
    *keys = si.keys;
    let view = view_rect(obj);
    if scroll.moved_x || scroll.moved_y {
        carry.scratch = view;
        host.repaint(view);
        let (dx, dy) = scroll.delta();
        if !dirtied && r8(obj, 0x2B26) & 1 != 0 {
            host.update_step(UpdateStep::ScrollBlit(dx, dy));
        }
        if dx != 0 {
            w8(obj, 0x2B28, 1);
        }
        w8(obj, 0x2B26, 1);
        host.minimap_view(minimap_rect(obj));
        if r8(obj, 0x2B1F) != 0 {
            drag_box(obj, input.mouse);
        }
    }
    if i32_at(obj, 0x98) != -1 {
        host.update_step(UpdateStep::Drag);
    }
    let inside = mx >= view[0] && mx < view[2] && my >= view[1] && my < view[3];
    if inside && r8(obj, 0x2B24) == 0 && r8(obj, 0x2B23) == 0 && r8(obj, 0x2B21) & 1 != 0 {
        hover_town(obj, world, towns, input, &mut carry.scratch, host);
    }
    host.update_step(UpdateStep::Status);
    host.update_step(UpdateStep::Presence);
    host.update_step(UpdateStep::Visible);
    if r8(obj, 0x2B23) != 0 && input.clock.wrapping_sub(r32(obj, 0x2914)) > 100 {
        host.update_step(UpdateStep::Preview);
        w32(obj, 0x2914, input.clock);
    }
    host.update_step(UpdateStep::Ambient);
    host.update_step(UpdateStep::Weather);
    count_ports(obj, world, towns, ports, &view, host);
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let (vy, vw, vh) = (i32_at(obj, 0x18), i32_at(obj, 0x2C), i32_at(obj, 0x30));
    for k in 0..i32_at(obj, 0x2924).max(0) {
        let ship = world.ship(input.frame_list.get(k as usize).copied().unwrap_or(0));
        let x = i16_at(ship, 0x1E).wrapping_sub(sx);
        let y = i16_at(ship, 0x22).wrapping_sub(sy).wrapping_add(vy);
        hull_frame(tex, r8(ship, 0xE), i32::from(r8(ship, 1)), &mut carry.rect);
        let x = x.wrapping_add((carry.rect[2] / 2).wrapping_neg());
        let y = y.wrapping_add((carry.rect[3] / 2).wrapping_neg());
        let r = [
            x.wrapping_sub(0x14),
            y.wrapping_sub(0x14),
            carry.rect[2].wrapping_add(x).wrapping_add(0x14),
            carry.rect[3].wrapping_add(y).wrapping_add(0x14),
        ];
        if r[1] < vh.wrapping_add(vy) && r[3] > vy && r[0] < vw && r[2] > 0 {
            repaint_dirty(r, &view, host);
        }
    }
    if r8(obj, 0x2B22) != 0 {
        carry.scratch = view;
        host.repaint(view);
        w8(obj, 0x2B22, 0);
    }
    w32(obj, 0x290C, input.clock);
    if count != 0 {
        w8(obj, 0x2B29, 1);
    } else {
        w8(obj, 0x2B29, 0);
        w16(obj, 0x2B0C, 10);
    }
    if r8(obj, 0x2B29) != 0 && r32(obj, 0x2B10) < input.clock {
        if r8(obj, 0x2B2A) != 0 {
            let v = r16(obj, 0x2B0C).wrapping_sub(1);
            w16(obj, 0x2B0C, v);
            if v == 0 {
                w8(obj, 0x2B2A, 0);
            }
        } else {
            let v = r16(obj, 0x2B0C).wrapping_add(1);
            w16(obj, 0x2B0C, v);
            if v >= r16(obj, 0x2B08) {
                w8(obj, 0x2B2A, 1);
            }
        }
        w32(obj, 0x2B10, input.clock.wrapping_add(u32::from(r16(obj, 0x2B0A))));
        host.lookup(ARROW_TEXTURE);
        let (aw, ah) = host.size(tex.lookup(ARROW_TEXTURE).handle);
        let mut h = carry.rect[3];
        let bounce = i32::from(r16(obj, 0x2B0C));
        for &id in sel {
            let mut ship = world.ship(u32::from(id));
            let c = r16(ship, 8);
            if c < world.convoy_count {
                ship = world.ship(u32::from(r16(world.convoy(u32::from(c)), 0x10)));
            }
            let x = i16_at(ship, 0x1E).wrapping_sub(sx);
            let y = i16_at(ship, 0x22).wrapping_sub(sy).wrapping_add(vy);
            let mut f = [carry.rect[0], carry.rect[1], carry.rect[2], h];
            if hull_frame(tex, r8(ship, 0xE), i32::from(r8(ship, 1)), &mut f) {
                carry.rect[..3].copy_from_slice(&f[..3]);
                h = f[3];
            }
            let left = x.wrapping_sub(aw / 2).wrapping_sub(5);
            let top = y.wrapping_sub(h / 2).wrapping_sub(bounce).wrapping_sub(ah).wrapping_sub(5);
            let r = [left, top, aw.wrapping_add(left).wrapping_add(0xA), top.wrapping_add(ah).wrapping_add(0xA)];
            repaint_dirty(r, &view, host);
        }
    }
    if r16(obj, 0x2B14) < world.ship_count {
        host.lookup(ARROW_TEXTURE);
        let (aw, ah) = host.size(tex.lookup(ARROW_TEXTURE).handle);
        let ship = world.ship(u32::from(r16(obj, 0x2B14)));
        let x = i16_at(ship, 0x1E).wrapping_sub(sx);
        let y = i16_at(ship, 0x22).wrapping_sub(sy).wrapping_add(vy);
        let mut f = carry.rect;
        let h = if hull_frame(tex, r8(ship, 0xE), i32::from(r8(ship, 1)), &mut f) {
            carry.rect[..3].copy_from_slice(&f[..3]);
            f[3]
        } else {
            carry.rect[3]
        };
        let left = x.wrapping_sub(aw / 2).wrapping_sub(2);
        let top = y.wrapping_sub(h / 2).wrapping_sub(ah).wrapping_sub(2);
        let r = [left, top, aw.wrapping_add(left).wrapping_add(4), top.wrapping_add(ah).wrapping_add(4)];
        repaint_dirty(r, &view, host);
        if input.clock2 > r32(obj, 0x2B18) {
            let v = r32(obj, 0x2B18).wrapping_add(0x23);
            w32(obj, 0x2B18, if v < input.clock2 { input.clock2 } else { v });
            let a = r8(obj, 0x2B1D) as i8;
            let s = i32::from(r8(obj, 0x2B1C)) + i32::from(a);
            if s > 0xFF {
                w8(obj, 0x2B1C, 0xFF);
                w8(obj, 0x2B1D, a.wrapping_neg() as u8);
            } else if s < 0 {
                w8(obj, 0x2B1C, 0);
                w8(obj, 0x2B1D, a.wrapping_neg() as u8);
                w16(obj, 0x2B14, world.ship_count);
            } else {
                w8(obj, 0x2B1C, s as u8);
            }
        }
    }
}

/// `0x0044DECC`–`0x0044E246`: ruta del único barco elegido bajo el cursor. Devuelve si hay que repintar la vista.
fn hover_route(obj: &mut [u8], world: &World, tex: &Textures, input: &UpdateInput, carry: &mut UpdateCarry, host: &mut dyn NavHost) -> bool {
    let (mx, my) = input.mouse;
    let ship = world.ship(u32::from(input.selection[0]));
    let route = r32(ship, 0x24);
    let (vx, vy, vw, vh) = (i32_at(obj, 0x14), i32_at(obj, 0x18), i32_at(obj, 0x2C), i32_at(obj, 0x30));
    carry.scratch[1] = mx;
    carry.scratch[2] = my.wrapping_sub(vy);
    let old = r32(obj, 0x2920);
    w32(obj, 0x2920, ROUTE_COLOR);
    let kind = r8(ship, 0xE);
    let frame = i32::from(r8(ship, 1));
    hull_frame(tex, kind, frame, &mut carry.rect);
    let (ship_x, ship_y) = (i16_at(ship, 0x1E), i16_at(ship, 0x22));
    let px = i16_at(obj, 0x29E4).wrapping_add(carry.rect[2] / 2).wrapping_sub(ship_x).wrapping_add(mx);
    let py = i16_at(obj, 0x29E6).wrapping_add(carry.rect[3] / 2).wrapping_sub(ship_y).wrapping_sub(vy).wrapping_add(my);
    carry.rect[1] = py;
    if let Some(t) = tex.hull(kind) {
        carry.tex = t.handle;
    }
    let mut dirtied = false;
    if my <= vy.wrapping_add(vh) && mx <= vx.wrapping_add(vw) && my >= vy && mx >= vx {
        if r32(ship, 0x30) != 0 && route != 0 && r8(obj, 0x2B23) == 0 && i32_at(obj, 0x98) == -1 {
            host.lock_texture(carry.tex);
            let mut on_hull = false;
            if py < 0x38 && px < 0x34 && py > 0 && px > 0 {
                let row = py.wrapping_add(frame / 4 * 56).wrapping_shl(8);
                let i = px.wrapping_add(row).wrapping_add(frame % 4 * 52);
                on_hull = host.texture_pixel(carry.tex, i) != 0;
            }
            host.unlock_texture(carry.tex);
            if !on_hull {
                let args = [
                    input.route_head[0] as i32,
                    input.route_head[1] as i32,
                    carry.scratch[0],
                    carry.scratch[1],
                    carry.scratch[2],
                    carry.scratch[3],
                    i32::from(r16(ship, 0x34)).wrapping_sub(1),
                    ship_x,
                    ship_y,
                ];
                if host.route_hit(args) {
                    w32(obj, 0x2920, ROUTE_COLOR_HOVER);
                    dirtied = true;
                }
            }
        }
    } else if r8(obj, 0x2B23) != 0 {
        w8(obj, 0x2B23, 0);
        w16(obj, 0x28F8, 0xFFFF);
        w16(obj, 0x28FA, 0xFFFF);
        let p = r32(obj, 0xA8);
        if p != 0 {
            host.free(input.preview_inner);
            host.free(p);
            w32(obj, 0xA8, 0);
        }
    }
    if old != r32(obj, 0x2920) {
        carry.scratch = view_rect(obj);
        host.repaint(carry.scratch);
        dirtied = true;
    }
    dirtied
}

/// `0x0044E88D`–`0x0044E96A`: rectángulo de arrastre (`+0x94` izquierda, `+0x98` arriba, `+0x9C`, `+0xA0`) desde el ancla
/// `+0x28F4`; el original recorta la parte de arriba contra `+0x18` y la de abajo contra `+0x18 + 0x707`.
fn drag_box(obj: &mut [u8], (mx, my): (i32, i32)) {
    let ex = mx.wrapping_add(i16_at(obj, 0x29E4));
    let ax = i16_at(obj, 0x28F4);
    if i32_at(obj, 0x94) > ex {
        w32(obj, 0x9C, ax as u32);
        w32(obj, 0x94, ex as u32);
    } else {
        w32(obj, 0x94, ax as u32);
        w32(obj, 0x9C, ex as u32);
    }
    let ey = i16_at(obj, 0x29E6).wrapping_add(my);
    let ay = i16_at(obj, 0x28F6);
    if i32_at(obj, 0x98) > ey {
        w32(obj, 0xA0, ay as u32);
        w32(obj, 0x98, ey as u32);
    } else {
        w32(obj, 0x98, ay as u32);
        w32(obj, 0xA0, ey as u32);
    }
    let vy = i32_at(obj, 0x18);
    if i32_at(obj, 0x98) < vy {
        w32(obj, 0x98, vy as u32);
    }
    if i32_at(obj, 0xA0) > vy.wrapping_add(0x707) {
        w32(obj, 0xA0, vy.wrapping_add(0x707) as u32);
    }
    if i32_at(obj, 0x94) < 0 {
        w32(obj, 0x94, 0);
    }
    if i32_at(obj, 0x9C) > 0x95F {
        w32(obj, 0x9C, 0x95F);
    }
}

/// `0x0044E9EA`–`0x0044EB78`: ciudad bajo el cursor (`0x00446D50`: dentro del símbolo de 22 × 22 o en el punto de llegada,
/// que se copia a `+0x28FC`).
fn hover_town(obj: &mut [u8], world: &World, towns: &Towns, input: &UpdateInput, scratch: &mut [i32; 4], host: &mut dyn NavHost) {
    let old = u32::from(r16(obj, 0x2B04));
    let found = find_town_at_point(towns, map_point(obj, input.mouse), Some(&mut obj[0x28FC..0x2900]));
    w16(obj, 0x2B04, found as u16);
    host.get_merchant(world.player);
    if input.home_town < 0x28 {
        host.color(0xFF00_0000);
        let t = towns.info_by_id(usize::from(input.home_town));
        let (ox, oy) = (i16_at(obj, 0x28FC), i16_at(obj, 0x28FE));
        let (x, y) = t.symbol;
        let over = y.wrapping_add(0x16) > oy && x.wrapping_add(0x16) > ox && y < oy && x < ox;
        if over || (t.approach.0 == ox && t.approach.1 == oy) {
            w16(obj, 0x2B04, 0x27);
        }
    }
    let new = u32::from(r16(obj, 0x2B04));
    if old != new {
        let n = towns.order.len() as u32;
        let i = if new < n { new } else { old };
        if i < n {
            let t = towns.info(i as usize);
            let x = t.symbol.0.wrapping_sub(i16_at(obj, 0x29E4));
            let y = t.symbol.1.wrapping_sub(i16_at(obj, 0x29E6)).wrapping_add(i32_at(obj, 0x18));
            let r = [x, y, x.wrapping_add(0x1A), y.wrapping_add(0x1A)];
            *scratch = r;
            host.repaint(r);
            host.dirty(r);
        }
    }
}

/// `0x0044EBCA`–`0x0044EE3B`: barcos propios en puerto por ciudad y repintado de los rótulos de puerto que cambian
/// (55 × 25 en `SchiffAufLandXPos/YPos`; el original compara la parte de arriba con `+0x14 + ancho`, no con la altura).
fn count_ports(obj: &[u8], world: &World, towns: &Towns, ports: &mut [Vec<u8>; 2], view: &[i32; 4], host: &mut dyn NavHost) {
    host.get_merchant(world.player);
    let n = towns.order.len();
    for p in ports.iter_mut() {
        p.resize(p.len().max(n), 0);
    }
    let old = [ports[0][..n].to_vec(), ports[1][..n].to_vec()];
    for p in ports.iter_mut() {
        p[..n].fill(0);
    }
    let mut s = u32::from(world.first_ship);
    while s < u32::from(world.ship_count) {
        let ship = world.ship(s);
        let t = usize::from(r8(ship, 0x39));
        if t < n && r16(ship, 0x134) < 4 {
            let convoy = r16(ship, 8) < world.convoy_count;
            let c = &mut ports[usize::from(convoy)][t];
            *c = c.wrapping_add(1);
        }
        s = u32::from(r16(ship, 4));
    }
    let (vx, vy, vw) = (i32_at(obj, 0x14), i32_at(obj, 0x18), i32_at(obj, 0x2C));
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    for i in 0..n {
        if ports[1][i] != old[1][i] || ports[0][i] != old[0][i] {
            let t = towns.info(i);
            let left = t.harbor.0.wrapping_sub(sx);
            let top = t.harbor.1.wrapping_sub(sy).wrapping_add(vy);
            let r = [left, top, left.wrapping_add(0x37), top.wrapping_add(0x19)];
            if top < vw.wrapping_add(vx) && left < vw && r[3] > vx && r[2] > 0 {
                repaint_dirty(r, view, host);
            }
        }
    }
}

fn box_edges(obj: &[u8], host: &mut dyn NavHost) {
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let l = i32_at(obj, 0x94).wrapping_sub(sx);
    let r = i32_at(obj, 0x9C).wrapping_sub(sx);
    let t = i32_at(obj, 0x98).wrapping_sub(sy);
    let b = i32_at(obj, 0xA0).wrapping_sub(sy);
    let (l0, l1, r0, r1) = (l.wrapping_sub(1), l.wrapping_add(1), r.wrapping_sub(1), r.wrapping_add(1));
    let (t0, t1, b0, b1) = (t.wrapping_sub(1), t.wrapping_add(1), b.wrapping_sub(1), b.wrapping_add(1));
    for e in [[l0, t0, l1, b1], [r0, t0, r1, b1], [l0, t0, r1, t1], [l0, b0, r1, b1]] {
        host.repaint(e);
        host.dirty(e);
    }
}

/// `0x0044D400` (`nav_map_update_drag_selection_rect`): repinta los cuatro bordes de 2 píxeles del rectángulo de arrastre,
/// lo rehace entre el ancla `+0x28F4` y el ratón (recortado a la vista: `x < ancho`, `y ≥ +0x18`; si `+0x94` y `+0x98` son
/// negativos el ancla pasa a ser el ratón) y repinta los bordes nuevos. A diferencia de `0x0044E88D`, aquí se compara con el
/// ancla y no con el borde actual.
pub fn drag_selection_rect(obj: &mut [u8], (mx, my): (i32, i32), host: &mut dyn NavHost) {
    box_edges(obj, host);
    let vw = i32_at(obj, 0x2C);
    let mx = if mx >= vw { vw.wrapping_sub(1) } else { mx };
    let vy = i32_at(obj, 0x18);
    let my = if my < vy { vy } else { my };
    if i32_at(obj, 0x98) < 0 && i32_at(obj, 0x94) < 0 {
        let ax = r16(obj, 0x29E4).wrapping_add(mx as u16);
        let ay = r16(obj, 0x29E6).wrapping_add(my as u16);
        w16(obj, 0x28F4, ax);
        w16(obj, 0x28F6, ay);
        let (ax, ay) = (i32::from(ax as i16) as u32, i32::from(ay as i16) as u32);
        w32(obj, 0x94, ax);
        w32(obj, 0x98, ay);
        w32(obj, 0x9C, ax);
        w32(obj, 0xA0, ay);
    }
    let ex = mx.wrapping_add(i16_at(obj, 0x29E4));
    let ax = i16_at(obj, 0x28F4);
    if ax > ex {
        w32(obj, 0x9C, ax as u32);
        w32(obj, 0x94, ex as u32);
    } else {
        w32(obj, 0x94, ax as u32);
        w32(obj, 0x9C, ex as u32);
    }
    let ey = i16_at(obj, 0x29E6).wrapping_add(my);
    let ay = i16_at(obj, 0x28F6);
    if ay > ey {
        w32(obj, 0xA0, ay as u32);
        w32(obj, 0x98, ey as u32);
    } else {
        w32(obj, 0x98, ay as u32);
        w32(obj, 0xA0, ey as u32);
    }
    if i32_at(obj, 0x98) < vy {
        w32(obj, 0x98, vy as u32);
    }
    if i32_at(obj, 0xA0) > vy.wrapping_add(0x707) {
        w32(obj, 0xA0, vy.wrapping_add(0x707) as u32);
    }
    if i32_at(obj, 0x94) < 0 {
        w32(obj, 0x94, 0);
    }
    if i32_at(obj, 0x9C) > 0x95F {
        w32(obj, 0x9C, 0x95F);
    }
    box_edges(obj, host);
}

/// `0x00446A00` (`route_point_hit_test_polyline`): ¿el punto `query` (vista; se le suma el desplazamiento de la carta
/// `0x006CBDD8`) está a ±2 píxeles de algún tramo de la ruta desde el punto `start`? Cada tramo se resuelve por el eje mayor
/// (`|dx| > |dy|`: `y` en función de `x`), con el parámetro `t ∈ [0, 1]` en `f32`; en el primer tramo el punto tiene que estar
/// más lejos del inicio del tramo que el barco (`ship`), es decir, por delante de él. `count` es el número de puntos que guarda
/// la ruta; un índice fuera de `points` se trata como fin de la ruta (el original leería fuera de la tabla).
pub fn route_point_hit(points: &[(i16, i16)], count: i32, start: i32, scroll: (i32, i32), query: (i32, i32), ship: (i32, i32)) -> bool {
    let qx = scroll.0.wrapping_add(query.0);
    let qy = scroll.1.wrapping_add(query.1);
    let at = |i: i32| -> Option<(i32, i32)> { usize::try_from(i).ok().and_then(|i| points.get(i)).map(|&(x, y)| (i32::from(x), i32::from(y))) };
    let mut i = start;
    if i == count {
        return false;
    }
    loop {
        let Some((x0, y0)) = at(i) else { return false };
        i = i.wrapping_add(1);
        if i == count {
            return false;
        }
        let Some((x1, y1)) = at(i) else { return false };
        let dx = x1.wrapping_sub(x0) as f32;
        let dy = y1.wrapping_sub(y0) as f32;
        let ratio = |a: i32, d: f32| (f64::from(a as f32) / f64::from(d)) as f32;
        let first = start.wrapping_add(1) == i;
        let behind = || {
            let q = (x0.wrapping_sub(qx), y0.wrapping_sub(qy));
            let s = (x0.wrapping_sub(ship.0), y0.wrapping_sub(ship.1));
            let dq = q.0.wrapping_mul(q.0).wrapping_add(q.1.wrapping_mul(q.1));
            let ds = s.0.wrapping_mul(s.0).wrapping_add(s.1.wrapping_mul(s.1));
            ds > dq
        };
        let along = |t: f32| t >= 0.0 && t <= 1.0;
        let hit = if f64::from(dy).abs() < f64::from(dx).abs() {
            let t = ratio(qx.wrapping_sub(x0), dx);
            let u = ratio(qy.wrapping_sub(y0), dy);
            let k = if dx == 0.0 { u } else { t };
            let v = crate::navmap::ftol(f64::from(k) * f64::from(dy) + f64::from(y0 as f32) + 0.5);
            qy >= v.wrapping_sub(2) && qy <= v.wrapping_add(2) && along(t) && !(first && behind())
        } else {
            let t = ratio(qy.wrapping_sub(y0), dy);
            let u = ratio(qx.wrapping_sub(x0), dx);
            let k = if dy == 0.0 { u } else { t };
            let v = crate::navmap::ftol(f64::from(k) * f64::from(dx) + f64::from(x0 as f32) + 0.5);
            qx >= v.wrapping_sub(2) && qx <= v.wrapping_add(2) && along(t) && !(first && behind())
        };
        if hit {
            return true;
        }
    }
}

/// Grupos de iconos de estado de `0x00453EE0` (bit de `0x00535300`, fotograma de `+0xC4`) en orden de prioridad. El primero
/// usa `0x20000` cuando los bits 18..22 no valen todos 1 (en el dibujo, `0x0044BFD0`, es el nivel de la textura `+0xC8`).
const STATUS_A: [(u32, i32); 3] = [(0x1000, 6), (0x2_0000, 7), (0x1_0000, 5)];
const STATUS_B: [(u32, i32); 7] = [(0x200, 12), (0x400_0000, 14), (0x400, 13), (0x10, 8), (0x8000, 11), (0x8, 10), (0x800, 9)];

/// `0x00453EE0` (`scroll_map_draw_town_status_icons`): pregunta por la ayuda de cada icono de estado visible
/// ([`NavHost::hover_status_icon`]) y repinta su rectángulo; `+0x2B25` acumula si alguno estaba bajo el cursor. El primer icono
/// de una ciudad sin ninguno se repinta igualmente si la ciudad anterior tenía segundo icono (registro `bl` del original), con
/// el tamaño de fotograma que quedó. `frame` es el rectángulo de fotograma de pila que se arrastra (`[esp+0x30]`).
pub fn update_town_status_icons(obj: &mut [u8], towns: &Towns, icons: &crate::navmap::Tex, frame: &mut [i32; 4], host: &mut dyn NavHost) {
    host.color(0xFFFF_FFFF);
    let view = view_rect(obj);
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let (vx, vy, vw, vh) = (i32_at(obj, 0x14), i32_at(obj, 0x18), i32_at(obj, 0x2C), i32_at(obj, 0x30));
    let mut hovered = false;
    let mut prev_second = false;
    let mut take = |frame: &mut [i32; 4], i: i32| {
        if let Some((_, f)) = icons.frame(i) {
            *frame = f;
        }
    };
    let repaint = |x: i32, y: i32, frame: &[i32; 4], host: &mut dyn NavHost| {
        let (right, bottom) = (x.wrapping_add(frame[2]), y.wrapping_add(frame[3]));
        if y < vh.wrapping_add(vy) && x < vw.wrapping_add(vx) && bottom > vy && right > 0 {
            repaint_dirty([x, y, right, bottom], &view, host);
        }
    };
    for i in 0..towns.order.len() {
        let t = towns.info(i);
        let x = t.event.0.wrapping_sub(sx);
        let y = t.event.1.wrapping_sub(sy).wrapping_add(vy);
        let f = host.town_flags(i as u32);
        let a = if f & 0x1000 != 0 {
            Some(STATUS_A[0])
        } else if f & 0x7C_0000 != 0x7C_0000 {
            Some(STATUS_A[1])
        } else if f & 0x1_0000 != 0 {
            Some(STATUS_A[2])
        } else {
            None
        };
        if let Some((bit, k)) = a {
            take(frame, k);
            hovered |= host.hover_status_icon([x, y, frame[0], frame[1], frame[2], frame[3]], bit, i as u32);
        }
        if a.is_some() || prev_second {
            repaint(x, y, frame, host);
        }
        prev_second = false;
        let x = x.wrapping_add(0x14);
        if let Some(&(bit, k)) = STATUS_B.iter().find(|(b, _)| f & b != 0) {
            take(frame, k);
            hovered |= host.hover_status_icon([x, y, frame[0], frame[1], frame[2], frame[3]], bit, i as u32);
            repaint(x, y, frame, host);
            prev_second = true;
        }
    }
    if hovered {
        w8(obj, 0x2B25, r8(obj, 0x2B25) | 1);
    }
}

/// `0x00454430` (`scroll_map_update_town_ship_presence_icons`): iconos de barcos ajenos en cada ciudad bajo el cursor.
///
/// Primero marca las ciudades con barcos propios (`1`; estados distintos de 6 y 14) y las de convoyes propios. Luego, para
/// cada barco de otro comerciante (estado `< 4`) en una ciudad con barco propio u oficina (`0x005308A0`), pone `2` (humano) o
/// `3` (IA, `merchant+8 ≠ 0`) en la tabla de barcos solos o, si va en convoy ajeno sin el bit 1 de `convoy+0x14`, en la de
/// convoyes. Igual que en `0x0044C3B0`, el convoy se acepta con índice `<=` que el número de convoyes. Por cada icono pregunta
/// por la ayuda ([`NavHost::hover_status_icon`] con `{x, y, rectángulo repintado}`) y repinta. `firsts[m]` es `merchant+0xE`.
#[allow(clippy::too_many_arguments)]
pub fn update_town_presence_icons(
    obj: &mut [u8],
    world: &World,
    towns: &Towns,
    icons: &crate::navmap::Tex,
    firsts: &[u16],
    tooltip_set: bool,
    frame: &mut [i32; 4],
    host: &mut dyn NavHost,
) {
    let n = towns.order.len();
    let (mut solo, mut convoy) = (vec![0u8; n], vec![0u8; n]);
    let first = |m: u32| u32::from(firsts.get(m as usize).copied().unwrap_or(0xFFFF));
    let player = world.player;
    host.get_merchant(player);
    let mut s = first(player);
    while s < u32::from(world.ship_count) {
        let ship = world.ship(s);
        let t = usize::from(r8(ship, 0x39));
        let st = r16(ship, 0x134);
        if t < n && st != 0xE && st != 6 {
            solo[t] = 1;
            let c = r16(ship, 8);
            if c < world.convoy_count && u32::from(r8(world.ship(u32::from(r16(world.convoy(u32::from(c)), 0x10))), 0)) == player {
                convoy[t] = 1;
            }
        }
        s = u32::from(r16(ship, 4));
    }
    for m in 0..u32::from(world.merchant_count) {
        host.get_merchant(m);
        let mut s = first(m);
        while s < u32::from(world.ship_count) {
            let ship = world.ship(s);
            let t = usize::from(r8(ship, 0x39));
            let office = host.office(player & 0xFFFF, t as u32);
            if t < n && (solo[t] != 0 || office) && m != player && r16(ship, 0x134) < 4 {
                let c = r16(ship, 8);
                let kind = |who: u32, host: &mut dyn NavHost| 2 + u8::from(host.merchant_word8(who as u8) != 0);
                if c > world.convoy_count {
                    solo[t] = kind(m, host);
                } else {
                    let cv = world.convoy(u32::from(c));
                    let owner = u32::from(r8(world.ship(u32::from(r16(cv, 0x10))), 0));
                    if owner == player {
                        convoy[t] = 1;
                    } else if (r8(cv, 0x14) >> 1) & 1 == 0 {
                        convoy[t] = kind(owner, host);
                    } else {
                        solo[t] = kind(m, host);
                    }
                }
            }
            s = u32::from(r16(ship, 4));
        }
    }
    let view = view_rect(obj);
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let (vx, vy, vw, vh) = (i32_at(obj, 0x14), i32_at(obj, 0x18), i32_at(obj, 0x2C), i32_at(obj, 0x30));
    let mut hovered = false;
    let mut icon = |x: i32, test_x: i32, y: i32, k: i32, bit: u32, i: usize, frame: &mut [i32; 4], host: &mut dyn NavHost| {
        if let Some((_, f)) = icons.frame(k) {
            *frame = f;
        }
        let mut r = [x, y, frame[2].wrapping_add(x), frame[3].wrapping_add(y)];
        if test_x < vx.wrapping_add(vw) && y < vh.wrapping_add(vy) && y > vy && test_x > 0 {
            match intersect(&r, &view) {
                Some(q) => {
                    r = q;
                    host.repaint(r);
                    host.dirty(r);
                }
                None => r = [0; 4],
            }
        }
        host.hover_status_icon([x, y, r[0], r[1], r[2], r[3]], bit, i as u32)
    };
    for i in 0..n {
        let t = towns.info(i);
        let x = t.event.0.wrapping_sub(sx);
        let y = t.event.1.wrapping_sub(sy).wrapping_add(vy).wrapping_add(0x14);
        match solo[i] {
            2 => hovered |= icon(x, x, y, 4, 0x4_0000, i, frame, host),
            3 => hovered |= icon(x, x, y, 3, 0x8_0000, i, frame, host),
            _ => {}
        }
        let xb = x.wrapping_add(0x14);
        match convoy[i] {
            2 => hovered |= icon(xb, x, y, 1, 0x20_0000, i, frame, host),
            3 => hovered |= icon(xb, x, y, 0, 0x40_0000, i, frame, host),
            _ => {}
        }
    }
    let v = r8(obj, 0x2B25) | u8::from(hovered);
    w8(obj, 0x2B25, v);
    if v == 0 {
        if tooltip_set {
            host.clear_tooltip();
        }
    } else {
        host.show_tooltip();
    }
}

/// Zonas fluviales `0x0066D76C` (`x0, y0, x1, y1`, límites excluidos).
pub const RIVER_ZONES: [[i16; 4]; 3] = [[442, 1521, 658, 1691], [2265, 147, 2332, 453], [1497, 1263, 1578, 1444]];

/// `0x00446D50` (`find_town_at_point`): posición de la ciudad cuyo símbolo (22 × 22, bordes excluidos) contiene el punto o
/// cuyo punto de llegada coincide; `0x28` si ninguna. Con `out`, escribe el punto y, si hay ciudad, su llegada.
pub fn find_town_at_point(towns: &Towns, (px, py): (i16, i16), out: Option<&mut [u8]>) -> u32 {
    let (qx, qy) = (i32::from(px), i32::from(py));
    let mut found = None;
    for k in 0..towns.order.len() {
        let t = towns.info(k);
        let (x, y) = t.symbol;
        let over = x.wrapping_add(0x16) > qx && y.wrapping_add(0x16) > qy && x < qx && y < qy;
        if over || (t.approach.0 == qx && t.approach.1 == qy) {
            found = Some((k as u32, t.approach));
            break;
        }
    }
    if let Some(o) = out {
        let (x, y) = found.map(|(_, a)| (a.0 as u16, a.1 as u16)).unwrap_or((px as u16, py as u16));
        w16(o, 0, x);
        w16(o, 2, y);
    }
    found.map(|(k, _)| k).unwrap_or(0x28)
}

fn map_point(obj: &[u8], (mx, my): (i32, i32)) -> (i16, i16) {
    (r16(obj, 0x29E4).wrapping_add(mx as u16) as i16, r16(obj, 0x29E6).wrapping_sub(r16(obj, 0x18)).wrapping_add(my as u16) as i16)
}

/// `0x0045BCF0` (`scroll_map_is_point_navigable_for_hull_type`): con casco de tipo mayor que 1 (16 bits bajos), un punto de
/// las [`RIVER_ZONES`] solo vale sobre una ciudad cuyo byte `+0x2F` sea 0 (si el cursor está sobre una ciudad, la zona se
/// prueba con su punto de llegada). `0x0045BDC0`, que el original llama a
/// continuación, siempre devuelve verdadero (prueba un bit 26 sobre un valor de 16 bits).
pub fn point_navigable_for_hull(obj: &[u8], towns: &Towns, mouse: (i32, i32), kind: u32) -> bool {
    let mut out = [0u8; 4];
    let town = find_town_at_point(towns, map_point(obj, mouse), Some(&mut out));
    // Las zonas se prueban con el punto que deja 0x00446D50: la llegada de la ciudad si la hay.
    let (px, py) = (r16(&out, 0) as i16, r16(&out, 2) as i16);
    if kind as u16 <= 1 {
        return true;
    }
    for z in RIVER_ZONES {
        if py < z[3] && px > z[0] && px < z[2] && py > z[1] && (town as usize >= towns.order.len() || towns.info(town as usize).river != 0) {
            return false;
        }
    }
    true
}

/// `0x00451230` (`scroll_map_update_travel_time_hint`): con un único barco propio elegido, en puerto (estado 0 o 15) y el
/// cursor sobre una ciudad de la vista, pone en la ayuda los días de viaje hasta ella (`+0x29E0`): si ya navega hacia esa
/// ciudad, los que quedan (`ship+0x48 − DAT_006DE4B4` con el byte bajo del reloj a cero, en 256 avos); si no, los de una ruta nueva desde la posición del barco
/// (se rehace solo si el barco se movió más de 50 píxeles desde `cache`, `0x006CBDE0`, o no había valor). `now` es
/// `DAT_006DE4B4`. El tipo de casco del convoy es el mayor de sus barcos.
#[allow(clippy::too_many_arguments)]
pub fn update_travel_time_hint(
    obj: &mut [u8],
    world: &World,
    towns: &Towns,
    selection: &[u16],
    mouse: (i32, i32),
    now: i32,
    cache: &mut (i16, i16),
    host: &mut dyn NavHost,
) {
    if selection.len() != 1 {
        w8(obj, 0x2B25, 0);
        return;
    }
    let id = u32::from(selection[0]);
    let ship = world.ship(id);
    if u32::from(r8(ship, 0)) != world.player {
        w8(obj, 0x2B25, 0);
        return;
    }
    let hide = |obj: &mut [u8], host: &mut dyn NavHost| {
        if i32_at(obj, 0x29E0) != -1 {
            w32(obj, 0x29E0, u32::MAX);
            w8(obj, 0x2B25, 0);
            host.show_tooltip();
        }
    };
    let v = view_rect(obj);
    let (mx, my) = mouse;
    if !(mx >= v[0] && mx < v[2] && my >= v[1] && my < v[3]) {
        hide(obj, host);
        return;
    }
    let p = map_point(obj, mouse);
    let town = find_town_at_point(towns, p, Some(&mut obj[0x28FC..0x2900]));
    let st = r16(ship, 0x134);
    if !((st == 0xF || st == 0) && town < 0x28) {
        hide(obj, host);
        return;
    }
    let mut kind = u32::from(r8(ship, 0xE));
    let c = r16(ship, 8);
    if c < world.convoy_count {
        let mut m = u32::from(r16(world.convoy(u32::from(c)), 0xA));
        while m < u32::from(world.ship_count) {
            let s = world.ship(m);
            kind = kind.max(u32::from(r8(s, 0xE)));
            m = u32::from(r16(s, 6));
        }
    }
    if !point_navigable_for_hull(obj, towns, mouse, kind) {
        return;
    }
    let show = |obj: &mut [u8], days: i32, host: &mut dyn NavHost| {
        host.tooltip_days(days);
        host.show_tooltip();
        w8(obj, 0x2B25, 1);
    };
    if u32::from(r8(ship, 0x38)) == town {
        if r8(ship, 0x38) == r8(ship, 0x39) {
            w32(obj, 0x29E0, u32::MAX);
            w8(obj, 0x2B25, 0);
            return;
        }
        // El original borra el byte bajo del reloj (`xor al, al` tras leerlo).
        let (eta, now) = (i32_at(ship, 0x48), now & !0xFF);
        let days = if eta > now { eta.wrapping_sub(now) >> 8 } else { 0 };
        w32(obj, 0x29E0, days as u32);
        show(obj, days, host);
        return;
    }
    let (x, y) = (r16(ship, 0x1E) as i16, r16(ship, 0x22) as i16);
    let dx = i32::from(cache.0).wrapping_sub(i32::from(x)).wrapping_abs();
    let dy = i32::from(cache.1).wrapping_sub(i32::from(y)).wrapping_abs();
    if dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)) > 0x9C4 || i32_at(obj, 0x29E0) == -1 {
        *cache = (x, y);
        let target = (r16(obj, 0x28FC) as i16, r16(obj, 0x28FE) as i16);
        let (found, route) = host.find_route(&[(x, y), target]);
        let days = host.route_ticks(id, route) >> 8;
        w32(obj, 0x29E0, days as u32);
        show(obj, days, host);
        if found {
            host.free_route(route);
        }
    } else {
        show(obj, i32_at(obj, 0x29E0), host);
    }
}


/// Globales de la vista previa de ruta: `DAT_006CBDD0`/`DAT_006CBDD2` (punto intermedio que se enviará) y
/// `DAT_006CBDE4`/`DAT_006CBDE6` (cursor de la última vista previa).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoutePreview {
    pub point: (i16, i16),
    pub mouse: (i16, i16),
}

/// `0x00451EB0` (`scroll_map_commit_route_preview`). Con `selection[0]` como barco guía y el tipo de casco mayor del convoy:
///
/// * Si la vista previa está activa (`+0x2B23`), el cursor debe pasar [`point_navigable_for_hull`]. El ancla
///   `+0x28F8/+0x28FA` se fija en el punto del mapa bajo el cursor si no la había (−1); si ese punto no es agua, busca en
///   cuadrados crecientes de radio 1 a 9 (columna por columna) el primer punto navegable y lo toma como ancla, o borra el
///   ancla y termina. Si el punto está a más de 50 píxeles del ancla o no es agua, se acerca al ancla en 51 pasos
///   (`ancla ± ⌊d·k/‖d‖ + 0,5⌋`, k = 50…0) hasta dar con agua; cada paso vuelve a probar el casco y sale sin más si falla.
///   Con un punto navegable libera la ruta previa `+0xA8` (`preview_inner` es su primer campo), pide una ruta
///   [posición del barco, punto, último punto de su ruta actual] (`route_last`, de `ship+0x24`; sin ruta no la pide),
///   guarda la nueva en `+0xA8`, repinta la vista y recuerda punto y cursor en `state`. Sin punto navegable borra
///   `state.point` y termina.
/// * Si no está activa y el barco tiene recorrido pendiente (`ship+0x30 > 0`) y `state.point` no tiene componentes nulas,
///   envía cuatro operaciones 0x0B (subcódigos 0, 1, 2 y −1: posición, punto, destino y cierre). En cualquier caso libera
///   `+0xA8` y borra el ancla; `state` queda igual.
///
/// `route_last` es obligatorio cuando se envían las operaciones (el original lo lee sin comprobar el puntero).
#[allow(clippy::too_many_arguments)]
pub fn commit_route_preview(
    obj: &mut [u8],
    world: &World,
    towns: &Towns,
    selection: &[u16],
    mouse: (i32, i32),
    route_last: Option<(i16, i16)>,
    preview_inner: u32,
    state: &mut RoutePreview,
    host: &mut dyn NavHost,
) {
    let id = selection[0];
    let ship = world.ship(u32::from(id));
    let (mut px, mut py) = map_point(obj, mouse);
    let probe = |x: i16, y: i16, host: &mut dyn NavHost| host.navigable(x.wrapping_sub(0x44), y.wrapping_sub(0x44));
    let mut nav = probe(px, py, host);
    let mut kind = u32::from(r8(ship, 0xE)) as u16;
    let c = r16(ship, 8);
    if c < world.convoy_count {
        let mut m = r16(world.convoy(u32::from(c)), 0xA);
        while m < world.ship_count {
            let s = world.ship(u32::from(m));
            kind = kind.max(u16::from(r8(s, 0xE)));
            m = r16(s, 6);
        }
    }
    let kind = u32::from(kind);
    let free_preview = |obj: &mut [u8], host: &mut dyn NavHost| {
        let p = r32(obj, 0xA8);
        if p != 0 {
            host.free(preview_inner);
            host.free(p);
            w32(obj, 0xA8, 0);
        }
    };
    if obj[0x2B23] == 0 {
        if i32_at(ship, 0x30) > 0 && state.point.0 != 0 && state.point.1 != 0 {
            let last = route_last.unwrap_or_default();
            let (sx, sy) = (i32::from(r16(ship, 0x1E) as i16), i32::from(r16(ship, 0x22) as i16));
            let id = i32::from(id);
            host.enqueue_operation([0xB, id, 0, sx, sy]);
            host.enqueue_operation([0xB, id, 1, i32::from(state.point.0), i32::from(state.point.1)]);
            host.enqueue_operation([0xB, id, 2, i32::from(last.0), i32::from(last.1)]);
            host.enqueue_operation([0xB, id, -1, i32::from(last.0), i32::from(last.1)]);
        }
        free_preview(obj, host);
        w16(obj, 0x28F8, 0xFFFF);
        w16(obj, 0x28FA, 0xFFFF);
        return;
    }
    if !point_navigable_for_hull(obj, towns, mouse, kind) {
        return;
    }
    if r16(obj, 0x28F8) == 0xFFFF {
        w16(obj, 0x28F8, px as u16);
        w16(obj, 0x28FA, py as u16);
        nav = probe(px, py, host);
        if !nav {
            // Cuadrados de radio r = 1..9: columnas a = −r..r, filas b = −r..r.
            let mut found = false;
            'rings: for r in 1i16..10 {
                for a in -r..=r {
                    for b in -r..=r {
                        if probe(px.wrapping_add(a), py.wrapping_add(b), host) {
                            w16(obj, 0x28F8, px.wrapping_add(a) as u16);
                            w16(obj, 0x28FA, py.wrapping_add(b) as u16);
                            found = true;
                            break 'rings;
                        }
                    }
                }
            }
            if !found {
                w16(obj, 0x28F8, 0xFFFF);
                w16(obj, 0x28FA, 0xFFFF);
                return;
            }
        }
    }
    let (ax, ay) = (r16(obj, 0x28F8) as i16, r16(obj, 0x28FA) as i16);
    let dx = (i32::from(ax) - i32::from(px)).wrapping_abs();
    let dy = (i32::from(ay) - i32::from(py)).wrapping_abs();
    let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    if d2 > 0x9C4 || !nav {
        let dist = f64::from(d2).sqrt();
        let (mut kx, mut ky) = (dx.wrapping_mul(50), dy.wrapping_mul(50));
        for _ in 0..51 {
            let step = |k: i32| ftol(f64::from(k) / dist + 0.5) as u16;
            px = if ax < px { step(kx).wrapping_add(ax as u16) } else { (ax as u16).wrapping_sub(step(kx)) } as i16;
            py = if ay < py { step(ky).wrapping_add(ay as u16) } else { (ay as u16).wrapping_sub(step(ky)) } as i16;
            nav = probe(px, py, host);
            let screen = (
                i32::from(px) - i32::from(r16(obj, 0x29E4) as i16),
                (i32::from(py) - i32::from(r16(obj, 0x29E6) as i16)).wrapping_add(i32_at(obj, 0x18)),
            );
            if !point_navigable_for_hull(obj, towns, screen, kind) {
                return;
            }
            if nav {
                break;
            }
            kx = kx.wrapping_sub(dx);
            ky = ky.wrapping_sub(dy);
        }
    }
    state.point = (0, 0);
    if !nav {
        return;
    }
    free_preview(obj, host);
    let Some(last) = route_last else { return };
    let (sx, sy) = (r16(ship, 0x1E) as i16, r16(ship, 0x22) as i16);
    state.point = (px, py);
    let (_, route) = host.find_route(&[(sx, sy), (px, py), last]);
    w32(obj, 0xA8, route);
    host.repaint(view_rect(obj));
    state.mouse = (mouse.0 as i16, mouse.1 as i16);
}


/// Grupo de problemas de ruta de la tabla `+0x2880` (0x14 bytes por grupo, recuento `+0x292C`): barcos (`+0`, cuenta en
/// `+4`), motivo por barco (`+8`: 4 = convoy cuyo líder no se eligió, 8 = punto de ruta no navegable), `+0xC` código del grupo,
/// `+0xE` clave (líder del convoy o barco) y `+0x10/+0x12` (`0x0045A6B0` los pone a −1; `0x0045B9C0` no los inicializa,
/// aquí 0 como el asignador del oráculo).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProblemGroup {
    pub ships: Vec<u16>,
    pub reasons: Vec<u16>,
    pub code: u16,
    pub key: u16,
    pub link: [i16; 2],
}

/// `0x0045B9C0` (`scroll_map_add_blocked_route_problem`): añade `ship` con motivo 8 al grupo de clave `ship` (sin mirar el
/// código) o crea uno con código 8; después abre el diálogo de problemas si no está abierto. El original reserva solo dos
/// bytes por lista al crear el grupo y escribe más allá si se reutiliza.
pub fn add_blocked_route_problem(groups: &mut Vec<ProblemGroup>, ship: u16, host: &mut dyn NavHost) {
    if let Some(g) = groups.iter_mut().find(|g| g.key == ship) {
        g.ships.push(ship);
        g.reasons.push(8);
    } else {
        groups.push(ProblemGroup { ships: vec![ship], reasons: vec![8], code: 8, key: ship, link: [0, 0] });
    }
    if !host.problem_dialog_open() {
        host.open_problem_dialog();
    }
}

/// `0x0045A6B0` (`scroll_map_commit_waypoint_route_for_selected_ships`). `flag` es `DAT_006DCBD1`: si no es cero se
/// rehace (`flag & 0xFE | (flag >> 1) & 0x7E`), se deja `+0x2B1E` a 1 y no se hace nada más. Con `+0x2B1E` activo repinta la
/// vista y, si hay puntos (`+0x291C`, en `+0x28D4`):
///
/// 1. Líderes: barcos elegidos propios que encabezan su convoy.
/// 2. Cada barco propio cuyo convoy no tiene un líder elegido va al grupo (clave = líder, código 0) con motivo 4, creado con
///    listas del tamaño del convoy si no existe. Luego `refresh_problems(0xFF, 0)` y todos los grupos a `link = −1`.
/// 3. Por cada barco elegido: se descarta si un barco de su convoy ya está en un grupo de su líder (los solos, si están en
///    un grupo de clave mayor que el número de convoyes); se sustituye por el líder si este está elegido; con el casco
///    mayor del convoy, cada punto debe pasar [`point_navigable_for_hull`] o se añade el problema
///    ([`add_blocked_route_problem`]) y se descarta. Se omite si otro líder elegido distinto del barco original encabeza su
///    convoy, o si va en convoy sin el bit 1 de `convoy+0x14`; si no, encola la operación `[0x0B, barco, −1, 0, 0]`.
///
/// Al final `+0x291C` y `+0x2B1E` quedan a 0. Los puntos no se encolan aquí.
pub fn commit_waypoint_route(
    obj: &mut [u8],
    world: &World,
    towns: &Towns,
    selection: &[u16],
    flag: &mut u8,
    groups: &mut Vec<ProblemGroup>,
    host: &mut dyn NavHost,
) {
    let old = *flag;
    *flag = (old & 0xFE) | ((old >> 1) & 0x7E);
    if old != 0 {
        obj[0x2B1E] = 1;
        return;
    }
    if obj[0x2B1E] == 0 {
        return;
    }
    host.repaint(view_rect(obj));
    let done = |obj: &mut [u8]| {
        w32(obj, 0x291C, 0);
        obj[0x2B1E] = 0;
    };
    if i32_at(obj, 0x291C) == 0 {
        done(obj);
        return;
    }
    let (sc, cc) = (world.ship_count, world.convoy_count);
    let own = |id: u16| u32::from(r8(world.ship(u32::from(id)), 0)) == world.player;
    let convoy_of = |id: u16| {
        let c = r16(world.ship(u32::from(id)), 8);
        (c < cc).then_some(c)
    };
    let leader_of = |c: u16| r16(world.convoy(u32::from(c)), 0x10);
    let mut leaders: Vec<u16> = Vec::new();
    for &id in selection {
        if own(id) && convoy_of(id).is_some_and(|c| leader_of(c) == id) {
            leaders.push(id);
        }
    }
    for &id in selection {
        if !own(id) {
            continue;
        }
        if leaders.iter().any(|&l| convoy_of(id).is_some_and(|c| leader_of(c) == l)) {
            continue;
        }
        let Some(c) = convoy_of(id) else { continue };
        let leader = leader_of(c);
        if leader == id {
            continue;
        }
        if let Some(g) = groups.iter_mut().find(|g| g.key == leader && g.code == 0) {
            g.ships.push(id);
            g.reasons.push(4);
        } else {
            groups.push(ProblemGroup { ships: vec![id], reasons: vec![4], code: 0, key: leader, link: [0, 0] });
        }
    }
    host.refresh_problems(0xFF, 0);
    for g in groups.iter_mut() {
        g.link = [-1, -1];
    }
    let members = |c: u16| {
        let mut v = Vec::new();
        let mut m = r16(world.convoy(u32::from(c)), 0xA);
        while m < sc {
            v.push(m);
            m = r16(world.ship(u32::from(m)), 6);
        }
        v
    };
    for &orig in selection {
        let mut id = orig;
        if let Some(c) = convoy_of(orig) {
            let leader = leader_of(c);
            let ms = members(c);
            if groups.iter().any(|g| g.key == leader && g.ships.iter().any(|s| ms.contains(s))) {
                continue;
            }
            if selection.contains(&leader) {
                id = leader;
            }
        } else if groups.iter().any(|g| g.key > cc && g.ships.contains(&orig)) {
            continue;
        }
        let ship = world.ship(u32::from(id));
        let mut kind = u16::from(r8(ship, 0xE));
        if let Some(c) = convoy_of(id) {
            for m in members(c) {
                kind = kind.max(u16::from(r8(world.ship(u32::from(m)), 0xE)));
            }
        }
        let n = i32_at(obj, 0x291C);
        let mut ok = true;
        for w in 0..n.max(0) as usize {
            let (px, py) = (r16(obj, 0x28D4 + 4 * w) as i16, r16(obj, 0x28D6 + 4 * w) as i16);
            let screen = (
                i32::from(px) - i32::from(r16(obj, 0x29E4) as i16),
                (i32::from(py) - i32::from(r16(obj, 0x29E6) as i16)).wrapping_add(i32_at(obj, 0x18)),
            );
            if !point_navigable_for_hull(obj, towns, screen, u32::from(kind)) {
                add_blocked_route_problem(groups, id, host);
                ok = false;
                break;
            }
        }
        if !ok || n < 0 {
            continue;
        }
        // 0x0045BDC0 con el último punto: siempre verdadero.
        if let Some(c) = convoy_of(id) {
            if leaders.iter().any(|&l| l != orig && leader_of(c) == l) {
                continue;
            }
            if r8(world.convoy(u32::from(c)), 0x14) & 2 == 0 {
                continue;
            }
        }
        host.enqueue_operation([0xB, i32::from(id), -1, 0, 0]);
    }
    done(obj);
}
