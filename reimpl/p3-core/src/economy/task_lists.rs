//! Listas que construye el planificador para la interfaz (C08), sobre la imagen de memoria del original: ofertas de reparto de beneficios
//! (`0x004D9320`), lista de reparto del jugador (`0x004D9620`) y deudas de/hacia un comerciante (`0x004D9100`, `0x004D9210`).
//! El montón (`0x0064F7B9`/`0x0064F7E2`) lo aporta el anfitrión mediante [`Heap`].
use crate::economy::sched_queries::{merchant_addr, G_MERCHANTS, G_PLAYER, G_TICK};
use crate::flatmem::Mem;

/// Montón del proceso: `alloc` devuelve memoria a cero (0 si no hay), `free` la libera.
pub trait Heap {
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    fn free(&mut self, addr: u32);
}

/// Divisor del año del reparto (`0x00672828` = 1987,0 en el ejecutable inglés), operando x87 con precisión doble.
pub const YEAR_DIVISOR: f64 = 1987.0;

fn mc(m: &mut dyn Mem) -> u32 {
    u32::from(m.r16(G_MERCHANTS))
}
fn head(m: &mut dyn Mem, s: u32) -> u32 {
    u32::from(m.r16(s + 8))
}
fn count(m: &mut dyn Mem, s: u32) -> u32 {
    u32::from(m.r16(s + 0xc))
}
fn rec(m: &mut dyn Mem, s: u32, i: u32) -> u32 {
    m.r32(s).wrapping_add(i.wrapping_mul(0x18))
}

/// `scheduled_tasks_build_profit_share_offer_list` (`0x004D9320`): lista de 12 bytes por oferta de reparto (tarea `0x18` libre); `out` recibe
/// el número de ofertas. Devuelve la dirección de la lista o 0.
pub fn build_profit_share_offer_list(m: &mut dyn Mem, h: &mut dyn Heap, s: u32, out: u32) -> u32 {
    let n_mer = mc(m);
    let block = h.alloc(m, n_mer << 2);
    if block == 0 {
        return 0;
    }
    let player = m.r32(G_PLAYER);
    m.w32(block.wrapping_add(player.wrapping_mul(4)), 100);
    m.w32(out, 0);
    let n = count(m, s);
    let mut idx = head(m, s);
    let mut seen_own = false;
    if idx < n {
        loop {
            let r = rec(m, s, idx);
            if m.r16(r + 6) == 0x18 {
                let owner = u32::from(m.r8(r + 0x10));
                if m.r8(r + 0x13) != 0 && m.r8(r + 0x11) == 0xff && owner < n_mer {
                    let c = m.r32(out).wrapping_add(1);
                    m.w32(out, c);
                    if owner == player {
                        if seen_own {
                            m.w8(r + 0x10, 0xff);
                            m.w8(r + 0x12, 0);
                        } else {
                            seen_own = true;
                        }
                    }
                }
                let owner = u32::from(m.r8(r + 0x10));
                if owner < n_mer {
                    let amount = u32::from(m.r8(r + 0x12));
                    if owner == player {
                        let a = block.wrapping_add(player.wrapping_mul(4));
                        let v = m.r32(a).wrapping_sub(amount);
                        m.w32(a, v);
                    } else if u32::from(m.r8(r + 0x11)) == player {
                        let a = block.wrapping_add(owner.wrapping_mul(4));
                        let v = m.r32(a).wrapping_add(amount);
                        m.w32(a, v);
                    }
                }
            }
            idx = u32::from(m.r16(r + 4));
            if idx >= n {
                break;
            }
        }
        if (m.r32(out) as i32) > 0 {
            let total = m.r32(out);
            let list = h.alloc(m, total.wrapping_mul(0xc));
            if list == 0 {
                m.w32(out, 0);
                h.free(block);
                return 0;
            }
            let mut idx = head(m, s);
            let mut done = 0u32;
            let mut p = list + 2;
            if idx < n {
                loop {
                    if !seen_own && m.r32(out) <= done {
                        break;
                    }
                    let r = rec(m, s, idx);
                    let owner = u32::from(m.r8(r + 0x10));
                    if m.r16(r + 6) == 0x18 && m.r8(r + 0x13) != 0 && m.r8(r + 0x11) == 0xff && owner < n_mer {
                        m.w8(p, 0xff);
                        m.w8(p + 1, player as u8);
                        let (o, w) = (m.r8(r + 0x10), m.r8(r + 0x13));
                        m.w8(p - 2, o);
                        m.w8(p - 1, w);
                        let balance = m.r32(block.wrapping_add(owner.wrapping_mul(4)));
                        m.w32(p + 6, if balance < 0x65 { balance } else { 0 });
                        if owner < n_mer {
                            let mer = merchant_addr(m, owner);
                            let income = m.r32(mer + 0x46c);
                            if (income as i32) < 0x1f5 {
                                m.w32(p + 2, u32::from(w).wrapping_mul(6));
                            } else {
                                let k = income / 500 + 1;
                                m.w32(p + 2, u32::from(w).wrapping_mul(k).wrapping_mul(6));
                            }
                        } else {
                            m.w32(p + 2, 0x4b0);
                        }
                        done += 1;
                        p += 0xc;
                    }
                    idx = u32::from(m.r16(r + 4));
                    if idx >= n {
                        break;
                    }
                }
            }
            h.free(block);
            return list;
        }
    }
    h.free(block);
    0
}

/// `scheduled_tasks_build_player_profit_share_list` (`0x004D9620`): primera fila = jugador (cuota restante y aportación del año); le siguen las
/// ofertas en que participa. `out` recibe el número de filas (0 si falla la reserva).
pub fn build_player_profit_share_list(m: &mut dyn Mem, h: &mut dyn Heap, s: u32, out: u32) -> u32 {
    let player = m.r32(G_PLAYER);
    m.w32(out, 1);
    let n = count(m, s);
    let mut idx = head(m, s);
    let mut start = 0u32;
    let mut share = 100u32;
    if idx < n {
        loop {
            let r = rec(m, s, idx);
            if m.r16(r + 6) == 0x18 {
                if u32::from(m.r8(r + 0x11)) == player {
                    let c = m.r32(out).wrapping_add(1);
                    m.w32(out, c);
                } else if u32::from(m.r8(r + 0x10)) == player {
                    share = share.wrapping_sub(u32::from(m.r8(r + 0x12)));
                    if m.r8(r + 0x11) == 0xff {
                        start = m.r32(r + 8).wrapping_add(0x700);
                    }
                }
            }
            idx = u32::from(m.r16(r + 4));
            if idx >= n {
                break;
            }
        }
    }
    let rows = m.r32(out);
    let buf = h.alloc(m, rows.wrapping_mul(0xc));
    if buf == 0 {
        m.w32(out, 0);
        return 0;
    }
    m.w8(buf + 3, 0xff);
    m.w8(buf + 2, player as u8);
    m.w8(buf, player as u8);
    m.w32(buf + 8, share);
    let tick = m.r32(G_TICK);
    if share < 0x38 || tick <= start {
        m.w8(buf + 1, 0);
        m.w32(buf + 4, 0);
    } else {
        let mer = merchant_addr(m, player);
        let income = m.r32(mer + 0x46c) as i32;
        // El original guarda el cociente como f32 y enmascara sus BITS con 0xF (no el valor entero): se reproduce el redondeo x87 a doble y luego a f32.
        let quotient = (f64::from((tick >> 10) as f32) / YEAR_DIVISOR) as f32;
        let u3 = ((quotient.to_bits() & 0xf) / 3) * 5;
        let mut b1 = if u3 < 0x1a { u3 as u8 } else { 0x19 };
        m.w8(buf + 1, b1);
        if share.wrapping_sub(0x37) < u32::from(b1) {
            b1 = (share as u8).wrapping_sub(0x37);
            m.w8(buf + 1, b1);
        }
        if income < 0x65 {
            m.w32(buf + 4, u32::from(b1));
        } else {
            let k = income / 100;
            m.w32(buf + 4, u32::from(b1).wrapping_mul(k as u32));
        }
    }
    let mut idx = head(m, s);
    let mut local = 1u32;
    let mut p = buf + 0x10;
    if idx < n {
        loop {
            if m.r32(out) <= local {
                return buf;
            }
            let r = rec(m, s, idx);
            if m.r16(r + 6) == 0x18 && u32::from(m.r8(r + 0x11)) == player {
                m.w8(p - 1, 0xff);
                m.w8(p - 2, player as u8);
                let (o, w, a) = (m.r8(r + 0x10), m.r8(r + 0x13), m.r8(r + 0x12));
                m.w8(p - 4, o);
                m.w8(p - 3, w);
                m.w32(p + 4, u32::from(a));
                if u32::from(o) < mc(m) {
                    let mer = merchant_addr(m, u32::from(o));
                    let income = m.r32(mer + 0x46c);
                    let k = if income < 0x65 { 1 } else { income / 100 };
                    m.w32(p, k.wrapping_add(1).wrapping_mul(u32::from(w)));
                } else {
                    m.w32(p, u32::from(w).wrapping_mul(1000));
                }
                local += 1;
                p += 0xc;
            }
            idx = u32::from(m.r16(r + 4));
            if idx >= n {
                break;
            }
        }
    }
    buf
}

/// Copia las tareas de préstamo (tipo 1) cuyo byte `field` (`0x11` prestatario, `0x10` prestamista) es `merchant`; la copia lleva su índice en `+4`.
fn collect_debts(m: &mut dyn Mem, h: &mut dyn Heap, s: u32, merchant: u32, out: u32, field: u32) -> u32 {
    m.w32(out, 0);
    let n = count(m, s);
    let mut idx = head(m, s);
    if idx >= n {
        return 0;
    }
    loop {
        let r = rec(m, s, idx);
        if m.r16(r + 6) == 1 && u32::from(m.r8(r + field)) == merchant {
            let c = m.r32(out).wrapping_add(1);
            m.w32(out, c);
        }
        idx = u32::from(m.r16(r + 4));
        if idx >= n {
            break;
        }
    }
    if (m.r32(out) as i32) <= 0 {
        return 0;
    }
    let total = m.r32(out);
    let buf = h.alloc(m, total.wrapping_mul(0x18));
    if buf == 0 {
        m.w32(out, 0);
        return 0;
    }
    let mut idx = head(m, s);
    let mut done = 0u32;
    let mut p = buf;
    if idx < n {
        loop {
            if m.r32(out) <= done {
                return buf;
            }
            let r = rec(m, s, idx);
            if m.r16(r + 6) == 1 && u32::from(m.r8(r + field)) == merchant {
                for k in 0..0x18 {
                    let v = m.r8(r + k);
                    m.w8(p + k, v);
                }
                done += 1;
                m.w16(p + 4, idx as u16);
                p += 0x18;
            }
            idx = u32::from(m.r16(r + 4));
            if idx >= n {
                break;
            }
        }
    }
    buf
}

/// `scheduled_tasks_collect_debts_owed_to_merchant` (`0x004D9100`): préstamos en que `merchant` es el prestatario (`+0x11`).
pub fn collect_debts_owed_to_merchant(m: &mut dyn Mem, h: &mut dyn Heap, s: u32, merchant: u32, out: u32) -> u32 {
    collect_debts(m, h, s, merchant, out, 0x11)
}

/// `scheduled_tasks_collect_debts_owed_by_merchant` (`0x004D9210`): préstamos en que `merchant` es el prestamista (`+0x10`).
pub fn collect_debts_owed_by_merchant(m: &mut dyn Mem, h: &mut dyn Heap, s: u32, merchant: u32, out: u32) -> u32 {
    collect_debts(m, h, s, merchant, out, 0x10)
}

const G_TOWNS: u32 = 0x006d_e4b0;
const TOWN_TABLE: u32 = 0x006d_e508;
const SETTINGS_PTR: u32 = 0x006c_c3e8;
const SHIP_TABLE_PTR: u32 = 0x006d_d7a4;
const SHIP_TABLE_LEN: u32 = 0x006d_d894;

/// Si la tarea `r` (subasta, tipo `0x0A`/`0x36`) debe mostrarse: ciudad válida con bandera `0xC0` y estado 3, 2 o 1 (este con `ajustes+0xD == 0`).
fn auction_visible(m: &mut dyn Mem, r: u32) -> bool {
    let k = m.r16(r + 6);
    if k != 10 && k != 0x36 {
        return false;
    }
    let town = u32::from(m.r8(r + 0x15));
    if town >= m.r32(G_TOWNS) {
        return false;
    }
    let base = m.r32(TOWN_TABLE).wrapping_add(town.wrapping_mul(0x9f8));
    if m.r32(base.wrapping_add(0x2c8)) & 0xc0 == 0 {
        return false;
    }
    let state = m.r8(r + 0x14);
    if state == 3 || state == 2 {
        return true;
    }
    if state == 1 {
        let settings = m.r32(SETTINGS_PTR);
        return m.r8(settings + 0xd) == 0;
    }
    false
}

/// `scheduled_tasks_collect_visible_auctions` (`0x004DA1A0`): hasta tres subastas visibles, de `0x14` bytes cada una (`+0` orden, `+4` tres
/// ids de u16, `+0xA` ciudad, `+0xC` índice de la tarea, `+0x10` cuatro contadores por clase). `out` recibe el número de filas.
pub fn collect_visible_auctions(m: &mut dyn Mem, h: &mut dyn Heap, s: u32, out: u32) -> u32 {
    m.w32(out, 0);
    let n = count(m, s);
    let mut idx = head(m, s);
    if idx >= n {
        return 0;
    }
    loop {
        let r = rec(m, s, idx);
        if auction_visible(m, r) {
            let c = m.r32(out).wrapping_add(1);
            m.w32(out, c);
        }
        idx = u32::from(m.r16(r + 4));
        if idx >= n {
            break;
        }
    }
    if (m.r32(out) as i32) <= 0 {
        return 0;
    }
    if (m.r32(out) as i32) > 3 {
        m.w32(out, 3);
    }
    let total = m.r32(out);
    let list = h.alloc(m, total.wrapping_mul(0x14));
    if list == 0 {
        m.w32(out, 0);
        return 0;
    }
    let mut idx = head(m, s);
    let mut done = 0u32;
    let mut e = list;
    if idx < n {
        loop {
            let r = rec(m, s, idx);
            if auction_visible(m, r) {
                let due = m.r32(r);
                m.w32(e, due);
                let state = m.r8(r + 0x14);
                if state == 1 {
                    let bid = u32::from(m.r16(r + 0xe));
                    m.w32(e, bid.wrapping_add(due));
                }
                if state < 3 {
                    let v = m.r32(e).wrapping_add(0x100);
                    m.w32(e, v);
                }
                let town = u32::from(m.r8(r + 0x15));
                m.w32(e + 0xc, idx);
                m.w16(e + 0xa, town as u16);
                m.w32(e + 0x10, 0);
                let is_ship = m.r16(r + 6) == 10;
                for k in 0..3u32 {
                    let id = m.r16(r + 8 + 2 * k);
                    if is_ship {
                        if id < m.r16(SHIP_TABLE_LEN) {
                            let table = m.r32(SHIP_TABLE_PTR);
                            let class = u32::from(m.r8(table.wrapping_add(u32::from(id) * 0x180).wrapping_add(0xe)) & 3);
                            let a = e + 0x10 + class;
                            let v = m.r8(a).wrapping_add(1);
                            m.w8(a, v);
                            m.w16(e + 4 + 2 * k, id);
                        } else {
                            m.w16(e + 4 + 2 * k, 0xffff);
                        }
                    } else {
                        let base = m.r32(TOWN_TABLE).wrapping_add(town.wrapping_mul(0x9f8));
                        if m.r16(base.wrapping_add(0x776)) <= id & 0x7fff {
                            m.w16(e + 4 + 2 * k, 0xffff);
                        } else {
                            m.w8(e + 0x10, 0xff);
                            m.w16(e + 4 + 2 * k, id);
                        }
                    }
                }
                done += 1;
                e += 0x14;
                if m.r32(out) <= done {
                    return list;
                }
            }
            idx = u32::from(m.r16(r + 4));
            if idx >= n {
                break;
            }
        }
    }
    list
}

/// `scheduled_tasks_remove_mission_script_by_id` (`0x004F3564`): desenlaza y libera la tarea de guion (tipo `0x1B`) con ese id.
pub fn remove_mission_script_by_id(m: &mut dyn Mem, h: &mut dyn Heap, s: u32, id: u32) -> bool {
    let n = count(m, s);
    let mut link = s + 8;
    let mut idx = head(m, s);
    if idx >= n {
        return false;
    }
    loop {
        let r = rec(m, s, idx);
        if m.r16(r + 6) == 0x1b && u32::from(m.r16(r + 0x10)) == id {
            let cur = u32::from(m.r16(link));
            let r = rec(m, s, cur);
            let next = m.r16(r + 4);
            m.w16(link, next);
            let payload = m.r32(r + 0xc);
            if payload != 0 {
                h.free(payload);
            }
            m.w16(r + 6, 0xffff);
            let free_head = m.r16(s + 10);
            m.w16(r + 4, free_head);
            m.w16(s + 10, cur as u16);
            return true;
        }
        link = r + 4;
        idx = u32::from(m.r16(r + 4));
        if idx >= n {
            return false;
        }
    }
}

/// Bytes de una lista de nodos del guion de tipo `0x34` (`0x004F39C0`): 5 por nodo, siguiendo el puntero de `+0`.
fn script_chain_size(m: &mut dyn Mem, node: u32) -> u32 {
    let mut size = 5;
    let mut p = node;
    while m.r32(p) != 0 {
        p = m.r32(p);
        size += 5;
    }
    size
}

/// `scheduled_tasks_get_serialized_size` (`0x004D8080`): tamaño del bloque de tareas en el guardado.
pub fn serialized_size(m: &mut dyn Mem, s: u32) -> u32 {
    let n = count(m, s);
    let mut size = n * 0x18 + 6;
    for i in 0..n {
        let r = rec(m, s, i);
        let k = m.r16(r + 6);
        if k == 0x1b {
            size = size.wrapping_add(u32::from(m.r16(r + 0x12)) * 4);
        } else if k == 0x34 && m.r8(r + 0x15) > 6 {
            let node = m.r32(r + 8);
            if node != 0 {
                size = size.wrapping_add(script_chain_size(m, node));
            }
        }
    }
    size
}
