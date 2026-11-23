//! Final acotado de ataque a guarida vacía (`0x005FE7E0`).
use super::{
    battle::advance_battle_rng,
    finish::{format, Arg},
    r16, r32, w16, w32,
};

pub struct EmptyHideoutFinish<'a> {
    pub convoy_owner: u8,
    pub convoy_count: u16,
    pub defender_convoy_empty: bool,
    pub convoy_ship_owners: &'a [u8],
    pub band_level: u16,
    pub fallback_town: u8,
    pub report_flag: u8,
    pub first_name: &'a [u8],
    pub last_name: &'a [u8],
    pub format: &'a [u8],
    pub own_separator: &'a [u8],
    pub enemy_separator: &'a [u8],
}
fn c_prefix(value: &[u8]) -> &[u8] {
    &value[..value.iter().position(|&b| b == 0).unwrap_or(value.len())]
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinishMessage {
    pub recipient: u32,
    pub descriptor: [u8; 16],
    pub text: Option<Vec<u8>>,
}
/// Prepara la rama de victoria directa, ambos bandos IA y sin barcos locales.
/// Devuelve peticiones de cartas, sin insertarlas en el depósito ni resolver
/// bajas, UI, convoyes o efectos de otros estados de combate.
pub fn prepare_empty_hideout_finish(
    object: &mut [u8],
    merchants: &mut [u8],
    merchant_count: u16,
    v: &EmptyHideoutFinish<'_>,
) -> Result<Vec<FinishMessage>, &'static str> {
    if object.len() < 0x2f24 || merchants.len() < usize::from(merchant_count) * 0x650 {
        return Err("batalla o comerciantes truncados");
    }
    if v.band_level == 0 {
        return Err("recompensa de guarida con divisor cero");
    }
    if object[0x5d] != 1
        || object[0x5e] != 1
        || object[0xac8] == 0
        || object[0xac4] != 3
        || r16(object, 0x65e) >= v.convoy_count
        || (r16(object, 0x660) != u16::MAX && (!v.defender_convoy_empty || r16(object, 0x660) >= v.convoy_count))
        || r16(object, 0x15c) != u16::MAX
        || r16(object, 0x14) != 0
        || r16(object, 0x10) != u16::MAX
        || r16(object, 0x65c) != u16::MAX
        || object[0x558] != 2
        || object[0x559] != 0
        || object[0x55a] != 1
        || !(250..=254).contains(&object[0x1e98])
    {
        return Err("final de guarida fuera del caso contrastado");
    }
    let receiver = usize::from(r16(object, 0x35a));
    if receiver >= usize::from(merchant_count) {
        return Err("receptor de informe ausente");
    }
    let mut battle = object.to_vec();
    let mut people = merchants.to_vec();
    let next = advance_battle_rng(r32(&battle, 0xadc) as u32);
    let level = u32::from(v.band_level);
    let reward = 50u32.wrapping_mul(level.wrapping_mul(2).wrapping_add(next % level));
    let mut text = format(
        c_prefix(v.format),
        &[
            Arg::S(c_prefix(v.first_name)),
            Arg::S(c_prefix(v.last_name)),
            Arg::D(reward as i32),
        ],
    )?;
    text.extend_from_slice(c_prefix(v.own_separator));
    text.extend_from_slice(c_prefix(v.enemy_separator));
    if text.len() > u16::MAX as usize {
        return Err("informe de guarida excesivo");
    }
    let owner = usize::from(v.convoy_owner);
    let town = if owner < usize::from(merchant_count) {
        let base = owner * 0x650;
        for at in [0, 0x4b8] {
            let value = (r32(&people, base + at) as u32).wrapping_add(reward);
            w32(&mut people, base + at, value as i32)
        }
        people[base + 0x19]
    } else {
        v.fallback_town
    };
    let mut news = [0u8; 16];
    news[..4].copy_from_slice(&[0, 0x80, 0xff, 0xff]);
    news[4] = 0x3a;
    news[5] = town;
    news[8..12].copy_from_slice(&(reward | u32::from(battle[0x1e90]) << 24).to_le_bytes());
    let receiver_base = receiver * 0x650;
    let home = usize::from(people[receiver_base + 0x19]);
    news[12] = 0x80 | v.report_flag.wrapping_shl(4) | people[receiver_base + 0x39c + home];
    news[13] = people[receiver_base + 0x1c];
    news[14] = people[receiver_base + 0x1d];
    news[15] = battle[0x1e91];
    let mut participants = vec![r16(&battle, 0x35a), r16(&battle, 0x35c)];
    for &id in v.convoy_ship_owners {
        let id = u16::from(id);
        if participants.contains(&id) {
            continue;
        }
        let at = participants.len();
        if at >= 255 {
            return Err("participantes de convoy excesivos");
        };
        w16(&mut battle, 0x35a + 2 * at, id);
        battle[0x5d + at] = 1;
        battle[0x559 + at] = 0;
        participants.push(id);
    }
    w32(&mut battle, 0xadc, next as i32);
    battle[0x5c] = 1;
    w16(&mut battle, 0x15c, 1);
    battle[0x130e] = 0;
    let mut result = vec![FinishMessage {
        recipient: u32::MAX - 1,
        descriptor: news,
        text: None,
    }];
    for (i, &id) in participants.iter().enumerate() {
        let receiver = usize::from(id);
        if receiver >= usize::from(merchant_count) || battle[0x5d + i] == 0 {
            continue;
        }
        let mut report = [0u8; 16];
        report[..4].copy_from_slice(&[0, 0x80, 0xff, 0xff]);
        report[4] = 0x29;
        report[5] = people[receiver * 0x650 + 0x19];
        report[12..16].copy_from_slice(&(text.len() as u32).to_le_bytes());
        result.push(FinishMessage {
            recipient: u32::from(id),
            descriptor: report,
            text: Some(text.clone()),
        });
    }
    object.copy_from_slice(&battle);
    merchants.copy_from_slice(&people);
    Ok(result)
}
/// Datos del diálogo de victoria cuando el atacante es el jugador humano.
pub struct HumanHideoutFinish<'a> {
    pub base: EmptyHideoutFinish<'a>,
    pub player: u16,
    pub battle_address: u32,
    pub dialog_active: u8,
    pub simple_text: &'a [u8],
    pub reward_format: &'a [u8],
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HumanFinishEffect {
    Letter(FinishMessage),
    OpenDialog { address: u32 },
}
pub struct HumanFinishResult {
    pub effects: Vec<HumanFinishEffect>,
    pub dialog_active: u8,
}
/// Variante de la victoria sin defensores para un atacante humano local.
/// El diálogo se describe con una dirección lógica; el llamante lo muestra.
pub fn prepare_human_hideout_finish(
    object: &mut [u8],
    merchants: &mut [u8],
    merchant_count: u16,
    v: &HumanHideoutFinish<'_>,
) -> Result<HumanFinishResult, &'static str> {
    if object.len() < 0x2f24 || object[0x5d] != 0 || object[0x5e] != 1 || r16(object, 0x35a) != v.player {
        return Err("victoria humana fuera del caso contrastado");
    }
    let address = v
        .battle_address
        .checked_add(0x1310)
        .ok_or("dirección de diálogo desbordada")?;
    let dialog = v
        .battle_address
        .checked_add(0x12ec)
        .ok_or("dirección de diálogo desbordada")?;
    let mut battle = object.to_vec();
    let mut people = merchants.to_vec();
    let mode = battle[0xac8];
    battle[0x5d] = 1;
    battle[0xac8] = 1;
    let base = EmptyHideoutFinish {
        convoy_owner: v.base.convoy_owner,
        convoy_count: v.base.convoy_count,
        defender_convoy_empty: v.base.defender_convoy_empty,
        convoy_ship_owners: v.base.convoy_ship_owners,
        band_level: v.base.band_level,
        fallback_town: v.base.fallback_town,
        report_flag: v.base.report_flag,
        first_name: v.base.first_name,
        last_name: v.base.last_name,
        format: v.base.format,
        own_separator: v.base.own_separator,
        enemy_separator: v.base.enemy_separator,
    };
    let mut events = prepare_empty_hideout_finish(&mut battle, &mut people, merchant_count, &base)?;
    let news = events.remove(0);
    let amount = u32::from_le_bytes(news.descriptor[8..12].try_into().unwrap()) & 0x00ff_ffff;
    let text = if r16(&battle, 0x1e94) == 0 {
        c_prefix(v.simple_text).to_vec()
    } else {
        format(c_prefix(v.reward_format), &[Arg::D(amount as i32)])?
    };
    if text.len() >= 0xb80 {
        return Err("texto del diálogo fuera de rango");
    }
    battle[0x5d] = 0;
    battle[0xac8] = mode;
    w16(&mut battle, 0x15c, if mode == 0 { 250 } else { 1 });
    battle[0x1310..0x1310 + text.len()].copy_from_slice(&text);
    battle[0x1310 + text.len()] = 0;
    w32(&mut battle, 0x12ec, address as i32);
    battle[0x12f0] = 1;
    w16(&mut battle, 0x12f2, 0);
    w32(&mut battle, 0x12f4, 0);
    battle[0x12f8..0x12fc].copy_from_slice(&[0xff, 0, 0, 0xff]);
    w32(&mut battle, 0x12fc, 0x3f3504f4);
    battle[0x1300] = 1;
    w16(&mut battle, 0x1302, 0);
    w32(&mut battle, 0x1304, 0);
    w32(&mut battle, 0x1308, 0);
    let mut effects = vec![HumanFinishEffect::Letter(news)];
    if v.dialog_active != 0 {
        battle[0x12e9] = 1
    } else {
        battle[0x12e9] = 0;
        battle[0x12ea] = 1;
        effects.push(HumanFinishEffect::OpenDialog { address: dialog })
    }
    effects.extend(
        events
            .into_iter()
            .filter(|m| m.recipient != u32::from(v.player))
            .map(HumanFinishEffect::Letter),
    );
    object.copy_from_slice(&battle);
    merchants.copy_from_slice(&people);
    Ok(HumanFinishResult {
        effects,
        dialog_active: if v.dialog_active == 0 { 1 } else { v.dialog_active },
    })
}
