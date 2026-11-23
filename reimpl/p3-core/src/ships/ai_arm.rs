//! `0x0051A070`: el barco de IA atracado monta cañones del arsenal de la ciudad, recoge un capitán libre del puerto y completa
//! sus espadas con las existencias de armas (mercancía 20). Con `0x005269A0` (buscar capitán) y `0x00526AA0` (sacarlo de la lista).

use super::cargo::recompute_load;
use super::remove::{SHIP_SIZE, TOWN_SIZE};
use super::tick::TickWorld;
use super::{r16, r32, w16, w32};

/// Existencias de cañones que consume cada pieza por tipo, `0x00672CB4` (la ciudad guarda las existencias en `+0x124+4·k`).
pub const GUN_UNIT: [i32; 6] = [1000, 1000, 2000, 2000, 2000, 1000];

/// `0x005269A0`: capitán libre de la ciudad: primero el de `+0xF == owner`, luego el de `+0xF == 0xFF`; `0xFFFF` si no hay.
pub fn find_captain(w: &TickWorld, town: usize, owner: u16) -> u16 {
    let capacity = usize::from(r16(w.remove.manager, 0xF2));
    let head = usize::from(r16(w.remove.towns, town * TOWN_SIZE + 0x82E));
    for pass in 0..2 {
        let mut i = head;
        let mut guard = 0;
        while i < capacity && guard <= capacity + 1 {
            let e = i * 16;
            if w.remove.captains[e + 8] <= 0x20 {
                let tag = w.remove.captains[e + 0xF];
                if (pass == 0 && u16::from(tag) == owner) || (pass == 1 && tag == 0xFF) {
                    return i as u16;
                }
            }
            i = usize::from(r16(w.remove.captains, e));
            guard += 1;
        }
    }
    0xFFFF
}

/// `0x00526AA0`: quita el capitán `index` de la lista de la ciudad; `false` si no estaba.
pub fn unlink_captain(w: &mut TickWorld, town: usize, index: u16) -> bool {
    let capacity = usize::from(r16(w.remove.manager, 0xF2));
    let head = town * TOWN_SIZE + 0x82E;
    if usize::from(r16(w.remove.towns, head)) >= capacity {
        return false;
    }
    // `prev` es el campo `next` que apunta al capitán actual: la cabeza de la ciudad (None) o el siguiente de otro capitán.
    let mut prev: Option<usize> = None;
    let mut guard = 0;
    loop {
        let cur = usize::from(match prev {
            None => r16(w.remove.towns, head),
            Some(e) => r16(w.remove.captains, e),
        });
        let e = cur * 16;
        if cur == usize::from(index) {
            let next = r16(w.remove.captains, e);
            match prev {
                None => w16(w.remove.towns, head, next),
                Some(p) => w16(w.remove.captains, p, next),
            }
            w16(w.remove.captains, e, 0xFFFF);
            return true;
        }
        if usize::from(r16(w.remove.captains, e)) >= capacity {
            return false;
        }
        prev = Some(e);
        guard += 1;
        if guard > capacity + 1 {
            return false;
        }
    }
}

pub fn arm_and_crew(w: &mut TickWorld, ship: u16, town: u32) {
    let o = ship as usize * SHIP_SIZE;
    if w.remove.ships[o + 0xE] == 0 {
        w.remove.ships[o + 0x136] &= 0xEF;
        return;
    }
    let t = town as usize * TOWN_SIZE;
    let (mut pair, mut single_pair, mut single) = (0usize, 0usize, 0usize);
    'guns: for k in (0..6usize).rev() {
        let unit = GUN_UNIT[k];
        let slot = t + 0x124 + 4 * k;
        if r32(w.remove.towns, slot) < unit {
            continue;
        }
        let mut take = |w: &mut TickWorld| -> bool {
            let v = r32(w.remove.towns, slot).wrapping_sub(unit);
            w32(w.remove.towns, slot, v);
            v < unit
        };
        if unit == 2000 {
            while pair < 24 {
                if w.remove.ships[o + 0x13C + pair] == 0xFF && w.remove.ships[o + 0x13D + pair] == 0xFF {
                    w.remove.ships[o + 0x13C + pair] = k as u8;
                    w.remove.ships[o + 0x13D + pair] = 6;
                    if take(w) {
                        continue 'guns;
                    }
                }
                pair += 2;
            }
            continue;
        }
        let mut exhausted = false;
        while single_pair < 24 {
            if w.remove.ships[o + 0x13C + single_pair] == 0xFF && w.remove.ships[o + 0x13D + single_pair] == 7 {
                w.remove.ships[o + 0x13C + single_pair] = k as u8;
                if take(w) {
                    exhausted = true;
                    break;
                }
            }
            single_pair += 2;
        }
        let _ = exhausted;
        while single < 24 {
            if w.remove.ships[o + 0x13C + single] == 0xFF {
                w.remove.ships[o + 0x13C + single] = k as u8;
                if take(w) {
                    continue 'guns;
                }
            }
            single += 1;
        }
    }
    let capacity = r16(w.remove.manager, 0xF2);
    if r16(w.remove.ships, o + 0x42) >= capacity {
        let here = usize::from(w.remove.ships[o + 0x39]);
        let n = find_captain(w, here, w.remove.merchant_count);
        w16(w.remove.ships, o + 0x42, n);
        if n < capacity {
            unlink_captain(w, here, n);
        }
    }
    let blades = r16(w.remove.ships, o + 0x154);
    let crew = r16(w.remove.ships, o + 0x40);
    if blades < crew {
        let here = usize::from(w.remove.ships[o + 0x39]) * TOWN_SIZE;
        let need = (i32::from(crew) - i32::from(blades)) * 10;
        let stock = r32(w.remove.towns, here + 0x54);
        if stock >= need {
            w32(w.remove.towns, here + 0x54, stock - need);
            w16(w.remove.ships, o + 0x154, crew);
        } else {
            w16(
                w.remove.ships,
                o + 0x154,
                (stock / 10).wrapping_add(i32::from(blades)) as u16,
            );
            w32(w.remove.towns, here + 0x54, stock % 10);
        }
    }
    recompute_load(&mut w.remove.ships[o..o + SHIP_SIZE]);
}
