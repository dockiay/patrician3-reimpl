//! Petición de obra de la muralla (C05): `town_fill_wall_plan_build_request` (0x005226C0) traza la muralla
//! del punto del plano `tipo - 0x36`, calcula materiales, coste, total y plazo, y entrega a la clase 22 la
//! lista de piezas «fantasma» que se dibujan mientras el jugador decide.

use super::evaluate::BuildRequest;
use super::tables::{BASE_COST, BRICK, EXTRA_COST, IRON, RECORD_BYTE_6, TIMBER};
use super::walls::WALL_SEGMENT;
use super::Site;
use crate::numeric::ftol_i32;
use crate::town::Town;

/// Bytes de las tablas por tipo en las entradas `0x36` y `0x37` (`0x00672AFA`, `0x00672A82`, `0x00672ABE`,
/// `0x00672B36`, `0x00672BAE` y `0x00672BEA`, con sus vecinos `+1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallRequestTables {
    pub brick: [u8; 2],
    pub timber: [u8; 2],
    pub iron: [u8; 2],
    pub extra: u8,
    pub base_cost: [u8; 2],
    pub byte6: [u8; 2],
}

impl Default for WallRequestTables {
    fn default() -> Self {
        let at = |t: &[u8; 64]| [t[0x36], t[0x37]];
        WallRequestTables {
            brick: at(&BRICK),
            timber: at(&TIMBER),
            iron: at(&IRON),
            extra: EXTRA_COST[0x36],
            base_cost: at(&BASE_COST),
            byte6: at(&RECORD_BYTE_6),
        }
    }
}

/// Lo que pide la función fuera de la ciudad.
pub trait WallRequestHost {
    /// `get_ware_spot_price` (0x0052E700) con (propietario, mercancía, 1): el precio como `f32`.
    fn spot_price(&mut self, owner: u32, ware: u32, flag: u32) -> f32;
    /// `operator new` (0x0064F7B9) del búfer de piezas: `false` si no hay memoria.
    fn alloc_ghost(&mut self, bytes: u32) -> bool;
    /// `class22_find_shared_map_object_id` (0x005928A0) con (0x2E, byte 7 de la pieza, 1).
    fn find_shared_map_object(&mut self, class: u32, param: u32, flag: u32) -> u16;
    /// `town_object_type2f_fill_part_table` (0x00582770): rellena la entrada de 36 bytes cuyos 12 primeros
    /// ya están escritos.
    fn fill_part_table(&mut self, entry: &mut [u8]);
    /// `class22_set_ghost_object_list` (0x00593800): (unidades de 12 bytes, búfer o nada si falló la reserva).
    fn set_ghost_object_list(&mut self, units: u32, data: Option<&[u8]>);
}

impl Site {
    /// `town_fill_wall_plan_build_request`. La petición `r` trae el tipo (`0x36..=0x39`: el punto del plano es
    /// `tipo - 0x36`); se reescriben todos sus campos menos el motivo de bloqueo. El tipo vuelve a `0x36`.
    pub fn fill_wall_plan_build_request(
        &mut self,
        t: &Town,
        r: &mut BuildRequest,
        kinds: &[u8; 8],
        k: &WallRequestTables,
        host: &mut dyn WallRequestHost,
    ) {
        let trace = self.trace_wall_pieces(t, (r.ty as u32).wrapping_sub(0x36), kinds);
        let count = trace.entries.len() as i32;
        let towers = trace.towers as i32;
        let per = |a: u8, b: u8| (a as i32).wrapping_mul(count).wrapping_add((b as i32) * 3);
        r.need = [
            per(k.brick[0], k.brick[1]),
            per(k.timber[0], k.timber[1]),
            per(k.iron[0], k.iron[1]),
            per(k.extra, k.iron[1]),
        ];
        r.have = [0; 4];
        r.short = r.need;
        let owner = t.u8(0x2C0) as u32;
        let mut cost = 0i32;
        for (need, ware, scale) in [
            (r.need[0], 0x13, 2000),
            (r.need[1], 0x13, 2000),
            (r.need[2], 0xC, 200),
            (r.need[3], 0x11, 2000),
        ] {
            let price = host.spot_price(owner, ware, 1);
            let units = need.wrapping_mul(scale) as f32;
            cost = cost.wrapping_add(ftol_i32(price as f64 * units as f64));
        }
        r.cost = cost;
        r.total = (k.base_cost[0] as i32)
            .wrapping_mul(count)
            .wrapping_mul(100)
            .wrapping_add((k.base_cost[1] as i32) * 300)
            .wrapping_add(cost);
        r.time = (k.byte6[0] as i32)
            .wrapping_mul(count)
            .wrapping_add((k.byte6[1] as i32) * 3)
            / 30;
        r.ty = WALL_SEGMENT as u16;
        r.merchant = t.u8(0x6F1) as u16;
        (r.affordable, r.materials, r.office) = (1, 1, 1);

        let units = count.wrapping_add(towers.wrapping_mul(2)) as u32;
        if !host.alloc_ghost(units.wrapping_mul(12)) {
            host.set_ghost_object_list(0, None);
            return;
        }
        let mut buf = vec![0u8; units as usize * 12];
        let mut pos = 0usize;
        for e in &trace.entries {
            buf[pos..pos + 4].copy_from_slice(&(e[0] as u32).to_le_bytes());
            buf[pos + 4..pos + 8].copy_from_slice(&(e[1] as u32).to_le_bytes());
            if e[2] == WALL_SEGMENT {
                let id = host.find_shared_map_object(0x2E, e[3] as u32, 1);
                buf[pos + 8..pos + 10].copy_from_slice(&id.to_le_bytes());
                pos += 12;
            } else {
                buf[pos + 8..pos + 10].copy_from_slice(&((e[3] as u16) << 8).to_le_bytes());
                host.fill_part_table(&mut buf[pos..pos + 36]);
                pos += 36;
            }
        }
        host.set_ghost_object_list(units, Some(&buf));
    }
}
