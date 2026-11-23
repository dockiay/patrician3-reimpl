//! `0x00598150` (`Class22::make_walkers_flee`): al empezar el asalto, los transeúntes de la vista de la ciudad dan media vuelta y corren. Recorre los
//! cuatro grupos (índice `g`), cuya lista de punteros está en `vista+0xC5A0+4g`, el primer índice en `vista+0xC5B8+2g`, el recuento en
//! `vista+0xC5C0+2g` y los altas pendientes en `vista+0xC5C8+2g` (se suman al recuento y se ponen a 0). A cada figura le pasa a
//! `0x0055C4A0` (`TownPathMover::turn_back_and_run`) la rapidez del conjunto de sprites de su tipo (`+0x78`): la tabla del grupo `g` (cuyo recuento y puntero
//! son globales distintos por grupo), o 0 si el tipo no está en la tabla.
use crate::flatmem::Mem;

/// (recuento de tipos, tabla de rapideces) de cada grupo.
const GROUP_TABLES: [(u32, u32); 4] = [(0x006e_1710, 0x006e_1714), (0x006e_176c, 0x006e_1770), (0x006e_178c, 0x006e_1790), (0x006e_177c, 0x006e_1780)];

pub trait FleeHost {
    /// `0x0055C4A0` (`thiscall` sobre la figura, un argumento: la rapidez).
    fn turn_back_and_run(&mut self, m: &mut dyn Mem, walker: u32, speed: u32);
}

pub fn make_walkers_flee(m: &mut dyn Mem, host: &mut dyn FleeHost, view: u32) {
    for (g, (types_count, types_table)) in GROUP_TABLES.iter().enumerate() {
        let g = g as u32;
        let slot = view.wrapping_add(0xc5c0).wrapping_add(2 * g);
        let added = m.r16(slot + 8);
        let first = m.r16(slot.wrapping_sub(8));
        let count = m.r16(slot).wrapping_add(added);
        m.w16(slot, count);
        m.w16(slot + 8, 0);
        if count == 0 {
            continue;
        }
        let mut index = u32::from(first);
        loop {
            let list = m.r32(view.wrapping_add(0xc5a0).wrapping_add(4 * g));
            let walker = m.r32(list.wrapping_add(index * 4));
            let kind = u32::from(m.r8(walker.wrapping_add(0x78)));
            let speed = if kind >= m.r32(*types_count) {
                0
            } else {
                let table = m.r32(*types_table);
                m.r32(table.wrapping_add(kind * 4))
            };
            host.turn_back_and_run(m, walker, speed);
            index = (index + 1) & 0xffff;
            // Tras cada figura el original vuelve a leer recuento e inicio (palabras).
            let limit = u32::from(m.r16(slot)) + u32::from(m.r16(slot.wrapping_sub(8)));
            if index >= limit {
                break;
            }
        }
    }
}
