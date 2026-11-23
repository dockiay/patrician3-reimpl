//! Embargo de bienes del comerciante por una deuda (`0x004F6570`, C06).

use crate::ai::misc::{blank_record, MessageHost};
use crate::ai::reputation::Merchant;

/// Barco embargable: ciudad `+0x39`, estado `+0x134`, siguiente `+4`, etiqueta `+0x15E` y existencias `+0x54+4*bien`.
#[derive(Clone, Copy, Default)]
pub struct SeizeShip {
    pub owner: u8,
    pub state: u16,
    pub next: u16,
    pub tag: u16,
    pub stock: [i32; 24],
}

/// Ganchos del embargo: oficina `0x5308A0`, valoración `0x52E1D0`, aviso `0x548CA0`, `0x517770(4)` y recuento `0x5182B0`.
pub trait SeizeHost: MessageHost {
    fn find_office(&mut self, kind: u32, town: u32) -> Option<usize>;
    fn sell_value(&mut self, good: u32, town: u32, amount: i32) -> i32;
    fn notify(&mut self, args: [u32; 6]);
    fn ship_changed(&mut self, ship: usize);
}

impl Merchant {
    /// `0x004F6570` (`ret 0xC`; argumentos `debt`, `kind`, `town`): cobra `debt` con el oro, luego con las existencias de la
    /// oficina de la ciudad y, si queda deuda, con las del primer barco de la ciudad que tenga algo; los bienes pasan a
    /// `town_stock`. Con oro la suma cobrada se trunca a 16 bits cuando no alcanza (como el original).
    #[allow(clippy::too_many_arguments)]
    pub fn seize_assets_for_debt(
        &mut self,
        debt: &mut i32,
        kind: u8,
        town: u8,
        town_stock: &mut [i32; 24],
        offices: &mut [[i32; 24]],
        ships: &mut [SeizeShip],
        ship_count: u16,
        host: &mut dyn SeizeHost,
    ) {
        let t = town as u32;
        let mut r = blank_record();
        r[4] = 0x13;
        r[5] = town;
        let mut total: u32 = 0;
        let mut mode: u8 = 0;
        let mut flag: u8 = 0;
        if *debt <= 0 {
            return;
        }
        let gold = self.i32(0);
        if gold > 0 {
            if gold >= *debt {
                self.set_i32(0, gold - *debt);
                let old = *debt;
                *debt = 0;
                r[0xB] = 1;
                r[0xC..0x10].copy_from_slice(&old.to_le_bytes());
                host.post_message(kind as i32, &r);
                return;
            }
            *debt -= gold;
            total = gold as u16 as u32;
            self.set_i32(0, 0);
        }
        if let Some(oi) = host.find_office(kind as u32, t) {
            for g in (0..24).rev() {
                if *debt > 0 {
                    let s = offices[oi][g];
                    if s > 0 {
                        let v = host.sell_value(g as u32, t, s);
                        town_stock[g] = town_stock[g].wrapping_add(s);
                        offices[oi][g] = 0;
                        total = total.wrapping_add(v as u32);
                        *debt = debt.wrapping_sub(v);
                        mode = 1;
                    }
                }
            }
            if mode == 1 {
                if *debt <= 0 {
                    total = total.wrapping_add(*debt as u32);
                    self.set_i32(0, self.i32(0).wrapping_sub(*debt));
                    flag = 1;
                    *debt = 0;
                }
                if self.u16(8) != 0 {
                    return;
                }
                r[0xA] = mode;
                r[0xB] = flag;
                r[0xC..0x10].copy_from_slice(&total.to_le_bytes());
                host.post_message(kind as i32, &r);
                host.notify([kind as u32, 0, 0, t, 0x15, t]);
                return;
            }
        }
        let mut idx = self.u16(0xE) as usize;
        while idx < ship_count as usize {
            let sh = &mut ships[idx];
            if sh.owner == town && sh.state < 0xC {
                for g in (0..24).rev() {
                    if *debt > 0 {
                        let s = sh.stock[g];
                        if s > 0 {
                            let v = host.sell_value(g as u32, t, s);
                            let n = total.wrapping_add(v as u32);
                            if n < 0xFDE8 {
                                *debt = debt.wrapping_sub(v);
                                total = n;
                                mode = 2;
                                town_stock[g] = town_stock[g].wrapping_add(s);
                                sh.stock[g] = 0;
                            }
                        }
                    }
                }
                if mode == 2 {
                    if *debt <= 0 {
                        total = total.wrapping_add(*debt as u32);
                        flag = 1;
                        self.set_i32(0, self.i32(0).wrapping_sub(*debt));
                        *debt = 0;
                    }
                    r[8..10].copy_from_slice(&sh.tag.to_le_bytes());
                    r[0xA] = mode;
                    r[0xB] = flag;
                    r[0xC..0x10].copy_from_slice(&total.to_le_bytes());
                    host.post_message(kind as i32, &r);
                    host.ship_changed(idx);
                    return;
                }
            }
            idx = sh.next as usize;
        }
        if total != 0 && self.u16(8) == 0 {
            r[0xA] = mode;
            r[0xC..0x10].copy_from_slice(&total.to_le_bytes());
            host.post_message(kind as i32, &r);
            host.notify([kind as u32, 1, 0, t, 0x15, t]);
        }
    }
}
