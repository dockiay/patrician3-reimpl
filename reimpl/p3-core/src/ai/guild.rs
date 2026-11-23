//! Reputación y gremios de la IA (C06): `get_merchant_reputation_in_town_int` (0x004F9580),
//! `town_get_guild_entry_fee` (0x0051B7E0) y `merchant_ai_try_join_guild` (0x004F4A20).

use crate::numeric::ftol_i32;

/// Límite de la reputación entera (`0x00672F58`): por encima devuelve `0x7FFF`.
const REPUTATION_CAP: f32 = 32758.0;
/// Coste y comisión de entrada.
const BASE_FEE: i32 = 0x4E20;
const FOREIGN_SURCHARGE: i32 = 0x2710;
const REPUTATION_FEE_CAP: i32 = 0x3A98;
const INCOME_FEE_CAP: i32 = 0xEA60;
/// Ciudad que queda elegida si ninguna reputación supera 1.0.
const NO_TOWN: usize = 100;
const JOIN_GUILD: u32 = 0x37;

/// Lo que lee la IA del comerciante.
#[derive(Clone, Debug)]
pub struct GuildMerchant {
    /// `+0`: dinero.
    pub gold: i32,
    /// `+8`: marcas (el bit 2 impide a la IA entrar en gremios).
    pub flags: u8,
    /// `+0xC`: primera oficina.
    pub office_head: u16,
    /// `+0x19`: ciudad natal.
    pub home: u8,
    /// `+0x468`: bit por ciudad con gremio.
    pub guild_mask: u32,
    /// `+0x46C`: ingresos.
    pub income: i32,
    /// `+0x2FC + 4·ciudad`: reputación de cada ciudad (f32). Las ciudades fuera de la lista leen otros campos del
    /// comerciante, que aquí valen cero.
    pub reputation: Vec<f32>,
}

impl GuildMerchant {
    fn reputation_of(&self, town: usize) -> f32 {
        self.reputation.get(town).copied().unwrap_or(0.0)
    }

    /// `get_merchant_reputation_in_town_int`: 0 si la ciudad no existe, `0x7FFF` por encima del límite y, si no, la
    /// reputación truncada a 16 bits.
    pub fn reputation_int(&self, town_count: u32, town: u32) -> u16 {
        if town_count <= town {
            return 0;
        }
        let r = self.reputation_of(town as usize);
        if REPUTATION_CAP < r {
            0x7FFF
        } else {
            ftol_i32(r as f64) as u16
        }
    }

    /// `town_get_guild_entry_fee` para la ciudad de propietario `owner` (`Town+0x2C0`).
    pub fn guild_entry_fee(&self, town_count: u32, owner: u8) -> i32 {
        let base = BASE_FEE + if self.home != owner { FOREIGN_SURCHARGE } else { 0 };
        let rep = self.reputation_int(town_count, owner as u32) as i16 as i32;
        let by_reputation = (rep.wrapping_abs().wrapping_mul(75))
            .wrapping_mul(40)
            .min(REPUTATION_FEE_CAP);
        let by_income = if self.income > 0 {
            (magic_div(self.income, 0x1062_4DD3, 5).wrapping_mul(50)).min(INCOME_FEE_CAP)
        } else {
            0
        };
        by_income.wrapping_sub(by_reputation).wrapping_add(base)
    }
}

/// División por una constante mágica de `imul`.
fn magic_div(value: i32, magic: i32, shift: u32) -> i32 {
    let high = ((value as i64 * magic as i64) >> 32) as i32 >> shift;
    high.wrapping_add(((high as u32) >> 31) as i32)
}

/// Oficina del comerciante: ciudad (`+0x2C6`) y siguiente (`+0x2C8`).
#[derive(Clone, Copy, Debug)]
pub struct GuildOffice {
    pub town: u8,
    pub next: u16,
}

/// Cola de operaciones.
pub trait GuildHost {
    /// `enqueue_operation` (0x0054AA70): los cinco dwords del registro.
    fn enqueue_operation(&mut self, record: &[u32; 5]);
}

/// `merchant_ai_try_join_guild`. `town_owner` es `Town+0x2C0` de cada ciudad y `flag_2a` el byte `0x006DE52A`.
pub fn try_join_guild(
    m: &GuildMerchant,
    task: u32,
    town_count: u32,
    town_owner: &[u8],
    offices: &[GuildOffice],
    flag_2a: bool,
    host: &mut dyn GuildHost,
) {
    if flag_2a || m.flags & 4 != 0 {
        return;
    }
    let mut best = 1.0f32;
    let mut chosen = NO_TOWN;
    let mut office = offices.get(m.office_head as usize).copied();
    while let Some(o) = office {
        if m.guild_mask & (1u32 << (o.town & 31)) == 0 {
            let r = m.reputation_int(town_count, o.town as u32) as i16 as f32;
            if r > best {
                best = r;
                chosen = o.town as usize;
            }
        }
        office = offices.get(o.next as usize).copied();
    }
    if chosen as u32 >= town_count {
        return;
    }
    let fee = m.guild_entry_fee(town_count, town_owner[chosen]);
    if (m.gold >> 2) < fee {
        return;
    }
    host.enqueue_operation(&[JOIN_GUILD, task & 0xFFFF, chosen as u32, (fee as u32) >> 1, 0]);
}
