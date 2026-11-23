//! Construcción de una calle (C05): `op_build_road` (0x005244F0).

use super::construction::{BuildHost, Hook};
use super::evaluate::{BuildRequest, EvalEnv, EvalHost, EvalShip};
use super::housing::HousingHost;
use super::tables::RECORD_BYTE_6;
use super::Site;
use crate::town::{off, Town};

/// Lo que lee y escribe `op_build_road` además de la ciudad.
pub struct RoadEnv<'a> {
    /// Número de comerciantes (`0x006DE4AA`).
    pub merchant_count: u16,
    pub ship_count: u16,
    pub ship_head: u16,
    pub mayor: u8,
    pub ships: &'a [EvalShip],
    /// Dinero (`+0`) e ingresos (`+0x4C0`) del comerciante que devuelve `get_merchant`.
    pub merchant_gold: &'a mut i32,
    pub merchant_income: &'a mut i32,
    /// Propietario (`+0xC324`) del objeto `0x006E51AC`, si existe.
    pub visual_owner: Option<u32>,
}

/// Reenvía los avisos al mapa visual como llamadas registradas.
struct MapAdapter<'a, H: BuildHost>(&'a mut H);

impl<H: BuildHost> HousingHost for MapAdapter<'_, H> {
    fn add_map_object(&mut self, row: u32, col: u32, param: u32, packed: u32, owner: u32) {
        self.0.call(Hook::AddMapObject, &[row, col, param, packed, owner]);
    }
}

/// Mercancías 19, 11, 12 y 17; desplazamiento de su existencia en la ciudad y unidad de la petición.
const ROAD_WARES: [(u32, usize, i32); 4] = [(19, 0x50, 2000), (11, 0x30, 2000), (12, 0x34, 200), (17, 0x48, 2000)];

impl Site {
    /// `op_build_road`: valora una calle (tipo 2) para el comerciante `merchant`; si se puede, cobra al
    /// comerciante, retira materiales de la oficina y de la ciudad, añade un registro de calle a la cola
    /// de obras y completa las celdas de calle junto al centro. Con un comerciante inexistente se valora
    /// con el comerciante 0xFF y no se cobra nada.
    pub fn op_build_road<H: EvalHost + BuildHost>(
        &mut self,
        t: &mut Town,
        merchant: u32,
        env: &mut RoadEnv,
        host: &mut H,
    ) {
        let known = merchant < env.merchant_count as u32;
        let mut req = BuildRequest {
            ty: 2,
            merchant: if known { merchant as u16 } else { 0xFF },
            ..Default::default()
        };
        let possible = {
            let mut ev = EvalEnv {
                merchant_count: env.merchant_count,
                ship_count: env.ship_count,
                ship_head: env.ship_head,
                merchant_gold: *env.merchant_gold,
                mayor: env.mayor,
                ships: env.ships,
                host: &mut *host,
            };
            Site::evaluate_build_request(t, &mut req, &mut ev)
        };
        if !possible {
            return;
        }
        if known {
            let town = t.u8(off::TOWN_INDEX) as u32;
            let arg = req.merchant as u32 | (req.affordable as u32) << 16 | (req.materials as u32) << 24;
            if host.call(Hook::FindOrCreateOffice, &[arg, town, 0]) == 0 {
                return;
            }
            let amounts = [
                req.need[0].wrapping_mul(2000),
                req.need[1].wrapping_mul(2000),
                req.need[2].wrapping_mul(200),
                req.need[3].wrapping_mul(2000),
            ];
            host.call(Hook::GetMerchant, &[req.merchant as u32]);
            host.call(Hook::ReputationHospital, &[town, 0]);
            *env.merchant_gold = env.merchant_gold.wrapping_sub(req.total);
            *env.merchant_income = env.merchant_income.wrapping_add(req.total);
            for (k, (ware, _, _)) in ROAD_WARES.iter().enumerate() {
                if amounts[k] > 0 {
                    host.call(Hook::TakeWaresFromOffice, &[*ware, amounts[k] as u32]);
                }
            }
        }
        for (k, (_, offset, unit)) in ROAD_WARES.iter().enumerate() {
            let amount = req.short[k].wrapping_mul(*unit);
            if amount > 0 {
                let left = t.i32(*offset).wrapping_sub(amount);
                t.set_i32(*offset, left.max(0));
            }
        }
        let notify = env.visual_owner == Some(t.u8(off::TOWN_INDEX) as u32);
        let i = self.alloc_building_record(t) as usize;
        let r = &mut self.records[i * 8..i * 8 + 8];
        r.copy_from_slice(&[0xFF, 0xFF, 0xFF, 2, 0xFF, 0xFF, RECORD_BYTE_6[2], 0xFF]);
        self.enqueue_construction_record(t, i as u16);
        let n = self.complete_street_cells_near_center(t, notify, &mut MapAdapter(host));
        t.add_u16(off::STREETS_BUILT, n as u16);
        let built = t.u16(off::STREETS_BUILT);
        if built > t.u16(off::STREETS_TOTAL) {
            t.set_u16(off::STREETS_TOTAL, built);
        }
    }
}
