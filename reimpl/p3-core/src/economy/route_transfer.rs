//! `transfer_loaded_traderoute` (`0x005492D0`, thiscall): vuelca la ruta comercial cargada (`this+0x930`, registros de parada de 0xDC bytes,
//! `this+0x934` índice del barco) en la tabla global de paradas `0x6DD72C` del barco, avisando por la ventana `0x54AA70` de cada diferencia
//! o rechazo (códigos 0x68, 0x69, 0x6A, 0x6C, 0x6D). Sobre la imagen de memoria con el marco de pila modelado a partir de `esp` (registro de
//! aviso en `esp-0x44+0x30`); el resto de llamadas, por [`OpsHost`] con la dirección del original.

use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

const SHIPS: u32 = 0x006d_d7a4;
const PORTS: u32 = 0x006d_d7a8;
const SHIP_COUNT: u32 = 0x006d_d894;
const STOPS: u32 = 0x006d_d72c;
const STOP_COUNT: u32 = 0x006d_d72a;
const TOWN_COUNT: u32 = 0x006d_e4b0;
const TOWNS: u32 = 0x006d_e508;
const WORLD: u32 = 0x006d_e4a0;
const FREE: u32 = 0x0064_f7e2;

fn cp(m: &mut dyn Mem, dst: u32, src: u32, n: u32) {
    for i in 0..n {
        let b = m.r8(src.wrapping_add(i));
        m.w8(dst.wrapping_add(i), b);
    }
}
fn rind32(m: &mut dyn Mem, ptr: u32, off: u32) -> u32 {
    let b = m.r32(ptr);
    m.r32(b.wrapping_add(off))
}
fn rind16(m: &mut dyn Mem, ptr: u32, off: u32) -> u32 {
    let b = m.r32(ptr);
    u32::from(m.r16(b.wrapping_add(off)))
}
fn rind8(m: &mut dyn Mem, ptr: u32, off: u32) -> u32 {
    let b = m.r32(ptr);
    u32::from(m.r8(b.wrapping_add(off)))
}

pub fn transfer_loaded_traderoute(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = esp.wrapping_sub(0x44);
    let fr = |o: u32| f.wrapping_add(o);
    let rec = fr(0x30);
    host.call(m, 0x535740, rec, &[]);
    let flag_a = u32::from(m.r32(this) as u8 & 0xc != 0);
    m.w32(fr(0x28), flag_a);
    let count = m.r32(SHIP_COUNT) & 0xffff;
    m.w32(fr(0x18), 0);
    m.w32(fr(0x10), 0);
    m.w32(fr(0x1c), 0);
    let idx = m.r32(this + 0x934);
    let free_buf = |m: &mut dyn Mem, host: &mut dyn OpsHost| -> u32 {
        let p = m.r32(this + 0x930);
        let r = host.call(m, FREE, 0, &[p]);
        m.w32(this + 0x930, 0);
        r
    };
    if idx >= count {
        return free_buf(m, host);
    }
    let ships = m.r32(SHIPS);
    let mut ship = ships.wrapping_add(idx.wrapping_mul(0x180));
    m.w32(fr(0x24), ship);
    let ax = u32::from(m.r16(ship + 8));
    if (ax as u16) < (count as u16) {
        let port = m.r32(PORTS).wrapping_add(ax.wrapping_mul(60));
        let p10 = u32::from(m.r16(port + 0x10));
        let pa = u32::from(m.r16(port + 0xa));
        ship = ships.wrapping_add(p10.wrapping_mul(0x180));
        m.w32(fr(0x24), ship);
        if pa < count {
            let mut s = pa;
            loop {
                let sp = ships.wrapping_add(s.wrapping_mul(0x180));
                if m.r8(sp + 0xe) >= 2 {
                    m.w32(fr(0x1c), 1);
                    s = 0xffff;
                } else {
                    s = u32::from(m.r16(sp + 6));
                }
                if s >= count {
                    break;
                }
            }
        }
    } else {
        let v = u32::from(m.r8(ship + 0xe) >= 2);
        m.w32(fr(0x1c), v);
    }
    m.w32(fr(0x14), 0);
    m.w32(fr(0x20), 0x1c);
    let mut i = 0u32; // índice del registro (esi = i * 0xdc)
    let mut n_records = 0u32;
    loop {
        let off = i * 0xdc;
        let buf = m.r32(this + 0x930);
        let towns = m.r32(TOWN_COUNT);
        let town = u32::from(m.r8(buf + off + 2));
        let mut counted = false;
        let invalid = if town >= towns {
            true
        } else if m.r32(fr(0x1c)) != 0 {
            let tp = m.r32(TOWNS).wrapping_add(town.wrapping_mul(0x9f8));
            m.r32(tp + 0x2c8) & 1 != 0
        } else {
            false
        };
        if invalid {
            m.w8(buf + off + 2, 0xff);
        } else {
            let owner = u32::from(m.r8(ship));
            let r = host.call(m, 0x5308a0, WORLD, &[owner, town, 0]);
            if r == 0 {
                let mut o = m.r32(fr(0x20));
                for j in 0..0x18u32 {
                    let b = m.r32(this + 0x930);
                    if m.r32(b.wrapping_add(o)) == 0 {
                        let k = m.r32(fr(0x14)).wrapping_add(j);
                        let a = b.wrapping_add(k.wrapping_mul(4)).wrapping_add(0x7c);
                        if m.r32(a) != 0 {
                            m.w32(a, 0);
                        }
                    }
                    o = o.wrapping_add(4);
                }
            }
            counted = true;
        }
        let kept = m.r32(fr(0x10));
        if counted {
            m.w32(fr(0x10), kept + 1);
        }
        let bb = m.r32(this + 0x930);
        let nxt = m.r16(bb.wrapping_add(off));
        n_records = m.r32(fr(0x18)) + 1;
        let e14 = m.r32(fr(0x14)).wrapping_add(0x37);
        let e20 = m.r32(fr(0x20)).wrapping_add(0xdc);
        m.w32(fr(0x18), n_records);
        m.w32(fr(0x14), e14);
        m.w32(fr(0x20), e20);
        i += 1;
        if nxt == 0 {
            break;
        }
    }
    let kept = m.r32(fr(0x10));
    if kept == 0 {
        return free_buf(m, host);
    }
    let mut ebp_stops = 0u32;
    if m.r32(fr(0x28)) == 0 {
        // Sin bandera: libera el estado y copia las paradas válidas sobre la ruta del barco.
        m.w8(ship + 0x136, 0);
        let idx34 = m.r32(this + 0x934);
        m.w32(fr(0x34), idx34);
        let buf = m.r32(this + 0x930);
        m.w32(fr(0x30), 0x6c);
        let mut town = u32::from(m.r8(buf + 2));
        let mut e = 1u32;
        if n_records > e {
            loop {
                let towns = m.r32(TOWN_COUNT);
                if town >= towns {
                    town = u32::from(m.r8(buf.wrapping_add(e.wrapping_mul(220)).wrapping_add(2)));
                    m.w32(fr(0x38), town);
                    e += 1;
                    let nr = m.r32(fr(0x18));
                    if e >= nr {
                        break;
                    }
                    continue;
                }
                break;
            }
        }
        let _ = ebp_stops;
        m.w32(fr(0x38), town);
        m.w32(fr(0x3c), kept);
        host.call(m, 0x535760, rec, &[]);
        let cur = u32::from(m.r16(ship + 0x132));
        let nstops = u32::from(m.r16(STOP_COUNT));
        if cur < nstops && n_records > 0 {
            let mut cur = cur;
            let mut o = 0u32;
            for _ in 0..n_records {
                let buf = m.r32(this + 0x930);
                let t = u32::from(m.r8(buf + o + 2));
                if t < m.r32(TOWN_COUNT) {
                    let g = m.r32(STOPS).wrapping_add(cur.wrapping_mul(220));
                    m.w8(g + 2, t as u8);
                    let b = m.r8(buf + o + 3);
                    m.w8(g + 3, b);
                    cp(m, g + 4, buf + o + 4, 0x18);
                    cp(m, g + 0x1c, buf + o + 0x1c, 0x60);
                    cp(m, g + 0x7c, buf + o + 0x7c, 0x60);
                    cur = u32::from(m.r16(g));
                }
                o += 0xdc;
            }
        }
        return finish(m, host, this);
    }
    if m.r8(ship + 0x136) == 1 {
        let idx34 = m.r32(this + 0x934);
        m.w32(fr(0x30), 0x68);
        m.w32(fr(0x34), idx34);
        m.w32(fr(0x38), 0);
        host.call(m, 0x54aa70, this, &[rec]);
    }
    let cur = u32::from(m.r16(ship + 0x132));
    // Longitud del ciclo de paradas que contiene la actual.
    let mut chain = 0u32;
    if (cur as u16) < m.r16(STOP_COUNT) {
        chain = 1;
        let mut nxt = rind16(m, STOPS, cur * 220);
        if nxt != cur {
            loop {
                let nstops = u32::from(m.r16(STOP_COUNT));
                m.w32(fr(0x28), nstops);
                chain += 1;
                let nn = rind16(m, STOPS, nxt.wrapping_mul(220));
                nxt = nn;
                if nxt >= nstops || chain > kept {
                    chain = 0;
                    break;
                }
                if nxt == cur {
                    break;
                }
            }
        }
    }
    if kept != chain {
        let idx34 = m.r32(this + 0x934);
        let buf = m.r32(this + 0x930);
        m.w32(fr(0x34), idx34);
        m.w32(fr(0x30), 0x6c);
        let t = u32::from(m.r8(buf + 2));
        m.w32(fr(0x38), t);
        m.w32(fr(0x3c), kept);
        return host.call(m, 0x54aa70, this, &[rec]);
    }
    let cur = cur & 0xffff;
    m.w32(fr(0x28), 0);
    if n_records == 0 {
        return finish(m, host, this);
    }
    m.w32(fr(0x10), 0);
    m.w32(fr(0x14), 7);
    let mut cur = cur;
    let mut done = 0u32;
    loop {
        let off = m.r32(fr(0x10));
        let buf = m.r32(this + 0x930);
        let town = u32::from(m.r8(buf + off + 2));
        let towns = m.r32(TOWN_COUNT);
        if town < towns {
            let g = cur.wrapping_mul(220);
            m.w32(fr(0x20), g);
            let stops = m.r32(STOPS);
            if u32::from(m.r8(stops.wrapping_add(g).wrapping_add(2))) != town {
                let owner_town = u32::from(m.r8(buf + off + 2));
                m.w32(fr(0x30), 0x6a);
                m.w32(fr(0x34), cur);
                m.w32(fr(0x38), 0);
                m.w32(fr(0x3c), owner_town);
                let idx34 = m.r32(this + 0x934);
                m.w32(fr(0x40), idx34);
                let w56 = u32::from(m.r16(this + 0x56));
                if w56 == 0x34 {
                    return 0x34;
                }
                host.call(m, 0x54aa70, this, &[rec]);
            }
            let stops = m.r32(STOPS);
            let g = m.r32(fr(0x20));
            let gflags = m.r8(stops.wrapping_add(g).wrapping_add(3));
            let buf = m.r32(this + 0x930);
            let off = m.r32(fr(0x10));
            let lflags = m.r8(buf.wrapping_add(off).wrapping_add(3));
            if gflags != lflags {
                m.w32(fr(0x30), 0x6d);
                m.w32(fr(0x34), cur);
                m.w32(fr(0x38), u32::from(lflags));
                let w56 = u32::from(m.r16(this + 0x56));
                if w56 == 0x34 {
                    return 0x34;
                }
                host.call(m, 0x54aa70, this, &[rec]);
            }
            m.w32(fr(0x1c), 0);
            for j in 0..0x18u32 {
                let buf = m.r32(this + 0x930);
                let off = m.r32(fr(0x10));
                let w = u32::from(m.r8(buf.wrapping_add(off).wrapping_add(j).wrapping_add(4)));
                let g = m.r32(fr(0x20));
                let stops = m.r32(STOPS);
                let gb = m.r8(stops.wrapping_add(g).wrapping_add(j).wrapping_add(4));
                let mut ok = false;
                if u32::from(gb) == w {
                    let e14 = m.r32(fr(0x14));
                    let c55 = cur.wrapping_mul(55);
                    let gv = m.r32(stops.wrapping_add(c55.wrapping_add(w).wrapping_mul(4)).wrapping_add(0x7c));
                    let lv = m.r32(buf.wrapping_add(e14.wrapping_add(w).wrapping_mul(4)).wrapping_add(0x60));
                    if gv == lv {
                        m.w32(fr(0x2c), c55);
                        let gv2 = m.r32(stops.wrapping_add(c55.wrapping_add(w).wrapping_mul(4)).wrapping_add(0x1c));
                        let lv2 = m.r32(buf.wrapping_add(e14.wrapping_add(w).wrapping_mul(4)));
                        ok = gv2 == lv2;
                    } else {
                        m.w32(fr(0x2c), c55);
                    }
                }
                if !ok {
                    let e14 = m.r32(fr(0x14));
                    let sp24 = m.r32(fr(0x24));
                    let owner = u32::from(m.r8(sp24));
                    m.w32(fr(0x30), 0x69);
                    m.w16(fr(0x34), cur as u16);
                    m.w16(fr(0x36), owner as u16);
                    m.w16(fr(0x38), w as u16);
                    m.w16(fr(0x3a), j as u16);
                    let lv = m.r32(buf.wrapping_add(e14.wrapping_add(w).wrapping_mul(4)).wrapping_add(0x60));
                    m.w32(fr(0x3c), lv);
                    let lv2 = m.r32(buf.wrapping_add(e14.wrapping_add(w).wrapping_mul(4)));
                    m.w32(fr(0x40), lv2);
                    let w56 = u32::from(m.r16(this + 0x56));
                    if w56 == 0x34 {
                        return lv2;
                    }
                    host.call(m, 0x54aa70, this, &[rec]);
                }
                m.w32(fr(0x1c), j + 1);
            }
            let g = m.r32(fr(0x20));
            cur = rind16(m, STOPS, g);
        }
        done = m.r32(fr(0x28)) + 1;
        let off = m.r32(fr(0x10)) + 0xdc;
        let e14 = m.r32(fr(0x14)) + 0x37;
        m.w32(fr(0x28), done);
        m.w32(fr(0x10), off);
        m.w32(fr(0x14), e14);
        if done >= m.r32(fr(0x18)) {
            break;
        }
    }
    let _ = done;
    finish(m, host, this)
}

fn finish(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let p = m.r32(this + 0x930);
    let r = host.call(m, FREE, 0, &[p]);
    m.w32(this + 0x930, 0);
    m.w32(this + 0x934, 0xffff);
    r
}
