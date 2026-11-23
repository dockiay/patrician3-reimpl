//! IA de expediciones: 0x00503B50 y elección de punto 0x00534F10.
//! La cola de operaciones es un efecto devuelto; la CRT pertenece al anfitrión.
use super::{
    r16, r32,
    remove::{CONVOY_SIZE, MERCHANT_SIZE, SHIP_SIZE},
    tick::TickWorld,
};
use crate::{pump::Operation, rng::CrtRand};

#[derive(Clone, Copy)]
pub struct PlanEnv<'a> {
    pub expedition_lock: u8,
    pub expedition_countdown: u8,
    pub network_client: bool,
    pub trade_centers: &'a [Vec<u8>],
    pub locations: &'a [(i16, i16)],
    pub map_points: &'a [(i16, i16)],
}

/// El original conserva BL=1 tras agotar la búsqueda de centros descubiertos:
/// la segunda pasada (cualquier centro) recorre sólo semilla%15..15.
pub fn pick_target(
    known: &[u8],
    env: &PlanEnv<'_>,
    tick: u32,
    prefer_center: bool,
    seed: u8,
) -> Result<(u16, u16), String> {
    if prefer_center {
        if env.trade_centers.len() < 15 || env.trade_centers[..15].iter().any(|v| v.len() < 12) {
            return Err("centros comerciales incompletos".into());
        }
        let start = usize::from(seed) % 15;
        let center = (start..15)
            .chain(0..15)
            .find(|&i| {
                let c = &env.trade_centers[i];
                c[4] < 30 && u32::from_le_bytes(c[8..12].try_into().unwrap()) < tick
            })
            .or_else(|| (start..15).find(|&i| env.trade_centers[i][4] < 30));
        if let Some(i) = center {
            let &(x, y) = env
                .locations
                .get(usize::from(env.trade_centers[i][4]))
                .ok_or("posición de centro comercial ausente")?;
            return Ok((x as u16, y as u16));
        }
    }
    let count = env.map_points.len();
    if count <= 100 || known.len() < ((count + 31) / 32) * 4 {
        return Err("puntos o bits de descubrimiento incompletos".into());
    }
    let start = 100 + usize::from(seed) % (count - 100);
    for i in (start..count).chain(100..count) {
        let at = (i / 32) * 4;
        let bits = u32::from_le_bytes(known[at..at + 4].try_into().unwrap());
        if bits & (1u32 << (i & 31)) == 0 {
            let (x, y) = env.map_points[i];
            return Ok((x as u16, y as u16));
        }
    }
    Ok((u16::MAX, u16::MAX))
}

/// Mantiene también las salidas sin expedición (humano, carga, casco o guards).
/// El error por contexto incompleto no publica flags ni consume CRT.
pub fn plan(
    w: &mut TickWorld,
    cv: u16,
    env: &PlanEnv<'_>,
    rng: &mut Option<CrtRand>,
) -> Result<Vec<Operation>, String> {
    let at = usize::from(cv) * CONVOY_SIZE;
    let convoy = w
        .remove
        .convoys
        .get(at..at + CONVOY_SIZE)
        .ok_or("convoy de expedición inexistente")?;
    let owner = usize::from(convoy[0]);
    if owner >= usize::from(w.remove.merchant_count) {
        return Ok(Vec::new());
    }
    let merchant = w
        .remove
        .merchants
        .get(owner * MERCHANT_SIZE..(owner + 1) * MERCHANT_SIZE)
        .ok_or("comerciante de expedición inexistente")?;
    if r16(merchant, 8) == 0 {
        return Ok(Vec::new());
    }
    let flags = convoy[0x14] & !2;
    let count = usize::from(r16(w.remove.manager, 0xf4));
    let leader = usize::from(r16(convoy, 0x10));
    if env.expedition_lock != 0 || env.expedition_countdown != 0 || env.network_client || leader >= count {
        w.remove.convoys[at + 0x14] = flags;
        return Ok(Vec::new());
    }
    let leader_image = w
        .remove
        .ships
        .get(leader * SHIP_SIZE..(leader + 1) * SHIP_SIZE)
        .ok_or("líder de expedición inexistente")?;
    let id = r16(leader_image, 8);
    let mut next = usize::from(r16(convoy, 0xa));
    let mut seen = vec![false; count];
    while next < count {
        if seen[next] {
            return Err("cadena de expedición cíclica".into());
        }
        seen[next] = true;
        let ship = w
            .remove
            .ships
            .get(next * SHIP_SIZE..(next + 1) * SHIP_SIZE)
            .ok_or("miembro de expedición inexistente")?;
        let maximum = r32(ship, 0x14);
        let threshold = maximum.wrapping_sub(maximum >> 4);
        if r32(ship, 0x18) < threshold || r32(ship, 0x118) > 5000 {
            w.remove.convoys[at + 0x14] = flags;
            return Ok(Vec::new());
        }
        next = usize::from(r16(ship, 6));
    }
    let mut random = rng.ok_or("CRT de expedición ausente")?;
    let amount = ((random.rand() as u8) & 31) + 20;
    let mut ops = vec![Operation([0x1e, 0, u32::from(id), u32::from(amount), 0])];
    for slot in 0..4u32 {
        let (x, y) = pick_target(&merchant[0x4d8..0x53c], env, w.time, slot == 0, random.rand() as u8)?;
        ops.push(Operation([
            0x1f,
            u32::from(id) | (slot << 16),
            u32::from(x) | (u32::from(y) << 16),
            3,
            0,
        ]));
    }
    w.remove.convoys[at + 0x14] = flags;
    *rng = Some(random);
    Ok(ops)
}
