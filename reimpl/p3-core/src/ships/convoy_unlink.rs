//! Desenlace de convoy activo antes de una batalla (`0x0050A4E0`).
use super::order::XOrder;
const SHIP: usize = 0x180;
const CONVOY: usize = 0x3c;
fn word(v: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([v[o], v[o + 1]])
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unlinked {
    pub removed: bool,
    pub refresh_ship: Option<u16>,
}
/// Conserva las tres imágenes si hay cadenas cíclicas o datos truncados.
/// `refresh_ship` representa la llamada de interfaz original 0x0044F940.
pub fn unlink(manager: &mut [u8], ships: &mut [u8], convoys: &mut [u8], convoy: u32) -> Result<Unlinked, &'static str> {
    if manager.len() < 0xf8 {
        return Err("gestor naval truncado");
    };
    let sn = usize::from(word(manager, 0xf4));
    let cn = usize::from(word(manager, 0xf6));
    if ships.len() < sn * SHIP || convoys.len() < cn * CONVOY {
        return Err("pools navales truncados");
    }
    let mut m = manager.to_vec();
    let mut s = ships.to_vec();
    let mut c = convoys.to_vec();
    let mut current = word(&m, 0xec);
    let mut previous = None;
    let mut seen = vec![false; cn];
    loop {
        if u32::from(current) == convoy {
            if usize::from(current) >= cn {
                return Err("convoy coincidente fuera del pool");
            };
            let at = usize::from(current) * CONVOY;
            let next = word(&c, at + 8);
            match previous {
                Some(i) => c[i * CONVOY + 8..i * CONVOY + 10].copy_from_slice(&next.to_le_bytes()),
                None => m[0xec..0xee].copy_from_slice(&next.to_le_bytes()),
            };
            c[at + 8..at + 10].fill(255);
            let leader = word(&c, at + 0x10);
            let refresh_ship = if usize::from(leader) < sn {
                let mut order = XOrder {
                    ships: &mut s,
                    count: sn as u16,
                    sea_head: word(&m, 0xea),
                };
                order.remove(leader);
                Some(leader)
            } else {
                None
            };
            manager.copy_from_slice(&m);
            ships.copy_from_slice(&s);
            convoys.copy_from_slice(&c);
            return Ok(Unlinked {
                removed: true,
                refresh_ship,
            });
        }
        if usize::from(current) >= cn {
            return Ok(Unlinked {
                removed: false,
                refresh_ship: None,
            });
        }
        let i = usize::from(current);
        if seen[i] {
            return Err("lista de convoyes cíclica");
        }
        seen[i] = true;
        previous = Some(i);
        current = word(&c, i * CONVOY + 8);
    }
}
