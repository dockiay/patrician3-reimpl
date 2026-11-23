//! Actualización diaria de reputación y valor de un comerciante (`0x004F7BB0`, C06).

use crate::ai::misc::{blank_record, RankEnv, RankHost};
use crate::ai::reputation::{add, mul, Merchant};
use crate::numeric::ftol_i32;

/// Barco de la tabla `0x006DD7A4`: propietario `+0`, siguiente `+4`, `+0x10`, `+0x14`, `+0x18`, marineros `+0x40`, estado `+0x134`
/// y existencias `+0x54+4*bien`.
#[derive(Clone, Copy, Default)]
pub struct DailyShip {
    pub owner: u8,
    pub next: u16,
    pub state: u16,
    pub f10: i32,
    pub f14: i32,
    pub f18: i32,
    pub w40: u16,
    pub stock: [i32; 24],
}

/// Campos de la oficina que lee la actualización: reputación `+0x2C6`, siguiente `+0x2C8`, primer negocio `+0x2CC`,
/// palabras `+0x2DE` y bytes `+0x2E4`.
#[derive(Clone, Copy, Default)]
pub struct DailyOffice {
    pub rep_index: u8,
    pub next: u16,
    pub first_facility: u16,
    pub words: [u16; 3],
    pub bytes: [u8; 3],
}

/// Negocio de comerciante (0x14 bytes): valor `+4`, siguiente `+8`.
#[derive(Clone, Copy, Default)]
pub struct Facility {
    pub price: u16,
    pub next: u16,
}

pub struct DailyEnv<'a> {
    /// Argumento (índice del comerciante).
    pub index: u32,
    /// `0x006DE4B4` y `0x006DE4A4`.
    pub date: u32,
    pub word_4a4: u32,
    /// Byte `0x006DE4A0 == 1`.
    pub world_active: bool,
    pub ships: &'a [DailyShip],
    pub ship_count: u16,
    /// Palabra `+0x994` de cada ciudad.
    pub town_ship_refs: &'a [u16],
    pub offices: &'a [DailyOffice],
    pub facilities: &'a [Facility],
    pub facility_count: u16,
    pub rank: RankEnv<'a>,
}

/// Ganchos: deudas `0x4D9080`, valor de barco `0x52A7D0`, coste diario `0x517F70` y valor de oficina `0x500330`.
pub trait DailyHost: RankHost {
    fn debts(&mut self, index: u32) -> (i32, i32);
    fn ship_value(&mut self, ship: usize) -> i32;
    fn ship_crew_cost(&mut self, ship: usize) -> i32;
    fn office_value(&mut self, office: usize, town: u32) -> i32;
}

/// Precio por unidad de cada bien (`0x00673A18`), como bits de `f32`.
pub const GOOD_VALUES: [u32; 24] = [
    0x3D6147AF, 0x3EF50482, 0x3E6154CA, 0x3E322D0E, 0x3E11EB85, 0x3F0CCCCD, 0x3FB33333, 0x3F8CCCCD, 0x3F845A1D,
    0x40587AE1, 0x3ED33333, 0x3CE147AF, 0x3FA39581, 0x3F8F5C29, 0x3EE147AF, 0x3E8E5604, 0x3EE147AF, 0x3E6147AF,
    0x3F5AE147, 0x3D236E30, 0x41200000, 0x41000000, 0x41400000, 0x41600000,
];
/// Tabla de factores `0x00672DF0` (índice = byte de la oficina), como bits de `f32`.
pub const OFFICE_FACTORS: [u32; 16] = [
    0x3F800000, 0x3F1EB852, 0x3ED70A3D, 0x3E6B851F, 0x00000000, 0x3FA66666, 0x3F800000, 0x3F333333, 0x3FAAAAAB,
    0x402AAAAB, 0x40AAAAAB, 0x40800000, 0x402AAAAB, 0x402AAAAB, 0x3FAAAAAB, 0x40800000,
];

fn f32_of(bits: u32) -> f32 {
    f32::from_bits(bits)
}

impl Merchant {
    fn rep_add_f(&mut self, slot: usize, value: f64) {
        let r = add(value, self.f32(0x2FC + 4 * slot) as f64) as f32;
        self.set_f32(0x2FC + 4 * slot, r);
    }

    /// `0x004F7BB0` (`ret 4`; argumento `index`): día a día del comerciante. Las ramas de interfaz (globales `0x6CC7E8` y
    /// `0x6E51AC` no nulos tras un cambio de rango) no están portadas.
    pub fn update_reputation_and_value(&mut self, env: &DailyEnv, host: &mut dyn DailyHost) {
        let idx16 = env.index & 0xFFFF;
        let tc = env.rank.town_count;
        let home = self.bytes[0x19] as usize;
        let mut rec = blank_record();
        // libro de reputación
        let old: Vec<u8> = self.bytes[0x2FC..0x2FC + 0xA0].to_vec();
        self.bytes[0x3C4..0x3C4 + 0xA0].copy_from_slice(&old);
        self.bytes[0x2FC..0x2FC + 0xA0].fill(0);
        let weekly = (env.date >> 8) % 7 == 0;
        if weekly {
            self.bytes[0x25] = 0;
            if self.bytes[0x15] > 0xC8 {
                self.bytes[0x15] = 0xC8;
            }
            let (c480, c488, c490, c4a8, c4d0) = (
                self.i32(0x480),
                self.i32(0x488),
                self.i32(0x490),
                self.i32(0x4A8),
                self.i32(0x4D0),
            );
            let delta = c488
                .wrapping_sub(c480)
                .wrapping_sub(c490)
                .wrapping_sub(c4a8)
                .wrapping_sub(c4d0);
            self.set_i32(0, self.i32(0).wrapping_add(delta));
            if self.u16(8) == 0 {
                if c490 > 0 || c480 > 0 {
                    rec[5] = home as u8;
                    rec[8..12].copy_from_slice(&c490.to_le_bytes());
                    rec[0xC..0x10].copy_from_slice(&c480.to_le_bytes());
                    rec[4] = 0xF;
                    host.post_message(idx16 as i32, &rec);
                }
                if c488 > 0 || c4d0 > 0 {
                    rec[5] = home as u8;
                    rec[8..12].copy_from_slice(&c488.to_le_bytes());
                    rec[0xC..0x10].copy_from_slice(&c4d0.to_le_bytes());
                    rec[4] = 0x10;
                    host.post_message(idx16 as i32, &rec);
                }
                if c4a8 > 0 {
                    let c4d4 = self.i32(0x4D4);
                    rec[5] = home as u8;
                    rec[0xC..0x10].copy_from_slice(&c4d4.to_le_bytes());
                    rec[8..12].copy_from_slice(&c4a8.wrapping_sub(c4d4).to_le_bytes());
                    rec[4] = 0x11;
                    host.post_message(idx16 as i32, &rec);
                }
            }
            for o in [0x480, 0x490, 0x488, 0x4A8, 0x4D0, 0x4D4] {
                self.set_i32(o, 0);
            }
        }
        let (debt, owed) = host.debts(idx16);
        self.set_i32(0x46C, debt);
        self.set_i32(0x498, owed);
        // barcos
        self.set_i32(0x470, 0);
        self.set_i32(0x474, 0);
        self.set_u16(0x478, 0);
        self.set_i32(0x47C, 0);
        let mut sum14: i32 = 0;
        let mut sum18: i32 = 0;
        let mut idx = self.u16(0xE) as usize;
        while idx < env.ship_count as usize {
            let sh = &env.ships[idx];
            self.set_i32(0x47C, self.i32(0x47C).wrapping_add(1));
            if sh.state != 0x11 && sh.state != 0xE {
                self.set_u16(0x478, self.u16(0x478).wrapping_add(1));
                self.set_i32(0x470, self.i32(0x470).wrapping_add(sh.f10));
                sum18 = sum18.wrapping_add(sh.f18);
                sum14 = sum14.wrapping_add(sh.f14);
                self.set_i32(0x474, self.i32(0x474).wrapping_add(sh.w40 as i32));
                for g in (0..24).rev() {
                    let cnt = sh.stock[g];
                    if cnt > 0 {
                        let v = ftol_i32(mul(cnt as f32 as f64, f32_of(GOOD_VALUES[g]) as f64));
                        self.set_i32(0x46C, self.i32(0x46C).wrapping_add(v));
                    }
                }
                let value = host.ship_value(idx);
                self.set_i32(0x46C, self.i32(0x46C).wrapping_add(value));
                let cost = host.ship_crew_cost(idx);
                self.set_i32(0x490, self.i32(0x490).wrapping_add(cost));
                self.set_i32(0x494, self.i32(0x494).wrapping_add(cost));
            }
            idx = sh.next as usize;
        }
        self.set_u16(0x47A, 0);
        while sum14 > 0x20C49B {
            sum14 /= 10;
            sum18 /= 10;
        }
        if sum14 > 0 {
            let n = sum18.wrapping_mul(5).wrapping_mul(5).wrapping_mul(5).wrapping_shl(3);
            self.set_u16(0x47A, (n / sum14) as u16);
        } else {
            self.set_u16(0x47A, 0x3E8);
        }
        // ciudades que tienen un barco del comerciante como alcalde
        for i in 0..tc as usize {
            let r = env.town_ship_refs[i] as usize;
            if r < env.ship_count as usize && env.ships[r].owner as u32 == idx16 {
                self.rep_add_f(i, 1.0);
            }
        }
        // oficinas y negocios
        let mut o = self.u16(0xC) as usize;
        let ocount = env.rank.office_count as usize;
        if o < ocount {
            loop {
                let off = &env.offices[o];
                let v = host.office_value(o, home as u32);
                self.set_i32(0x46C, self.i32(0x46C).wrapping_add(v));
                for k in 0..3 {
                    let t = f32_of(OFFICE_FACTORS[off.bytes[k] as usize]) as f64;
                    let r = mul(mul(t, off.words[k] as f64), f64::from_bits(0x3F68_9374_BC6A_7EFA));
                    let slot = off.rep_index as usize;
                    let nv = add(r, self.f32(0x2FC + 4 * slot) as f64) as f32;
                    self.set_f32(0x2FC + 4 * slot, nv);
                }
                let mut f = off.first_facility as usize;
                if f < env.facility_count as usize {
                    loop {
                        let fac = &env.facilities[f];
                        let price = fac.price as u32 as i32 as f32;
                        let r = mul(price as f64, 0.01f32 as f64);
                        let slot = off.rep_index as usize;
                        let nv = add(r, self.f32(0x2FC + 4 * slot) as f64) as f32;
                        self.set_f32(0x2FC + 4 * slot, nv);
                        f = fac.next as usize;
                        if f >= env.facility_count as usize {
                            break;
                        }
                    }
                }
                o = off.next as usize;
                if o >= ocount {
                    break;
                }
            }
        }
        // valor de la compañía
        let total = self.i32(0x46C).wrapping_add(self.i32(0));
        self.set_i32(0x46C, total);
        if self.bytes[0x1E] != 0 && weekly {
            self.bytes[0x1E] -= 1;
        }
        let f_ships = ((self.i32(0x470) as f32) as f64 / 100_000f32 as f64) as f32;
        let mut f1 = f_ships;
        if f1 > 5.0 {
            f1 = 5.0;
        }
        let f2 = mul((total as f32) as f64, f32_of(0x3727_C5AC) as f64) as f32;
        f1 = if !(f2 > 5.0) {
            add(f2 as f64, f1 as f64) as f32
        } else {
            add(f1 as f64, 5.0) as f32
        };
        f1 = mul(f1 as f64, self.f32(0x464) as f64) as f32;
        for i in 0..tc as usize {
            self.rep_add_f(i, f1 as f64);
        }
        if (self.bytes[0x31] as u32) < tc {
            let f = (self.bytes[0x32] as i32) as f32;
            self.rep_add_f(home, f as f64);
        }
        for i in 0..tc as usize {
            let c3 = f32_of(0x3F7D_70A4) as f64;
            let b = mul(c3, self.f32(0x120 + 12 * i) as f64) as f32;
            self.set_f32(0x120 + 12 * i, b);
            let t = mul(c3, self.f32(0x124 + 12 * i) as f64) as f32;
            self.set_f32(0x124 + 12 * i, t);
            let a = self.f32(0x11C + 12 * i);
            let v = add(add(add(t as f64, a as f64), self.f32(0x2FC + 4 * i) as f64), b as f64) as f32;
            self.set_f32(0x2FC + 4 * i, v);
            if v < 0.0 {
                self.set_f32(0x2FC + 4 * i, 0.0);
            }
        }
        // rangos
        if env.world_active {
            let old_rank = self.bytes[0x39C + home];
            self.update_rank_in_town(&env.rank, home as u32, host);
            if self.u16(8) == 0 {
                let new_rank = self.bytes[0x39C + home];
                if old_rank < new_rank && new_rank < 6 {
                    rec[5] = home as u8;
                    rec[4] = 6;
                    rec[8..10].copy_from_slice(&(old_rank as u16).to_le_bytes());
                    rec[10..12].copy_from_slice(&(new_rank as u16).to_le_bytes());
                    rec[12..14].copy_from_slice(&(idx16 as u16).to_le_bytes());
                    rec[14..16].copy_from_slice(&(self.get_merchant_epithet_index() as u16).to_le_bytes());
                    host.post_message(-2, &rec);
                }
            }
        }
        let mut e = (((env.word_4a4 & 0xFFFF) + idx16) % tc) as usize;
        if e != home {
            self.update_rank_in_town(&env.rank, e as u32, host);
        }
        e += (tc >> 1) as usize;
        if !(tc as usize > e) {
            e -= tc as usize;
        }
        if e != home {
            self.update_rank_in_town(&env.rank, e as u32, host);
        }
        // ánimo, edad y humor
        if self.bytes[0x1F] < 0x14 {
            self.bytes[0x1F] += 1;
        }
        let w16 = self.u16(0x16) as i16;
        let delta = 11u16;
        self.set_u16(
            0x16,
            if w16 > 0 {
                (w16 as u16).wrapping_sub(delta)
            } else {
                (w16 as u16).wrapping_add(delta)
            },
        );
        let mut rank = self.bytes[0x39C + home] as i32;
        if rank > 6 {
            rank = rank * 2 - 6;
        }
        let m17 = self.bytes[0x17] as i8;
        if m17 > 0 {
            rank -= m17 as i32;
        }
        let target = (rank * 5 * 4).clamp(-30000, 0x7530);
        let mood = self.u16(0x22) as i16 as i32;
        self.set_u16(
            0x22,
            if mood > target {
                (mood as u16).wrapping_sub(1)
            } else {
                (mood as u16).wrapping_add(1)
            },
        );
    }
}
