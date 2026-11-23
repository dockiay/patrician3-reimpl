//! Prefijo acotado de `0x005FBFE0` para una guarida sin defensores ni torres.
//! El llamante aporta un objeto ya construido y debe ejecutar el final real.
use super::{
    town_attack_mask::{hideout_land_mask, TownProjection, LAND_COLLISION_ARGS},
    w16, w32,
};

pub struct EmptyHideoutStart<'a> {
    pub target: u8,
    pub attacker: u16,
    pub convoy: Option<u16>,
    pub convoy_head: u16,
    pub attacker_count: u16,
    pub defender_convoy: Option<u16>,
    pub owner: u8,
    pub human: bool,
    pub tick: u32,
    pub position: (u16, u16),
    pub portrait: (u8, u8),
    pub plan_width: u32,
    pub plan_height: u32,
    pub cells: &'a [u8],
    pub projection: TownProjection,
    pub collision_id: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmptyHideoutEffect {
    Collision([u32; 6]),
    Notice([u32; 6]),
    SlowTime,
    Sync,
    Finish(u32),
}
/// Prepara los bytes del tramo previo a `town_attack_battle_finish(0)`.
/// Exige que el llamante haya comprobado que no hay torres ni defensores.
/// No realiza callbacks ni publica efectos; no representa el fin del asalto.
pub fn prepare_empty_hideout_start(
    object: &mut [u8],
    v: &EmptyHideoutStart<'_>,
) -> Result<Vec<EmptyHideoutEffect>, &'static str> {
    if object.len() < 0x2f1b {
        return Err("objeto de ataque urbano truncado");
    }
    if !(250..=254).contains(&v.target)
        || v.attacker_count == 0
        || v.convoy.is_none() && (v.attacker_count != 1 || v.convoy_head != v.attacker)
        || v.defender_convoy.is_some_and(|id| id == u16::MAX)
        || v.convoy.is_some_and(|id| id == u16::MAX)
        || v.convoy.is_some() && v.convoy_head == u16::MAX
    {
        return Err("entrada de asalto vacía no contrastada");
    }
    let mut out = object.to_vec();
    out[0x1e98] = v.target;
    out[0x1e99..0x1e9d].fill(0);
    out[0x1e9e..0x1eae].fill(0xff);
    out[0x1ec4..0x1ed4].fill(0);
    out[0x2f08..0x2f18].fill(0);
    w16(&mut out, 0x1e94, 0);
    out[0xad2] = 2;
    w16(&mut out, 0xad6, v.position.0);
    w16(&mut out, 0xada, v.position.1);
    out[0x1e90] = v.portrait.0;
    out[0x1e91] = v.portrait.1;
    hideout_land_mask(
        &mut out,
        v.plan_width,
        v.plan_height,
        v.cells,
        v.projection,
        v.collision_id,
    )?;
    out[0x2f1a] = 0;
    w16(&mut out, 0x35a, u16::from(v.owner));
    w16(&mut out, 0x35c, 0xff);
    out[0x558] = 2;
    out[0x559] = 0;
    out[0x55a] = 1;
    out[0x5d] = u8::from(!v.human);
    out[0x5e] = 1;
    w16(&mut out, 0x658, v.attacker_count);
    w16(&mut out, 0x65a, 0);
    w16(&mut out, 0x65e, v.convoy.unwrap_or(u16::MAX));
    w16(&mut out, 0x660, v.defender_convoy.unwrap_or(u16::MAX));
    w32(&mut out, 0xa84, i32::from(v.convoy_head));
    w32(&mut out, 0xa88, 0xffff);
    out[0xab0] = 1;
    out[0xab1] = 1;
    w32(
        &mut out,
        0xadc,
        v.tick
            .wrapping_add(u32::from(v.target))
            .wrapping_add(u32::from(v.owner))
            .wrapping_add(u32::from(v.attacker))
            .wrapping_add(255) as i32,
    );
    out[0x12e8] = 1;
    let mut effects = vec![EmptyHideoutEffect::Collision(LAND_COLLISION_ARGS)];
    if v.human {
        effects.push(EmptyHideoutEffect::Notice([
            u32::from(v.owner),
            0,
            u32::from(v.position.0),
            u32::from(v.position.1),
            14,
            u32::from(v.attacker),
        ]));
        effects.push(EmptyHideoutEffect::SlowTime);
        out[0xac8] = 0;
    }
    if out[0xac8] == 0 {
        effects.push(EmptyHideoutEffect::Sync);
        out[0xac8] = 1;
    }
    out[0x5c] = 1;
    out[0xac4] = 3;
    effects.push(EmptyHideoutEffect::Finish(0));
    object.copy_from_slice(&out);
    Ok(effects)
}
