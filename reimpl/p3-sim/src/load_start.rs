//! Arranque o reanudación de una partida cargada: opcode 0xC1 de `execute_operations` (0x00546870, caso en
//! 0x00546953). Tras cargar, el original pone el jugador local `0x006DFC14` a 0xFFFF y recorre los
//! `0x006DE4AA` comerciantes: cada uno con palabra de control `+8 == 0` (humano) lo sustituye, así que queda
//! el último humano. Sin humanos se conserva 0xFFFF.
//!
//! El resto del caso (marcas de `Operations`, `schedule_human_merchant_reminders` 0x0054A420) no se modela aquí.
use p3_core::economy::world::Merchant;

/// Valor de `0x006DFC14` tras el opcode 0xC1.
pub(crate) fn local_player(merchants: &[Merchant]) -> u32 {
    let mut local = 0xffff;
    for (i, merchant) in merchants.iter().enumerate().take(usize::from(u16::MAX)) {
        if merchant.kind == 0 {
            local = (i & 0xff) as u32;
        }
    }
    local
}

#[cfg(test)]
mod tests {
    use super::*;
    fn m(kind: u16) -> Merchant {
        Merchant { gold: 0, kind, home_town: 0, income: 0 }
    }
    #[test]
    fn last_human_wins_and_default_is_ffff() {
        assert_eq!(local_player(&[]), 0xffff);
        assert_eq!(local_player(&[m(1), m(8)]), 0xffff);
        assert_eq!(local_player(&[m(0), m(1), m(0), m(8)]), 2);
    }
    /// Fixture de `oracle/x86_load_start.py` (600 casos del tramo original).
    #[test]
    fn matches_x86_fixture() {
        let text = include_str!("../tests/fixtures/load_start.json");
        let doc: serde_json::Value = serde_json::from_str(text).unwrap();
        let cases = doc["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 600);
        for (i, case) in cases.iter().enumerate() {
            let merchants = case["kinds"].as_array().unwrap().iter().map(|k| m(k.as_u64().unwrap() as u16)).collect::<Vec<_>>();
            assert_eq!(u64::from(local_player(&merchants)), case["local"].as_u64().unwrap(), "caso {i}");
        }
    }
}
