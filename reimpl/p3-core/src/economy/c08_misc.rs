//! Funciones sueltas de C08 sobre la imagen de memoria del original: ciclo de construcción de la IA (`0x004DE9C0`), progreso de la ruta
//! terrestre (`0x004EC704`), admisión de un barco en la flota hanseática (`0x005039F0`) y consumo de mercancías de fiesta (`0x005004C0`).
//! Las dependencias externas (`0x004DEA20`, `0x004DEBA0`, `0x0052E430`) van por el trait [`MiscHost`].
use crate::flatmem::Mem;

const G_TICK: u32 = 0x006d_e4b4;
const G_TOWNS: u32 = 0x006d_e4b0;
const G_PLAYER: u32 = 0x006d_fc14;
const G_WORLD_BYTES: u32 = 0x006d_e4b8;
const G_AI_FLAG: u32 = 0x006d_f31c;
const G_TOWN_TABLE: u32 = 0x006d_e508;
const SCHED: u32 = 0x006d_d73c;
const SHIP_TABLE_PTR: u32 = 0x006d_d7a4;
const SHIP_TABLE_LEN: u32 = 0x006d_d894;

/// Tabla de rutas del EXE (`0x00672BF0`, 36 bytes): 4 coeficientes, luego 7 filas de 4 bytes (par de celdas y multiplicador).
pub const ROUTE_TABLE: [u8; 36] = [
    0x03, 0x04, 0x01, 0x02, 0x0b, 0x13, 0x0c, 0x11, 0x03, 0x04, 0x50, 0x20, 0x09, 0x0a, 0x82, 0x34, 0x0b, 0x0d, 0x1e, 0x0c, 0x0c, 0x0e, 0x28, 0x10,
    0x1c, 0x1f, 0x78, 0x30, 0x24, 0x25, 0x3c, 0x18, 0x26, 0x27, 0x5a, 0x24,
];
/// Límite de la flota hanseática (`0x00672CC8`).
pub const HANSE_LIMIT: u8 = 9;
/// Cantidades base por mercancía de fiesta (`0x006734E8`) y bandera de redondeo (`0x00672C14`).
pub const WARE_A: [u8; 8] = [3, 2, 2, 2, 0, 1, 0, 2];
pub const WARE_B: [u8; 8] = [0, 0, 0, 1, 1, 1, 1, 1];

pub trait MiscHost {
    /// `0x004DEA20` (recálculo del ciclo, `ecx` = planificador).
    fn recompute(&mut self, m: &mut dyn Mem);
    /// `0x004DEBA0` (paso del ciclo).
    fn step(&mut self, m: &mut dyn Mem);
    /// `0x0052E430`: precio de `amount` unidades de la mercancía `ware` en la ciudad `town`.
    fn price(&mut self, m: &mut dyn Mem, ware: u32, town: u32, amount: u32) -> u32;
}

/// `0x004DE9C0`: ciclo de construcción de la IA sobre el registro de cabeza del planificador `sched`.
pub fn ai_construction_cycle(m: &mut dyn Mem, host: &mut dyn MiscHost, sched: u32) {
    let head = u32::from(m.r16(sched + 8));
    let rec = m.r32(sched).wrapping_add(head * 0x18);
    let last = m.r32(rec + 0x10);
    if m.r32(G_TICK).wrapping_sub(last) >= 0x5d00 {
        host.recompute(m);
        let now = m.r32(G_TICK);
        m.w32(rec + 0x10, now);
    }
    if m.r32(G_AI_FLAG) == 0 {
        host.step(m);
    }
    let v = m.r32(rec).wrapping_add(m.r32(rec + 0xc));
    m.w32(rec, v);
}

/// `0x004EC704`: progreso de la ruta terrestre del registro `index` en `out` (24 bytes).
pub fn route_progress(m: &mut dyn Mem, sched: u32, index: u32, out: u32) {
    for k in 0..0x18 {
        m.w8(out + k, 0);
    }
    if index >= u32::from(m.r16(sched + 0xc)) {
        return;
    }
    let rec = m.r32(sched).wrapping_add(index * 0x18);
    if m.r16(rec + 6) != 0x32 || m.r8(rec + 0xc) != 1 || u32::from(m.r8(rec + 0xd)) != m.r32(G_PLAYER) {
        return;
    }
    let i0 = u32::from(m.r8(rec + 0x10));
    let c0 = m.r8(G_WORLD_BYTES + i0);
    let mut found = None;
    for k in 0..7usize {
        if c0 == ROUTE_TABLE[k * 4 + 8] {
            let i1 = u32::from(m.r8(rec + 0x11));
            let c1 = m.r8(G_WORLD_BYTES + i1);
            if c1 == ROUTE_TABLE[k * 4 + 9] {
                found = Some(k);
                break;
            }
        }
    }
    let Some(k) = found else { return };
    let mult = u32::from(ROUTE_TABLE[k * 4 + 10]);
    let rest = mult.wrapping_sub(u32::from(m.r8(rec + 0x12)));
    for i in 0..4u32 {
        let c = u32::from(ROUTE_TABLE[i as usize]);
        m.w16(out + 2 * i, c.wrapping_mul(mult) as u16);
        m.w16(out + 8 + 2 * i, c.wrapping_mul(rest) as u16);
    }
    for i in 0..4u32 {
        let hi = (u32::from(m.r8(rec + 0x13)) >> (2 * i)) & 3;
        let v = (hi << 8) + u32::from(m.r8(rec + 0x14 + i));
        m.w16(out + 0x10 + 2 * i, v as u16);
    }
}

/// `0x005039F0`: ¿se admite el barco `ship` para la orden `op` (`+0x1C` comerciante, `+0x12` tipo 9/10, `+2` tarea)?
pub fn hanse_join_allowed(m: &mut dyn Mem, op: u32, ship: u32) -> bool {
    if u32::from(m.r16(op + 0x1c)) >= m.r32(G_TOWNS) {
        return false;
    }
    let ty = m.r16(op + 0x12);
    if ty != 9 && ty != 0xa {
        return false;
    }
    let task = u32::from(m.r16(op + 2));
    if task >= u32::from(m.r16(SCHED + 0xc)) {
        return false;
    }
    let ship = ship & 0xffff;
    if ship as u16 >= m.r16(SHIP_TABLE_LEN) {
        return false;
    }
    let rec = m.r32(SCHED).wrapping_add(task * 0x18);
    let kind = m.r16(rec + 6);
    if kind != 0x34 && kind != 0x23 && kind != 0x24 {
        return false;
    }
    if m.r8(rec + 0x15) != 2 {
        return false;
    }
    let s = m.r32(SHIP_TABLE_PTR).wrapping_add(ship * 0x180);
    if m.r8(s + 0x39) != m.r8(rec + 0x14) || m.r16(s + 0x134) > 1 {
        return false;
    }
    kind != 0x34 || (m.r32(s + 0x120) as i32) >= i32::from(HANSE_LIMIT) * 8
}

/// `0x005004C0`: consume las mercancías de fiesta de la oficina `office` para `amount` personas; devuelve el coste acumulado.
pub fn celebration_wares(m: &mut dyn Mem, host: &mut dyn MiscHost, office: u32, amount: u32) -> u32 {
    let town = u32::from(m.r8(office + 0x2c6));
    let flags_at = m.r32(G_TOWN_TABLE).wrapping_add(town * 0x9f8 + 0x2c8);
    let flag = (m.r32(flags_at) >> 12) & 1;
    let people = amount & 0xffff;
    let mut cost = 0u32;
    for i in 0..8u32 {
        let mut need = u32::from(WARE_A[i as usize]) * people;
        if (need as i32) <= 0 {
            continue;
        }
        need = if flag != 0 { need << 2 } else { need << 1 };
        let want = if WARE_B[i as usize] != 0 { ((need + 199) / 200) * 200 } else { ((need + 0x7cf) / 2000) * 2000 };
        let slot = office + 4 + 4 * i;
        let have = m.r32(slot);
        if (want as i32) <= (have as i32) {
            cost = cost.wrapping_add(host.price(m, i, town, want));
            m.w32(slot, have.wrapping_sub(want));
        } else {
            cost = cost.wrapping_add(host.price(m, i, town, have));
            m.w32(slot, 0);
        }
    }
    cost
}
