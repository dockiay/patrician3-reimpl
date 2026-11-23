//! Desplazamiento de la flota antes de atacar un puerto o escondite (`0x0050E280`).
const SHIP_SIZE: usize = 0x180;
const CONVOY_SIZE: usize = 0x3c;
fn u16_at(v: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([v[at], v[at + 1]])
}
/// Modifica copias y las publica únicamente si la cadena es válida. `direction`
/// procede de `StadtDaten[destino]+0x11`; se reproduce también el caso >7.
pub fn nudge(manager: &[u8], ships: &mut [u8], convoys: &[u8], convoy: u32, direction: u8) -> Result<(), &'static str> {
    if manager.len() < 0xf8 {
        return Err("gestor naval truncado");
    }
    let n = usize::from(u16_at(manager, 0xf4));
    let nc = usize::from(u16_at(manager, 0xf6));
    if ships.len() < n * SHIP_SIZE || convoys.len() < nc * CONVOY_SIZE {
        return Err("pools navales truncados");
    }
    let id = usize::from(convoy as u16);
    if id >= nc {
        return Ok(());
    }
    let x = if direction > 4 {
        u32::MAX - 0xffff
    } else if direction > 0 && direction < 4 {
        0x10000
    } else {
        0
    };
    let y = if direction < 2 || direction == 7 {
        u32::MAX - 0xffff
    } else if (3..6).contains(&direction) {
        0x10000
    } else {
        0
    };
    let mut next = usize::from(u16_at(convoys, id * CONVOY_SIZE + 0xa));
    let mut seen = vec![false; n];
    let mut out = ships.to_vec();
    while next < n {
        if seen[next] {
            return Err("cadena de convoy cíclica");
        }
        seen[next] = true;
        let at = next * SHIP_SIZE;
        for (offset, delta) in [(0x1c, x), (0x20, y)] {
            let v = u32::from_le_bytes(out[at + offset..at + offset + 4].try_into().unwrap()).wrapping_add(delta);
            out[at + offset..at + offset + 4].copy_from_slice(&v.to_le_bytes())
        }
        next = usize::from(u16_at(&out, at + 6));
    }
    ships.copy_from_slice(&out);
    Ok(())
}
