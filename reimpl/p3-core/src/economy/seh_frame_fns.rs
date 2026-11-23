//! Funciones con marco SEH o con locales de pila pasados por dirección, portadas con [`crate::frame::Frame`]: el porte recibe el ESP de
//! entrada del original y coloca los locales en las mismas direcciones (oráculo `oracle/x86_seh_frame.py`, prueba `tests/x86_seh_frame.rs`).
use super::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::frame::{peek, Frame};

const WORLD: u32 = 0x6de4a0;
const TOWN_COUNT: u32 = 0x6de4aa;
const TICK: u32 = 0x6de4b4;
const SCHED: u32 = 0x6dd73c;
const SHIP_COUNT: u32 = 0x6dd894;
const SHIP_ARRAY: u32 = 0x6dd7a4;
const SHIP_FIRST_FREE: u32 = 0x6dd888;
const SHIP_MGR: u32 = 0x6dd7a0;
const MSGS: u32 = 0x6dd730;
const GOOD_BY_WEIGHT: u32 = 0x672c14;
const SHIP_SIZE: u32 = 0x180;

fn ship(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(SHIP_ARRAY).wrapping_add(i.wrapping_mul(SHIP_SIZE))
}
/// Redondeo hacia cero a la unidad de la mercancía: 200 (por peso, tabla `0x672C14`) o 2.000.
fn round_unit(v: i32, by_weight: bool) -> i32 {
    let u = if by_weight { 200 } else { 2000 };
    (v / u) * u
}

/// 0x005338A0 `trade_offer_init_and_escrow` (`TradeOffer*` en ECX, `stdcall` de 2 argumentos: la oferta recibida `src` y el comerciante
/// `who`, que puede ser nulo). Marco SEH (manejador `0x6634B8`) con un mensaje local de 16 bytes en `esp-0x1C` (constructor `0x51A850`,
/// destructor `0x51A880`). Comprueba ciudades (`src+8`, `src+9` < `0x6DE4AA`); si hay comerciante, avanza `src+0` hacia atrás hasta que
/// coincide con el comerciante o `0x533EA0` lo acepta; programa la tarea `0x13` a `tick+5` y copia la oferta en `this+4..0x14`. Según el
/// tipo `src+0xB` retiene el depósito: 2 dinero de la ciudad (`0x5303C0`), 1 barco (lo saca de la lista libre con `0x508FC0`), 0 mercancía
/// (almacén de `0x5308A0` y luego barcos de la ciudad con `0x5182B0`/`0x517770(4)`). Termina con dos avisos `0x4D6530` (códigos `0x1E` y
/// `0x1F`) con el mensaje local. Devuelve `this`.
pub fn trade_offer_init_and_escrow(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let src = m.r32(f.arg(0));
    let who = m.r32(f.arg(1));
    let msg = f.at(0x1c);
    f.seh_enter(m, 0x66_34b8);
    host.call(m, 0x51a850, msg, &[]);
    f.seh_state(m, 0);
    'body: {
        let nt = m.r16(TOWN_COUNT);
        if u16::from(m.r8(src + 8)) >= nt || u16::from(m.r8(src + 9)) >= nt {
            break 'body;
        }
        if who != 0 {
            loop {
                let id = m.r32(src);
                let a = u32::from(m.r8(src + 8));
                let b = u32::from(m.r8(src + 9));
                let same = m.r32(who + 4) == id && u32::from(m.r8(who + 0xc)) == a && u32::from(m.r8(who + 0xd)) == b;
                if !same {
                    let o = m.r32(who);
                    if o == 0 {
                        break;
                    }
                    if host.call(m, 0x533ea0, o, &[id, a, b]) as u8 != 0 {
                        break;
                    }
                }
                let n = m.r32(src).wrapping_sub(1);
                m.w32(src, n);
                if n < 1 {
                    break 'body;
                }
            }
        }
        m.w32(this, who);
        let t = m.r32(TICK).wrapping_add(5);
        let task = host.call(m, 0x4d8cf0, SCHED, &[t]);
        m.w16(task + 6, 0x13);
        let v = m.r32(src);
        m.w32(task + 8, v);
        let v = u32::from(m.r8(src + 8));
        m.w32(task + 0x10, v);
        let v = u32::from(m.r8(src + 9));
        m.w32(task + 0x14, v);
        let v = m.r32(src);
        m.w32(this + 4, v);
        let v = m.r32(src + 4);
        m.w32(this + 8, v);
        for o in 8..0xc {
            let b = m.r8(src + o);
            m.w8(this + 4 + o, b);
        }
        let v = m.r32(src + 0xc);
        m.w32(this + 0x10, v);
        match m.r8(src + 0xb) {
            2 => {
                let town = u32::from(m.r8(this + 0xc));
                let p = host.call(m, 0x5303c0, WORLD, &[town]);
                let have = m.r32(p) as i32;
                let want = m.r32(this + 0x10) as i32;
                if have >= want {
                    m.w32(p, have.wrapping_sub(want) as u32);
                } else if have <= 0 {
                    m.w32(this + 0x10, 0);
                } else {
                    m.w32(this + 0x10, have as u32);
                    m.w32(p, 0);
                }
            }
            1 => {
                let idx = m.r16(this + 0x10);
                let cnt = m.r16(SHIP_COUNT);
                if idx >= cnt {
                    break 'body;
                }
                let s = ship(m, u32::from(idx));
                if m.r16(s + 8) < idx {
                    m.w16(this + 0x10, 0xffff);
                    break 'body;
                }
                let mut e = u32::from(m.r16(SHIP_FIRST_FREE));
                let cnt = u32::from(cnt);
                loop {
                    if e >= cnt {
                        m.w16(this + 0x10, 0xffff);
                        break 'body;
                    }
                    if e == u32::from(idx) {
                        break;
                    }
                    let base = m.r32(SHIP_ARRAY);
                    e = u32::from(m.r16(e.wrapping_mul(SHIP_SIZE).wrapping_add(base).wrapping_add(6)));
                }
                host.call(m, 0x508fc0, SHIP_MGR, &[u32::from(idx)]);
                let b = m.r8(s + 0x3c) | 0x20;
                m.w8(s + 0x3c, b);
                m.w16(s + 0x134, 0xd);
                m.w16(s + 6, 0xffff);
            }
            0 => {
                let good = u32::from(m.r16(this + 0x10));
                let qty = m.r16(this + 0x12) as i32;
                let flag = m.r8(GOOD_BY_WEIGHT + good) != 0;
                let mut left: i32 = if flag { qty * 200 } else { qty * 2000 };
                let town = u32::from(m.r8(this + 0xc));
                let side = u32::from(m.r8(this + 0xe));
                let st = host.call(m, 0x5308a0, WORLD, &[town, side, 0]);
                if st != 0 {
                    let p = st + good * 4 + 4;
                    let c = m.r32(p) as i32;
                    if c >= left {
                        m.w32(p, c.wrapping_sub(left) as u32);
                        left = 0;
                    } else {
                        let flag = m.r8(GOOD_BY_WEIGHT + good) != 0;
                        let r = round_unit(c, flag);
                        left = left.wrapping_sub(r);
                        m.w32(p, c.wrapping_sub(r) as u32);
                    }
                }
                let town = u32::from(m.r8(this + 0xc));
                let t = host.call(m, 0x5303c0, WORLD, &[town]);
                let mut e = u32::from(m.r16(t + 0xe));
                if left > 0 {
                    loop {
                        let cnt = u32::from(m.r16(SHIP_COUNT));
                        if (e as i32) >= (cnt as i32) {
                            break;
                        }
                        let s = ship(m, e);
                        e = u32::from(m.r16(s + 4));
                        if m.r8(s + 0x39) == m.r8(this + 0xe) && m.r16(s + 0x134) < 4 {
                            let g = u32::from(m.r16(this + 0x10));
                            let p = s + g * 4 + 0x54;
                            let v = m.r32(p) as i32;
                            if left >= v {
                                left = left.wrapping_sub(v);
                                m.w32(p, 0);
                            } else {
                                m.w32(p, v.wrapping_sub(left) as u32);
                                left = 0;
                                let g = u32::from(m.r16(this + 0x10));
                                let flag = m.r8(GOOD_BY_WEIGHT + g) != 0;
                                let p = s + g * 4 + 0x54;
                                let v = m.r32(p) as i32;
                                m.w32(p, round_unit(v, flag) as u32);
                            }
                            host.call(m, 0x5182b0, s, &[]);
                            host.call(m, 0x517770, s, &[4]);
                        }
                        if left <= 0 {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        // Avisos con el mensaje local.
        let b = m.r8(src + 0xa);
        m.w8(msg + 5, b);
        let v = m.r32(src);
        m.w32(msg + 8, v);
        let a = m.r8(src + 8);
        m.w16(msg + 0xc, u16::from(a));
        let b = m.r8(src + 9);
        m.w16(msg + 0xe, u16::from(b));
        m.w8(msg + 4, 0x1e);
        let args = peek(m, &[u32::from(a), msg], msg, 4);
        host.call(m, 0x4d6530, MSGS, &args);
        let b = u32::from(m.r8(src + 9));
        m.w8(msg + 4, 0x1f);
        let args = peek(m, &[b, msg], msg, 4);
        host.call(m, 0x4d6530, MSGS, &args);
    }
    f.seh_state(m, 0xffff_ffff);
    host.call(m, 0x51a880, msg, &[]);
    f.seh_leave(m);
    this
}

/// 0x00533510 `setup_hamburg_luebeck_overland_route` (`GameWorld*` en ECX, sin argumentos). Busca en la lista de ciudades (`this+0x18`,
/// `this+0x10` entradas) las de código `0xC` y `0xE`; con cuatro bytes locales (`esp-0x10`, `esp-0xC`, `esp-8`, `esp-4`; los dos primeros
/// son los contadores de la búsqueda, reutilizados) pide a `0x4F9C80` de cada ciudad (`+0x7A4`) la casilla de la mercancía `0x27`, y si
/// las dos son válidas (byte bajo distinto de `0xFF`) crea con `0x521900` la ruta en las dos ciudades, marca `+0x76C |= 0x8000` y, si el
/// índice `+0x762` es menor que `+0x776`, pone a 1 el byte `+6` del registro devuelto por `0x51F1F0`. Los contadores locales son bytes: el
/// porte conserva los bytes altos de la pila tal como estaban.
pub fn setup_hamburg_luebeck_overland_route(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let (li, lj, la, lb) = (f.at(0x10), f.at(0xc), f.at(8), f.at(4));
    let first = m.r8(this + 0x18);
    m.w8(la, 0xff);
    m.w8(lb, 0xff);
    m.w8(li, 0);
    m.w8(lj, 0);
    let find = |m: &mut dyn Mem, slot: u32, code: u8| -> bool {
        if first == code {
            return true;
        }
        loop {
            let i = m.r8(slot).wrapping_add(1);
            m.w8(slot, i);
            if u32::from(i) >= m.r32(this + 0x10) {
                return false;
            }
            if m.r8(this + 0x18 + u32::from(i)) == code {
                return true;
            }
        }
    };
    if !find(m, li, 0xc) || !find(m, lj, 0xe) {
        return 0;
    }
    let i = u32::from(m.r8(li));
    let j = u32::from(m.r8(lj));
    let base = m.r32(this + 0x68);
    let a = base.wrapping_add(i * 0x9f8);
    let b = base.wrapping_add(j * 0x9f8);
    let args = peek(m, &[la, lj, 0x27, 0, 1], la, 1);
    let args = peek(m, &args, lj, 1);
    host.call(m, 0x4f9c80, a + 0x7a4, &args);
    let args = peek(m, &[lb, li, 0x27, 0, 1], lb, 1);
    let args = peek(m, &args, li, 1);
    host.call(m, 0x4f9c80, b + 0x7a4, &args);
    if m.r8(la) == 0xff || m.r8(lb) == 0xff {
        return 0;
    }
    let (x, y) = (m.r32(la), m.r32(lj));
    host.call(m, 0x521900, a, &[0xff, x, y, 0x27, 0]);
    let (x, y) = (m.r32(lb), m.r32(li));
    host.call(m, 0x521900, b, &[0xff, x, y, 0x27, 0]);
    for t in [a, b] {
        let v = m.r32(t + 0x76c) | 0x8000;
        m.w32(t + 0x76c, v);
    }
    for t in [a, b] {
        let k = m.r16(t + 0x762);
        if k < m.r16(t + 0x776) {
            let r = host.call(m, 0x51f1f0, t, &[u32::from(k)]);
            m.w8(r + 6, 1);
        }
    }
    0
}

const MALLOC: u32 = 0x64f7b9;
const FREE: u32 = 0x64f7e2;

/// 0x00538DE0 `op_edit_ship_route_waypoints` (operación en ECX: `+4` barco, `+8` índice del punto, `+0xC`/`+0x10` coordenadas). Edita la
/// lista de puntos de ruta de la ciudad del barco (`town+0xEC`, palabras: `[0]` capacidad, `[2]` palabras usadas, pares x/y desde la palabra
/// 4; se crea con 0x80 bytes y crece al doble con `malloc`/`free` del CRT, `0x64F7B9`/`0x64F7E2`). Índice 0: reinicia la lista y añade el
/// punto. Índice `n >= 0`: reemplaza el punto `n`, lo añade si es el siguiente o libera la lista si queda fuera. Índice -1: confirma la
/// ruta: si el barco no puede navegar avisa a `0x44F940`; si no, toma como destino el primer punto en zona de otro comerciante
/// (`0x517260`), o la posición de la ciudad del comerciante (estados 2/3) o del barco, llama a `0x537670`, libera la ruta anterior, pone
/// la posición del barco como primer punto, quita puntos repetidos consecutivos y entrega al planificador `0x445010` un par local
/// `{puntero, recuento}` en `esp-8` (la dirección se pasa por la pila). El tiempo de llegada (`0x5169C0` + tick) queda en `ship+0x48`.
pub fn op_edit_ship_route_waypoints(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let (l_op, l_ptr, l_cnt) = (f.at(0xc), f.at(8), f.at(4));
    let idx = m.r32(op + 4);
    m.w32(l_op, op);
    if idx >= u32::from(m.r16(SHIP_COUNT)) {
        return 0;
    }
    let s = ship(m, idx);
    let home = m.r8(s);
    if u16::from(home) >= m.r16(TOWN_COUNT) {
        return 0;
    }
    let t = host.call(m, 0x5303c0, WORLD, &[u32::from(home)]);
    let lst = t + 0xec;
    if m.r32(lst) == 0 {
        let p = host.call(m, MALLOC, 0, &[0x80]);
        m.w32(lst, p);
        if p == 0 {
            return 0;
        }
        m.w16(p, 0x40);
        let p = m.r32(lst);
        m.w16(p + 2, 4);
    }
    let k = m.r32(op + 8);
    let append = |m: &mut dyn Mem| {
        for o in [0xc, 0x10] {
            let b = m.r32(lst);
            let n = u32::from(m.r16(b + 2));
            let v = m.r16(op + o);
            m.w16(b + n * 2, v);
            let b = m.r32(lst);
            let n = m.r16(b + 2).wrapping_add(1);
            m.w16(b + 2, n);
        }
    };
    if k == 0 {
        let b = m.r32(lst);
        m.w16(b + 2, 4);
        append(m);
        return 0;
    }
    if k != 0xffff_ffff {
        let b = m.r32(lst);
        let want = k.wrapping_mul(2).wrapping_add(4);
        let used = u32::from(m.r16(b + 2));
        if want > used {
            host.call(m, FREE, 0, &[b]);
            m.w32(lst, 0);
            return 0;
        }
        if want < used {
            let v = m.r16(op + 0xc);
            m.w16(b.wrapping_add(k.wrapping_mul(4)).wrapping_add(8), v);
            let k = m.r32(op + 8);
            let b = m.r32(lst);
            let v = m.r16(op + 0x10);
            m.w16(b.wrapping_add(k.wrapping_mul(4)).wrapping_add(0xa), v);
            return 0;
        }
        let cap = i32::from(m.r16(b));
        if (used as i32) + 2 > cap {
            let nb = host.call(m, MALLOC, 0, &[(cap as u32) * 4]);
            if nb == 0 {
                let b = m.r32(lst);
                host.call(m, FREE, 0, &[b]);
                m.w32(lst, 0);
                return 0;
            }
            let b = m.r32(lst);
            let mut n = u32::from(m.r16(b + 2));
            while n > 0 {
                n -= 1;
                let b = m.r32(lst);
                let w = m.r16(b + n * 2);
                m.w16(nb + n * 2, w);
            }
            let w = m.r16(nb).wrapping_add(m.r16(nb));
            m.w16(nb, w);
            let b = m.r32(lst);
            host.call(m, FREE, 0, &[b]);
            m.w32(lst, nb);
        }
        append(m);
        return 0;
    }
    // Índice -1: confirmar la ruta.
    let st = m.r16(s + 0x134);
    let mut ok = !(st >= 0x11 || st == 0xe || (m.r32(s + 0x18) as i32) < 1);
    if ok {
        let owner = u32::from(m.r8(s + 0x39));
        if owner < m.r32(0x6de4b0) {
            ok = host.call(m, 0x519ba0, s, &[]) as u8 != 0;
        }
    }
    if !ok {
        let w = m.r32(0x6cbdd8);
        if w != 0 {
            let i = m.r32(op + 4);
            host.call(m, 0x44f940, w, &[i]);
        }
        return 0;
    }
    let b = m.r32(lst);
    let used = m.r16(b + 2);
    if used <= 5 {
        return 0;
    }
    let mut e: u32 = 4;
    if used > 4 {
        loop {
            let b = m.r32(lst);
            let y = u32::from(m.r16(b + e * 2 + 2));
            let x = u32::from(m.r16(b + e * 2));
            let r = host.call(m, 0x517260, 0, &[x, y]);
            let owner = u32::from(m.r8(s + 0x39));
            if (r as i32) >= 0 && r < m.r32(0x6de4b0) && r != owner {
                let o = m.r32(l_op);
                m.w32(o + 8, r);
                let b = m.r32(lst);
                let v = u32::from(m.r16(b + e * 2));
                m.w32(o + 0xc, v);
                let b = m.r32(lst);
                let v = u32::from(m.r16(b + e * 2 + 2));
                e += 2;
                m.w32(o + 0x10, v);
                let b = m.r32(lst);
                m.w16(b + 2, e as u16);
                break;
            }
            let b = m.r32(lst);
            e += 2;
            if e as i32 >= i32::from(m.r16(b + 2)) {
                break;
            }
        }
    }
    let op = m.r32(l_op);
    if m.r32(op + 8) == 0xffff_ffff {
        let st = m.r16(s + 0x134);
        if st == 3 || st == 2 {
            for (o, tbl) in [(0xc, 0x6ddbb0), (0x10, 0x6ddbb4)] {
                let side = u32::from(m.r8(s + 0x39)) & 1;
                let k = u32::from(m.r8(0x6de4b9 - side));
                let v = m.r32(tbl + k * 0x34);
                m.w32(op + o, v);
            }
        } else {
            let v = m.r16(s + 0x1e) as i16 as i32 as u32;
            m.w32(op + 0xc, v);
            let v = m.r16(s + 0x22) as i16 as i32 as u32;
            m.w32(op + 0x10, v);
        }
    }
    host.call(m, 0x537670, op, &[]);
    if m.r8(s + 0x36) != 0 {
        let r = m.r32(s + 0x24);
        if r != 0 {
            let q = m.r32(r);
            if q != 0 {
                host.call(m, FREE, 0, &[q]);
            }
            let r = m.r32(s + 0x24);
            host.call(m, FREE, 0, &[r]);
        }
    }
    m.w32(s + 0x24, 0);
    let b = m.r32(lst);
    let mut cnt = (u32::from(m.r16(b + 2)) >> 1).wrapping_sub(1);
    m.w32(l_cnt, cnt);
    let b = m.r32(lst);
    m.w32(l_ptr, b + 4);
    for o in [(4, 0x1e), (6, 0x22)] {
        let b = m.r32(lst);
        let v = m.r16(s + o.1);
        m.w16(b + o.0, v);
    }
    if (cnt as i32) > 1 {
        let mut i: i32 = 1;
        while i < cnt as i32 {
            let p = m.r32(l_ptr);
            let a = p.wrapping_add((i as u32) * 4);
            if m.r16(a) == m.r16(a - 4) && m.r16(a + 2) == m.r16(a - 2) {
                let mut j = i + 1;
                while j < cnt as i32 {
                    let p = m.r32(l_ptr);
                    let w = m.r16(p + (j as u32) * 4);
                    m.w16(p + (j as u32) * 4 - 4, w);
                    let p = m.r32(l_ptr);
                    let w = m.r16(p + (j as u32) * 4 + 2);
                    m.w16(p + (j as u32) * 4 - 2, w);
                    j += 1;
                }
                cnt = cnt.wrapping_sub(1);
                m.w32(l_cnt, cnt);
            }
            i += 1;
        }
    }
    let planner = m.r32(0x6cbdc8);
    let args = peek(m, &[l_ptr, s + 0x24], l_ptr, 2);
    let r = host.call(m, 0x445010, planner, &args);
    m.w8(s + 0x36, r as u8);
    let tick = m.r32(TICK);
    let route = m.r32(s + 0x24);
    let d = host.call(m, 0x5169c0, s, &[route]);
    m.w32(s + 0x48, d.wrapping_add(tick));
    0
}

const TOWNS: u32 = 0x6de508;
const TOWN_SIZE: u32 = 0x9f8;
const CUR_OFFER: u32 = 0x6de584;

/// 0x0053DA30 `op_create_trade_offer` (operación en ECX). Marco SEH (`0x66358D`) con el puntero recién reservado en `esp-0x10`: reserva
/// 0x14 bytes y, si hay memoria, construye la oferta con `0x5338A0(op+4, oferta actual 0x6DE584)`; un resultado no nulo pasa a ser la
/// oferta actual. El estado del marco queda en 0 (el original no lo restablece). Devuelve EAX (0 sin memoria).
pub fn op_create_trade_offer(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66_358d);
    let p = host.call(m, MALLOC, 0, &[0x14]);
    m.w32(f.at(0x10), p);
    f.seh_state(m, 0);
    let mut r = p;
    if p != 0 {
        let cur = m.r32(CUR_OFFER);
        r = host.call(m, 0x5338a0, p, &[op + 4, cur]);
        if r != 0 {
            m.w32(CUR_OFFER, r);
        }
    }
    f.seh_leave(m);
    r
}

/// 0x005424D0 `op_add_town_supply_entry` (operación en ECX: `+4` ciudad, `+8`, `+0xC`). Marco SEH (`0x66366A`), puntero reservado en
/// `esp-0x10`: reserva 0x14 bytes, los construye con `0x50FCA0` (nulo si no hay memoria), los rellena con `0x50FE00(op+0xC (16 bits),
/// op+4 (byte), op+8 (byte))` y los enlaza al principio de la lista de la ciudad (`town+0x9B4`). Devuelve la dirección de la ciudad.
pub fn op_add_town_supply_entry(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66_366a);
    let p = host.call(m, MALLOC, 0, &[0x14]);
    m.w32(f.at(0x10), p);
    f.seh_state(m, 0);
    let e = if p != 0 { host.call(m, 0x50fca0, p, &[]) } else { 0 };
    let a = u32::from(m.r8(op + 8));
    let b = u32::from(m.r8(op + 4));
    let c = u32::from(m.r16(op + 0xc));
    f.seh_state(m, 0xffff_ffff);
    host.call(m, 0x50fe00, e, &[c, b, a]);
    let town = m.r32(op + 4);
    let t = m.r32(TOWNS).wrapping_add(town.wrapping_mul(TOWN_SIZE));
    let head = m.r32(t + 0x9b4);
    m.w32(e, head);
    m.w32(t + 0x9b4, e);
    f.seh_leave(m);
    t
}

/// 0x004DBB00 `st_town_mayor_election` (tarea programada 0x02; planificador en ECX: `[0]` registros de 0x18 bytes, `[8]` registro actual).
/// Si el día guardado (`rec+8`) no es válido (>= 365) lo deduce de la fecha del registro (`rec[0] >> 8`) con `0x5311D0`, que escribe en
/// `rec+0xF`, `rec+0x10` y en un día local de 16 bits en `esp-2` (la mitad alta del ECX empujado en el prólogo). Si el día es hoy
/// (`0x6DE4A4`) convoca la elección en la ciudad `rec+0xE` (`0x528E60`) y marca `sched+4 = 1`. Reprograma el registro para el mismo día
/// de este año o del siguiente: `((año·365 + día) << 8) | 0xFA`, que también devuelve.
pub fn st_town_mayor_election(m: &mut dyn Mem, host: &mut dyn OpsHost, sched: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    m.w32(f.at(4), sched);
    let rec_of = |m: &mut dyn Mem| m.r32(sched).wrapping_add(u32::from(m.r16(sched + 8)) * 0x18);
    let mut rec = rec_of(m);
    if m.r16(rec + 8) >= 0x16d {
        let date = m.r32(rec);
        m.w16(rec + 8, ((date >> 8) % 0x16d) as u16);
        host.call(m, 0x5311d0, WORLD, &[date, rec + 0xf, rec + 0x10, f.at(2)]);
    }
    let day = m.r16(rec + 8);
    let today = m.r16(0x6de4a4);
    let year = if today < day {
        u32::from(m.r16(0x6de4a2))
    } else {
        if today == day {
            m.w32(sched + 4, 0);
            let town = u32::from(m.r8(rec + 0xe));
            let t = m.r32(TOWNS).wrapping_add(town * TOWN_SIZE);
            host.call(m, 0x528e60, t, &[]);
            m.w32(sched + 4, 1);
            rec = rec_of(m);
        }
        u32::from(m.r16(0x6de4a2)) + 1
    };
    let day = u32::from(m.r16(rec + 8));
    let v = (year.wrapping_mul(365).wrapping_add(day) << 8) | 0xfa;
    m.w32(rec, v);
    v
}

/// 0x004DDE70 `st_captain_return_to_pool` (tarea 0x29; planificador en ECX). El capitán `rec+0xC` (< `0x6DD892`) de la ciudad `rec+8`
/// (< `0x6DE4B0`) vuelve al grupo de la taberna si `0x526AA0` lo acepta; el registro del capitán (`0x6DD7A0 + idx·16`) se pasa por la pila
/// como un local en `esp-4` (el ECX empujado del prólogo, con el índice en su mitad baja). Si el registro tiene destino (`rec+0x10`)
/// marca `cap+0xF = 0xFF`, lo entrega a `0x526B00` y aplaza la tarea 0xE00 (devuelve 1); si no, lo libera con `0x5098B0` (devuelve 0).
pub fn st_captain_return_to_pool(m: &mut dyn Mem, host: &mut dyn OpsHost, sched: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let loc = f.at(4);
    m.w32(loc, sched);
    let base = m.r32(sched);
    let rec = base.wrapping_add(u32::from(m.r16(sched + 8)) * 0x18);
    let cap = m.r16(rec + 0xc);
    m.w32(loc, (base & 0xffff_0000) | u32::from(cap));
    let town = u32::from(m.r16(rec + 8));
    if cap >= m.r16(0x6dd892) {
        return 0;
    }
    let c = m.r32(0x6dd7a0).wrapping_add(u32::from(cap) << 4);
    if town >= m.r32(0x6de4b0) {
        return 0;
    }
    let t = m.r32(TOWNS).wrapping_add(town * TOWN_SIZE);
    if host.call(m, 0x526aa0, t, &[u32::from(cap)]) as u8 == 0 {
        return 0;
    }
    let rec = m.r32(sched).wrapping_add(u32::from(m.r16(sched + 8)) * 0x18);
    if m.r32(rec + 0x10) != 0 {
        m.w8(c + 0xf, 0xff);
        let args = peek(m, &[loc], loc, 1);
        host.call(m, 0x526b00, t, &args);
        let rec = m.r32(sched).wrapping_add(u32::from(m.r16(sched + 8)) * 0x18);
        let v = m.r32(rec).wrapping_add(0xe00);
        m.w32(rec, v);
        1
    } else {
        let args = peek(m, &[loc], loc, 1);
        host.call(m, 0x5098b0, 0x6dd7a0, &args);
        0
    }
}

/// 0x00543810 `op_reset_world_state_on_start`. Marco SEH (`0x6636A9`) con un local en `esp-0x10` que primero guarda el byte `+0x11` de
/// la configuración (`0x6CC3E8`) convertido a `f32` (x87 `fild`/`fstp`, exacto para un byte) y después el puntero reservado. Llama a
/// `0x532420(f32, cfg+0xE + 1)`, reinicia cada comerciante de `0x6DF270` (registros de 0x1024 bytes, del último al primero) con
/// `0x635170`, destruye el grupo de guiones `0x6DD8B4` (`0x511080` y destructor virtual con 3) y crea uno nuevo (`0x511000`, 12 bytes) que
/// carga con `0x511270(1)`. Devuelve EAX de esa carga.
pub fn op_reset_world_state_on_start(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let loc = f.at(0x10);
    f.seh_enter(m, 0x66_36a9);
    let cfg = m.r32(0x6cc3e8);
    let a = m.r8(cfg + 0x11);
    let b = u32::from(m.r8(cfg + 0xe)) + 1;
    let fl = (a as f32).to_bits();
    m.w32(loc, fl);
    host.call(m, 0x532420, WORLD, &[fl, b]);
    let n = m.r32(0x6de4b0) as i32;
    let mut k = n;
    while k > 0 {
        k -= 1;
        let p = m.r32(0x6df270).wrapping_add((k as u32).wrapping_mul(0x1024));
        host.call(m, 0x635170, p, &[]);
    }
    let g = m.r32(0x6dd8b4);
    if g != 0 {
        host.call(m, 0x511080, g, &[]);
        let g = m.r32(0x6dd8b4);
        if g != 0 {
            let vt = m.r32(g);
            let fnp = m.r32(vt);
            host.call(m, fnp, g, &[3]);
        }
        m.w32(0x6dd8b4, 0);
    }
    let p = host.call(m, MALLOC, 0, &[0xc]);
    m.w32(loc, p);
    f.seh_state(m, 0);
    let g = if p != 0 { host.call(m, 0x511000, p, &[]) } else { 0 };
    f.seh_state(m, 0xffff_ffff);
    m.w32(0x6dd8b4, g);
    let r = host.call(m, 0x511270, g, &[1]);
    f.seh_leave(m);
    r
}

/// 0x0053DA90 `op_schedule_wedding` (operación en ECX: `+4` fecha, `+8..+0x11` datos de la boda, `+0x11` ciudad). Marco SEH (`0x66359F`)
/// con un mensaje local en `esp-0x1C` (`0x51A850`/`0x51A880`) y dos bytes de fecha en `esp-0x1E`/`esp-0x1D`. Si la fecha no ha pasado:
/// pone a 0 el byte bajo de `op+4`, programa la tarea `0x15` con los datos, avisa de la ciudad (`0x4D6FD0(town+0xA)`), rellena el mensaje
/// (código `0x33`, `op+0xE`, `town+0x19`, `op+0xC:op+0xD`), convierte la fecha con `0x5311D0` (que escribe los dos bytes y `msg+0xE`),
/// guarda `(b1 << 8) + b2 - 1` en `msg+0xC` y lo envía con `0x4D6530`. Devuelve EAX del destructor.
pub fn op_schedule_wedding(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let msg = f.at(0x1c);
    f.seh_enter(m, 0x66_359f);
    host.call(m, 0x51a850, msg, &[]);
    f.seh_state(m, 0);
    'body: {
        if m.r32(TICK) >= m.r32(op + 4) {
            break 'body;
        }
        m.w8(op + 4, 0);
        let d = m.r32(op + 4);
        let t = host.call(m, 0x4d8cf0, SCHED, &[d]);
        m.w16(t + 6, 0x15);
        let v = m.r32(op + 4);
        m.w32(t + 8, v);
        let v = m.r16(op + 8);
        m.w16(t + 0xc, v);
        for o in 0..8 {
            let b = m.r8(op + 0xa + o);
            m.w8(t + 0xe + o, b);
        }
        let town = m.r8(op + 0x11);
        if u16::from(town) >= m.r16(TOWN_COUNT) {
            break 'body;
        }
        let tw = host.call(m, 0x5303c0, WORLD, &[u32::from(town)]);
        let v = u32::from(m.r16(tw + 0xa));
        let r = host.call(m, 0x4d6fd0, MSGS, &[v]);
        m.w8(msg + 4, 0x33);
        let b = m.r8(op + 0xe);
        m.w8(msg + 5, b);
        let hi = u32::from(m.r8(op + 0xc));
        let lo = u32::from(m.r8(op + 0xd));
        let _ = r;
        let v = m.r8(tw + 0x19);
        m.w16(msg + 8, u16::from(v));
        m.w16(msg + 0xa, ((hi << 8) + lo) as u16);
        let d = m.r32(op + 4);
        host.call(m, 0x5311d0, WORLD, &[d, f.at(0x1e), f.at(0x1d), msg + 0xe]);
        let b1 = u32::from(m.r8(f.at(0x1e)));
        let b2 = u32::from(m.r8(f.at(0x1d)));
        m.w16(msg + 0xc, ((b1 << 8) + b2).wrapping_sub(1) as u16);
        let town = u32::from(m.r8(op + 0x11));
        let args = peek(m, &[town, msg], msg, 4);
        host.call(m, 0x4d6530, MSGS, &args);
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x51a880, msg, &[]);
    f.seh_leave(m);
    r
}

/// 0x0053EAC0 `op_pay_matchmaker_fee` (`stdcall`, un argumento: la orden `a`). Marco SEH (`0x6635C3`) con mensaje local en `esp-0x1C`.
/// Si la ciudad `a+0xB` es válida, pasa la cuota `a+0` (16 bits) del dinero de la ciudad (`town+0`) a `town+0x4BC`. Con `a+0xF` programa
/// la tarea `0x17` dentro de `min(rel/8 + 2, 8)` días (`rel` = `0x531B70(town+0x19, a+0xC)`) y copia en ella los 16 bytes de la orden
/// (`memcpy` del CRT); sin él envía el aviso `0x2B` con el mensaje local. Si el aviso `a+2` sigue en la cola (`0x6DD730`, código `0x21`)
/// pone a 0 su `+0xE`. Devuelve EAX del destructor.
pub fn op_pay_matchmaker_fee(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let a = m.r32(f.arg(0));
    let msg = f.at(0x1c);
    f.seh_enter(m, 0x66_35c3);
    host.call(m, 0x51a850, msg, &[]);
    f.seh_state(m, 0);
    'body: {
        let town = m.r8(a + 0xb);
        if u16::from(town) >= m.r16(TOWN_COUNT) {
            break 'body;
        }
        let t = host.call(m, 0x5303c0, WORLD, &[u32::from(town)]);
        let fee = u32::from(m.r16(a));
        let v = m.r32(t).wrapping_sub(fee);
        m.w32(t, v);
        let v = m.r32(t + 0x4bc).wrapping_add(fee);
        m.w32(t + 0x4bc, v);
        if m.r8(a + 0xf) != 0 {
            let x = u32::from(m.r8(t + 0x19));
            let y = u32::from(m.r8(a + 0xc));
            let r = host.call(m, 0x531b70, WORLD, &[x, y]);
            let d = ((r & 0xff) >> 3).wrapping_add(2).min(8);
            let when = (d << 8).wrapping_add(m.r32(TICK));
            let task = host.call(m, 0x4d8cf0, SCHED, &[when]);
            m.w16(task + 6, 0x17);
            for o in 0..0x10 {
                let b = m.r8(a + o);
                m.w8(task + 8 + o, b);
            }
        } else {
            let v = m.r8(a + 0xe);
            m.w16(msg + 0xa, u16::from(v));
            let v = m.r8(a + 0xd);
            m.w16(msg + 8, u16::from(v));
            let v = m.r8(a + 0xc);
            m.w8(msg + 5, v);
            let town = u32::from(m.r8(a + 0xb));
            m.w8(msg + 4, 0x2b);
            let v = m.r16(a);
            m.w16(msg + 0xc, v);
            let args = peek(m, &[town, msg], msg, 4);
            host.call(m, 0x4d6530, MSGS, &args);
        }
        let k = m.r16(a + 2);
        let n = m.r16(0x6dd736);
        if k >= n {
            break 'body;
        }
        let p = m.r32(0x6dd730).wrapping_add(u32::from(k) << 4);
        if m.r8(p + 4) == 0x21 {
            m.w16(p + 0xe, 0);
        }
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x51a880, msg, &[]);
    f.seh_leave(m);
    r
}

/// 0x00635B80 `op_create_siege_supply_request` (`thiscall`, comerciante en ECX y ocho argumentos: `a0` ciudad del aviso, `a1..a6` bytes de
/// la petición, `a7` fecha). Marco SEH (`0x668521`) con mensaje local en `esp-0x1C` y tres bytes de fecha en `esp-0x20..0x1E`. Reserva la
/// petición (0x14 bytes, sin comprobar nulo), la inserta en la lista ordenada por fecha de `this+0x1020` (siempre detrás de la cabeza),
/// convierte la fecha con `0x5311D0` y envía el aviso `0x64` con `0x4D6530`. Devuelve EAX del destructor.
pub fn op_create_siege_supply_request(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let a: Vec<u32> = (0..8).map(|i| m.r32(f.arg(i))).collect();
    let msg = f.at(0x1c);
    f.seh_enter(m, 0x66_8521);
    let e = host.call(m, MALLOC, 0, &[0x14]);
    host.call(m, 0x51a850, msg, &[]);
    for (o, v) in [(4, a[1]), (2, a[2]), (0, a[3]), (3, a[4]), (5, a[0])] {
        m.w8(e + o, v as u8);
    }
    for o in [7, 9, 8] {
        m.w8(e + o, 0);
    }
    m.w8(e + 1, a[5] as u8);
    m.w8(e + 6, a[6] as u8);
    m.w32(e + 0xc, a[7]);
    f.seh_state(m, 0);
    let head = m.r32(this + 0x1020);
    if head == 0 {
        m.w32(this + 0x1020, e);
        m.w32(e + 0x10, 0);
    } else {
        let mut c = head;
        loop {
            let n = m.r32(c + 0x10);
            if n == 0 || m.r32(n + 0xc) >= a[7] {
                break;
            }
            c = n;
        }
        let n = m.r32(c + 0x10);
        m.w32(e + 0x10, n);
        m.w32(c + 0x10, e);
    }
    let d = m.r32(e + 0xc);
    host.call(m, 0x5311d0, WORLD, &[d, f.at(0x20), f.at(0x1f), f.at(0x1e)]);
    m.w8(msg + 4, 0x64);
    let v = m.r8(this + 4);
    m.w8(msg + 5, v);
    for (o, src) in [(8, 4), (9, 2), (0xa, 0), (0xb, 3)] {
        let v = m.r8(e + src);
        m.w8(msg + o, v);
    }
    let d0 = m.r8(f.at(0x20));
    let d1 = m.r8(f.at(0x1f));
    m.w8(msg + 0xd, d0);
    let v = m.r8(e + 1);
    m.w8(msg + 0xc, v);
    m.w8(msg + 0xe, d1);
    let v = m.r8(e + 6);
    m.w8(msg + 0xf, v);
    let args = peek(m, &[a[0] & 0xff, msg], msg, 4);
    host.call(m, 0x4d6530, MSGS, &args);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x51a880, msg, &[]);
    f.seh_leave(m);
    r
}

/// 0x0051E780 `reset_buildings` (ciudad en ECX). Marco SEH (`0x66320E`) con `this` guardado en `esp-0x10`: reinicia los edificios
/// (`0x5286E0`), el almacén `+0x7A4` (`0x4F95C0`, `0x4F9930`) y `+0x794` (`0x4FE2B0`), y deja en su valor inicial los campos
/// `+0x75C..+0x792` (`0xFFFF` seis índices, 1 en `+0x786`/`+0x78C`/`+0x78E`, ceros el resto; dos `memset` de 6 bytes del CRT). Devuelve `this`.
pub fn reset_buildings(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66_320e);
    m.w32(f.at(0x10), this);
    host.call(m, 0x5286e0, this, &[]);
    host.call(m, 0x4f95c0, this + 0x7a4, &[]);
    for o in [0x762, 0x76a, 0x764, 0x768, 0x766, 0x784] {
        m.w16(this + o, 0xffff);
    }
    f.seh_state(m, 0);
    m.w32(this + 0x75c, 0);
    m.w16(this + 0x776, 0);
    m.w16(this + 0x760, 0);
    m.w8(this + 0x774, 0);
    m.w32(this + 0x76c, 0);
    for o in 0..6 {
        m.w8(this + 0x778 + o, 0);
        m.w8(this + 0x77e + o, 0);
    }
    for o in [0x770, 0x771, 0x772, 0x773, 0x789, 0x788, 0x78a] {
        m.w8(this + o, 0);
    }
    for o in [0x78c, 0x78e, 0x786] {
        m.w16(this + o, 1);
    }
    m.w16(this + 0x790, 0);
    m.w16(this + 0x792, 0);
    host.call(m, 0x4f9930, this + 0x7a4, &[]);
    host.call(m, 0x4fe2b0, this + 0x794, &[]);
    f.seh_leave(m);
    this
}

/// 0x00517260 `get_town_index_at_point` (`cdecl`, `x` e `y` de 16 bits). Empaqueta el punto en su propio argumento (`esp+4` = `y:x`) y
/// pasa el valor y la dirección de ese argumento al mapa (`0x446D50`, objeto `0x6CBDC8`). Devuelve la ciudad o -1 si es >= 40.
pub fn get_town_index_at_point(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let x = m.r16(f.arg(0));
    let y = m.r16(f.arg(1));
    m.w16(f.arg(0), x);
    m.w16(f.arg(0) + 2, y);
    let p = m.r32(f.arg(0));
    let map = m.r32(0x6cbdc8);
    let args = peek(m, &[p, f.arg(0)], f.arg(0), 1);
    let r = host.call(m, 0x446d50, map, &args);
    if (r as i32) < 0x28 {
        r
    } else {
        0xffff_ffff
    }
}

/// 0x00543570 `op_append_indexed_text_chunk` (operación en ECX: `+4..+0x10` trozo de texto, `+0x11` entrada, `+0x12`, `+0x13` posición).
/// Copia la entrada actual del banco de nombres (`0x6DDAA0`, 40 cadenas de 0x30 bytes) a un búfer local de 0x30 bytes en `esp-0x30`,
/// añade hasta 13 caracteres del trozo en `+0x13` (que avanza; para en el cero o en 0x2F), termina el búfer y lo guarda con `0x512830`.
/// El valor de EAX al salir no está definido en la salida temprana.
pub fn op_append_indexed_text_chunk(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let buf = f.at(0x30);
    let idx = m.r8(op + 0x11);
    if idx >= 0x28 || m.r8(op + 0x12) > 0x2f || m.r8(op + 0x13) > 0x27 {
        return 0;
    }
    let src = m.r32(0x6ddaa8 + u32::from(idx) * 4);
    for o in 0..0x30 {
        let b = m.r8(src + o);
        m.w8(buf + o, b);
    }
    for i in 0..0xd {
        let pos = m.r8(op + 0x13);
        let c = m.r8(op + 4 + i);
        let np = pos.wrapping_add(1);
        m.w8(op + 0x13, np);
        m.w8(buf + u32::from(pos), c);
        if m.r8(op + 4 + i) == 0 || np >= 0x2f {
            break;
        }
    }
    let idx = u32::from(m.r8(op + 0x11));
    m.w8(buf + 0x2f, 0);
    let args = peek(m, &[idx, buf], buf, 12);
    host.call(m, 0x512830, 0x6ddaa0, &args)
}

/// 0x004E2634 `st_captain_pool_refill` (tarea programada, `stdcall` con el registro `rec`). Marca en un vector local de 40 bytes
/// (`esp-0x28`) cada ciudad: bit 0 si su taberna tiene menos capitanes que el límite (`0x5269A0`, < `0x6DD892`) y bit 1 si
/// `0x5261D0` no lo supera; el resto vale `0xFF`. Con menos de 3 ciudades con bit 1, rota `rec+0x14`/`rec+0x10` hasta una ciudad sin él y
/// le añade un capitán (`0x526A50(1)`). Con 8 o más ciudades con bit 0, aplaza 0x700. Si no, compara con los capitanes del grupo
/// (`0x509930` rellena dos pares locales en `esp-0x30` y `esp-0x38`): si sobran ciudades, aplaza 0x400; si no, elige la siguiente ciudad
/// sin bit 0 con el paso `rec+0xC` (mod 29, nunca 0 ni 20) y le añade un capitán (`0x526A50(0)`), aplazando 0x200. Devuelve `rec[0]`.
pub fn st_captain_pool_refill(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let rec = m.r32(f.arg(0));
    let flags = f.at(0x28);
    let l_cnt = f.at(0x3c);
    let mut count: i32 = 0;
    let mut full: i32 = 0;
    m.w32(l_cnt, 0);
    let mut n = m.r32(0x6de4b0);
    let mut i = 0u32;
    while i < n {
        let t = m.r32(TOWNS).wrapping_add(i * TOWN_SIZE);
        let a = u32::from(m.r16(TOWN_COUNT));
        let r = host.call(m, 0x5269a0, t, &[a]);
        if r < u32::from(m.r16(0x6dd892)) {
            m.w8(flags + i, 1);
            count += 1;
            m.w32(l_cnt, count as u32);
        } else {
            m.w8(flags + i, 0);
        }
        let r2 = host.call(m, 0x5261d0, t, &[]);
        if u32::from(m.r16(0x6dd892)) >= r2 {
            let b = m.r8(flags + i) | 2;
            m.w8(flags + i, b);
            full += 1;
        }
        n = m.r32(0x6de4b0);
        i += 1;
    }
    if (n as i32) < 0x28 {
        for k in n..0x28 {
            m.w8(flags + k, 0xff);
        }
    }
    let town = |m: &mut dyn Mem, k: u32| m.r32(TOWNS).wrapping_add(k * TOWN_SIZE);
    if full < 3 {
        let step = (m.r32(rec + 0x14).wrapping_add(3) % 0x17) + 1;
        m.w32(rec + 0x14, step);
        let mut tries = 0;
        let mut d;
        loop {
            d = m.r32(rec + 0x10).wrapping_add(step) % 0x25;
            m.w32(rec + 0x10, d);
            if m.r8(flags + d) & 2 == 0 {
                break;
            }
            tries += 1;
            if tries >= 0x28 {
                break;
            }
        }
        let t = town(m, d);
        host.call(m, 0x526a50, t, &[1]);
    }
    let count = m.r32(l_cnt) as i32;
    if count >= 8 {
        let v = m.r32(rec).wrapping_add(0x700);
        m.w32(rec, v);
        return v;
    }
    host.call(m, 0x509930, 0x6dd7a0, &[f.at(0x30), f.at(0x38)]);
    let a0 = m.r32(f.at(0x38)) as i32;
    let a1 = m.r32(f.at(0x34)) as i32;
    let b0 = m.r32(f.at(0x30)) as i32;
    let b1 = m.r32(f.at(0x2c)) as i32;
    let v = (a1 / 5).wrapping_add(a0.wrapping_add(1) >> 1).wrapping_sub(b1).wrapping_sub(b0);
    if count > v && count > 1 {
        let r = m.r32(rec).wrapping_add(0x400);
        m.w32(rec, r);
        return r;
    }
    let mut c = m.r32(rec + 8) % m.r32(0x6de4b0);
    let mut s = m.r32(rec + 0xc).wrapping_add(3) % 0x1d;
    m.w32(rec + 0xc, s);
    if s == 0 {
        m.w32(rec + 0xc, 3);
    } else if s == 0x14 {
        m.w32(rec + 0xc, 2);
    }
    s = m.r32(rec + 0xc);
    loop {
        c = s.wrapping_add(c) % 0x25;
        if m.r8(flags + c) & 1 == 0 {
            break;
        }
    }
    m.w32(rec + 8, c);
    let t = town(m, c);
    host.call(m, 0x526a50, t, &[0]);
    let r = m.r32(rec).wrapping_add(0x200);
    m.w32(rec, r);
    r
}

/// 0x004DE790 `st_ai_build_ship` (tarea programada; planificador en ECX). Construye en la pila una orden de la cola de operaciones
/// (`esp-0x7C`, cabecera de 5 palabras como `0x535740`); los huecos `esp-0x84` (suma), `esp-0x80` (ciudad elegida), `esp-0x68` (mínimo)
/// y `esp-0x64` (`this`) son locales que el compilador reparte con la orden. Suma `town+0x2D4` de todas las ciudades y recorre los
/// puertos (`0x5303C0`) del último al primero: los que no tienen astillero activo (`+8` = 0 o bit 4) cuentan `+0x478`; los demás cuentan
/// sus barcos (lista `+0xE` por `ship+4`) y el de menos barcos es el elegido. Si `400·barcos >= suma` o no hay elegido, aplaza 0x1500.
/// Si no, aplaza 0x2400 y (sin cliente de red `0x6DF31C`) pide la orden `0x2C` con el tipo de barco (1 o 3 según dinero y `+0x2C8`;
/// uno menos si `0x52B0D0` lo rechaza), recorriendo comerciantes desde el dueño del puerto con `0x52B2C0` hasta que uno acepta
/// (`0x54AA70` encola la orden) o se da la vuelta.
pub fn st_ai_build_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let op = f.at(0x7c);
    let (l_sum, l_best, l_min, l_this) = (f.at(0x84), f.at(0x80), f.at(0x68), f.at(0x64));
    m.w32(l_this, this);
    m.w32(op, 0xffff_ffff);
    for o in [4, 8, 0xc, 0x10] {
        m.w32(op + o, 0);
    }
    let n = m.r32(0x6de4b0);
    let mut best: u32 = 0xffff;
    m.w32(l_best, best);
    m.w32(l_sum, 0);
    m.w32(l_min, 1000);
    let mut total: u32 = 0;
    for i in 0..n {
        let t = m.r32(TOWNS).wrapping_add(i * TOWN_SIZE);
        total = total.wrapping_add(m.r32(t + 0x2d4));
    }
    let mut esi = u32::from(m.r16(TOWN_COUNT));
    let mut ships_total: u32 = 0;
    while esi > 0 {
        esi -= 1;
        let t = host.call(m, 0x5303c0, WORLD, &[esi]);
        let fl = m.r16(t + 8);
        let cnt;
        if fl == 0 || fl & 4 != 0 {
            cnt = u32::from(m.r16(t + 0x478));
        } else {
            let mut e = u32::from(m.r16(t + 0xe));
            let lim = u32::from(m.r16(SHIP_COUNT));
            let mut c = 0;
            while e < lim {
                let base = m.r32(SHIP_ARRAY);
                e = u32::from(m.r16(base.wrapping_add(e * SHIP_SIZE) + 4));
                c += 1;
            }
            if c < m.r32(l_min) {
                best = esi;
                m.w32(l_min, c);
                m.w32(l_best, best);
            }
            cnt = c;
        }
        ships_total = m.r32(l_sum).wrapping_add(cnt);
        m.w32(l_sum, ships_total);
    }
    let sched_rec = |m: &mut dyn Mem| m.r32(this).wrapping_add(u32::from(m.r16(this + 8)) * 0x18);
    if ships_total.wrapping_mul(400) >= total || best >= u32::from(m.r16(TOWN_COUNT)) {
        let r = sched_rec(m);
        let v = m.r32(r).wrapping_add(0x1500);
        m.w32(r, v);
        return 0;
    }
    let r = sched_rec(m);
    let v = m.r32(r).wrapping_add(0x2400);
    m.w32(r, v);
    if m.r32(0x6df31c) != 0 {
        return 0;
    }
    let t = host.call(m, 0x5303c0, WORLD, &[best]);
    m.w32(op, 0x2c);
    let kind = if (m.r32(t) as i32) < 0x9c40 {
        1
    } else {
        let o = u32::from(m.r8(t + 0x19));
        let tw = m.r32(TOWNS).wrapping_add(o * TOWN_SIZE);
        if m.r8(tw + 0x2c8) & 1 != 0 {
            1
        } else {
            3
        }
    };
    m.w32(op + 4, kind);
    m.w32(op + 8, best);
    let owner = u32::from(m.r8(t + 0x19));
    m.w32(op + 0xc, owner);
    m.w32(op + 0x10, 1);
    let tw = m.r32(TOWNS).wrapping_add(owner * TOWN_SIZE);
    if host.call(m, 0x52b0d0, tw, &[kind]) as u8 == 0 {
        let v = m.r32(op + 4).wrapping_sub(1);
        m.w32(op + 4, v);
    }
    loop {
        let k = m.r32(op + 0xc);
        let kind = m.r32(op + 4);
        let tw = m.r32(TOWNS).wrapping_add(k.wrapping_mul(TOWN_SIZE));
        if host.call(m, 0x52b2c0, tw, &[op + 0x1c, kind, best]) as u8 != 0 {
            let args = peek(m, &[op], op, 0x1c);
            host.call(m, 0x54aa70, 0x6df2f0, &args);
            return 0;
        }
        let mut k = k + 1;
        if k >= m.r32(0x6de4b0) {
            k = 0;
        }
        m.w32(op + 0xc, k);
        if k == u32::from(m.r8(t + 0x19)) {
            return 0;
        }
    }
}

/// 0x0050CC90 `detach_town_outrigger_for_battle` (`thiscall`: gestor de barcos en ECX, `+4` barcos de 0x180, `+8` registros de 0x3C, `+0xE8`
/// cabeza de lista, `+0xF4`/`+0xF6` límites; un argumento: ciudad `a`). Saca el barco de guardia de la ciudad (`town+0x994`) de la
/// lista enlazada por `ship+6`, lo deja sin ruta (libera `ship+0x24` con `free` si `ship+0x36`), lo reinicia para la batalla (estado
/// 0x14, dueño `a`, dos `memset` de 0x60 del CRT) y avisa a `0x5182B0`. Después pide a `0x5062F0` un registro usando su propio argumento
/// de la pila como salida (`esp+4`, sembrado con 0xFFFF); si es válido le pone el bit 0x10 en `+0x14`. EAX: ese registro, o el último
/// valor con los 16 bits bajos a 0xFFFF en las salidas tempranas.
pub fn detach_town_outrigger_for_battle(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let a = m.r32(f.arg(0));
    if a >= m.r32(0x6de4b0) {
        return a | 0xffff;
    }
    let ptr = m.r32(TOWNS).wrapping_add(a.wrapping_mul(TOWN_SIZE)) + 0x994;
    let bp = m.r16(ptr);
    if bp >= m.r16(this + 0xf4) {
        return ptr | 0xffff;
    }
    m.w16(ptr, 0xffff);
    let lim = m.r16(this + 0xf4);
    let ships = m.r32(this + 4);
    let s = ships.wrapping_add(u32::from(bp) * SHIP_SIZE);
    let mut link = this + 0xe8;
    let mut e = m.r16(link);
    while e < lim {
        if e == bp {
            let v = m.r16(s + 6);
            m.w16(link, v);
            m.w16(s + 6, 0xffff);
            break;
        }
        link = ships.wrapping_add(u32::from(e) * SHIP_SIZE) + 6;
        e = m.r16(link);
    }
    let r = m.r32(s + 0x24);
    m.w8(s + 1, 0);
    if r != 0 {
        if m.r8(s + 0x36) != 0 {
            m.w8(s + 0x36, 0);
            let q = m.r32(r);
            if q != 0 {
                host.call(m, FREE, 0, &[q]);
            }
            let r = m.r32(s + 0x24);
            host.call(m, FREE, 0, &[r]);
        }
        m.w32(s + 0x24, 0);
    }
    let al = m.r8(f.arg(0));
    for o in [0x3a, 0x3c, 0x50, 0x52] {
        m.w16(s + o, 0xffff);
    }
    for o in [0x28, 0x2c, 0x30] {
        m.w32(s + o, 0);
    }
    m.w16(s + 0x34, 0);
    m.w8(s + 0x37, al);
    m.w8(s + 0x38, al);
    m.w8(s + 0x39, 0xff);
    m.w32(s + 0x48, 0);
    for o in 0..0x60 {
        m.w8(s + 0x54 + o, 0);
    }
    for o in 0..0x60 {
        m.w8(s + 0xb4 + o, 0);
    }
    let al = m.r8(f.arg(0));
    m.w32(s + 0x114, 0);
    m.w32(s + 0x118, 0);
    m.w16(s + 0x134, 0x14);
    m.w16(s + 0x138, 0);
    m.w8(s + 0x13b, al);
    m.w8(s + 0x15c, 0);
    m.w8(s + 0x15d, 0xff);
    host.call(m, 0x5182b0, s, &[]);
    m.w32(f.arg(0), 0xffff);
    let args = peek(m, &[f.arg(0), u32::from(bp)], f.arg(0), 1);
    host.call(m, 0x5062f0, this, &args);
    let v = m.r32(f.arg(0));
    if (v as u16) >= m.r16(this + 0xf6) {
        return v | 0xffff;
    }
    let p = m.r32(this + 8).wrapping_add(u32::from(v as u16) * 0x3c) + 0x14;
    let b = m.r8(p) | 0x10;
    m.w8(p, b);
    v
}

/// 0x0053CCE0 `op_rename_ship` (operación en ECX: `+0` código, `+4` nombre de 12 bytes, `+0x10` barco). Copia el nombre a un búfer local
/// de 0x40 bytes (`esp-0x40`, `memcpy` del CRT) y lo registra en el banco de nombres `0x6DDAA0`: con el código `0x2D` directamente
/// (`0x512D40`); si no, solo si `0x512FC0` no lo reconoce, anteponiendo el nombre actual (`0x512B20`). Después escribe el nombre en
/// `ship+0x160` (reemplaza con `0x2D`, añade al final en otro caso; como mucho 0x1E caracteres) y avisa con `0x517770(1)`. EAX no
/// definido.
pub fn op_rename_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let buf = f.at(0x40);
    m.w32(f.at(0x44), op);
    let idx = m.r32(op + 0x10);
    if idx >= u32::from(m.r16(SHIP_COUNT)) {
        return 0;
    }
    let s = ship(m, idx);
    let name = op + 4;
    for o in 0..0xc {
        let b = m.r8(name + o);
        m.w8(buf + o, b);
    }
    m.w8(buf + 0xc, 0);
    let slot = s + 0x15e;
    if m.r32(op) == 0x2d {
        let args = peek(m, &[buf, slot], buf, 16);
        host.call(m, 0x512d40, 0x6ddaa0, &args);
    } else {
        let k = u32::from(m.r16(slot));
        let args = peek(m, &[buf, k], buf, 16);
        if host.call(m, 0x512fc0, 0x6ddaa0, &args) as u8 == 0 {
            let k = u32::from(m.r16(slot));
            let mut p = host.call(m, 0x512b20, 0x6ddaa0, &[k]);
            let mut i = 0;
            loop {
                let c = m.r8(p);
                if c == 0 {
                    break;
                }
                m.w8(buf + i, c);
                i += 1;
                p += 1;
            }
            let mut q = name;
            loop {
                let c = m.r8(q);
                if c == 0 {
                    break;
                }
                m.w8(buf + i, c);
                i += 1;
                q += 1;
            }
            m.w8(buf + i, 0);
            let args = peek(m, &[buf, slot], buf, 16);
            host.call(m, 0x512d40, 0x6ddaa0, &args);
        }
    }
    let mut e: u32 = 0;
    if m.r32(op) == 0x2d {
        m.w8(s + 0x160, 0);
    } else {
        while m.r8(s + 0x160 + e) != 0 {
            e += 1;
            if e >= 0x20 {
                break;
            }
        }
    }
    let mut c = 0;
    let mut dl = m.r8(name);
    while dl != 0 && e < 0x1e && c < 0xc {
        let b = m.r8(name + c);
        m.w8(s + 0x160 + e, b);
        dl = m.r8(name + c + 1);
        e += 1;
        c += 1;
    }
    m.w8(s + 0x160 + e, 0);
    host.call(m, 0x517770, s, &[1]);
    0
}

/// 0x00542380 `op_request_create_merchant_from_lobby` (operación en ECX). Rellena en la pila (`esp-0x14`) una orden `0xA7` de cinco
/// palabras con el jugador local (`0x6DF31C`), su perfil del vestíbulo (`0x6CBB94 + jugador + 0x1C4/0x1CC`), los bytes `+4`, `+8`,
/// `+0xC`, `+0x10` de la operación y dos `rand` (`0x639C03`: día `% 365` y una palabra), guarda `op+0xC % 5` en `0x6DE52C` y la encola
/// con `0x54AA70`. Devuelve EAX de la cola.
pub fn op_request_create_merchant_from_lobby(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let o = f.at(0x14);
    m.w32(o, 0xa7);
    for k in [8, 0xc, 0x10] {
        m.w32(o + k, 0);
    }
    let pl = m.r32(0x6df31c);
    let b = m.r8(op + 0xc);
    m.w8(o + 8, b);
    let prof = m.r32(0x6cbb94);
    m.w32(o + 4, pl);
    let v = m.r8(prof.wrapping_add(pl) + 0x1c4);
    m.w8(o + 9, v);
    let v = m.r8(op + 4);
    m.w8(o + 0xa, v);
    let v = m.r8(op + 8);
    m.w8(o + 0xb, v);
    let v = m.r8(prof.wrapping_add(pl) + 0x1cc);
    m.w8(o + 0xc, v);
    let r = host.call(m, 0x639c03, 0, &[]) as i32;
    m.w8(o + 0xd, (r % 0x16d) as u8);
    let r = host.call(m, 0x639c03, 0, &[]);
    m.w16(o + 0xe, r as u16);
    let v = m.r8(op + 0x10);
    m.w8(o + 0x10, v);
    let v = (m.r8(op + 0xc) % 5) as u8;
    m.w8(0x6de52c, v);
    let args = peek(m, &[o], o, 5);
    host.call(m, 0x54aa70, 0x6df2f0, &args)
}

/// 0x0053A810 `op_change_hometown` (operación en ECX: `+4` ciudad, `+8` comerciante nuevo, `+0xC` precio). Marco SEH (`0x663542`) con
/// mensaje local en `esp-0x1C` y el dueño anterior en `esp-0x20`. Si el puerto tiene otro dueño, sin bloqueo (`+8 & 4`) y el nuevo
/// admite el cambio (`+0x2C8 & 0x40`): envía el aviso `0xE` (a todos, -2) con el nivel `0x4F9430·8 + (town+0x39C+dueño & 7)`, cambia
/// el dueño, retira al anterior el encargo `+0x6F1` sobre esta ciudad (`0x4F75F0`), pasa el precio del dinero de la ciudad a `+0x4BC`,
/// avisa a los dos comerciantes (`0x635040(0/1)`) y actualiza la sede global `0x6DE52E`. EAX: `op+4` en la salida temprana, si no el del
/// destructor.
pub fn op_change_hometown(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let msg = f.at(0x1c);
    let l_old = f.at(0x20);
    f.seh_enter(m, 0x66_3542);
    let town = m.r32(op + 4);
    if town >= u32::from(m.r16(TOWN_COUNT)) || m.r32(op + 8) >= m.r32(0x6de4b0) {
        f.seh_leave(m);
        return town;
    }
    let t = host.call(m, 0x5303c0, WORLD, &[town]);
    host.call(m, 0x51a850, msg, &[]);
    let old = m.r8(t + 0x19);
    let new = m.r32(op + 8);
    f.seh_state(m, 0);
    'body: {
        if u32::from(old) == new || m.r8(t + 8) & 4 != 0 {
            break 'body;
        }
        let w = m.r32(TOWNS).wrapping_add(new.wrapping_mul(TOWN_SIZE));
        if m.r32(w + 0x2c8) & 0x40 == 0 {
            break 'body;
        }
        m.w8(l_old, old);
        m.w8(msg + 4, 0xe);
        let v = m.r8(op + 8);
        m.w8(msg + 5, v);
        let v = m.r16(op + 4);
        m.w16(msg + 8, v);
        let v = m.r8(t + 0x19);
        m.w16(msg + 0xa, u16::from(v));
        let o = u32::from(m.r8(t + 0x19));
        let bl = m.r8(t + o + 0x39c);
        let r = host.call(m, 0x4f9430, t, &[]);
        m.w16(msg + 0xe, 0);
        let lvl = u16::from(bl & 7).wrapping_add(u16::from(r as u8) * 8);
        m.w16(msg + 0xc, lvl);
        let args = peek(m, &[0xffff_fffe, msg], msg, 4);
        host.call(m, 0x4d6530, MSGS, &args);
        let old = u32::from(m.r8(l_old));
        let v = m.r8(op + 8);
        m.w8(t + 0x19, v);
        let w = m.r32(TOWNS).wrapping_add(old * TOWN_SIZE);
        if u32::from(m.r8(w + 0x6f1)) == m.r32(op + 4) && m.r32(w + 0x2c8) & 0xe0 != 0 {
            m.w8(w + 0x6f1, 0xff);
            m.w8(w + 0x6e4, 6);
            host.call(m, 0x4f75f0, t, &[old]);
        }
        let price = m.r32(op + 0xc);
        let v = m.r32(t).wrapping_sub(price);
        m.w32(t, v);
        let v = m.r32(t + 0x4bc).wrapping_add(m.r32(op + 0xc));
        m.w32(t + 0x4bc, v);
        let base = m.r32(0x6df270);
        host.call(m, 0x635040, base.wrapping_add(old * 0x1024), &[0]);
        let nw = m.r32(op + 8);
        let base = m.r32(0x6df270);
        host.call(m, 0x635040, base.wrapping_add(nw.wrapping_mul(0x1024)), &[1]);
        if m.r32(op + 4) == u32::from(m.r8(0x6de52d)) && m.r8(l_old) == m.r8(0x6de52e) {
            let v = m.r8(op + 8);
            m.w8(0x6de52e, v);
        }
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x51a880, msg, &[]);
    f.seh_leave(m);
    r
}

/// 0x00505B60 `unlink_ship_from_at_sea_list` (`thiscall`: gestor de barcos en ECX; un argumento, el barco `k` de 16 bits). Avisa a
/// `0x50D040`, saca `k` de la lista de barcos en el mar (`this+0xEA`, enlazada por `ship+6`; si no está, el último enlace recorrido
/// recibe el siguiente de `k`, como en el original), borra las referencias a `k` en `+0x50`/`+0x52` de los barcos que quedan y
/// refresca el mapa (`0x44F940`). Devuelve AL = 1 sobre el EAX del aviso.
pub fn unlink_ship_from_at_sea_list(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let di = m.r16(f.arg(0));
    let k = u32::from(di);
    host.call(m, 0x50d040, this, &[k]);
    let head = this + 0xea;
    let mut link = head;
    let mut e = m.r16(link);
    while e != di {
        if e >= m.r16(this + 0xf4) {
            break;
        }
        link = m.r32(this + 4).wrapping_add(u32::from(e) * SHIP_SIZE) + 6;
        e = m.r16(link);
    }
    let s = m.r32(this + 4).wrapping_add(k * SHIP_SIZE);
    let nx = m.r16(s + 6);
    m.w16(link, nx);
    let s = m.r32(this + 4).wrapping_add(k * SHIP_SIZE);
    m.w16(s + 6, 0xffff);
    let mut e = u32::from(m.r16(head));
    while (e as i32) < i32::from(m.r16(this + 0xf4)) {
        let s = m.r32(this + 4).wrapping_add(e * SHIP_SIZE);
        for o in [0x50, 0x52] {
            if m.r16(s + o) == di {
                m.w16(s + o, 0xffff);
            }
        }
        e = u32::from(m.r16(s + 6));
    }
    let w = m.r32(0x6cbdd8);
    let r = host.call(m, 0x44f940, w, &[k]);
    (r & 0xffff_ff00) | 1
}

/// 0x005062F0 `create_convoy_for_ship` (`thiscall`: gestor en ECX, `+8` registros de convoy de 0x3C, `+0xE4` primer libre, `+0xF6`
/// capacidad; argumentos: `head`, dirección de la cabeza de una lista de 16 bits (en los llamadores, un local de su pila), y el barco
/// `k`). Si no queda sitio crece con `0x509AE0`; toma el primer registro libre, lo enlaza delante de `*head`, copia del barco el dueño,
/// la ciudad, el capitán y la posición, marca `+0x18 |= 0x400`, apunta `ship+8` al convoy, avisa con `0x501680(k, 2)` y deja el tipo
/// `0x100` y el estado del barco (`ship+0x134`). Devuelve la base de barcos.
pub fn create_convoy_for_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let head = m.r32(f.arg(0));
    let k = m.r32(f.arg(1));
    if m.r16(this + 0xe4) >= m.r16(this + 0xf6) {
        host.call(m, 0x509ae0, this, &[]);
    }
    let bp = m.r16(this + 0xe4);
    let rec = m.r32(this + 8).wrapping_add(u32::from(bp) * 0x3c);
    let v = m.r16(rec + 8);
    m.w16(this + 0xe4, v);
    let v = m.r16(head);
    m.w16(rec + 8, v);
    m.w16(head, bp);
    let off = (k & 0xffff) * SHIP_SIZE;
    let s = m.r32(this + 4).wrapping_add(off);
    m.w16(rec + 0xa, k as u16);
    let v = m.r8(s);
    m.w8(rec, v);
    let v = m.r8(s + 0xe);
    m.w8(rec + 1, v);
    let v = m.r8(s + 0x39);
    m.w16(rec + 0x1c, u16::from(v));
    let v = m.r8(s + 0x37);
    m.w8(rec + 0x1e, v);
    let v = m.r8(s + 0x38);
    m.w8(rec + 0x1f, v);
    let v = m.r32(s + 0x10);
    m.w32(rec + 4, v);
    let v = m.r32(rec + 0x18) | 0x400;
    m.w32(rec + 0x18, v);
    let ships = m.r32(this + 4);
    m.w16(ships.wrapping_add(off) + 8, bp);
    host.call(m, 0x501680, rec, &[k, 2]);
    let ships = m.r32(this + 4);
    let st = m.r16(ships.wrapping_add(off) + 0x134);
    m.w16(rec + 0x16, 0x100);
    m.w16(rec + 0x12, st);
    ships
}

/// 0x00534750 `map_discovery_construct` (objeto en ECX). Pone a cero `+0x60` y los dos bloques de 0x30 bytes (`memset` del CRT). Si aún
/// no hay plantilla de descubrimiento (`0x6DF2E0`), la lee del archivo `0x6DCCC8` con `0x6389EE("…", &dato, &tamaño)` (dos locales en
/// `esp-4`/`esp-8`), guarda `tamaño/4` en `0x6DF2E4`, copia el dato a un bloque nuevo (`malloc` de `tamaño` redondeado a par, `memcpy`)
/// y libera el recurso (`0x6389F4`). Devuelve `this`.
pub fn map_discovery_construct(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    m.w32(this + 0x60, 0);
    for o in 0..0x60 {
        m.w8(this + o, 0);
    }
    if m.r32(0x6df2e0) != 0 {
        return this;
    }
    let arc = m.r32(0x6dccc8);
    let (l_data, l_size) = (f.at(4), f.at(8));
    let r = host.call(m, 0x6389ee, 0, &[arc, 0x6c_363c, l_data, l_size]);
    if r == 0 {
        return this;
    }
    let size = m.r32(l_size);
    m.w32(0x6df2e4, (size >> 2) & 0xffff);
    let p = host.call(m, MALLOC, 0, &[(size >> 1) * 2]);
    m.w32(0x6df2e0, p);
    let src = m.r32(l_data);
    let n = m.r32(l_size);
    for o in 0..n {
        let b = m.r8(src + o);
        m.w8(p + o, b);
    }
    let arc = m.r32(0x6dccc8);
    host.call(m, 0x6389f4, 0, &[arc, r]);
    this
}

/// 0x00531BD0 `build_town_distance_matrix` (`GameWorld*` en ECX: `+0x10` ciudades, `+0x18` índices, `+0x6C` matriz de bytes n×n).
/// Reserva la matriz nueva, copia la anterior (de `n-1` filas) y libera la vieja, y para cada pareja nueva de ciudades pide al
/// planificador `0x445010` una ruta entre sus posiciones (`0x6DDBB0/4 + ciudad·0x34`) con un par local `{puntero, 2}` en `esp-0x14`
/// que apunta a dos puntos en `esp-0xC`; la ruta sale en el local `esp-0x1C`. Guarda la distancia `0x516930(ruta)` en los dos sentidos
/// y libera la ruta si el planificador la creó. EAX no definido.
pub fn build_town_distance_matrix(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let (l_pair, l_route, l_new) = (f.at(0x14), f.at(0x1c), f.at(0x18));
    let pts = f.at(0xc);
    m.w32(l_pair, pts);
    m.w32(l_pair + 4, 2);
    let n = m.r32(this + 0x10);
    let nb = host.call(m, MALLOC, 0, &[n.wrapping_mul(n)]);
    m.w32(l_new, nb);
    let old = m.r32(this + 0x6c);
    let mut i0 = 0u32;
    if old != 0 {
        let w = m.r32(this + 0x10).wrapping_sub(1);
        let mut src = 0u32;
        let mut r = 0u32;
        while w != 0 {
            let dst = m.r32(this + 0x10).wrapping_mul(r).wrapping_add(m.r32(l_new));
            let s = m.r32(this + 0x6c).wrapping_add(src);
            for o in 0..w {
                let b = m.r8(s + o);
                m.w8(dst + o, b);
            }
            r += 1;
            src += w;
            if r >= w {
                break;
            }
        }
        let o = m.r32(this + 0x6c);
        host.call(m, FREE, 0, &[o]);
        i0 = w;
    }
    let nb = m.r32(l_new);
    m.w32(this + 0x6c, nb);
    let mut i = i0;
    while i < m.r32(this + 0x10) {
        let n = m.r32(this + 0x10);
        let mat = m.r32(this + 0x6c);
        m.w8(mat.wrapping_add((n + 1).wrapping_mul(i)), 0);
        let t = u32::from(m.r8(this + 0x18 + i));
        let v = m.r16(0x6ddbb0 + t * 0x34);
        m.w16(pts, v);
        let v = m.r16(0x6ddbb4 + t * 0x34);
        m.w16(pts + 2, v);
        for j in 0..i {
            let t = u32::from(m.r8(this + 0x18 + j));
            let x = m.r16(0x6ddbb0 + t * 0x34);
            let y = m.r16(0x6ddbb4 + t * 0x34);
            m.w16(pts + 4, x);
            m.w16(pts + 6, y);
            let pl = m.r32(0x6cbdc8);
            let args = peek(m, &[l_pair, l_route], l_pair, 5);
            let made = host.call(m, 0x445010, pl, &args) & 0xff;
            let route = m.r32(l_route);
            let d = host.call(m, 0x516930, 0, &[route]) as u8;
            let n = m.r32(this + 0x10);
            let mat = m.r32(this + 0x6c);
            m.w8(mat.wrapping_add(n.wrapping_mul(i).wrapping_add(j)), d);
            let mat = m.r32(this + 0x6c);
            m.w8(mat.wrapping_add(n.wrapping_mul(j).wrapping_add(i)), d);
            if made != 0 {
                let route = m.r32(l_route);
                let q = m.r32(route);
                host.call(m, FREE, 0, &[q]);
                let route = m.r32(l_route);
                host.call(m, FREE, 0, &[route]);
            }
        }
        i += 1;
    }
    0
}

/// 0x0052BBE0 `trade_center_deserialize` (`thiscall`: centro de comercio en ECX; argumentos `src` y versión). Copia la cabecera (8 bytes y
/// tres `memcpy` de 4, 8 y 8), vacía la lista circular de ofertas (`+0x24` centinela, `+0x28` recuento) con el iterador `0x52DE00`, que
/// usa los dos argumentos de la pila como locales, y, desde la versión 0x72, lee el recuento `+0x1C` y vuelve a crear las ofertas (nodos
/// de 0x18 bytes con 16 de datos, insertados al final). Devuelve el puntero de lectura.
pub fn trade_center_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let mut src = m.r32(f.arg(0));
    let ver = m.r32(f.arg(1));
    for o in 0..0x1c {
        let b = m.r8(src + o);
        m.w8(this + o, b);
    }
    src += 0x1c;
    let newer = ver > 0x71;
    if newer {
        for o in 0..2 {
            let b = m.r8(src + o);
            m.w8(this + 0x1c + o, b);
        }
        src += 2;
    }
    let head = m.r32(this + 0x24);
    let mut node = m.r32(head);
    while node != head {
        // 0x52DE00: it = [it]; *out = antiguo.
        m.w32(f.arg(0), node);
        let next = m.r32(node);
        m.w32(f.arg(0), next);
        m.w32(f.arg(1), node);
        let e = m.r32(f.arg(1));
        let prev = m.r32(e + 4);
        let nx = m.r32(e);
        m.w32(prev, nx);
        let nx = m.r32(e);
        let prev = m.r32(e + 4);
        m.w32(nx + 4, prev);
        host.call(m, FREE, 0, &[e]);
        let c = m.r32(this + 0x28).wrapping_sub(1);
        m.w32(this + 0x28, c);
        node = next;
    }
    if !newer {
        m.w16(this + 0x1c, 0);
        return src;
    }
    m.w32(f.arg(0), 0);
    if m.r16(this + 0x1c) == 0 {
        return src;
    }
    loop {
        let h = m.r32(this + 0x24);
        let mut tail = m.r32(h + 4);
        let p = host.call(m, MALLOC, 0, &[0x18]);
        m.w32(p, h);
        if tail == 0 {
            tail = p;
        }
        m.w32(p + 4, tail);
        m.w32(h + 4, p);
        let pv = m.r32(p + 4);
        m.w32(pv, p);
        let d = p.wrapping_add(8);
        if d != 0 {
            for o in 0..4 {
                let v = m.r32(src + 4 * o);
                m.w32(d + 4 * o, v);
            }
        }
        let c = m.r32(this + 0x28).wrapping_add(1);
        m.w32(this + 0x28, c);
        src += 0x10;
        let i = m.r32(f.arg(0)).wrapping_add(1);
        m.w32(f.arg(0), i);
        if (i as i32) >= i32::from(m.r16(this + 0x1c)) {
            return src;
        }
    }
}

/// 0x00530470 `create_merchant` (`thiscall`: contenedor del mundo en ECX, `+0xA` ciudades usadas, `+0x80` capacidad, `+0x78` vector de
/// 0x650 bytes con el recuento en `-4`; argumentos: dueño (byte), tipo (16 bits), dos bytes). Marco SEH (`0x66344F`) con mensaje local
/// en `esp-0x1C` y el bloque nuevo en `esp-0x20` (el estado del marco pasa por 1 mientras se construye el vector). Si está lleno crece a
/// 0x2D (o +5): `malloc`, constructor de vector `0x639E9C`, copia de las antiguas (`memcpy`), destructor `0x639DA8` y `free`; sin memoria
/// devuelve EAX con los 16 bits bajos a 0xFFFF. Inicializa la ciudad nueva (`0x4F72C0`, dinero 1.000.000, tipo y bytes), con tipo 0
/// envía el aviso `0xE` y, si no, marca `+0x468 = 1 << dueño`. Devuelve en AX el índice de la ciudad.
pub fn create_merchant(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let msg = f.at(0x1c);
    let a: Vec<u32> = (0..4).map(|i| m.r32(f.arg(i))).collect();
    f.seh_enter(m, 0x66_344f);
    host.call(m, 0x51a850, msg, &[]);
    f.seh_state(m, 0);
    let cap = m.r16(this + 0x80);
    if m.r16(this + 0xa) >= cap {
        m.w16(this + 0xa, cap);
        let nc = if cap < 0x28 { 0x2d } else { cap.wrapping_add(5) };
        m.w16(this + 0x80, nc);
        let nc = u32::from(nc);
        let p = host.call(m, MALLOC, 0, &[nc * 0x650 + 4]);
        m.w32(f.at(0x20), p);
        m.w8(f.at(4), 1);
        let mut nb = 0;
        if p != 0 {
            m.w32(p, nc);
            nb = p + 4;
            // ECX residual: el tamaño que quedó del `malloc` (los dos iteradores del CRT son `stdcall`).
            host.call(m, 0x639e9c, nc * 0x650 + 4, &[nb, 0x650, nc, 0x4f_3eb0, 0x4f_3fa0]);
        }
        m.w8(f.at(4), 0);
        if nb == 0 {
            f.seh_state(m, 0xffff_ffff);
            let r = host.call(m, 0x51a880, msg, &[]);
            f.seh_leave(m);
            return r | 0xffff;
        }
        let old = m.r32(this + 0x78);
        if old != 0 {
            let n = u32::from(m.r16(this + 0xa)) * 0x650;
            for o in 0..n {
                let b = m.r8(old + o);
                m.w8(nb + o, b);
            }
            let old = m.r32(this + 0x78);
            if old != 0 {
                let c = m.r32(old - 4);
                host.call(m, 0x639da8, c, &[old, 0x650, c, 0x4f_3fa0]);
                host.call(m, FREE, 0, &[old - 4]);
            }
        }
        m.w32(this + 0x78, nb);
    }
    let bp = m.r16(this + 0xa);
    let bl = a[0] as u8;
    m.w16(this + 0xa, bp.wrapping_add(1));
    let s = m.r32(this + 0x78).wrapping_add(u32::from(bp) * 0x650);
    m.w8(s + 0x19, bl);
    host.call(m, 0x4f72c0, s, &[]);
    if a[1] as u16 == 0 {
        m.w8(msg + 5, bl);
        m.w8(msg + 4, 0xe);
        m.w16(msg + 8, bp);
        m.w16(msg + 0xa, 0xffff);
        let o = u32::from(m.r8(s + 0x19));
        let b2 = m.r8(s + o + 0x39c);
        let r = host.call(m, 0x4f9430, s, &[]);
        m.w16(msg + 0xe, 1);
        let lvl = u16::from(b2 & 7).wrapping_add(u16::from(r as u8) * 8);
        m.w16(msg + 0xc, lvl);
        let args = peek(m, &[0xffff_fffe, msg], msg, 4);
        host.call(m, 0x4d6530, MSGS, &args);
    } else {
        m.w32(s + 0x468, 1u32 << (bl & 31));
    }
    m.w32(s + 0x53c, 0);
    m.w32(s + 0x540, 0);
    m.w16(s + 0x544, 0);
    m.w16(s + 8, a[1] as u16);
    m.w32(s, 1_000_000);
    m.w16(s + 0xe, 0xffff);
    m.w8(s + 0x546, a[2] as u8);
    m.w8(s + 0x547, a[3] as u8);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x51a880, msg, &[]);
    f.seh_leave(m);
    (r & 0xffff_0000) | u32::from(bp)
}

const CSTR_DTOR: u32 = 0x64f253;
const CSTR_CTOR: u32 = 0x64f1de;
const VEC_CTOR: u32 = 0x639e9c;
const VEC_DTOR: u32 = 0x639da8;

/// 0x0052F550 `game_world_construct` (`GameWorld*` en ECX). Marco SEH (`0x6633AE`), `this` en `esp-0x10`: construye `+0xE8`
/// (`0x535080`) y el vector de 16 elementos de 0x2C en `+0xB10` (iterador `stdcall` del CRT con `0x533810`/`0x533840`), y pone los
/// campos de cabecera a su valor inicial (`+0x88`/`+0x8F` = 0xFF, `+0x8B` = 0x5A, el resto a cero). El estado del marco queda en 0.
pub fn game_world_construct(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66_33ae);
    m.w32(f.at(0x10), this);
    host.call(m, 0x535080, this + 0xe8, &[]);
    f.seh_state(m, 0);
    host.call(m, VEC_CTOR, 0, &[this + 0xb10, 0x2c, 0x10, 0x53_3810, 0x53_3840]);
    m.w8(this + 0x88, 0xff);
    m.w8(this + 0x8f, 0xff);
    for o in [0, 1, 0x89, 0x8c, 0x8a] {
        m.w8(this + o, 0);
    }
    m.w16(this + 2, 0);
    m.w8(this + 0x8b, 0x5a);
    for o in [0x14, 0xc, 0x10, 0x68, 0x6c, 0x70, 0x74, 0x78, 0xe4, 0x84] {
        m.w32(this + o, 0);
    }
    for o in [6, 0x7c, 8, 0x7e, 0xa, 0x80] {
        m.w16(this + o, 0);
    }
    f.seh_leave(m);
    this
}

/// 0x0052F620 `game_world_destruct` (`GameWorld*` en ECX). Marco SEH (`0x6633DE`, estado 1→0→-1): libera los bloques `+0x70`, `+0x74`,
/// `+0x6C`, los vectores de ciudades (`+0x78`, 0x650 bytes) y de comerciantes (`+0x68`, 0x9F8, destructor `0x51B290`), el objeto `+0xE4`
/// (`0x533E80`), la plantilla global `0x6DF2E0`, el vector `+0xB10` y `+0xE8` (`0x5350B0`); deja a cero los recuentos. EAX del último.
pub fn game_world_destruct(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66_33de);
    m.w32(f.at(0x10), this);
    f.seh_state(m, 1);
    let free_field = |m: &mut dyn Mem, host: &mut dyn OpsHost, o: u32| {
        let p = m.r32(this + o);
        if p != 0 {
            host.call(m, FREE, 0, &[p]);
            m.w32(this + o, 0);
        }
    };
    free_field(m, host, 0x70);
    m.w16(this + 6, 0);
    m.w16(this + 0x7c, 0);
    free_field(m, host, 0x74);
    m.w16(this + 8, 0);
    m.w16(this + 0x7e, 0);
    let vec_free = |m: &mut dyn Mem, host: &mut dyn OpsHost, o: u32, size: u32, dtor: u32| {
        let p = m.r32(this + o);
        if p != 0 {
            let n = m.r32(p - 4);
            host.call(m, VEC_DTOR, 0, &[p, size, n, dtor]);
            host.call(m, FREE, 0, &[p - 4]);
            m.w32(this + o, 0);
        }
    };
    vec_free(m, host, 0x78, 0x650, 0x4f_3fa0);
    m.w16(this + 0x80, 0);
    m.w16(this + 0xa, 0);
    let e = m.r32(this + 0xe4);
    if e != 0 {
        host.call(m, 0x533e80, e, &[]);
        m.w32(this + 0xe4, 0);
    }
    let g = m.r32(0x6df2e0);
    if g != 0 {
        host.call(m, FREE, 0, &[g]);
    }
    m.w32(0x6df2e0, 0);
    vec_free(m, host, 0x68, 0x9f8, 0x51_b290);
    m.w32(this + 0x10, 0);
    free_field(m, host, 0x6c);
    m.w8(f.at(4), 0);
    host.call(m, VEC_DTOR, 0, &[this + 0xb10, 0x2c, 0x10, 0x53_3840]);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x5350b0, this + 0xe8, &[]);
    f.seh_leave(m);
    r
}

/// 0x0054A600 `operations_construct` (`Operations*` en ECX). Marco SEH (`0x6639D2`, estados 0..3), `this` en `esp-0x10`: cadena vacía en
/// `+0x40`, construye `+0x48` (`0x543EA0`), las 52 órdenes de `+0x484` (cabecera de `0x535740`), las dos colas `+0x89C`/`+0x8B8`
/// (`0x543920`), pone los campos a cero (`+0x2C`, `+0x24`, `+0x28` a -1) y construye la cadena `+0x40` (`0x64F1DE`). Devuelve `this`.
pub fn operations_construct(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66_39d2);
    let empty = m.r32(0x6c7ccc);
    m.w32(f.at(0x10), this);
    m.w32(this + 0x40, empty);
    f.seh_state(m, 0);
    host.call(m, 0x543ea0, this + 0x48, &[]);
    m.w8(f.at(4), 1);
    for k in 0..0x34 {
        let o = this + 0x484 + k * 0x14;
        m.w32(o, 0xffff_ffff);
        for q in [4, 8, 0xc, 0x10] {
            m.w32(o + q, 0);
        }
    }
    host.call(m, 0x543920, this + 0x89c, &[]);
    m.w8(f.at(4), 2);
    host.call(m, 0x543920, this + 0x8b8, &[]);
    m.w8(f.at(4), 3);
    m.w32(this + 0x10, 0);
    m.w32(this, 0);
    m.w32(this + 0x2c, 0xffff_ffff);
    for o in [8, 0x44, 0x30, 0x38, 0x46c, 0x470, 0x894, 0x898, 0x14, 0x18, 0x3c, 0xc] {
        m.w32(this + o, 0);
    }
    host.call(m, CSTR_CTOR, this + 0x40, &[]);
    m.w32(this + 0x24, 0xffff_ffff);
    m.w32(this + 0x28, 0xffff_ffff);
    f.seh_leave(m);
    this
}

/// 0x0054A6E0 `operations_destroy` (`Operations*` en ECX). Marco SEH (`0x663A0E`) con `this` en `esp-0x10` y un indicador de byte en
/// `esp-0x11`. Libera los búferes de red: si hay tabla de clientes `+0x44`, cierra cada conexión con la función de la IAT `0x66A2FC`
/// (las 7 entradas activas de 0x24 bytes del anfitrión o la única del cliente) y la libera; libera `+8`, cierra el socket `+0x3C`
/// (`0x66A2F8`, `0x66A2F4`) y llama a `0x66A334` si había red; libera `+0x14`, `+0x18` y el mapa propio `+0xC` (vector de 40 cadenas en
/// `+0x1BC`), destruye las colas, `+0x48` y la cadena `+0x40`. Las llamadas a Windows se enganchan por su entrada de la IAT.
pub fn operations_destroy(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let flag = f.at(0x11);
    f.seh_enter(m, 0x66_3a0e);
    m.w32(f.at(0x10), this);
    f.seh_state(m, 3);
    m.w8(flag, 0);
    let t = m.r32(this + 0x44);
    if t != 0 {
        m.w8(flag, 1);
        let close = m.r32(0x66a2fc);
        if m.r32(this + 0x2c) == 0 {
            let mut o = 0x24;
            while o < 0x120 {
                let e = m.r32(this + 0x44) + o;
                if m.r32(e + 0x10) != 0 {
                    let sock = m.r32(this + 0x3c);
                    host.call(m, close, 0, &[sock, e]);
                }
                o += 0x24;
            }
        } else {
            let sock = m.r32(this + 0x3c);
            host.call(m, close, 0, &[sock, t]);
        }
        let t = m.r32(this + 0x44);
        host.call(m, FREE, 0, &[t]);
        m.w32(this + 0x44, 0);
    }
    let p = m.r32(this + 8);
    if p != 0 {
        m.w8(flag, 1);
        host.call(m, FREE, 0, &[p]);
        m.w32(this + 8, 0);
    }
    if m.r32(this + 0x38) != 0 || m.r8(flag) != 0 {
        let s = m.r32(this + 0x3c);
        if s != 0 {
            if m.r32(this + 0x10) != 0 {
                m.w32(this + 0x10, 0);
                let fp = m.r32(0x66a2f8);
                host.call(m, fp, 0, &[s]);
            }
            let s = m.r32(this + 0x3c);
            m.w32(this + 0x3c, 0);
            let fp = m.r32(0x66a2f4);
            host.call(m, fp, 0, &[s]);
        }
        let fp = m.r32(0x66a334);
        host.call(m, fp, 0, &[]);
    }
    for o in [0x14, 0x18] {
        let p = m.r32(this + o);
        if p != 0 {
            host.call(m, FREE, 0, &[p]);
            m.w32(this + o, 0);
        }
    }
    let mp = m.r32(this + 0xc);
    if mp != 0 {
        host.call(m, VEC_DTOR, 0, &[mp + 0x1bc, 4, 0x28, CSTR_DTOR]);
        host.call(m, FREE, 0, &[mp]);
        m.w32(this + 0xc, 0);
    }
    m.w32(this + 0x10, 0);
    m.w8(f.at(4), 2);
    host.call(m, 0x543940, this + 0x8b8, &[]);
    m.w8(f.at(4), 1);
    host.call(m, 0x543940, this + 0x89c, &[]);
    m.w8(f.at(4), 0);
    host.call(m, 0x543ee0, this + 0x48, &[]);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, CSTR_DTOR, this + 0x40, &[]);
    f.seh_leave(m);
    r
}

/// 0x00546540 `operations_destructor` (`Operations*` en ECX). Marco SEH (`0x663822`), `this` en `esp-0x10`: libera `+0x930` (sin
/// ponerlo a cero), destruye el vector de 8 cadenas de `+0x8E0` y llama a `operations_destroy` (0x54A6E0). EAX de esta.
pub fn operations_destructor(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66_3822);
    m.w32(f.at(0x10), this);
    let p = m.r32(this + 0x930);
    if p != 0 {
        host.call(m, FREE, 0, &[p]);
    }
    f.seh_state(m, 0);
    host.call(m, VEC_DTOR, 0, &[this + 0x8e0, 4, 8, CSTR_DTOR]);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x54a6e0, this, &[]);
    f.seh_leave(m);
    r
}

/// 0x00544830 `custom_map_construct` (mapa propio en ECX). Marco SEH (`0x663758`), `this` en `esp-0x10`: vector de 40 cadenas en
/// `+0x1BC` (iterador del CRT con `0x4103C0`/`0x64F253`), `+0 = 0` y construye cada cadena con `0x64F1DE`. Devuelve `this`.
pub fn custom_map_construct(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66_3758);
    m.w32(f.at(0x10), this);
    host.call(m, VEC_CTOR, 0, &[this + 0x1bc, 4, 0x28, 0x41_03c0, CSTR_DTOR]);
    f.seh_state(m, 0);
    m.w32(this, 0);
    for k in 0..0x28 {
        host.call(m, CSTR_CTOR, this + 0x1bc + 4 * k, &[]);
    }
    f.seh_leave(m);
    this
}

/// 0x004E94A4 `st_unfreeze_port` (tarea programada; planificador en ECX). Marco SEH (`0x662D00`) y dos locales en `esp-0x14`/`esp-0x10`.
/// La ciudad `rec+8` (< `0x6DE4B0`) pierde el bit 0x04000000 de `town+0x2C8`; después se forma el aviso: CString local (`esp-0x14`)
/// iniciado con la cadena vacía `0x6C7CCC`, `0x64DF80(&local, [0x6B05BC], nombre[0x6DDA00 + t·4])` (cdecl de 3 que solo retira 8 bytes:
/// el tercer argumento queda como hueco de un CString pasado por valor), copia `0x64EFC8` en ese hueco (`esp-0x18`), entrega a
/// `0x42B4C0` (ECX `[0x6CBB40]`) y destructor `0x64F253` del local. Devuelve EAX del destructor (o el índice si la ciudad no existe).
pub fn st_unfreeze_port(m: &mut dyn Mem, host: &mut dyn OpsHost, sched: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x662d00);
    let rec = m.r32(sched).wrapping_add(u32::from(m.r16(sched + 8)) * 0x18);
    let t = m.r32(rec + 8);
    if t >= m.r32(0x6de4b0) {
        f.seh_leave(m);
        return t;
    }
    let tw = m.r32(TOWNS).wrapping_add(t.wrapping_mul(TOWN_SIZE)) + 0x2c8;
    let v = m.r32(tw) & 0xfbff_ffff;
    m.w32(tw, v);
    let s = m.r32(0x6c7ccc);
    let local = f.at(0x14);
    m.w32(local, s);
    let name = m.r32(t.wrapping_mul(4).wrapping_add(0x6dda00));
    let fmt = m.r32(0x6b05bc);
    f.seh_state(m, 0);
    host.call(m, 0x64df80, 0, &[local, fmt, name]);
    let slot = f.at(0x18);
    m.w32(slot, name);
    m.w32(f.at(0x10), slot);
    host.call(m, 0x64efc8, slot, &[local]);
    let arg = m.r32(slot);
    let ui = m.r32(0x6cbb40);
    host.call(m, 0x42b4c0, ui, &[arg]);
    f.seh_state(m, 0xffff_ffff);
    m.w32(slot, 0x4e954e);
    let r = host.call(m, CSTR_DTOR, local, &[]);
    f.seh_leave(m);
    r
}

/// 0x004E8684 `st_matchmaker_candidate_dossier` (stdcall de 1: carga útil del registro). Marco SEH (`0x662CC2`) y mensaje local de 0x10
/// bytes en `esp-0x20` (`0x51A850`/`0x51A880`). Si la ciudad `p+0xB` existe (< `0x6DE4AA`), consulta `0x5303C0` y compone el expediente
/// del candidato: rasgo `(p+4 % 3)·5 + (p+4 / 3 % 5)` mirado en `0x6726BC` (si la ciudad tiene `+0x1A`) o `0x6726CB`, edad `p+6` módulo
/// `0x6DDB72` (con bit 0x80) o `0x6DDB70`, fortuna `p+8 % 0x6DDB74`, ciudad `p+0xA % 0x6DE4B0` y los bytes `p+0xC..0xE`; lo envía con
/// código 0x22 por `0x4D6530(p+0xB, &msg)`. El original guarda `0x6DDB72` en la mitad baja de su propio argumento. Devuelve EAX del
/// destructor.
pub fn st_matchmaker_candidate_dossier(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x662cc2);
    let msg = f.at(0x20);
    host.call(m, 0x51a850, msg, &[]);
    let p = m.r32(f.arg(0));
    f.seh_state(m, 0);
    let town = m.r8(p + 0xb);
    if u16::from(town) < m.r16(TOWN_COUNT) {
        let tp = host.call(m, 0x5303c0, WORLD, &[u32::from(town)]);
        let rich = m.r8(tp + 0x1a) != 0;
        let a = u32::from(m.r16(p + 4));
        let r3 = (a % 3) as u8;
        let r5 = (a / 3 % 5) as u8;
        m.w8(f.at(0x16), r3);
        let trait_ = r5.wrapping_add(r3.wrapping_mul(5));
        m.w8(f.at(0x12), trait_);
        let n = m.r32(0x6de4b0);
        let home = (u32::from(m.r8(p + 0xa)) % n) as u8;
        m.w8(f.at(0x15), home);
        let k = u32::from(trait_) % 15;
        let age = u32::from(m.r16(p + 6));
        let age_b = if rich {
            let d = m.r16(0x6ddb72);
            let lo = m.r32(f.arg(0)) & 0xffff_0000 | u32::from(d);
            m.w32(f.arg(0), lo);
            let b = m.r8(0x6726bc + k);
            m.w8(f.at(0x12), b);
            ((age % u32::from(d)) as u8) | 0x80
        } else {
            let d = m.r16(0x6ddb70);
            let b = m.r8(0x6726cb + k);
            m.w8(f.at(0x12), b);
            (age % u32::from(d)) as u8
        };
        m.w8(f.at(0x14), age_b);
        m.w8(f.at(0x11), 0xe);
        let w = (u32::from(m.r16(p + 8)) % u32::from(m.r16(0x6ddb74))) as u8;
        let e = m.r8(p + 0xe);
        m.w8(f.at(0x17), e);
        let d = m.r8(p + 0xd);
        m.w8(f.at(0x18), d);
        m.w8(f.at(0x1c), 0x22);
        m.w8(f.at(0x13), w);
        let c = m.r8(p + 0xc);
        m.w8(f.at(0x1b), c);
        let tb = u32::from(m.r8(p + 0xb));
        let args = peek(m, &[tb, msg], msg, 4);
        host.call(m, 0x4d6530, MSGS, &args);
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x51a880, msg, &[]);
    f.seh_leave(m);
    r
}

/// 0x004E2144 `st_update_shipyard_experience` (sin argumentos). Marco SEH (`0x662B06`), índice de ciudad en `esp-0x28`, desplazamiento
/// en `esp-0x24`, temporal `f32` en `esp-0x20` y mensaje local en `esp-0x1C`. Para cada ciudad suma `ship+0x10` de los barcos de su
/// cadena del astillero (`town+0x81C`, siguiente en `ship+6`), acumula `town+0x814` en `town+0x810` (y lo pone a 0) y actualiza
/// `town+0x818 = (f32)(suma/2000 + 0x814·0x6AFF5F81>>45) / [0x672840] + town+0x818 · [0x67283C]` (x87 de 53 bits, igual a `f64`). Después
/// sube el nivel de cada uno de los cuatro tipos (`town+0x824+j`) si `town+0x810` supera `2800 · u16[0x673818 + (nivel+1 + 4j)·2]`; al
/// estrenar el tipo 3 (nivel negativo) avisa con el mensaje 0x81 (`0x4D6530(-1, &msg)`) si `town+0x2C8 == 0` y `town+0x76C & 0x20000`.
/// Devuelve el índice final (el número de ciudades).
pub fn st_update_shipyard_experience(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x662b06);
    m.w32(f.at(0x28), 0);
    if m.r32(0x6de4b0) == 0 {
        f.seh_leave(m);
        return 0;
    }
    m.w32(f.at(0x24), 0);
    let msg = f.at(0x1c);
    loop {
        let off = m.r32(f.at(0x24));
        let t = off.wrapping_add(m.r32(TOWNS));
        let cnt = m.r32(SHIP_COUNT) & 0xffff;
        let mut sum = 0i32;
        let mut i = u32::from(m.r16(t + 0x81c));
        while i < cnt {
            let s = m.r32(SHIP_ARRAY).wrapping_add(i * SHIP_SIZE);
            i = u32::from(m.r16(s + 6));
            sum = sum.wrapping_add(m.r32(s + 0x10) as i32);
        }
        let add = m.r32(t + 0x814);
        let v = (sum / 2000).wrapping_add(((u64::from(add) * 0x6aff_5f81) >> 45) as i32);
        m.w32(t + 0x814, 0);
        let tot = m.r32(t + 0x810).wrapping_add(add);
        m.w32(t + 0x810, tot);
        let vf = v as f32;
        m.w32(f.at(0x20), vf.to_bits());
        let q = f64::from(vf) / f64::from(f32::from_bits(m.r32(0x672840)));
        let p = f64::from(f32::from_bits(m.r32(t + 0x818))) * f64::from(f32::from_bits(m.r32(0x67283c)));
        m.w32(t + 0x818, ((q + p) as f32).to_bits());
        for j in 0..4u32 {
            let lv = m.r8(t + j + 0x824) as i8;
            let nx = i32::from(lv) + 1;
            if nx >= 4 {
                continue;
            }
            let idx = (nx as u32).wrapping_add(j * 4);
            let th = u32::from(m.r16(idx.wrapping_mul(2).wrapping_add(0x673818))) * 2800;
            if m.r32(t + 0x810) <= th {
                continue;
            }
            if lv < 0 && j == 3 {
                host.call(m, 0x51a850, msg, &[]);
                f.seh_state(m, 0);
                if m.r32(t + 0x2c8) == 0 && m.r32(t + 0x76c) & 0x20000 != 0 {
                    let ti = m.r8(f.at(0x28));
                    m.w8(f.at(0x18), 0x81);
                    m.w8(f.at(0x17), ti);
                    let args = peek(m, &[0xffff_ffff, msg], msg, 4);
                    host.call(m, 0x4d6530, MSGS, &args);
                }
                f.seh_state(m, 0xffff_ffff);
                host.call(m, 0x51a880, msg, &[]);
            }
            m.w8(t + j + 0x824, nx as u8);
        }
        let i = m.r32(f.at(0x28)) + 1;
        m.w32(f.at(0x28), i);
        m.w32(f.at(0x24), off.wrapping_add(TOWN_SIZE));
        if i >= m.r32(0x6de4b0) {
            f.seh_leave(m);
            return i;
        }
    }
}

/// 0x004DC1F0 `st_hanse_diet_vote` (tarea de la Dieta de la Hansa; planificador en ECX, stdcall de 1: índice de un registro del
/// planificador, cuyo hueco se reutiliza como local). Marco SEH (`0x662A8C`), mensaje local en `esp-0x1C` (`0x51A850`/`0x51A880`). El
/// byte bajo de `rec+0` es la fase:
/// - **0**: si la ciudad sede (`rec+9`) tiene `town+0x2C8 & 0x40` no hace nada (0). Si no, recorre los comerciantes (`0x5303C0`): los
///   que no son jugador muerto (`+8 == 0`) y cuyo rango en la sede (`+0x39C + [+0x19]`) es ≥ 4 quedan marcados en `rec+0x16`
///   (bit por orden), invitados con `0x548CA0` y suman 1/2/3 votos (rango 4–5/6/7+) en el hueco del argumento. Con algún invitado
///   anuncia la reunión (`0x469380(0)`, dos CString por valor de `0x6DD754`/`0x6DD755`, `0x469AF0(0x1E, …)`). Cuenta en `rec+0xF` las
///   ciudades con regente (`town+0x6DC..+0x6DF` < `0x6DE4AA`), reparte `0x26 − votos` (o 10 si son ≥ 0x1C) en sí/no según la
///   propuesta (`rec+0xB`, `rec+0xE`) y la suma de `town+4` módulo 5, limpia los recuentos `rec+0x12..0x15` y avanza 10 (1).
/// - **1..0xE7**: un votante de la rejilla `[TOWNS + (fase&0xF)·4 + 4] & (0xE8−fase)` pasa de pendiente a emitido (`rec+0x10/0x12` o
///   `+0x11/0x13` según la paridad) y un voto extra `rec+0x15 → rec+0x14`; avanza 1 (1).
/// - **0xE8**: suma pendientes a emitidos y avanza 1 (1).
/// - **> 0xE8**: envía el resultado (código 0x0B, `0x4D6530(-1, &msg)`). Si ganan los síes ejecuta la propuesta `rec+0xE`: 0–2
///   reprograman el registro del argumento (+0x100, tarea 0x34/0x24/0x23, `memset(+8, 0xFF, 8)`) y devuelven 1; 3–5 reparten
///   160.000/120.000/80.000 (como mucho `town+0x700` de la sede) con `0x529F90` entre las demás ciudades (0). Sin mayoría, 0.
pub fn st_hanse_diet_vote(m: &mut dyn Mem, host: &mut dyn OpsHost, sched: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x662a8c);
    let rec = m.r32(sched).wrapping_add(u32::from(m.r16(sched + 8)) * 0x18);
    let phase = m.r32(rec) & 0xff;
    let msg = f.at(0x1c);
    let finish = |m: &mut dyn Mem, host: &mut dyn OpsHost, r: u32| -> u32 {
        f.seh_state(m, 0xffff_ffff);
        host.call(m, 0x51a880, msg, &[]);
        f.seh_leave(m);
        r
    };
    host.call(m, 0x51a850, msg, &[]);
    f.seh_state(m, 0);
    m.w8(f.at(0x25), 0);
    let count = |m: &mut dyn Mem| m.r32(0x6de4b0);
    let town = |m: &mut dyn Mem, i: u32| m.r32(TOWNS).wrapping_add(i.wrapping_mul(TOWN_SIZE));
    if phase == 0 {
        let seat = u32::from(m.r8(rec + 9));
        if seat < count(m) {
            let t = town(m, seat);
            if m.r32(t + 0x2c8) & 0x40 == 0x40 {
                return finish(m, host, 0);
            }
        }
        m.w8(rec + 0x16, 0);
        m.w32(f.arg(0), 0);
        let mut invited = 0u32;
        let mut i = 0u32;
        while i < u32::from(m.r16(TOWN_COUNT)) {
            let tp = host.call(m, 0x5303c0, WORLD, &[i]);
            if m.r16(tp + 8) == 0 {
                let sel = u32::from(m.r8(tp + 0x19));
                let rank = m.r8(tp + sel + 0x39c);
                m.w8(f.at(0x24), rank);
                if rank >= 4 {
                    let bit = if invited & 31 < 8 { 1u8 << (invited & 31) } else { 0 };
                    let b = m.r8(rec + 0x16) | bit;
                    m.w8(rec + 0x16, b);
                    let ax = (tp & 0xffff_0000) | u32::from(m.r8(0x6de52e));
                    host.call(m, 0x548ca0, 0x6df2f0, &[i, 0, 0, ax, 0xd, ax]);
                    invited += 1;
                    m.w8(f.at(0x25), 1);
                    let v = m.r32(f.arg(0)).wrapping_add(match rank {
                        0..=5 => 1,
                        6 => 2,
                        _ => 3,
                    });
                    m.w32(f.arg(0), v);
                }
            }
            i += 1;
        }
        if m.r8(f.at(0x25)) != 0 {
            let ui = m.r32(0x6cc7e8);
            host.call(m, 0x469380, ui, &[0]);
            let s1 = f.push(m, 0x3c, ui);
            m.w32(f.at(0x24), s1);
            host.call(m, 0x64f2c1, s1, &[0x6dd754]);
            m.w8(f.at(4), 1);
            let s2 = f.push(m, 0x40, s1);
            m.w32(f.at(0x20), s2);
            host.call(m, 0x64f2c1, s2, &[0x6dd755]);
            let cx = (s2 & 0xffff_0000) | u32::from(m.r8(0x6de52e));
            m.w8(f.at(4), 0);
            let (a3, a4) = (m.r32(s2), m.r32(s1));
            let ui = m.r32(0x6cc7e8);
            host.call(m, 0x469af0, ui, &[0x1e, cx, 0, a3, a4]);
        }
        m.w8(rec + 0xf, 0);
        let mut i = 0;
        while i < count(m) {
            let t = town(m, i);
            for j in 0..4 {
                if u16::from(m.r8(t + 0x6dc + j)) < m.r16(TOWN_COUNT) {
                    let b = m.r8(rec + 0xf).wrapping_add(1);
                    m.w8(rec + 0xf, b);
                }
            }
            i += 1;
        }
        let votes = m.r32(f.arg(0)).wrapping_add(u32::from(m.r8(rec + 0xf)));
        let pool = if votes < 0x1c { 0x26u32.wrapping_sub(votes) } else { 0xa };
        let n = count(m);
        let mut sum = 0u32;
        for i in 0..n {
            let t = town(m, i);
            sum = sum.wrapping_add(m.r32(t + 4));
        }
        let d = (u32::from(m.r8(rec + 0xb)) * 2).wrapping_sub(u32::from(m.r8(rec + 0xe)) * 2);
        let mut yes = d.wrapping_add((sum as i32 % 5) as u32).wrapping_add(7);
        if (yes as i32) > 0x10 {
            yes = 0x10;
        }
        let y = (yes.wrapping_mul(pool) >> 4) as u8;
        m.w8(rec + 0x11, y);
        m.w8(rec + 0x10, (pool as u8).wrapping_sub(y));
        for o in 0x12..0x16 {
            m.w8(rec + o, 0);
        }
        let v = m.r32(rec).wrapping_add(0xa);
        m.w32(rec, v);
        return finish(m, host, 1);
    }
    if phase < 0xe8 {
        let side = phase & 1;
        let base = m.r32(TOWNS);
        let lim = m.r32(base.wrapping_add((phase & 0xf) * 4 + 4)) & (0xe8 - phase);
        let p = rec + 0x10 + side;
        let pend = m.r8(p);
        if (u32::from(pend) as i32) > lim as i32 {
            let c = m.r8(rec + 0x12 + side).wrapping_add(1);
            m.w8(rec + 0x12 + side, c);
            m.w8(p, pend.wrapping_sub(1));
        }
        let x = m.r8(rec + 0x15);
        if x != 0 {
            let c = m.r8(rec + 0x14).wrapping_add(1);
            m.w8(rec + 0x14, c);
            m.w8(rec + 0x15, x - 1);
        }
        let v = m.r32(rec).wrapping_add(1);
        m.w32(rec, v);
        return finish(m, host, 1);
    }
    if phase == 0xe8 {
        let b13 = m.r8(rec + 0x13).wrapping_add(m.r8(rec + 0x11));
        m.w8(rec + 0x13, b13);
        let b12 = m.r8(rec + 0x12).wrapping_add(m.r8(rec + 0x10));
        let b14 = m.r8(rec + 0x14).wrapping_add(m.r8(rec + 0x15));
        m.w8(rec + 0x12, b12);
        m.w8(rec + 0x14, b14);
        let v = m.r32(rec).wrapping_add(1);
        m.w32(rec, v);
        return finish(m, host, 1);
    }
    m.w8(f.at(0x18), 0xb);
    let b8 = m.r8(rec + 8);
    m.w8(f.at(0x17), b8);
    let b9 = m.r8(rec + 9);
    m.w16(f.at(0x14), u16::from(b9));
    let w = (u16::from(m.r8(rec + 0xe)) << 8).wrapping_add(u16::from(m.r8(rec + 0xa)));
    m.w16(f.at(0x12), w);
    let b12 = m.r8(rec + 0x12);
    m.w16(f.at(0x10), u16::from(b12));
    let b13 = m.r8(rec + 0x13);
    m.w16(f.at(0xe), u16::from(b13));
    let args = peek(m, &[0xffff_ffff, msg], msg, 4);
    host.call(m, 0x4d6530, MSGS, &args);
    if m.r8(rec + 0x12) <= m.r8(rec + 0x13) {
        return finish(m, host, 0);
    }
    let seat = u32::from(m.r8(rec + 9));
    let prop = u32::from(m.r8(rec + 0xe));
    m.w32(f.at(0x24), seat);
    match prop {
        0..=2 => {
            let idx = m.r32(f.arg(0));
            let r2 = m.r32(sched).wrapping_add(idx.wrapping_mul(0x18));
            let v = m.r32(r2).wrapping_add(0x100);
            m.w32(r2, v);
            let al = match prop {
                0 => {
                    m.w16(r2 + 6, 0x34);
                    m.r8(0x6de52e)
                }
                1 => {
                    m.w16(r2 + 6, 0x24);
                    seat as u8
                }
                _ => {
                    m.w16(r2 + 6, 0x23);
                    seat as u8
                }
            };
            m.w8(r2 + 0x14, al);
            for o in 8..0x10 {
                m.w8(r2 + o, 0xff);
            }
            m.w32(r2 + 0x10, 0);
            m.w8(r2 + 0x15, 0);
            m.w16(r2 + 0x16, 0xffff);
            finish(m, host, 1)
        }
        3..=5 => {
            let mut total: u32 = [160_000, 120_000, 80_000][(prop - 3) as usize];
            let t = town(m, seat);
            let cap = m.r32(t + 0x700);
            if total > cap {
                total = cap;
            }
            host.call(m, 0x529f90, t, &[total, 0xa]);
            let mut left = count(m);
            m.w32(f.arg(0), 0);
            let mut i = 0u32;
            while i < count(m) {
                if m.r32(f.at(0x24)) != i {
                    let ti = m.r32(TOWNS).wrapping_add(i * TOWN_SIZE);
                    left -= 1;
                    let share = ((left as i32 >> 1) as u32).wrapping_add(total) / left;
                    host.call(m, 0x529f90, ti, &[share, 4]);
                    total = total.wrapping_sub(share);
                }
                i += 1;
                m.w32(f.arg(0), i);
            }
            finish(m, host, 0)
        }
        _ => finish(m, host, 0),
    }
}

/// 0x00542860 `op_create_initial_ai_merchants_all_towns` (operación en ECX). Marco SEH (`0x66367F`). Solo sin comerciantes
/// (`0x6DE4AA == 0`): reserva y pone a cero una tabla de `u16[0x6DDB74]` bytes, guarda los productos `op+0x10·op+0xC` en `esp-0x10` y
/// `op+0x12·op+0xE` en `esp-0x14` y llama a `0x542560(i, &a, &b, tabla)` por ciudad (los dos locales se comparan con `peek`); libera la
/// tabla. Crea el gestor `0x6CBDC8` si falta (`malloc(0x2C)` + `0x444BA0`), lo prepara (`0x444D30` si `+0x28 != 1`, `0x4482C0`) y
/// termina con `0x531BA0` y `0x533510` del mundo. Devuelve EAX de la última llamada (o el enlace SEH previo si no hace nada).
pub fn op_create_initial_ai_merchants_all_towns(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let prev = f.seh_enter(m, 0x66367f);
    if m.r16(TOWN_COUNT) != 0 {
        f.seh_leave(m);
        return prev;
    }
    let n = m.r32(0x6ddb74) & 0xffff;
    let (a, b) = (f.at(0x10), f.at(0x14));
    let va = u32::from(m.r16(op + 0x10)) * u32::from(m.r16(op + 0xc));
    m.w32(a, va);
    let vb = u32::from(m.r16(op + 0x12)) * u32::from(m.r16(op + 0xe));
    m.w32(b, vb);
    let buf = host.call(m, MALLOC, 0, &[n]);
    for i in 0..n {
        m.w8(buf + i, 0);
    }
    let mut i = 0;
    while i < m.r32(0x6de4b0) {
        let args = peek(m, &[i, a, b, buf], a, 1);
        let args = peek(m, &args, b, 1);
        host.call(m, 0x542560, op, &args);
        i += 1;
    }
    host.call(m, FREE, 0, &[buf]);
    let mut g = m.r32(0x6cbdc8);
    if g == 0 {
        let p = host.call(m, MALLOC, 0, &[0x2c]);
        m.w32(a, p);
        f.seh_state(m, 0);
        g = if p != 0 { host.call(m, 0x444ba0, p, &[]) } else { 0 };
        f.seh_state(m, 0xffff_ffff);
        m.w32(0x6cbdc8, g);
    }
    if m.r8(g + 0x28) != 1 {
        host.call(m, 0x444d30, g, &[]);
        g = m.r32(0x6cbdc8);
    }
    host.call(m, 0x4482c0, g, &[]);
    host.call(m, 0x531ba0, WORLD, &[]);
    let r = host.call(m, 0x533510, WORLD, &[]);
    f.seh_leave(m);
    r
}

/// 0x00542A30 `op_create_ai_merchants_batch` (operación en ECX; sin SEH). Locales en `esp-0x34..`: tabla de colores ocupados (`esp-0x18`,
/// `malloc(u16[0x6DDB74])` a cero, marcada con `+0x1D` de cada comerciante existente), semillas `op+0xC/0xE/0x10/0x12`, temporales x87
/// y el índice de barco que devuelve `0x507D60` en `esp-0x32`. Crea `min(op+4, 30)` comerciantes; cada uno en la siguiente ciudad con
/// `town+0x2C8 & 0x40` (rotando): `0x530470(ciudad, 1, (a+K)/(c+K), (a+K)/((c+1)·M))` en `f32`, tarea 0x04 (`0x4D8CF0`) con sus bytes,
/// `0x4F68D0`, color libre de la tabla (`+0x1D`, `0x4F6450`), escudo (`+0x1C`, `0x4F6420`), `0x5308A0`, `+0x53C = byte[0x673841]·10000`,
/// `+0 = op+8` y `op+6` barcos (`0x507D60`, `0x52A8D0`, `0x5181D0`, `0x5182B0`, enlazados en `+0xE`, `0x509A50`, `+0x40` de 0x673660);
/// cierra con `0x4F7BB0` y `0x54E170`. Devuelve EAX de `free`.
pub fn op_create_ai_merchants_batch(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let (lc, la, l10, l12) = (f.at(0x28), f.at(0x30), f.at(0x24), f.at(0x20));
    let v = u32::from(m.r16(op + 0xe));
    m.w32(lc, v);
    let v = u32::from(m.r16(op + 0xc));
    m.w32(la, v);
    let v = u32::from(m.r16(op + 0x10));
    m.w32(l10, v);
    let v = u32::from(m.r16(op + 0x12));
    m.w32(l12, v);
    let n = m.r32(0x6ddb74) & 0xffff;
    let buf = host.call(m, MALLOC, 0, &[n]);
    m.w32(f.at(0x18), buf);
    for i in 0..n {
        m.w8(buf + i, 0);
    }
    let mut i = 0u32;
    while (i as u16) < m.r16(TOWN_COUNT) {
        let tp = host.call(m, 0x5303c0, WORLD, &[i & 0xffff]);
        let c = u32::from(m.r8(tp + 0x1d));
        m.w8(c.wrapping_add(buf), 1);
        i += 1;
    }
    if m.r16(op + 4) > 0x1e {
        m.w16(op + 4, 0x1e);
    }
    let mut j = 0u32;
    m.w32(f.at(8), 0);
    let mut next = 0u32;
    if m.r16(op + 4) != 0 {
        loop {
            let cur = loop {
                let count = m.r32(0x6de4b0);
                let cur = next;
                next = next.wrapping_add(1);
                m.w32(f.at(0x2c), cur);
                m.w32(f.at(0x1c), next);
                if next & 0xffff >= count {
                    m.w32(f.at(0x1c), 0);
                    next = 0;
                }
                let t = m.r32(TOWNS).wrapping_add((cur & 0xffff) * TOWN_SIZE);
                if m.r32(t + 0x2c8) & 0x40 != 0 {
                    break cur;
                }
            };
            let kf = f64::from(f32::from_bits(m.r32(0x66bc80)));
            let a = u32::from(m.r16(op + 0xc));
            m.w32(f.at(0xc), (a as i32 as f32).to_bits());
            let p = (f64::from(a as i32 as f32) + kf) as f32;
            let c = u32::from(m.r16(op + 0x10));
            m.w32(f.at(0x14), p.to_bits());
            let q = (f64::from(p) / (f64::from(c as i32 as f32) + kf)) as f32;
            m.w32(f.at(4), q.to_bits());
            let c1 = c.wrapping_add(1);
            let km = f64::from(f32::from_bits(m.r32(0x672f0c)));
            let r = (f64::from(p) / (f64::from(c1 as i32 as f32) * km)) as f32;
            m.w32(f.at(0xc), r.to_bits());
            let mi = host.call(m, 0x530470, WORLD, &[cur, 1, q.to_bits(), r.to_bits()]);
            if (mi as u16) < m.r16(TOWN_COUNT) {
                let tick = m.r32(TICK);
                let rec = host.call(m, 0x4d8cf0, SCHED, &[(j & 0xffff).wrapping_add(tick).wrapping_add(0xa05)]);
                let bl = mi as u8;
                m.w16(rec + 6, 4);
                m.w16(rec + 0xa, 0x8300);
                m.w16(rec + 8, u16::from(bl));
                m.w16(rec + 0xc, 0xf448);
                m.w8(rec + 0xe, bl.wrapping_add(3) & 7);
                m.w8(rec + 0x10, bl & 3);
                m.w8(rec + 0x11, bl & 0xf);
                m.w8(rec + 0xf, bl.wrapping_add(5) & 0xf);
                let tp = host.call(m, 0x5303c0, WORLD, &[mi & 0xffff]);
                let (x10, x12) = (m.r32(l10), m.r32(l12));
                host.call(m, 0x4f68d0, tp, &[x10, x12, mi, 0, 1]);
                let ce = (u32::from(m.r16(op + 0xe)) + j + m.r32(lc)) & 0x3f;
                let mut ca = (u32::from(m.r16(op + 0xc)) + j + m.r32(la)) & 0x3f;
                m.w32(lc, ce);
                m.w32(la, ca);
                while m.r8(buf.wrapping_add(ca)) != 0 {
                    ca = (ca + 1) & 0x3f;
                    m.w32(la, ca);
                }
                let d = m.r32(0x6ddb74) & 0xffff;
                m.w8(tp + 0x1d, (ca % d) as u8);
                host.call(m, 0x4f6450, tp, &[]);
                m.w8(buf.wrapping_add(ca), 1);
                let e = m.r32(lc);
                let d = m.r32(0x6ddb70) & 0xffff;
                m.w8(tp + 0x1c, (e % d) as u8);
                host.call(m, 0x4f6420, tp, &[]);
                let (o12, o10) = (m.r32(l12), m.r32(l10));
                let n10 = (u32::from(m.r16(op + (o12 & 3) * 2 + 0xc)).wrapping_add(o10)) & 0xffff;
                m.w32(l10, n10);
                let n12 = (u32::from(m.r16(op + (n10 & 3) * 2 + 0xc)).wrapping_add(o12)) & 0xffff;
                let cur_l = m.r32(f.at(0x2c));
                m.w32(l12, n12);
                host.call(m, 0x5308a0, WORLD, &[mi, cur_l, 1]);
                m.w8(f.at(0x10), 0);
                m.w32(f.at(0xc), 0);
                let gold = u32::from(m.r8(0x673841)) * 10000;
                m.w32(tp + 0x53c, gold);
                let v = m.r32(op + 8);
                m.w32(tp, v);
                if m.r16(op + 6) != 0 {
                    let toff = (m.r32(f.at(0x2c)) & 0xffff) * TOWN_SIZE;
                    m.w32(f.at(0x14), toff);
                    loop {
                        let cur_l = m.r32(f.at(0x2c));
                        let slot = f.at(0x32);
                        let sh = host.call(m, 0x507d60, SHIP_MGR, &[slot, cur_l, 1]);
                        let toff = m.r32(f.at(0x14));
                        let t = m.r32(TOWNS).wrapping_add(toff);
                        let fl = m.r32(f.at(0x10));
                        host.call(m, 0x52a8d0, t, &[sh, fl]);
                        host.call(m, 0x5181d0, sh, &[]);
                        host.call(m, 0x5182b0, sh, &[]);
                        m.w8(sh, bl);
                        let hd = m.r16(tp + 0xe);
                        m.w16(sh + 4, hd);
                        let idx = m.r16(slot);
                        let tb = m.r8(f.at(0x2c));
                        m.w16(tp + 0xe, idx);
                        let v = m.r32(sh + 0x14);
                        m.w32(sh + 0x18, v);
                        m.w8(sh + 0x13a, tb);
                        host.call(m, 0x509a50, SHIP_MGR, &[sh]);
                        let ty = u32::from(m.r8(sh + 0xe)) & 3;
                        let w = u16::from(m.r8(0x673660 + ty));
                        m.w16(sh + 0x40, w);
                        let t = m.r32(TOWNS).wrapping_add(m.r32(f.at(0x14)));
                        if m.r8(t + 0x2c8) & 1 == 0 {
                            m.w8(f.at(0x10), 2);
                        }
                        let k = m.r32(f.at(0xc)).wrapping_add(1);
                        m.w32(f.at(0xc), k);
                        if (k as u16) >= m.r16(op + 6) {
                            break;
                        }
                    }
                }
                host.call(m, 0x4f7bb0, tp, &[mi]);
                host.call(m, 0x54e170, 0x6dfc90, &[mi]);
                next = m.r32(f.at(0x1c));
                j = m.r32(f.at(8));
            }
            j = j.wrapping_add(1);
            m.w32(f.at(8), j);
            if (j as u16) >= m.r16(op + 4) {
                break;
            }
        }
    }
    let buf = m.r32(f.at(0x18));
    host.call(m, FREE, 0, &[buf])
}

/// 0x00541D70 `op_create_player_merchant` (operación en ECX; sin SEH). Mensaje local de 0x14 bytes en `esp-0x14` (`{-1, 0, 0, 0, 0}`),
/// índice del comerciante en `esp-0x18` e índice de barco de salida de `0x507D60` en `esp-0x1A`. Normaliza la ciudad `op+9` (módulo
/// `0x6DE4B0` si la supera), crea el comerciante (`0x530470(ciudad, 0, op+0xE, op+0xD + op+0xE)`; 0xFFFF = fallo) y lo prepara:
/// fortuna `u16[0x672D20 + op+0xA·2]`, `0x4F68D0`, `0x4F9550(op+0x10)`, tarea 0x04 a +0x5A00 (`0x4D8CF0`), colores con `0x512BA0`,
/// `0x4F6420`/`0x4F6450`, jugador local (`op+4 == 0x6DF31C` → `0x6DFC14`), `0x5308A0`; crea `op+0xB` barcos (`0x507D60`, `0x52A8D0`, el
/// último del jugador con tripulación −4000·`[0x66E048]` y hasta cuatro huecos 0xFF de `+0x13C` a cero), `0x4F7BB0`, `0x54E170`. En red
/// (`0x6DF31C == 0`) envía 0xC0/0xC6/0xC8 por `0x54AA70` si `0x6DF2F0 & 1`, o si `& 0xC` y hay más comerciantes vivos que `0x54BD60`;
/// para el jugador local en red manda sus dos nombres (0xAA/0xAB, trozos de 12 bytes). Devuelve el EAX que deja el original.
pub fn op_create_player_merchant(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let lo8 = |x: u32, b: u8| (x & 0xffff_ff00) | u32::from(b);
    let msg = f.at(0x14);
    m.w32(msg, 0xffff_ffff);
    for k in 1..5 {
        m.w32(msg + 4 * k, 0);
    }
    let n = m.r32(0x6de4b0);
    let t = u32::from(m.r8(op + 9));
    if t > n {
        m.w8(op + 9, (t % n) as u8);
    }
    let t9 = u32::from(m.r8(op + 9));
    let oe = u32::from(m.r16(op + 0xe));
    let od = u32::from(m.r8(op + 0xd));
    let r = host.call(m, 0x530470, WORLD, &[t9, 0, oe, od + oe]);
    let mi = r & 0xffff;
    m.w32(f.at(0x18), mi);
    if mi == 0xffff {
        return r;
    }
    let tp = host.call(m, 0x5303c0, WORLD, &[mi]);
    let fa = u32::from(m.r8(op + 0xa));
    let fortune = u32::from(m.r16(fa * 2 + 0x672d20));
    m.w16(tp + 8, 0);
    m.w32(tp, fortune);
    m.w32(tp + 4, 0);
    let flag = u32::from(m.r8(op + 0xc) > 0);
    host.call(m, 0x4f68d0, tp, &[od, 0x15, mi, 1, flag]);
    let o10 = u32::from(m.r8(op + 0x10));
    host.call(m, 0x4f9550, tp, &[o10]);
    let tick = m.r32(TICK);
    let rec = host.call(m, 0x4d8cf0, SCHED, &[tick.wrapping_add(0x5a00)]);
    m.w16(rec + 6, 4);
    m.w16(rec + 0xa, 0);
    m.w16(rec + 8, (mi & 0xff) as u16);
    m.w16(rec + 0xc, 0);
    for o in [0xe, 0x10, 0x11, 0xf] {
        m.w8(rec + o, 0);
    }
    let flag = u32::from(m.r8(op + 0xc) > 0);
    let cl = (mi as u8).wrapping_sub(m.r8(0x6de4b0)).wrapping_sub(0xc);
    host.call(m, 0x512ba0, 0x6ddaa0, &[tp + 0x1c, tp + 0x1d, u32::from(cl), flag]);
    host.call(m, 0x4f6420, tp, &[]);
    host.call(m, 0x4f6450, tp, &[]);
    if m.r32(op + 4) == m.r32(0x6df31c) {
        m.w32(0x6dfc14, mi);
    }
    let t9 = u32::from(m.r8(op + 9));
    host.call(m, 0x5308a0, WORLD, &[mi, t9, 1]);
    let towns = |m: &mut dyn Mem, t: u32| m.r32(TOWNS).wrapping_add(t * TOWN_SIZE);
    if (m.r8(op + 0xb) as i8) > 0 {
        loop {
            let left = m.r8(op + 0xb).wrapping_sub(1);
            m.w8(op + 0xb, left);
            let slot = f.at(0x1a);
            let t9 = u32::from(m.r8(op + 9));
            let sh = host.call(m, 0x507d60, SHIP_MGR, &[slot, t9, 1]);
            let t9 = u32::from(m.r8(op + 9));
            let t = towns(m, t9);
            if m.r8(op + 8) == 0 && m.r8(op + 0xb) == 0 {
                host.call(m, 0x52a8d0, t, &[sh, 1]);
                m.w8(sh + 0xf, 2);
                let c = m.r32(0x66e048);
                let v = m.r32(sh + 0x10).wrapping_add(c.wrapping_mul(4000u32.wrapping_neg()));
                m.w32(sh + 0x10, v);
                host.call(m, 0x5181d0, sh, &[]);
                let mut cnt = 0;
                for off in (0..0x18).step_by(2) {
                    let a = sh + 0x13c + off;
                    if m.r8(a) == 0xff {
                        cnt += 1;
                        m.w8(a, 0);
                        if cnt >= 4 {
                            break;
                        }
                    }
                }
            } else {
                host.call(m, 0x52a8d0, t, &[sh, 0]);
                host.call(m, 0x5181d0, sh, &[]);
            }
            host.call(m, 0x5182b0, sh, &[]);
            m.w8(sh, mi as u8);
            let hd = m.r16(tp + 0xe);
            m.w16(sh + 4, hd);
            let idx = m.r16(slot);
            m.w16(tp + 0xe, idx);
            let v = m.r32(sh + 0x14);
            m.w32(sh + 0x18, v);
            let b = m.r8(op + 9);
            m.w8(sh + 0x13a, b);
            host.call(m, 0x509a50, SHIP_MGR, &[sh]);
            let ty = u32::from(m.r8(sh + 0xe)) & 3;
            let w = u16::from(m.r8(0x673660 + ty));
            m.w16(sh + 0x40, w);
            m.w8(sh + 0x3f, 5);
            if (m.r8(op + 0xb) as i8) <= 0 {
                break;
            }
        }
    }
    let b = m.r8(op + 0xb).wrapping_sub(1);
    m.w8(op + 0xb, b);
    host.call(m, 0x4f7bb0, tp, &[mi]);
    let mut eax = host.call(m, 0x54e170, 0x6dfc90, &[mi]);
    if m.r32(0x6df31c) == 0 {
        let mut alive = 0i32;
        let mut i = 0u32;
        while i < u32::from(m.r16(TOWN_COUNT)) {
            eax = host.call(m, 0x5303c0, WORLD, &[i]);
            if m.r16(eax + 8) == 0 {
                alive += 1;
            }
            i += 1;
        }
        let net = m.r8(0x6df2f0);
        eax = lo8(eax, net);
        let send = if net & 1 != 0 {
            true
        } else if net & 0xc == 0 {
            false
        } else {
            let r = host.call(m, 0x54bd60, 0x6df2f0, &[]);
            eax = u32::from(alive > r as i32);
            eax != 0
        };
        if send {
            for code in [0xc0u32, 0xc6] {
                m.w32(msg, code);
                let a = peek(m, &[msg], msg, 5);
                host.call(m, 0x54aa70, 0x6df2f0, &a);
            }
            let cfg = m.r32(0x6cc3e8);
            m.w32(msg, 0xc8);
            let lvl = m.r32(cfg + 4);
            m.w32(f.at(8), 0);
            let v = m.r32(lvl.wrapping_mul(4).wrapping_add(0x66de7c)).wrapping_mul(1000) as i32;
            m.w32(f.at(0x10), ((v + ((v >> 31) & 0xff)) >> 8) as u32);
            m.w32(f.at(0xc), 0xffff_ffff);
            m.w32(f.at(4), 0xffff_ffff);
            let a = peek(m, &[msg], msg, 5);
            eax = host.call(m, 0x54aa70, 0x6df2f0, &a);
        }
    }
    if m.r32(0x6dfc14) != mi || m.r8(0x6df2f0) & 0xc == 0 {
        return eax;
    }
    for (code, off) in [(0xaau32, 0x1908u32), (0xab, 0x1788)] {
        let ui = m.r32(0x6cbb94);
        m.w32(msg, code);
        m.w16(f.at(0x10), mi as u16);
        m.w16(f.at(0xe), 0);
        eax = host.call(m, 0x4d2ec0, ui.wrapping_add(off), &[]);
        let mut p = m.r32(eax);
        let mut k = 0u32;
        loop {
            let ch = m.r8(p);
            if code == 0xaa {
                eax = lo8(eax, ch);
            }
            m.w8(f.at(0xc) + k, ch);
            k += 1;
            if k >= 0xc {
                let a = peek(m, &[msg], msg, 5);
                eax = host.call(m, 0x54aa70, 0x6df2f0, &a);
                let w = m.r16(f.at(0xe)).wrapping_add(0xc);
                m.w16(f.at(0xe), w);
                k = 0;
            }
            let ch = m.r8(p);
            p = p.wrapping_add(1);
            if ch == 0 {
                break;
            }
        }
        if k > 1 {
            let a = peek(m, &[msg], msg, 5);
            eax = host.call(m, 0x54aa70, 0x6df2f0, &a);
        }
    }
    eax
}

/// 0x00516520 `ship_load_core` (thiscall, `ret 8`: fuente y versión, que no usa). Lee la cabecera de 0x54 bytes del barco: `+4` (2),
/// bytes `+0/+0xE/+0xF/+1`, `+2 = (sx(b6) << 8) + sx(b7)` en 16 bits, `+0x10/+0x14` (4), `+6/+8/+0xA/+0xC` (2), `+0x18..+0x36` y
/// `+0x3C..+0x54` con memcpy del CRT, bytes `+0x36..+0x39` y `+0x3A = (b3A << 8) + b3B`. Si hay ruta (`+0x24 != 0`): con `+0x36` lee un
/// recuento u16 en el hueco de su primer argumento (`esp+4`) y crea la ruta (`malloc(8)` = {puntos, recuento}, `malloc(4n)` y n pares de
/// u16); sin `+0x36` la recalcula con el planificador (`0x445010` sobre un par local {&dos puntos, 2} con las posiciones de las ciudades
/// `0x6DE4B8[+0x37/+0x38]` de `0x6DDBB0 + i·0x34`) y guarda su resultado en `+0x36`. Devuelve el puntero tras lo leído.
pub fn ship_load_core(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let src = m.r32(f.arg(0));
    let cp = |m: &mut dyn Mem, d: u32, s: u32, n: u32| {
        for k in 0..n {
            let b = m.r8(s + k);
            m.w8(d + k, b);
        }
    };
    cp(m, this + 4, src, 2);
    for (d, s) in [(0, 2), (0xe, 3), (0xf, 4), (1, 5)] {
        let b = m.r8(src + s);
        m.w8(this + d, b);
    }
    let hi = m.r8(src + 6) as i8 as i16 as u16;
    let lo = m.r8(src + 7) as i8 as i16 as u16;
    m.w16(this + 2, (hi << 8).wrapping_add(lo));
    cp(m, this + 0x10, src + 8, 8);
    cp(m, this + 6, src + 0x10, 8);
    cp(m, this + 0x18, src + 0x18, 0x1e);
    for o in 0x36..0x3a {
        let b = m.r8(src + o);
        m.w8(this + o, b);
    }
    let w = (u16::from(m.r8(src + 0x3a)) << 8).wrapping_add(u16::from(m.r8(src + 0x3b)));
    m.w16(this + 0x3a, w);
    cp(m, this + 0x3c, src + 0x3c, 0x18);
    let mut s = src + 0x54;
    let route = this + 0x24;
    m.w32(f.at(0x14), route);
    if m.r32(route) == 0 {
        return s;
    }
    if m.r8(this + 0x36) != 0 {
        cp(m, f.arg(0), s, 2);
        s += 2;
        let p = host.call(m, MALLOC, 0, &[8]);
        let n = m.r32(f.arg(0)) & 0xffff;
        m.w32(p + 4, n);
        let arr = host.call(m, MALLOC, 0, &[n << 2]);
        m.w32(p, arr);
        for i in 0..n {
            let a = m.r32(p) + i * 4;
            cp(m, a, s, 2);
            let a = m.r32(p) + i * 4 + 2;
            cp(m, a, s + 2, 2);
            s += 4;
        }
        m.w32(route, p);
        return s;
    }
    m.w32(f.at(4), 2);
    let pts = f.at(0x10);
    for (k, o) in [(0u32, 0x37u32), (4, 0x38)] {
        let i = u32::from(m.r8(this + o));
        let t = u32::from(m.r8(0x6de4b8 + i));
        let x = m.r16(0x6ddbb0 + t * 0x34);
        let y = m.r16(0x6ddbb4 + t * 0x34);
        m.w16(pts + k, x);
        m.w16(pts + k + 2, y);
    }
    let pair = f.at(8);
    m.w32(pair, pts);
    let pl = m.r32(0x6cbdc8);
    let args = peek(m, &[pair, route], pair, 2);
    let r = host.call(m, 0x445010, pl, &args);
    m.w8(this + 0x36, r as u8);
    s
}

/// 0x005462F0 `town_add_facility_supply_entry` (ciudad en ECX, stdcall de 1: tipo). Marco SEH (`0x6637DD`). Para tipos 4..0x14 crea un
/// nodo (`malloc(0x14)` + `0x50FCA0`; el puntero se guarda en el hueco del argumento), lo pone al frente de la lista `town+0x9B4` y lo
/// inicia con `0x50FE00(cantidad, town+0x2C0, tipo)`, donde `cantidad = (0x400 / (ciudades·3 >> 4)) · {200, 700, 500, 100}[tipo 4/0xD/0x10/
/// otro] >> 10`. El segundo argumento lleva en sus bytes altos el resto de esa división (o `7·cociente` para el tipo 0xD), como el original. Devuelve EAX de `0x50FE00` (o
/// el enlace SEH previo si el tipo no vale).
pub fn town_add_facility_supply_entry(m: &mut dyn Mem, host: &mut dyn OpsHost, town: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let prev = f.seh_enter(m, 0x6637dd);
    let kind = m.r32(f.arg(0));
    if kind >= 0x15 || kind <= 3 {
        f.seh_leave(m);
        return prev;
    }
    let p = host.call(m, MALLOC, 0, &[0x14]);
    m.w32(f.arg(0), p);
    f.seh_state(m, 0);
    let node = if p != 0 { host.call(m, 0x50fca0, p, &[]) } else { 0 };
    let head = m.r32(town + 0x9b4);
    f.seh_state(m, 0xffff_ffff);
    m.w32(node, head);
    m.w32(town + 0x9b4, node);
    let d = m.r32(0x6de4b0).wrapping_mul(3) >> 4;
    let (q, rem) = (0x400 / d, 0x400 % d);
    let amount = q.wrapping_mul(match kind {
        4 => 200,
        0xd => 700,
        0x10 => 500,
        _ => 100,
    }) >> 10;
    let edx = if kind == 0xd { q.wrapping_mul(7) } else { rem };
    let b = (edx & 0xffff_ff00) | u32::from(m.r8(town + 0x2c0));
    let r = host.call(m, 0x50fe00, node, &[amount, b, kind]);
    f.seh_leave(m);
    r
}

/// 0x005419C0 `op_append_lobby_player_name_chunk` (operación en ECX). Marco SEH (`0x663655`) y dos CString locales (`esp-0x14` iniciado
/// con `0x6C7CCC`, `esp-0x10` temporal). Con interfaz (`0x6CBB94`), toma los primeros `op+5·14` caracteres del nombre del jugador `op+4`
/// (`ui + 0x4440 + j·4`, `0x64D230` y `0x64F340`), añade hasta 14 bytes de `op+6` (`0x64F60A`), guarda el resultado de vuelta y, si el
/// trozo terminó antes de 14 bytes, avisa con `0x43E240(j)`. Devuelve EAX del destructor final.
pub fn op_append_lobby_player_name_chunk(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x663655);
    let (s1, s2) = (f.at(0x14), f.at(0x10));
    let e = m.r32(0x6c7ccc);
    m.w32(s1, e);
    f.seh_state(m, 0);
    let n = u32::from(m.r8(op + 5)) * 14;
    let ui = m.r32(0x6cbb94);
    if ui != 0 {
        let j = u32::from(m.r8(op + 4));
        let args = peek(m, &[s2, n], s2, 1);
        let r = host.call(m, 0x64d230, ui + j * 4 + 0x4440, &args);
        m.w8(f.at(4), 1);
        let args = peek(m, &[r], r, 1);
        host.call(m, 0x64f340, s1, &args);
        m.w8(f.at(4), 0);
        host.call(m, CSTR_DTOR, s2, &[]);
        let mut i = 0u32;
        while i < 0xe {
            let c = m.r8(op + 6 + i);
            if c == 0 {
                break;
            }
            host.call(m, 0x64f60a, s1, &[u32::from(c)]);
            i += 1;
        }
        let ui = m.r32(0x6cbb94);
        let j = u32::from(m.r8(op + 4));
        let args = peek(m, &[s1], s1, 1);
        host.call(m, 0x64f340, ui + j * 4 + 0x4440, &args);
        if i < 0xe {
            let j = u32::from(m.r8(op + 4));
            let ui = m.r32(0x6cbb94);
            host.call(m, 0x43e240, ui, &[j]);
        }
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, CSTR_DTOR, s1, &[]);
    f.seh_leave(m);
    r
}

/// 0x004D6B40 `message_pool_grow` (grupo de mensajes en ECX: `+0` vector, `+4` mínimo libre, `+6` recuento). Marco SEH (`0x6629E0`),
/// puntero reservado en `esp-0x10`. Reserva `(n+200)·16 + 4` bytes (prefijo con la capacidad), construye los mensajes con el iterador
/// del CRT (`0x639E9C`, `0x51A850`/`0x51A880`), copia los `n` existentes (`0x51AC00`), destruye y libera el vector anterior
/// (`0x639DA8`, `free(p-4)`) y encadena los 200 huecos nuevos (`+4 = 0xFF`, `+6 = siguiente`), subiendo el recuento. Devuelve EAX del
/// original (desplazamiento del último hueco).
pub fn message_pool_grow(m: &mut dyn Mem, host: &mut dyn OpsHost, pool: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x6629e0);
    let n = m.r16(pool + 6);
    if n < m.r16(pool + 4) {
        m.w16(pool + 4, n);
    }
    let cap = u32::from(n) + 0xc8;
    let p = host.call(m, MALLOC, 0, &[(cap << 4) + 4]);
    m.w32(f.at(0x10), p);
    f.seh_state(m, 0);
    let mut arr = 0;
    if p != 0 {
        m.w32(p, cap);
        host.call(m, VEC_CTOR, 0, &[p + 4, 0x10, cap, 0x51a850, 0x51a880]);
        arr = p + 4;
    }
    f.seh_state(m, 0xffff_ffff);
    let mut i = 0u32;
    while i < u32::from(m.r16(pool + 6)) {
        let src = m.r32(pool).wrapping_add(i * 16);
        host.call(m, 0x51ac00, arr + i * 16, &[src]);
        i += 1;
    }
    let old = m.r32(pool);
    if old != 0 {
        let c = m.r32(old - 4);
        host.call(m, VEC_DTOR, 0, &[old, 0x10, c, 0x51a880]);
        host.call(m, FREE, 0, &[old - 4]);
    }
    m.w32(pool, arr);
    let mut eax = 0;
    for _ in 0..0xc8 {
        let idx = u32::from(m.r16(pool + 6));
        let a = m.r32(pool);
        m.w8(idx * 16 + a + 4, 0xff);
        let a = m.r32(pool);
        m.w16(idx * 16 + a + 6, (idx + 1) as u16);
        m.w16(pool + 6, (idx + 1) as u16);
        eax = idx << 4;
    }
    f.seh_leave(m);
    eax
}

/// 0x00519D90 `ship_send_lost_letter_to_owner` (barco en ECX). Marco SEH (`0x663198`) y mensaje local en `esp-0x1C` (`0x51A850`/
/// `0x51A880`). Si el dueño (`0x5303C0(ship+0)`) sigue vivo (`+8 == 0`) le envía la carta 0x1B (`0x4D6530(dueño, &msg)`): retrato `+0x19`
/// del dueño, `(ship+0xE & 3) + (ship+0x1E << 2)`, `ship+0x22 & 0x7FF` y `ship+0x15E & 0x3FFF` más los bits del capitán `ship+0x42`
/// (`0x6DD7A0 + c·16`, `+2` y `+3`; 0xFF sin capitán) y `ship+0x40`. Devuelve EAX del destructor.
pub fn ship_send_lost_letter_to_owner(m: &mut dyn Mem, host: &mut dyn OpsHost, ship: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x663198);
    let owner = u32::from(m.r8(ship));
    let tp = host.call(m, 0x5303c0, WORLD, &[owner]);
    let msg = f.at(0x1c);
    host.call(m, 0x51a850, msg, &[]);
    f.seh_state(m, 0);
    if m.r16(tp + 8) == 0 {
        let pic = m.r8(tp + 0x19);
        m.w8(f.at(0x17), pic);
        let a = u16::from(m.r8(ship + 0xe) & 3).wrapping_add(m.r16(ship + 0x1e) << 2);
        m.w16(f.at(0x14), a);
        let b40 = m.r8(ship + 0x40);
        m.w8(f.at(0xd), b40);
        let cap = m.r16(ship + 0x42);
        let mut cx = m.r16(ship + 0x22) & 0x7ff;
        let mut dx = m.r16(ship + 0x15e) & 0x3fff;
        m.w8(f.at(0x18), 0x1b);
        m.w16(f.at(0x12), cx);
        m.w16(f.at(0x10), dx);
        if cap < m.r16(0x6dd892) {
            let c = m.r32(0x6dd7a0).wrapping_add(u32::from(cap) << 4);
            let b2 = u16::from(m.r8(c + 2));
            cx = cx.wrapping_add(b2 << 0xb);
            m.w16(f.at(0x12), cx);
            dx = dx.wrapping_add((b2 & 0xe0) << 9);
            m.w16(f.at(0x10), dx);
            let b3 = m.r8(c + 3);
            m.w8(f.at(0xe), b3);
        } else {
            m.w8(f.at(0xe), 0xff);
        }
        let owner = u32::from(m.r8(ship));
        let args = peek(m, &[owner, msg], msg, 4);
        host.call(m, 0x4d6530, MSGS, &args);
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x51a880, msg, &[]);
    f.seh_leave(m);
    r
}

/// 0x00533F20 `trade_offer_format_description` (oferta en ECX, stdcall de 1: búfer de salida). Marco SEH (`0x6634CA`) y CString local en
/// `esp-0x10` (iniciado con `0x6C7CCC`) que rellena `0x64DF80` (sprintf). Según `offer+0xF`: 0 mercancía (`u16 +0x12`, unidad
/// `0x6C0DF8[0x672C14[g]]`, nombre `0x6C0BB0[g]`), 1 barco `+0x10` (nombre `+0x160`, tipo `0x6C0E80[+0xE]`, valor `+0x10 / 2000`),
/// 2 importe `+0x10`. Copia la línea y la de `offer+8` (formato `0x6B5004`) al búfer, cada una terminada en `\n`, y devuelve los bytes
/// escritos.
pub fn trade_offer_format_description(m: &mut dyn Mem, host: &mut dyn OpsHost, offer: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x6634ca);
    let s = f.at(0x10);
    let e = m.r32(0x6c7ccc);
    m.w32(s, e);
    f.seh_state(m, 0);
    let mut n = 2u32;
    match m.r8(offer + 0xf) {
        0 => {
            let g = u32::from(m.r16(offer + 0x10));
            let name = m.r32(g * 4 + 0x6c0bb0);
            let u = u32::from(m.r8(g + 0x672c14));
            let unit = m.r32(u * 4 + 0x6c0df8);
            let amt = u32::from(m.r16(offer + 0x12));
            let fmt = m.r32(0x6b4fdc);
            host.call(m, 0x64df80, 0, &[s, fmt, amt, unit, name]);
        }
        1 => {
            let i = m.r16(offer + 0x10);
            if i < m.r16(SHIP_COUNT) {
                let sh = m.r32(SHIP_ARRAY).wrapping_add(u32::from(i) * SHIP_SIZE);
                let v = (m.r32(sh + 0x10) as i32 / 2000) as u32;
                let ty = u32::from(m.r8(sh + 0xe));
                let tn = m.r32(ty * 4 + 0x6c0e80);
                let fmt = m.r32(0x6b4fe0);
                host.call(m, 0x64df80, 0, &[s, fmt, sh + 0x160, tn, v]);
            }
        }
        2 => {
            let v = m.r32(offer + 0x10);
            let fmt = m.r32(0x6b4fe4);
            host.call(m, 0x64df80, 0, &[s, fmt, v]);
        }
        _ => {}
    }
    let mut out = m.r32(f.arg(0));
    let mut copy = |m: &mut dyn Mem, out: &mut u32, n: &mut u32| {
        let mut p = m.r32(s);
        loop {
            let c = m.r8(p);
            if c == 0 {
                break;
            }
            *n += 1;
            m.w8(*out, c);
            *out += 1;
            p += 1;
        }
        m.w8(*out, 0xa);
    };
    copy(m, &mut out, &mut n);
    let v = m.r32(offer + 8);
    let fmt = m.r32(0x6b5004);
    host.call(m, 0x64df80, 0, &[s, fmt, v]);
    out += 1;
    copy(m, &mut out, &mut n);
    f.seh_state(m, 0xffff_ffff);
    host.call(m, CSTR_DTOR, s, &[]);
    f.seh_leave(m);
    n
}

/// 0x00544730 `load_received_game_statistics_block`. Marco SEH (`0x663735`) y lector local de 4 bytes en `esp-0x10` (`0x54C3F0`/
/// `0x54C400`). Sin bloque recibido (`0x6DF310 == 0`) avisa del fallo (`0x54DDF0` del gestor de partida `0x6DFC90`); con bloque
/// (`0x6DF308`: tamaño y datos), si el tamaño es ≤ 0 entrega los datos tal cual (`0x54DA60(+4)`), y si no los descomprime en un búfer
/// nuevo (`malloc(tamaño & 0x3FFFFFFF)`, `0x54C480`) que entrega y libera (sin memoria, `0x54DDF0`). Devuelve EAX del destructor.
pub fn load_received_game_statistics_block(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x663735);
    let rd = f.at(0x10);
    m.w32(rd, this);
    host.call(m, 0x54c3f0, rd, &[]);
    f.seh_state(m, 0);
    if m.r32(0x6df310) == 0 {
        host.call(m, 0x54ddf0, 0x6dfc90, &[]);
    } else {
        let blk = m.r32(0x6df308);
        let size = m.r32(blk) as i32;
        if size <= 0 {
            host.call(m, 0x54da60, 0x6dfc90, &[blk + 4]);
        } else {
            let p = host.call(m, MALLOC, 0, &[size as u32 & 0x3fff_ffff]);
            if p == 0 {
                host.call(m, 0x54ddf0, 0x6dfc90, &[]);
            } else {
                let blk = m.r32(0x6df308);
                host.call(m, 0x54c480, rd, &[blk, p]);
                host.call(m, 0x54da60, 0x6dfc90, &[p]);
                host.call(m, FREE, 0, &[p]);
            }
        }
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x54c400, rd, &[]);
    f.seh_leave(m);
    r
}

/// 0x00549940 `operations_load_custom_map` (operaciones en ECX, stdcall de 1: ruta como CString por valor, que destruye al salir). Marco
/// SEH (`0x663954`). Espera con `Sleep(50)` (IAT `0x66A2BC`) a que termine la carga en curso (`+0x38`), crea el mapa propio si falta
/// (`+0xC`: `malloc(0x25C)` + `0x544830`), copia la ruta a un hueco de la pila (`0x64EFC8`, puntero en `esp-0x10`) y la entrega a
/// `0x5448A0`. Devuelve en AL el resultado de la carga (bytes altos del destructor).
pub fn operations_load_custom_map(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x663954);
    f.seh_state(m, 0);
    let mut ecx = this;
    if m.r32(this + 0x38) != 0 {
        let sleep = m.r32(0x66a2bc);
        loop {
            host.call(m, sleep, this, &[0x32]);
            if m.r32(this + 0x38) == 0 {
                break;
            }
        }
    }
    if m.r32(this + 0xc) == 0 {
        let p = host.call(m, MALLOC, 0, &[0x25c]);
        m.w32(f.at(0x10), p);
        m.w8(f.at(4), 1);
        let v = if p != 0 {
            ecx = p;
            host.call(m, 0x544830, p, &[])
        } else {
            0
        };
        m.w8(f.at(4), 0);
        m.w32(this + 0xc, v);
    }
    let slot = f.push(m, 0x20, ecx);
    m.w32(f.at(0x10), slot);
    host.call(m, 0x64efc8, slot, &[f.arg(0)]);
    let map = m.r32(this + 0xc);
    let a = m.r32(slot);
    let ok = host.call(m, 0x5448a0, map, &[a]) as u8;
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, CSTR_DTOR, f.arg(0), &[]);
    f.seh_leave(m);
    (r & 0xffff_ff00) | u32::from(ok)
}

/// 0x0053FEA0 `op_add_captain_candidate` (operación en ECX). Marco SEH (`0x6635E7`) y mensaje local en `esp-0x1C`. Si el comerciante
/// `op+4` existe, tiene ciudad válida (`+0x31`), ha pasado su plazo (`+0x28 < día`, `+0x2C + 0x11700 ≤ día`) y ningún capitán de su
/// cadena (`+0x34`, siguiente en `cap+0`) es más reciente que `día − 0x11700`, toma un capitán libre (`0x5097C0(1)`), lo pone al frente
/// de la cadena con los datos `op+5..9`, su color `+0x1D` y la fecha, y avisa al dueño vivo con el mensaje 0x7A. Devuelve EAX del
/// destructor (o el del original si el comerciante no existe).
pub fn op_add_captain_candidate(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let prev = f.seh_enter(m, 0x6635e7);
    let b4 = m.r8(op + 4);
    if u16::from(b4) >= m.r16(TOWN_COUNT) {
        f.seh_leave(m);
        return (prev & 0xffff_ff00) | u32::from(b4);
    }
    let tp = host.call(m, 0x5303c0, WORLD, &[u32::from(b4)]);
    let tick = m.r32(TICK);
    let lim = tick.wrapping_sub(0x11700);
    let msg = f.at(0x1c);
    host.call(m, 0x51a850, msg, &[]);
    f.seh_state(m, 0);
    'body: {
        if m.r32(0x6de4b0) <= u32::from(m.r8(tp + 0x31)) {
            break 'body;
        }
        let tick = m.r32(TICK);
        if tick <= m.r32(tp + 0x28) || tick < m.r32(tp + 0x2c).wrapping_add(0x11700) {
            break 'body;
        }
        let cnt = m.r16(0x6dd892);
        let mut si = m.r16(tp + 0x34);
        while si < cnt {
            let c = m.r32(SHIP_MGR).wrapping_add(u32::from(si) << 4);
            if lim < m.r32(c + 4) {
                break 'body;
            }
            si = m.r16(c);
        }
        let ax = host.call(m, 0x5097c0, SHIP_MGR, &[1]) as u16;
        let c = m.r32(SHIP_MGR).wrapping_add(u32::from(ax) << 4);
        let head = m.r16(tp + 0x34);
        m.w16(c, head);
        let n = m.r8(tp + 0x36).wrapping_add(1);
        m.w16(tp + 0x34, ax);
        m.w8(tp + 0x36, n);
        let b = m.r8(op + 5);
        m.w8(c + 2, b);
        let b = m.r8(tp + 0x1d);
        m.w8(c + 3, b);
        let t = m.r32(TICK);
        m.w32(c + 4, t);
        for k in 0..4 {
            let b = m.r8(op + 6 + k);
            m.w8(c + 8 + k, b);
        }
        m.w8(c + 0xf, 0xff);
        if m.r16(tp + 8) == 0 {
            let b5 = m.r8(op + 5);
            m.w8(f.at(0x18), 0x7a);
            let pic = m.r8(tp + 0x19);
            m.w8(f.at(0x17), pic);
            m.w16(f.at(0x14), u16::from(b5));
            let b4 = u32::from(m.r8(op + 4));
            let args = peek(m, &[b4, msg], msg, 4);
            host.call(m, 0x4d6530, MSGS, &args);
        }
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x51a880, msg, &[]);
    f.seh_leave(m);
    r
}

/// 0x00541840 `op_set_lobby_player_profile` (operación en ECX). Marco SEH (`0x66363B`) y CString temporal en `esp-0x10`. Con interfaz
/// (`0x6CBB94`) guarda el perfil del jugador `j = op+4`: color `op+5 & 0x7F` (`ui+0x1C4`), listo `op+6` (`+0x1CC`), espectador
/// `op+5 & 0x80` (`+0x1FC`); si es el jugador local, deja de ser espectador y ya estaba presente, llama a la virtual `+0x34` del panel
/// `ui+0x10E0` con 1. Lo marca presente (`+0x1F4`), recorta su nombre (`ui+0x164+j·4`) a `op+7` caracteres (`0x64D230`/`0x64F340`), le
/// añade hasta 12 bytes de `op+8` (`0x64F60A`) y refresca (`0x43E3C0`, `0x439DB0`). Devuelve EAX de la última llamada (0 sin interfaz).
pub fn op_set_lobby_player_profile(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66363b);
    let s2 = f.at(0x10);
    m.w32(s2, op);
    let ui = m.r32(0x6cbb94);
    if ui == 0 {
        f.seh_leave(m);
        return 0;
    }
    let j = u32::from(m.r8(op + 4));
    let b5 = m.r8(op + 5);
    m.w8(j + ui + 0x1c4, b5 & 0x7f);
    let ready = u8::from(m.r8(op + 6) != 0);
    let ui = m.r32(0x6cbb94);
    m.w8(j + ui + 0x1cc, ready);
    let spec = u8::from(m.r8(op + 5) & 0x80 == 0x80);
    let ui = m.r32(0x6cbb94);
    m.w8(j + ui + 0x1fc, spec);
    let mut ui = m.r32(0x6cbb94);
    if j == m.r32(0x6df31c) && m.r8(j + ui + 0x1fc) == 0 && m.r8(j + ui + 0x1f4) != 0 {
        let vt = m.r32(ui + 0x10e0);
        let fun = m.r32(vt + 0x34);
        host.call(m, fun, ui + 0x10e0, &[1]);
        ui = m.r32(0x6cbb94);
    }
    m.w8(j + ui + 0x1f4, 1);
    let ui = m.r32(0x6cbb94);
    let st = ui + j * 4 + 0x164;
    let data = m.r32(st);
    let len = m.r32(data.wrapping_sub(8)) as i32;
    let n = u32::from(m.r8(op + 7));
    if len > n as i32 {
        let args = peek(m, &[s2, n], s2, 1);
        let r = host.call(m, 0x64d230, st, &args);
        f.seh_state(m, 0);
        let ui = m.r32(0x6cbb94);
        let args = peek(m, &[r], r, 1);
        host.call(m, 0x64f340, ui + j * 4 + 0x164, &args);
        f.seh_state(m, 0xffff_ffff);
        host.call(m, CSTR_DTOR, s2, &[]);
    }
    let mut p = op + 8;
    let mut k = 0;
    loop {
        let c = m.r8(p);
        if c == 0 || k >= 0xc {
            break;
        }
        k += 1;
        let ui = m.r32(0x6cbb94);
        let j = u32::from(m.r8(op + 4));
        host.call(m, 0x64f60a, ui + j * 4 + 0x164, &[u32::from(c)]);
        p += 1;
    }
    let ui = m.r32(0x6cbb94);
    host.call(m, 0x43e3c0, ui, &[]);
    let ui = m.r32(0x6cbb94);
    let r = host.call(m, 0x439db0, ui, &[]);
    f.seh_leave(m);
    r
}

/// 0x005110E0 `script_blob_pool_register_group_scripts` (grupo de guiones en ECX, stdcall de 2: número de grupo y marca). Marco SEH
/// (`0x6630D0`), CString local en `esp-0x10` y un segundo CString construido en el hueco del primer argumento. Forma la clave del grupo
/// (`0x64DF80` con `0x6C039C`), lee su valor del INI (`0x4BE060` sobre `0x6DD524`, búfer `0x6DBB58`), lo pasa a CString y lo recorre
/// por espacios (`0x64F6DC`, `atoi` 0x639CDA): cada número ya presente en la tabla `+4` (registros de 8 bytes: `+4` número, `+6`
/// recuento) suma 1 si la marca está puesta; uno nuevo amplía la tabla (`realloc` 0x63A29F) con `{0, número, marca != 0}`. Devuelve EAX
/// del último destructor.
pub fn script_blob_pool_register_group_scripts(m: &mut dyn Mem, host: &mut dyn OpsHost, pool: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x6630d0);
    let s1 = f.at(0x10);
    let e = m.r32(0x6c7ccc);
    m.w32(s1, e);
    let g = m.r32(f.arg(0));
    f.seh_state(m, 0);
    host.call(m, 0x64df80, 0, &[s1, 0x6c039c, g]);
    let key = m.r32(s1);
    host.call(m, 0x4be060, 0x6dd524, &[key, 0x6c03c0, 0x6dd8b8, 0x6dbb58, 0x1000, 0x6c03a4]);
    let s2 = f.arg(0);
    host.call(m, 0x64f2c1, s2, &[0x6dbb58]);
    m.w8(f.at(4), 1);
    host.call(m, 0x64e2f0, s2, &[]);
    let flag = m.r8(f.arg(1));
    let mut start = 0u32;
    loop {
        let mut end = host.call(m, 0x64f6dc, s2, &[0x20, start]);
        let data = m.r32(s2);
        if end == 0xffff_ffff {
            end = m.r32(data.wrapping_sub(8));
            if (start as i32) >= end as i32 {
                break;
            }
        }
        let v = host.call(m, 0x639cda, 0, &[data.wrapping_add(start)]) as u16;
        let cnt = u32::from(m.r16(pool + 8));
        let mut i = 0u32;
        if cnt > 0 {
            let t = m.r32(pool + 4);
            while i < cnt {
                if m.r16(t + 4 + i * 8) == v {
                    if flag != 0 {
                        let c = m.r16(t + i * 8 + 6).wrapping_add(1);
                        m.w16(t + i * 8 + 6, c);
                    }
                    break;
                }
                i += 1;
            }
        }
        if i >= u32::from(m.r16(pool + 8)) {
            let n = m.r16(pool + 8).wrapping_add(1);
            m.w16(pool + 8, n);
            let old = m.r32(pool + 4);
            let t = host.call(m, 0x63a29f, 0, &[old, u32::from(n) << 3]);
            m.w32(pool + 4, t);
            let n = u32::from(n);
            let fl = u16::from(m.r8(f.arg(1)) != 0);
            m.w16(t + n * 8 - 2, fl);
            m.w16(t + n * 8 - 4, v);
            m.w32(t + n * 8 - 8, 0);
        }
        start = end.wrapping_add(1);
    }
    m.w8(f.at(4), 0);
    host.call(m, CSTR_DTOR, s2, &[]);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, CSTR_DTOR, s1, &[]);
    f.seh_leave(m);
    r
}

/// 0x005116F0 `script_blob_pool_load_script_blob` (grupo en ECX, stdcall de 2: número de guion y hueco). Marco SEH (`0x663114`), CFile
/// local en `esp-0x14` (`0x402740`/`0x402750`) y CString en `esp-0x1C`/`esp-0x18`. Busca el nombre del guion en el INI (`0x64DF80` con
/// `0x6C0464`, `0x4BE060`), forma la ruta (`0x6C04AC` + `0x6DBB58`), abre el fichero (`0x402760(ruta, 0x8020)`) y, si tiene longitud
/// (`0x402860`), lo lee entero en un búfer nuevo (`malloc`, `0x4027A0`) guardado en `tabla[hueco·8]`. Devuelve AL = 1 si lo cargó.
pub fn script_blob_pool_load_script_blob(m: &mut dyn Mem, host: &mut dyn OpsHost, pool: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x663114);
    let (s1, s2, file) = (f.at(0x1c), f.at(0x18), f.at(0x14));
    host.call(m, 0x402740, file, &[]);
    f.seh_state(m, 0);
    let e = m.r32(0x6c7ccc);
    m.w32(s1, e);
    let idx = m.r32(f.arg(0)) & 0xffff;
    m.w8(f.at(4), 1);
    host.call(m, 0x64df80, 0, &[s1, 0x6c0464, idx]);
    let key = m.r32(s1);
    let found = host.call(m, 0x4be060, 0x6dd524, &[0x6c0484, key, 0x6dd8ba, 0x6dbb58, 0x1000, 0x6c0468]);
    let mut ok = false;
    if found != 0 {
        host.call(m, 0x64f2c1, s2, &[0x6c04ac]);
        m.w8(f.at(4), 2);
        host.call(m, 0x64f5e3, s2, &[0x6dbb58]);
        let path = m.r32(s2);
        if host.call(m, 0x402760, file, &[path, 0x8020]) != 0 {
            let len = host.call(m, 0x402860, file, &[]);
            if len != 0 {
                let p = host.call(m, MALLOC, 0, &[len]);
                let slot = m.r32(f.arg(1));
                let t = m.r32(pool + 4);
                m.w32(t.wrapping_add(slot.wrapping_mul(8)), p);
                let t = m.r32(pool + 4);
                let b = m.r32(t.wrapping_add(slot.wrapping_mul(8)));
                host.call(m, 0x4027a0, file, &[b, len]);
                ok = true;
            }
            host.call(m, 0x402780, file, &[]);
        }
        m.w8(f.at(4), 1);
        host.call(m, CSTR_DTOR, s2, &[]);
    }
    m.w8(f.at(4), 0);
    host.call(m, CSTR_DTOR, s1, &[]);
    f.seh_state(m, 0xffff_ffff);
    let last = host.call(m, 0x402750, file, &[]);
    f.seh_leave(m);
    (last & 0xffff_ff00) | u32::from(ok)
}

/// 0x00544590 `load_received_savegame_blob`. Marco SEH (`0x663723`) y lector local en `esp-0x14` (`0x54C3F0`/`0x54C400`). Crea o
/// prepara el gestor `0x6CBDC8` (`malloc(0x2C)` + `0x444BA0`, `0x444D30` si `+0x28 != 1`) y carga la partida recibida (`0x6DF304`):
/// tal cual (`0x549F40(+4, 0)`) si su tamaño es ≤ 0, o descomprimida en un búfer temporal (`0x54C480`). Después marca como
/// desconectados (`+8 = 0x8001`) a los comerciantes libres desde el 0x14. Devuelve AL = 1 (0 si no hubo memoria).
pub fn load_received_savegame_blob(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x663723);
    let rd = f.at(0x14);
    host.call(m, 0x54c3f0, rd, &[]);
    f.seh_state(m, 0);
    if m.r32(0x6cbdc8) == 0 {
        let p = host.call(m, MALLOC, 0, &[0x2c]);
        m.w32(f.at(0x10), p);
        m.w8(f.at(4), 1);
        let g = if p != 0 { host.call(m, 0x444ba0, p, &[]) } else { 0 };
        m.w32(0x6cbdc8, g);
        let b = m.r8(g + 0x28);
        m.w8(f.at(4), 0);
        if b != 1 {
            host.call(m, 0x444d30, g, &[]);
        }
    }
    let blk = m.r32(0x6df304);
    let size = m.r32(blk) as i32;
    if size <= 0 {
        host.call(m, 0x549f40, 0x6df2f0, &[blk + 4, 0]);
    } else {
        let p = host.call(m, MALLOC, 0, &[size as u32 & 0x3fff_ffff]);
        if p == 0 {
            f.seh_state(m, 0xffff_ffff);
            let r = host.call(m, 0x54c400, rd, &[]);
            f.seh_leave(m);
            return r & 0xffff_ff00;
        }
        let blk = m.r32(0x6df304);
        host.call(m, 0x54c480, rd, &[blk, p]);
        host.call(m, 0x549f40, 0x6df2f0, &[p, 0]);
        host.call(m, FREE, 0, &[p]);
    }
    let mut i = 0x14u32;
    while i < u32::from(m.r16(TOWN_COUNT)) {
        let tp = host.call(m, 0x5303c0, WORLD, &[i]);
        if m.r16(tp + 8) == 0 {
            let tp = host.call(m, 0x5303c0, WORLD, &[i]);
            m.w16(tp + 8, 0x8001);
        }
        i += 1;
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x54c400, rd, &[]);
    f.seh_leave(m);
    (r & 0xffff_ff00) | 1
}

/// 0x00549190 `handle_player_left_game` (stdcall de 2: marca y comerciante). Marco SEH (`0x663937`) y CString local en `esp-0x10`.
/// Marca al comerciante como desconectado (`+8 = 0x8001`), forma el aviso con su título (`0x6A0608[rango + (+0x1A & 1)·8]`, rango
/// `+0x39C + [+0x19]`, que el original guarda en el byte bajo de su segundo argumento), nombre (`+0xE8`, `+0xE4`) y ciudad
/// (`0x6DDA00`), y lo muestra (`0x64EFC8` en un hueco de la pila + `0x42B4C0` de `0x6CBB40`). Sin la marca, muestra también el segundo
/// aviso (`0x6C35E8`). Devuelve EAX del destructor.
pub fn handle_player_left_game(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x663937);
    let s = f.at(0x10);
    let e = m.r32(0x6c7ccc);
    m.w32(s, e);
    f.seh_state(m, 0);
    let idx = m.r32(f.arg(1));
    if idx < u32::from(m.r16(TOWN_COUNT)) {
        let tp = host.call(m, 0x5303c0, WORLD, &[idx]);
        let b19 = u32::from(m.r8(tp + 0x19));
        let e4 = m.r32(tp + 0xe4);
        m.w16(tp + 8, 0x8001);
        let rank = m.r8(b19 + tp + 0x39c);
        let town = m.r32(b19 * 4 + 0x6dda00);
        m.w8(f.arg(1), rank);
        let e8 = m.r32(tp + 0xe8);
        let sex = u32::from(m.r8(tp + 0x1a)) & 1;
        let title = m.r32((u32::from(rank) + sex * 8) * 4 + 0x6a0608);
        let fmt = m.r32(0x6c35e4);
        host.call(m, 0x64df80, 0, &[s, fmt, title, e8, e4, town]);
        let show = |m: &mut dyn Mem, host: &mut dyn OpsHost, ecx: u32| {
            let ui = m.r32(0x6cbb40);
            if ui != 0 {
                let slot = f.push(m, 0x18, ecx);
                m.w32(f.arg(1), slot);
                host.call(m, 0x64efc8, slot, &[s]);
                let a = m.r32(slot);
                let ui = m.r32(0x6cbb40);
                host.call(m, 0x42b4c0, ui, &[a]);
            }
        };
        show(m, host, fmt);
        if m.r32(f.arg(0)) == 0 {
            let e4 = m.r32(tp + 0xe4);
            let e8 = m.r32(tp + 0xe8);
            let fmt = m.r32(0x6c35e8);
            host.call(m, 0x64df80, 0, &[s, fmt, e8, e4]);
            show(m, host, s);
        }
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, CSTR_DTOR, s, &[]);
    f.seh_leave(m);
    r
}

/// 0x004D7B10 `message_notice_for_new_message` (stdcall de 2: comerciante u16 y mensaje). Marco SEH (`0x662A04`). Con interfaz
/// (`0x6E51AC`) y si el destinatario es el jugador que mira (`0x6DFC14`; en partida compartida `0x6DFC18`, a otros vivos solo les marca
/// `msg+1 |= 0x40`) y no hay ventana modal (`0x6E59CC == 0xFF`, `0x6E59CD`/`0x6E59FC`), avisa según las preferencias `+0x26` del
/// comerciante y la clase del mensaje (`0x6C0198[msg+4]`): sonido (`0x443150`) y texto (`0x64DF80` con `0x6A4570/74/78`, texto propio
/// `msg+0xC` para los códigos 0x3C..0x40 o `0x6A52D0[código]`) en un CString construido en el hueco del primer argumento, copiado a la
/// pila (`0x64EFC8`) y entregado a `0x42BB20` + `0x4237D0`. Devuelve el EAX del original.
pub fn message_notice_for_new_message(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x662a04);
    let leave = |m: &mut dyn Mem, r: u32| {
        f.seh_leave(m);
        r
    };
    if m.r32(0x6e51ac) == 0 {
        return leave(m, 0);
    }
    let who = m.r32(f.arg(0)) & 0xffff;
    let tp = host.call(m, 0x5303c0, WORLD, &[who]);
    let me = m.r32(0x6dfc14);
    let msg = m.r32(f.arg(1));
    if m.r32(0x6dfc18) != 0 {
        if !(m.r32(0x6dfc08) == 0 && who == me) {
            if m.r16(tp + 8) != 0 {
                return leave(m, me);
            }
            let b = m.r8(msg + 1) | 0x40;
            m.w8(msg + 1, b);
            return leave(m, msg);
        }
    } else if who != me {
        return leave(m, me);
    }
    if m.r8(0x6e59cc) != 0xff {
        return leave(m, me);
    }
    let a = m.r8(0x6e59cd);
    if a != 0 {
        let b = m.r8(0x6e59fc);
        if b == 0 {
            return leave(m, me & 0xffff_ff00);
        }
    }
    let s = f.arg(0);
    let e = m.r32(0x6c7ccc);
    m.w32(s, e);
    let fl = m.r8(tp + 0x26);
    f.seh_state(m, 0);
    let code = u32::from(m.r8(msg + 4));
    let class = m.r8(code + 0x6c0198);
    if (fl & 1 != 0 && class == 1) || (fl & 2 != 0 && class == 2) || (fl & 4 != 0 && class == 3) {
        let snd = m.r32(0x6cbdb8);
        host.call(m, 0x443150, snd, &[0x1770, 0, 0, 0xffff_ffff, 0x40, 0, 1]);
    }
    let fl = m.r8(tp + 0x26);
    let fmt = if fl & 8 != 0 && class == 1 {
        Some(0x6a4570)
    } else if fl & 0x10 != 0 && class == 2 {
        Some(0x6a4574)
    } else if fl & 0x20 != 0 && class == 3 {
        Some(0x6a4578)
    } else {
        None
    };
    if let Some(fa) = fmt {
        let txt = if (0x3c..=0x40).contains(&code) { m.r32(msg + 0xc) } else { m.r32(code * 4 + 0x6a52d0) };
        let fv = m.r32(fa);
        host.call(m, 0x64df80, 0, &[s, fv, txt]);
        let pushed = if fa == 0x6a4574 { fv } else { txt };
        let slot = f.push(m, 0x18, pushed);
        m.w32(f.arg(1), slot);
        host.call(m, 0x64efc8, slot, &[s]);
        let ui = m.r32(0x6cbb40);
        let a = m.r32(slot);
        host.call(m, 0x42bb20, ui, &[a]);
        let ui = m.r32(0x6cbb40);
        host.call(m, 0x4237d0, ui, &[]);
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, CSTR_DTOR, s, &[]);
    leave(m, r)
}

/// 0x0052ACD0 `op_repair_ship` (ciudad en ECX, stdcall de 1: barco). Marco SEH (`0x663318`), ciudad en `esp-0x24`, coste acumulado en
/// `esp-0x20` y mensaje local en `esp-0x1C`. Coste: `0x52A9E0` del barco o la suma de los barcos de su convoy (`0x6DD7A8 + c·0x3C`,
/// cadena desde `+0xA`, siguiente en `+6`), guardado en el hueco del argumento. Con dueño vivo: si no hay daño (casco `+0x18` ≥ máximo
/// `+0x14`, sumados en convoy) o no le alcanza el dinero avisa con el mensaje 2 y devuelve 0; si no, cobra (`tp+0`, `tp+0x4AC`), repara
/// con `0x4F9120(+0x39, daño)` y devuelve 1. Siempre deja `+0x134 = 4` y `+0x138 = 0` al reparar (`+0x134 = 0` al rechazar).
pub fn op_repair_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, town: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x663318);
    let msg = f.at(0x1c);
    m.w32(f.at(0x24), town);
    host.call(m, 0x51a850, msg, &[]);
    let ship = m.r32(f.arg(0));
    f.seh_state(m, 0);
    m.w32(f.at(0x20), 0);
    let di = m.r16(ship + 8);
    let ncv = m.r16(0x6dd896);
    let cost = if di >= ncv {
        let c = host.call(m, 0x52a9e0, town, &[ship]);
        m.w32(f.arg(0), c);
        c
    } else {
        let cv = m.r32(0x6dd7a8).wrapping_add(u32::from(di) * 0x3c);
        let cnt = m.r32(SHIP_COUNT) & 0xffff;
        let mut i = u32::from(m.r16(cv + 0xa));
        while i < cnt {
            let s = m.r32(SHIP_ARRAY).wrapping_add(i * SHIP_SIZE);
            i = u32::from(m.r16(s + 6));
            let c = host.call(m, 0x52a9e0, town, &[s]);
            let sum = m.r32(f.at(0x20)).wrapping_add(c);
            m.w32(f.at(0x20), sum);
        }
        let sum = m.r32(f.at(0x20));
        m.w32(f.arg(0), sum);
        sum
    };
    let owner = m.r8(ship);
    let (mut hp, mut max) = (0i32, 0i32);
    let finish = |m: &mut dyn Mem, host: &mut dyn OpsHost, ok: bool| -> u32 {
        f.seh_state(m, 0xffff_ffff);
        let r = host.call(m, 0x51a880, msg, &[]);
        f.seh_leave(m);
        (r & 0xffff_ff00) | u32::from(ok)
    };
    if u16::from(owner) >= m.r16(TOWN_COUNT) {
        m.w16(ship + 0x134, 4);
        m.w16(ship + 0x138, 0);
        return finish(m, host, true);
    }
    if m.r16(ship + 8) < m.r16(0x6dd896) {
        let cv = m.r32(0x6dd7a8).wrapping_add(u32::from(m.r16(ship + 8)) * 0x3c);
        let arr = m.r32(SHIP_ARRAY);
        let mut s = arr.wrapping_add(u32::from(m.r16(cv + 0xa)) * SHIP_SIZE);
        while s != 0 {
            hp = hp.wrapping_add(m.r32(s + 0x18) as i32);
            max = max.wrapping_add(m.r32(s + 0x14) as i32);
            let nx = m.r16(s + 6);
            if nx >= m.r16(SHIP_COUNT) {
                break;
            }
            s = arr.wrapping_add(u32::from(nx) * SHIP_SIZE);
        }
    } else {
        hp = m.r32(ship + 0x18) as i32;
        max = m.r32(ship + 0x14) as i32;
    }
    let tp = host.call(m, 0x5303c0, WORLD, &[u32::from(owner)]);
    if m.r16(tp + 8) == 0 {
        let kind = |m: &mut dyn Mem| if m.r16(ship + 8) < m.r16(0x6dd896) { 4 } else { u16::from(m.r8(ship + 0xe)) };
        if hp >= max || (m.r32(tp) as i32) < cost as i32 {
            m.w8(f.at(0x18), 2);
            let a = m.r16(ship + 0x15e);
            m.w16(f.at(0x14), a);
            let b = m.r8(town + 0x2c0);
            m.w8(f.at(0x17), b);
            let k = kind(m);
            m.w16(f.at(0x12), k);
            m.w32(f.at(0x10), if hp >= max { 0 } else { cost });
            let o = u32::from(m.r8(ship));
            let args = peek(m, &[o, msg], msg, 4);
            host.call(m, 0x4d6530, MSGS, &args);
            m.w16(ship + 0x134, 0);
            return finish(m, host, false);
        }
        m.w8(f.at(0x18), 1);
        let b = m.r8(town + 0x2c0);
        m.w8(f.at(0x17), b);
        let k = kind(m);
        m.w16(f.at(0x14), k);
        m.w16(f.at(0x12), (hp / (max / 100)) as u16);
        m.w32(f.at(0x10), cost);
    }
    let money = m.r32(tp).wrapping_sub(cost);
    m.w32(tp, money);
    let spent = m.r32(tp + 0x4ac).wrapping_add(cost);
    m.w32(tp + 0x4ac, spent);
    let b39 = u32::from(m.r8(ship + 0x39));
    host.call(m, 0x4f9120, tp, &[b39, max.wrapping_sub(hp) as u32]);
    m.w16(ship + 0x134, 4);
    m.w16(ship + 0x138, 0);
    finish(m, host, true)
}

/// 0x0051E440 `load_town_ini_entry` (ficha de ciudad en ECX, stdcall de 1: índice). Marco SEH (`0x6631F0`), CString del fichero en
/// `esp-0x10` (`0x6C06A4`) y de la sección en `esp-0x14` (`0x64DF80` con `0x6C06C0`). Lee del INI (`0x4BE0B0(sección, clave,
/// omisión, fichero)` sobre `0x6DD524`) las coordenadas y datos `+0x20/+0x24/+8/+0xC/+0/+4/+0x10/+0x14/+0x18/+0x1C`, `+0x2C` (u16),
/// `+0x2E` (omisión 0xFF), `+0x2F = (valor == 1)`, `+0x30`, `+0x31`, dos cadenas (`0x4BE060`: primer byte en `+0x33`, CString `+0x28`)
/// y `+0x32` (omisión 0x2D). Devuelve AL = 1 si las dos cadenas existían.
pub fn load_town_ini_entry(m: &mut dyn Mem, host: &mut dyn OpsHost, t: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x6631f0);
    let (sec, file) = (f.at(0x14), f.at(0x10));
    host.call(m, 0x64f2c1, file, &[0x6c06a4]);
    f.seh_state(m, 0);
    let e = m.r32(0x6c7ccc);
    m.w32(sec, e);
    let idx = m.r32(f.arg(0)) & 0xff;
    m.w8(f.at(4), 1);
    host.call(m, 0x64df80, 0, &[sec, 0x6c06c0, idx]);
    let mut ok = true;
    let mut get = |m: &mut dyn Mem, host: &mut dyn OpsHost, key: u32, def: u32| {
        let (s, fl) = (m.r32(sec), m.r32(file));
        host.call(m, 0x4be0b0, 0x6dd524, &[s, key, def, fl])
    };
    for (key, off) in [
        (0x6c06c8, 0x20),
        (0x6c06d4, 0x24),
        (0x6c06e0, 8),
        (0x6c06ec, 0xc),
        (0x6c06f8, 0),
        (0x6c0708, 4),
        (0x6c0718, 0x10),
        (0x6c072c, 0x14),
        (0x6c0740, 0x18),
        (0x6c0754, 0x1c),
    ] {
        let v = get(m, host, key, 0);
        m.w32(t + off, v);
    }
    let v = get(m, host, 0x6c0768, 0);
    m.w16(t + 0x2c, v as u16);
    let v = get(m, host, 0x6c0784, 0xff);
    m.w8(t + 0x2e, v as u8);
    let v = get(m, host, 0x6c0790, 0);
    m.w8(t + 0x2f, u8::from(v == 1));
    let v = get(m, host, 0x6c0798, 0);
    m.w8(t + 0x30, v as u8);
    let v = get(m, host, 0x6c07a0, 0);
    m.w8(t + 0x31, v as u8);
    let (s, fl) = (m.r32(sec), m.r32(file));
    if host.call(m, 0x4be060, 0x6dd524, &[s, 0x6c07b8, 0x6de3b2, 0x6dbb58, 0x1000, fl]) != 0 {
        let b = m.r8(0x6dbb58);
        m.w8(t + 0x33, b);
    } else {
        ok = false;
    }
    let (s, fl) = (m.r32(sec), m.r32(file));
    if host.call(m, 0x4be060, 0x6dd524, &[s, 0x6c07d4, 0x6de3b3, 0x6dbb58, 0x1000, fl]) != 0 {
        host.call(m, 0x64f390, t + 0x28, &[0x6dbb58]);
    } else {
        ok = false;
    }
    let v = get(m, host, 0x6c07e8, 0x2d);
    m.w8(t + 0x32, v as u8);
    m.w8(f.at(4), 0);
    host.call(m, CSTR_DTOR, sec, &[]);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, CSTR_DTOR, file, &[]);
    f.seh_leave(m);
    (r & 0xffff_ff00) | u32::from(ok)
}

/// 0x005448A0 `custom_map_load_from_archive` (mapa propio en ECX, stdcall de 1: CString por valor con el nombre). Marco SEH
/// (`0x663782`), objeto de descompresión en `esp-0x20` (`0x54C3F0`/`0x54C480`/`0x54C400`), temporales CString en `esp-0x14`/`esp-0x18`
/// (nombre = `0x6CBDF8` + nombre + `0x6C376C`), entrada del archivo en `esp-0x28`/`esp-0x2C` (`0x6389EE`, primero en el archivo
/// `[0x6DCCC8]`, después en disco). Copia del bloque descomprimido el recuento (máx. 0x28), cinco tablas (`+4`, `+0xA4`, `+0x144`,
/// `+0x16C`, `+0x194`) y 40 nombres CString en `+0x1BC`. Devuelve AL = 0 si no hay entrada.
pub fn custom_map_load_from_archive(m: &mut dyn Mem, host: &mut dyn OpsHost, t: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x663782);
    let (obj, tmp1, tmp2, src, len, flag, hnd, bufl) = (f.at(0x20), f.at(0x14), f.at(0x18), f.at(0x28), f.at(0x2c), f.at(0x1c), f.at(0x24), f.at(0x10));
    let name = f.arg(0);
    f.seh_state(m, 0);
    host.call(m, 0x54c3f0, obj, &[]);
    m.w8(f.at(4), 1);
    let r = host.call(m, 0x64f436, tmp1, &[tmp1, 0x6cbdf8, name]);
    m.w8(f.at(4), 2);
    let r = host.call(m, 0x64f49c, tmp1, &[tmp2, r, 0x6c376c]);
    m.w8(f.at(4), 3);
    host.call(m, 0x64f340, name, &[r]);
    m.w8(f.at(4), 2);
    host.call(m, CSTR_DTOR, tmp2, &[]);
    m.w8(f.at(4), 1);
    host.call(m, CSTR_DTOR, tmp1, &[]);
    let (nm, ar) = (m.r32(name), m.r32(0x6dccc8));
    let mut h = host.call(m, 0x6389ee, 0, &[ar, nm, src, len]);
    m.w32(hnd, h);
    let mut p = m.r32(src);
    if p == 0 {
        let nm = m.r32(name);
        m.w32(flag, 0);
        h = host.call(m, 0x6389ee, 0, &[0, nm, src, len]);
        m.w32(hnd, h);
        p = m.r32(src);
    } else {
        m.w32(flag, 1);
    }
    let r = if h == 0 {
        m.w8(f.at(4), 0);
        host.call(m, 0x54c400, obj, &[]);
        f.seh_state(m, 0xffff_ffff);
        host.call(m, CSTR_DTOR, name, &[]) & 0xffff_ff00
    } else {
        if p != 0 {
            let n = m.r32(p) & 0x3fff_ffff;
            m.w32(len, n);
            let b = host.call(m, 0x64f7b9, 0, &[n]);
            m.w32(bufl, b);
            if b != 0 {
                let s = m.r32(src);
                host.call(m, 0x54c480, obj, &[s, b]);
                let cnt = m.r32(b);
                m.w32(t, cnt);
                let mut q = b + 4;
                if m.r16(q) == 0xaffe {
                    let x = u32::from(m.r16(q + 2));
                    q += 4;
                    q = q.wrapping_add(x);
                }
                if cnt <= 0x28 {
                    for (off, sz, w) in [(4, 0xa0, 4), (0xa4, 0xa0, 4), (0x144, 0x28, 1), (0x16c, 0x28, 1), (0x194, 0x28, 1)] {
                        for k in 0..sz {
                            m.w8(t + off + k, 0);
                        }
                        let c = m.r32(t).wrapping_mul(w);
                        for k in 0..c {
                            let v = m.r8(q + k);
                            m.w8(t + off + k, v);
                        }
                        q = q.wrapping_add(c);
                    }
                    for i in 0..0x28 {
                        m.w32(len, i);
                        host.call(m, 0x64f1de, t + 0x1bc + i * 4, &[]);
                    }
                    m.w32(len, 0);
                    for i in 0..0x28u32 {
                        let mut eax = 0u32;
                        loop {
                            let ch = m.r8(q);
                            if ch == 0 {
                                break;
                            }
                            let a = (eax & 0xffff_ff00) | u32::from(ch);
                            q += 1;
                            eax = host.call(m, 0x64f60a, t + 0x1bc + i * 4, &[a]);
                        }
                        q += 1;
                        m.w32(len, i + 1);
                    }
                }
                let b = m.r32(bufl);
                host.call(m, 0x64f7e2, 0, &[b]);
                h = m.r32(hnd);
            }
        }
        let fl = m.r32(flag);
        let a = if fl != 0 { m.r32(0x6dccc8) } else { 0 };
        host.call(m, 0x6389f4, 0, &[a, h]);
        m.w8(f.at(4), 0);
        host.call(m, 0x54c400, obj, &[]);
        f.seh_state(m, 0xffff_ffff);
        (host.call(m, CSTR_DTOR, name, &[]) & 0xffff_ff00) | 1
    };
    f.seh_leave(m);
    r
}

/// 0x00544B80 `custom_map_save_to_file` (mapa propio en ECX, stdcall de 1: CString por valor con el nombre). Marco EBP con SEH
/// (`0x6637AC`, try/catch de CFile): CFile local en `ebp-0x38`, compresor en `ebp-0x28`, búferes en `ebp-0x20`/`ebp-0x24`, iterador y
/// contador en `ebp-0x18`/`ebp-0x1C`, ruta CString en `ebp-0x14` (`0x64DF80` con `0x6C3774`), ESP guardado en `ebp-0x10`. Serializa
/// recuento (recortado a 0x28 en el mapa), cinco tablas y 40 nombres terminados en cero, crea el directorio (`[0x66A2B0]`), abre,
/// comprime (`0x54C410`) y escribe. Las ramas `catch` sólo se alcanzan con excepción de CFile y no se modelan.
pub fn custom_map_save_to_file(m: &mut dyn Mem, host: &mut dyn OpsHost, t: u32, esp: u32) -> u32 {
    let f = Frame::new(esp.wrapping_sub(4));
    f.seh_enter(m, 0x6637ac);
    let bp = |k: u32| esp.wrapping_sub(4 + k);
    m.w32(bp(0x10), esp.wrapping_sub(0x48));
    let (file, comp, path, name) = (bp(0x38), bp(0x28), bp(0x14), f.arg(1));
    f.seh_state(m, 0);
    host.call(m, 0x653b0b, file, &[]);
    m.w8(f.at(4), 1);
    host.call(m, 0x54c3f0, comp, &[]);
    m.w8(f.at(4), 2);
    let strs = t + 0x1bc;
    m.w32(bp(0x1c), strs);
    let mut total = 0x284u32;
    for i in 0..0x28 {
        let d = m.r32(strs + 4 * i);
        total = total.wrapping_add(m.r32(d.wrapping_sub(8)));
    }
    let b1 = host.call(m, 0x64f7b9, 0, &[total]);
    m.w32(bp(0x20), b1);
    let b2 = host.call(m, 0x64f7b9, 0, &[total.wrapping_add(4)]);
    m.w32(bp(0x24), b2);
    if b1 != 0 && b2 != 0 {
        if m.r32(t) > 0x28 {
            m.w32(t, 0x28);
        }
        let cnt = m.r32(t);
        m.w32(b1, cnt);
        let mut o = 4u32;
        for (src, w) in [(4, 4), (0xa4, 4), (0x144, 1), (0x16c, 1), (0x194, 1)] {
            for k in 0..cnt * w {
                let v = m.r8(t + src + k);
                m.w8(b1 + o + k, v);
            }
            o += cnt * w;
        }
        m.w32(bp(0x18), strs);
        for i in 0..0x28u32 {
            m.w32(bp(0x1c), 0x28 - i);
            let d = m.r32(strs + 4 * i);
            let n = m.r32(d.wrapping_sub(8));
            for k in 0..n {
                let v = m.r8(d + k);
                m.w8(b1 + o + k, v);
            }
            o += n;
            m.w8(b1 + o, 0);
            o += 1;
            m.w32(bp(0x18), strs + 4 * (i + 1));
        }
        m.w32(bp(0x1c), 0);
        let e = m.r32(0x6c7ccc);
        m.w32(path, e);
        let (nd, pre) = (m.r32(name), m.r32(0x6cbdf8));
        m.w8(f.at(4), 3);
        host.call(m, 0x64df80, 0, &[path, 0x6c3774, pre, nd]);
        let mkdir = m.r32(0x66a2b0);
        host.call(m, mkdir, nd, &[0x6c3780, 0]);
        let p = m.r32(path);
        if host.call(m, 0x653c1d, file, &[p, 0x9001, 0]) != 0 {
            let n = host.call(m, 0x54c410, comp, &[b1, b2, o]);
            m.w8(f.at(4), 4);
            host.call(m, 0x653d74, file, &[b2, n]);
            f.seh_state(m, 3);
            m.w8(f.at(4), 6);
            host.call(m, 0x653e38, file, &[]);
        }
        f.seh_state(m, 2);
        host.call(m, CSTR_DTOR, path, &[]);
        host.call(m, 0x64f7e2, 0, &[b1]);
        host.call(m, 0x64f7e2, 0, &[b2]);
    } else if b1 != 0 {
        host.call(m, 0x64f7e2, 0, &[b1]);
    } else if b2 != 0 {
        host.call(m, 0x64f7e2, 0, &[b2]);
    }
    m.w8(f.at(4), 1);
    host.call(m, 0x54c400, comp, &[]);
    m.w8(f.at(4), 0);
    host.call(m, 0x653b60, file, &[]);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, CSTR_DTOR, name, &[]);
    f.seh_leave(m);
    r
}

fn hdr_copy(m: &mut dyn Mem, out: u32, src: u32, ver: u16) {
    let n = if ver < 0x76 { 0x28 } else if ver < 0x7a { 0x3c } else { 0x4c };
    for k in 0..n {
        let v = m.r8(src + k);
        m.w8(out + k, v);
    }
    if ver < 0x76 {
        for (k, b) in b"???\0".iter().enumerate() {
            m.w8(out + 0x27 + k as u32, *b);
        }
    }
    if ver < 0x7a {
        m.w8(out + 0x3b, 0);
    }
    for k in 0..0x10 {
        m.w8(out + 0x3c + k, 0);
    }
}

/// 0x00547BE0 `savegame_read_header` (stdcall de 3: nombre de 12 caracteres, cabecera de salida, modo). Marco SEH (`0x6638F9`),
/// descompresor en `esp-0x18`, CString de carpeta+nombre en `esp-0x20` y ruta `.pat` en `esp-0x1C` (el temporal de `0x64F49C` usa el
/// hueco del argumento de modo), entrada de archivo en `esp-0x14`/`esp-0x10`, origen (archivo `[0x6DCCC8]` o disco) en `esp-0x21`.
/// Copia 0x28/0x3C/0x4C bytes según la versión (`<0x76`, `<0x7A`), del bloque comprimido o del bruto (`+0xC`); el comprimido limita
/// `+8` a 0xE salvo tipo 6. AL = 0 sin modo (0xFF), sin entrada o sin memoria.
pub fn savegame_read_header(m: &mut dyn Mem, host: &mut dyn OpsHost, _ecx: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x6638f9);
    let (obj, x, y, src, len, flag) = (f.at(0x18), f.at(0x20), f.at(0x1c), f.at(0x14), f.at(0x10), f.at(0x21));
    let (name, out, tmp) = (m.r32(f.arg(0)), m.r32(f.arg(1)), f.arg(2));
    host.call(m, 0x54c3f0, obj, &[]);
    f.seh_state(m, 0);
    let e = m.r32(0x6c7ccc);
    m.w32(y, e);
    m.w32(x, e);
    m.w8(f.at(4), 2);
    m.w8(flag, 1);
    let mode = m.r32(tmp);
    let mut ok = false;
    'body: {
        if mode & 0xff == 0xff {
            break 'body;
        }
        let mut eax = mode & 0xff;
        let pre = match eax {
            3 => Some(0x6c38d0),
            4 => Some(0x6c38e8),
            5 => Some(0x6c38dc),
            0..=2 => Some(0x6c38f0),
            _ => None,
        };
        if let Some(p) = pre {
            eax = host.call(m, 0x64f390, x, &[p]);
        }
        for i in 0..12 {
            let ch = m.r8(name + i);
            if ch == 0 {
                break;
            }
            eax = host.call(m, 0x64f60a, x, &[(eax & 0xffff_ff00) | u32::from(ch)]);
        }
        let r = host.call(m, 0x64f49c, tmp, &[tmp, x, 0x6c38fc]);
        m.w8(f.at(4), 3);
        host.call(m, 0x64f340, y, &[r]);
        m.w8(f.at(4), 2);
        host.call(m, CSTR_DTOR, tmp, &[]);
        let (yd, ar) = (m.r32(y), m.r32(0x6dccc8));
        let mut h = host.call(m, 0x6389ee, 0, &[ar, yd, src, len]);
        if h == 0 {
            let yd = m.r32(y);
            h = host.call(m, 0x6389ee, 0, &[0, yd, src, len]);
            if h == 0 {
                break 'body;
            }
            m.w8(flag, 0);
        }
        let p = m.r32(src);
        let n = m.r32(p) as i32;
        let release = |m: &mut dyn Mem, host: &mut dyn OpsHost| {
            let a = if m.r8(flag) != 0 { m.r32(0x6dccc8) } else { 0 };
            host.call(m, 0x6389f4, 0, &[a, h]);
        };
        if n > 0 {
            let n = n as u32 & 0x3fff_ffff;
            m.w32(len, n);
            let b = host.call(m, 0x64f7b9, 0, &[n]);
            if b == 0 {
                release(m, host);
                break 'body;
            }
            let s = m.r32(src);
            host.call(m, 0x54c480, obj, &[s, b]);
            release(m, host);
            let ver = m.r16(b + 2);
            hdr_copy(m, out, b + 4, ver);
            if m.r8(out + 0xf) != 6 && m.r8(out + 8) > 0xe {
                m.w8(out + 8, 0xe);
            }
            host.call(m, 0x64f7e2, 0, &[b]);
        } else {
            let ver = m.r16(p + 6);
            hdr_copy(m, out, p + 0xc, ver);
            release(m, host);
        }
        ok = true;
    }
    m.w8(f.at(4), 1);
    host.call(m, CSTR_DTOR, x, &[]);
    m.w8(f.at(4), 0);
    host.call(m, CSTR_DTOR, y, &[]);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x54c400, obj, &[]);
    f.seh_leave(m);
    (r & 0xffff_ff00) | u32::from(ok)
}

/// 0x005483D0 `operations_show_ship_event_notification` (operaciones en ECX, stdcall de 2: tipo 0..0x2A, valor). Marco SEH
/// (`0x66390B`), CString local en `esp-0x10`. Según el tipo (tablas `0x548834`/`0x5487E8`) compone el texto con la plantilla
/// `[0x6B28A8 + 4·tipo]` y el nombre del barco (`+0x160`), de la ciudad (`0x6DDA00`) o un valor, y lo muestra copiándolo por valor
/// (`0x64EFC8`, hueco `esp-0x18`, cuya dirección queda en el argumento 2) a `0x42B4C0` sobre `[0x6CBB40]`. Los tipos sin aviso sólo
/// destruyen la cadena.
pub fn operations_show_ship_event_notification(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66390b);
    let s = f.at(0x10);
    let e = m.r32(0x6c7ccc);
    m.w32(s, e);
    let (kind, v) = (m.r32(f.arg(0)), m.r32(f.arg(1)));
    f.seh_state(m, 0);
    let tpl = |m: &mut dyn Mem, k: u32| m.r32(0x6b28a8u32.wrapping_add(k.wrapping_mul(4)));
    let town = |m: &mut dyn Mem, i: u32| m.r32(0x6dda00u32.wrapping_add(i.wrapping_mul(4)));
    let ns = u32::from(m.r16(0x6dd894));
    let ship = |m: &mut dyn Mem| m.r32(0x6dd7a4).wrapping_add(v.wrapping_mul(0x180));
    let nt = m.r32(0x6de4b0);
    let fmt = |m: &mut dyn Mem, host: &mut dyn OpsHost, a: &[u32]| {
        let mut args = vec![s];
        args.extend_from_slice(a);
        host.call(m, 0x64df80, 0, &args);
    };
    let assign = |m: &mut dyn Mem, host: &mut dyn OpsHost, p: u32| {
        host.call(m, 0x64f390, s, &[p]);
    };
    let empty = |m: &mut dyn Mem, host: &mut dyn OpsHost| {
        host.call(m, 0x64f1de, s, &[]);
    };
    // Tabla de bytes en 0x548834 (imagen del ejecutable).
    const SEL: [u8; 0x2b] = [
        0, 1, 2, 3, 4, 5, 6, 6, 7, 18, 6, 8, 18, 1, 8, 2, 18, 2, 18, 2, 2, 1, 9, 2, 2, 18, 2, 1, 1, 10, 2, 2, 1, 1, 11, 12, 13, 14, 2, 2,
        15, 16, 17,
    ];
    let case = if kind > 0x2a { 18 } else { SEL[kind as usize] };
    let mut show = true;
    match case {
        0 => {
            if v < ns {
                let (t, p) = (m.r32(0x6b23f0), ship(m) + 0x160);
                fmt(m, host, &[t, p]);
            } else {
                let t = m.r32(0x6b28a8);
                assign(m, host, t);
            }
        }
        1 => {
            let (t, n) = (tpl(m, kind), town(m, v));
            fmt(m, host, &[t, n]);
        }
        6 | 8 | 7 => {
            if v < ns {
                let sp = ship(m);
                let t = tpl(m, kind);
                let a = if case == 6 {
                    let b = u32::from(m.r8(sp + 0x39));
                    town(m, b)
                } else {
                    sp + 0x160
                };
                let t = if case == 7 { m.r32(0x6b28c8) } else { t };
                fmt(m, host, &[t, a]);
            } else {
                empty(m, host);
            }
        }
        9 => {
            let r = host.call(m, 0x512b20, 0x6ddaa0, &[v]);
            let t = m.r32(0x6b2900);
            fmt(m, host, &[t, r]);
        }
        5 | 3 => {
            if v < ns {
                let sp = ship(m);
                let b = u32::from(m.r8(sp + if case == 5 { 0x38 } else { 0x39 }));
                if b < nt {
                    let n = town(m, b);
                    let x = if case == 5 {
                        let q = u32::from(m.r8(sp + 0xe)) & 3;
                        m.r32(0x6c0e38 + q * 4)
                    } else {
                        m.r32(0x6c3638)
                    };
                    let t = m.r32(0x6b28bc);
                    fmt(m, host, &[t, x, sp + 0x160, n]);
                } else if case == 3 {
                    let t = m.r32(0x6b28b4);
                    fmt(m, host, &[t, sp + 0x160]);
                } else {
                    empty(m, host);
                }
            } else {
                empty(m, host);
            }
        }
        4 => {
            let a = if v < ns { ship(m) + 0x160 } else { 0x6dfc8a };
            let t = m.r32(0x6b28b8);
            fmt(m, host, &[t, a]);
        }
        10 => {
            if v < 2 {
                let t = m.r32(0x6c3594);
                assign(m, host, t);
            } else {
                let t = m.r32(0x6b291c);
                fmt(m, host, &[t, v]);
            }
        }
        11..=15 => {
            let i = [6, 7, 8, 10, 11][case as usize - 11];
            if v < ns {
                let (t, p) = (m.r32(0x6b04f4 + i * 4), ship(m) + 0x160);
                fmt(m, host, &[t, p]);
            }
        }
        16 => {
            let a = town(m, (v >> 8) % nt);
            let b = town(m, (v & 0xff) % nt);
            let t = m.r32(0x6b294c);
            fmt(m, host, &[t, b, a]);
        }
        17 => {
            if m.r32(this + 0x928) == 0 {
                show = false;
            } else {
                let n = town(m, v % nt);
                let t = m.r32(0x6b2950);
                fmt(m, host, &[t, n]);
                let t = tpl(m, kind);
                assign(m, host, t);
            }
        }
        2 => show = false,
        _ => {
            let t = tpl(m, kind);
            assign(m, host, t);
        }
    }
    if show {
        let slot = f.push(m, 0x18, 0);
        m.w32(f.arg(1), slot);
        host.call(m, 0x64efc8, slot, &[s]);
        let (ui, a) = (m.r32(0x6cbb40), m.r32(slot));
        host.call(m, 0x42b4c0, ui, &[a]);
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, CSTR_DTOR, s, &[]);
    f.seh_leave(m);
    r
}

/// Texto y aviso del cambio de jugador de 0x548860: CString en `s`, ficha del jugador (`0x5303C0`), plantilla del rango
/// (`0x4F9430`), aviso de 0x20 bytes en `note` mostrado por `[0x6CBB10]` (vtable `+0xC8` / `0x41D9B0`).
fn hotseat_notice(m: &mut dyn Mem, host: &mut dyn OpsHost, f: &Frame, player: u32, s: u32, note: u32) {
    let p = host.call(m, 0x5303c0, WORLD, &[player]);
    let i = u32::from(m.r8(p + 0x19));
    let cl = m.r8(p + 0x39c + i);
    let town = m.r32(0x6dda00 + i * 4);
    let (e4, e8) = (m.r32(p + 0xe4), m.r32(p + 0xe8));
    m.w8(f.arg(2), cl);
    let b1a = u32::from(m.r8(p + 0x1a));
    let t1 = m.r32(0x6a0608 + (u32::from(cl) + b1a * 8) * 4);
    let r = host.call(m, 0x4f9430, p, &[]) & 0xff;
    let t2 = m.r32(0x69ee48 + r * 4);
    let t3 = m.r32(0x6c0ef8 + b1a * 4);
    let fm = m.r32(0x6c2a34);
    host.call(m, 0x64df80, 0, &[s, fm, t3, t2, t1, e8, e4, town]);
    let sd = m.r32(s);
    m.w32(note, sd);
    m.w8(note + 0x4, 0);
    m.w16(note + 0x6, 0);
    m.w32(note + 0x8, 0);
    m.w8(note + 0xc, 0xff);
    m.w8(note + 0xd, 0);
    m.w8(note + 0xe, 0);
    m.w8(note + 0xf, 0xff);
    m.w32(note + 0x10, 0x3f4c_cccd);
    m.w8(note + 0x14, 1);
    m.w16(note + 0x16, 0);
    m.w32(note + 0x18, 0);
    m.w32(note + 0x1c, 0);
    let ui = m.r32(0x6cbb10);
    let vt = m.r32(ui);
    let fnp = m.r32(vt + 0xc8);
    if host.call(m, fnp, ui, &[]) & 0xff == 0 {
        host.call(m, 0x41d9b0, ui, &[note]);
    }
}

fn vcall1(m: &mut dyn Mem, host: &mut dyn OpsHost, obj: u32, off: u32, a: u32) {
    let vt = m.r32(obj);
    let fnp = m.r32(vt + off);
    host.call(m, fnp, obj, &[a]);
}

/// 0x00548860 `operations_switch_to_hotseat_player` (operaciones en ECX, stdcall de 3: jugador, modo, ciudad). Marco SEH (`0x663925`).
/// Reinicia `+0x914/+0x92C`, guarda el jugador en `+0x924`; sin interfaz (`[0x6CBDD8]`, `[0x6E51AC]`) devuelve el enlace SEH previo.
/// Repinta, marca las ciudades del jugador en `[[0x6CBDD8]+0x28CC]`, cambia entre vista de mapa (modo 0 y ciudad válida) y vista de
/// ciudad, y muestra uno o dos avisos: el primero con su CString en el hueco del argumento 1 y el aviso en `esp-0x4C`, el segundo con
/// CString en `esp-0x50` y aviso en `esp-0x2C`.
pub fn operations_switch_to_hotseat_player(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let prev = f.seh_enter(m, 0x663925);
    let player = m.r32(f.arg(0)) & 0xffff;
    m.w32(this + 0x914, 0);
    m.w32(this + 0x92c, 0);
    m.w32(this + 0x924, player);
    let (gui, map) = (m.r32(0x6cbdd8), m.r32(0x6e51ac));
    if gui == 0 || map == 0 {
        f.seh_leave(m);
        return prev;
    }
    let nt = m.r32(0x6de4b0);
    let town_view = !(m.r16(f.arg(1)) == 0 && (m.r32(f.arg(2)) & 0xffff) < nt);
    host.call(m, 0x50dc50, SHIP_MGR, &[]);
    for _ in 0..5 {
        let ui = m.r32(0x6cbb40);
        host.call(m, 0x42c020, ui, &[]);
    }
    let w = m.r32(0x6dfc14);
    host.call(m, 0x50dde0, SHIP_MGR, &[w]);
    let g = m.r32(0x6cbdd8);
    let arr = m.r32(g + 0x28cc);
    for k in 0..nt {
        m.w8(arr + k, 0);
    }
    let p = host.call(m, 0x5303c0, WORLD, &[player]);
    let mut i = u32::from(m.r16(p + 0xc));
    while i < u32::from(m.r16(0x6de4a8)) {
        let q = host.call(m, 0x530400, WORLD, &[i]);
        let b = m.r8(q + 0x2c6);
        m.w8(f.arg(0), b);
        let g = m.r32(0x6cbdd8);
    let arr = m.r32(g + 0x28cc);
        m.w8(arr + u32::from(b), 1);
        i = u32::from(m.r16(q + 0x2c8));
    }
    let (a1, a2) = (m.r32(f.arg(1)), m.r32(f.arg(2)));
    if town_view {
        let gui = m.r32(0x6cbdd8);
        if host.call(m, 0x4b2200, gui, &[]) & 0xff == 0 {
            vcall1(m, host, gui, 0xcc, 1);
            let gui = m.r32(0x6cbdd8);
            vcall1(m, host, gui, 0x34, 1);
            let map = m.r32(0x6e51ac);
            vcall1(m, host, map, 0xcc, 0);
            let map = m.r32(0x6e51ac);
            vcall1(m, host, map, 0x34, 0);
        }
        let gui = m.r32(0x6cbdd8);
        host.call(m, 0x44f490, gui, &[a1, a2]);
        let e = m.r32(0x6c7ccc);
        m.w32(f.arg(0), e);
        f.seh_state(m, 0);
        hotseat_notice(m, host, &f, player, f.arg(0), f.at(0x4c));
        f.seh_state(m, 0xffff_ffff);
        host.call(m, CSTR_DTOR, f.arg(0), &[]);
    } else {
        let map = m.r32(0x6e51ac);
        if host.call(m, 0x4b2200, map, &[]) & 0xff == 0 {
            let gui = m.r32(0x6cbdd8);
            vcall1(m, host, gui, 0xcc, 0);
            let gui = m.r32(0x6cbdd8);
            vcall1(m, host, gui, 0x34, 0);
            let map = m.r32(0x6e51ac);
            vcall1(m, host, map, 0xcc, 1);
            let map = m.r32(0x6e51ac);
            vcall1(m, host, map, 0x34, 1);
        }
        let map = m.r32(0x6e51ac);
        host.call(m, 0x58a4f0, map, &[a2, 0, 0, 1, 1]);
        let map = m.r32(0x6e51ac);
        host.call(m, 0x593910, map, &[1]);
        host.call(m, 0x495be0, 0x6ce6d8, &[]);
    }
    let s2 = f.at(0x50);
    let e = m.r32(0x6c7ccc);
    m.w32(s2, e);
    f.seh_state(m, 1);
    hotseat_notice(m, host, &f, player, s2, f.at(0x2c));
    let w = u32::from(m.r16(0x6dfc14));
    host.call(m, 0x4d7d50, MSGS, &[w]);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, CSTR_DTOR, s2, &[]);
    f.seh_leave(m);
    r
}

fn copy_in(m: &mut dyn Mem, dst: u32, src: u32, n: u32) -> u32 {
    for k in 0..n {
        let v = m.r8(src + k);
        m.w8(dst + k, v);
    }
    src + n
}

/// 0x0051B520 `town_deserialize` (ciudad en ECX, stdcall de 2: datos, versión; devuelve el puntero tras lo leído). Marco SEH
/// (`0x6631CD`) con un objeto de lista local en `esp-0x20` (`0x50FCA0`, destino de las entradas sin memoria) y el recuento de la lista
/// leído en `esp-0x24` (también contador de los bucles). Encadena los lectores de sistemas, 21 bloques de `+0x840`, campos fijos
/// `+0x990..+0x9EA` (`+0x9B8..+0x9C0` desde la versión 0x6C, `+0x9C4` desde 0x6F), libera la lista `+0x9B4` y la reconstruye (cada nodo
/// de 0x14 bytes se guarda en el hueco del argumento 2).
pub fn town_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, t: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x6631cd);
    let (loc, cnt) = (f.at(0x20), f.at(0x24));
    host.call(m, 0x50fca0, loc, &[]);
    let (p0, ver) = (m.r32(f.arg(0)), m.r32(f.arg(1)));
    let mut p = p0.wrapping_add(1);
    for a in [0x527ac0, 0x525e30, 0x51ec10, 0x52a2b0, 0x528870] {
        p = host.call(m, a, t, &[p, ver]).wrapping_add(1);
    }
    for i in 0..0x15u32 {
        m.w32(cnt, i);
        p = host.call(m, 0x510070, t + (i + 0x84) * 16, &[p]);
    }
    m.w32(cnt, 0x15);
    p = copy_in(m, t + 0x9a4, p, 0x10);
    p = copy_in(m, t + 0x990, p, 4);
    p = copy_in(m, t + 0x994, p, 2);
    p = copy_in(m, t + 0x996, p, 2);
    for i in 0..4u32 {
        m.w32(cnt, i);
        p = copy_in(m, t + 0x998 + i, p, 1);
        p = copy_in(m, t + 0x99c + i, p, 1);
    }
    m.w32(cnt, 4);
    p = copy_in(m, t + 0x9a0, p, 2);
    p = copy_in(m, t + 0x9a2, p, 2);
    if ver < 0x6c {
        p = copy_in(m, t + 0x9c0, p, 4);
        m.w32(t + 0x9b8, 0);
        m.w8(t + 0x9bd, 0);
        m.w8(t + 0x9bc, 0);
        m.w32(t + 0x9c0, 0);
    } else {
        p = copy_in(m, t + 0x9b8, p, 4);
        p = copy_in(m, t + 0x9bc, p, 1);
        p = copy_in(m, t + 0x9bd, p, 1);
        p = copy_in(m, t + 0x9c0, p, 4);
    }
    if ver < 0x6f {
        m.w8(t + 0x9c4, 0);
    } else {
        p = copy_in(m, t + 0x9c4, p, 1);
    }
    p = copy_in(m, t + 0x9c6, p, 0x12);
    p = copy_in(m, t + 0x9d8, p, 0x12);
    p = copy_in(m, cnt, p, 2);
    loop {
        let n = m.r32(t + 0x9b4);
        if n == 0 {
            break;
        }
        let nx = m.r32(n);
        m.w32(t + 0x9b4, nx);
        host.call(m, 0x64f7e2, 0, &[n]);
    }
    m.w32(t + 0x9b4, 0);
    while m.r16(cnt) as i16 > 0 {
        let c = m.r32(cnt).wrapping_sub(1);
        m.w32(cnt, c);
        let a = host.call(m, 0x64f7b9, 0, &[0x14]);
        m.w32(f.arg(1), a);
        f.seh_state(m, 0);
        let node = if a != 0 { host.call(m, 0x50fca0, a, &[]) } else { 0 };
        f.seh_state(m, 0xffff_ffff);
        if node != 0 {
            p = host.call(m, 0x50fd40, node, &[p, ver]);
            let hd = m.r32(t + 0x9b4);
            m.w32(node, hd);
            m.w32(t + 0x9b4, node);
        } else {
            p = host.call(m, 0x50fd40, loc, &[p, ver]);
        }
    }
    f.seh_leave(m);
    p
}

/// 0x00504230 `ships_deserialize` (gestor de barcos en ECX, stdcall de 1: datos; devuelve el puntero tras lo leído). Marco SEH
/// (`0x662FE6`); la versión se lee en el local `esp-0x14`, el temporal de `new` en `esp-0x10` y el hueco del argumento sirve de
/// contador/desplazamiento. Lee 17 recuentos, reconstruye las tablas `+0` (16 bytes, `0x4FDEB0`), barcos `+4` (0x180, `0x517990`,
/// aviso `0x45BF80`) y convoyes `+8` (0x3C, `0x5014E0`), el bloque `+0x20` (0xC0), los cinco objetos `+0xC..` (0x18, `0x5138B0`: por
/// recuento antes de la versión 0x63, por bandera después), recuenta los barcos de cada ciudad (`+0x47C`) y suelta escoltas inválidas.
pub fn ships_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, t: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x662fe6);
    let (verl, tmp, ctr) = (f.at(0x14), f.at(0x10), f.arg(0));
    let mut p = m.r32(ctr);
    m.w32(verl, 0);
    p = copy_in(m, verl, p, 2);
    for off in [0xe0, 0xe2, 0xe4, 0xe8, 0xea, 0xec, 0xee, 0xf0, 0xf2, 0xf4, 0xf6, 0xe6, 0xfa, 0xfc, 0xfe, 0xf8] {
        p = copy_in(m, t + off, p, 2);
    }
    let ver = m.r32(verl);
    let free_old = |m: &mut dyn Mem, host: &mut dyn OpsHost, a: u32| {
        let o = m.r32(a);
        if o != 0 {
            host.call(m, 0x64f7e2, 0, &[o]);
        }
    };
    free_old(m, host, t);
    let n = u32::from(m.r16(t + 0xf2));
    if n > 0 {
        let a = host.call(m, 0x64f7b9, 0, &[n << 4]);
        m.w32(t, a);
        m.w32(ctr, 0);
        let mut i = 0;
        while i < u32::from(m.r16(t + 0xf2)) {
            let off = m.r32(ctr);
            let base = m.r32(t);
            p = host.call(m, 0x4fdeb0, base.wrapping_add(off), &[p, ver & 0xffff]);
            m.w32(ctr, off.wrapping_add(0x10));
            i += 1;
        }
    } else {
        m.w32(t, 0);
    }
    free_old(m, host, t + 4);
    let n = u32::from(m.r16(t + 0xf4));
    if n > 0 {
        let a = host.call(m, 0x64f7b9, 0, &[n * 0x180]);
        m.w32(t + 4, a);
        let mut i = 0;
        while i < u32::from(m.r16(t + 0xf4)) {
            let base = m.r32(t + 4);
            p = host.call(m, 0x517990, base.wrapping_add(i * 0x180), &[p, ver]);
            i += 1;
        }
        let g = m.r32(0x6cbdd8);
        if g != 0 {
            let n = u32::from(m.r16(t + 0xf4));
            host.call(m, 0x45bf80, g, &[n]);
        }
    } else {
        m.w32(t + 4, 0);
    }
    free_old(m, host, t + 8);
    let n = u32::from(m.r16(t + 0xf6));
    if n > 0 {
        let a = host.call(m, 0x64f7b9, 0, &[n * 0x3c]);
        m.w32(t + 8, a);
        let mut i = 0;
        while i < u32::from(m.r16(t + 0xf6)) {
            let base = m.r32(t + 8);
            p = host.call(m, 0x5014e0, base.wrapping_add(i * 0x3c), &[p, ver & 0xffff]);
            i += 1;
        }
    } else {
        m.w32(t + 8, 0);
    }
    p = copy_in(m, t + 0x20, p, 0xc0);
    for i in 0..0x10 {
        m.w16(t + 0x2a + i * 0xc, 0);
    }
    let old = ver & 0xffff < 0x63;
    for i in 0..5u32 {
        m.w32(ctr, if old { i } else { 5 - i });
        let slot = t + 0xc + i * 4;
        let o = m.r32(slot);
        if o != 0 {
            host.call(m, 0x513760, o, &[]);
            host.call(m, 0x64f7e2, 0, &[o]);
            m.w32(slot, 0);
        }
        let make = if old {
            i < u32::from(m.r16(t + 0xf8))
        } else {
            let b = m.r8(p);
            p += 1;
            b != 0
        };
        if make {
            let a = host.call(m, 0x64f7b9, 0, &[0x18]);
            m.w32(tmp, a);
            f.seh_state(m, if old { 0 } else { 1 });
            let obj = if a != 0 { host.call(m, 0x513720, a, &[]) } else { 0 };
            f.seh_state(m, 0xffff_ffff);
            m.w32(slot, obj);
            p = host.call(m, 0x5138b0, obj, &[p, ver & 0xffff]);
        }
    }
    m.w32(ctr, if old { 5 } else { 0 });
    let mut town = u32::from(m.r16(TOWN_COUNT));
    while town > 0 {
        town -= 1;
        let tr = host.call(m, 0x5303c0, WORLD, &[town]);
        let cntp = tr + 0x47c;
        m.w32(cntp, 0);
        let tr = host.call(m, 0x5303c0, WORLD, &[town]);
        let mut s = u32::from(m.r16(tr + 0xe));
        while s < u32::from(m.r16(t + 0xf4)) {
            let c = m.r32(cntp);
            m.w32(cntp, c.wrapping_add(1));
            let base = m.r32(t + 4);
            s = u32::from(m.r16(base.wrapping_add(s * 0x180 + 4)));
        }
    }
    m.w32(ctr, 0);
    let mut i = 0u32;
    while i < u32::from(m.r16(t + 0xf4)) {
        let base = m.r32(t + 4);
        let sp = base.wrapping_add(i * 0x180);
        if m.r16(sp + 0x134) == 0xf {
            for j in 0..2 {
                let v = m.r16(sp + 0x50 + j * 2);
                if v < m.r16(t + 0xf4) {
                    let ty = m.r16(base.wrapping_add(u32::from(v) * 0x180 + 0x134));
                    if ty != 0xf && ty != 0x12 {
                        m.w16(sp + 0x50 + j * 2, 0xffff);
                    }
                }
            }
        }
        i += 1;
        m.w32(ctr, i);
    }
    f.seh_leave(m);
    p
}

/// Vector de objetos con recuento delante (`new T[n]` de MSVC): libera el anterior con `eh vector destructor iterator` (`0x639DA8`).
fn vec_free(m: &mut dyn Mem, host: &mut dyn OpsHost, slot: u32, size: u32, dtor: u32) {
    let p = m.r32(slot);
    if p != 0 {
        let n = m.r32(p - 4);
        host.call(m, 0x639da8, 0, &[p, size, n, dtor]);
        host.call(m, 0x64f7e2, 0, &[p - 4]);
        m.w32(slot, 0);
    }
}

/// `new T[n]` con `eh vector constructor iterator` (`0x639E9C`); devuelve el primer elemento o 0.
fn vec_new(m: &mut dyn Mem, host: &mut dyn OpsHost, f: &Frame, n: u32, size: u32, ctor: u32, dtor: u32, state: u32) -> u32 {
    let a = host.call(m, 0x64f7b9, 0, &[n.wrapping_mul(size).wrapping_add(4)]);
    m.w32(f.arg(0), a);
    f.seh_state(m, state);
    let r = if a != 0 {
        m.w32(a, n);
        host.call(m, 0x639e9c, 0, &[a + 4, size, n, ctor, dtor]);
        a + 4
    } else {
        0
    };
    f.seh_state(m, 0xffff_ffff);
    r
}

/// 0x0052FB60 `game_world_deserialize` (mundo en ECX, stdcall de 2: datos, versión; devuelve el puntero tras lo leído). Marco SEH
/// (`0x663432`). Lee la cabecera (`+0..+0x8F`, siete bloques de 12 bytes en `+0x90`), rehace con `new[]` y los iteradores de vector
/// de MSVC las ciudades (`+0x68`, 0x9F8, `0x51B520`, índice en `+0x2C0`), las oficinas globales (`[0x6DF270]`, 0x1024, `0x633730`),
/// la matriz de distancias (`+0x6C`, n²), los bloques `+0x70` (0x14), `+0x74` (0x44C, el new en `esp-0x3C`/`esp-0x40`), `+0x78` (0x650),
/// el bloque `+0xE8`, 15/16 rutas `+0xB10` (0x2C; desde la versión 0x75 la decimosexta se lee en un objeto local `esp-0x38` con lista
/// centinela en `esp-0x14`, hasta 0x81) y el objeto opcional `+0xE4`. Los huecos de argumento guardan los temporales de `new`.
pub fn game_world_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, w: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x663432);
    let ver = m.r32(f.arg(1));
    let mut p = m.r32(f.arg(0));
    for (off, n) in [(0, 1), (1, 1), (2, 2), (4, 2), (0x10, 4), (6, 2), (8, 2), (0xa, 2)] {
        p = copy_in(m, w + off, p, n);
    }
    if ver < 0x5e {
        p += 4;
    }
    let b = m.r8(p);
    m.w8(w + 0x89, u8::from(b != 0));
    copy_in(m, w + 0x8c, p + 1, 1);
    let b = m.r8(p + 2);
    m.w8(w + 0x8a, u8::from(b == 0x7f));
    copy_in(m, w + 0x8b, p + 3, 1);
    p += 4;
    for (off, n) in [(0xc, 4), (0x14, 4), (0x7c, 2), (0x7e, 2), (0x18, 0x28), (0x40, 0x28)] {
        p = copy_in(m, w + off, p, n);
    }
    copy_in(m, w + 0x8d, p, 1);
    copy_in(m, w + 0x8e, p + 1, 1);
    p += 2;
    for k in 0..7 {
        p = copy_in(m, w + 0x90 + k * 12, p, 12);
    }
    vec_free(m, host, w + 0x68, 0x9f8, 0x51b290);
    vec_free(m, host, 0x6df270, 0x1024, 0x633520);
    let n = m.r32(w + 0x10);
    if n > 0 {
        let v = vec_new(m, host, &f, n, 0x9f8, 0x51b100, 0x51b290, 0);
        m.w32(w + 0x68, v);
        let mut i = 0;
        while i < m.r32(w + 0x10) {
            let base = m.r32(w + 0x68);
            p = host.call(m, 0x51b520, base.wrapping_add(i * 0x9f8), &[p, ver]);
            let base = m.r32(w + 0x68);
            m.w8(base.wrapping_add(i * 0x9f8 + 0x2c0), i as u8);
            i += 1;
        }
    }
    let mut ui = m.r32(0x6cbdc8);
    if ui == 0 {
        let a = host.call(m, 0x64f7b9, 0, &[0x2c]);
        m.w32(f.arg(0), a);
        f.seh_state(m, 1);
        ui = if a != 0 { host.call(m, 0x444ba0, a, &[]) } else { 0 };
        f.seh_state(m, 0xffff_ffff);
        m.w32(0x6cbdc8, ui);
    }
    host.call(m, 0x4482c0, ui, &[]);
    let n = m.r32(w + 0x10);
    let nn = n.wrapping_mul(n);
    let o = m.r32(w + 0x6c);
    if o != 0 {
        host.call(m, 0x64f7e2, 0, &[o]);
    }
    let a = host.call(m, 0x64f7b9, 0, &[nn]);
    m.w32(w + 0x6c, a);
    p = copy_in(m, a, p, nn);
    if ver >= 0x5a {
        p += 4;
    }
    let n = m.r32(w + 0x10);
    if n > 0 {
        let v = vec_new(m, host, &f, n, 0x1024, 0x6334b0, 0x633520, 2);
        m.w32(0x6df270, v);
        let mut i = 0u32;
        while i < m.r32(w + 0x10) {
            let base = m.r32(0x6df270);
            m.w8(base.wrapping_add(i * 0x1024 + 4), i as u8);
            let base = m.r32(0x6df270);
            p = host.call(m, 0x633730, base.wrapping_add(i * 0x1024), &[p, ver]);
            i += 1;
        }
    }
    let o = m.r32(w + 0x70);
    if o != 0 {
        host.call(m, 0x64f7e2, 0, &[o]);
        m.w32(w + 0x70, 0);
    }
    let n = u32::from(m.r16(w + 6));
    if n > 0 {
        let a = host.call(m, 0x64f7b9, 0, &[n * 0x14]);
        m.w32(w + 0x70, a);
        let mut i = 0;
        while i < u32::from(m.r16(w + 6)) {
            let base = m.r32(w + 0x70);
            p = host.call(m, 0x4d32f0, base.wrapping_add(i * 0x14), &[p, ver]);
            i += 1;
        }
    }
    let o = m.r32(w + 0x74);
    if o != 0 {
        host.call(m, 0x64f7e2, 0, &[o]);
        m.w32(w + 0x74, 0);
    }
    let n = u32::from(m.r16(w + 8));
    if n > 0 {
        let a = host.call(m, 0x64f7b9, 0, &[n * 0x44c]);
        m.w32(f.at(0x3c), a);
        f.seh_state(m, 3);
        if a != 0 {
            m.w32(f.at(0x40), a);
            for k in 0..n {
                m.w32(f.arg(0), n - k);
                host.call(m, 0x4febb0, a + k * 0x44c, &[]);
            }
            m.w32(f.arg(0), 0);
        }
        f.seh_state(m, 0xffff_ffff);
        m.w32(w + 0x74, a);
        let mut i = 0;
        while i < u32::from(m.r16(w + 8)) {
            let base = m.r32(w + 0x74);
            p = host.call(m, 0x4feff0, base.wrapping_add(i * 0x44c), &[p, ver]);
            i += 1;
        }
    }
    vec_free(m, host, w + 0x78, 0x650, 0x4f3fa0);
    let n = u32::from(m.r16(w + 0xa));
    if n > 0 {
        let v = vec_new(m, host, &f, n, 0x650, 0x4f3eb0, 0x4f3fa0, 4);
        let c = m.r16(w + 0xa);
        m.w32(w + 0x78, v);
        m.w16(w + 0x80, c);
        let mut i = 0;
        while i < u32::from(m.r16(w + 0xa)) {
            let base = m.r32(w + 0x78);
            p = host.call(m, 0x4f4100, base.wrapping_add(i * 0x650), &[p, ver]);
            i += 1;
        }
    }
    p = host.call(m, 0x5351d0, w + 0xe8, &[p]);
    let routes = if ver < 0x75 { 0x10 } else { 0xf };
    for k in 0..routes {
        p = host.call(m, 0x52bbe0, w + 0xb10 + k * 0x2c, &[p, ver]);
    }
    if (0x75..0x81).contains(&ver) {
        let (lst, cnt) = (f.at(0x14), f.at(0x10));
        m.w8(f.at(0x18), ver as u8);
        let a = host.call(m, 0x64f7b9, 0, &[0x18]);
        m.w32(a, a);
        m.w32(a + 4, a);
        m.w32(lst, a);
        m.w32(cnt, 0);
        f.seh_state(m, 5);
        for _ in 0..3 {
            p = host.call(m, 0x52bbe0, f.at(0x38), &[p, ver]);
        }
        f.seh_state(m, 0xffff_ffff);
        let head = m.r32(lst);
        let mut node = m.r32(head);
        while node != head {
            let nx = m.r32(node);
            let (pv, nv) = (m.r32(node + 4), m.r32(node));
            m.w32(pv, nv);
            let (nv, pv) = (m.r32(node), m.r32(node + 4));
            m.w32(nv + 4, pv);
            host.call(m, 0x64f7e2, 0, &[node]);
            let c = m.r32(cnt);
            m.w32(cnt, c.wrapping_sub(1));
            node = nx;
        }
        let head = m.r32(lst);
        host.call(m, 0x64f7e2, 0, &[head]);
        m.w32(lst, 0);
        m.w32(cnt, 0);
    }
    let o = m.r32(w + 0xe4);
    if o != 0 {
        host.call(m, 0x533e80, o, &[]);
    }
    m.w32(w + 0xe4, 0);
    let b = m.r8(p);
    p += 1;
    if b != 0 {
        let a = host.call(m, 0x64f7b9, 0, &[0x14]);
        m.w32(f.arg(1), a);
        f.seh_state(m, 6);
        let obj = if a != 0 { host.call(m, 0x533890, a, &[]) } else { 0 };
        f.seh_state(m, 0xffff_ffff);
        m.w32(w + 0xe4, obj);
        p = host.call(m, 0x533da0, obj, &[p]);
    }
    if ver >= 0x6d {
        let b = m.r8(p);
        m.w8(w + 0x8f, b);
        p += 1;
    }
    f.seh_leave(m);
    p
}

/// Añade la cadena C de `src` al búfer del mensaje de 0x528E60 (puntero en `buf`, longitud en `len`).
fn mayor_append(m: &mut dyn Mem, buf: u32, len: u32, mut src: u32) {
    loop {
        let c = m.r8(src);
        if c == 0 {
            break;
        }
        let (b, l) = (m.r32(buf), m.r32(len));
        m.w8(b.wrapping_add(l), c);
        m.w32(len, l.wrapping_add(1));
        src = src.wrapping_add(1);
    }
}

/// 0x00528E60 `run_mayor_election` (ciudad en ECX). Marco SEH (`0x6632E3`) con un mensaje local en `esp-0x9C` (`0x51A850`/`0x51A880`;
/// tipo en `+4`, búfer `+8`, longitud `+0xC`, máximo `+0x10`), votos `esp-0x88`, pesos `esp-0x78`, candidatos `esp-0x68` (`0x528CF0`)
/// y búfer de `sprintf` en `esp-0x4C`. Pesos de comerciantes (`0x4F9580`) o de la tabla `0x6737B4`; sin comerciantes, ruido por fecha.
/// Reparte 120 votos (3 por cada concejal propio en `+0x6DC`), elige al máximo, lleva la racha `+0x6F3` (evento 0x38 al empezar), cambia
/// el alcalde `+0x6F1` (`0x4F75F0`), reinicia los rangos `+0x6E4` de las ciudades, avisa al jugador humano (`0x548CA0`, y con dos CString
/// por valor a `0x469AF0` si es el local) y, si no hay red (`[0x6DE52A]`), envía el acta 0x47 y el resumen 0x55 (`0x4D6530`).
pub fn run_mayor_election(m: &mut dyn Mem, host: &mut dyn OpsHost, t: u32, esp: u32) -> u32 {
    const TABLE: [u8; 4] = [59, 65, 70, 80];
    let f = Frame::new(esp);
    f.seh_enter(m, 0x6632e3);
    let (msg, votes, wts, cand, sbuf) = (f.at(0x9c), f.at(0x88), f.at(0x78), f.at(0x68), f.at(0x4c));
    let (buf, len, maxs) = (msg + 8, msg + 0xc, msg + 0x10);
    host.call(m, 0x51a850, msg, &[]);
    f.seh_state(m, 0);
    host.call(m, 0x528cf0, t, &[cand]);
    let count = |m: &mut dyn Mem| u32::from(m.r16(TOWN_COUNT));
    let mut any = false;
    let mut total = 0u32;
    for i in 0..4 {
        let a = u32::from(m.r8(cand + i));
        let w = if a < count(m) {
            let p = host.call(m, 0x5303c0, WORLD, &[a]);
            let tw = u32::from(m.r8(t + 0x2c0));
            any = true;
            host.call(m, 0x4f9580, p, &[tw]) as u16 as i16 as i32 as u32
        } else {
            u32::from(TABLE[(!a & 3) as usize])
        };
        m.w32(wts + i * 4, w);
        total = total.wrapping_add(w);
    }
    m.w32(maxs, total);
    if !any {
        let tick = m.r32(TICK);
        for c in 0..4 {
            let s = m.r32(wts + c * 4);
            let d = if s == 0x50 { 0x1f } else { (((tick >> 8).wrapping_mul(c + 7) as i32) % 0x43) as u32 & 0x1f };
            m.w32(wts + c * 4, s.wrapping_add(d));
            total = total.wrapping_add(d);
            m.w32(maxs, total);
        }
    }
    for i in 0..4 {
        m.w32(votes + i * 4, 0);
    }
    let mut left = 0x78u32;
    for k in 0..4 {
        let dl = m.r8(t + 0x6dc + k);
        if u32::from(dl) < count(m) {
            if let Some(j) = (0..4).find(|&j| m.r8(cand + j) == dl) {
                let v = m.r32(votes + j * 4);
                m.w8(t + 0x6dc + k, 0xff);
                m.w32(votes + j * 4, v.wrapping_add(3));
                left = left.wrapping_sub(3);
            }
        }
    }
    let total = m.r32(maxs);
    for j in 0..4 {
        let w = m.r32(wts + j * 4) as i32;
        let v = m.r32(votes + j * 4);
        m.w32(votes + j * 4, v.wrapping_add(w.wrapping_mul(left as i32).wrapping_div(total as i32) as u32));
    }
    let mut best = m.r32(votes);
    let mut win = m.r8(cand);
    m.w32(maxs, best);
    for j in 1..4 {
        let v = m.r32(votes + j * 4);
        if (best as i32) < (v as i32) {
            win = m.r8(cand + j);
            best = v;
        }
    }
    m.w32(maxs, best);
    if win == m.r8(t + 0x6f1) {
        let d = m.r8(t + 0x6f3).wrapping_add(1);
        m.w8(t + 0x6f3, d.min(0xfa));
        if m.r8(t + 0x6f3) == 1 && u32::from(win) < count(m) {
            let mut sum = 0u32;
            for j in 0..4 {
                sum = sum.wrapping_add(m.r32(votes + j * 4));
            }
            let q = best.wrapping_mul(0x5d00) / sum.wrapping_add(1);
            let at = m.r32(TICK).wrapping_sub(q).wrapping_add(0xba00);
            let ev = host.call(m, 0x4d8cf0, SCHED, &[at]);
            m.w16(ev + 6, 0x38);
            let tw = m.r8(t + 0x2c0);
            m.w8(ev + 8, tw);
            let my = m.r8(t + 0x6f1);
            m.w8(ev + 9, my);
        }
    } else {
        m.w8(t + 0x6f3, 0);
    }
    let old = u32::from(m.r8(t + 0x6f1));
    let p = if old < count(m) { host.call(m, 0x5303c0, WORLD, &[old]) } else { 0 };
    m.w8(t + 0x6f1, win);
    if p != 0 {
        let tw = u32::from(m.r8(t + 0x2c0));
        host.call(m, 0x4f75f0, p, &[tw]);
    }
    let nt = m.r32(0x6de4b0);
    for i in 0..nt {
        let ta = m.r32(0x6de508).wrapping_add(i * 0x9f8);
        let dl = m.r8(ta + 0x6f1);
        m.w8(ta + 0x6e4, 5);
        m.w8(ta + 0x6e5, 5);
        m.w8(ta + 0x6e6, 4);
        m.w8(ta + 0x6e7, 4);
        if u32::from(dl) >= count(m) {
            let k = u32::from(!dl & 3);
            let v = if m.r8(0x6de52e) == m.r8(ta + 0x2c0) && u32::from(m.r8(0x6de52d)) == k { 7 } else { 6 };
            m.w8(ta + 0x6e4 + k, v);
        }
    }
    let my = u32::from(m.r8(t + 0x6f1));
    if my < count(m) {
        let p = host.call(m, 0x5303c0, WORLD, &[my]);
        let tw = u32::from(m.r8(t + 0x2c0));
        host.call(m, 0x4f75f0, p, &[tw]);
        if m.r16(p + 8) == 0 {
            let (tw, my) = (u32::from(m.r8(t + 0x2c0)), u32::from(m.r8(t + 0x6f1)));
            host.call(m, 0x548ca0, 0x6df2f0, &[my, 1, 0, tw, 0x18, tw]);
            if u32::from(m.r8(t + 0x6f1)) == m.r32(0x6dfc14) {
                let ui = m.r32(0x6cc7e8);
                host.call(m, 0x469380, ui, &[0]);
                let ui = m.r32(0x6cc7e8);
                let x1 = f.push(m, 0xb0, ui);
                m.w32(f.at(0x50), x1);
                host.call(m, 0x64f2c1, x1, &[0x6de3b8]);
                let x2 = f.push(m, 0xb4, x1);
                m.w8(f.at(4), 1);
                m.w32(f.at(0x54), x2);
                host.call(m, 0x64f2c1, x2, &[0x6de3b9]);
                let (tw, my) = (u32::from(m.r8(t + 0x2c0)), u32::from(m.r8(t + 0x6f1)));
                let ui = m.r32(0x6cc7e8);
                m.w8(f.at(4), 0);
                let (a2, a1) = (m.r32(x2), m.r32(x1));
                host.call(m, 0x469af0, ui, &[5, my, tw, a2, a1]);
            }
        }
    }
    if m.r8(0x6de52a) == 0 {
        let tw = m.r8(t + 0x2c0);
        m.w8(msg + 4, 0x47);
        m.w8(msg + 5, tw);
        m.w32(len, 0);
        let b = host.call(m, 0x64f7b9, 0, &[0x800]);
        m.w32(buf, b);
        if b != 0 {
            let s = m.r32(0x6b72dc);
            mayor_append(m, buf, len, s);
            let s = m.r32(0x6dda00 + u32::from(tw) * 4);
            mayor_append(m, buf, len, s);
            let s = m.r32(0x6b72e0);
            mayor_append(m, buf, len, s);
            for j in 0..4 {
                let a = u32::from(m.r8(cand + j));
                let (name, x) = if a < count(m) {
                    let p = host.call(m, 0x5303c0, WORLD, &[a]);
                    (m.r32(p + 0xe8), u32::from(m.r8(p + 0x1d)))
                } else {
                    let k = !a & 3;
                    let v = u32::from(m.r8(t + 0x6d4 + k));
                    let n = host.call(m, 0x512a60, 0x6ddaa0, &[v]);
                    (n, u32::from(m.r8(t + 0x6d8 + k)))
                };
                let r = host.call(m, 0x512ae0, 0x6ddaa0, &[x]);
                let (fm, v) = (m.r32(0x6b72e4), m.r32(votes + j * 4));
                host.call(m, 0x64d410, 0, &[sbuf, fm, name, r, v]);
                mayor_append(m, buf, len, sbuf);
            }
            let s = m.r32(0x6b72e8);
            mayor_append(m, buf, len, s);
            let my = u32::from(m.r8(t + 0x6f1));
            let (s, x) = if my >= count(m) {
                let k = !my & 3;
                let v = u32::from(m.r8(t + 0x6d4 + k));
                (host.call(m, 0x512a60, 0x6ddaa0, &[v]), u32::from(m.r8(t + 0x6d8 + k)))
            } else {
                let p = host.call(m, 0x5303c0, WORLD, &[my]);
                (host.call(m, 0x4f6420, p, &[]), u32::from(m.r8(p + 0x1d)))
            };
            mayor_append(m, buf, len, s);
            let (b, l) = (m.r32(buf), m.r32(len));
            m.w8(b.wrapping_add(l), 0x20);
            m.w32(len, l.wrapping_add(1));
            let s = host.call(m, 0x512ae0, 0x6ddaa0, &[x]);
            mayor_append(m, buf, len, s);
            let s = m.r32(0x6b72ec);
            mayor_append(m, buf, len, s);
            let (b, l) = (m.r32(buf), m.r32(len));
            m.w8(b.wrapping_add(l), 0);
            m.w32(len, l.wrapping_add(1));
            let mut i = 0;
            while i < count(m) {
                let p = host.call(m, 0x5303c0, WORLD, &[i]);
                if m.r16(p + 8) == 0 && m.r8(p + 0x19) == m.r8(t + 0x2c0) {
                    let a = peek(m, &[i, msg], msg, 5);
                    host.call(m, 0x4d6530, MSGS, &a);
                }
                i += 1;
            }
            let b = m.r32(buf);
            if b != 0 {
                host.call(m, 0x64f7e2, 0, &[b]);
                m.w32(buf, 0);
            }
            let my = u32::from(m.r8(t + 0x6f1));
            let (w1, w2) = if my >= count(m) {
                let k = !my & 3;
                (m.r8(t + 0x6d4 + k), m.r8(t + 0x6d8 + k))
            } else {
                let p = host.call(m, 0x5303c0, WORLD, &[my]);
                (m.r8(p + 0x1c), m.r8(p + 0x1d))
            };
            m.w16(msg + 8, u16::from(w1));
            m.w16(msg + 0xa, u16::from(w2));
            let tw = m.r8(t + 0x2c0);
            m.w8(msg + 5, tw);
            let mx = m.r16(maxs);
            m.w16(msg + 0xc, mx);
            let mut sum = 0u32;
            for j in 0..4 {
                sum = sum.wrapping_add(m.r32(votes + j * 4));
            }
            m.w8(msg + 4, 0x55);
            m.w16(msg + 0xe, sum as u16);
            let a = peek(m, &[0xffff_fffe, msg], msg, 5);
            host.call(m, 0x4d6530, MSGS, &a);
        }
    }
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x51a880, msg, &[]);
    f.seh_leave(m);
    r
}
