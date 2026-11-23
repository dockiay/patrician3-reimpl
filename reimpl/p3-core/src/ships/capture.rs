//! Captura y botín tras un combate cuerpo a cuerpo (0x0060C1C9). El texto, los mensajes y la interfaz son efectos del
//! anfitrión (`CaptureHost`); el estado de barcos, listas del combate, comerciantes y cargas se reproduce byte a byte.
//! Fixture: `oracle/x86_battle_capture.py`.

use super::boarding::{Boarding, Captains};
use super::cargo::recompute_load;
use crate::economy::prices::{BASE_PRICE, SMALL_LOT_WARES};
use crate::numeric::ftol_i32;

const MERCHANT_SIZE: usize = 1616;
/// Palabra de reputación por botín (`word [0x672DE2]`, dato constante del ejecutable).
const REPUTATION_STEP: i16 = 65;

pub trait CaptureHost {
    /// Texto de aviso: se anota el global del que sale la cadena de formato (o 0x6A16F8, la copia sin formato).
    fn text(&mut self, format: u32);
    /// 0x0041D9B0: mostrar el aviso de interfaz.
    fn ui_message(&mut self);
    /// 0x004D6530: mensaje de noticias (`target` -2 = todos). Solo los bytes iniciales (0..0x10 y 0x11) son datos.
    fn news(&mut self, target: i32, message: &[u8; 0x18]);
    /// 0x00535760 con el suceso 0x91: cambio de dueño del barco.
    fn ownership_event(&mut self, first_world: u32, second_world: u32);
    /// Synchronous operation 0x91. News above this call observes the old owner
    /// and captain; the local-list updates below it observe the changed world.
    /// The default keeps the original effect-only API used by isolated ports.
    fn ownership_event_with_state(
        &mut self,
        first_world: u32,
        second_world: u32,
        _first: &mut [u8],
        _second: &mut [u8],
        _merchants: &mut [u8],
    ) {
        self.ownership_event(first_world, second_world);
    }
    /// 0x004F9430: valoración del comerciante.
    fn rating(&mut self, merchant: &[u8]) -> u8;
    /// 0x00515F70 sobre el objeto `0x6DD7AC[index]`.
    fn notify_object(&mut self, index: u8);
    /// Runs the band effect after the cargo/reward changes. A connected host
    /// must see these current images, not the snapshot from before boarding.
    fn notify_object_with_state(
        &mut self,
        index: u8,
        _first_world: u32,
        _second_world: u32,
        _first: &mut [u8],
        _second: &mut [u8],
        _merchants: &mut [u8],
    ) {
        self.notify_object(index);
    }
}

/// Barco del mundo señalado por la flota (campos que lee el original).
#[derive(Clone, Copy)]
pub struct FleetShip {
    pub health: u32,
    pub captain: u16,
    pub owner: u8,
    pub kind_word: u16,
}

pub struct CaptureEnv<'a> {
    pub captains: Captains<'a>,
    pub player_slot: u32,
    pub fleet_threshold: u32,
    pub merchant_limit: u16,
    pub fleet_count: u16,
    /// Tabla de flotas (0x6DD7A8), 60 bytes por entrada.
    pub fleets: &'a [u8],
    pub fleet_ship: &'a dyn Fn(u16) -> Option<FleetShip>,
    pub objects: [bool; 5],
    pub virtual_10: bool,
    pub battle_address: u32,
}

pub struct CaptureData<'a> {
    pub battle: &'a mut [u8],
    pub prev: &'a mut [u16],
    pub next: &'a mut [u16],
    pub first_ship: &'a mut [u8],
    pub second_ship: &'a mut [u8],
    pub first_world: &'a mut [u8],
    pub second_world: &'a mut [u8],
    pub merchants: &'a mut [u8],
    pub ui_flag: &'a mut bool,
}

fn rd16(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn rd32(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn wr16(b: &mut [u8], o: usize, v: u16) {
    b[o..o + 2].copy_from_slice(&v.to_le_bytes());
}
fn wr32(b: &mut [u8], o: usize, v: u32) {
    b[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

static NO_MERCHANT: [u8; MERCHANT_SIZE] = [0; MERCHANT_SIZE];

/// Registro del comerciante; fuera de la tabla el original lee memoria ajena y aquí se lee como ceros.
fn merchant(merchants: &[u8], owner: u8) -> &[u8] {
    let start = usize::from(owner) * MERCHANT_SIZE;
    if start + MERCHANT_SIZE <= merchants.len() {
        &merchants[start..start + MERCHANT_SIZE]
    } else {
        &NO_MERCHANT
    }
}

/// Cabecera que deja el constructor 0x0051A850.
fn fresh_message() -> [u8; 0x18] {
    let mut m = [0u8; 0x18];
    m[1] = 0x80;
    m[2] = 0xFF;
    m[3] = 0xFF;
    m[4] = 0xFF;
    m[5] = 0xFF;
    m
}

/// Cadena de la flota (0x60C440..): ¿hay un barco con vida y capitán en la flota del bando?
fn fleet_valid(data: &CaptureData, env: &CaptureEnv, map: u8) -> bool {
    let v = if map == 0 {
        rd16(data.battle, 0x65E)
    } else {
        rd16(data.battle, 0x660)
    };
    let entry = usize::from(v) * 60 + 0x10;
    let world = rd16(env.fleets, entry);
    if world >= env.fleet_count {
        return false;
    }
    match (env.fleet_ship)(world) {
        Some(ship) => ship.health != 0 && ship.captain < env.captains.count,
        None => false,
    }
}

pub fn capture_ships<H: CaptureHost>(first: u16, second: u16, data: CaptureData, env: &CaptureEnv, host: &mut H) {
    let CaptureData {
        battle,
        prev,
        next,
        first_ship,
        second_ship,
        first_world,
        second_world,
        merchants,
        ui_flag,
    } = data;
    let first_world_index = u32::from_le_bytes(first_ship[4..8].try_into().unwrap());
    let second_world_index = u32::from_le_bytes(second_ship[4..8].try_into().unwrap());
    let capture = {
        let (fw, sw): (&[u8], &[u8]) = (first_world, second_world);
        let lookup = |i: u32| -> &[u8] {
            if i == first_world_index {
                fw
            } else {
                sw
            }
        };
        let view = Boarding {
            battle,
            virtual_10: env.virtual_10,
            world: &lookup,
            captains: env.captains,
        };
        view.can_capture(first_ship, second_ship, true)
    };
    // noticias de captura de un barco sin dueño
    if second_world[0] == 0xFF && capture {
        let captain = rd16(second_world, 0x42);
        if captain < env.captains.count {
            let mut message = fresh_message();
            let owner = merchant(merchants, first_world[0]);
            let record = &env.captains.table[usize::from(captain) * 16..][..16];
            message[4] = 0x2A;
            message[5] = owner[0x19];
            message[8] = owner[0x1A];
            message[9] = host.rating(owner);
            message[10] = owner[usize::from(owner[0x19]) + 0x39C];
            message[11] = owner[0x1C];
            message[12] = owner[0x1D];
            message[13] = record[2];
            message[14] = record[3];
            host.news(-2, &message);
            message[4] = 0x54;
            message[5] = 0;
            let word = (u16::from(record[2]) << 8).wrapping_add(u16::from(record[3]));
            wr16(&mut message, 8, word);
            wr16(&mut message, 10, captain);
            wr16(&mut message, 12, second_world_index as u16);
            host.news(i32::from(first_world[0]), &message);
        }
    }
    first_ship[0x148] = 0;
    second_ship[0x148] = 0;
    wr16(battle, 0xAC6, second);
    battle[0xAC4] = 2;
    // aviso al jugador de esta máquina
    let side_of = |ship: &[u8]| usize::from(ship[0x13D]);
    let participant = |battle: &[u8], side: usize| rd16(battle, 0x35A + 2 * side);
    let slot = env.player_slot;
    let first_part = u32::from(participant(battle, side_of(first_ship)));
    let second_part = u32::from(participant(battle, side_of(second_ship)));
    let mine_is_first = slot == first_part;
    if mine_is_first || slot == second_part {
        let (me_side, other_side, other_world_owner, other_part) = if mine_is_first {
            (side_of(first_ship), side_of(second_ship), second_world[0], second_part)
        } else {
            (side_of(second_ship), side_of(first_ship), first_world[0], first_part)
        };
        let map = battle[0x559 + other_side];
        let mut done = false;
        if battle[0xAD2] & (1u8 << map) != 0 {
            if other_part == 0xFF
                && fleet_valid(
                    &CaptureData {
                        battle,
                        prev,
                        next,
                        first_ship,
                        second_ship,
                        first_world,
                        second_world,
                        merchants,
                        ui_flag,
                    },
                    env,
                    map,
                )
            {
                host.text(0x6A16F0);
                done = true;
            }
            if !done {
                if env.virtual_10 && u32::from(battle[0x1E98]) >= env.fleet_threshold {
                    host.text(0x6A16F0);
                } else {
                    host.text(0x6A16F8);
                }
            }
        } else {
            let _ = merchant(merchants, other_world_owner)[0x1A];
            host.text(0x6A16E4);
        }
        host.text(match (mine_is_first, capture) {
            (true, true) => 0x6A1D78,
            (true, false) => 0x6AA178,
            (false, true) => 0x6A1D7C,
            (false, false) => 0x6AA17C,
        });
        if (second & 0xFF) < 0xFF && battle[0x5D + me_side] == 0 {
            let base = 0x12EC;
            wr32(battle, base, 0);
            battle[base + 4] = 0;
            wr16(battle, base + 6, 0);
            wr32(battle, base + 8, 0);
            battle[base + 0xC] = 0xFF;
            wr32(battle, base + 0x10, 0x3FB0_0000);
            battle[base + 0x14] = 1;
            wr16(battle, base + 0x16, 0);
            wr32(battle, base + 0x18, 0);
            wr32(battle, base + 0x1C, 0);
            battle[base + 0xF] = 0xFF;
            battle[base + 0xD] = 0;
            battle[base + 0xE] = 0;
            wr32(battle, base, env.battle_address.wrapping_add(0x1310));
            wr16(battle, 0x12F2, 0);
            wr32(battle, 0x12F4, 0);
            battle[0x12F0] = 1;
            battle[0x12F8] = 0xFF;
            wr32(battle, 0x12FC, 0x3F35_04F4);
            battle[0x1300] = 1;
            if *ui_flag {
                battle[0x12E9] = 1;
            } else {
                host.ui_message();
                battle[0x12E9] = 0;
                *ui_flag = true;
                battle[0x12EA] = 1;
            }
        }
    }
    second_ship[0x12A] &= 0xF7;
    first_ship[0x12A] &= 0xF7;
    for ship in [&mut *second_ship, &mut *first_ship] {
        wr16(ship, 0x128, 0xFFFF);
        let heading = u16::from(ship[0x11]);
        wr16(ship, 0x24, heading);
        wr16(ship, 0x20, 0xFFFF);
    }
    if capture {
        take_ship(
            first,
            second,
            battle,
            prev,
            next,
            first_ship,
            second_ship,
            first_world,
            second_world,
            merchants,
            env,
            host,
        );
    } else {
        loot(
            first_ship,
            second_ship,
            first_world,
            second_world,
            merchants,
            env,
            host,
            battle,
            ui_flag,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn take_ship<H: CaptureHost>(
    first: u16,
    second: u16,
    battle: &mut [u8],
    prev: &mut [u16],
    next: &mut [u16],
    first_ship: &mut [u8],
    second_ship: &mut [u8],
    first_world: &mut [u8],
    second_world: &mut [u8],
    merchants: &mut [u8],
    env: &CaptureEnv,
    host: &mut H,
) {
    let first_world_index = u32::from_le_bytes(first_ship[4..8].try_into().unwrap());
    let second_world_index = u32::from_le_bytes(second_ship[4..8].try_into().unwrap());
    wr16(second_world, 0x40, 0);
    second_ship[0x13D] = first_ship[0x13D];
    second_ship[0x12A] &= 0xDE;
    host.ownership_event_with_state(
        first_world_index,
        second_world_index,
        first_world,
        second_world,
        merchants,
    );
    let side = usize::from(battle[0x559 + usize::from(second_ship[0x13D])]);
    if second_world_index == rd32(battle, 0xA84 + 4 * side) {
        let word = u32::from(rd16(second_world, 6));
        wr32(battle, 0xA84 + 4 * side, word);
    }
    let (b, a) = (usize::from(second), usize::from(first));
    let map = battle[0x559 + usize::from(first_ship[0x13D])];
    let (count_up, count_down, head) = if map == 0 {
        (0x658, 0x65A, 0x65C)
    } else {
        (0x65A, 0x658, 0x10)
    };
    wr16(battle, count_up, rd16(battle, count_up).wrapping_add(1));
    wr16(battle, count_down, rd16(battle, count_down).wrapping_sub(1));
    let before = prev[b];
    if before == 0xFFFF {
        wr16(battle, head, next[b]);
    } else {
        next[usize::from(before)] = next[b];
    }
    if next[b] != 0xFFFF {
        prev[usize::from(next[b])] = prev[b];
    }
    next[b] = next[a];
    prev[b] = first;
    next[a] = second;
    if next[b] != 0xFFFF {
        prev[usize::from(next[b])] = second;
    }
    wr16(second_world, 0x42, 0xFFFF);
    let side = u32::from(battle[0x559 + usize::from(first_ship[0x13D])]) + 1;
    if side == u32::from(battle[0xAD2]) {
        let current = rd16(battle, 0x1E94);
        let level = (current >> 10) as u8;
        let limit = if env.virtual_10 { 0xE } else { 0xC };
        if limit > level {
            wr16(battle, 0x1E94, (u16::from(limit) << 10).wrapping_add(current & 0x3FF));
        }
    }
    let _ = host;
}

#[allow(clippy::too_many_arguments)]
fn loot<H: CaptureHost>(
    first_ship: &mut [u8],
    second_ship: &mut [u8],
    first_world: &mut [u8],
    second_world: &mut [u8],
    merchants: &mut [u8],
    env: &CaptureEnv,
    host: &mut H,
    battle: &mut [u8],
    _ui_flag: &mut bool,
) {
    second_ship[0x12A] |= 0x60;
    let mut free = (rd32(first_world, 0x10) as i32)
        .wrapping_sub(rd32(first_world, 0x11C) as i32)
        .wrapping_sub(rd32(first_world, 0x118) as i32);
    let mut looted = false;
    let extra = rd16(second_world, 0x154);
    if extra != 0 {
        let need = u32::from(extra) * 10;
        if (free as u32) >= need {
            wr16(first_world, 0x154, rd16(first_world, 0x154).wrapping_add(extra));
            wr16(second_world, 0x154, 0);
            free = (free as u32).wrapping_sub(need) as i32;
            looted = true;
        } else {
            let units = (free as u32) / 10;
            if units != 0 {
                wr16(first_world, 0x154, rd16(first_world, 0x154).wrapping_add(units as u16));
                wr16(
                    second_world,
                    0x154,
                    rd16(second_world, 0x154).wrapping_sub(units as u16),
                );
                free = free.wrapping_sub(units.wrapping_mul(10) as i32);
                looted = true;
            }
        }
    }
    let mut order: Vec<u8> = Vec::new();
    for ware in 0..24usize {
        if rd32(second_world, 0x54 + 4 * ware) != 0 {
            let mut position = order.len();
            order.push(0);
            while position > 0 && BASE_PRICE[ware] > BASE_PRICE[usize::from(order[position - 1])] {
                order[position] = order[position - 1];
                position -= 1;
            }
            order[position] = ware as u8;
        }
    }
    let mut total: u32 = 0;
    for &ware in &order {
        let ware = usize::from(ware);
        let stock = rd32(second_world, 0x54 + 4 * ware) as i32;
        let mut take = if (stock as u32) > (free as u32) {
            free as u32
        } else {
            stock as u32
        };
        let lot: u32 = if ware < 20 {
            if SMALL_LOT_WARES[ware] {
                200
            } else {
                2000
            }
        } else {
            10
        };
        take -= take % lot;
        let remainder = stock.wrapping_rem(lot as i32);
        let kept = stock.wrapping_sub(remainder);
        wr32(second_world, 0x54 + 4 * ware, kept as u32);
        if take != 0 {
            looted = true;
            let mine = rd32(first_world, 0x54 + 4 * ware);
            wr32(first_world, 0x54 + 4 * ware, mine.wrapping_add(take));
            let rest = kept.wrapping_sub(take as i32);
            wr32(second_world, 0x54 + 4 * ware, rest as u32);
            let value = f64::from(f64::from(take) as f32) * f64::from(BASE_PRICE[ware]);
            free = free.wrapping_sub(take as i32);
            total = total.wrapping_add(ftol_i32(value) as u32);
        }
    }
    if looted {
        recompute_load(first_world);
        recompute_load(second_world);
        let a = rd16(first_world, 0x3C) | 0x14;
        wr16(first_world, 0x3C, a);
        let b = rd16(second_world, 0x3C) | 0x14;
        wr16(second_world, 0x3C, b);
    }
    // recompensa al comerciante de la flota
    let fleet = rd16(first_world, 8);
    let entry = usize::from(fleet) * 60 + 0x10;
    let reward_world = rd16(env.fleets, entry);
    if reward_world < env.fleet_count {
        if let Some(ship) = (env.fleet_ship)(reward_world) {
            if u16::from(ship.owner) < env.merchant_limit && ship.captain < env.captains.count {
                let mut value = total.wrapping_mul(65) / 100;
                if value > 0xEA60 {
                    value = 0xEA60;
                }
                total = value;
                let base = usize::from(ship.owner) * MERCHANT_SIZE;
                let gold = rd32(merchants, base).wrapping_add(value);
                wr32(merchants, base, gold);
                let record = rd32(merchants, base + 0x4B8).wrapping_add(value);
                wr32(merchants, base + 0x4B8, record);
                let reputation = i16::from_le_bytes([merchants[base + 0x16], merchants[base + 0x17]]);
                let updated = if i32::from(reputation) < 0x7FFF - i32::from(REPUTATION_STEP) {
                    reputation + REPUTATION_STEP
                } else {
                    0x7FFF
                };
                merchants[base + 0x16..base + 0x18].copy_from_slice(&updated.to_le_bytes());
                if value != 0 {
                    let mut message = fresh_message();
                    let m = &merchants[base..base + MERCHANT_SIZE];
                    let captain = &env.captains.table[usize::from(ship.captain) * 16..][..16];
                    message[5] = m[0x19];
                    message[4] = 0x30;
                    message[8] = captain[2];
                    message[9] = captain[3];
                    if second_world[0] != 0xFF {
                        let owner = merchant(merchants, second_world[0]);
                        message[10] = owner[0x1C];
                        message[11] = owner[0x1D];
                    } else {
                        message[10] = first_ship[4] & 0x3F;
                        message[11] = second_ship[4] & 0x3F;
                    }
                    wr16(&mut message, 12, value as u16);
                    wr16(
                        &mut message,
                        14,
                        u16::from(second_world[0xE] & 3).wrapping_add(ship.kind_word << 2),
                    );
                    message[9] |= 0x80;
                    host.news(i32::from(ship.owner), &message);
                }
            }
        }
    }
    if total >= 0x1388 {
        let hint = first_world[0x158];
        let mut index = 0u8;
        if hint < 5 && env.objects[usize::from(hint)] {
            index = hint;
        } else {
            for i in 0..5u8 {
                if env.objects[usize::from(i)] {
                    index = i;
                    break;
                }
            }
        }
        if env.objects[usize::from(index)] {
            host.notify_object_with_state(
                index,
                rd32(first_ship, 4),
                rd32(second_ship, 4),
                first_world,
                second_world,
                merchants,
            );
        }
    }
    let side = u32::from(battle[0x559 + usize::from(first_ship[0x13D])]) + 1;
    if side == u32::from(battle[0xAD2]) {
        let current = rd16(battle, 0x1E94);
        if (current >> 10) < 10 {
            wr16(battle, 0x1E94, (current & 0x3FF).wrapping_add(0x2800));
        }
    }
}
