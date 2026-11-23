//! Registro de asedio (`SiegeRecord`, 0x1100 bytes): `siege_record_clear` (0x006339C0),
//! `siege_record_forget_unit_entities` (0x006358A0) y `siege_record_reroll_goodwill_for_players` (0x00635040).
use crate::flatmem::Mem;

pub trait SiegeHost {
    /// `0x006278E0(objeto)`: destructor del objeto de `+8`.
    fn destroy(&mut self, obj: u32);
    /// `operator delete`.
    fn free(&mut self, addr: u32);
}

/// Pone a cero `n` dwords desde `base + start` recorriendo con un contador de byte como el original (`n` ≤ 255).
fn zero_counted(m: &mut dyn Mem, base: u32, count: u16) {
    assert!(
        count < 0x100,
        "el contador de byte del original no termina con 256 o más unidades"
    );
    for i in 0..count {
        m.w32(base + 4 * u32::from(i), 0);
    }
}

pub fn forget_unit_entities(m: &mut dyn Mem, r: u32) {
    m.w32(r + 0xc, 0);
    m.w32(r + 0x10, 0);
    m.w8(r + 0x100e, 0);
    m.w8(r + 0x100f, 0);
    let (n0, n1, n2, n3) = (
        m.r16(r + 0x1004),
        m.r16(r + 0x1006),
        m.r16(r + 0x1008),
        m.r16(r + 0x100a),
    );
    zero_counted(m, r + 0x14, n0);
    zero_counted(m, r + 0x410, n1);
    zero_counted(m, r + 0x80c, n2);
    zero_counted(m, r + 0xc08, n3);
    m.w32(r + 0x1004, 0);
    m.w32(r + 0x1008, 0);
    m.w16(r + 0x100c, 0);
}

pub fn clear(m: &mut dyn Mem, host: &mut dyn SiegeHost, r: u32) {
    let obj = m.r32(r + 8);
    if obj != 0 {
        host.destroy(obj);
        host.free(obj);
        m.w32(r + 8, 0);
    }
    for (head, link) in [(0x101cu32, 8u32), (0x1020, 0x10)] {
        let mut p = m.r32(r + head);
        while p != 0 {
            let next = m.r32(p + link);
            m.w32(r + head, next);
            host.free(p);
            p = m.r32(r + head);
        }
    }
    m.w32(r + 0xc, 0);
    m.w32(r + 0x10, 0);
    m.w8(r + 0x100e, 0);
    m.w8(r + 0x100f, 0);
    for off in [0x14u32, 0x410, 0x80c, 0xc08] {
        for k in 0..0xffu32 {
            m.w32(r + off + 4 * k, 0);
        }
    }
    m.w32(r + 0x1004, 0);
    m.w32(r + 0x1008, 0);
    m.w16(r + 0x100c, 0);
}

/// `reroll_goodwill_for_players(indicador)`: sortea la actitud del asedio según la ciudad (`+4`) y si hay comerciantes humanos en ella.
pub fn reroll_goodwill(m: &mut dyn Mem, r: u32, flag: u8) {
    let count = u32::from(m.r16(0x6DE4AA));
    let home = m.r8(r + 4);
    let mut humans = 0u8;
    for i in 0..count {
        let merchant = m.r32(0x6DE4A0 + 0x78).wrapping_add(i.wrapping_mul(0x650));
        if m.r8(merchant + 0x19) == home && m.r16(merchant + 8) == 0 {
            humans = humans.wrapping_add(1);
        }
    }
    let town = m.r32(0x6DE508).wrapping_add(u32::from(home).wrapping_mul(0x9f8));
    let seed = m.r32(town + 0x2d4).wrapping_mul(m.r32(0x6DE4B4));
    if flag != 0 {
        if humans != 1 {
            return;
        }
        m.w8(r + 0x101b, 0);
        m.w16(r + 0x1018, ((seed % 0x3001) + 0x1800) as u16);
    } else {
        if humans != 0 {
            return;
        }
        m.w8(r + 0x101b, 0xff);
        m.w16(r + 0x1018, ((seed % 0xa001) + 0x2800) as u16);
    }
}

pub trait SupplyHost {
    /// `0x005308A0(0x6DE4A0, comerciante, ciudad, 0)`: dirección del almacén (existencias en `+4 + 4·mercancía`) o 0.
    fn find_office(&mut self, merchant: u32, town: u32) -> u32;
    /// `0x005182B0(barco)`: recalcula la carga.
    fn recompute_load(&mut self, ship: u32);
}

const SMALL_LOT_FLAGS: u32 = 0x672C14;

/// `op_sell_wares_to_besieged_town` (0x00635990): el comerciante vende `qty` unidades de `ware` a `price` cada una a la ciudad sitiada,
/// tomándolas del almacén y de sus barcos atracados allí; sube la actitud del asedio (`+0x1018`, tope 0x4FFF).
pub fn sell_wares(
    m: &mut dyn Mem,
    host: &mut dyn SupplyHost,
    r: u32,
    merchant_index: u32,
    ware: u32,
    qty: u32,
    price: u32,
) {
    let merchant = m
        .r32(0x6DE4A0 + 0x78)
        .wrapping_add((merchant_index & 0xff).wrapping_mul(0x650));
    let town = u32::from(m.r8(r + 4));
    let office = host.find_office(merchant_index & 0xff, town);
    let ware = ware & 0xff;
    let small = m.r8(SMALL_LOT_FLAGS + ware) != 0;
    let lot: u32 = if small { 200 } else { 2000 };
    let mut left = qty;
    let mut sold = 0u32;
    let mut n;
    if office != 0 {
        let stock = m.r32(office + ware * 4 + 4);
        n = stock / lot;
        if n > left {
            n = left;
        }
        left -= n;
        sold = n;
        m.w32(office + ware * 4 + 4, stock.wrapping_sub(n.wrapping_mul(lot)));
    } else {
        n = qty;
    }
    let mut ship_index = u32::from(m.r16(merchant + 0xe));
    while ship_index < u32::from(m.r16(0x6DD894)) && left > 0 {
        let ship = m.r32(0x6DD7A4).wrapping_add(ship_index.wrapping_mul(0x180));
        if m.r8(ship + 0x39) == m.r8(r + 4) && m.r8(ship + 0x136) & 1 == 0 {
            let cargo = m.r32(ship + ware * 4 + 0x54);
            n = cargo / lot;
            if n > left {
                n = left;
            }
            left -= n;
            sold = sold.wrapping_add(n);
            m.w32(ship + ware * 4 + 0x54, cargo.wrapping_sub(n.wrapping_mul(lot)));
            host.recompute_load(ship);
        }
        ship_index = u32::from(m.r16(ship + 4));
    }
    let pay = n.wrapping_mul(price);
    let gold = m.r32(merchant).wrapping_add(pay);
    let total = m.r32(merchant + 0x49c).wrapping_add(pay);
    m.w32(merchant, gold);
    m.w32(merchant + 0x49c, total);
    let goodwill = m.r16(r + 0x1018);
    if goodwill > 0x4fff {
        return;
    }
    let mut units = sold;
    if !small {
        units = units.wrapping_mul(5);
    }
    if units >= 0x3e8 {
        m.w16(r + 0x1018, 0x4fff);
        return;
    }
    let add = ((u64::from(units << 9) * 0x51eb_851f) >> 32) as u32 >> 3;
    let new = u32::from(goodwill) + add;
    m.w16(r + 0x1018, if new > 0x4fff { 0x4fff } else { new as u16 });
}

pub trait SupplyLetterHost: SupplyHost {
    /// `0x004D6530(0x6DD730, comerciante, carta)`: carta de 16 bytes.
    fn send_letter(&mut self, merchant: u32, record: &[u8; 16]);
}

/// Busca en la lista de pedidos (`+0x1020`, enlace `+0x10`) el de `merchant` y `key`; devuelve (anterior, pedido).
fn find_request(m: &mut dyn Mem, r: u32, merchant: u8, key: u8) -> Option<(u32, u32)> {
    let mut prev = 0u32;
    let mut p = m.r32(r + 0x1020);
    while p != 0 {
        if m.r8(p + 5) == merchant && m.r8(p + 6) == key {
            return Some((prev, p));
        }
        prev = p;
        p = m.r32(p + 0x10);
    }
    None
}

/// `op_accept_siege_supply_request` (0x00635CD0): marca el pedido como aceptado y avisa al comerciante con la fecha límite.
pub fn accept_supply_request(m: &mut dyn Mem, host: &mut dyn SupplyLetterHost, r: u32, merchant: u32, key: u32) {
    let Some((_, p)) = find_request(m, r, merchant as u8, key as u8) else {
        return;
    };
    m.w8(p + 7, 1);
    let due = m.r32(p + 0xc) as i32;
    let (day, month, _) = super::tavern_passage::date_from_tick(due);
    let mut rec = [0u8; 16];
    rec[1] = 0x80;
    rec[2] = 0xff;
    rec[3] = 0xff;
    rec[4] = 0x65;
    rec[5] = m.r8(r + 4);
    rec[8] = m.r8(p + 4);
    rec[9] = m.r8(p + 2);
    rec[0xa] = m.r8(p);
    rec[0xb] = m.r8(p + 3);
    rec[0xc] = m.r8(p + 1);
    rec[0xd] = day;
    rec[0xe] = month;
    host.send_letter(merchant & 0xff, &rec);
}

/// `op_deliver_siege_supply_request` (0x00635DE0): entrega el pedido desde el almacén del comerciante; reordena la lista por fecha
/// y sube la actitud del asedio según el valor de lo entregado.
pub fn deliver_supply_request(m: &mut dyn Mem, host: &mut dyn SupplyHost, r: u32, merchant: u32, key: u32, due: u32) {
    let town = u32::from(m.r8(r + 4));
    let office = host.find_office(merchant & 0xff, town);
    let Some((prev, p)) = find_request(m, r, merchant as u8, key as u8) else {
        return;
    };
    // (mercancía, desplazamiento en la petición, desplazamiento en el almacén)
    const GOODS: [(u32, u32); 5] = [(3, 4), (12, 2), (11, 0), (15, 3), (0, 1)];
    let lot_of = |m: &mut dyn Mem, ware: u32| -> u32 {
        if m.r8(SMALL_LOT_FLAGS + ware) != 0 {
            200
        } else {
            2000
        }
    };
    let mut need = [0i32; 5];
    for (k, (ware, field)) in GOODS.iter().enumerate() {
        let lot = lot_of(m, *ware);
        need[k] = (lot * u32::from(m.r8(p + field))) as i32;
        if (m.r32(office + 4 + 4 * ware) as i32) < need[k] {
            return;
        }
    }
    for (k, (ware, _)) in GOODS.iter().enumerate() {
        let v = (m.r32(office + 4 + 4 * ware) as i32).wrapping_sub(need[k]);
        m.w32(office + 4 + 4 * ware, v as u32);
    }
    m.w32(p + 0xc, due);
    m.w8(p + 8, 1);
    let next = m.r32(p + 0x10);
    if prev != 0 {
        m.w32(prev + 0x10, next);
    } else {
        m.w32(r + 0x1020, next);
    }
    let head = m.r32(r + 0x1020);
    if head == 0 {
        m.w32(r + 0x1020, p);
        m.w32(p + 0x10, 0);
    } else {
        let mut at = head;
        let mut nx = m.r32(at + 0x10);
        let due_now = m.r32(p + 0xc);
        while nx != 0 && m.r32(nx + 0xc) < due_now {
            at = nx;
            nx = m.r32(at + 0x10);
        }
        let after = m.r32(at + 0x10);
        m.w32(p + 0x10, after);
        m.w32(at + 0x10, p);
    }
    // Valor de lo entregado: x87 a 53 bits, productos y sumas sin redondear a f32.
    let weights = |idx: usize| f64::from(f32::from_bits(super::loan_due::WARE_VALUE[idx]));
    let term = |m: &mut dyn Mem, ware: u32, field: u32, w: f64| -> f64 {
        let lot = f64::from(lot_of(m, ware) as f32);
        let count = f64::from(f32::from(m.r8(p + field)));
        lot * count * w
    };
    let mut v = term(m, 0, 1, weights(0));
    v += term(m, 15, 3, weights(15));
    v += term(m, 11, 0, weights(11));
    v += term(m, 12, 2, weights(12));
    v += term(m, 3, 4, weights(3));
    let value = crate::numeric::ftol_i32(v) as u32;
    let goodwill = m.r16(r + 0x1018);
    if goodwill > 0x4fff {
        return;
    }
    if value >= 0x7a120 {
        m.w16(r + 0x1018, 0x4fff);
        return;
    }
    let x = value << 7;
    let high = ((u64::from(x) * 0x4f8b_588f) >> 32) as u32;
    let add = (((x - high) >> 1) + high) >> 11;
    let total = u32::from(goodwill) + add;
    if total > 0x4fff {
        m.w16(r + 0x1018, 0x4fff);
    } else {
        m.w16(r + 0x1018, total as u16);
    }
}

/// Suma `add` a la actitud (`+0x1018`) con tope 0x4FFF (lo común de las tres funciones de ganancia).
fn bump_goodwill(m: &mut dyn Mem, r: u32, goodwill: u16, add: u32) {
    let total = u32::from(goodwill) + add;
    m.w16(r + 0x1018, if total > 0x4fff { 0x4fff } else { total as u16 });
}

/// `siege_record_add_goodwill_from_value` (0x00634870): actitud según un valor en monedas (tope en 0x7A120).
pub fn add_goodwill_from_value(m: &mut dyn Mem, r: u32, value: u32) {
    let goodwill = m.r16(r + 0x1018);
    if goodwill > 0x4fff {
        return;
    }
    if value >= 0x7a120 {
        m.w16(r + 0x1018, 0x4fff);
        return;
    }
    let x = value << 7;
    let high = ((u64::from(x) * 0x4f8b_588f) >> 32) as u32;
    let add = (((x - high) >> 1) + high) >> 11;
    bump_goodwill(m, r, goodwill, add);
}

/// `siege_record_add_goodwill_from_gift` (0x006348E0): modo 1 baja a la franja anterior; otro modo suma por valor (tope 0x3D090).
pub fn add_goodwill_from_gift(m: &mut dyn Mem, r: u32, mode: u32) {
    if mode == 1 {
        let g = m.r16(r + 0x1018);
        if g > 0xfff {
            m.w16(r + 0x1018, ((u32::from(g) & 0xf000).wrapping_sub(1)) as u16);
        }
        return;
    }
    let goodwill = m.r16(r + 0x1018);
    if goodwill > 0x4fff {
        return;
    }
    if mode >= 0x3d090 {
        m.w16(r + 0x1018, 0x4fff);
        return;
    }
    let x = mode << 8;
    let high = ((u64::from(x) * 0x4f8b_588f) >> 32) as u32;
    let add = (((x - high) >> 1) + high) >> 11;
    bump_goodwill(m, r, goodwill, add);
}

/// `siege_record_add_goodwill_from_ware` (0x00634970): actitud por una cantidad de mercancía (lote 5x si el indicador es 0).
pub fn add_goodwill_from_ware(m: &mut dyn Mem, r: u32, ware: u32, qty: u32) {
    let goodwill = m.r16(r + 0x1018);
    if goodwill > 0x4fff {
        return;
    }
    let units = if m.r8(SMALL_LOT_FLAGS + (ware & 0xff)) == 0 {
        qty.wrapping_mul(5)
    } else {
        qty
    };
    if units >= 0x3e8 {
        m.w16(r + 0x1018, 0x4fff);
        return;
    }
    let x = units << 9;
    let add = ((u64::from(x) * 0x51eb_851f) >> 32) as u32 >> 3;
    bump_goodwill(m, r, goodwill, add);
}

/// Llamadas externas de las órdenes del asedio.
pub trait OrderHost {
    /// `0x00529F90(ciudad, a, 8)`.
    fn town_event(&mut self, town: u32, a: u32);
    /// `0x0062B860(a, b, 0)` (stdcall): coordenadas de 16 bits.
    fn move_to(&mut self, a: u16, b: u16);
    /// `0x0062DD70(objeto, 1)`.
    fn refresh_unit(&mut self, unit: u32);
    /// `0x004D6530(0x6DD730, -2, carta)`.
    fn send_letter(&mut self, record: &[u8; 16]);
    /// `0x00634C50(registro, 1)`.
    fn finish_order(&mut self, r: u32);
}

/// `siege_record_order_surrender` (0x006347B0).
pub fn order_surrender(m: &mut dyn Mem, host: &mut dyn OrderHost, r: u32) {
    let o = m.r32(r + 8);
    if m.r16(o + 0x3a) != 0 || m.r16(o + 0x3c) != 0 {
        return;
    }
    m.w16(o + 0x42, 0);
    let b = m.r8(o + 0x6f);
    m.w8(o + 0x6e, b);
    m.w8(o + 0x6f, 0xff);
    let v = m.r16(o + 0x68);
    m.w16(o + 0x3c, v);
    m.w8(o + 0x79, 1);
    m.w8(o + 0x78, 1);
    if m.r8(o + 0x77) != 0 {
        let (a, c) = (m.r16(o + 0x3e), m.r16(o + 0x40));
        host.move_to(a, c);
        host.refresh_unit(o);
    }
}

/// `siege_record_order_attack` (0x00634820).
pub fn order_attack(m: &mut dyn Mem, host: &mut dyn OrderHost, r: u32) {
    let o = m.r32(r + 8);
    if m.r16(o + 0x42) != 0 || m.r16(o + 0x3a) != 0 || m.r16(o + 0x3c) != 0 || m.r8(o + 0x78) != 0 {
        return;
    }
    m.w8(o + 0x78, 1);
    if m.r8(o + 0x77) != 0 {
        let (a, c) = (m.r16(o + 8), m.r16(o + 0xa));
        host.move_to(a, c);
        host.refresh_unit(o);
    }
}

/// `siege_record_buy_freedom` (0x00634660): la ciudad compra la liberación; aviso al jugador y cierre de la orden.
pub fn buy_freedom(m: &mut dyn Mem, host: &mut dyn OrderHost, r: u32, arg: u32) {
    let towns = m.r32(0x6DE508);
    let idx = u32::from(m.r8(r + 4));
    let town = towns.wrapping_add(idx.wrapping_mul(0x9f8));
    host.town_event(town, arg);
    let flags = m.r32(town + 0x2c8) & !0x10;
    m.w32(town + 0x2c8, flags);
    let o = m.r32(r + 8);
    let w = m.r16(o + 0x6a);
    m.w16(o + 0x3a, w.wrapping_add(w));
    m.w16(o + 0x42, 0);
    m.w8(o + 0x78, 1);
    m.w8(o + 0x79, 1);
    if m.r8(o + 0x77) != 0 {
        let (a, c) = (m.r16(o + 0x36), m.r16(o + 0x38));
        host.move_to(a, c);
        host.refresh_unit(o);
    }
    let mut rec = [0u8; 16];
    rec[1] = 0x80;
    rec[2] = 0xff;
    rec[3] = 0xff;
    rec[4] = 0x38;
    rec[5] = m.r8(r + 4);
    let tick = m.r32(0x6DE4B4) >> 8;
    let day = (tick as u16).wrapping_sub(m.r16(o + 0x584)).wrapping_add(1);
    rec[8..10].copy_from_slice(&day.to_le_bytes());
    rec[10] = 0xff;
    rec[11] = 0xff;
    rec[12] = 0xff;
    rec[13] = 0xff;
    let al = m.r8(o + 0x6f);
    let w14 = if al != 0xff {
        u16::from(al).wrapping_sub(1)
    } else {
        u16::from(m.r8(o + 0x6e))
    };
    rec[14..16].copy_from_slice(&w14.to_le_bytes());
    host.send_letter(&rec);
    host.finish_order(r);
}

/// `siege_record_find_delivery_agreement` (0x00636590): acuerdo de entrega con `key` en `+3` (lista `+0x101c`, enlace `+8`), o 0.
pub fn find_delivery_agreement(m: &mut dyn Mem, r: u32, key: u32) -> u32 {
    let mut p = m.r32(r + 0x101c);
    while p != 0 && u32::from(m.r8(p + 3)) != key & 0xff {
        p = m.r32(p + 8);
    }
    p
}

/// `siege_record_find_pending_supply_request` (0x006365B0): pedido aceptado y aún sin entregar de `merchant`, o 0.
pub fn find_pending_supply_request(m: &mut dyn Mem, r: u32, merchant: u32) -> u32 {
    let mut p = m.r32(r + 0x1020);
    while p != 0 {
        if u32::from(m.r8(p + 5)) == merchant & 0xff && m.r8(p + 7) != 0 && m.r8(p + 8) == 0 {
            return p;
        }
        p = m.r32(p + 0x10);
    }
    0
}

/// `siege_record_find_next_accepted_supply_request` (0x006365E0): siguiente pedido aceptado de `merchant` tras `prev` (o desde el primero), o 0.
pub fn find_next_accepted_supply_request(m: &mut dyn Mem, r: u32, merchant: u32, prev: u32) -> u32 {
    let mut p = if prev != 0 {
        m.r32(prev + 0x10)
    } else {
        m.r32(r + 0x1020)
    };
    while p != 0 {
        if u32::from(m.r8(p + 5)) == merchant & 0xff && m.r8(p + 7) != 0 {
            return p;
        }
        p = m.r32(p + 0x10);
    }
    0
}

/// `siege_record_get_urgent_demanded_ware` (0x00635660): mercancía exigida que toca según el turno, o 0xFF.
pub fn get_urgent_demanded_ware(m: &mut dyn Mem, r: u32) -> u8 {
    let bl = u32::from(m.r8(0x6DE4A1));
    let town = u32::from(m.r8(r + 4));
    let mut count = 0u8;
    let mut res = 0xffu8;
    for i in 0..4 {
        let e = r + 0x1010 + i;
        if m.r8(e + 4) == 3 {
            count = count.wrapping_add(1);
            if (town + bl) % u32::from(count) == 0 {
                res = m.r8(e);
            }
        }
    }
    res
}

/// `siege_record_get_displeasure_level` (0x005F3750): franja de actitud (0..4).
pub fn get_displeasure_level(m: &mut dyn Mem, r: u32) -> u32 {
    let g = m.r16(r + 0x1018);
    if g > 0x3fff {
        4
    } else {
        u32::from(g) >> 12
    }
}

/// `0x006278F0(objeto)`: tamaño guardado del objeto del asedio.
pub trait SaveSizeHost {
    fn unit_save_size(&mut self, unit: u32) -> u32;
}

/// `siege_record_get_save_size` (0x00633530).
pub fn get_save_size(m: &mut dyn Mem, host: &mut dyn SaveSizeHost, r: u32) -> u32 {
    let mut a = 0u8;
    let mut p = m.r32(r + 0x101c);
    while p != 0 {
        p = m.r32(p + 8);
        a = a.wrapping_add(1);
    }
    let mut b = 0u8;
    let mut p = m.r32(r + 0x1020);
    while p != 0 {
        p = m.r32(p + 0x10);
        b = b.wrapping_add(1);
    }
    let mut size = (u32::from(a) + u32::from(b) * 2) * 8 + 0x14;
    let unit = m.r32(r + 8);
    if unit != 0 {
        size = size.wrapping_add(host.unit_save_size(unit));
    }
    size
}

/// Llamadas externas de la inicialización del registro.
pub trait InitHost {
    /// `0x00635220(registro)`: elige las mercancías exigidas.
    fn pick_demands(&mut self, r: u32);
}

/// `siege_record_init_goodwill` (0x00635170): actitud inicial (0x3FFF si algún comerciante local no tiene barcos; si no, aleatoria por ciudad y tick).
pub fn init_goodwill(m: &mut dyn Mem, host: &mut dyn InitHost, r: u32) {
    let count = u32::from(m.r16(0x6DE4AA));
    let base = m.r32(0x6DE4A0 + 0x78);
    let town_idx = m.r8(r + 4);
    let mut found = 0u8;
    for i in 0..count {
        let mer = base.wrapping_add(i.wrapping_mul(0x650));
        if m.r8(mer + 0x19) == town_idx && m.r16(mer + 8) == 0 {
            found = found.wrapping_add(1);
        }
    }
    if found != 0 {
        m.w16(r + 0x1018, 0x3fff);
        m.w8(r + 0x101b, 0);
    } else {
        let towns = m.r32(0x6DE508);
        let v = m
            .r32(towns.wrapping_add(u32::from(town_idx).wrapping_mul(0x9f8)) + 0x2d4)
            .wrapping_mul(m.r32(0x6DE4B4));
        m.w8(r + 0x101b, 0xff);
        m.w16(r + 0x1018, ((v % 0xa001) + 0x4800) as u16);
    }
    host.pick_demands(r);
}

/// `0x0064F7B9(0xC)`: reserva de memoria del juego.
pub trait AgreementHost {
    fn alloc(&mut self, size: u32) -> u32;
}

/// `op_add_siege_delivery_agreement` (0x006356D0): añade un acuerdo de entrega (12 bytes) si no existe ya uno con `key`.
pub fn add_delivery_agreement(
    m: &mut dyn Mem,
    host: &mut dyn AgreementHost,
    r: u32,
    kind: u32,
    key: u32,
    a2: u32,
    a3: u32,
    flag: u32,
) {
    let key = key & 0xff;
    if find_delivery_agreement(m, r, key) != 0 {
        return;
    }
    let e = host.alloc(12);
    m.w8(e, kind as u8);
    m.w8(e + 1, 6);
    m.w8(e + 2, if flag & 0xff != 0 { 0xff } else { 0 });
    m.w8(e + 3, key as u8);
    m.w16(e + 4, a2 as u16);
    m.w16(e + 6, a3 as u16);
    let head = m.r32(r + 0x101c);
    m.w32(e + 8, head);
    m.w32(r + 0x101c, e);
}

/// `op_accept_siege_supply_request_and_mark_letter` (0x00540150): acepta el pedido de la ciudad `this+4` y marca la carta `this+0x10`.
pub fn accept_supply_request_and_mark_letter(m: &mut dyn Mem, host: &mut dyn SupplyLetterHost, this: u32) {
    let town = m.r32(this + 4);
    if town >= m.r32(0x6DE4B0) {
        return;
    }
    let r = m.r32(0x6DF270).wrapping_add(town.wrapping_mul(0x1024));
    let merchant = u32::from(m.r8(this + 8));
    let key = u32::from(m.r8(this + 0xc));
    accept_supply_request(m, host, r, merchant, key);
    let count = u32::from(m.r16(0x6DD736));
    let idx = m.r32(this + 0x10);
    if idx >= count {
        return;
    }
    let letter = m.r32(0x6DD730).wrapping_add(idx << 4);
    if m.r8(letter + 4) == 0x64 {
        m.w8(letter + 0xf, 0xfe);
    }
}

pub trait PickHost {
    /// `0x00532290` (ECX = 0x6DE4A0): rellena dos tablas de 24 valores (primero la de `+0x94`, luego la de `+0x34`).
    fn ware_tables(&mut self) -> ([u32; 24], [u32; 24]);
}

/// `siege_record_pick_demands` (0x00635220): elige las cuatro mercancías exigidas (`+0x1010..+0x1013`) y su nivel
/// (`+0x1014..+0x1017`) entre las 24 mercancías, quedándose con las cuatro de mayor puntuación (inserción en lista ordenada).
/// Divide por la tabla de 24 divisores en `0x67B638`.
pub fn pick_demands(m: &mut dyn Mem, host: &mut dyn PickHost, r: u32) {
    let mut all_empty = true;
    for k in 0..4u32 {
        if m.r8(r + 0x1010 + k) != 0xff {
            all_empty = false;
            break;
        }
    }
    let (a, b) = host.ware_tables();
    let mut t = [0xffu8; 4];
    let mut v = [0u32; 4];
    for ware in 0..24usize {
        let mut ecx = a[ware];
        let mut esi = b[ware];
        let town = |m: &mut dyn Mem| {
            let towns = m.r32(0x6DE508);
            let idx = u32::from(m.r8(r + 4));
            m.r32(towns.wrapping_add(idx.wrapping_mul(0x9f8)) + 0x2d4)
        };
        let eax: u32;
        let mut fall = false;
        let mut dividend = 0u32;
        if ecx > 0x80_0000 {
            esi >>= 9;
            if esi == 0 {
                fall = true;
            } else {
                dividend = town(m).wrapping_add(ecx);
                ecx >>= 9;
            }
        } else if esi == 0 {
            fall = true;
        } else {
            dividend = town(m).wrapping_add(ecx);
        }
        if fall {
            eax = ecx;
        } else {
            let rem = dividend % 0x66;
            eax = (rem.wrapping_add(0xcc)).wrapping_mul(ecx) / esi;
        }
        let score = eax / m.r32(0x67B638 + 4 * ware as u32);
        if !(score > 0xff || all_empty) {
            continue;
        }
        if score <= v[3] {
            continue;
        }
        let mut al = 3usize;
        let mut idx = 3usize;
        loop {
            if al < 3 {
                t[idx + 1] = t[idx];
                v[idx + 1] = v[idx];
            }
            t[idx] = ware as u8;
            v[idx] = score;
            if al == 0 {
                break;
            }
            al -= 1;
            idx = al;
            if !(score > v[idx]) {
                break;
            }
        }
    }
    if all_empty {
        for k in 1..4 {
            if v[k] < 0x100 {
                t[k] = 0xff;
            }
        }
    }
    let mut n = [0u8; 4];
    for e in 0..4 {
        if t[e] == 0xff {
            continue;
        }
        n[e] = 1;
        for k in 0..4u32 {
            if t[e] == m.r8(r + 0x1010 + k) {
                let lvl = m.r8(r + 0x1014 + k);
                n[e] = if lvl < 3 { lvl + 1 } else { lvl };
            }
        }
    }
    for k in 0..4usize {
        m.w8(r + 0x1010 + k as u32, t[k]);
        m.w8(r + 0x1014 + k as u32, n[k]);
    }
}
