//! Envío de mercancías por tierra desde la ventana del almacén (`barn_content_window`) y operación `0x3A`.
//!
//! - [`quote`]: lo que la ventana recalcula al cambiar la selección (`0x005EAE19..0x005EAFA1`: existencias disponibles, carros
//!   `+0x2E16` y valor `+0x2E18`), la escolta exigida (`0x005EB181..0x005EB1C2`, `+0x2E15`) y el coste de la escolta elegida
//!   (`0x005EB232..0x005EB2C3`, `+0x2E10`).
//! - [`send`]: `barn_content_window_send_transport` (`0x005EB3C0`): emboscada (`rand() & 3` frente a la diferencia de escolta),
//!   cargas perdidas por orden de valor y una operación `0x3A` por mercancía.
//! - [`start`]: `op_start_land_transport` (`0x0053A2C0`) hasta antes del despacho final `0x004D85C0(reloj)`, que hace quien llama.
//!
//! Tablas del ejecutable inglés: [`WEIGHTS`] (`0x00673A18`, valor por unidad) y [`BARRELS`] (`0x00672C14`: 1 = mercancía en
//! barriles de 200 unidades, 0 = cargas de 2000).
use crate::pump::Operation;
use crate::scheduler::Scheduler;

/// Bits de `0x00673A18`: valor por unidad de cada mercancía (`float`; 0.055, 0.47855, 0.22005, …).
const WEIGHT_BITS: [u32; 20] = [
    0x3D6147AF, 0x3EF50482, 0x3E6154CA, 0x3E322D0E, 0x3E11EB85, 0x3F0CCCCD, 0x3FB33333, 0x3F8CCCCD, 0x3F845A1D, 0x40587AE1, 0x3ED33333, 0x3CE147AF, 0x3FA39581, 0x3F8F5C29, 0x3EE147AF, 0x3E8E5604, 0x3EE147AF, 0x3E6147AF, 0x3F5AE147, 0x3D236E30,
];
/// `0x00673A18`.
pub const WEIGHTS: [f32; 20] = {
    let mut w = [0f32; 20];
    let mut i = 0;
    while i < 20 {
        w[i] = f32::from_bits(WEIGHT_BITS[i]);
        i += 1;
    }
    w
};
/// `0x00673A68`, el `float` que sigue a la tabla de pesos.
const AFTER_WEIGHTS: f32 = 10.0;
/// `0x00672C14`: 1 si la mercancía va en barriles (200 unidades por carga), 0 si en cargas de 2000.
pub const BARRELS: [u8; 20] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0];
/// Código de la operación.
pub const OP_START_LAND_TRANSPORT: u32 = 0x3A;
/// Tope de la selección en unidades de 200 (`+0x2E2C`).
pub const MAX_SLOTS: u32 = 100;

/// Unidades por carga de la mercancía (`2000 - 1800 * (tabla != 0)`).
pub fn unit(good: usize) -> u32 {
    if BARRELS[good] != 0 {
        200
    } else {
        2000
    }
}

/// `fild`/`fstp float`/`fmul float`/`fadd 0.5`/`__ftol` (`0x00639C28`, trunca) con la FPU a 53 bits.
fn value_of(units: u32, weight: f32) -> u32 {
    ((units as i64 as f32) as f64 * f64::from(weight) + 0.5) as i64 as u32
}

/// Coste de la escolta de nivel `level` para `wagons` carros: `(carros + 9) / 10` por 50, 200 u 800.
pub fn escort_cost(level: u8, wagons: u8) -> Option<u32> {
    let q = (i32::from(wagons) + 9) / 10;
    match level {
        0 => Some((q * 50) as u32),
        1 => Some((q * 200) as u32),
        2 => Some((q * 800) as u32),
        _ => None,
    }
}

/// Resultado del recálculo de la ventana.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Quote {
    /// Cargas elegidas tras recortarlas a lo disponible (`+0x2DA5`).
    pub loads: [u8; 20],
    /// Cargas que aún quedan en la oficina (`+0x2DBC`).
    pub available: [u32; 20],
    /// `+0x2E16`: carros (unidades de 200).
    pub wagons: u8,
    /// `+0x2E18`: valor de la carga.
    pub value: u32,
    /// `+0x2E15`: escolta exigida (0, 1, 2).
    pub required: u8,
    /// `+0x2E10`: coste de la escolta elegida.
    pub cost: u32,
}

/// Recalcula la ventana para las cargas `loads` con las existencias `stock` de la oficina de origen (`None` si no hay oficina) y
/// la escolta elegida `chosen` (`+0x2E14`, 0..=2).
pub fn quote(loads: &[u8; 20], stock: Option<&[i32]>, chosen: u8) -> Quote {
    let mut q = Quote { loads: *loads, ..Default::default() };
    for i in 0..20 {
        let u = unit(i);
        let avail = match stock {
            Some(s) => (s.get(i).copied().unwrap_or(0) as u32) / u,
            None => 0,
        };
        if u32::from(q.loads[i]) > avail {
            q.loads[i] = avail as u8;
            q.available[i] = 0;
        } else {
            q.available[i] = avail - u32::from(q.loads[i]);
        }
        let units = u32::from(q.loads[i]) * u;
        q.wagons = q.wagons.wrapping_add(((u64::from(units) * 0x51EB_851F) >> 38) as u8);
        q.value = q.value.wrapping_add(value_of(units, WEIGHTS[i]));
    }
    let w = u32::from(q.wagons);
    q.required = if q.value <= w * 100 {
        0
    } else if q.value <= w * 400 {
        1
    } else {
        2
    };
    q.cost = escort_cost(chosen, q.wagons).unwrap_or(0);
    q
}

/// Estado de la ventana que lee [`send`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Window {
    /// `+0x2DA4`, `+0x2E0C`.
    pub origin: u8,
    pub destination: u8,
    /// `+0x2DA5`.
    pub loads: [u8; 20],
    /// `+0x2E10`.
    pub cost: u32,
    /// `+0x2E14`, `+0x2E15`, `+0x2E16`.
    pub chosen: u8,
    pub required: u8,
    pub wagons: u8,
    /// `0x006DFC14`.
    pub player: u8,
}

/// Resultado de [`send`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sent {
    pub operations: Vec<Operation>,
    /// Cargas perdidas por mercancía.
    pub lost: [u8; 20],
    pub ambush: bool,
}

/// `0x005EB3C0`. `rand` es `_rand` (`0x00639C03`); solo se llama si la escolta elegida es menor que la exigida. Devuelve `None`
/// donde el original dividiría entre cero (inalcanzable con las tablas del ejecutable).
pub fn send(w: &Window, rand: &mut dyn FnMut() -> u32) -> Option<Sent> {
    let mut lost = [0u8; 20];
    let mut ambush = false;
    if w.chosen < w.required {
        let diff = w.required.wrapping_sub(w.chosen);
        let r = (rand() & 3) as i32;
        if r < i32::from(diff) {
            ambush = true;
            let base = escort_cost(w.required, w.wagons).unwrap_or(u32::from(diff));
            let mut budget = (base.wrapping_sub(w.cost) << 2).checked_div(u32::from(diff))?;
            // Orden de las mercancías por inserción, de mayor a menor clave. El original pretende ordenar por el valor de una carga
            // de 2000, pero lee el peso de la posición en que insertó la mercancía anterior (`[esp+0x14]`), y para la primera la
            // posición 0x14, que cae en el `float` siguiente a la tabla (`0x00673A68` = 10.0).
            let mut order = [0u8; 20];
            let mut keys = [0u32; 20];
            let mut at = 0x14usize;
            for b in 0..20u8 {
                let weight = WEIGHTS.get(at).copied().unwrap_or(AFTER_WEIGHTS);
                let key = ((f64::from(weight) * 2000.0) + 0.5) as i64 as u32;
                let mut idx = usize::from(b);
                while idx != 0 {
                    if key <= keys[idx - 1] {
                        break;
                    }
                    keys[idx] = keys[idx - 1];
                    order[idx] = order[idx - 1];
                    idx -= 1;
                }
                order[idx] = b;
                keys[idx] = key;
                at = idx;
            }
            let mut k = 0u8;
            while k < 20 {
                let g = usize::from(order[usize::from(k)]);
                // El original toma el valor de la posición `k`, no el de la mercancía `g`.
                let v = value_of(unit(g), WEIGHTS[usize::from(k)]);
                let n = (budget.wrapping_add(v).wrapping_sub(1).checked_div(v)?) as u8;
                lost[g] = n.min(w.loads[g]);
                let spent = u32::from(lost[g]).wrapping_mul(v);
                if budget > spent {
                    budget -= spent;
                } else {
                    k = 0x13;
                }
                k += 1;
            }
        }
    }
    let mut operations = Vec::new();
    let mut flag = u8::from(!ambush);
    let mut cost = w.cost as u16;
    let mut pending_ambush = ambush;
    for g in 0..20u8 {
        let loads = w.loads[usize::from(g)];
        if loads == 0 {
            continue;
        }
        let l = lost[usize::from(g)];
        if pending_ambush && l != 0 {
            flag = 1;
            pending_ambush = false;
        }
        let b1 = u32::from_le_bytes([w.origin, w.destination, w.player, flag]);
        let b2 = u32::from(loads) | u32::from(cost) << 16;
        let b3 = u32::from(l) | u32::from(g) << 16 | u32::from(w.chosen) << 24;
        operations.push(Operation([OP_START_LAND_TRANSPORT, b1, b2, b3, 0]));
        cost = 0;
        flag = 0;
    }
    Some(Sent { operations, lost, ambush })
}

/// Campos de la operación `0x3A`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartOp {
    pub origin: u8,
    pub destination: u8,
    pub merchant: u8,
    pub flag: u8,
    pub loads: u16,
    pub cost: u16,
    pub lost: u16,
    pub good: u8,
    pub escort: u8,
}

impl StartOp {
    pub fn decode(op: Operation) -> Self {
        let b = op.0[1].to_le_bytes();
        StartOp {
            origin: b[0],
            destination: b[1],
            merchant: b[2],
            flag: b[3],
            loads: op.0[2] as u16,
            cost: (op.0[2] >> 16) as u16,
            lost: op.0[3] as u16,
            good: (op.0[3] >> 16) as u8,
            escort: (op.0[3] >> 24) as u8,
        }
    }
}

/// `0x0053A2C0` hasta el despacho: añade la tarea `0x0C` con vencimiento `tick` y la rellena. Devuelve el índice de la tarea, o
/// `None` si la operación no lleva cargas (el original deja entonces el registro tomado sin tipo ni carga y no despacha).
pub fn start(s: &mut Scheduler, op: Operation, tick: u32) -> Option<u16> {
    let o = StartOp::decode(op);
    let idx = s.add_task(tick);
    if o.loads == 0 {
        return None;
    }
    let i = usize::from(idx);
    {
        let r = &mut s.records[i];
        r.set_u8_at(8, o.origin);
        r.set_u8_at(9, o.destination);
        r.set_u8_at(0xA, o.merchant);
        let mut b = ((u32::from(o.lost) * 200) / u32::from(o.loads)) as u8;
        r.set_u8_at(0xB, b);
        if o.flag != 0 {
            b |= 1;
            r.set_u8_at(0xB, b);
        } else {
            r.set_u8_at(0xB, b & 0xFE);
        }
    }
    if o.flag != 0 && o.lost == 0 {
        // Un solo carro por ciudad de origen: si otro transporte de allí acaba de salir (fase 1, progreso < 9), este no se dibuja.
        let capacity = s.capacity;
        let mut at = u32::from(s.head);
        while let Some(cur) = s.get_task(at) {
            let Some(rec) = s.records.get(cur as usize).copied() else { break };
            if rec.kind == 0x0C && rec.u8_at(8) == o.origin && rec.u8_at(0x15) == 1 && rec.u8_at(0x16) < 9 {
                let v = s.records[i].u8_at(0xB) & 0xFE;
                s.records[i].set_u8_at(0xB, v);
            }
            let next = rec.next;
            if capacity <= next || rec.due.wrapping_sub(0x10) < tick {
                break;
            }
            at = u32::from(next);
        }
    }
    let r = &mut s.records[i];
    r.set_u8_at(0x12, o.loads as u8);
    r.set_u8_at(0x13, (o.loads >> 8) as u8);
    r.set_u8_at(0x10, o.cost as u8);
    r.set_u8_at(0x11, (o.cost >> 8) as u8);
    r.set_u8_at(0x15, 0);
    r.set_u8_at(0x16, 0);
    r.set_u8_at(0x14, o.good);
    r.set_u32_at(0xC, 0);
    r.kind = 0x0C;
    r.set_u8_at(0x17, o.escort);
    Some(idx)
}

/// `0x00672BF8`: las siete rutas terrestres `(id a, id b, …)`.
pub const LAND_ROUTES: [[u8; 4]; 7] = [[3, 4, 80, 32], [9, 10, 130, 52], [11, 13, 30, 12], [12, 14, 40, 16], [28, 31, 120, 48], [36, 37, 60, 24], [38, 39, 90, 36]];

/// `0x00525630` sobre sus datos: `index` es `ciudad+0x2C0`, `flags` `ciudad+0x76C`, `ids` la tabla `0x006DE4B8` y `count` `0x006DE4B0`.
/// Devuelve la posición de la ciudad unida por tierra (`0xFF` si no hay).
pub fn land_partner_raw(index: u8, flags: u32, ids: &[u8], count: usize) -> u8 {
    if flags & 0x8000 == 0 {
        return 0xFF;
    }
    let me = ids.get(usize::from(index)).copied().unwrap_or(0);
    let mut other = 0xFFu8;
    for r in &LAND_ROUTES {
        if r[0] == me {
            other = r[1];
            break;
        }
        if r[1] == me {
            other = r[0];
            break;
        }
    }
    if other >= 0x28 {
        return 0xFF;
    }
    ids.iter().take(count).position(|&id| id == other).map_or(0xFF, |p| p as u8)
}

/// `0x00525630` para la ciudad en la posición `town`.
pub fn land_partner(towns: &[crate::town::Town], ids: &[u8], town: usize) -> Option<u8> {
    let t = towns.get(town)?;
    let p = land_partner_raw(t.u8(0x2C0), t.u32(0x76C), ids, towns.len());
    (p != 0xFF).then_some(p)
}
