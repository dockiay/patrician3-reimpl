//! Vencimiento de un préstamo concedido a un solicitante (`op_resolve_loan_due`, `0x0053EEC0`): si el pago ya estaba resuelto solo suma reputación; si no, aplica la
//! penalización, puede crear una banda pirata cobradora y embargar mercancías del deudor, y envía la carta al prestamista.
//! Todo lo que toca el original fuera de este módulo (cartas, tablas de textos, creación de la banda, `0x5182B0`…) pasa por [`LoanDueHost`]; los argumentos de los
//! textos se registran como palabras sin resolver. Las tablas numéricas son datos del ejecutable. No hay prueba en partida.
use crate::{ai::reputation::Merchant, numeric::ftol_i32, scheduler::Scheduler};

pub const PIRATE_BASE: u32 = 0x5a00_0000;
/// Mercancías embargadas por fila `(sentido >> 1) & 3`: tres índices por fila (`0x00673B38`).
const GOODS: [u8; 12] = [0, 2, 11, 3, 1, 14, 7, 9, 8, 6, 9, 5];
/// Marca por mercancía (`0x00672C14`): 0 = unidad de 2000, otra = unidad de 200.
const FLAG: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];
/// `0x00673660`: valor de 16 bits de la banda según el byte `+0xE`.
const BAND_TAB: [u8; 32] = [
    5, 8, 10, 12, 10, 16, 30, 24, 6, 6, 5, 5, 123, 2, 181, 2, 8, 2, 66, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 192,
];
/// `0x00673A18`: valor por unidad de cada mercancía (f32).
pub const WARE_VALUE: [u32; 24] = [
    0x3d6147af, 0x3ef50482, 0x3e6154ca, 0x3e322d0e, 0x3e11eb85, 0x3f0ccccd, 0x3fb33333, 0x3f8ccccd, 0x3f845a1d,
    0x40587ae1, 0x3ed33333, 0x3ce147af, 0x3fa39581, 0x3f8f5c29, 0x3ee147af, 0x3e8e5604, 0x3ee147af, 0x3e6147af,
    0x3f5ae147, 0x3d236e30, 0x41200000, 0x41000000, 0x41400000, 0x41600000,
];

pub trait LoanDueHost {
    /// Lectura de una palabra de las tablas de punteros/textos del ejecutable (`0x6A0628`, `0x6DDA00`, `0x6C0BB0`, …).
    fn mem32(&mut self, addr: u32) -> u32;
    /// `0x00512AE0` y `0x00512A60` (ecx = `0x6DDAA0`): nombre por índice.
    fn name_a(&mut self, v: u32) -> u32;
    fn name_b(&mut self, v: u32) -> u32;
    /// Una llamada a `sprintf` (`0x0064D410`): sitio de la llamada y palabras tras el búfer.
    fn text(&mut self, site: u32, args: &[u32]);
    /// `0x00507D60(0x6DD7A0, ciudad, 1)`: devuelve los bytes de la banda creada (al menos 0x140).
    fn spawn_pirate(&mut self, town: u16) -> Vec<u8>;
    /// `0x00509A50`
    fn register_pirate(&mut self);
    /// `0x0052A8D0` sobre la ciudad con la banda y la marca.
    fn town_notify(&mut self, town: u8, big: bool);
    /// `0x005181D0`
    fn pirate_f5181d0(&mut self);
    /// `0x005182B0`
    fn pirate_f5182b0(&mut self) -> i32;
    /// `0x005308A0(0x6DE4A0, prestamista, ciudad, 0)`: almacén del deudor en la ciudad, si existe (bytes).
    fn other(&mut self, lender: u8, town: u8) -> Option<Vec<u8>>;
    /// `0x005007A0(otro, mercancía, cantidad, valor)`
    fn f5007a0(&mut self, k: u32, qty: i32, val: i32);
    /// `0x004D6530(0x6DD730, prestamista, carta)`
    fn send_letter(&mut self, lender: u8, town: u8);
}

pub struct Outcome {
    pub pirate: Option<Vec<u8>>,
    pub other: Option<Vec<u8>>,
}

fn r32(b: &[u8], o: usize) -> i32 {
    i32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn w32(b: &mut [u8], o: usize, v: i32) {
    b[o..o + 4].copy_from_slice(&v.to_le_bytes());
}
fn wf(x: f64) -> f64 {
    f64::from(x as f32)
}
fn unit(flag: u8) -> i32 {
    if flag != 0 {
        200
    } else {
        2000
    }
}
/// Redondeo a la baja a la unidad de la mercancía con el mismo `imul` mágico que el original.
fn floor_unit(cap: i32, flag: u8) -> i32 {
    let (magic, shift, mul) = if flag == 0 {
        (0x1062_4DD3i64, 7, 2000i32)
    } else {
        (0x51EB_851Fi64, 6, 200)
    };
    let q = ((i64::from(cap) * magic) >> 32) as i32 >> shift;
    q.wrapping_add(((q as u32) >> 31) as i32).wrapping_mul(mul)
}

/// `op`: bytes de la operación (`+4` tarea, `+8` carta, `+0xC` indicador). `letters`: entradas de 16 bytes de `0x6DD730` y `letter_count` (`0x6DD736`).
#[allow(clippy::too_many_arguments)]
pub fn resolve(
    op: &[u8],
    s: &mut Scheduler,
    merchants: &mut [Merchant],
    letters: &mut [u8],
    letter_count: u16,
    tick: u32,
    day: u32,
    host: &mut dyn LoanDueHost,
) -> Outcome {
    let mut out = Outcome {
        pirate: None,
        other: None,
    };
    let letter = r32(op, 8) as u32;
    if letter < u32::from(letter_count) {
        let e = (letter as usize) << 4;
        if letters[e + 4] == 0x79 {
            letters[e + 0xa] = 0xff;
            letters[e + 0xb] = 0xff;
        }
    }
    let task_idx = r32(op, 4) as u32;
    if task_idx >= u32::from(s.capacity) {
        return out;
    }
    let t = s.records[task_idx as usize];
    if t.kind != 1 || t.u8_at(0x12) != 0 {
        return out;
    }
    let mc = merchants.len();
    if usize::from(t.u8_at(0x11)) < mc || usize::from(t.u8_at(0x10)) >= mc {
        return out;
    }
    let lender = t.u8_at(0x10);
    let town = t.u8_at(0x14);
    let finish = |s: &mut Scheduler| s.records[task_idx as usize].set_u8_at(0x10, 0xff);
    if r32(op, 0xc) != 0 {
        merchants[usize::from(lender)].add_social_reputation_goods_value(t.u32_at(8) as i32, u32::from(town));
        finish(s);
        return out;
    }
    let m = &mut merchants[usize::from(lender)];
    let town_name = host.mem32(0x6dda00 + 4 * u32::from(town));
    // Texto del aviso al prestamista (nombres, epíteto y rango).
    let female = m.bytes[0x1a] & 1;
    let (na, nb) = {
        let a = host.name_a(u32::from(m.bytes[0x1d]));
        let b = host.name_b(u32::from(m.bytes[0x1c]));
        (a, b)
    };
    let rank = m.bytes[0x39c + usize::from(m.bytes[0x19])];
    let epithet = m.get_merchant_epithet_index();
    let (t1, t2, t3) = if female != 0 {
        (0x6a0628u32, 0x6a0c54u32, 0x6b19fcu32)
    } else {
        (0x6a0608, 0x6a0c18, 0x6b1a00)
    };
    let p1 = host.mem32(t1 + 4 * u32::from(rank));
    let p2 = host.mem32(t2 + 4 * u32::from(epithet));
    let fmt = host.mem32(t3);
    host.text(0x53f0b6, &[fmt, p2, p1, nb, na]);
    let nc = host.name_a(u32::from(t.u8_at(0x17)));
    let nd = host.name_b(u32::from(t.u8_at(0x16)));
    let fmt2 = host.mem32(0x6b1bc0);
    let p3 = host.mem32(0x6a0628 + 4 * u32::from(t.u8_at(0x15)));
    host.text(0x53f119, &[fmt2, p3, nd, nc, town_name]);
    m.add_social_reputation_goods_value((t.u32_at(8) as i32).wrapping_neg(), u32::from(town));
    let mut amt = ((0x100 - i32::from(t.u8_at(0xc))).wrapping_mul(t.u32_at(8) as i32)) >> 8;
    let set_amt = |s: &mut Scheduler, v: i32| s.records[task_idx as usize].set_u32_at(8, v as u32);
    set_amt(s, amt);
    let mut pirate: Option<Vec<u8>> = None;
    if amt > 0x3a98 {
        let mut p = host.spawn_pirate(u16::from(town));
        host.register_pirate();
        let big = amt > 0x4e20;
        host.town_notify(town, big);
        let f = host.mem32(if big { 0x6b1bcc } else { 0x6b1bc8 });
        host.text(0x53f201, &[f, PIRATE_BASE + 0x160, town_name]);
        host.pirate_f5181d0();
        host.pirate_f5182b0();
        p[0] = lender;
        let old = [m.bytes[0xe], m.bytes[0xf]];
        p[4] = old[0];
        p[5] = old[1];
        p[0x3d] |= 4;
        m.bytes[0xe] = 0xff;
        m.bytes[0xf] = 0xff;
        p[0x13a] = town;
        let c = r32(&p, 0x14) >> 5;
        w32(&mut p, 0x18, c);
        w32(&mut p, 0x18, ((tick & 0x1f) as i32).wrapping_mul(c));
        p[0x134] = 3;
        p[0x135] = 0;
        let v = u16::from(BAND_TAB[usize::from(p[0xe])]);
        p[0x40..0x42].copy_from_slice(&v.to_le_bytes());
        host.pirate_f5182b0();
        let mut ax = ((tick & 0xff) as u16).wrapping_add(0xffff).wrapping_add(0x2ee);
        if (ax as i16) > 0x700 {
            ax = 0x700;
        }
        p[0x3e..0x40].copy_from_slice(&ax.to_le_bytes());
        amt -= if big { 20000 } else { 15000 };
        set_amt(s, amt);
        pirate = Some(p);
    } else {
        let f = host.mem32(0x6b1bc4);
        host.text(0x53f2e4, &[f, town_name]);
    }
    let mut other = host.other(lender, town);
    if pirate.is_some() || other.is_some() {
        let row = usize::from((t.u8_at(0x15) >> 1) & 3) * 3;
        let d = (i32::from(town) + (day & 0xffff) as i32) % 3;
        let d = d as usize;
        let k2 = if d == 2 { GOODS[row] } else { GOODS[row + d + 1] };
        let k1 = GOODS[row + d];
        let (k1, k2) = (usize::from(k1), usize::from(k2));
        let half = amt.wrapping_sub(amt >> 31) >> 1;
        let c91c = 200.0f64;
        let c918 = 2000.0f64;
        let div = |num: i32, k: usize| -> i32 {
            let d = f64::from(f32::from_bits(WARE_VALUE[k])) * if FLAG[k] != 0 { c91c } else { c918 };
            ftol_i32(wf(f64::from(num)) / d)
        };
        let q1 = div(half, k1);
        let ch = |k: usize| if FLAG[k] != 0 { 0x42u32 } else { 0x4c };
        if pirate.is_none() {
            let nm = host.mem32(0x6c0bb0 + 4 * k1 as u32);
            host.text(0x53f401, &[0x6c36d4, q1 as u32, ch(k1), nm]);
        }
        let mut ebx = unit(FLAG[k1]).wrapping_mul(q1);
        if let Some(p) = pirate.as_mut() {
            let cap = r32(p, 0x10);
            if ebx > cap {
                ebx = floor_unit(cap, FLAG[k1]);
            }
            let cur = r32(p, 0x54 + 4 * k1);
            if cur == 0 {
                w32(p, 0xb4 + 4 * k1, WARE_VALUE[k1] as i32);
            }
            w32(p, 0x54 + 4 * k1, cur.wrapping_add(ebx));
        } else {
            let val = ftol_i32(wf(f64::from(ebx)) * f64::from(f32::from_bits(WARE_VALUE[k1])));
            host.f5007a0(k1 as u32, ebx, val);
        }
        let val = ftol_i32(wf(f64::from(ebx)) * f64::from(f32::from_bits(WARE_VALUE[k1])));
        amt = amt.wrapping_sub(val);
        set_amt(s, amt);
        let q2 = div(amt, k2);
        if pirate.is_none() {
            let nm = host.mem32(0x6c0bb0 + 4 * k2 as u32);
            host.text(0x53f581, &[0x6c36e0, q2 as u32, ch(k2), nm]);
        }
        let mut ebx = unit(FLAG[k2]).wrapping_mul(q2);
        if let Some(p) = pirate.as_mut() {
            let edx = host.pirate_f5182b0();
            if ebx > edx {
                ebx = floor_unit(edx, FLAG[k2]);
            }
            let cur = r32(p, 0x54 + 4 * k2);
            w32(p, 0x54 + 4 * k2, cur.wrapping_add(ebx));
            host.pirate_f5182b0();
        } else if let Some(o) = other.as_mut() {
            let cur = r32(o, 4 + 4 * k2);
            w32(o, 4 + 4 * k2, cur.wrapping_add(ebx));
        }
        let val2 = ftol_i32(wf(f64::from(ebx)) * f64::from(f32::from_bits(WARE_VALUE[k2])));
        amt = amt.wrapping_sub(val2);
        set_amt(s, amt);
    }
    let amt = s.records[task_idx as usize].u32_at(8) as i32;
    if amt > 0 {
        m.set_i32(0, m.i32(0).wrapping_add(amt));
        m.set_i32(0x4b8, m.i32(0x4b8).wrapping_add(amt));
    }
    let f = host.mem32(0x6b1bd0);
    host.text(0x53f699, &[f, town_name]);
    host.send_letter(lender, town);
    finish(s);
    out.pirate = pirate;
    out.other = other;
    out
}
