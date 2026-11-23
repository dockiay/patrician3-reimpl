//! Transacción acotada del destructor 602990/5FB9F0 para guarida vacía.
use crate::{convoy_absorption::AbsorptionState, pirate_empty_band_end};
use p3_core::{
    rng::CrtRand,
    scheduler::Scheduler,
    ships::{
        pirate_engage,
        remove::RemoveWorld,
        return_convoy::{self, DissolvedEffects, Effects},
        tick::TickWorld,
    },
};
fn r16(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}
fn r32(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(b[i..i + 4].try_into().unwrap())
}
#[derive(Clone, Debug)]
pub enum ConvoyReturn {
    Preserved(Effects),
    Dissolved(DissolvedEffects),
}
#[derive(Clone, Debug)]
pub struct Prepared {
    pub world: pirate_empty_band_end::Prepared,
    pub scene_manager: Vec<u8>,
    pub slot: u8,
    pub return_effects: Option<ConvoyReturn>,
    pub sync_time: bool,
    pub release_collision_index: bool,
}
/// El caller descarta la escena y su índice representado por separado al
/// publicar todo. Las asignaciones x86 sin representación se rechazan.
pub fn prepare(
    w: &TickWorld,
    bands: &AbsorptionState,
    index: usize,
    scheduler: &Scheduler,
    scene_manager: &[u8],
    scene: &[u8],
    address: u32,
    network_client: bool,
    convoy_ui: bool,
    gui_present: bool,
    route_pool_limit: u16,
    rng: CrtRand,
) -> Result<Prepared, String> {
    if scene.len() != 0x2f24
        || scene_manager.len() < 0x405
        || address == 0
        || r32(scene, 0) != 0x67aaa0
    {
        return Err("escena o gestor incompletos".into());
    }
    let slot = r16(scene, 0x662);
    if slot >= 255 || r32(scene_manager, usize::from(slot) * 4) != address {
        return Err("ranura de escena desincronizada".into());
    }
    if scene[0x1e98] != 254u8.checked_sub(index as u8).ok_or("banda inválida")? || index >= 5 {
        return Err("escena de otra banda".into());
    }
    if r16(scene, 0x14) != 0
        || r16(scene, 0x10) != u16::MAX
        || r16(scene, 0x65a) != 0
        || r16(scene, 0x65c) != u16::MAX
        || r16(scene, 0xacc) != 0
        || r16(scene, 0x2f20) != 0
        || r32(scene, 4) != 0
        || r32(scene, 0x18) != 0
        || r32(scene, 0x66c) != 0
        || r32(scene, 0x2f1c) != 0
    {
        return Err("asignaciones o defensores de escena no representados".into());
    }
    if gui_present && scene_manager[0x3fc] == slot as u8 {
        return Err("cierre de ventana activa pendiente".into());
    }
    let count = usize::from(scene[0x558]);
    if count > 255 || 0x5d + count > scene.len() {
        return Err("participantes de escena truncados".into());
    }
    let sync_time = !network_client && (0..count).any(|i| scene[0x5d + i] == 0);
    let mut scene_manager = scene_manager.to_vec();
    let p = usize::from(slot) * 4;
    scene_manager[p..p + 4].fill(0);
    while scene_manager[0x404] != 0 {
        let n = usize::from(scene_manager[0x404] - 1);
        if r32(&scene_manager, n * 4) != 0 {
            break;
        }
        scene_manager[0x404] -= 1
    }
    if scene[0x12ea] != 0 {
        scene_manager[0x3fd] = 0
    }
    let mut manager = w.remove.manager.to_vec();
    let mut ships = w.remove.ships.to_vec();
    let mut caps = w.remove.captains.to_vec();
    let mut convoys = w.remove.convoys.to_vec();
    let mut merchants = w.remove.merchants.to_vec();
    let mut towns = w.remove.towns.to_vec();
    let mut returned = None;
    if scene[0x5c] != 0 && r16(scene, 0x15c) == 0 {
        if r16(scene, 0x35c) != 0xff || w.remove.merchant_count > 255 {
            return Err("participantes de retorno no soportados".into());
        }
        let cv = r16(scene, 0x65e);
        let count = if manager.len() >= 0xf8 {
            r16(&manager, 0xf6)
        } else {
            return Err("gestor naval truncado".into());
        };
        if cv >= count || usize::from(cv) * 0x3c + 0x3c > convoys.len() {
            return Err("convoy atacante ausente".into());
        }
        let cp = usize::from(cv) * 0x3c;
        let ns = r16(&manager, 0xf4);
        let hanse = pirate_engage::is_hanse_expedition(&convoys[cp..cp + 0x3c], cv, scheduler)
            .map_err(str::to_owned)?;
        let dissolve = (u16::from(convoys[cp]) < w.remove.merchant_count
            && convoys[cp + 0x14] & 0x10 != 0)
            || r16(&convoys, cp + 0xa) >= ns;
        returned = Some(if dissolve {
            ConvoyReturn::Dissolved(
                return_convoy::return_dissolved(
                    &mut manager,
                    &mut ships,
                    &mut convoys,
                    &mut merchants,
                    cv,
                    255,
                    w.player,
                    w.remove.town_count,
                    route_pool_limit,
                    hanse,
                )
                .map_err(str::to_owned)?,
            )
        } else {
            ConvoyReturn::Preserved(
                return_convoy::return_preserved(
                    &mut manager,
                    &mut ships,
                    &mut convoys,
                    cv,
                    w.remove.merchant_count,
                    255,
                    w.player,
                    route_pool_limit,
                    hanse,
                )
                .map_err(str::to_owned)?,
            )
        });
    }
    let mut remove = RemoveWorld {
        manager: &mut manager,
        ships: &mut ships,
        captains: &mut caps,
        convoys: &mut convoys,
        merchants: &mut merchants,
        towns: &mut towns,
        pool_limit: w.remove.pool_limit,
        merchant_count: w.remove.merchant_count,
        town_count: w.remove.town_count,
        ui_a: w.remove.ui_a,
        ui_b: w.remove.ui_b,
    };
    let staged = TickWorld {
        remove: &mut remove,
        time: w.time,
        player: w.player,
    };
    let world = pirate_empty_band_end::prepare(
        &staged,
        bands,
        index,
        scheduler,
        network_client,
        convoy_ui,
        rng,
    )?;
    Ok(Prepared {
        world,
        scene_manager,
        slot: slot as u8,
        return_effects: returned,
        sync_time,
        release_collision_index: true,
    })
}

/// Paso de reloj terminado; el destructor se ejecuta sincrónicamente cuando
/// el temporizador vence. Los efectos de UI aún requieren al anfitrión.
#[derive(Clone, Debug)]
pub enum FinishedFrame {
    Active {
        scene: Vec<u8>,
        scene_manager: Vec<u8>,
        effects: Vec<p3_core::ships::town_attack_frame::FinishedAttackEffect>,
    },
    Destroyed {
        result: Prepared,
        effects: Vec<p3_core::ships::town_attack_frame::FinishedAttackEffect>,
    },
}
pub fn prepare_finished_frame(
    w: &TickWorld,
    bands: &AbsorptionState,
    index: usize,
    scheduler: &Scheduler,
    scene_manager: &[u8],
    scene: &[u8],
    address: u32,
    delta: u32,
    ui_active: bool,
    network_client: bool,
    convoy_ui: bool,
    gui_present: bool,
    route_pool_limit: u16,
    rng: CrtRand,
    static_terrain: bool,
) -> Result<FinishedFrame, String> {
    if scene_manager.len() < 0x405
        || scene.len() != 0x2f24
        || address == 0
        || r32(scene, 0) != 0x67aaa0
    {
        return Err("escena o gestor truncado".into());
    }
    let slot = r16(scene, 0x662);
    if slot >= 255
        || r32(scene_manager, usize::from(slot) * 4) != address
        || index >= 5
        || scene[0x1e98] != 254 - index as u8
    {
        return Err("ranura o banda de escena desincronizada".into());
    }
    let mut object = scene.to_vec();
    let mut manager = scene_manager.to_vec();
    let mut step = p3_core::ships::town_attack_frame::finished_attack_prefix(
        &mut object,
        address,
        delta,
        manager[0x3fd],
        ui_active,
    )
    .map_err(str::to_owned)?;
    manager[0x3fd] = step.dialog_state;
    if step.destroy_requested {
        if step.effects.pop()
            != Some(
                p3_core::ships::town_attack_frame::FinishedAttackEffect::Destroy { argument: 1 },
            )
        {
            return Err("destructor de frame inconsistente".into());
        }
        let result = prepare(
            w,
            bands,
            index,
            scheduler,
            &manager,
            &object,
            address,
            network_client,
            convoy_ui,
            gui_present,
            route_pool_limit,
            rng,
        )?;
        Ok(FinishedFrame::Destroyed {
            result,
            effects: step.effects,
        })
    } else {
        // Tras el prefijo el original sigue simulando cada tick del frame.
        // Esta ruta solo acepta la escena sin barcos, proyectiles ni extremos
        // espaciales; las demás necesitan el simulador completo de asalto.
        if delta as u16 != 0 {
            if r16(&object, 0x15c) > 0xf000
                || r16(&object, 0x14) != 0 || r16(&object, 0x10) != u16::MAX
                || r16(&object, 0x65a) != 0 || r16(&object, 0x65c) != u16::MAX
                || r16(&object, 0xacc) != 0 || r16(&object, 0x2f20) != 0
                || (!static_terrain && (r16(&object, 0x34) != u16::MAX || r16(&object, 0x44) != u16::MAX))
                || object[0x1ec4..0x1ed4].iter().any(|b| *b != 0)
                || (object[0x12e0] != 0 && r16(&object, 0xad0) != 0)
            { return Err("continuación de asalto con objetos aún no simulada".into()); }
            for _ in 0..delta as u16 {
                for offset in [0xa90usize, 0xa94, 0xaa0, 0xaa4, 0xab4, 0xab8, 0xabc, 0xac0] {
                    object[offset..offset + 4].fill(0);
                }
                object[0xa98..0xa9c].fill(0);
                for offset in [0xab4usize, 0xab8] { object[offset..offset + 4].copy_from_slice(&0x880u32.to_le_bytes()); }
                for offset in [0xabcusize, 0xac0] { object[offset..offset + 4].copy_from_slice(&0x781u32.to_le_bytes()); }
                object[0xa8c] = object[0xa8c].wrapping_add(1);
                if u32::from(object[0xa8c]) * 40 >= 1000 { object[0xa8c] = 0; }
                let wind = p3_core::ships::wind::update_wind(&mut object, manager[0x3fc], true);
                if wind.refresh_drift { return Err("deriva visual del asalto pendiente".into()); }
            }
        }
        Ok(FinishedFrame::Active {
            scene: object,
            scene_manager: manager,
            effects: step.effects,
        })
    }
}
