//! `Convoy::advance` (0x00502110): avance de un convoy por tick. Cabecera, integración de posición, recorrido de
//! los barcos del convoy, cierre sin ruta y tramo con ruta (0x0050248A..0x00502B18). El tramo de la tabla de
//! objetos (0x00502285..0x00502331, solo con la tabla de 0x006DD748 no vacía) se omite.

use super::remove::{remove_ship, RemoveEvent, CONVOY_SIZE, MERCHANT_SIZE, SHIP_SIZE};
use super::tick::{ConvoyHost, TickWorld};
use super::{r16, r32, w16, w32, DivideError};

/// `flag` es el argumento que recibe la función (`time & 0x1F == 0` desde el bucle, o 1).
pub fn convoy_advance(w: &mut TickWorld, cv: u16, flag: u32, host: &mut dyn ConvoyHost) -> Result<bool, DivideError> {
    let b = cv as usize * CONVOY_SIZE;
    let count = r16(w.remove.manager, 0xF4);
    let main = r16(w.remove.convoys, b + 0x10);
    let e = main as usize * SHIP_SIZE;
    let time = w.time;
    let every4 = (time & 3 == 0) as i32;
    let mark = time & 0x7F;
    let local14 = (mark == 0x28 || mark == 0x50 || mark == 0x78) as i32;
    let (old_x, old_y) = (r16(w.remove.ships, e + 0x1E), r16(w.remove.ships, e + 0x22));
    if (r32(w.remove.ships, e + 0x18) as i32) <= 0 {
        pick_main(w, cv, host);
    }
    let x = r32(w.remove.ships, e + 0x1C).wrapping_add(r32(w.remove.ships, e + 0x28));
    let y = r32(w.remove.ships, e + 0x20).wrapping_add(r32(w.remove.ships, e + 0x2C));
    w32(w.remove.ships, e + 0x1C, x);
    w32(w.remove.ships, e + 0x20, y);
    // El original compara el valor antiguo sin signo con el nuevo con extensión de signo (`movsx`).
    let moved = old_x as u32 != r16(w.remove.ships, e + 0x1E) as i16 as i32 as u32
        || old_y as u32 != r16(w.remove.ships, e + 0x22) as i16 as i32 as u32;
    let mark_cell: u16 = if moved { 0x2000 } else { 0 };
    // Recorre la lista de barcos del convoy; `slot` es el enlace que apunta al barco actual.
    let mut slot: Option<usize> = None;
    let mut guard = 0;
    loop {
        let sh = match slot {
            None => r16(w.remove.convoys, b + 0xA),
            Some(s) => r16(w.remove.ships, s * SHIP_SIZE + 6),
        };
        if sh >= count || guard > 0x10000 {
            break;
        }
        guard += 1;
        let o = sh as usize * SHIP_SIZE;
        let f = r16(w.remove.ships, o + 0x3C) | mark_cell;
        w16(w.remove.ships, o + 0x3C, f);
        w32(w.remove.ships, o + 0x1C, x);
        w32(w.remove.ships, o + 0x20, y);
        let owner = w.remove.ships[o] as u16;
        let step = if owner < w.remove.merchant_count {
            if r16(w.remove.merchants, owner as usize * MERCHANT_SIZE + 8) == 0 {
                every4
            } else {
                local14
            }
        } else {
            local14
        };
        let left = r32(w.remove.ships, o + 0x18).wrapping_sub(step);
        w32(w.remove.ships, o + 0x18, left);
        if left >= 1 {
            if flag != 0 {
                let v = r16(w.remove.ships, o + 0x3E).wrapping_sub(1);
                w16(w.remove.ships, o + 0x3E, v);
                if v & 0xFF == 0xFF {
                    let hi = w.remove.ships[o + 0x3F] as i8;
                    w.remove.ships[o + 0x3D] |= 2;
                    if hi < -1 {
                        w.remove.ships[o + 0x3F] = 0xFF;
                    }
                }
            }
            slot = Some(sh as usize);
            continue;
        }
        // El barco terminó: sale de la lista del convoy.
        let next = r16(w.remove.ships, o + 6);
        match slot {
            None => w16(w.remove.convoys, b + 0xA, next),
            Some(s) => w16(w.remove.ships, s * SHIP_SIZE + 6, next),
        }
        // 0x00502285..0x00502331 (lista de bajas de la caza): el anfitrión con planificador lo aplica.
        host.convoy_ship_finished_with_world(w, cv, sh);
        if r16(w.remove.convoys, b + 0x10) == sh {
            host.remove_order_with_world(w, sh);
            w16(w.remove.convoys, b + 0x10, 0xFFFF);
        }
        if r16(w.remove.ships, o + 0x134) != 0x12 {
            w16(w.remove.ships, o + 0x134, 0xF);
        }
        w32(w.remove.ships, o + 0x18, 0);
        let f = r16(w.remove.ships, o + 0x3C) | 0x160;
        w16(w.remove.ships, o + 0x3C, f);
        w16(w.remove.ships, o + 8, 0xFFFF);
        put_at_sea(w, sh, host);
    }
    if r16(w.remove.convoys, b + 0x10) >= count {
        return Ok(false);
    }
    if r32(w.remove.ships, e + 0x24) != 0 {
        return Ok(route_tail(w, cv, flag, host, main));
    }
    let owner = w.remove.ships[e] as u16;
    if owner < w.remove.merchant_count && r16(w.remove.merchants, owner as usize * MERCHANT_SIZE + 8) != 0 {
        if w.remove.town_count == 0 {
            return Err(DivideError);
        }
        let town = w.remove.ships[e + 0x37] as u32 % w.remove.town_count;
        let leader = r16(w.remove.convoys, b + 0x10) as u32;
        host.divert_to_nearest_port_with_world(w, leader, town);
    }
    let flags = r32(w.remove.convoys, b + 0x18) | 0x80;
    w32(w.remove.convoys, b + 0x18, flags);
    let mut sh = r16(w.remove.convoys, b + 0xA);
    let mut g = 0;
    while sh < count && g < 0x10000 {
        g += 1;
        let o = sh as usize * SHIP_SIZE;
        let next = r16(w.remove.ships, o + 6);
        let f = r16(w.remove.ships, o + 0x3C) | 0x80;
        w16(w.remove.ships, o + 0x3C, f);
        w32(w.remove.ships, o + 0x28, 0);
        w32(w.remove.ships, o + 0x2C, 0);
        w32(w.remove.ships, o + 0x30, 0);
        sh = next;
    }
    Ok(true)
}

/// Tramo con ruta (0x0050248A..0x00502B18): el barco principal recorre los puntos de `ship+0x24`.
fn route_tail(w: &mut TickWorld, cv: u16, flag: u32, host: &mut dyn ConvoyHost, se: u16) -> bool {
    let b = cv as usize * CONVOY_SIZE;
    let count = r16(w.remove.manager, 0xF4);
    let e = se as usize * SHIP_SIZE;
    let route = r32(w.remove.ships, e + 0x24) as u32;
    let idx = r16(w.remove.ships, e + 0x34);
    let arr = host.read(route, 4);
    let entry = arr.wrapping_add(idx as u32 * 4);
    let (ex, ey) = (host.read(entry, 2) as u16, host.read(entry.wrapping_add(2), 2) as u16);
    if ex != r16(w.remove.ships, e + 0x1E) || ey != r16(w.remove.ships, e + 0x22) {
        return true;
    }
    let next = idx.wrapping_add(1);
    w32(w.remove.ships, e + 0x28, 0);
    w32(w.remove.ships, e + 0x2C, 0);
    w16(w.remove.ships, e + 0x1C, 0x7FFF);
    w16(w.remove.ships, e + 0x20, 0x7FFF);
    w16(w.remove.ships, e + 0x34, next);
    let total = host.read(route.wrapping_add(4), 4) as i32;
    if (next as i32) < total {
        host.refresh_convoy(w, cv);
        let (nx, ny) = (
            host.read(arr.wrapping_add(next as u32 * 4), 2),
            host.read(arr.wrapping_add(next as u32 * 4 + 2), 2),
        );
        let c = r32(w.remove.convoys, b + 0xC) as u32;
        host.set_velocity(w, se, c, nx as u16, ny as u16);
        let v = c << 6;
        let hi = ((0x1CF0_6ADBu64 * v as u64) >> 32) as u32;
        let x = ((v.wrapping_sub(hi) >> 1).wrapping_add(hi) >> 6).wrapping_add(4);
        let (head, sail, owner_town) = (
            r32(w.remove.ships, e + 0x48) as u32,
            r16(w.remove.ships, e + 8),
            w.remove.ships[e + 1],
        );
        host.set_motion(w, head, x, sail, owner_town);
        return true;
    }
    // `bx` conserva la mitad baja de la coordenada y: el original pasa a 0x517260 el puntero con ese resto.
    let arr_arg = (arr & 0xFFFF_0000) | r16(w.remove.ships, e + 0x22) as u32;
    arrive_final(w, cv, flag, host, route, arr_arg, count, se)
}

fn merchant_human(w: &TickWorld, owner: u8) -> bool {
    r16(w.remove.merchants, owner as usize * MERCHANT_SIZE + 8) == 0
}

fn ship_chain(w: &TickWorld, b: usize, count: u16) -> Vec<usize> {
    let mut v = Vec::new();
    let mut a = r16(w.remove.convoys, b + 0xA);
    while a < count && v.len() < 0x10000 {
        v.push(a as usize);
        a = r16(w.remove.ships, a as usize * SHIP_SIZE + 6);
    }
    v
}

/// Cierre común (0x00502642): avisa a la interfaz si el convoy es del jugador y devuelve `ret`.
fn finish(w: &TickWorld, b: usize, host: &mut dyn ConvoyHost, ret: bool) -> bool {
    if w.remove.convoys[b] as u32 == w.player {
        let main = r16(w.remove.convoys, b + 0x10);
        host.hook("cv_ui", &[main as u32]);
    }
    ret
}

fn arrive_final(
    w: &mut TickWorld,
    cv: u16,
    flag: u32,
    host: &mut dyn ConvoyHost,
    route: u32,
    arr: u32,
    count: u16,
    se: u16,
) -> bool {
    let b = cv as usize * CONVOY_SIZE;
    // El original lee los campos del barco por el puntero de la entrada (`edi`) y los argumentos por `convoy+0x10`.
    let main = r16(w.remove.convoys, b + 0x10);
    let e = se as usize * SHIP_SIZE;
    let owner = w.remove.ships[e];
    let cx = r16(w.remove.ships, e + 0x1E);
    let town = host.hook("cv_517260", &[cx as u32, arr]) as i32;
    let cowner = w.remove.convoys[b];
    let mercc = w.remove.merchant_count;
    let townc = w.remove.town_count;
    let ships_flag = flag & 0xFF != 0;
    if cowner as u16 >= mercc {
        host.clear_ship_route(w, main);
        w16(w.remove.ships, e + 0x134, 0x12);
        w16(w.remove.convoys, b + 0x12, 0x12);
        if town < 0 || town as u32 >= townc || w.remove.convoys[b + 0x1E] as i32 == town {
            w16(w.remove.convoys, b + 0x1C, 0xFF);
            return true;
        }
        w16(w.remove.convoys, b + 0x1C, town as u16);
        return false;
    }
    if town < 0 || town as u32 >= townc {
        return off_map(w, cv, host, route, count, se);
    }
    let t = town as usize;
    host.clear_ship_route(w, main);
    let convoy = r16(w.remove.ships, e + 8);
    let heading = w.remove.ships[e + 1];
    host.set_motion(w, 0, 0, convoy, heading);
    let tflags = r32(w.remove.towns, t * super::remove::TOWN_SIZE + 0x2C8) as u32;
    let bl = town as u8;
    if w.remove.ships[e + 0x15C] != 0 {
        if tflags & 0x0400_0000 == 0x0400_0000 {
            if merchant_human(w, cowner) {
                let (x, y) = (r16(w.remove.ships, e + 0x1E), r16(w.remove.ships, e + 0x22));
                host.hook("report", &[cowner as u32, 0, x as u32, y as u32, 3, main as u32]);
            }
            return finish(w, b, host, ships_flag);
        }
        w.remove.convoys[b + 0x1E] = bl;
        w16(w.remove.convoys, b + 0x12, 0x14);
        w.remove.convoys[b + 0x1F] = 0xFF;
        w16(
            w.remove.convoys,
            b + 0x1C,
            if tflags & 0x200 == 0 { 0xFF } else { bl as u16 },
        );
        let c1c = w.remove.convoys[b + 0x1C];
        for s in ship_chain(w, b, count) {
            let o = s * SHIP_SIZE;
            let f = r16(w.remove.ships, o + 0x3C) | 0x28;
            w16(w.remove.ships, o + 0x3C, f);
            w.remove.ships[o + 0x39] = c1c;
            w.remove.ships[o + 0x38] = 0xFF;
            w16(w.remove.ships, o + 0x138, 0);
            w16(w.remove.ships, o + 0x134, 0x14);
        }
        return false;
    }
    let human_flag = w.remove.ships[e + 0x136] & 1 == 1;
    let own_ok = (owner as u16) < mercc && human_flag;
    if tflags & 0x0400_0200 != 0 || (tflags & 0x400 != 0 && own_ok) {
        if cowner as u16 >= mercc {
            host.hook("cv_5172a0", &[0xFF, 0x220, 0x368, main as u32]);
        } else if !merchant_human(w, cowner) || w.remove.convoys[b + 0x14] & 2 == 0 {
            host.divert_to_nearest_port_with_world(w, main as u32, bl as u32);
        } else {
            if w.remove.ships[e + 0x136] & 1 == 1 {
                host.hook("pool", &[main as u32]);
            }
            if w.remove.ships[e + 0x38] as i32 == town {
                let (x, y) = (r16(w.remove.ships, e + 0x1E), r16(w.remove.ships, e + 0x22));
                host.hook("report", &[cowner as u32, 0, x as u32, y as u32, 3, main as u32]);
                w.remove.ships[e + 0x136] &= 0xFC;
            }
        }
        w.remove.convoys[b + 0x1E] = bl;
        for s in ship_chain(w, b, count) {
            w.remove.ships[s * SHIP_SIZE + 0x37] = bl;
        }
        w16(w.remove.convoys, b + 0x1C, 0xFF);
        return finish(w, b, host, true);
    }
    w16(w.remove.convoys, b + 0x1C, bl as u16);
    w16(w.remove.convoys, b + 0x12, 3);
    w16(w.remove.convoys, b + 0x16, 0);
    w.remove.convoys[b + 0x1F] = bl;
    for s in ship_chain(w, b, count) {
        let o = s * SHIP_SIZE;
        w.remove.ships[o + 0x3C] |= 0x28;
        w.remove.ships[o + 0x39] = bl;
        w.remove.ships[o + 0x38] = bl;
        w16(w.remove.ships, o + 0x138, 0);
        w16(w.remove.ships, o + 0x134, 3);
    }
    let f = r32(w.remove.convoys, b + 0x18) | 8;
    w32(w.remove.convoys, b + 0x18, f);
    finish(w, b, host, false)
}

/// El destino no es una ciudad válida (0x005028CC).
fn off_map(w: &mut TickWorld, cv: u16, host: &mut dyn ConvoyHost, route: u32, count: u16, se: u16) -> bool {
    let b = cv as usize * CONVOY_SIZE;
    let main = r16(w.remove.convoys, b + 0x10);
    let e = se as usize * SHIP_SIZE;
    let total = host.read(route.wrapping_add(4), 4);
    if r16(w.remove.ships, e + 0x34) as u32 != total {
        return finish(w, b, host, true);
    }
    if w.remove.ships[e + 0x36] != 0 {
        let arr = host.read(route, 4);
        host.hook("cv_free", &[arr]);
        host.hook("cv_free", &[route]);
        w.remove.ships[e + 0x36] = 0;
    }
    w32(w.remove.ships, e + 0x24, 0);
    w16(w.remove.ships, e + 0x34, 0);
    let (x, y) = (
        r16(w.remove.ships, e + 0x1E) as i16,
        r16(w.remove.ships, e + 0x22) as i16,
    );
    if r16(w.remove.convoys, b + 0x24) < 0x7FFF && x < 0x82 && y > 0x578 && y < 0x708 {
        super::convoy_drop::lost_at_edge(w, cv, host);
        if w.remove.convoys[b] as u32 == w.player {
            host.hook("cv_ui", &[main as u32]);
        }
        return false;
    }
    for s in ship_chain(w, b, count) {
        let o = s * SHIP_SIZE;
        let f = r16(w.remove.ships, o + 0x3C) | 0x80;
        w16(w.remove.ships, o + 0x3C, f);
        w32(w.remove.ships, o + 0x28, 0);
        w32(w.remove.ships, o + 0x2C, 0);
        w32(w.remove.ships, o + 0x30, 0);
    }
    let cowner = w.remove.convoys[b];
    if (cowner as u16) < w.remove.merchant_count && merchant_human(w, cowner) {
        let (x, y) = (r16(w.remove.ships, e + 0x1E), r16(w.remove.ships, e + 0x22));
        host.hook("report", &[cowner as u32, 0, x as u32, y as u32, 3, main as u32]);
    }
    finish(w, b, host, true)
}

/// Traduce un evento de `remove_ship` a la llamada con que el oráculo lo registra.
pub(crate) fn remove_hook(host: &mut dyn ConvoyHost, e: &RemoveEvent) {
    match e {
        RemoveEvent::UiA(i) => host.hook("ui_a", &[*i, 0x17]),
        RemoveEvent::UiB(i) => host.hook("ui_b", &[*i, 0x17]),
        RemoveEvent::ClearRoute(i) => host.hook("clear", &[*i]),
        RemoveEvent::OrderRemove(i) => host.hook("order", &[*i]),
        RemoveEvent::PoolRelease(s) => host.hook("release", &[*s as u32, 0]),
        RemoveEvent::MerchantChanged(m) => host.hook("merchant", &[*m as u32]),
        RemoveEvent::ConvoyRelease(p, i) => host.hook("convoy_release", &[*p as u32, *i as u32]),
        RemoveEvent::ConvoyRefresh(c) => host.hook("convoy_refresh", &[*c as u32]),
    };
}

/// `Convoy::release` (0x0050A3D0): da de baja los barcos del convoy, lo saca de la lista de convoyes y lo pone
/// al frente de la lista libre (`manager+0xE4`).
pub fn release_convoy(w: &mut TickWorld, idx: u16, host: &mut dyn ConvoyHost) {
    let b = idx as usize * CONVOY_SIZE;
    if w.remove.ui_a {
        host.hook("cv_ui_a", &[idx as u32]);
    }
    let count = r16(w.remove.manager, 0xF4);
    let mut sh = r16(w.remove.convoys, b + 0xA);
    w16(w.remove.convoys, b + 0xA, 0xFFFF);
    let mut guard = 0;
    while sh < count && guard < 0x10000 {
        guard += 1;
        let next = r16(w.remove.ships, sh as usize * SHIP_SIZE + 6);
        for e in remove_ship(w.remove, sh as u32) {
            remove_hook(host, &e);
        }
        sh = next;
    }
    // Saca el convoy de la lista de convoyes (`manager+0xEC`).
    let total = r16(w.remove.manager, 0xF6);
    let mut cur = r16(w.remove.manager, 0xEC);
    let mut slot: Option<usize> = None; // None = manager+0xEC
    let mut guard = 0;
    while cur < total && guard < 0x10000 {
        guard += 1;
        if cur == idx {
            let next = r16(w.remove.convoys, b + 8);
            match slot {
                None => w16(w.remove.manager, 0xEC, next),
                Some(p) => w16(w.remove.convoys, p * CONVOY_SIZE + 8, next),
            }
            break;
        }
        slot = Some(cur as usize);
        cur = r16(w.remove.convoys, cur as usize * CONVOY_SIZE + 8);
    }
    // Lista libre (`manager+0xE4`): si ya estaba, solo se marca el estado.
    let free = r16(w.remove.manager, 0xE4);
    let mut f = free;
    let mut guard = 0;
    while f < total && guard < 0x10000 {
        guard += 1;
        if f == idx {
            w16(w.remove.convoys, b + 0x12, 0xFF);
            return;
        }
        f = r16(w.remove.convoys, f as usize * CONVOY_SIZE + 8);
    }
    w16(w.remove.convoys, b + 8, free);
    w16(w.remove.manager, 0xE4, idx);
    w16(w.remove.convoys, b + 0x12, 0xFF);
}

/// `Convoy::remove_ship` (0x00509F80): saca un barco de la lista de su convoy y lo coloca en la lista que
/// corresponde al estado del convoy (puerto `+0xE8`, mar `+0xEA`). Si era el barco principal, vacía el convoy.
pub fn remove_from_convoy(w: &mut TickWorld, ship: u16, host: &mut dyn ConvoyHost, depth: u32) {
    let count = r16(w.remove.manager, 0xF4);
    if ship >= count || depth > 64 {
        return;
    }
    let s = ship as usize * SHIP_SIZE;
    let cv = r16(w.remove.ships, s + 8);
    if cv >= r16(w.remove.manager, 0xF6) {
        return;
    }
    let b = cv as usize * CONVOY_SIZE;
    let state = r16(w.remove.convoys, b + 0x12);
    if state > 0x12 || !matches!(state, 0..=5 | 0xF | 0x12) {
        return;
    }
    // Saca el barco de la lista del convoy.
    let mut slot: Option<usize> = None; // None = convoy+0xA
    let mut cur = r16(w.remove.convoys, b + 0xA);
    let mut guard = 0;
    while cur < count && guard < 0x10000 {
        guard += 1;
        if cur == ship {
            let next = r16(w.remove.ships, s + 6);
            match slot {
                None => w16(w.remove.convoys, b + 0xA, next),
                Some(p) => w16(w.remove.ships, p * SHIP_SIZE + 6, next),
            }
            break;
        }
        slot = Some(cur as usize);
        cur = r16(w.remove.ships, cur as usize * SHIP_SIZE + 6);
    }
    let captain = r16(w.remove.ships, s + 0x42);
    let f = r16(w.remove.ships, s + 0x3C) | 0x1068;
    w16(w.remove.ships, s + 0x3C, f);
    w16(w.remove.ships, s + 8, 0xFFFF);
    if captain >= r16(w.remove.manager, 0xF2) {
        w.remove.ships[s + 0x15C] = 0;
    }
    let town = w.remove.convoys[b + 0x1C];
    let mut main_removed = false;
    let push_a = |w: &mut TickWorld| {
        let head = r16(w.remove.manager, 0xE8);
        w16(w.remove.ships, s + 6, head);
        w16(w.remove.manager, 0xE8, ship);
    };
    match state {
        0 | 1 | 4 => {
            w16(w.remove.ships, s + 0x134, state);
            w16(w.remove.ships, s + 0x138, 0);
            w.remove.ships[s + 0x38] = town;
            w.remove.ships[s + 0x37] = town;
            w.remove.ships[s + 0x39] = town;
            push_a(w);
            return;
        }
        5 => {
            w16(w.remove.ships, s + 0x134, 0);
            w16(w.remove.ships, s + 0x138, 0);
            w.remove.ships[s + 0x38] = town;
            w.remove.ships[s + 0x37] = town;
            w.remove.ships[s + 0x39] = town;
            push_a(w);
            return;
        }
        2 | 3 => {
            let main = r16(w.remove.convoys, b + 0x10);
            let last;
            if ship != main {
                w16(w.remove.ships, s + 0x134, 3);
                if state == 3 {
                    let v = r16(w.remove.convoys, b + 0x16);
                    w16(w.remove.ships, s + 0x138, v);
                } else {
                    let v = r16(w.remove.convoys, b + 0x16);
                    w16(w.remove.ships, s + 0x138, if v < 0x40 { 0x40 - v } else { 0 });
                }
                w.remove.ships[s + 0x37] = town;
                w.remove.ships[s + 0x39] = town;
                last = town;
            } else {
                w16(w.remove.ships, s + 0x134, state);
                w.remove.ships[s + 0x37] = town;
                w.remove.ships[s + 0x39] = town;
                last = w.remove.convoys[b + 0x1F];
                main_removed = true;
            }
            w.remove.ships[s + 0x38] = last;
            push_a(w);
        }
        0xF => {
            let main = r16(w.remove.convoys, b + 0x10);
            if ship == main {
                main_removed = true;
            } else {
                host.hook("insert", &[ship as u32, main as u32]);
                w32(w.remove.ships, s + 0x30, 0);
                w32(w.remove.ships, s + 0x28, 0);
                w32(w.remove.ships, s + 0x2C, 0);
            }
            let sea = r16(w.remove.manager, 0xEA);
            w16(w.remove.ships, s + 6, sea);
            w16(w.remove.manager, 0xEA, ship);
            w16(w.remove.ships, s + 0x134, 0xF);
            w.remove.ships[s + 0x39] = 0xFF;
        }
        _ => {
            pick_main(w, cv, host);
        }
    }
    let f = r32(w.remove.convoys, b + 0x18) | 0x4000;
    w32(w.remove.convoys, b + 0x18, f);
    if main_removed {
        let mut g = 0;
        loop {
            let head = r16(w.remove.convoys, b + 0xA);
            if head >= count || g > 0x10000 {
                break;
            }
            g += 1;
            remove_from_convoy(w, head, host, depth + 1);
        }
    }
}

/// `ships::put_at_sea` (0x00505C30): el barco entra en la lista del mar, sin ciudad actual.
pub(crate) fn put_at_sea(w: &mut TickWorld, sh: u16, host: &mut dyn ConvoyHost) {
    let sea = r16(w.remove.manager, 0xEA);
    host.insert_order(w, sh, sea);
    host.recompute_ship(w, sh);
    let sea = r16(w.remove.manager, 0xEA);
    let o = sh as usize * SHIP_SIZE;
    w16(w.remove.ships, o + 6, sea);
    w.remove.ships[o + 0x39] = 0xFF;
    w16(w.remove.manager, 0xEA, sh);
}

/// `Convoy::pick_main` (0x005017F0): comprueba o elige el barco principal del convoy. Devuelve `true` si el
/// convoy sigue con barco principal utilizable.
pub fn pick_main(w: &mut TickWorld, cv: u16, host: &mut dyn ConvoyHost) -> bool {
    let b = cv as usize * CONVOY_SIZE;
    let count = r16(w.remove.manager, 0xF4);
    let capacity = r16(w.remove.manager, 0xF2);
    let mercc = w.remove.merchant_count;
    let ship_o = |i: u16| i as usize * SHIP_SIZE;
    let mut flag = false;
    let head = r16(w.remove.convoys, b + 0xA);
    if head >= count {
        w16(w.remove.convoys, b + 0x10, 0xFFFF);
        return false;
    }
    let main = r16(w.remove.convoys, b + 0x10);
    if main < count {
        let m = ship_o(main);
        if (r32(w.remove.ships, m + 0x18) as i32) > 0 && r16(w.remove.ships, m + 0x40) != 0 && head < count {
            // El barco principal sigue en la lista y sano: no hay nada que cambiar.
            let mut a = head;
            let mut g = 0;
            while a < count && g < 0x10000 {
                g += 1;
                if a == r16(w.remove.convoys, b + 0x10) {
                    return true;
                }
                a = r16(w.remove.ships, ship_o(a) + 6);
            }
        }
        let st = r16(w.remove.ships, m + 0x134);
        if st == 0xF || st == 0x10 || st == 0x12 {
            flag = r16(w.remove.ships, m + 0xA) < count || r16(w.remove.ships, m + 0xC) < count;
            host.remove_order_with_world(w, main);
        }
    }
    let insert_main = |w: &mut TickWorld, host: &mut dyn ConvoyHost| {
        let mn = r16(w.remove.convoys, b + 0x10);
        host.hook("insert", &[mn as u32, 0xFFFF]);
    };
    let owner = w.remove.convoys[b];
    if owner as u16 >= mercc {
        // Convoy sin comerciante: el primer barco vivo con un capitán libre.
        let h = r16(w.remove.convoys, b + 0xA);
        w16(w.remove.convoys, b + 0x10, h);
        let mut cur = h;
        let mut g = 0;
        while cur < count && g < 0x10000 {
            g += 1;
            let o = ship_o(cur);
            if (r32(w.remove.ships, o + 0x18) as i32) > 0 && r16(w.remove.ships, o + 0x42) < capacity {
                w.remove.convoys[b + 0x14] &= 0xEF;
                if flag {
                    insert_main(w, host);
                }
                return true;
            }
            cur = r16(w.remove.ships, o + 6);
            w16(w.remove.convoys, b + 0x10, cur);
        }
        w16(w.remove.convoys, b + 0x10, h);
        let mut cur = h;
        g = 0;
        while cur < count && g < 0x10000 {
            g += 1;
            let o = ship_o(cur);
            if (r32(w.remove.ships, o + 0x18) as i32) > 0 {
                let id = host.allocate_captain_with_world(w, 1);
                w16(w.remove.ships, o + 0x42, id);
                w.remove.convoys[b + 0x14] &= 0xEF;
                if flag {
                    insert_main(w, host);
                }
                return true;
            }
            cur = r16(w.remove.ships, o + 6);
            w16(w.remove.convoys, b + 0x10, cur);
        }
        return false;
    }
    // Convoy de un comerciante: un barco suyo que el comerciante pueda aceptar.
    let main_now = r16(w.remove.convoys, b + 0x10);
    let mut cand = r16(w.remove.convoys, b + 0xA);
    let mut g = 0;
    while cand < count && g < 0x10000 {
        g += 1;
        if cand != main_now && w.remove.ships[ship_o(cand)] == owner && super::convoy_drop::pool_ok(w, cand) {
            // Intercambia la ranura de escena con el antiguo principal.
            if main_now < count {
                let new_o = ship_o(cand);
                let old_o = ship_o(main_now);
                let slot = r16(w.remove.ships, new_o + 0x132);
                if slot < w.remove.pool_limit {
                    host.hook("release", &[slot as u32, 1]);
                }
                let v = r16(w.remove.ships, old_o + 0x132);
                w16(w.remove.ships, new_o + 0x132, v);
                let bit = (w.remove.ships[old_o + 0x134] & 1) as u16;
                w16(w.remove.ships, old_o + 0x132, 0xFFFF);
                let st = r16(w.remove.ships, new_o + 0x134) | bit;
                w16(w.remove.ships, new_o + 0x134, st);
            }
            w16(w.remove.convoys, b + 0x10, cand);
            if flag {
                host.hook("insert", &[cand as u32, 0xFFFF]);
            }
            return true;
        }
        cand = r16(w.remove.ships, ship_o(cand) + 6);
    }
    let head = r16(w.remove.convoys, b + 0xA);
    w.remove.convoys[b + 0x14] |= 0x10;
    // Sin candidato aceptable: un barco del mismo dueño con capitán libre.
    let mut cur = head;
    g = 0;
    while cur < count && g < 0x10000 {
        g += 1;
        let o = ship_o(cur);
        if w.remove.ships[o] == owner && r16(w.remove.ships, o + 0x42) < capacity {
            w16(w.remove.convoys, b + 0x10, cur);
            if flag {
                host.hook("insert", &[cur as u32, 0xFFFF]);
            }
            return true;
        }
        cur = r16(w.remove.ships, o + 6);
    }
    let mut cur = head;
    g = 0;
    while cur < count && g < 0x10000 {
        g += 1;
        let o = ship_o(cur);
        if w.remove.ships[o] == owner {
            w16(w.remove.convoys, b + 0x10, cur);
            if flag {
                host.hook("insert", &[cur as u32, 0xFFFF]);
            }
            return false;
        }
        cur = r16(w.remove.ships, o + 6);
    }
    w16(w.remove.convoys, b + 0x10, head);
    let first_owner = w.remove.ships[ship_o(head)];
    let f = r32(w.remove.convoys, b + 0x18) | 0x400;
    w.remove.convoys[b] = first_owner;
    w32(w.remove.convoys, b + 0x18, f);
    false
}

/// `Convoy::refresh_from_ships` (0x00501BE0), sobre las imágenes del gestor.
/// Rechaza ciclos para no recorrer indefinidamente una partida dañada.
pub fn refresh_from_ships(w: &mut TickWorld, cv: u16) -> Result<(), DivideError> {
    let b = usize::from(cv) * CONVOY_SIZE;
    let count = r16(w.remove.manager, 0xf4);
    let mut slot = None;
    let mut cur = r16(w.remove.convoys, b + 0xa);
    let mut seen = vec![false; count as usize];
    let mut found = false;
    w.remove.convoys[b + 1] = 0;
    w32(w.remove.convoys, b + 4, 0);
    w32(w.remove.convoys, b + 0xc, -1);
    while cur < count {
        let i = cur as usize;
        if seen[i] {
            return Err(DivideError);
        }
        seen[i] = true;
        let s = i * SHIP_SIZE;
        w.remove.convoys[b + 1] |= w.remove.ships[s + 0xe];
        let speed = super::motion::get_speed(&w.remove.ships[s..s + SHIP_SIZE])?;
        if speed < (r32(w.remove.convoys, b + 0xc) as u32) {
            w32(w.remove.convoys, b + 0xc, speed as i32)
        }
        if cur == r16(w.remove.convoys, b + 0x10) {
            let flags = r32(w.remove.convoys, b + 0x18) | 0x20;
            w32(w.remove.convoys, b + 0x18, flags);
            let state = r16(w.remove.ships, s + 0x134);
            let typ = if state <= 0x16 {
                [0u8, 0, 0, 0, 0, 0, 0, 1, 1, 2, 3, 8, 1, 1, 1, 0, 4, 5, 0, 0, 0, 6, 7][state as usize]
            } else {
                8
            };
            match typ {
                0 => w16(w.remove.convoys, b + 0x12, state),
                1 => {
                    w16(w.remove.convoys, b + 0x12, 0);
                    w16(w.remove.convoys, b + 0x10, 0xffff);
                }
                2 => w16(w.remove.convoys, b + 0x12, 9),
                3 => w16(w.remove.convoys, b + 0x12, 0xa),
                4 => {
                    w16(w.remove.convoys, b + 0x12, 0xf);
                    w.remove.ships[s + 0x3c] |= 0x20;
                    w16(w.remove.ships, s + 0x134, 0xf)
                }
                5 => {
                    w16(w.remove.convoys, b + 0x12, 0xf);
                    w16(w.remove.convoys, b + 0x10, 0xffff)
                }
                6 => {
                    let next = r16(w.remove.ships, s + 6);
                    match slot {
                        None => w16(w.remove.convoys, b + 0xa, next),
                        Some(o) => w16(w.remove.ships, o, next),
                    }
                }
                7 => w16(w.remove.convoys, b + 0x12, 0x16),
                _ => match slot {
                    None => w16(w.remove.convoys, b + 0xa, 0xffff),
                    Some(o) => w16(w.remove.ships, o, 0xffff),
                },
            }
            if matches!(typ, 0 | 2 | 3 | 4 | 7) {
                found = true;
                w.remove.convoys[b] = w.remove.ships[s];
                w16(w.remove.convoys, b + 0x1c, u16::from(w.remove.ships[s + 0x39]));
                w.remove.convoys[b + 0x1e] = w.remove.ships[s + 0x37];
                w.remove.convoys[b + 0x1f] = w.remove.ships[s + 0x38];
                w16(w.remove.convoys, b + 0x16, r16(w.remove.ships, s + 0x138));
            }
            if typ == 6 {
                cur = r16(w.remove.ships, s + 6);
                continue;
            }
            if typ == 8 {
                break;
            }
        }
        slot = Some(s + 6);
        cur = r16(w.remove.ships, s + 6);
    }
    if !found {
        w16(w.remove.convoys, b + 0x10, 0xffff);
        return Ok(());
    }
    cur = r16(w.remove.convoys, b + 0xa);
    seen.fill(false);
    while cur < count {
        let i = cur as usize;
        if seen[i] {
            return Err(DivideError);
        }
        seen[i] = true;
        let s = i * SHIP_SIZE;
        if w.remove.ships[s] == w.remove.convoys[b] {
            w32(
                w.remove.convoys,
                b + 4,
                r32(w.remove.convoys, b + 4).wrapping_add(r32(w.remove.ships, s + 0x10)),
            )
        } else {
            let f = r16(w.remove.convoys, b + 0x14) & 0xfffd;
            w16(w.remove.convoys, b + 0x14, f)
        }
        let town = r16(w.remove.convoys, b + 0x1c);
        if u16::from(w.remove.ships[s + 0x39]) != town {
            let f = r16(w.remove.ships, s + 0x3c) | 8;
            w16(w.remove.ships, s + 0x3c, f);
            w.remove.ships[s + 0x39] = town as u8
        }
        if r16(w.remove.convoys, b + 0x12) == 1 {
            let a = r16(w.remove.convoys, b + 0x16);
            let v = r16(w.remove.ships, s + 0x138);
            if a < v {
                w16(w.remove.convoys, b + 0x16, v)
            }
        } else {
            w16(w.remove.ships, s + 0x138, r16(w.remove.convoys, b + 0x16))
        }
        w.remove.ships[s + 0x37] = w.remove.convoys[b + 0x1e];
        w.remove.ships[s + 0x38] = w.remove.convoys[b + 0x1f];
        w16(w.remove.ships, s + 0x134, r16(w.remove.convoys, b + 0x12));
        cur = r16(w.remove.ships, s + 6);
    }
    Ok(())
}

/// Primer paso de salida de convoy con ruta ya calculada; separa miembros no aptos.
/// Los demás caminos se rechazan sin modificar el mundo.
pub fn begin_departure_ready(w: &mut TickWorld, cv: u16, port: (u16, u16)) -> Result<(), String> {
    begin_departure_ready_with_route(w, cv, port, None)
}
pub fn begin_departure_ready_with_route(
    w: &mut TickWorld,
    cv: u16,
    port: (u16, u16),
    prepared: Option<(&[(i16, i16)], bool, u32)>,
) -> Result<(), String> {
    use super::remove::{RemoveWorld, TOWN_SIZE};
    let mut manager = w.remove.manager.to_vec();
    let mut ships = w.remove.ships.to_vec();
    let mut captains = w.remove.captains.to_vec();
    let mut convoys = w.remove.convoys.to_vec();
    let mut merchants = w.remove.merchants.to_vec();
    let mut towns = w.remove.towns.to_vec();
    let b = usize::from(cv) * CONVOY_SIZE;
    let count = r16(&manager, 0xf4);
    if b + CONVOY_SIZE > convoys.len() || cv >= r16(&manager, 0xf6) {
        return Err("convoy inválido".into());
    }
    let leader = r16(&convoys, b + 0x10);
    if leader >= count {
        return Err("líder inválido".into());
    }
    let o = usize::from(leader) * SHIP_SIZE;
    let cap = r16(&manager, 0xf2);
    let ncv = r16(&manager, 0xf6);
    let city = r16(&convoys, b + 0x1c) as usize;
    let target = convoys[b + 0x1f] as usize;
    if o + SHIP_SIZE > ships.len()
        || city >= w.remove.town_count as usize
        || target >= w.remove.town_count as usize
        || towns.len() < (w.remove.town_count as usize) * TOWN_SIZE
    {
        return Err("ciudades de convoy inválidas".into());
    }
    let town_images = towns
        .chunks_exact(TOWN_SIZE)
        .map(crate::town::Town::from_bytes)
        .collect::<Vec<_>>();
    if town_images[city].flags() & 0x04000200 != 0 {
        return Err("puerto bloqueado".into());
    }
    let l = &ships[o..o + SHIP_SIZE];
    if r16(l, 0x40) < 20
        || r32(l, 0x120) < 0x43
        || r16(l, 0x42) >= cap
        || l[0x15c] != 0
        || r16(l, 8) >= ncv
        || !super::departure::ready_to_sail(l, &town_images, cap).unwrap_or(false)
    {
        return Err("líder no apto".into());
    }
    let mut chain = Vec::new();
    let mut cur = r16(&convoys, b + 0xa);
    while cur < count {
        if chain.len() >= count as usize {
            return Err("ciclo en convoy".into());
        }
        let at = usize::from(cur) * SHIP_SIZE;
        if at + SHIP_SIZE > ships.len() {
            return Err("miembro inválido".into());
        }
        chain.push(at);
        cur = r16(&ships, at + 6)
    }
    if !chain.contains(&o) {
        return Err("líder ausente".into());
    }
    super::route::start_route_from_port(&mut ships[o..o + SHIP_SIZE], port, prepared, w.time).ok_or("ruta ausente")?;
    convoys[b + 0x14] &= 0xef;
    let arrival = r32(&ships, o + 0x48);
    let owner = convoys[b];
    for at in chain {
        if !super::departure::ready_to_sail(&ships[at..at + SHIP_SIZE], &town_images, cap).unwrap_or(false) {
            ships[at + 0x3c] |= 0x40;
            ships[at + 0x38] = ships[at + 0x39];
            let f = r32(&convoys, b + 0x18) | 0x4020;
            w32(&mut convoys, b + 0x18, f);
            struct H {
                called: bool,
            }
            impl super::tick::ConvoyHost for H {
                fn hook(&mut self, _: &'static str, _: &[u32]) -> u32 {
                    self.called = true;
                    0
                }
                fn read(&mut self, _: u32, _: u8) -> u32 {
                    self.called = true;
                    0
                }
            }
            let mut host = H { called: false };
            {
                let mut remove = RemoveWorld {
                    manager: &mut manager,
                    ships: &mut ships,
                    captains: &mut captains,
                    convoys: &mut convoys,
                    merchants: &mut merchants,
                    towns: &mut towns,
                    pool_limit: w.remove.pool_limit,
                    merchant_count: w.remove.merchant_count,
                    town_count: w.remove.town_count,
                    ui_a: w.remove.ui_a,
                    ui_b: w.remove.ui_b,
                };
                let mut staged = TickWorld {
                    remove: &mut remove,
                    time: w.time,
                    player: w.player,
                };
                remove_from_convoy(&mut staged, (at / SHIP_SIZE) as u16, &mut host, 0);
            }
            if host.called {
                return Err("efecto de separación no admitido".into());
            }
            w16(&mut ships, at + 0x134, 0);
            continue;
        }
        for ware in 0..24 {
            let value = r32(&ships, at + 0x54 + 4 * ware);
            let a = target * TOWN_SIZE + 0x670 + 4 * ware;
            let z = city * TOWN_SIZE + 0x370 + 4 * ware;
            let av = r32(&towns, a).wrapping_add(value);
            let zv = r32(&towns, z).wrapping_add(value);
            w32(&mut towns, a, av);
            w32(&mut towns, z, zv);
        }
        if ships[at] != owner {
            convoys[b + 0x14] &= 0xfd
        }
        w16(&mut ships, at + 0x134, 2);
        ships[at + 0x37] = city as u8;
        ships[at + 0x3c] |= 0x20;
        ships[at + 0x38] = target as u8;
        w32(&mut ships, at + 0x48, arrival);
    }
    convoys[b + 0x1e] = city as u8;
    {
        let mut remove = RemoveWorld {
            manager: &mut manager,
            ships: &mut ships,
            captains: &mut captains,
            convoys: &mut convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: w.remove.pool_limit,
            merchant_count: w.remove.merchant_count,
            town_count: w.remove.town_count,
            ui_a: w.remove.ui_a,
            ui_b: w.remove.ui_b,
        };
        let mut staged = TickWorld {
            remove: &mut remove,
            time: w.time,
            player: w.player,
        };
        refresh_from_ships(&mut staged, cv).map_err(|_| "recuento de convoy inválido")?;
    }
    w16(&mut convoys, b + 0x12, 2);
    w16(&mut convoys, b + 0x16, 1);
    w.remove.manager.copy_from_slice(&manager);
    w.remove.ships.copy_from_slice(&ships);
    w.remove.captains.copy_from_slice(&captains);
    w.remove.convoys.copy_from_slice(&convoys);
    w.remove.merchants.copy_from_slice(&merchants);
    w.remove.towns.copy_from_slice(&towns);
    Ok(())
}

/// `convoy_set_ship_speeds_to_half` (0x00503CF0).
pub fn set_ship_speeds_to_half(w: &mut TickWorld, cv: u16) -> Result<(), String> {
    let b = usize::from(cv) * CONVOY_SIZE;
    let count = r16(w.remove.manager, 0xf4);
    if b + CONVOY_SIZE > w.remove.convoys.len() {
        return Err("convoy inválido".into());
    }
    let speed = (r32(w.remove.convoys, b + 0xc) as u32) >> 1;
    let mut cur = r16(w.remove.convoys, b + 0xa);
    let mut visited = vec![false; count as usize];
    let mut chain = Vec::new();
    while cur < count {
        let i = usize::from(cur);
        if visited[i] {
            return Err("ciclo en convoy".into());
        }
        visited[i] = true;
        let s = i * SHIP_SIZE;
        if s + SHIP_SIZE > w.remove.ships.len() {
            return Err("barco inválido".into());
        }
        chain.push(s);
        cur = r16(w.remove.ships, s + 6)
    }
    for s in chain {
        w32(w.remove.ships, s + 0x30, speed as i32)
    }
    Ok(())
}

/// Primer paso del convoy con cancelación por líder no apto o bloqueo de puerto.
/// La rama con ruta ausente se rechaza sin modificar el mundo.
pub fn begin_departure(
    w: &mut TickWorld,
    cv: u16,
    port: (u16, u16),
    host: &mut dyn super::route::RouteHost,
) -> Result<bool, String> {
    begin_departure_with_route(w, cv, port, host, None)
}
pub fn begin_departure_with_route(
    w: &mut TickWorld,
    cv: u16,
    port: (u16, u16),
    host: &mut dyn super::route::RouteHost,
    prepared: Option<(&[(i16, i16)], bool, u32)>,
) -> Result<bool, String> {
    use super::remove::TOWN_SIZE;
    let b = usize::from(cv) * CONVOY_SIZE;
    if b + CONVOY_SIZE > w.remove.convoys.len() || cv >= r16(w.remove.manager, 0xf6) {
        return Err("convoy inválido".into());
    }
    let count = r16(w.remove.manager, 0xf4);
    let leader = r16(w.remove.convoys, b + 0x10);
    if leader >= count {
        w16(w.remove.convoys, b + 0x12, 0);
        return Ok(false);
    }
    let at = usize::from(leader) * SHIP_SIZE;
    if at + SHIP_SIZE > w.remove.ships.len() {
        return Err("líder inválido".into());
    }
    let cap = r16(w.remove.manager, 0xf2);
    let city = r16(w.remove.convoys, b + 0x1c) as usize;
    let towns = w
        .remove
        .towns
        .chunks_exact(TOWN_SIZE)
        .map(crate::town::Town::from_bytes)
        .collect::<Vec<_>>();
    let ship = &w.remove.ships[at..at + SHIP_SIZE];
    let convoy_count = r16(w.remove.manager, 0xf6);
    let can_lead = r16(ship, 0x40) >= 20
        && (r16(ship, 8) < convoy_count || r32(ship, 0x18) >= (r32(ship, 0x14) >> 1))
        && r32(ship, 0x120) >= 0x43
        && r16(ship, 0x42) < cap
        && ship[0x15c] == 0;
    if !can_lead || !super::departure::ready_to_sail(ship, &towns, cap).unwrap_or(false) {
        let f = r32(w.remove.convoys, b + 0x18) | 0x20;
        w32(w.remove.convoys, b + 0x18, f);
        w16(w.remove.convoys, b + 0x12, 0);
        let s = &mut w.remove.ships[at..at + SHIP_SIZE];
        s[0x38] = s[0x39];
        w16(s, 0x134, 0);
        w16(s, 0x3c, 0xa0);
        return Ok(false);
    }
    if city < towns.len() && towns[city].flags() & 0x04000200 != 0 {
        let f = r32(w.remove.convoys, b + 0x18) | 0x20;
        w32(w.remove.convoys, b + 0x18, f);
        w16(w.remove.convoys, b + 0x12, 1);
        w16(w.remove.convoys, b + 0x16, 0x100);
        let s = &mut w.remove.ships[at..at + SHIP_SIZE];
        s[0x38] = city as u8;
        w16(s, 0x134, 1);
        w16(s, 0x3c, 0xa0);
        w16(s, 0x138, 0x100);
        super::route::clear_route(s, leader, w.player, false, host);
        return Ok(false);
    }
    begin_departure_ready_with_route(w, cv, port, prepared)?;
    Ok(true)
}

/// `set_convoy_ships_motion` (0x0050A240). Las cadenas inválidas se rechazan antes de escribir.
pub fn set_convoy_motion(w: &mut TickWorld, cv: u16, arrival: u32, speed: u32, heading: u8) -> Result<(), String> {
    let count = r16(w.remove.manager, 0xf4);
    if cv >= r16(w.remove.manager, 0xf6) {
        return Ok(());
    }
    let b = usize::from(cv) * CONVOY_SIZE;
    if b + CONVOY_SIZE > w.remove.convoys.len() {
        return Err("convoy inválido".into());
    }
    let mut cur = r16(w.remove.convoys, b + 0xa);
    let mut visited = vec![false; count as usize];
    let mut chain = Vec::new();
    while cur < count {
        let i = usize::from(cur);
        if visited[i] {
            return Err("ciclo en convoy".into());
        }
        visited[i] = true;
        let at = i * SHIP_SIZE;
        if at + SHIP_SIZE > w.remove.ships.len() {
            return Err("barco inválido".into());
        }
        chain.push(at);
        cur = r16(w.remove.ships, at + 6)
    }
    for at in chain {
        w32(w.remove.ships, at + 0x30, speed as i32);
        w32(w.remove.ships, at + 0x48, arrival as i32);
        w.remove.ships[at + 1] = heading
    }
    Ok(())
}

/// Anfitrión de `pick_main` (0x005017F0) sobre las listas vivas de un convoy sin comerciante:
/// `remove_from_x_order` 0x50D040 directo, `allocate_captain` 0x5097C0(1) sin crecimiento del pool
/// (`init_captain(flag=false)` con los recursos dados) y, si se admite, la inserción final
/// `insert_in_x_order(main, 0xFFFF)` aplazada a `inserts` (es la última acción antes de volver).
/// Cualquier otro efecto marca `unsupported`.
pub(crate) struct LeaderHooks {
    pub unsupported: bool,
    pub resources: Option<crate::economy::pirate_group_callbacks::CaptainResources>,
    pub allow_insert: bool,
    pub inserts: Vec<u16>,
    pub allocated: Vec<u16>,
    /// La lista libre estaba agotada (`free_head == capacidad`): 0x5097C0 ampliaría el pool.
    pub needs_growth: bool,
}

impl LeaderHooks {
    pub fn new(resources: Option<crate::economy::pirate_group_callbacks::CaptainResources>, allow_insert: bool) -> Self {
        Self { unsupported: false, resources, allow_insert, inserts: Vec::new(), allocated: Vec::new(), needs_growth: false }
    }
}

impl ConvoyHost for LeaderHooks {
    fn hook(&mut self, name: &'static str, args: &[u32]) -> u32 {
        match (name, args) {
            ("order", _) => {}
            ("insert", [ship, 0xFFFF]) if self.allow_insert => self.inserts.push(*ship as u16),
            _ => self.unsupported = true,
        }
        0
    }
    fn read(&mut self, _: u32, _: u8) -> u32 {
        self.unsupported = true;
        0
    }
    fn remove_order_with_world(&mut self, world: &mut TickWorld, ship: u16) {
        super::order::remove_from_tick(world, ship)
    }
    fn allocate_captain_with_world(&mut self, world: &mut TickWorld, flag: u8) -> u16 {
        let (Some(r), 1) = (self.resources, flag) else {
            self.unsupported = true;
            return u16::MAX;
        };
        let head = r16(world.remove.manager, 0xe0);
        let at = usize::from(head) * 16;
        if head == r16(world.remove.manager, 0xf2) {
            self.needs_growth = true;
        }
        if head >= r16(world.remove.manager, 0xf2) || at + 16 > world.remove.captains.len() {
            self.unsupported = true;
            return u16::MAX;
        }
        let next = r16(world.remove.captains, at);
        let mut record: [u8; 16] = world.remove.captains[at..at + 16].try_into().unwrap();
        record[..2].fill(255);
        if super::factory::init_captain(&mut record, u32::from(head), false, r.tick, r.day_of_year, r.day, r.population, r.first_names, r.last_names).is_none() {
            self.unsupported = true;
            return u16::MAX;
        }
        world.remove.captains[at..at + 16].copy_from_slice(&record);
        w16(world.remove.manager, 0xe0, next);
        self.allocated.push(head);
        head
    }
}
