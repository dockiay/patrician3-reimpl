//! Consultas de solo lectura del planificador de tareas (`scheduled_tasks_*`), sobre la imagen de memoria del original.
//! `s` es la dirección del objeto del planificador (`0x006DD73C`): `+0` tabla de registros de `0x18` bytes, `+8` cabeza de la lista
//! activa, `+0xC` capacidad. Cada registro: `+0` vencimiento, `+4` siguiente (u16), `+6` tipo (u16). Los globales (`0x006DE4B4` reloj,
//! `0x006DE4AA` comerciantes, `0x006DFC14` jugador local) se leen de la propia imagen.
use crate::flatmem::Mem;

pub const G_TICK: u32 = 0x006d_e4b4;
pub const G_MERCHANTS: u32 = 0x006d_e4aa;
pub const G_PLAYER: u32 = 0x006d_fc14;
const G_MERCHANT_TABLE: u32 = 0x006d_e518;

/// `0x005303C0`: dirección del comerciante `i` (`0x650` bytes por ranura).
pub fn merchant_addr(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(G_MERCHANT_TABLE).wrapping_add(i.wrapping_mul(0x650))
}

fn rec(m: &mut dyn Mem, s: u32, i: u32) -> u32 {
    m.r32(s).wrapping_add(i.wrapping_mul(0x18))
}
fn head(m: &mut dyn Mem, s: u32) -> u32 {
    u32::from(m.r16(s + 8))
}
fn count(m: &mut dyn Mem, s: u32) -> u32 {
    u32::from(m.r16(s + 0xc))
}
fn next(m: &mut dyn Mem, r: u32) -> u32 {
    u32::from(m.r16(r + 4))
}

/// Recorre la lista activa y devuelve `(índice, dirección)` del primer registro de `kind` que acepte `pred`.
fn find_chain(
    m: &mut dyn Mem,
    s: u32,
    kind: u16,
    mut pred: impl FnMut(&mut dyn Mem, u32) -> bool,
) -> Option<(u32, u32)> {
    let n = count(m, s);
    let mut i = head(m, s);
    while i < n {
        let r = rec(m, s, i);
        if m.r16(r + 6) == kind && pred(m, r) {
            return Some((i, r));
        }
        i = next(m, r);
    }
    None
}

/// `find_mayor_election` (`0x004D9F10`): registro de tipo 2 de la ciudad, o 0.
pub fn find_mayor_election(m: &mut dyn Mem, s: u32, town: u32) -> u32 {
    find_chain(m, s, 2, |m, r| u32::from(m.r8(r + 0xe)) == town).map_or(0, |(_, r)| r)
}

/// `find_todays_vote_task` (`0x004D9C70`): índice del registro de votación (tipo `0x20`/`0x21`) de hoy o `0xFFFFFFFF`.
pub fn find_todays_vote_task(m: &mut dyn Mem, s: u32) -> u32 {
    let today = m.r32(G_TICK) & 0xffff_ff00;
    let n = count(m, s);
    let mut i = head(m, s);
    while i < n {
        let r = rec(m, s, i);
        if m.r32(r).wrapping_sub(today) > 0xff {
            return 0xffff_ffff;
        }
        let k = m.r16(r + 6);
        if k > 0x1f && k < 0x22 {
            return i;
        }
        i = next(m, r);
    }
    0xffff_ffff
}

/// `find_alderman_election` (`0x004D9D10`): primer registro de tipo `0x20` por posición en la tabla, o 0.
pub fn find_alderman_election(m: &mut dyn Mem, s: u32) -> u32 {
    for i in 0..count(m, s) {
        let r = rec(m, s, i);
        if m.r16(r + 6) == 0x20 {
            return r;
        }
    }
    0
}

/// `find_hanse_diet_vote` (`0x004D9CD0`): primer registro de tipo `0x21` de la lista activa, o 0.
pub fn find_hanse_diet_vote(m: &mut dyn Mem, s: u32) -> u32 {
    find_chain(m, s, 0x21, |_, _| true).map_or(0, |(_, r)| r)
}

/// `find_council_meeting_for_town` (`0x004DA140`): índice (16 bits bajos) de la reunión del consejo (tipo `0x2E`) o `0xFFFF`.
/// La búsqueda se detiene en cuanto un vencimiento pasa de `reloj + 0x1E00`.
pub fn find_council_meeting_for_town(m: &mut dyn Mem, s: u32, town: u8) -> u16 {
    let n = count(m, s);
    let mut i = head(m, s);
    if i >= n {
        return 0xffff;
    }
    let limit = m.r32(G_TICK).wrapping_add(0x1e00);
    loop {
        let r = rec(m, s, i);
        if m.r16(r + 6) == 0x2e && m.r8(r + 0xc) == town {
            return i as u16;
        }
        if m.r32(r) >= limit {
            return 0xffff;
        }
        i = next(m, r);
        if i >= n {
            return 0xffff;
        }
    }
}

/// `touch_reminder_task` (`0x004D8FB0`): fija el vencimiento del recordatorio (tipo `0x14`) `id` al reloj actual.
pub fn touch_reminder_task(m: &mut dyn Mem, s: u32, id: u32) {
    let tick = m.r32(G_TICK);
    if let Some((_, r)) = find_chain(m, s, 0x14, |m, r| u32::from(m.r16(r + 0xc)) == id) {
        m.w32(r + 8, tick);
    }
}

/// Número de ranuras de comerciante humano anteriores al jugador local (`0x4D9D40`/`0x4DA480` comparten el recuento).
fn merchants_without_slot_before(m: &mut dyn Mem, limit: u32) -> u32 {
    let mut n = 0u32;
    for i in 0..limit {
        let mer = merchant_addr(m, i);
        if m.r16(mer + 8) == 0 {
            n += 1;
        }
    }
    n
}

/// `player_vote_bit_set` (`0x004D9D40`): el jugador local ya votó en la votación de hoy (devuelve el byte bajo).
pub fn player_vote_bit_set(m: &mut dyn Mem, s: u32) -> u8 {
    let player = m.r32(G_PLAYER);
    if player >= u32::from(m.r16(G_MERCHANTS)) {
        return 0;
    }
    let mer = merchant_addr(m, player);
    let home = u32::from(m.r8(mer + 0x19));
    if m.r8(mer + 0x39c + home) < 4 {
        return 0;
    }
    let today = m.r32(G_TICK) & 0xffff_ff00;
    let n = count(m, s);
    // El original lee el primer registro sin comprobar el límite (bucle do-while).
    let mut i = head(m, s);
    loop {
        let r = rec(m, s, i);
        if m.r32(r) & 0xffff_ff00 != today {
            return 0;
        }
        let k = m.r16(r + 6);
        if k == 0x21 || k == 0x20 {
            let mask = 1u32.wrapping_shl(merchants_without_slot_before(m, player) & 0x1f);
            let field = if k == 0x21 { 0x16 } else { 0x14 };
            return u8::from(u32::from(m.r8(r + field)) & mask == mask);
        }
        i = next(m, r);
        if i >= n {
            return 0;
        }
    }
}

/// `find_auction_check` (`0x004D9000`) sin escribir: devuelve `(índice o 0xFFFF, dato +8 o 0)`.
pub fn find_auction_check_value(m: &mut dyn Mem, s: u32, a: u32, b: u32, c: u32) -> (u32, u32) {
    let found = find_chain(m, s, 0x2d, |m, r| {
        u32::from(m.r8(r + 0x11)) == b && u32::from(m.r8(r + 0x10)) == a && u32::from(m.r16(r + 0xe)) == c
    });
    match found {
        Some((i, r)) => (i, m.r32(r + 8)),
        None => (0xffff, 0),
    }
}

/// `find_auction_check` (`0x004D9000`): tarea de comprobación de subasta (tipo `0x2D`) que coincide; escribe su dato `+8` en `out`.
pub fn find_auction_check(m: &mut dyn Mem, s: u32, a: u32, b: u32, c: u32, out: u32) -> u32 {
    let (i, v) = find_auction_check_value(m, s, a, b, c);
    m.w32(out, v);
    i
}

/// `auction_player_bit_set` (`0x004DA480`): el jugador local ya está marcado en la subasta (tipo `0x0A`/`0x36`); byte bajo.
pub fn auction_player_bit_set(m: &mut dyn Mem, s: u32, index: u32) -> u8 {
    if index > count(m, s) {
        return 0;
    }
    let r = rec(m, s, index);
    let k = m.r16(r + 6);
    if k != 0x0a && k != 0x36 {
        return 0;
    }
    let bits = u32::from(m.r8(r + 0x16));
    let player = m.r32(G_PLAYER);
    let mc = u32::from(m.r16(G_MERCHANTS));
    let mut bit = 1u32;
    for i in 0..mc {
        if i == player {
            return u8::from(bit & bits == bit);
        }
        let mer = merchant_addr(m, i);
        if m.r16(mer + 8) == 0 {
            bit = bit.wrapping_mul(2);
        }
    }
    0
}

/// `collect_pending_pirate_demands` (`0x004D9EA0`): índices (u16) de las exigencias piratas pendientes (tipo `0x30` con `+8` no nulo), hasta 8.
pub fn collect_pending_pirate_demands(m: &mut dyn Mem, s: u32, out: u32) -> u32 {
    let n = count(m, s);
    let mut i = head(m, s);
    let limit = m.r32(G_TICK).wrapping_add(0x1d00);
    let mut found = 0u32;
    if n <= i {
        return 0;
    }
    loop {
        let r = rec(m, s, i);
        if m.r16(r + 6) == 0x30 {
            if m.r32(r + 8) != 0 {
                m.w16(out.wrapping_add(found * 2), i as u16);
                found += 1;
                if found > 7 {
                    return 8;
                }
            }
        } else if limit < m.r32(r) {
            return found;
        }
        i = next(m, r);
        if i >= n {
            return found;
        }
    }
}

/// `collect_open_alderman_missions` (`0x004EC5F4`): índices de las misiones de concejal abiertas (tipo `0x32`, grado < 5, ciudad ajena a los
/// comerciantes); rellena con `0xFFFF` hasta `capacity`. Devuelve el número encontrado.
pub fn collect_open_alderman_missions(m: &mut dyn Mem, s: u32, capacity: u32, out: u32) -> u32 {
    if capacity == 0 {
        return 0;
    }
    let mut left = capacity;
    let mut found = 0u32;
    let mut at = out;
    let n = count(m, s);
    let mc = u32::from(m.r16(G_MERCHANTS));
    let mut i = head(m, s);
    while i < n {
        let r = rec(m, s, i);
        if m.r16(r + 6) == 0x32 && mc <= u32::from(m.r8(r + 0xd)) && m.r8(r + 0xc) < 5 {
            m.w16(at, i as u16);
            at = at.wrapping_add(2);
            found += 1;
            left -= 1;
            if left == 0 {
                return found;
            }
        }
        i = next(m, r);
    }
    for _ in 0..left {
        m.w16(at, 0xffff);
        at = at.wrapping_add(2);
    }
    found
}

/// `get_merchant_alderman_mission_task_index` (`0x004EC694`): índice de la misión de concejal del comerciante o `0xFFFF`.
pub fn merchant_alderman_mission_index(m: &mut dyn Mem, s: u32, merchant: u32) -> u32 {
    if merchant < u32::from(m.r16(G_MERCHANTS)) {
        let mer = merchant_addr(m, merchant);
        if m.r8(mer + 0x37) != 0 {
            if let Some((i, _)) = find_chain(m, s, 0x32, |m, r| u32::from(m.r8(r + 0xd)) == merchant) {
                return i;
            }
        }
    }
    0xffff
}

/// Llamada diferida al emisor `0x00590610` con sus siete argumentos.
pub type EmitterCall = [u32; 7];

/// `spawn_land_transport_entities_for_town` (`0x004D8F00`): por cada transporte terrestre (tipo `0x0C`, bit 0 de `+0xB`) de la ciudad,
/// pide la figura al emisor `0x00590610`. Los registros con vencimiento posterior a `reloj + 2` y que no son de tipo `0x0C` cortan la búsqueda.
pub fn spawn_land_transport_entities(m: &mut dyn Mem, s: u32, town: u32) -> Vec<EmitterCall> {
    let mut calls = Vec::new();
    let n = count(m, s);
    let mut i = head(m, s);
    let limit = m.r32(G_TICK).wrapping_add(2);
    while i < n {
        let r = rec(m, s, i);
        if m.r16(r + 6) == 0x0c {
            if m.r8(r + 0xb) & 1 != 0 {
                let which = m.r8(r + 0x15);
                let owner = if which == 1 {
                    Some(m.r8(r + 8))
                } else if which == 6 {
                    Some(m.r8(r + 9))
                } else {
                    None
                };
                if owner.is_some_and(|b| u32::from(b) == town) {
                    let flag = u32::from(m.r8(0x0067_7929));
                    calls.push([0, 0, 0x1f, i, (r & 0xffff_ff00) | flag, 0, 0]);
                }
            }
        } else if limit < m.r32(r) {
            return calls;
        }
        i = next(m, r);
    }
    calls
}
