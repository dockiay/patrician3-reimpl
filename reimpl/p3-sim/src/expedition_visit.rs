//! Recorrido de los cuatro destinos de 4E6A54 antes de aplicar el desgaste.
//! Los centros se atienden mediante un anfitrión transaccional explícito.
use crate::expedition_discovery_event;
use p3_core::{ai::reputation::Merchant, calendar::Calendar, economy::trade_center};

#[derive(Clone)]
pub struct VisitState {
    pub merchants: Vec<Merchant>,
    pub global_map: Vec<u8>,
    pub world_flag: u8,
    pub message_pool: Vec<u8>,
    /// Quince centros serializados S04 (30 bytes más 16 por contrato).
    pub centers: Vec<Vec<u8>>,
    pub notifications: Vec<[u32; 6]>,
    pub windows: Vec<[u32; 3]>,
    pub dirty: bool,
}

/// Efectos del primer centro que corresponde a un waypoint; debe preparar la
/// marca, descubrimiento, comercio y sus cartas en `state` antes de devolver.
/// `allow_contract` empieza activo y se desactiva tras el primer contrato.
pub trait CenterHost: Clone {
    fn visit(
        &mut self,
        state: &mut VisitState,
        center: usize,
        waypoint: usize,
        mode: u16,
        allow_contract: bool,
    ) -> Result<CenterOutcome, String>;
}
#[derive(Clone, Copy, Debug)]
pub struct CenterOutcome {
    pub discovered: bool,
    pub contract_created: bool,
}
#[derive(Clone, Copy)]
pub struct VisitInput<'a> {
    pub waypoints: [(u16, u16); 4],
    pub owner: u8,
    pub merchant_count: u16,
    pub player: u32,
    pub large_renown: i16,
    pub tick: u32,
    pub calendar: Calendar,
    pub mode: u16,
    pub dirty_target: bool,
    /// Posiciones 673858 y puntos vivos Expeditionen.dat, en ese orden.
    pub locations: &'a [(u16, u16)],
    pub discovery_points: &'a [(u16, u16)],
}
#[derive(Clone)]
pub struct Prepared<H> {
    pub state: VisitState,
    pub host: H,
    pub discovered: Vec<Option<u8>>,
    pub unmatched: u8,
    pub has_contract: bool,
}

/// Ningún callback recibe las entradas originales: un error descarta mapas,
/// cartas, centros, comerciantes y estado del anfitrión preparados.
pub fn prepare<H: CenterHost>(before: &VisitState, host: &H, input: VisitInput<'_>) -> Result<Prepared<H>, String> {
    if before.centers.len() != 15
        || before.global_map.len() != 0x64
        || usize::from(input.merchant_count) > before.merchants.len()
    {
        return Err("estado de visita de expedición incompleto".into());
    }
    for c in &before.centers {
        if c.len() < 30 || c.len() != 30 + usize::from(u16::from_le_bytes([c[28], c[29]])) * 16 {
            return Err("centro de expedición truncado".into());
        }
    }
    let mut out = Prepared {
        state: before.clone(),
        host: host.clone(),
        discovered: Vec::new(),
        unmatched: 0,
        has_contract: false,
    };
    let wide = input.mode == 0;
    let mut allow_contract = true;
    for (slot, &waypoint) in input.waypoints.iter().enumerate() {
        if waypoint.0 >= 0x7fff {
            continue;
        }
        if u16::from(input.owner) < input.merchant_count {
            let mark = expedition_discovery_event::prepare_mark(
                out.state.world_flag,
                input.owner,
                &out.state.merchants,
                input.merchant_count,
                input.player,
                input.large_renown,
                &out.state.message_pool,
                input.calendar,
                &out.state.global_map,
                input.discovery_points,
                waypoint,
                wide,
                input.dirty_target,
            )?;
            out.state.merchants[usize::from(input.owner)] = mark.merchant;
            out.state.global_map = mark.global_map;
            out.state.world_flag = mark.world_flag;
            out.state.message_pool = mark.message_pool;
            out.state.dirty |= mark.dirty;
            if let Some(n) = mark.notify {
                out.state.notifications.push(n)
            }
            if let Some(w) = mark.event {
                out.state.windows.push(w)
            }
        }
        let mut found = None;
        for (i, c) in out.state.centers.iter().enumerate() {
            let id = c[4];
            if id >= 0x7f {
                continue;
            }
            let target = *input
                .locations
                .get(usize::from(id))
                .ok_or("posición de centro ausente")?;
            let expiry = u32::from_le_bytes(c[8..12].try_into().unwrap());
            if trade_center::matches_waypoint(
                id,
                expiry,
                input.tick,
                target,
                waypoint,
                input.discovery_points,
                u8::from(wide),
            )
            .map_err(str::to_owned)?
            {
                found = Some(i);
                break;
            }
        }
        if let Some(center) = found {
            if u16::from(input.owner) < input.merchant_count {
                out.state.merchants[usize::from(input.owner)].set_map_discovery_bit(
                    &mut out.state.global_map,
                    center as u32,
                    input.dirty_target,
                    &mut out.state.dirty,
                );
            }
            let id = out.state.centers[center][4];
            let kind = out.state.centers[center][5];
            let effect = out
                .host
                .visit(&mut out.state, center, slot, input.mode, allow_contract)?;
            if effect.discovered {
                out.discovered.push(if kind < 2 { Some(id) } else { None })
            }
            if effect.contract_created {
                allow_contract = false;
                out.has_contract = true
            }
        } else {
            out.unmatched = out.unmatched.wrapping_add(1)
        }
    }
    Ok(out)
}
