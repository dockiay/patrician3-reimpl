//! Comprobación de desincronía del original (PRE-SPEC-NET §4): petición `0xB8`, respuestas
//! `0xB9..0xBE` y `0xD4`, y la comparación del anfitrión.
//!
//! Portado de `op_send_sync_checksums` (0x00542F10, todas las variantes) y del caso `0xD4`
//! de `execute_operations` (0x0054715F), con contraste x86 en `oracle/x86_net_sync.py`.
//! Un cliente nuevo que responda a un anfitrión original debe producir estas mismas sumas.

use crate::wire::Record;

pub const OP_SYNC_REQUEST: u32 = 0xB8;
pub const OP_SYNC_SHIP: u32 = 0xB9;
pub const OP_SYNC_TOWN: u32 = 0xBA;
pub const OP_SYNC_MERCHANTS: u32 = 0xBB;
pub const OP_SYNC_MANAGER: u32 = 0xBC;
pub const OP_SYNC_ERROR: u32 = 0xBD;
pub const OP_SYNC_MERCHANT: u32 = 0xBE;
pub const OP_SAVE: u32 = 0xC2;
pub const OP_SYNC: u32 = 0xD4;
/// Nombre de la partida que guarda el anfitrión al detectar una diferencia.
pub const DESYNC_SAVE_NAME: &[u8] = b"NETZWERK";

pub const TOWN_LEN: usize = 0x9F8;
pub const MERCHANT_LEN: usize = 0x650;
pub const SHIP_LEN: usize = 0x180;

/// Lo que leen las sumas, con la disposición de memoria del original.
pub struct SyncWorld<'a> {
    /// `[0x6DF31C]`: rol de red (0 = anfitrión o sin red).
    pub role: u32,
    /// `[0x6DE4B4]`: tick del mundo.
    pub tick: u32,
    /// Ciudades de `[0x6DE508]` (0x9F8 bytes cada una; `[0x6DE4B0]` = número).
    pub towns: Vec<&'a [u8]>,
    /// Comerciantes de `[0x6DE4A0+0x78]` (0x650 bytes; `[0x6DE4AA]` = número).
    pub merchants: Vec<&'a [u8]>,
    /// Gestor de barcos desde 0x6DD7A0 (al menos 0xF8 bytes: tabla de 16 registros en
    /// `+0x20`, campos `+0xE0..+0xEA`, número de barcos `u16 +0xF4`).
    pub manager: &'a [u8],
    /// Barcos de `[0x6DD7A4]`, 0x180 bytes cada uno.
    pub ships: &'a [u8],
    /// Primer barco (`u16 +0xC`) de los objetos apuntados por `0x6DD7AC..0x6DD7BC`
    /// (bandas piratas); `None` = puntero nulo.
    pub band_heads: [Option<u16>; 5],
    /// `u16` de 0x6DD734, 0x6DD744 y 0x6DD746 (fuera del gestor).
    pub words_734_744_746: [u16; 3],
}

fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}
fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

impl SyncWorld<'_> {
    fn ship_count(&self) -> u32 {
        u32::from(u16_at(self.manager, 0xF4))
    }
    fn ship(&self, index: u32) -> &[u8] {
        let at = index as usize * SHIP_LEN;
        &self.ships[at..at + SHIP_LEN]
    }
    fn manager_word(&self, address: u32) -> u32 {
        u32::from(match address {
            0x6DD734 => self.words_734_744_746[0],
            0x6DD744 => self.words_734_744_746[1],
            0x6DD746 => self.words_734_744_746[2],
            a => u16_at(self.manager, (a - 0x6DD7A0) as usize),
        })
    }
    /// Recorre una lista de barcos (`+4` = siguiente) mientras el índice sea válido.
    fn walk(&self, head: u16, mut visit: impl FnMut(&[u8])) -> Result<(), &'static str> {
        let count = self.ship_count();
        let mut index = u32::from(head);
        let mut steps = 0u32;
        while index < count {
            let ship = self.ship(index);
            visit(ship);
            index = u32::from(u16_at(ship, 4));
            steps += 1;
            if steps > count {
                // El original no termina con una lista cíclica.
                return Err("lista de barcos cíclica");
            }
        }
        Ok(())
    }
}

/// Respuesta de una máquina a `0xB8 {tipo, indicador}`: registros que encola, en orden.
pub fn sync_responses(request: Record, world: &SyncWorld) -> Result<Vec<Record>, &'static str> {
    let mut out = Vec::new();
    let kind = request[1];
    match kind.wrapping_sub(1) {
        0 => {
            // Tipo 1: un 0xB9 por barco, solo en clientes.
            if world.role != 0 {
                for i in 0..world.ship_count() {
                    let s = world.ship(i);
                    out.push([OP_SYNC_SHIP, (i & 0xFFFF) | u32::from(u16_at(s, 0x34)) << 16, u32_at(s, 0x1C), u32_at(s, 0x20), u32_at(s, 0x18)]);
                }
            }
        }
        1 => {
            // Tipo 2: un 0xBA por ciudad, solo en clientes.
            if world.role != 0 {
                for (i, t) in world.towns.iter().enumerate() {
                    let packed = u32_at(t, 0x2C8);
                    let mixed = ((((packed as i32) >> 7) as u8) & 0x3F).wrapping_add(t[0x2C8] & 0x3F);
                    let sum16 = u16_at(t, 0x2E4).wrapping_add(u16_at(t, 0x2E0)).wrapping_add(u16_at(t, 0x2DC)).wrapping_add(u16_at(t, 0x2D8));
                    let (mut a, mut b, mut c) = (0u32, 0u32, 0u32);
                    for k in 0..24 {
                        b = b.wrapping_add(u32_at(t, 4 + 4 * k));
                        a = a.wrapping_add(u32_at(t, 0x310 + 4 * k));
                        c = c.wrapping_add(u32_at(t, 0x64 + 4 * k));
                    }
                    let word4 = u32::from(i as u8) | u32::from(mixed) << 8 | u32::from(sum16) << 16;
                    out.push([OP_SYNC_TOWN, word4, a, b, c]);
                }
            }
        }
        2 => {
            // Tipo 3: un 0xBC con los campos del gestor.
            let w = |a| world.manager_word(a);
            out.push([
                OP_SYNC_MANAGER,
                (w(0x6DD746) << 16).wrapping_add(w(0x6DD744)),
                (w(0x6DD888) << 16).wrapping_add(w(0x6DD88A)),
                (w(0x6DD882) << 16).wrapping_add(w(0x6DD880)),
                (w(0x6DD884) << 16).wrapping_add(w(0x6DD734)),
            ]);
        }
        3 => {
            // Tipo 4: un 0xBE por comerciante.
            for (i, m) in world.merchants.iter().enumerate() {
                out.push([OP_SYNC_MERCHANT, u32_at(m, 0), u32_at(m, 0x46C), i32::from(u16_at(m, 0x22) as i16) as u32, i as u32]);
            }
        }
        4 => {}
        _ => {
            if request[2] == 1 {
                let mut gold = 0u32;
                for m in world.merchants.iter().rev() {
                    gold = gold.wrapping_add(u32_at(m, 0));
                    gold = gold.wrapping_add(u32_at(m, 0x46C));
                    gold = gold.wrapping_add(u32::from(u16_at(m, 0x22).wrapping_add(0x64))) & 0x3FFF_FFFF;
                }
                let towns = world.towns.iter().fold(0u32, |acc, t| acc.wrapping_add(u32_at(t, 0x2D4)));
                let w = |a| world.manager_word(a);
                let manager = [0x6DD746, 0x6DD744, 0x6DD888, 0x6DD88A, 0x6DD882, 0x6DD880, 0x6DD884].iter().fold(0u32, |acc, &a| acc.wrapping_add(w(a)));
                out.push([OP_SYNC_MERCHANTS, world.tick, gold, towns, manager]);
            }
            out.push(sync_checksum(world)?);
        }
    }
    Ok(out)
}

/// La suma `0xD4` del caso por defecto.
pub fn sync_checksum(world: &SyncWorld) -> Result<Record, &'static str> {
    let mut stock = 0u32;
    for t in &world.towns {
        for k in 0..24 {
            stock = stock.wrapping_add(u32_at(t, 4 + 4 * k) & 0x3F_FFFF);
        }
    }
    let count = u16_at(world.manager, 0xF4);
    let mut weather = 0u32;
    for e in 0..16 {
        let at = 0x20 + 12 * e;
        if u16_at(world.manager, at) < count {
            weather = weather.wrapping_add(u32::from(u16_at(world.manager, at + 4))).wrapping_add(u32::from(u16_at(world.manager, at + 6)));
        }
    }
    let word8 = (world.tick & 0xFFFF).wrapping_add((weather & 0xFFFF) << 16);
    let mut fleet = 0u32;
    for m in world.merchants.iter().rev() {
        world.walk(u16_at(m, 0xE), |s| fleet = fleet.wrapping_add(u32_at(s, 0x20).wrapping_add(u32_at(s, 0x1C))))?;
    }
    for head in world.band_heads.iter().flatten() {
        world.walk(*head, |s| fleet = fleet.wrapping_add(u32_at(s, 0x20).wrapping_add(u32_at(s, 0x1C))))?;
    }
    let mut cargo = 0u32;
    for m in world.merchants.iter().rev() {
        world.walk(u16_at(m, 0xE), |s| {
            for k in 0..24 {
                cargo = cargo.wrapping_add(u32_at(s, 0x54 + 4 * k));
            }
        })?;
    }
    Ok([OP_SYNC, stock, word8, fleet, cargo])
}

/// Comparación del anfitrión (`OPS+0x904..+0x913`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SyncChecker {
    pub reference: [u32; 4],
}

impl SyncChecker {
    /// `0xB8` reinicia la clave (`OPS+0x908 = 0`) al responder.
    pub fn on_request(&mut self) {
        self.reference[1] = 0;
    }

    /// Caso `0xD4`. Solo actúa con rol 0. `running` = bit `0x10` de `OPS+0`;
    /// `head_len` = registros en el paquete de cabeza (`OPS+0x56`): con 52 no se encola.
    /// Devuelve los registros encolados (guardar «NETZWERK» y `0xBD`).
    pub fn on_checksum(&mut self, role: u32, record: Record, running: bool, mut head_len: usize) -> Vec<Record> {
        let mut out = Vec::new();
        if role != 0 {
            return out;
        }
        let key = (self.reference[1] & 0xFFFF) as u16;
        let incoming = (record[2] & 0xFFFF) as u16;
        if key < incoming {
            self.reference.copy_from_slice(&record[1..5]);
            return out;
        }
        if key > incoming || self.reference == [record[1], record[2], record[3], record[4]] {
            return out;
        }
        self.reference[1] = self.reference[1].wrapping_add(0x100);
        if running {
            let mut name = [0u8; 12];
            name[..DESYNC_SAVE_NAME.len()].copy_from_slice(DESYNC_SAVE_NAME);
            // strcpy sobre +8: copia «NETZWERK» y su terminador; el resto conserva la carga.
            let mut save = record;
            save[0] = OP_SAVE;
            save[1] = 0xFFFF_FFFF;
            let mut bytes = [0u8; 12];
            for (k, w) in save[2..5].iter().enumerate() {
                bytes[4 * k..4 * k + 4].copy_from_slice(&w.to_le_bytes());
            }
            bytes[..DESYNC_SAVE_NAME.len() + 1].copy_from_slice(&name[..DESYNC_SAVE_NAME.len() + 1]);
            for k in 0..3 {
                save[2 + k] = u32::from_le_bytes(bytes[4 * k..4 * k + 4].try_into().unwrap());
            }
            if head_len < 0x34 {
                out.push(save);
                head_len += 1;
            }
        }
        if head_len < 0x34 {
            out.push([OP_SYNC_ERROR, 0, 0, 0, 0]);
        }
        out
    }
}
