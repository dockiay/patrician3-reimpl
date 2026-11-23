//! Ofertas de préstamo al jugador de una ciudad (`town_build_loan_offers_for_player`, `0x0051CE80`).
//! Tres ranuras (`Town+0x9C6`, seis bytes: semanas en `+3`, importe u16 en `+4`) generan registros de 0x10 bytes: importe redondeado a 50, tipo
//! (`+4`) por la reputación y el plazo, y total a devolver (`+0`). Devuelve el número de registros; `-1` si el comerciante está inhabilitado
//! (`Merchant+0x1E`), `-2` si le falta capacidad de crédito (menos de 2000). `changed` se pone a 1 cuando un campo difiere del registro previo.
//! Los intermedios del x87 (53 bits) se redondean a f32 solo donde el original hace `fstp dword`.
use crate::{ai::guild::GuildMerchant, ai::reputation::Merchant, numeric::ftol_i32, town::Town};

fn f(x: f64) -> f64 {
    f64::from(x as f32)
}

/// `merchants` es el vector `0x6DE4A0+0x78`; `player` es `[0x6DFC14]`; `town_count` es `[0x6DE4B0]`.
pub fn build(
    town: &Town,
    merchants: &[Merchant],
    player: u8,
    town_count: u32,
    recs: &mut [u8],
    changed: &mut bool,
) -> i32 {
    *changed = false;
    if usize::from(player) >= merchants.len() {
        return 0;
    }
    let m = &merchants[usize::from(player)];
    if m.bytes[0x1e] != 0 {
        return -1;
    }
    let income = m.i32(0x46c);
    let spent = m.i32(0x498);
    let capacity = (income / 2).wrapping_sub(spent) as u32;
    if capacity < 0x7d0 {
        return -2;
    }
    // `imul 0x10624DD3 >> 5` más el bit de signo: income / 1000 hacia cero; tope 2000 sin signo.
    let q = ((i64::from(income) * 0x1062_4DD3) >> 32) as i32 >> 5;
    let q = q.wrapping_add(((q as u32) >> 31) as i32) as u32;
    let limit = q.min(2000);
    let owner = town.u8(0x2c0);
    let guild = GuildMerchant {
        gold: 0,
        flags: 0,
        office_head: 0,
        home: 0,
        guild_mask: 0,
        income: 0,
        reputation: (0..town_count as usize)
            .map(|t| f32::from_bits(m.i32(0x2fc + 4 * t) as u32))
            .collect(),
    };
    let rep = i32::from(guild.reputation_int(town_count, u32::from(owner)) as i16).clamp(0, 30);
    let mut count = 0usize;
    for b in 0..3usize {
        let slot = 0x9c6 + 6 * b;
        let weeks = town.u8(slot + 3);
        let amount = u32::from(town.u16(slot + 4));
        if amount == 0 || weeks == 0 {
            continue;
        }
        let r = &mut recs[count * 0x10..count * 0x10 + 0x10];
        r[0xd] = player;
        r[0xc] = 0xff;
        r[0xe] = owner;
        r[0xb] = 0;
        r[0xf] = ((b as u8) << 4).wrapping_add(2);
        let home = usize::from(m.bytes[0x19]);
        r[5] = m.bytes[0x39c + home];
        r[6] = m.bytes[0x1c];
        r[7] = m.bytes[0x1d];
        let rounded = ((amount + limit) / 50).wrapping_mul(50);
        if rounded != (u32::from(r[8]) | u32::from(r[9]) << 8) {
            *changed = true;
            r[8..10].copy_from_slice(&(rounded as u16).to_le_bytes());
        }
        if rounded > capacity {
            return count as i32;
        }
        if weeks != r[0xa] {
            *changed = true;
            r[0xa] = weeks;
        }
        let principal = u32::from(r[8]) | u32::from(r[9]) << 8;
        let a = f(f64::from(principal));
        let w = f(f64::from(u32::from(weeks)));
        let t = f(w * a);
        let s = f(t.sqrt());
        let x = f64::from(3000.0f32) / s + f64::from(15.0f32);
        let rr = f(f64::from(rep));
        let y = f(x - rr / f64::from(3.0f32));
        let rate = ftol_i32(y) as u8;
        // el original compara el entero completo con el byte previo (cl)
        if (ftol_i32(y) as u32) != u32::from(r[4]) {
            *changed = true;
            r[4] = rate;
        }
        let factor = f(f(f64::from(r[4])) * f64::from(0.001f32) + 1.0);
        let mut v = f(f64::from(principal));
        for _ in 0..r[0xa] {
            v = f(v * factor);
        }
        if v > 65000.0 {
            return count as i32;
        }
        let total = (ftol_i32(v) as u32) & 0xffff;
        if total != u32::from_le_bytes([r[0], r[1], r[2], r[3]]) {
            *changed = true;
            r[0..4].copy_from_slice(&total.to_le_bytes());
        }
        count += 1;
    }
    count as i32
}
