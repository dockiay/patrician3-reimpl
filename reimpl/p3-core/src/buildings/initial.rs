//! Muros y edificios iniciales de una ciudad nueva (C05): `town_build_initial_walls_and_buildings`
//! (0x00545F50) traza la muralla desde el punto 0 del plano, da de alta sus piezas, levanta los edificios
//! básicos, rellena los tres huecos de población y termina todas las obras.

use super::advance::{AdvanceCtx, AdvanceHost};
use super::construction::{BuildCtx, BuildHost, PlanCtx};
use super::defense::DefenseHost;
use super::tables::{BRICK, EXTRA_COST, IRON, TIMBER};
use super::walls::{DEFAULT_KINDS, WALL_SEGMENT};
use super::Site;
use crate::rng::CrtRand;
use crate::town::{off, Town};

/// Tipos que levanta la primera fase (bytes de pila de la función): los 9 primeros, o los 12 si la ciudad
/// tiene alguno de los bits `0xC0` de `Town+0x2C8`.
const FIRST_TYPES: [u8; 12] = [0x23, 0x20, 3, 0x21, 0x24, 1, 0x1F, 0x25, 0x30, 0x26, 0x22, 0x2E];
/// Palabras `0x00672A18`: cuánto cuenta cada edificio del hueco `k` (la mitad).
const SLOT_WEIGHT: [u16; 3] = [80, 140, 280];
const TOWER_RIGHT: u8 = 0x38;

/// Todo lo que la función pide fuera de la ciudad: obras, avance y mapa visual.
pub trait InitialHost: BuildHost + AdvanceHost + DefenseHost {}
impl<T: BuildHost + AdvanceHost + DefenseHost> InitialHost for T {}

/// Entorno global del que leen las llamadas internas.
pub struct InitialEnv<'a> {
    /// Número de comerciantes (`0x006DE4AA`).
    pub merchant_count: u16,
    /// Propietario (`+0xC324`) del objeto `0x006E51AC`, si existe.
    pub visual_owner: Option<u32>,
    /// Byte global `0x0067792F`.
    pub special: u8,
    /// Byte global `0x00677929`.
    pub alt_special: u8,
    /// Tabla `0x006DE4B8`.
    pub owner_class: &'a [u8; 256],
    pub merchant_gold: &'a mut i32,
    pub merchant_income: &'a mut i32,
    pub rng: &'a mut CrtRand,
    /// Dword `0x006DE4A4` y byte `0x006DE4A1`.
    pub day: u32,
    pub month: u8,
}

impl Site {
    /// Suma al almacén el coste del tipo `ty` para que `place_planned_building` pueda pagarlo.
    fn credit_cost(t: &mut Town, ty: u8) {
        let i = ty as usize;
        t.set_u32(0x50, t.u32(0x50).wrapping_add((BRICK[i] as u32).wrapping_mul(2000)));
        t.set_u32(0x30, t.u32(0x30).wrapping_add((TIMBER[i] as u32).wrapping_mul(2000)));
        t.set_u32(0x34, t.u32(0x34).wrapping_add((IRON[i] as u32).wrapping_mul(200)));
        t.set_u32(
            0x48,
            t.u32(0x48).wrapping_add((EXTRA_COST[i] as u32).wrapping_mul(2000)),
        );
    }

    fn place_now(&mut self, t: &mut Town, env: &mut InitialEnv, host: &mut dyn InitialHost, ty: u8) -> u32 {
        let mut cx = PlanCtx {
            build: BuildCtx {
                merchant_count: env.merchant_count,
                visual_owner: env.visual_owner,
                special: env.special,
                owner_class: env.owner_class,
                merchant_gold: &mut *env.merchant_gold,
                merchant_income: &mut *env.merchant_income,
                host,
            },
            rng: &mut *env.rng,
            day: env.day,
            month: env.month,
            alt_special: env.alt_special,
        };
        self.place_planned_building(t, &mut cx, ty, true)
    }

    fn finish_now(&mut self, t: &mut Town, env: &InitialEnv, host: &mut dyn InitialHost) {
        let mut cx = AdvanceCtx {
            merchant_count: env.merchant_count,
            visual_owner: env.visual_owner,
            host,
        };
        self.finish_all_pending(t, &mut cx);
    }

    /// `town_build_initial_walls_and_buildings`.
    pub fn build_initial_walls_and_buildings(
        &mut self,
        t: &mut Town,
        env: &mut InitialEnv,
        host: &mut dyn InitialHost,
    ) {
        t.set_u32(0x76C, t.u32(0x76C) | 0x10);
        if t.u8(0x7B0) < 0xFF {
            let mut kinds = DEFAULT_KINDS;
            kinds[1] = env.alt_special;
            kinds[7] = env.special;
            let trace = self.trace_wall_pieces(t, 0, &kinds);
            for entry in trace.entries.iter().copied() {
                let (row, col, ty) = (entry[0] as i32, entry[1] as i32, entry[2]);
                if ty == WALL_SEGMENT {
                    self.grid(t).stamp_defense_cells(row, col);
                    self.add_wall_segment_record(t, entry[0], entry[1], entry[3], env.visual_owner, host);
                    continue;
                }
                let (side_row, side_col) = if ty == TOWER_RIGHT {
                    (row - 1, col - 2)
                } else {
                    (row + 1, col - 2)
                };
                self.grid(t).stamp_defense_cells(side_row, side_col);
                self.grid(t)
                    .stamp_defense_cells(if ty == TOWER_RIGHT { row - 2 } else { row + 2 }, col - 4);
                self.grid(t).stamp_defense_cells(row, col);
                self.add_defense_piece_from_trace_entry(t, entry, env.visual_owner, host);
                self.mark_defense_piece_fit_flags(t, row, col, ty as u32);
            }
        }
        let first = if t.u8(0x2C8) & 0xC0 != 0 { 12 } else { 9 };
        t.set_u8(0x7B0, 0xFF);
        t.set_u8(0x7B1, 0xFF);
        for &ty in FIRST_TYPES.iter().take(first) {
            Self::credit_cost(t, ty);
            self.place_now(t, env, host, ty);
            if ty == 1 {
                self.finish_now(t, env, host);
            }
        }
        for k in 0..3usize {
            let threshold = t.i32(0x2D8 + 4 * k);
            if threshold <= 0 {
                continue;
            }
            let ty = ((k as u8 + 7).wrapping_mul(3)) as u8;
            let mut acc = 0i32;
            loop {
                Self::credit_cost(t, ty);
                acc = if self.place_now(t, env, host, ty) == 1 {
                    acc + (SLOT_WEIGHT[k] >> 1) as i32
                } else {
                    0xFFF_FFFF
                };
                if acc >= threshold {
                    break;
                }
            }
        }
        self.finish_now(t, env, host);
        t.set_u32(0x76C, t.u32(0x76C) | 0x100);
        let (paved, straight) = self.count_street_cells(t);
        t.set_u16(off::STREETS_TOTAL, paved);
        t.set_u16(off::STREETS_BUILT, straight);
    }
}
