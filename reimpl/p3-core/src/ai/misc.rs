//! Métodos pequeños de `Merchant` de 0x004F36C0 a 0x004F74F0 (C06): puntuaciones, reserva de marineros, nombres, grupos de
//! barcos y borrado de reputación. Referencia: x86 original.

use super::groups::filter_own;
use super::reputation::{add, sub, Merchant};
use crate::numeric::ftol_i32;
use crate::town::Town;

/// Tablas de nombres `0x00512A60` / `0x00512AE0` (ganchos): devuelven un puntero a texto.
pub trait NameHost {
    fn name_a(&mut self, index: u32) -> u32;
    fn name_b(&mut self, index: u32) -> u32;
}

/// Cola de mensajes `0x004D6530` (gancho): recibe el tipo (`-2`) y el registro de 16 bytes.
pub trait MessageHost {
    fn post_message(&mut self, kind: i32, record: &[u8; 16]);
}

/// Entorno y ganchos de las rutinas de rango del comerciante (`0x4F75F0`, `0x4F7510`).
pub struct RankEnv<'a> {
    pub town_count: u32,
    pub towns: &'a [Town],
    /// Palabra `0x006DE4A8` y byte `+0x2C4` de cada oficina.
    pub office_count: u16,
    pub offices: &'a [u8],
    /// Bytes `0x006DE52D`, `0x006DE52E` y palabra `0x006DE4AA`.
    pub byte_52d: u8,
    pub byte_52e: u8,
    pub word_4aa: u16,
    /// `0x006DF31C != 0`.
    pub world_ready: bool,
}

pub trait RankHost: MessageHost {
    /// `0x005326E0`: programación de misiones del alcalde; gancho pendiente.
    fn init_world(&mut self);
    /// `0x004D7AC0`: ¿hay ya un mensaje 0x5F con esos datos? (gancho).
    fn message_pending(&mut self, word: u32, level: u32) -> bool;
}

pub(crate) fn blank_record() -> [u8; 16] {
    let mut r = [0u8; 16];
    r[1] = 0x80;
    r[2] = 0xFF;
    r[3] = 0xFF;
    r[4] = 0xFF;
    r[5] = 0xFF;
    r
}

/// Barco de la lista del comerciante para la máscara de presencia.
#[derive(Clone, Copy, Debug)]
pub struct PresenceShip {
    pub next: u16,
    pub state: u16,
    pub town: u8,
}

impl Merchant {
    /// `0x004F36C0`: puntuación `+0x17` (i8) acotada a [-20, 20].
    pub fn get_clamped_score_0x17(&self) -> i32 {
        (self.bytes[0x17] as i8 as i32).clamp(-20, 20)
    }

    /// `0x004F36E0`: suma saturada a la palabra con signo `+0x16`.
    pub fn add_saturating_word_0x16(&mut self, delta: i16) {
        let cur = self.u16(0x16) as i16;
        let new = if delta > 0 {
            if (cur as i32) >= 0x7FFF - delta as i32 {
                0x7FFF
            } else {
                cur.wrapping_add(delta)
            }
        } else if (cur as i32 + 0x8000) <= -(delta as i32) {
            i16::MIN
        } else {
            cur.wrapping_add(delta)
        };
        self.set_u16(0x16, new as u16);
    }

    /// `0x004F3740`: reputación de la ciudad `index` (f32 de `+0x2FC`) en punto fijo de 16 bits.
    pub fn get_town_reputation_fixed16(&self, index: u32) -> i32 {
        ftol_i32(self.f32(0x2FC + 4 * index as usize) as f64 * 65536.0)
    }

    /// `0x004F3760`: rango en la ciudad natal (`+0x39C + ciudad`).
    pub fn get_hometown_rank(&self) -> u8 {
        self.bytes[self.bytes[0x19] as usize + 0x39C]
    }

    /// `0x004F74F0`: rango acotado a 0..4 (valores mayores de 4 dan 4).
    pub fn get_hometown_rank_tier(&self) -> u8 {
        let v = self.get_hometown_rank();
        if v < 4 {
            v
        } else {
            (v > 4) as u8 + 3
        }
    }

    /// `0x004F3770`: suma `delta` a la reputación social (`+0x120 + 12*index`).
    pub fn add_local_social_reputation(&mut self, index: u32, delta: f32) {
        let off = (index as usize * 3 + 0x48) * 4;
        let v = add(delta as f64, self.f32(off) as f64);
        self.set_f32(off, v as f32);
    }

    /// `0x004F5840`: puntero, marca (bit 15) y recuento (15 bits) del grupo `slot`.
    pub fn get_ship_group(&self, slot: u32) -> (u32, u8, u32) {
        let w = self.u16(0x60C + 2 * slot as usize);
        (
            self.i32(0x624 + 4 * slot as usize) as u32,
            (w >> 15) as u8,
            (w & 0x7FFF) as u32,
        )
    }

    /// `0x004F5900`: quita de los once grupos los barcos cuyo propietario no es `merchant`; conserva la marca de cada recuento.
    pub fn prune_foreign_ships_from_groups(
        &mut self,
        merchant: u8,
        lists: &mut [Vec<u16>; 11],
        owner: &dyn Fn(u16) -> u8,
    ) {
        struct Owner<'a>(&'a dyn Fn(u16) -> u8);
        impl super::groups::GroupHost for Owner<'_> {
            fn alloc(&mut self, _: u32) -> u32 {
                0
            }
            fn free(&mut self, _: u32) {}
            fn read_words(&self, _: u32, _: usize) -> Vec<u16> {
                vec![]
            }
            fn write_words(&mut self, _: u32, _: usize, _: &[u16]) {}
            fn ship_owner(&self, id: u16) -> u8 {
                (self.0)(id)
            }
            fn enqueue_operation(&mut self, _: &[u32; 5]) {}
        }
        let host = Owner(owner);
        for (slot, list) in lists.iter_mut().enumerate() {
            let w = self.u16(0x60C + 2 * slot);
            let mut count = w & 0x7FFF;
            filter_own(list, &mut count, merchant, &host);
            self.set_u16(0x60C + 2 * slot, count | (w & 0x8000));
        }
    }

    /// `0x004F64B0`: máscara de ciudades donde el comerciante tiene ciudad natal, oficinas o barcos (estado < 0xC).
    pub fn get_presence_town_mask(
        &self,
        town_count: u32,
        office_count: u16,
        offices: &[(u8, u16)],
        ship_count: u16,
        ships: &[PresenceShip],
    ) -> u32 {
        let home = self.bytes[0x19] as u32;
        let mut mask = if home < town_count { 1u32 << home } else { 0 };
        let mut o = self.u16(0xC);
        while o < office_count {
            let (town, next) = offices[o as usize];
            mask |= 1u32 << (town & 31);
            o = next;
            if o >= office_count {
                break;
            }
        }
        let mut s = self.u16(0xE);
        while s < ship_count {
            let ship = ships[s as usize];
            if ship.state < 0xC {
                mask |= 1u32 << (ship.town & 31);
            }
            s = ship.next;
            if s >= ship_count {
                break;
            }
        }
        mask
    }

    /// `0x004F6C10`: actualiza las reservas de marineros para cada ciudad.
    /// El incremento usa la reputación antes de limitarla y suma como byte x86.
    pub fn update_sailor_pools(&mut self, towns: &[Town]) {
        let increase = self.bytes[0x1f] >> 2;
        let reputation = self.bytes[0x1f].min(20);
        self.bytes[0x1f] = reputation;
        for (i, town) in towns.iter().enumerate() {
            let cap = town.u32(0x2e4).min(100) * u32::from(reputation) / 20;
            let pool = self.bytes[0xf0 + i].wrapping_add(increase);
            self.bytes[0xf0 + i] = u32::from(pool).min(cap) as u8;
        }
    }

    /// `0x004F6BD0`: gasta `n` marineros de la reserva de la ciudad `town` (a cero si no alcanzan).
    pub fn consume_sailor_pool(&mut self, town: u32, n: u32, town_count: u32) {
        if town >= town_count {
            return;
        }
        let o = 0xF0 + town as usize;
        let pool = self.bytes[o];
        self.bytes[o] = if n >= pool as u32 { 0 } else { pool - n as u8 };
    }

    /// `0x004F6CA0`: marineros contratables en `town`: el menor entre su reserva y `+0x2E4 - 1` de la ciudad (bytes altos del segundo se conservan).
    pub fn get_hireable_sailors(&self, town: u32, towns: &[Town]) -> u32 {
        let limit = towns[town as usize].u32(0x2E4).wrapping_sub(1);
        if (limit as i32) < 1 {
            return limit & 0xFFFF_FF00;
        }
        let pool = self.bytes[0xF0 + town as usize];
        if (pool as i32) < limit as i32 {
            (limit & 0xFFFF_FF00) | pool as u32
        } else {
            limit
        }
    }

    /// `0x004F73D0`: puntuación total de reputación (ciudades de la última a la primera; solo suman edificios y social las marcadas con 0xC0).
    pub fn calc_total_reputation_score(&self, town_count: i32, towns: &[Town]) -> i32 {
        let mut x = sub(self.f32(0x2FC) as f64, self.f32(0x11C) as f64) as f32;
        x = sub(x as f64, self.f32(0x124) as f64) as f32;
        x = sub(x as f64, self.f32(0x120) as f64) as f32;
        x = add(x as f64, x as f64) as f32;
        if town_count > 0 {
            for i in (0..town_count as usize).rev() {
                x = add(x as f64, self.f32(0x124 + 12 * i) as f64) as f32;
                if towns[i].u32(0x2C8) & 0xC0 != 0 {
                    x = add(x as f64, self.f32(0x11C + 12 * i) as f64) as f32;
                    x = add(x as f64, self.f32(0x120 + 12 * i) as f64) as f32;
                }
            }
        }
        ftol_i32(x as f64)
    }

    /// `0x004F72C0`: borra la reputación y el libro de cuentas.
    pub fn reset_reputation_and_ledger(&mut self) {
        self.set_i32(0x118, 1);
        self.bytes[0x11C..0x11C + 0x1E0].fill(0);
        self.bytes[0x2FC..0x2FC + 0xA0].fill(0);
        self.bytes[0x3C4..0x3C4 + 0xA0].fill(0);
        self.bytes[0x39C..0x39C + 0x28].fill(0);
        for o in [0x468, 0x46C, 0x470, 0x474] {
            self.set_i32(o, 0);
        }
        self.set_u16(0x478, 0);
        self.set_u16(0x47A, 0);
        for o in (0x480..=0x4D4).step_by(4) {
            self.set_i32(o, 0);
        }
        self.set_f32(0x464, 1.0);
    }

    /// `0x004F5FD0`: valores iniciales de la identidad (ciudad natal `0xFF`, oficinas y barcos `0xFFFF`, nombres de las tablas).
    pub fn init_default_fields(&mut self, host: &mut dyn NameHost) {
        self.bytes[0x19] = 0xFF;
        for o in [0xA, 0xC, 0xE] {
            self.set_u16(o, 0xFFFF);
        }
        let b = host.name_b(0);
        self.set_i32(0xE4, b as i32);
        let a = host.name_a(0);
        self.set_i32(0xE8, a as i32);
        self.set_i32(0, 0);
        self.bytes[0x38..0x40].fill(0);
        self.bytes[0xF0..0xF0 + 0x28].fill(0x32);
        self.bytes[0x24] = 0;
        self.set_i32(0xEC, 0);
        self.bytes[0x26] = 0x3F;
    }

    /// `0x004F3EB0`: constructor. Identidad por defecto, mapa de conocimiento a cero (`0x00534750` con el mapa ya cargado),
    /// listas `+0x60C`/`+0x624` a cero y límites de compra/venta iniciales de los 24 bienes (`+0x548`, copiados a `+0x5A8`).
    pub fn construct(&mut self, host: &mut dyn NameHost) {
        self.init_default_fields(host);
        self.bytes[0x4D8..0x4D8 + 0x64].fill(0);
        self.bytes[0x60C..0x60C + 0x16].fill(0);
        self.bytes[0x624..0x624 + 0x2C].fill(0);
        const SCALED: [bool; 24] = [
            false, false, false, true, true, true, true, true, true, true, true, false, true, true, false, true, false,
            false, true, false, true, true, true, true,
        ];
        for g in 0..24 {
            let p = f32::from_bits(super::daily::GOOD_VALUES[g]) as f64;
            let factor = if SCALED[g] { 200.0 } else { 2000.0 };
            let v = ftol_i32(super::reputation::mul(p, factor));
            let o = 0x548 + 4 * g;
            self.set_u16(o, v as u16);
            self.set_u16(o + 2, v as u16);
            self.set_u16(o + 0x60, v as u16);
            self.set_u16(o + 0x62, v as u16);
        }
        self.set_i32(0x608, 0);
    }

    /// `0x004F3FE0`: tamaño de guardado: `0x52C + 0xE4` más, por cada una de las diez palabras de `+0x60C`, `2 * (w & 0x7FFF)`.
    pub fn get_save_data_size(&self) -> u32 {
        let mut n = 0x52C + 0xE4u32;
        for k in 0..10 {
            n = n.wrapping_add(2 * (self.u16(0x60C + 2 * k) as u32 & 0x7FFF));
        }
        n
    }

    /// `0x004F6050`: libera el buffer extra de `+0xEC` (si no es nulo) y lo anula.
    pub fn free_extra_buffer(&mut self, free: &mut dyn FnMut(u32)) {
        let p = self.i32(0xEC) as u32;
        if p != 0 {
            free(p);
        }
        self.set_i32(0xEC, 0);
    }

    /// `0x004F6420`: invierte el bit 7 de `+0x1C` según `+0x1A` y refresca el puntero del nombre de pila (`+0xE8`).
    pub fn refresh_first_name_ptr(&mut self, host: &mut dyn NameHost) {
        let flag = self.bytes[0x1A];
        self.bytes[0x1C] = if flag != 0 {
            self.bytes[0x1C] & 0x7F
        } else {
            self.bytes[0x1C] | 0x80
        };
        let p = host.name_a(self.bytes[0x1C] as u32);
        self.set_i32(0xE8, p as i32);
    }

    /// `0x004F6450`: refresca el puntero del apellido (`+0xE4`) desde `+0x1D`.
    pub fn refresh_family_name_ptr(&mut self, host: &mut dyn NameHost) {
        let p = host.name_b(self.bytes[0x1D] as u32);
        self.set_i32(0xE4, p as i32);
    }

    /// `0x004F6470`: copia el bit 0 de `+0x1A` al bit 7 de `+0x30` y consulta la tabla de nombres (resultado descartado).
    pub fn update_spouse_name_gender_bit(&mut self, host: &mut dyn NameHost) {
        let b = self.bytes[0x30];
        self.bytes[0x30] = if self.bytes[0x1A] & 1 != 0 { b | 0x80 } else { b & 0x7F };
        let _ = host.name_a(self.bytes[0x30] as u32);
    }

    /// `0x004F68D0` (`ret 0x14`): identidad inicial y nacimiento. `a1` son los días del año de nacimiento (módulo 365), `a2` la
    /// edad (bajo 16 bits), `a4` indica edad explícita (byte bajo) y `a5` el sexo (byte bajo); `year` es la palabra `0x006DE4A2`.
    pub fn init_identity_and_birth(&mut self, a1: u32, a2: u32, a4: u32, a5: u32, year: u16) {
        let age = if a4 as u8 != 0 {
            year.wrapping_sub(a2 as u16)
        } else {
            year.wrapping_sub((a2 & 0x1F) as u16).wrapping_sub(0x19)
        } as u32;
        let esi = age.wrapping_mul(73).wrapping_mul(5).wrapping_add(a1 % 365) << 8;
        self.bytes[0x18] = self.bytes[0x19];
        self.set_i32(0x10, esi as i32);
        self.bytes[0x1A] = (a5 as u8 != 0) as u8;
        self.set_i32(0x28, -1);
        self.set_i32(0x2C, -1);
        for o in [0x32, 0x33, 0x36, 0x25, 0x37, 0x1C, 0x1D, 0x1E, 0x1B, 0x20, 0x21, 0x24] {
            self.bytes[o] = 0;
        }
        self.set_u16(0x14, 0);
        self.set_u16(0x16, 0);
        self.set_i32(4, 0);
        for o in [0x27, 0x30, 0x31, 0xE0, 0xE1] {
            self.bytes[o] = 0xFF;
        }
        self.set_u16(0x34, 0xFFFF);
        self.bytes[0x1F] = 0xF;
        self.set_u16(0x22, 1);
        self.set_u16(0xE2, 0xFFFF);
    }

    /// `0x004F8560` (`ret 0xC`; argumentos `a1`, `town`, `cost`): compra el conocimiento del mapa de la ciudad `town` por `cost`
    /// monedas si hay oro, la ciudad no está ya comprada (`+0x468`) y `byte_52a` es cero; anota el gasto (`+0x4BC`), publica el mensaje
    /// 0x0C, funde el mapa propio en el global y suma el factor base (`+0x464`) a la reputación social de la ciudad.
    pub fn buy_town_map_knowledge(
        &mut self,
        global: &mut [u8],
        town_count: u32,
        byte_52a: u8,
        a1: u32,
        town: u32,
        cost: i32,
        host: &mut dyn MessageHost,
    ) {
        if town >= town_count || self.i32(0) < cost {
            return;
        }
        let bit = 1u32 << (town & 31);
        let flags = self.i32(0x468) as u32;
        if bit & flags != 0 || byte_52a != 0 {
            return;
        }
        self.set_i32(0x4BC, self.i32(0x4BC).wrapping_add(cost));
        self.set_i32(0x468, (flags | bit) as i32);
        self.set_i32(0, self.i32(0).wrapping_sub(cost));
        let mut record = [0u8; 16];
        record[1] = 0x80;
        record[2] = 0xFF;
        record[3] = 0xFF;
        record[4] = 0x0C;
        record[5] = town as u8;
        record[8..10].copy_from_slice(&(a1 as u16).to_le_bytes());
        record[10..12].copy_from_slice(&(self.get_merchant_epithet_index() as u16).to_le_bytes());
        record[12..14].copy_from_slice(&(self.get_hometown_rank() as u16).to_le_bytes());
        host.post_message(-2, &record);
        merge_map_objects(global, &self.bytes[0x4D8..0x4D8 + 0x64]);
        let off = (town as usize * 3 + 0x48) * 4;
        let v = add(self.f32(0x464) as f64, self.f32(off) as f64);
        self.set_f32(off, v as f32);
    }

    /// `0x004F7510` (`ret 0xC`; argumentos `kind`, `town`, `level`): avisa de que al comerciante le falta riqueza para subir de rango en su
    /// ciudad natal (mensajes 0x5E y 0x5F) salvo que ya haya uno pendiente.
    pub fn notify_rank_wealth_shortfall(&self, kind: u32, town: u32, level: u32, host: &mut dyn RankHost) {
        if town != self.bytes[0x19] as u32 || self.u16(8) != 0 || host.message_pending(self.u16(0xA) as u32, level) {
            return;
        }
        let mut r = blank_record();
        r[5] = self.bytes[0x19];
        r[8..10].copy_from_slice(&(kind as u16).to_le_bytes());
        r[10..12].copy_from_slice(&(level as u16).to_le_bytes());
        r[12..14].copy_from_slice(&(self.bytes[0x1C] as u16).to_le_bytes());
        r[14..16].copy_from_slice(&(self.bytes[0x1D] as u16).to_le_bytes());
        r[4] = 0x5E;
        host.post_message(kind as i32, &r);
        r[4] = 0x5F;
        host.post_message(kind as i32, &r);
    }

    /// `0x004F75F0` (`ret 4`; argumento `town`): actualiza el rango del comerciante en la ciudad (`+0x39C`): todas a 7 si el byte
    /// 0x6DE52D coincide con el de su oficina, 6 y aviso de fama si coincide el de la ciudad (`+0x6F1`) y, si no, sube de
    /// rango según reputación (`+0x2FC`) e ingresos (`+0x46C`).
    pub fn update_rank_in_town(&mut self, env: &RankEnv, town: u32, host: &mut dyn RankHost) {
        let first = self.u16(0xC);
        let flag = if first < env.office_count {
            env.offices[first as usize]
        } else {
            0
        };
        if env.byte_52d == flag {
            for i in 0..env.town_count as usize {
                self.bytes[0x39C + i] = 7;
            }
            return;
        }
        let t = town as usize;
        if env.towns[t].u8(0x6F1) == flag {
            self.bytes[0x39C + t] = 6;
            if self.bytes[0x37] != 0 {
                return;
            }
            let score = self.calc_total_reputation_score(env.town_count as i32, env.towns);
            if 120 > (score as i16) as i32 {
                return;
            }
            self.bytes[0x37] = 1;
            if !env.world_ready {
                host.init_world();
            }
            if self.u16(8) != 0 {
                return;
            }
            let mut r = blank_record();
            r[4] = 0x6D;
            r[5] = env.byte_52e;
            // el byte +7 queda sin inicializar en el original (basura de pila): aquí 0
            if env.byte_52d as u16 >= env.word_4aa {
                r[8] = 4;
                let idx = (!(env.byte_52d as u32)) & 3;
                let town2 = &env.towns[env.byte_52e as usize];
                r[9] = town2.u8(0x6D4 + idx as usize);
                r[10] = town2.u8(0x6D8 + idx as usize);
            }
            r[11] = self.get_merchant_epithet_index();
            r[12] = self.bytes[0x1C];
            r[13] = self.bytes[0x1D];
            host.post_message(flag as i32, &r);
            return;
        }
        let bits = self.i32(0x468) as u32;
        let has_bit = bits & (1u32 << (town & 31)) != 0;
        let rank = self.bytes[0x39C + t];
        let rep = self.f32(0x2FC + 4 * t);
        let income = self.i32(0x46C);
        // (reputación mínima, ingresos que exigen la subida, ¿requiere el bit de la ciudad?)
        let steps: [(f32, i32, bool); 5] = [
            (7.0, 100_000, false),
            (12.0, 200_000, false),
            (20.0, 300_000, true),
            (40.0, 500_000, true),
            (60.0, 900_000, true),
        ];
        match rank {
            0..=4 => {
                let (min_rep, min_income, needs_bit) = steps[rank as usize];
                if (needs_bit && !has_bit) || !(rep >= min_rep) {
                    return;
                }
                if income < min_income {
                    self.notify_rank_wealth_shortfall(flag as u32, town, rank as u32 + 1, host);
                    return;
                }
                self.bytes[0x39C + t] = rank + 1;
                if rank == 0 {
                    if town == self.bytes[0x19] as u32 || self.u16(8) != 0 {
                        return;
                    }
                    self.bytes[0x24] = self.bytes[0x24].wrapping_add(1);
                    let mut r = blank_record();
                    r[4] = 0x82;
                    r[5] = town as u8;
                    r[8..10].copy_from_slice(&(self.get_merchant_epithet_index() as u16).to_le_bytes());
                    r[10..12].copy_from_slice(&(self.bytes[self.bytes[0x19] as usize + 0x39C] as u16).to_le_bytes());
                    r[12..14].copy_from_slice(&(self.bytes[0x1C] as u16).to_le_bytes());
                    r[14..16].copy_from_slice(&(self.bytes[0x1D] as u16).to_le_bytes());
                    host.post_message(flag as i32, &r);
                }
            }
            6 => self.bytes[0x39C + t] = 5,
            7 => self.bytes[0x39C + t] = if has_bit { 5 } else { 2 },
            _ => {}
        }
    }

    /// `0x004F93C0`: marca el punto del mapa más cercano a `(x, y)` como visto en el objeto propio (`+0x4D8`) y, si `+0x468` no
    /// es cero, en el global `0x006DF278`. Devuelve el resultado de la última marca.
    pub fn mark_map_point_seen(
        &mut self,
        global: &mut [u8],
        map: &MapPoints,
        x: i16,
        y: i16,
        flag: bool,
        dirty: &mut bool,
    ) -> bool {
        if self.i32(0x468) != 0 {
            mark_point_seen(global, map, x, y, flag, dirty);
        }
        mark_point_seen(&mut self.bytes[0x4D8..0x4D8 + 0x64], map, x, y, flag, dirty)
    }

    /// `0x004F9400`: pone el bit de descubrimiento `bit` en el objeto propio y, si `+0x468` no es cero, en el global.
    pub fn set_map_discovery_bit(&mut self, global: &mut [u8], bit: u32, dirty_target: bool, dirty: &mut bool) {
        if self.i32(0x468) != 0 {
            set_discovery_bit(global, bit, dirty_target, dirty);
        }
        set_discovery_bit(&mut self.bytes[0x4D8..0x4D8 + 0x64], bit, dirty_target, dirty);
    }
}

/// Puntos del mapa de `0x006DF2E0` (pares de palabras sin signo) y si `0x006E5558` apunta a un objeto (marca de cambio).
pub struct MapPoints<'a> {
    pub points: &'a [(u16, u16)],
    pub dirty_target: bool,
}

/// `0x00534960`: pone el bit `bit & 31` de la palabra `+0x60` y marca el cambio.
pub fn set_discovery_bit(obj: &mut [u8], bit: u32, dirty_target: bool, dirty: &mut bool) {
    let v = u32::from_le_bytes(obj[0x60..0x64].try_into().unwrap()) | (1u32 << (bit & 31));
    obj[0x60..0x64].copy_from_slice(&v.to_le_bytes());
    *dirty |= dirty_target;
}

/// `0x00534990`: marca el punto más cercano (distancia al cuadrado con signo; el primero en empate) en el campo de 384 bits
/// (y en su copia `+0x30` si `flag`). Devuelve si el índice es menor que 100. Requiere al menos un punto.
pub fn mark_point_seen(obj: &mut [u8], map: &MapPoints, x: i16, y: i16, flag: bool, dirty: &mut bool) -> bool {
    assert!(!map.points.is_empty());
    let (mut best, mut idx) = (0x7FFF_FFFFi32, 0usize);
    for (i, &(px, py)) in map.points.iter().enumerate() {
        let dx = x as i32 - px as i32;
        let dy = y as i32 - py as i32;
        let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
        if d < best {
            best = d;
            idx = i;
        }
    }
    let (word, mask) = (idx / 32 * 4, 1u32 << (idx % 32));
    let v = u32::from_le_bytes(obj[word..word + 4].try_into().unwrap()) | mask;
    obj[word..word + 4].copy_from_slice(&v.to_le_bytes());
    if flag {
        let v = u32::from_le_bytes(obj[word + 0x30..word + 0x34].try_into().unwrap()) | mask;
        obj[word + 0x30..word + 0x34].copy_from_slice(&v.to_le_bytes());
    }
    *dirty |= map.dirty_target;
    idx < 0x64
}

/// `0x00534920`: une el mapa `src` al `dst` (OR de la palabra `+0x60` y de los doce pares de palabras de `+0` y `+0x30`).
pub fn merge_map_objects(dst: &mut [u8], src: &[u8]) {
    for o in std::iter::once(0x60)
        .chain((0..12).map(|i| i * 4))
        .chain((0..12).map(|i| 0x30 + i * 4))
    {
        let a = u32::from_le_bytes(dst[o..o + 4].try_into().unwrap())
            | u32::from_le_bytes(src[o..o + 4].try_into().unwrap());
        dst[o..o + 4].copy_from_slice(&a.to_le_bytes());
    }
}

/// `0x004F6080`: tamaño en memoria del registro de un capitán/candidato.
pub const MERCHANT_RECORD_SIZE: u32 = 0x10C;
/// `0x004F6CE0`: tamaño de la parte fija del registro de guardado.
pub const MERCHANT_SAVE_RECORD_SIZE: u32 = 0x52C;
