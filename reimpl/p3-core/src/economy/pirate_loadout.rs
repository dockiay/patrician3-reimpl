//! `ship_reset_loadout_for_pirate` (0x00515750): deja un barco pirata como nuevo (tripulación, carga y capacidades).
//! Traducción 1:1 sobre la memoria del original ([`Mem`]).
use crate::flatmem::Mem;

const SHIPS: u32 = 0x6DD7A4;
const SHIP_COUNT: u32 = 0x6DD894;

pub trait LoadoutHost {
    /// `0x00517770` sobre el barco con un argumento (siempre 4).
    fn ship_517770(&mut self, ship: u32, arg: u32);
    /// `0x005181D0` sobre el barco.
    fn ship_5181d0(&mut self, ship: u32);
    /// `0x005182B0` sobre el barco; devuelve EAX (se llama dos veces: al final es una llamada de cola).
    fn ship_5182b0(&mut self, ship: u32) -> u32;
    /// `0x0052BB00` sobre la ciudad (ECX = ciudad).
    fn town_52bb00(&mut self, town: u32) -> u32;
    /// `0x0051A4E0` sobre el barco: (valor, resultado de 0x52BB00).
    fn ship_51a4e0(&mut self, ship: u32, a: u32, b: u32);
    /// `0x005185C0` sobre el barco; devuelve EAX.
    fn ship_5185c0(&mut self, ship: u32) -> u32;
}

fn fill(m: &mut dyn Mem, dst: u32, v: u8, n: u32) {
    for i in 0..n {
        m.w8(dst + i, v);
    }
}

/// `arg0` = índice del barco; `value` = puntero a un entero con la tripulación (-1 = media de la flota del jugador).
/// El original termina con una llamada de cola a `0x5182B0`; el valor devuelto es el de esa llamada.
pub fn ship_reset_loadout_for_pirate(m: &mut dyn Mem, host: &mut dyn LoadoutHost, arg0: u32, value: u32) -> u32 {
    if m.r32(value) == 0xffff_ffff {
        let n = u32::from(m.r16(0x6DE4AA));
        let (mut sum, mut cnt) = (0i32, 0i32);
        let mut stored = false;
        if (n as i32) > 0 {
            let mut esi = n;
            loop {
                esi -= 1;
                let mer = m.r32(0x6DE4A0 + 0x78).wrapping_add(esi.wrapping_mul(0x650));
                let mut cx = u32::from(m.r16(mer + 0xe));
                let edx = m.r32(SHIP_COUNT) & 0xffff;
                if (cx as i32) < edx as i32 {
                    loop {
                        let ship = m.r32(SHIPS).wrapping_add(cx.wrapping_mul(0x180));
                        cx = u32::from(m.r16(ship + 4));
                        let v = m.r32(ship + 0x120) as i32;
                        if v > 0 {
                            sum = sum.wrapping_add(v);
                            cnt += 1;
                        }
                        if (cx as i32) >= edx as i32 {
                            break;
                        }
                    }
                }
                if (esi as i32) <= 0 {
                    break;
                }
            }
            if cnt > 0 {
                m.w32(value, sum.wrapping_div(cnt) as u32);
                stored = true;
            }
        }
        if !stored {
            m.w32(value, 0);
        }
    }
    if arg0 >= (m.r32(SHIP_COUNT) & 0xffff) {
        return 0;
    }
    let ship = m.r32(SHIPS).wrapping_add(arg0.wrapping_mul(0x180));
    m.w16(ship + 0x154, 0);
    fill(m, ship + 0x54, 0, 0x60);
    fill(m, ship + 0xb4, 0, 0x60);
    m.w8(ship + 0x156, 0);
    host.ship_517770(ship, 4);
    let al = m.r8(ship + 0x157);
    m.w8(ship + 0x157, if al > 1 { al - 1 } else { 0 });
    host.ship_517770(ship, 4);
    fill(m, ship + 0x13c, 0xff, 0x18);
    host.ship_5181d0(ship);
    let edi = host.ship_5182b0(ship) as i32;
    let g = m.r32(0x6CC3E8);
    let diff = u32::from(m.r8(g + 0x12));
    let v = (diff + 4).wrapping_mul(m.r32(value)) >> 2;
    m.w32(value, v);
    let town_no = u32::from(m.r8(0x6DE52E));
    let town = m.r32(0x6DE508).wrapping_add(town_no.wrapping_mul(0x9f8));
    let t = host.town_52bb00(town);
    let a = m.r32(value);
    host.ship_51a4e0(ship, a, t);
    let eax = host.ship_5185c0(ship);
    m.w16(ship + 0x40, eax as u16);
    let cx = m.r16(ship + 0x42);
    let g2 = m.r32(0x6CC3E8);
    let diff = u32::from(m.r8(g2 + 0x12));
    let mult = if cx < m.r16(0x6DD892) {
        match diff {
            0 => 70u32,
            1 => 85,
            _ => 100,
        }
    } else {
        match diff {
            0 => 50u32,
            1 => 70,
            _ => 90,
        }
    };
    m.w16(ship + 0x40, eax.wrapping_mul(mult) as u16);
    let cx = u32::from(m.r16(ship + 0x40).wrapping_add(0x63));
    let prod = i64::from(cx) * 0x51eb_851f;
    let mut q = (prod >> 32) as i32 >> 5;
    q = q.wrapping_add(((q as u32) >> 31) as i32);
    m.w16(ship + 0x40, q as u16);
    let lim = ((q as u32) & 0xffff).wrapping_mul(10) as i32;
    let mut out = q;
    if edi <= lim {
        let p = i64::from(edi) * 0x6666_6667;
        let mut d = (p >> 32) as i32 >> 2;
        d = d.wrapping_add(((d as u32) >> 31) as i32);
        out = d;
    }
    m.w16(ship + 0x154, out as u16);
    host.ship_5182b0(ship)
}
