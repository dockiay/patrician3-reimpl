//! Coste de reparación de un barco (`0x004DA860`) e información de su subasta (`0x004DAA50`), sobre la imagen de memoria del original.
//! Las 20 constantes de coste (`0x0066DECC..0x0066E08C`) se leen de la memoria, como hace el original.
use crate::flatmem::Mem;
use crate::numeric::ftol_i32;

const SHIP_TABLE_PTR: u32 = 0x006d_d7a4;

/// Por tipo de barco: (multiplicador del nivel, umbral bajo, umbral alto, base baja, base media, base alta, multiplicador del coste) en
/// direcciones de constantes. Tipos 0, 1, 2 y el resto.
struct Kind {
    level_k: u32,
    lo: i32,
    hi: i32,
    below: u32,
    mid: u32,
    above: u32,
    mult: u32,
}
fn kind(ty: u8) -> Kind {
    match ty {
        0 => Kind { level_k: 0x66e030, lo: 0x13, hi: 0x16, below: 0x66decc, mid: 0x66deec, above: 0x66df0c, mult: 0x66e044 },
        1 => Kind { level_k: 0x66e048, lo: 0x1f, hi: 0x21, below: 0x66df2c, mid: 0x66df4c, above: 0x66df6c, mult: 0x66e05c },
        2 => Kind { level_k: 0x66e060, lo: 0x30, hi: 0x33, below: 0x66df8c, mid: 0x66dfac, above: 0x66dfcc, mult: 0x66e074 },
        _ => Kind { level_k: 0x66e078, lo: 0x3b, hi: 0x40, below: 0x66dfec, mid: 0x66e00c, above: 0x66e02c, mult: 0x66e08c },
    }
}

/// `0x004DA860`: coste de reparación del barco `ship`, redondeado a centenas (división truncada).
pub fn ship_repair_cost(m: &mut dyn Mem, ship: u32) -> u32 {
    let s = m.r32(SHIP_TABLE_PTR).wrapping_add(ship.wrapping_mul(0x180));
    let ty = m.r8(s + 0xe);
    let level = u32::from(m.r8(s + 0xf));
    let value = m.r32(s + 0x10) as i32;
    let k = kind(ty);
    let level_k = m.r32(k.level_k);
    let score = (value / 2000).wrapping_add((level.wrapping_mul(level_k)) as i32);
    let base = if score < k.lo {
        m.r32(k.below)
    } else if score <= k.hi {
        m.r32(k.mid)
    } else {
        m.r32(k.above)
    };
    let extra = level.wrapping_mul(m.r32(k.mult));
    let d = i64::from((m.r32(s + 0x14) as i32).wrapping_sub(m.r32(s + 0x18) as i32));
    // `cdq` extiende el signo de la resta de 32 bits.
    let q = (d.wrapping_mul(300)) / 2800;
    let cost = base.wrapping_add(extra).wrapping_sub(q as u32) as i32;
    let f = f64::from(cost as f32) * 0.7 + 0.5;
    let t = ftol_i32(f);
    ((t / 100).wrapping_mul(100)) as u32
}

/// `0x004DAA50`: devuelve `(coste de reparación, vencimiento de la subasta o 0xFFFFFFFF)` del barco `ship` según el planificador `sched`.
pub fn ship_auction_info(m: &mut dyn Mem, sched: u32, ship: u32) -> (u32, u32) {
    let cost = ship_repair_cost(m, ship);
    let mut due = 0xffff_ffffu32;
    let count = u32::from(m.r16(sched + 0xc));
    let mut idx = u32::from(m.r16(sched + 8));
    while idx < count {
        let rec = m.r32(sched).wrapping_add(idx * 0x18);
        if m.r16(rec + 6) == 0xa {
            for k in 0..3 {
                if u32::from(m.r16(rec + 8 + 2 * k)) == ship {
                    due = m.r32(rec);
                    let state = m.r8(rec + 0x14);
                    if state <= 2 {
                        due = due.wrapping_add(0x100);
                        if state == 1 {
                            due = due.wrapping_add(u32::from(m.r16(rec + 0xe)));
                        }
                    }
                    return (cost, due);
                }
            }
        }
        idx = u32::from(m.r16(rec + 4));
    }
    (cost, due)
}
