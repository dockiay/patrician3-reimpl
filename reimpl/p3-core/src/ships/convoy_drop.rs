//! `Convoy::dissolve` (0x0050A580): disuelve un convoy, devolviendo barcos ajenos con su reembolso y avisos.
//! Las llamadas a mapa (0x531B70), mensajes (0x4D6530), 0x4F9050 y la interfaz son ganchos.

use super::convoy::{release_convoy, remove_from_convoy};
use super::remove::{CONVOY_SIZE, MERCHANT_SIZE, SHIP_SIZE, TOWN_SIZE};
use super::tick::{ConvoyError, ConvoyHost, TickWorld};
use super::{r16, r32, w16, w32};

const GUARD: usize = 0x10000;

fn merchant(w: &TickWorld, owner: u8) -> usize {
    owner as usize * MERCHANT_SIZE
}

fn remove_head(w: &mut TickWorld, b: usize, host: &mut dyn ConvoyHost) {
    let mut g = 0;
    while r16(w.remove.convoys, b + 0xA) < r16(w.remove.manager, 0xF4) && g < GUARD {
        g += 1;
        let head = r16(w.remove.convoys, b + 0xA);
        remove_from_convoy(w, head, host, 0);
    }
}

fn post(w: &mut TickWorld, host: &mut dyn ConvoyHost, name: &'static str, args: &[u32]) {
    let _ = w;
    host.hook(name, args);
}

/// Un convoy en el estado 6 (en la lista de su ciudad) vuelve a la lista del mar.
fn relink_from_town(w: &mut TickWorld, cv: u16, b: usize, count: u16) -> Result<bool, ConvoyError> {
    let town = r16(w.remove.convoys, b + 0x1C) as usize;
    if town >= w.remove.town_count as usize {
        return Err(ConvoyError::Unported(6));
    }
    let convoy_count = r16(w.remove.manager, 0xF6);
    let head_at = town * TOWN_SIZE + 0x820;
    let mut link: Option<usize> = None; // None = cabeza de la ciudad
    let mut val = r16(w.remove.towns, head_at);
    let mut g = 0;
    while val != cv {
        if val >= convoy_count || g > GUARD {
            return Ok(false);
        }
        g += 1;
        link = Some(val as usize);
        val = r16(w.remove.convoys, val as usize * CONVOY_SIZE + 8);
    }
    let next = r16(w.remove.convoys, b + 8);
    match link {
        None => w16(w.remove.towns, head_at, next),
        Some(c) => w16(w.remove.convoys, c * CONVOY_SIZE + 8, next),
    }
    let head = r16(w.remove.manager, 0xEC);
    w16(w.remove.convoys, b + 8, head);
    w16(w.remove.manager, 0xEC, cv);
    w16(w.remove.convoys, b + 0x12, 0);
    w16(w.remove.convoys, b + 0x16, 0);
    let mut s = r16(w.remove.convoys, b + 0xA);
    let mut g = 0;
    while s < count && g < GUARD {
        g += 1;
        let o = s as usize * SHIP_SIZE;
        let f = r16(w.remove.ships, o + 0x3C) | 0x20;
        w16(w.remove.ships, o + 0x3C, f);
        w16(w.remove.ships, o + 0x134, 0);
        w16(w.remove.ships, o + 0x138, 0);
        s = r16(w.remove.ships, o + 6);
    }
    Ok(true)
}

pub fn dissolve_convoy(w: &mut TickWorld, cv: u16, host: &mut dyn ConvoyHost) -> Result<(), ConvoyError> {
    let b = cv as usize * CONVOY_SIZE;
    if cv >= r16(w.remove.manager, 0xF6) {
        return Ok(());
    }
    let count = r16(w.remove.manager, 0xF4);
    let mercc = w.remove.merchant_count;
    let townc = w.remove.town_count;
    if w.remove.ui_a {
        host.hook("cv_ui_a", &[cv as u32]);
    }
    let mut nrem: u32 = 0;
    if r16(w.remove.convoys, b + 0x12) == 6 && !relink_from_town(w, cv, b, count)? {
        return Ok(());
    }
    let owner = w.remove.convoys[b];
    if owner as u16 >= mercc || w.remove.convoys[b + 0x14] & 2 != 0 {
        remove_head(w, b, host);
        release_convoy(w, cv, host);
        return Ok(());
    }
    let town = r16(w.remove.convoys, b + 0x1C);
    let dest = w.remove.convoys[b + 0x1F];
    let same_trip = (town as u32) < townc && town == dest as u16 && w.remove.convoys[b + 0x1E] != dest;
    if !same_trip {
        // Sin viaje de regreso con reembolso: los barcos ajenos solo se cuentan.
        let mut g = 0;
        while r16(w.remove.convoys, b + 0xA) < count && g < GUARD {
            g += 1;
            let head = r16(w.remove.convoys, b + 0xA);
            if w.remove.ships[head as usize * SHIP_SIZE] != w.remove.convoys[b] {
                nrem += 1;
            }
            remove_from_convoy(w, head, host, 0);
        }
        // `remove_from_convoy` puede reasignar el dueño del convoy: se vuelve a leer.
        let owner = w.remove.convoys[b];
        if (owner as u16) < mercc {
            host.ship_delivery_with_world(w, owner, nrem, 0);
        }
        release_convoy(w, cv, host);
        return Ok(());
    }
    if w.remove.convoys[b + 0x1E] as u32 >= townc {
        w.remove.convoys[b + 0x1E] = 0;
    }
    let cell = host.hook("cv_map", &[w.remove.convoys[b + 0x1E] as u32, town as u32]) & 0xFF;
    let dist = ((cell * 20) as i32 as f64).sqrt() as i32;
    let main = r16(w.remove.convoys, b + 0x10);
    let main_tag = if main < count {
        r16(w.remove.ships, main as usize * SHIP_SIZE + 0x15E)
    } else {
        0xFFFF
    };
    let mut total: u32 = 0;
    let (v1c, v20) = {
        let m = merchant(w, owner);
        let mm = &w.remove.merchants[m..];
        (
            (((mm[0x39C + mm[0x19] as usize] as u32) << 8) + mm[0x1A] as u32) as u16,
            (((mm[0x1C] as u32) << 8) + mm[0x1D] as u32) as u16,
        )
    };
    let mut g = 0;
    while r16(w.remove.convoys, b + 0xA) < count && g < GUARD {
        g += 1;
        let idx = r16(w.remove.convoys, b + 0xA);
        let so = w.remove.ships[idx as usize * SHIP_SIZE];
        if so == w.remove.convoys[b] || so as u16 >= mercc {
            remove_from_convoy(w, idx, host, 0);
            continue;
        }
        // Barcos de otro comerciante: se quitan todos y se paga su valor.
        nrem = 0;
        let mut value: u32 = 0;
        let mut e = idx;
        let mut g2 = 0;
        while e < count && g2 < GUARD {
            g2 += 1;
            let o = e as usize * SHIP_SIZE;
            if w.remove.ships[o] == so {
                let c = r32(w.remove.ships, o + 0x114);
                value = value.wrapping_add(if c < 100 { 100 } else { c as u32 });
                nrem += 1;
                remove_from_convoy(w, e, host, 0);
                e = r16(w.remove.convoys, b + 0xA);
            } else {
                e = r16(w.remove.ships, o + 6);
            }
        }
        let mut refund: u32 = 0;
        if value != 0 {
            let root = (value as f64).sqrt() as i32;
            let v = (root as u32).wrapping_mul(dist as u32).wrapping_add(0x63);
            refund = (((v as u64 * 0x51EB851F) >> 32) as u32 >> 5) * 10;
        }
        let m = merchant(w, so);
        let gold = r32(w.remove.merchants, m).wrapping_sub(refund as i32);
        w32(w.remove.merchants, m, gold);
        let spent = r32(w.remove.merchants, m + 0x4BC).wrapping_add(refund as i32);
        w32(w.remove.merchants, m + 0x4BC, spent);
        total = total.wrapping_add(refund);
        if r16(w.remove.merchants, m + 8) == 0 {
            post(
                w,
                host,
                "msg_refund",
                &[
                    so as u32,
                    dest as u32,
                    main_tag as u32,
                    v1c as u32,
                    refund & 0xFFFF,
                    v20 as u32,
                ],
            );
        }
    }
    let owner = w.remove.convoys[b];
    let m = merchant(w, owner);
    host.ship_delivery_with_world(w, owner, nrem, 1);
    let gold = r32(w.remove.merchants, m).wrapping_add(total as i32);
    w32(w.remove.merchants, m, gold);
    let got = r32(w.remove.merchants, m + 0x4C8).wrapping_add(total as i32);
    w32(w.remove.merchants, m + 0x4C8, got);
    if r16(w.remove.merchants, m + 8) == 0 {
        post(
            w,
            host,
            "msg_total",
            &[owner as u32, dest as u32, total & 0xFFFF, main_tag as u32],
        );
    }
    release_convoy(w, cv, host);
    Ok(())
}

/// Número mínimo de tripulantes por tipo de barco (tabla 0x673660).
const CREW_MIN: [u16; 4] = [5, 8, 10, 12];

/// `Ship::can_join_pool` (0x00519C40): el barco puede ser principal de un convoy de comerciante.
pub fn pool_ok(w: &TickWorld, ship: u16) -> bool {
    let o = ship as usize * SHIP_SIZE;
    let s = &w.remove.ships[o..o + SHIP_SIZE];
    if r16(s, 0x40) < 0x14 {
        return false;
    }
    if r16(s, 8) >= r16(w.remove.manager, 0xF6) && r32(s, 0x18) < (r32(s, 0x14) >> 1) {
        return false;
    }
    if r32(s, 0x120) < 0x43 {
        return false;
    }
    if r16(s, 0x42) >= r16(w.remove.manager, 0xF2) {
        return false;
    }
    s[0x15C] == 0
}

/// `Ship::can_dock_here` (0x00519BA0): el barco está en condiciones de entrar en puerto.
pub fn dock_ok(w: &TickWorld, ship: u16) -> bool {
    let o = ship as usize * SHIP_SIZE;
    let s = &w.remove.ships[o..o + SHIP_SIZE];
    if r16(s, 0x40) < CREW_MIN[(s[0xE] & 3) as usize] || (s[0x3F] as i8) <= 0 || r32(s, 0x118) > r32(s, 0x10) {
        return false;
    }
    if r16(s, 0x42) < r16(w.remove.manager, 0xF2) && r32(s, 0x14) > r32(s, 0x18).wrapping_mul(5) {
        return false;
    }
    let st = r16(s, 0x134);
    if st == 7 || st == 9 || (0xC..=0xE).contains(&st) || st >= 0x11 {
        return false;
    }
    let town = s[0x39] as u32;
    if w.remove.town_count > town {
        let flags = r32(w.remove.towns, town as usize * TOWN_SIZE + 0x2C8);
        if flags & 0x0400_0200 != 0 {
            return false;
        }
    }
    true
}

/// `Convoy::hold_state9` (0x00503930): los barcos del convoy pasan al estado 9 y, al cumplirse 0x20 turnos, se
/// paran. La rama de `get_speed` (cuando el contador vuelve a cero tras sumar) es inalcanzable.
pub fn state9(w: &mut TickWorld, cv: u16) {
    let b = cv as usize * CONVOY_SIZE;
    let count = r16(w.remove.manager, 0xF4);
    let mut sh = r16(w.remove.convoys, b + 0xA);
    let mut g = 0;
    while sh < count && g < GUARD {
        g += 1;
        let o = sh as usize * SHIP_SIZE;
        if r16(w.remove.ships, o + 0x134) != 9 {
            w.remove.ships[o + 0x3C] |= 0x20;
            w16(w.remove.ships, o + 0x134, 9);
            w16(w.remove.ships, o + 0x138, 0);
        }
        let turns = r16(w.remove.ships, o + 0x138);
        if turns < 0x20 {
            w16(w.remove.ships, o + 0x138, turns + 1);
            if turns + 1 == 0x20 {
                w.remove.ships[o + 0x3C] |= 0x80;
                w32(w.remove.ships, o + 0x30, 0);
            }
        }
        sh = r16(w.remove.ships, o + 6);
    }
}

/// `Convoy::lost_at_edge` (0x005029F0): el convoy sale del mapa por el borde; el comerciante humano paga una
/// multa y los barcos quedan fuera de juego (estado 0x13). La tabla de objetos y la interfaz son ganchos.
pub fn lost_at_edge(w: &mut TickWorld, cv: u16, host: &mut dyn ConvoyHost) {
    let b = cv as usize * CONVOY_SIZE;
    let count = r16(w.remove.manager, 0xF4);
    let main = r16(w.remove.convoys, b + 0x10) as usize;
    // El original toma el convoy del barco principal sin comprobar que exista.
    let from_ship = r16(w.remove.ships, main * SHIP_SIZE + 8);
    host.hook("cv_obj_new", &[w.time]);
    host.hook("cv_obj_set", &[from_ship as u32, 0xFFFF, 0x12, w.time]);
    w16(w.remove.convoys, b + 0x12, 0x13);
    host.remove_order(w, main as u16);
    if w.remove.ui_a && host.hook("cv_ui94", &[]) == 3 {
        host.hook("cv_4885a0", &[]);
    }
    let owner = w.remove.convoys[b];
    if (owner as u16) < w.remove.merchant_count {
        let m = merchant(w, owner);
        if r16(w.remove.merchants, m + 8) != 0 {
            let gold = r32(w.remove.merchants, m);
            if gold > 0x9C40 {
                let fine = (gold >> 2).min(0x186A0);
                w32(w.remove.convoys, b + 0x20, fine);
                w32(w.remove.merchants, m, gold.wrapping_sub(fine));
            } else {
                w32(w.remove.convoys, b + 0x20, 0x2710);
            }
        }
    }
    let mut sh = r16(w.remove.convoys, b + 0xA);
    let mut g = 0;
    while sh < count && g < GUARD {
        g += 1;
        let o = sh as usize * SHIP_SIZE;
        w16(w.remove.ships, o + 0x1E, 0x7530);
        w16(w.remove.ships, o + 0x22, 0x7530);
        w16(w.remove.ships, o + 0x134, 0x13);
        sh = r16(w.remove.ships, o + 6);
    }
}
