//! Llegada del convoy de caza (`0x004E09D4`, fase 10, tramo `0x004E19F1..0x004E1B36`) y la lista de bajas que cuelga del
//! registro (`+8`): constructores `0x004F3980`/`0x004F3AD0` y destructor recursivo `0x004F39A0` de un nodo de 8 bytes
//! (`+0` hijo, `+4` clase, `+5` marca, `+6` id de texto). Traducción sobre la memoria del original ([`Mem`]).
use crate::{flatmem::Mem, scheduler::TaskRecord};

pub trait ArrivalHost {
    /// `operator new` (0x0064F7B9).
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    /// `operator delete` (0x0064F7E2).
    fn free(&mut self, p: u32);
    /// `send_pirate_hunt_report_message` (0x004F3AF0, ECX = nodo): `(comerciante, +0xC, +0xD, +0xE del registro)`.
    fn report(&mut self, m: &mut dyn Mem, node: u32, merchant: u32, a2: u32, a3: u32, a4: u32);
}

/// `0x004F3980`.
pub fn node_default(m: &mut dyn Mem, p: u32) {
    m.w32(p, 0);
    m.w8(p + 4, 0xff);
    m.w8(p + 5, 0xff);
    m.w16(p + 6, 0xffff);
}

/// `0x004F3AD0` (`ret 0x10`): argumentos de pila `(marca, clase, id de texto, hijo)`.
pub fn node_init(m: &mut dyn Mem, p: u32, mark: u8, class: u8, text: u16, child: u32) {
    m.w32(p, child);
    m.w8(p + 4, class);
    m.w8(p + 5, mark);
    m.w16(p + 6, text);
}

/// `0x004F39A0`: destruye la cadena de hijos (no libera el propio nodo, lo hace el llamante).
pub fn node_destroy(m: &mut dyn Mem, h: &mut dyn ArrivalHost, p: u32) {
    let child = m.r32(p);
    if child != 0 {
        m.w32(p, 0);
        node_destroy(m, h, child);
        h.free(child);
    }
}

/// Estado que el tramo lee fuera del registro.
pub struct ArrivalWorld<'a> {
    /// Un comerciante es humano si su palabra `+8` es 0.
    pub humans: &'a [bool],
    /// `0x006DD894`.
    pub ship_count: u16,
    /// Byte `0x00672858` (clase del nodo raíz creado cuando no hay lista).
    pub class_byte: u8,
}

/// Informe a cada humano, liberación de la lista, convoy a estado 0xF y barcos en estado 0x16 a 0xF.
pub fn hunt_arrive(
    r: &mut TaskRecord,
    convoy: &mut [u8],
    ships: &mut [Option<Vec<u8>>],
    w: &ArrivalWorld,
    m: &mut dyn Mem,
    h: &mut dyn ArrivalHost,
) -> Result<(), String> {
    if convoy.len() < 0x1c {
        return Err("convoy truncado".into());
    }
    if r.u32_at(8) == 0 {
        let node = h.alloc(m, 8);
        for k in 0..8 {
            m.w8_seed(node + k);
        }
        node_default(m, node);
        node_init(m, node, 0xff, w.class_byte, 0, 0);
        r.set_u32_at(8, node);
    }
    let node = r.u32_at(8);
    for (i, human) in w.humans.iter().enumerate() {
        if *human {
            h.report(
                m,
                node,
                i as u32,
                u32::from(r.u8_at(0xc)),
                u32::from(r.u8_at(0xd)),
                u32::from(r.u8_at(0xe)),
            );
        }
    }
    let node = r.u32_at(8);
    if node != 0 {
        node_destroy(m, h, node);
        h.free(node);
    }
    r.set_u32_at(8, 0);
    let flags = u32::from_le_bytes(convoy[0x18..0x1c].try_into().unwrap()) | 0x20;
    convoy[0x18..0x1c].copy_from_slice(&flags.to_le_bytes());
    convoy[0x12..0x14].copy_from_slice(&0xfu16.to_le_bytes());
    let mut at = u16::from_le_bytes([convoy[0xa], convoy[0xb]]);
    let mut steps = 0usize;
    while at < w.ship_count {
        steps += 1;
        if steps > usize::from(w.ship_count) + 1 {
            return Err("cadena de barcos del convoy cíclica".into());
        }
        let ship = ships
            .get_mut(usize::from(at))
            .and_then(Option::as_mut)
            .ok_or("barco del convoy ausente")?;
        if ship.len() < 0x180 {
            return Err("barco del convoy truncado".into());
        }
        if u16::from_le_bytes([ship[0x134], ship[0x135]]) == 0x16 {
            let dirty = u16::from_le_bytes([ship[0x3c], ship[0x3d]]) | 0x20;
            ship[0x3c..0x3e].copy_from_slice(&dirty.to_le_bytes());
            ship[0x134..0x136].copy_from_slice(&0xfu16.to_le_bytes());
        }
        at = u16::from_le_bytes([ship[6], ship[7]]);
    }
    Ok(())
}
