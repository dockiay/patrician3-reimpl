//! `town_create_tavern_ware_offer` (0x00526650): la oferta de mercancía de la taberna.
use super::prices::{buy_price, sell_price, Market, SMALL_LOT_WARES};
use crate::ai::reputation::Merchant;
use crate::town::Town;

/// Mercancías (0..20) que pueden ofrecerse.
const WARE_MASK: u32 = 0x47FAF;
const STOCK_MARGIN: i32 = 0x9C40;
const DEFAULT_LIMIT: i32 = 0x7530;
const RECORD_STRIDE: usize = 0x180;

fn market(town: &Town, ware: usize) -> Market {
    Market {
        trading: town.more_flags() & 0x100 != 0,
        stock: town.stock(ware),
        thresholds: std::array::from_fn(|i| town.threshold(ware, i)),
        marked_ware: town.u8(0x6f0),
    }
}

/// Registro de 16 bytes que devuelve el original (`None` si el asignador falla o no hay mercancía con excedente).
/// `records`: tabla `0x6DD7A4` (0x180 bytes por entrada), `head`/`count`: `0x6DD888`/`0x6DD894`.
pub fn create(
    town: &Town,
    records: &[u8],
    head: u32,
    count: u32,
    merchants: &[Merchant],
    tick: u32,
    player: u8,
    difficulty: f32,
    alloc_ok: bool,
) -> Option<[u8; 16]> {
    let mut best = 0i32;
    let mut ware = 0xffusize;
    for i in 0..20usize {
        if (1u32 << i) & WARE_MASK != 0 {
            let d = town
                .stock(i)
                .wrapping_sub(town.threshold(i, 1))
                .wrapping_sub(STOCK_MARGIN);
            if d > best {
                best = d;
                ware = i;
            }
        }
    }
    if ware >= 0x14 {
        return None;
    }
    let owner = town.u8(0x2c0);
    let count = count & 0xffff;
    let mut cursor = head & 0xffff;
    let mut value = 0i32;
    let mut band = best as u32;
    if cursor < count {
        loop {
            let r = &records[cursor as usize * RECORD_STRIDE..][..RECORD_STRIDE];
            cursor = u32::from(u16::from_le_bytes([r[6], r[7]]));
            if r[0x39] == owner {
                let idx = usize::from(r[0]);
                if idx < merchants.len() {
                    let m = &merchants[idx].bytes;
                    if m[0x19] != owner && m[8] & 4 != 0 {
                        value = i32::from_le_bytes(r[0x10..0x14].try_into().unwrap());
                        band = u32::from(r[0]);
                        break;
                    }
                }
            }
            if cursor >= count {
                break;
            }
        }
    }
    if cursor >= count {
        band = u32::from(owner);
        value = DEFAULT_LIMIT;
    }
    let available = town.stock(ware).wrapping_sub(town.threshold(ware, 1));
    if value > available {
        value = available;
    }
    if !alloc_ok {
        return None;
    }
    let mut out = [0u8; 16];
    out[0xc] = band as u8;
    out[0..4].copy_from_slice(&tick.to_le_bytes());
    out[0xe] = ware as u8;
    let (magic, shift, unit) = if SMALL_LOT_WARES[ware] {
        (0x51EB_851Fi64, 6, 200i32)
    } else {
        (0x1062_4DD3i64, 7, 2000i32)
    };
    let hi = ((i64::from(value) * magic) >> 32) as i32;
    let mut q = hi >> shift;
    q = q.wrapping_add(((q as u32) >> 31) as i32);
    let mut q16 = q as u16;
    if q16 == 0 {
        q16 = 1;
    }
    out[8..10].copy_from_slice(&q16.to_le_bytes());
    let amount = i32::from(q16).wrapping_mul(unit);
    let m = market(town, ware);
    let sell = sell_price(ware, &m, amount, difficulty);
    let buy = buy_price(ware, &m, amount);
    out[4..8].copy_from_slice(&(sell.wrapping_add(buy) >> 1).to_le_bytes());
    out[0xd] = player;
    out[0xa..0xc].copy_from_slice(&0xffffu16.to_le_bytes());
    Some(out)
}
