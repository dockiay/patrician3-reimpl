//! Preparación del convoy atacante en la fase 1 de `0x004EAFE4`.
use super::{convoy_nudge, convoy_unlink, order::XOrder};
use crate::{ai::settlement::SettlementLocation, scheduler::TaskRecord};
const SHIP: usize = 0x180;
const CONVOY: usize = 0x3c;
fn word(v: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([v[at], v[at + 1]])
}
#[derive(Debug, Clone)]
pub struct Attack {
    pub ship: u16,
    pub convoy: u16,
    pub head: u16,
    pub leader: u16,
    pub owner: u8,
    pub task: TaskRecord,
    pub manager: Vec<u8>,
    pub ships: Vec<u8>,
    pub convoys: Vec<u8>,
    pub refresh_ship: Option<u16>,
}
impl Attack {
    /// `0x4EAFE4` recuerda el barco solo si abre una escena nueva; al reforzar
    /// una escena existente conserva el valor previo.
    pub fn task_after_scene(&self, new_scene: bool) -> TaskRecord {
        let mut t = self.task;
        if new_scene {
            t.set_u8_at(0x14, self.ship as u8);
            t.set_u8_at(0x15, (self.ship >> 8) as u8)
        }
        t
    }
}
#[derive(Debug, Clone)]
pub enum Prepared {
    Wait(TaskRecord),
    Attack(Attack),
    /// Barco suelto: el llamador debe ejecutar la extracción y creación reales.
    SingleShip {
        task: TaskRecord,
        ship: u16,
        output_initial: u16,
    },
}
/// Prepara copias. El caller debe completar la guarida/escena antes de publicar.
/// El caso de barco sin convoy queda rechazado hasta conectar su creación real.
pub fn prepare(
    task: &TaskRecord,
    targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
    direction: u8,
    head: u16,
    manager: &[u8],
    ships: &[u8],
    convoys: &[u8],
) -> Result<Option<Prepared>, &'static str> {
    prepare_inner(
        task, targets, locations, direction, head, manager, ships, convoys, false, None,
    )
}

/// Como `prepare`, pero devuelve un barco suelto si se aporta el contenido
/// previo de la variable local que el original pasa a 0x5062F0. Si un barco
/// previo ya la escribió se usa ese valor, independientemente del inicial.
pub fn prepare_with_single(
    task: &TaskRecord,
    targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
    direction: u8,
    head: u16,
    manager: &[u8],
    ships: &[u8],
    convoys: &[u8],
    output_initial: Option<u16>,
) -> Result<Option<Prepared>, &'static str> {
    prepare_inner(
        task,
        targets,
        locations,
        direction,
        head,
        manager,
        ships,
        convoys,
        true,
        output_initial,
    )
}

fn prepare_inner(
    task: &TaskRecord,
    targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
    direction: u8,
    head: u16,
    manager: &[u8],
    ships: &[u8],
    convoys: &[u8],
    allow_single: bool,
    mut output_initial: Option<u16>,
) -> Result<Option<Prepared>, &'static str> {
    if task.kind != 0x32
        || task.u8_at(0xc) != 3
        || task.u8_at(0xf) == 0
        || task.u8_at(0x12) != 1
        || task.due >= task.u32_at(8)
    {
        return Ok(None);
    }
    let target = task.u8_at(0x11);
    if targets.get(usize::from(task.u8_at(0x10))).copied().flatten() != Some(target) {
        return Ok(None);
    }
    let loc = locations.get(usize::from(target)).ok_or("destino pirata ausente")?;
    if manager.len() < 0xf8 {
        return Err("gestor naval truncado");
    };
    let sn = usize::from(word(manager, 0xf4));
    let cn = usize::from(word(manager, 0xf6));
    if ships.len() < sn * SHIP || convoys.len() < cn * CONVOY {
        return Err("pools navales truncados");
    }
    let mut t = *task;
    let tracked = usize::from(u16::from_le_bytes([t.u8_at(0x14), t.u8_at(0x15)]));
    let position = |id: usize| {
        let at = id * SHIP;
        (
            i32::from(i16::from_le_bytes([ships[at + 0x1e], ships[at + 0x1f]])),
            i32::from(i16::from_le_bytes([ships[at + 0x22], ships[at + 0x23]])),
        )
    };
    if tracked < sn && position(tracked) != (loc.x, loc.y) {
        t.set_u8_at(0x14, 255);
        t.set_u8_at(0x15, 255)
    }
    let mut delay = 0x1fu32;
    let mut id = usize::from(head);
    let mut seen = vec![false; sn];
    while id < sn {
        if seen[id] {
            return Err("barcos de comerciante cíclicos");
        }
        seen[id] = true;
        let at = id * SHIP;
        let next = usize::from(word(ships, at + 4));
        let tracked = usize::from(u16::from_le_bytes([t.u8_at(0x14), t.u8_at(0x15)]));
        if id != tracked && word(ships, at + 0x134) == 0xf {
            let (x, y) = position(id);
            let dx = x.wrapping_sub(loc.x);
            let dy = y.wrapping_sub(loc.y);
            let distance = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
            if distance == 0 {
                let cv = word(ships, at + 8);
                if usize::from(cv) >= cn {
                    if !allow_single {
                        return Err("ataque de barco sin convoy no conectado");
                    }
                    return Ok(Some(Prepared::SingleShip {
                        task: t,
                        ship: id as u16,
                        output_initial: output_initial.ok_or("contenido previo de pila para convoy no disponible")?,
                    }));
                }
                output_initial = Some(cv);
                let mut m = manager.to_vec();
                let mut s = ships.to_vec();
                let mut c = convoys.to_vec();
                let removed = convoy_unlink::unlink(&mut m, &mut s, &mut c, u32::from(cv))?;
                if removed.removed {
                    let base = usize::from(cv) * CONVOY;
                    let flags = u32::from_le_bytes(c[base + 0x18..base + 0x1c].try_into().unwrap()) | 0x20;
                    c[base + 0x18..base + 0x1c].copy_from_slice(&flags.to_le_bytes());
                    c[base + 0x12..base + 0x14].copy_from_slice(&0x14u16.to_le_bytes());
                    let mut member = usize::from(word(&c, base + 0xa));
                    let mut visited = vec![false; sn];
                    while member < sn {
                        if visited[member] {
                            return Err("miembros de convoy cíclicos");
                        }
                        visited[member] = true;
                        let p = member * SHIP;
                        let flags = word(&s, p + 0x3c) | 0x20;
                        s[p + 0x3c..p + 0x3e].copy_from_slice(&flags.to_le_bytes());
                        s[p + 0x134..p + 0x136].copy_from_slice(&0x14u16.to_le_bytes());
                        member = usize::from(word(&s, p + 6));
                    }
                    let leader = word(&c, base + 0x10);
                    if usize::from(leader) >= sn {
                        return Err("líder de convoy ausente");
                    };
                    let mut order = XOrder {
                        ships: &mut s,
                        count: sn as u16,
                        sea_head: word(&m, 0xea),
                    };
                    order.remove(leader);
                    convoy_nudge::nudge(&m, &mut s, &c, u32::from(cv), direction)?;
                    t.due = t.due.wrapping_add(0x40);
                    return Ok(Some(Prepared::Attack(Attack {
                        ship: id as u16,
                        convoy: cv,
                        head: word(&c, base + 0xa),
                        leader,
                        owner: c[base],
                        task: t,
                        manager: m,
                        ships: s,
                        convoys: c,
                        refresh_ship: removed.refresh_ship,
                    })));
                }
            } else if distance < 0x40 {
                delay = 1
            } else if distance < 0xe1 {
                delay &= 3
            } else if distance < 0x640 {
                delay &= 7
            }
        }
        id = next;
    }
    t.due = t.due.wrapping_add(delay);
    Ok(Some(Prepared::Wait(t)))
}
