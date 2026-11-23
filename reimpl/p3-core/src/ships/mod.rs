//! Barcos en el mar (C07): movimiento, orden espacial por coordenada x y capitanes.
//!
//! Los barcos se representan por sus 0x180 bytes originales (`crate::shipyard::SHIP_SIZE`), como en el
//! astillero y la IA. Cada módulo cita las direcciones que reproduce.

pub mod advance;
pub mod ai_arm;
pub mod ai_cargo;
pub mod ai_depart;
pub mod ai_dock;
pub mod ai_docked;
pub mod ai_trade;
pub mod battle;
pub mod battle_load;
pub mod battle_view;
pub mod boarding;
pub mod captains;
pub mod capture;
pub mod capture_world;
pub mod cargo;
pub mod collide;
pub mod convoy;
pub mod convoy_absorb;
pub mod convoy_dock;
pub mod convoy_drop;
pub mod convoy_join;
pub mod convoy_motion;
pub mod convoy_nudge;
pub mod convoy_plan;
pub mod convoy_report;
pub mod convoy_unlink;
pub mod departure;
pub mod docking;
pub mod finish;
pub mod finish_report;
pub mod harbor_legs;
pub mod harbor_list;
pub mod harbor_view;
pub mod hull;
pub mod lost_letter;
pub mod motion;
pub mod order;
pub mod pirate_mission_attack;
pub mod projectile;
pub mod pursuit;
pub mod remove;
pub mod return_convoy;
pub mod route;
pub mod sea_update;
pub mod siege;
pub mod spotting;
pub mod siege_weapons;
pub mod spatial;
pub mod steer;
pub mod step;
pub mod tick;
pub mod town_attack_finish;
pub mod town_attack_frame;
pub mod town_attack_mask;
pub mod town_attack_start;
pub mod volley;
pub mod weapon_damage;
pub mod weapons;
pub mod weather;
pub mod wind;

/// División entera que en x86 produce la excepción #DE (divisor cero o cociente fuera de rango).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DivideError;

pub(crate) fn r16(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}

pub(crate) fn w16(b: &mut [u8], o: usize, v: u16) {
    b[o..o + 2].copy_from_slice(&v.to_le_bytes());
}

pub(crate) fn r32(b: &[u8], o: usize) -> i32 {
    i32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

pub(crate) fn w32(b: &mut [u8], o: usize, v: i32) {
    b[o..o + 4].copy_from_slice(&v.to_le_bytes());
}
pub mod factory;

pub mod prey;

pub mod pirate_engage;

pub mod convoy_expedition;
pub mod expedition_finish;
