//! Contexto de memoria del original necesario para el lanzamiento B5.
use p3_core::ai::settlement::SettlementLocation;
use p3_save::tail::RouteStopPool;

/// Solo construye imágenes a partir de los recursos y el estado vivos de la
/// sesión. Las direcciones auxiliares no se publican en el guardado.
pub fn extra_memory(
    locations: &[SettlementLocation; 40],
    difficulty: u8,
    alderman_town: u8,
    route_stop_pool: &[u8],
) -> Result<Vec<(u32, Vec<u8>)>, String> {
    const SETTINGS: u32 = 0x5e00_0000;
    let pool = RouteStopPool::parse(route_stop_pool)
        .map_err(|error| format!("depósito de rutas B5: {error:?}"))?;
    let mut memory = vec![
        // La dirección solo identifica una invalidación de interfaz; la
        // sesión sin pantalla registra el evento y no desreferencia el objeto.
        (0x006c_bdd8, 0x5f00_0000u32.to_le_bytes().to_vec()),
        (0x006c_c3e8, SETTINGS.to_le_bytes().to_vec()),
        (SETTINGS + 0x12, vec![difficulty]),
        (0x006d_e52e, vec![alderman_town]),
        (0x006d_d72a, pool.count.to_le_bytes().to_vec()),
    ];
    for (i, location) in locations.iter().enumerate() {
        let mut position = location.x.to_le_bytes().to_vec();
        position.extend_from_slice(&location.y.to_le_bytes());
        memory.push((0x006d_dbb0 + i as u32 * 0x34, position));
    }
    Ok(memory)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_original_addresses_from_live_resources() {
        let mut locations = [SettlementLocation { x: 0, y: 0, kind: 0 }; 40];
        locations[3] = SettlementLocation { x: -17, y: 0x1234, kind: 7 };
        let mut pool = vec![0; 4 + 2 * 220];
        pool[..4].copy_from_slice(&[2, 0, 2, 0]);
        let memory = extra_memory(&locations, 2, 4, &pool).unwrap();
        assert_eq!(memory.iter().find(|(p, _)| *p == 0x6dd72a).unwrap().1, [2, 0]);
        assert_eq!(memory.iter().find(|(p, _)| *p == 0x6de52e).unwrap().1, [4]);
        let position = &memory.iter().find(|(p, _)| *p == 0x6ddbb0 + 3 * 0x34).unwrap().1;
        assert_eq!(&position[..4], &(-17i32).to_le_bytes());
        assert_eq!(&position[4..8], &0x1234i32.to_le_bytes());
        assert!(extra_memory(&locations, 2, 4, &[0]).is_err());
    }
}
