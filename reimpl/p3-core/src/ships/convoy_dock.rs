//! Parada de ruta de un convoy en puerto, `0x00503230` (thiscall, sin argumentos): decide si el convoy puede quedarse en la parada actual de la
//! ruta del barco líder, reparte el cargamento cambiado entre los barcos y envía la carta de balance, o avanza a la siguiente parada.
//!
//! Va sobre la imagen de memoria, con el marco de pila de la función modelado a partir de `esp` (el puntero de pila a la entrada): el original
//! guarda temporales (registro de carta, copia de 24 existencias, acumuladores) que alimentan llamadas con direcciones de pila. Llamadas a
//! otras funciones por [`OpsHost`] con la dirección del original; los argumentos que el original lee como palabra se pasan sin el residuo de registro.

use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

const SHIPS: u32 = 0x006d_d7a4;
const STOPS: u32 = 0x006d_d72c;
const TOWNS: u32 = 0x006d_e508;
const SHIP_COUNT: u32 = 0x006d_d894;
const LETTERS: u32 = 0x006d_d7a0;
const LETTER_COUNT: u32 = 0x006d_d892;
const MGR: u32 = 0x006d_d728;
const WORLD: u32 = 0x006d_e4a0;
const MSGS: u32 = 0x006d_d730;
const NOTIFY: u32 = 0x006d_f2f0;

fn cp(m: &mut dyn Mem, dst: u32, src: u32, n: u32) {
    for i in 0..n {
        let b = m.r8(src.wrapping_add(i));
        m.w8(dst.wrapping_add(i), b);
    }
}
fn ship(m: &mut dyn Mem, idx: u32) -> u32 {
    let base = m.r32(SHIPS);
    base.wrapping_add(idx.wrapping_mul(0x180))
}
fn stop(m: &mut dyn Mem, idx: u32) -> u32 {
    let base = m.r32(STOPS);
    base.wrapping_add(idx.wrapping_mul(220))
}
fn count(m: &mut dyn Mem) -> u32 {
    m.r32(SHIP_COUNT) & 0xffff
}
fn town_ptr(m: &mut dyn Mem, idx: u32) -> u32 {
    let base = m.r32(TOWNS);
    base.wrapping_add(idx.wrapping_mul(0x9f8))
}

/// Avisos `0x548CA0` con la ventana `0x6DF2F0`: `(propietario, a, b, tripulación, código, líder)`.
fn notify(m: &mut dyn Mem, host: &mut dyn OpsHost, ship_ptr: u32, this: u32, a: u32, code: u32) {
    let flags = m.r8(ship_ptr + 0x136) & 0xfc;
    let f = m.r8(ship_ptr + 0x3d) | 8;
    m.w8(ship_ptr + 0x3d, f);
    let owner = u32::from(m.r8(ship_ptr));
    m.w8(ship_ptr + 0x136, flags);
    let leader = u32::from(m.r16(this + 0x10));
    let crew = u32::from(m.r8(ship_ptr + 0x39));
    host.call(m, 0x548ca0, NOTIFY, &[owner, a, a, crew, code, leader]);
}

pub fn convoy_process_route_stop(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = esp.wrapping_sub(0xb4);
    let fr = |o: u32| f.wrapping_add(o);
    m.w32(fr(0x44), this);
    let rec = fr(0x28);
    host.call(m, 0x51a850, rec, &[]);
    let leader_idx = u32::from(m.r16(this + 0x10));
    let leader = ship(m, leader_idx);
    m.w32(fr(0x18), 0x40);
    m.w32(fr(0x24), 0);
    m.w32(fr(0x10), leader);
    let cur0 = u32::from(m.r16(leader + 0x132));
    m.w32(fr(0x40), cur0);
    let p = stop(m, cur0);
    let flags = m.r8(p + 3);
    let stop_town = u16::from(m.r8(p + 2));
    let flag4 = u32::from(flags & 4 == 4);
    m.w32(fr(0x20), flag4);
    let conv_town = u32::from(m.r16(this + 0x1c));
    let owner = u32::from(m.r8(this));
    if conv_town as u16 == stop_town {
        if flags & 1 != 0 {
            let t = town_ptr(m, conv_town);
            if m.r8(t + 0x76c) & 0x40 == 0 {
                m.w8(p + 3, flags.wrapping_sub(1));
            } else {
                let mut s = u32::from(m.r16(this + 0xa));
                while s < count(m) {
                    let sp = ship(m, s);
                    let a = m.r32(sp + 0x18) as i32;
                    let b = m.r32(sp + 0x14) as i32;
                    if b.wrapping_mul(9) > a.wrapping_mul(10) {
                        m.w16(this + 0x12, 4);
                    }
                    s = u32::from(m.r16(sp + 6));
                }
            }
        }
        if m.r16(this + 0x12) == 4 {
            let t = town_ptr(m, conv_town);
            if host.call(m, 0x52acd0, t, &[leader]) as u8 != 0 {
                m.w8(leader + 0x136, 2);
                return end(m, host, rec);
            }
            m.w16(this + 0x12, 0);
        }
        let id32 = host.call(m, 0x4d4c90, MGR, &[]);
        let id = id32 & 0xffff;
        m.w32(fr(0x38), id32);
        m.w32(fr(0x1c), id);
        let newrec = stop(m, id);
        m.w32(fr(0x10), newrec);
        cp(m, newrec, p, 0xdc);
        let town_b = u32::from(m.r8(newrec + 2));
        m.w8(newrec + 3, 0);
        m.w16(newrec, id as u16);
        let r = host.call(m, 0x5308a0, WORLD, &[owner, town_b, 0]);
        m.w32(fr(0x3c), r);
        let mut s = u32::from(m.r16(this + 0xa));
        if s < count(m) {
            loop {
                let sp = ship(m, s);
                if m.r8(sp) as u32 == owner {
                    cp(m, fr(0x48), sp + 0x54, 0x60);
                    let old_cur = m.r16(sp + 0x132);
                    let letter = m.r16(sp + 0x42);
                    m.w16(sp + 0x132, id as u16);
                    m.w16(fr(0x14), old_cur);
                    if letter >= m.r16(LETTER_COUNT) {
                        let l = m.r16(leader + 0x42);
                        m.w16(sp + 0x42, l);
                    }
                    let r = m.r32(fr(0x3c));
                    host.call(m, 0x4d5200, MGR, &[sp, r]);
                    if s == u32::from(m.r16(this + 0x10)) {
                        let v = m.r16(fr(0x40));
                        m.w16(sp + 0x132, v);
                    } else {
                        let v = m.r16(fr(0x14));
                        let l2 = m.r16(sp + 0x42);
                        m.w16(sp + 0x132, v);
                        if l2 == m.r16(leader + 0x42) {
                            m.w16(sp + 0x42, 0xffff);
                        }
                    }
                    let mut e = newrec + 0x7c;
                    let mut wp = sp + 0x54;
                    m.w32(fr(0x14), wp);
                    for j in 0..0x18u32 {
                        let old = m.r32(fr(0x48 + 4 * j));
                        let cur = m.r32(wp);
                        if old != cur {
                            let flag = m.r32(e - 0x60) as i32;
                            let acc = m.r32(e);
                            if flag > 0 {
                                m.w32(e, acc.wrapping_add(cur.wrapping_sub(old)));
                            } else {
                                m.w32(e, acc.wrapping_add(old.wrapping_sub(cur)));
                            }
                        }
                        wp = wp.wrapping_add(4);
                        e = e.wrapping_add(4);
                        m.w32(fr(0x14), wp);
                    }
                    let mut eax = 0u32;
                    if m.r32(fr(0x20)) != 0 {
                        eax = m.r32(sp + 0x124);
                        let acc = m.r32(fr(0x24));
                        m.w32(fr(0x24), acc.wrapping_add(eax));
                        m.w32(sp + 0x124, 0);
                    }
                    if (m.r8(sp + 0x3f) as i8) < 2 {
                        let ax = 0x500u16.wrapping_sub(m.r16(sp + 0x3e));
                        if m.r16(fr(0x18)) < ax {
                            m.w32(fr(0x18), (eax & 0xffff_0000) | u32::from(ax));
                        }
                    }
                }
                s = u32::from(m.r16(sp + 6));
                if s >= count(m) {
                    break;
                }
            }
        }
        host.call(m, 0x4d4dd0, MGR, &[id, 0]);
        return tail(m, host, this, esp, owner);
    }
    // El convoy no está en la ciudad de la parada.
    m.w32(fr(0x20), 0);
    if host.call(m, 0x519ba0, leader, &[]) as u8 == 0 {
        notify(m, host, leader, this, 0, 0x22);
        return end(m, host, rec);
    }
    let mut s = u32::from(m.r16(this + 0xa));
    if s >= count(m) {
        return tail(m, host, this, esp, owner);
    }
    loop {
        let sp = ship(m, s);
        let mut failed = m.r8(sp) as u32 != owner;
        if !failed {
            if s != u32::from(m.r16(this + 0x10)) {
                let cur = u32::from(m.r16(sp + 0x132));
                host.call(m, 0x4d4dd0, MGR, &[cur, 0]);
                m.w16(sp + 0x132, 0xffff);
            }
            let lc = m.r16(leader + 0x132);
            m.w16(sp + 0x132, lc);
            let r = host.call(m, 0x4d5ab0, MGR, &[sp]) & 0xff;
            if s != u32::from(m.r16(this + 0x10)) {
                m.w16(sp + 0x132, 0xffff);
            }
            failed = r != 0;
        }
        if failed {
            let l = m.r32(fr(0x10));
            m.w16(l + 0x134, 1);
            m.w16(l + 0x138, 1);
            let c = u32::from(m.r16(l + 0x132));
            let sp2 = stop(m, c);
            let town = m.r8(sp2 + 2);
            m.w8(l + 0x38, town);
            m.w8(l + 0x13b, town);
            return end(m, host, rec);
        }
        s = u32::from(m.r16(sp + 6));
        if s >= count(m) {
            return tail(m, host, this, esp, owner);
        }
    }
}

/// Desde `0x0050355A`: carta de balance (si `+0x20` de la pila no es cero) y avance de la parada del líder.
fn tail(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32, owner: u32) -> u32 {
    let f = esp.wrapping_sub(0xb4);
    let fr = |o: u32| f.wrapping_add(o);
    let rec = fr(0x28);
    let leader_idx = u32::from(m.r16(this + 0x10));
    let edi = ship(m, leader_idx);
    if m.r32(fr(0x20)) != 0 {
        let letters = m.r32(LETTERS);
        let li = u32::from(m.r16(edi + 0x42));
        m.w8(fr(0x2c), 7);
        let b = m.r8(edi + 0x39);
        m.w8(fr(0x2d), b);
        let w = m.r16(edi + 0x15e) | 0x8000;
        m.w16(fr(0x30), w);
        let lrec = letters.wrapping_add(li << 4);
        let b2 = u16::from(m.r8(lrec + 2));
        let b3 = u16::from(m.r8(lrec + 3));
        m.w16(fr(0x32), (b2 << 8).wrapping_add(b3));
        let sum = m.r32(fr(0x24));
        m.w32(fr(0x34), sum);
        host.call(m, 0x4d6530, MSGS, &[owner, rec]);
    }
    let cur2 = u32::from(m.r16(edi + 0x132));
    m.w32(fr(0x1c), cur2);
    if host.call(m, 0x519ba0, edi, &[]) as u8 == 0 {
        notify(m, host, edi, this, 0, 0x22);
        return end(m, host, rec);
    }
    loop {
        let cs = u32::from(m.r16(edi + 0x132));
        let sp0 = stop(m, cs);
        let next = m.r16(sp0);
        m.w16(edi + 0x132, next);
        let cur_saved = m.r32(fr(0x1c));
        if u32::from(next) == cur_saved {
            notify(m, host, edi, this, 0, 0x25);
            return end(m, host, rec);
        }
        let mut s = u32::from(m.r16(this + 0xa));
        if s >= count(m) {
            continue;
        }
        loop {
            let sp = ship(m, s);
            let failed = if m.r8(sp) as u32 != owner {
                true
            } else {
                if s != u32::from(m.r16(this + 0x10)) {
                    let c = u32::from(m.r16(sp + 0x132));
                    host.call(m, 0x4d4dd0, MGR, &[c, 0]);
                    m.w16(sp + 0x132, 0xffff);
                }
                let c = m.r16(edi + 0x132);
                m.w16(sp + 0x132, c);
                let r = host.call(m, 0x4d5ab0, MGR, &[sp]) & 0xff;
                if s != u32::from(m.r16(this + 0x10)) {
                    m.w16(sp + 0x132, 0xffff);
                }
                r != 0
            };
            if failed {
                m.w16(this + 0x12, 1);
                m.w16(edi + 0x134, 1);
                let v = m.r16(fr(0x18));
                m.w16(edi + 0x138, v);
                let c = u32::from(m.r16(edi + 0x132));
                let sp2 = stop(m, c);
                let town = m.r8(sp2 + 2);
                m.w8(edi + 0x38, town);
                m.w8(edi + 0x13b, town);
                let leader_idx = u32::from(m.r16(this + 0x10));
                host.call(m, 0x5172a0, edi, &[u32::from(town), 0, 0, leader_idx]);
                return end(m, host, rec);
            }
            s = u32::from(m.r16(sp + 6));
            if s >= count(m) {
                break;
            }
        }
    }
}

fn end(m: &mut dyn Mem, host: &mut dyn OpsHost, rec: u32) -> u32 {
    host.call(m, 0x51a880, rec, &[])
}
