//! Capture growth (0x515F70), including hut and fresh reserve-ship creation.
//! Local RNG is seeded from the clock; no CRT/global RNG or news is involved.
use p3_core::{
    ships::{
        cargo,
        factory::{self, HanseFactory},
    },
    town::Town,
};
use p3_save::pirate_bands::PirateBand;
#[derive(Clone)]
pub struct PirateCaptureState {
    pub bands: Vec<Option<PirateBand>>,
    pub factory: HanseFactory,
    pub towns: Vec<Town>,
    pub ship_names: Vec<Vec<u8>>,
    pub ports: Vec<(u16, u16)>,
    /// Tabla de lugares `0x006DDBB0` (registros de 0x34 bytes, `x` en `+0` e `y` en `+4`): 0x516170 coloca el
    /// barco nuevo en el lugar de la banda (`banda+0xE`) indexando la tabla directamente, no por el puerto de ciudad.
    pub locations: Vec<(u16, u16)>,
    pub tick: u32,
    pub news_stamp:u32,
    pub news:Vec<u8>,
    pub day_of_year: u16,
    /// Byte at 0x6DE52E: current town index for shipyard weapon quality.
    pub day: u8,
    /// Inputs of `Captain::init` 0x4FDF50 for a captain allocated during a capture (leader change).
    pub captain: Option<p3_core::economy::pirate_group_callbacks::CaptainResources>,
    /// Convoyes con tarea 0x34 de caza en fase >= 7 (`+0x16`), para 0x503B00.
    pub hunt_convoys: Vec<u16>,
    /// Ranura y registro de esas tareas 0x34 (la captura puede cambiar `+0xE`, 0x540B00).
    pub hunt_tasks: Vec<(u16, p3_core::scheduler::TaskRecord)>,
    /// Nodos de bajas que la captura antepone a `tarea+8` (ranura, nodo), en orden.
    pub hunt_added: Vec<(u16, p3_core::ships::return_convoy::HuntLoss)>,
}
fn r16(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes(b[p..p + 2].try_into().unwrap())
}
fn w16(b: &mut [u8], p: usize, v: u16) {
    b[p..p + 2].copy_from_slice(&v.to_le_bytes())
}
fn r32(b: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(b[p..p + 4].try_into().unwrap())
}
fn w32(b: &mut [u8], p: usize, v: u32) {
    b[p..p + 4].copy_from_slice(&v.to_le_bytes())
}
fn random(seed: &mut u32) -> u32 {
    *seed = p3_core::ships::wind::next_seed(*seed);
    *seed
}
impl PirateCaptureState {
    /// Atomic: invalid resources or chains leave all state untouched. The caller
    /// must synchronize factory ships/captains/convoys before each notification.
    pub fn register_capture(&mut self, index: u8) -> Result<(), String> {
        let band = self
            .bands
            .get(usize::from(index))
            .and_then(Option::as_ref)
            .ok_or("banda de captura ausente")?;
        if usize::from(r16(&band.header, 8)) < self.factory.convoys.len() {
            return Ok(());
        }
        let mut staged = self.clone();
        let mut band = staged.bands[usize::from(index)].take().unwrap();
        staged.apply(&mut band)?;
        staged.bands[usize::from(index)] = Some(band);
        *self = staged;
        Ok(())
    }
    fn apply(&mut self, band: &mut PirateBand) -> Result<(), String> {
        band.validate()?;
        let level = band.header[12];
        let maximum = *[4u8, 8, 16, 18]
            .get(usize::from(level))
            .ok_or("nivel pirata fuera de tabla")?;
        let score = r16(&band.header, 14).wrapping_add(200);
        w16(&mut band.header, 14, score);
        crate::absorb_hut::build_hut(
            band,
            u16::try_from(self.factory.convoys.len()).map_err(|_| "demasiados convoyes")?,
            self.tick,
            self.day_of_year,
        )?;
        let mut seed = self.tick;
        if random(&mut seed) as u8 & 15 <= level {
            let mut left = maximum.wrapping_add(maximum >> 1);
            let mut at = usize::from(r16(&band.header, 4));
            let mut seen = vec![false; self.factory.ships.len()];
            while at < self.factory.ships.len() {
                if seen[at] {
                    return Err("cadena de barcos piratas cíclica".into());
                }
                seen[at] = true;
                let ship = &self.factory.ships[at];
                if ship.len() != 0x180 {
                    return Err("barco pirata truncado".into());
                }
                if ship[0x15d] != 0 {
                    left = left.wrapping_sub(1);
                    if left == 0 {
                        break;
                    }
                }
                at = usize::from(r16(ship, 4));
            }
            if left != 0 {
                let kind = random(&mut seed) as u8 & 3;
                let id = self.create_ship(band, kind, u32::from(level) * 40 + 50)?;
                w32(&mut self.factory.ships[id], 0x18, 1)
            }
        }
        if random(&mut seed) as u8 & 31 <= level {
            Self::repair_piece(band)?
        }
        Ok(())
    }
    fn repair_piece(band: &mut PirateBand) -> Result<(), String> {
        let count = usize::from(r16(&band.header, 0));
        if count < 4 {
            return Err("reparación pirata requiere cuatro piezas reservadas".into());
        }
        let mut present = [false; 4];
        let mut at = 0usize;
        let mut seen = vec![false; count];
        // Original checks the NEXT index before processing the current piece: the
        // final active piece is deliberately not visited (0x51607D / 0x5160AF).
        while usize::from(r16(&band.pieces, at * 8 + 4)) < count {
            if seen[at] {
                return Err("cadena activa pirata cíclica".into());
            }
            seen[at] = true;
            if at < 4 {
                present[at] = true
            }
            if band.pieces[at * 8 + 3] == 0x30 {
                band.pieces[at * 8 + 3] = 0x31;
                return Ok(());
            }
            at = usize::from(r16(&band.pieces, at * 8 + 4));
        }
        at = usize::from(r16(&band.header, 2));
        seen.fill(false);
        while at < count {
            if seen[at] {
                return Err("cadena libre pirata cíclica".into());
            }
            seen[at] = true;
            if at < 4 {
                present[at] = true
            }
            at = usize::from(r16(&band.pieces, at * 8 + 4))
        }
        for at in 1..4 {
            if !present[at] && matches!(band.pieces[at * 8 + 3], 0x30 | 0x31) {
                band.pieces[at * 8 + 3] = 0x30;
                band.pieces[at * 8 + 6] = 100;
                let next = r16(&band.pieces, 4);
                w16(&mut band.pieces, at * 8 + 4, next);
                w16(&mut band.pieces, 4, at as u16);
                break;
            }
        }
        Ok(())
    }
    /// 0x516170 with a1=0, the only variant invoked by capture. Reuses native
    /// allocator primitives, type stats, upgrade, weapon slots and cargo.
    fn create_ship(&mut self, band: &mut PirateBand, kind: u8, armament: u32) -> Result<usize, String> {
        let f = &mut self.factory;
        if usize::from(f.free_ship) >= f.ships.len() {
            let start = f.ships.len();
            if start + 64 > u16::MAX as usize {
                return Err("pool de barcos desbordado".into());
            }
            f.free_ship = start as u16;
            for i in start..start + 64 {
                let mut ship = vec![0; 0x180];
                w16(&mut ship, 4, 65535);
                w16(&mut ship, 6, (i + 1) as u16);
                w16(&mut ship, 8, 65535);
                w16(&mut ship, 0x134, 255);
                f.ships.push(ship)
            }
        }
        let id = usize::from(f.free_ship);
        let ship = &mut f.ships[id];
        if ship.len() != 0x180 || r16(ship, 0x134) < 0x17 {
            return Err("ranura de barco pirata no libre".into());
        }
        f.free_ship = r16(ship, 6);
        factory::reset_ship(ship, self.tick).ok_or("reset de barco pirata")?;
        ship[0x137] = 0;
        w16(ship, 0x134, 0x12);
        w32(ship, 0x4c, self.tick.wrapping_sub((self.tick & 255) * 0x400));
        let port0 = *self.ports.first().ok_or("puerto cero ausente")?;
        factory::init_ship_at_town(ship, 0, port0.0, port0.1, self.tick).ok_or("inicio de barco pirata")?;
        ship[0x13b] = 0;
        w16(ship, 4, r16(&band.header, 4));
        w16(ship, 6, r16(&band.header, 10));
        w16(&mut band.header, 4, id as u16);
        w16(&mut band.header, 10, id as u16);
        factory::init_ship_stats(self.towns.first().ok_or("ciudad cero ausente")?, ship, kind)
            .ok_or("estadísticas de barco pirata")?;
        factory::recompute_weapon_slots(ship).ok_or("ranuras de barco pirata")?;
        let name = factory::next_ship_name_index(&mut f.name_cursor, f.name_step, self.ship_names.len())
            .ok_or("banco de nombres de barco inválido")?;
        w16(ship, 0x15e, name);
        let text = &self.ship_names[usize::from(name)];
        let n = text.iter().position(|&b| b == 0).unwrap_or(text.len()).min(31);
        ship[0x160..0x160 + n].copy_from_slice(&text[..n]);
        ship[0x160 + n] = 0;
        ship[0x3c] |= 1;
        crate::absorb_ship_refresh::refresh(ship, 1)?;
        let value = self
            .towns
            .get(usize::from(self.day))
            .ok_or("ciudad diaria ausente")?
            .u32(0x810);
        let max_kind = (((u64::from(value) * 0x09fc8735) >> 46) as u32 + 2).min(5) as u8;
        arm_fresh(ship, armament, max_kind)?;
        let crew = factory::additional_sailors(ship).ok_or("tripulación de barco pirata")? as u16;
        w16(ship, 0x40, crew);
        w16(ship, 0x154, crew);
        cargo::recompute_load(ship);
        ship[0] = 255;
        ship[0x13a] = 0;
        let hull = r32(ship, 0x14);
        w32(ship, 0x18, hull);
        let port = *self
            .locations
            .get(usize::from(band.header[6]))
            .ok_or("lugar de la banda pirata ausente")?;
        w16(ship, 0x1e, port.0);
        w16(ship, 0x22, port.1);
        Ok(id)
    }
}
/// 0x51A4E0 for an unarmed fresh ship. The middle replacement loop is inert
/// here: the only initial bytes are FF/7, and installed weapons already equal
/// the requested kind. Refuse prearmed input instead of silently omitting it.
fn arm_fresh(ship: &mut [u8], target: u32, kind: u8) -> Result<(), String> {
    if ship[0x13c..0x154].iter().any(|&b| b != 255 && b != 7) {
        return Err("armamento inicial no vacío".into());
    }
    let weight = [1000u32, 1000, 2000, 2000, 2000, 1000, 8224, 19789];
    let power = [9u32, 10, 22, 24, 30, 18, 0, 0];
    let mut room = r32(ship, 0x10)
        .wrapping_sub(r32(ship, 0x118))
        .wrapping_sub(r32(ship, 0x11c)) as i32;
    if target <= r32(ship, 0x120) || room < 1000 || kind < 2 {
        return Ok(());
    }
    let heavy = kind.min(4);
    for at in (0x13c..0x154).step_by(2) {
        if ship[at] == 255 && ship[at + 1] == 255 {
            let k = usize::from(heavy);
            if weight[k] as i32 > room {
                break;
            }
            ship[at] = heavy;
            ship[at + 1] = 6;
            w32(ship, 0x11c, r32(ship, 0x11c).wrapping_add(weight[k]));
            room -= weight[k] as i32;
            w32(ship, 0x120, r32(ship, 0x120).wrapping_add(power[k]));
            if target <= r32(ship, 0x120) {
                return Ok(());
            }
        }
    }
    for at in 0x13c..0x154 {
        if ship[at] == 255 {
            if room < 1000 {
                return Ok(());
            }
            ship[at] = 1;
            w32(ship, 0x11c, r32(ship, 0x11c).wrapping_add(1000));
            room -= 1000;
            w32(ship, 0x120, r32(ship, 0x120).wrapping_add(10));
            if target <= r32(ship, 0x120) {
                return Ok(());
            }
        }
    }
    Ok(())
}
