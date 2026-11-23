//! Prefijo de update de ataque terminado `0x005FD510`.
use super::{r16, w16};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinishedAttackEffect {
    QueryDialog,
    OpenDialog { address: u32 },
    Destroy { argument: u32 },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinishedAttackStep {
    pub effects: Vec<FinishedAttackEffect>,
    pub dialog_state: u8,
    pub destroy_requested: bool,
}
/// Procesa diálogo y temporizador antes del resto del update. La consulta de UI
/// y el destructor se representan como efectos; el llamante debe ejecutarlos.
/// El byte `ui_active` es el resultado de la consulta original, si se realiza.
pub fn finished_attack_prefix(
    object: &mut [u8],
    address: u32,
    delta: u32,
    dialog_state: u8,
    ui_active: bool,
) -> Result<FinishedAttackStep, &'static str> {
    if object.len() < 0x2f1b || object[0x5c] != 1 || object[0x2f1a] != 0 {
        return Err("escena terminada fuera del caso contrastado");
    }
    let dialog = address.checked_add(0x12ec).ok_or("dirección de diálogo desbordada")?;
    let mut out = object.to_vec();
    let mut state = dialog_state;
    let mut effects = Vec::new();
    let ticks = delta as u16;
    if ticks != 0 {
        if out[0x12ea] != 0 {
            effects.push(FinishedAttackEffect::QueryDialog);
            if !ui_active {
                state = 0;
                out[0x12ea] = 0;
            }
        }
        if state == 0 && out[0x12e9] != 0 {
            effects.push(FinishedAttackEffect::OpenDialog { address: dialog });
            out[0x12e9] = 0;
            out[0x12ea] = 1;
            state = 1;
        }
        let remaining = r16(&out, 0x15c);
        if remaining <= ticks {
            w16(&mut out, 0x15c, 0);
            effects.push(FinishedAttackEffect::Destroy { argument: 1 });
        } else {
            w16(&mut out, 0x15c, remaining - ticks);
        }
    }
    let destroyed = effects
        .iter()
        .any(|e| matches!(e, FinishedAttackEffect::Destroy { .. }));
    object.copy_from_slice(&out);
    Ok(FinishedAttackStep {
        effects,
        dialog_state: state,
        destroy_requested: destroyed,
    })
}
