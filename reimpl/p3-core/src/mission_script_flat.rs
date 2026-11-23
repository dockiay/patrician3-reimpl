//! Comandos del intérprete de guiones de misión `0x004ECF64` que leen o escriben el mundo con las direcciones
//! del original (memoria plana, como `economy::siege_record`).
//!
//! Cada comando se traduce instrucción a instrucción de su manejador (tabla de saltos `0x004F2E34`, índices
//! en `0x004F3054`). Los accesos triviales (`get_merchant`, `ships_get_ship_ptr`, `game_world_get_town`,
//! contadores, cabezas de lista, `ship_add_modified_flags`…) se hacen aquí sobre la memoria; las llamadas a
//! funciones con lógica propia pasan por [`ScriptCalls`] con su dirección original, `this` y argumentos,
//! para que el anfitrión las resuelva con sus puertos (o las rechace si no están conectadas).
//!
//! Contratos de las llamadas cuyo original usa punteros a variables locales (documentados en cada caso):
//! - `0x00515750` (`ship_reset_loadout_for_pirate`): argumentos `[0xFFFF]`; devuelve el valor final de la
//!   variable de salida (que empieza en -1).
//! - `0x00525B90` (`town_pick_random_building_position`): argumentos `[tipo, x, y]` con los dos bytes de
//!   salida antes de la llamada; devuelve `x | y << 8` tal como quedan.
//! - `0x0054AA70` (`enqueue_operation`): los cinco dwords del registro de operación en lugar del puntero.
//! - `0x004D6530` (`add_message`): destinatario y los 16 bytes del registro (convención del puente C08);
//!   los bytes `+6..+8` que el original deja sin inicializar se pasan a 0.
//! - `0x00533210` (`find_nearest_town_excluding`): las tres ciudades excluidas en lugar del puntero.
//! - `0x0050E000` (`create_docked_ship_for_merchant`): la palabra baja de la variable local en lugar del
//!   puntero.
//!
//! Solo presentación (sin efecto en la partida): `0xEF` (aviso), `0xF5` (pergamino de texto) y la ventana
//! de `0xE8`; su notificación `0x00548CA0` sí se entrega al anfitrión.
use crate::flatmem::Mem;

pub const WORLD: u32 = 0x006d_e4a0;
pub const SHIPS: u32 = 0x006d_d7a0;
pub const POOL: u32 = 0x006d_d730;
pub const SCHED: u32 = 0x006d_d73c;
pub const OPS: u32 = 0x006d_f2f0;
/// Opciones de la partida (puntero).
pub const OPTIONS_PTR: u32 = 0x006c_c3e8;
/// Jugador local (`OPS+0x924`).
pub const LOCAL_PLAYER: u32 = 0x006d_fc14;
/// Coordenadas de las ciudades (x, y en dwords, 8 bytes por ciudad).
pub const TOWN_COORDS: u32 = 0x0067_2340;
/// Mercancías por barriles (unidad 200) frente a las de carga (2.000).
pub const SMALL_LOT: u32 = 0x0067_2c14;
/// Factor por mercancía de `0x2F`.
pub const WARE_NEED: u32 = 0x0067_2660;
/// Valor base por mercancía (float) de `0x49`.
pub const WARE_BASE: u32 = 0x0067_3a18;
/// Objeto de rutas que usa `0x5B`.
pub const ROUTES_PTR: u32 = 0x006c_bdc8;
/// `this` de `get_sell_price`.
pub const PRICES: u32 = 0x006d_e3d8;

/// Llamadas al original que el anfitrión resuelve. Debe devolver `EAX` (o el valor del contrato documentado
/// en el módulo) o un error si la función no está conectada.
pub trait ScriptCalls {
    fn call(&mut self, mem: &mut dyn Mem, addr: u32, ecx: u32, args: &[u32]) -> Result<u32, String>;
}

/// Entorno de un comando: memoria plana, llamadas y la palabra `+0x10` de la tarea en curso (`0x1F`).
pub struct FlatScript<'a> {
    pub mem: &'a mut dyn Mem,
    pub calls: &'a mut dyn ScriptCalls,
    pub task_word_10: u16,
}

/// Comandos que implementa este módulo.
pub fn handles(op: u8) -> bool {
    matches!(
        op,
        0x02 | 0x03 | 0x08 | 0x0f | 0x10 | 0x1f | 0x25 | 0x2a..=0x2f | 0x31..=0x33 | 0x35 | 0x36 | 0x3b | 0x3c | 0x3e
            | 0x41 | 0x42 | 0x45..=0x4e | 0x50..=0x52 | 0x54..=0x64 | 0x66..=0x6a | 0x6c..=0x74 | 0xe4 | 0xe5
            | 0xe6 | 0xe8 | 0xea..=0xef | 0xf1..=0xf6 | 0xfa
    )
}

const LOOP_LIMIT: usize = 1 << 20;

fn v(vars: &[u32], i: u8) -> Result<u32, String> {
    vars.get(usize::from(i)).copied().ok_or_else(|| format!("variable {i} fuera de rango"))
}
fn vu(vars: &[u32], i: u32) -> Result<u32, String> {
    vars.get(i as usize).copied().ok_or_else(|| format!("variable {i} fuera de rango"))
}
fn set(vars: &mut [u32], i: u8, x: u32) -> Result<(), String> {
    *vars.get_mut(usize::from(i)).ok_or_else(|| format!("variable {i} fuera de rango"))? = x;
    Ok(())
}
fn setu(vars: &mut [u32], i: u32, x: u32) -> Result<(), String> {
    *vars.get_mut(i as usize).ok_or_else(|| format!("variable {i} fuera de rango"))? = x;
    Ok(())
}

fn merchant(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(WORLD + 0x78).wrapping_add(i.wrapping_mul(0x650))
}
fn ship(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(SHIPS + 4).wrapping_add(i.wrapping_mul(0x180))
}
fn town(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(WORLD + 0x68).wrapping_add(i.wrapping_mul(0x9f8))
}
fn office(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(WORLD + 0x74).wrapping_add(i.wrapping_mul(0x44c))
}
fn convoy(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(SHIPS + 8).wrapping_add(i.wrapping_mul(0x3c))
}
fn ship_count(m: &mut dyn Mem) -> u16 {
    m.r16(SHIPS + 0xf4)
}
fn convoy_count(m: &mut dyn Mem) -> u16 {
    m.r16(SHIPS + 0xf6)
}
fn merchant_count(m: &mut dyn Mem) -> u16 {
    m.r16(WORLD + 0xa)
}
fn towns_count(m: &mut dyn Mem) -> u32 {
    m.r32(WORLD + 0x10)
}
fn office_count(m: &mut dyn Mem) -> u16 {
    m.r16(WORLD + 8)
}
fn game_time(m: &mut dyn Mem) -> u32 {
    m.r32(WORLD + 0x14)
}
fn small_lot(m: &mut dyn Mem, ware: u32) -> bool {
    m.r8(SMALL_LOT.wrapping_add(ware)) != 0
}
/// `bulk ? 200 : 2000` (`neg; sbb; and 0xFFFFF8F8; add 0x7D0`).
fn lot(m: &mut dyn Mem, ware: u32) -> i32 {
    if small_lot(m, ware) {
        200
    } else {
        2000
    }
}
/// Cantidad en unidades a peso: `a·200` o `a·2000` según la mercancía.
fn scaled(m: &mut dyn Mem, ware: u32, a: u32) -> i32 {
    (a as i32).wrapping_mul(lot(m, ware))
}
/// Peso a unidades: `/200` o `/2000` (división con signo por multiplicación mágica `0x51EB851F >> 6` o
/// `0x10624DD3 >> 7`).
fn unscale(m: &mut dyn Mem, ware: u32, x: u32) -> u32 {
    (x as i32).wrapping_div(lot(m, ware)) as u32
}

/// `ship_add_modified_flags` (`0x00517770`).
pub fn mark_changed(m: &mut dyn Mem, s: u32, flags: u32) {
    let old = m.r16(s + 0x3c);
    m.w16(s + 0x3c, old | flags as u16);
    let c = m.r16(s + 8);
    if c < convoy_count(m) {
        let at = m.r32(SHIPS + 8).wrapping_add(u32::from(c) * 0x3c + 0x18);
        let old = m.r32(at);
        m.w32(at, old | (flags & 0xffff));
    }
}

/// `ship_can_lead_convoy` (`0x00519C40`).
fn can_lead_convoy(m: &mut dyn Mem, s: u32) -> bool {
    if m.r16(s + 0x40) < 0x14 {
        return false;
    }
    if m.r16(s + 8) >= convoy_count(m) && (m.r32(s + 0x18) as i32) < (m.r32(s + 0x14) as i32 >> 1) {
        return false;
    }
    if (m.r32(s + 0x120) as i32) < 0x43 {
        return false;
    }
    if m.r16(s + 0x42) >= m.r16(SHIPS + 0xf2) {
        return false;
    }
    m.r8(s + 0x15c) == 0
}

/// Recorre la cadena de barcos `head` (enlace `+4`) mientras el índice sea menor que la cuenta.
fn chain(m: &mut dyn Mem, head: u16) -> Result<Vec<u16>, String> {
    let count = ship_count(m);
    let mut out = Vec::new();
    let mut id = head;
    while id < count {
        if out.len() > usize::from(count) {
            return Err("cadena de barcos cíclica".into());
        }
        out.push(id);
        let s = ship(m, u32::from(id));
        id = m.r16(s + 4);
    }
    Ok(out)
}

/// Condición de puerto común de `0x6D/0x6E/0x74`: los barcos de tipo `>= 2` no entran en ciudades con el bit 0
/// de `Town+0x2C8`.
fn port_ok(m: &mut dyn Mem, s: u32, port: u8) -> bool {
    let kind = m.r8(s + 0xe);
    if kind < 2 {
        return true;
    }
    if kind <= 1 {
        return false;
    }
    let t = town(m, u32::from(port));
    m.r32(t + 0x2c8) & 1 == 0
}

/// Barco atracado en `town`, sin viaje pendiente (`+0x134 < 4`) y libre (`+0x136` bit 0 a cero).
fn idle_in(m: &mut dyn Mem, s: u32, town: u8) -> bool {
    m.r8(s + 0x39) == town && m.r16(s + 0x134) < 4 && m.r8(s + 0x136) & 1 == 0
}

/// Filtro de modo de `0x36` y `0x70` sobre el estado `+0x134`.
fn mode_ok(mode: u32, state: u16) -> bool {
    match mode as i32 {
        0 => state == 0xf,
        1 => state <= 3,
        x if x > 1 => state == 0xf || state <= 3,
        _ => false,
    }
}

fn call(f: &mut FlatScript<'_>, addr: u32, ecx: u32, args: &[u32]) -> Result<u32, String> {
    f.calls.call(f.mem, addr, ecx, args)
}

fn recompute_load(f: &mut FlatScript<'_>, s: u32) -> Result<u32, String> {
    call(f, 0x005182b0, s, &[])
}
fn find_office(f: &mut FlatScript<'_>, merchant: u32, town: u32, create: u32) -> Result<u32, String> {
    call(f, 0x005308a0, WORLD, &[merchant & 0xffff, town & 0xffff, create])
}
fn enqueue(f: &mut FlatScript<'_>, record: &[u32; 5]) -> Result<(), String> {
    call(f, 0x0054aa70, OPS, record).map(|_| ())
}
fn date_to_ticks(f: &mut FlatScript<'_>, msg: u32) -> Result<u32, String> {
    let day = u32::from(f.mem.r8(msg));
    let month = u32::from(f.mem.r8(msg + 1) & 0x7f);
    let year = u32::from(f.mem.r16(msg + 2));
    call(f, 0x00531260, WORLD, &[day, month, year])
}
fn add_message(f: &mut FlatScript<'_>, recipient: u32, mut record: [u8; 16]) -> Result<(), String> {
    record[6] = 0;
    record[7] = 0;
    let mut args = vec![recipient];
    args.extend(record.iter().map(|&b| u32::from(b)));
    call(f, 0x004d6530, POOL, &args).map(|_| ())
}
/// `letter_init` (`0x0051A850`): `+0 = 0`, `+1 = 0x80`, `+2 = 0xFFFF`, `+4 = +5 = 0xFF`, `+8..+16 = 0`.
fn letter_init() -> [u8; 16] {
    let mut r = [0u8; 16];
    r[1] = 0x80;
    r[2] = 0xff;
    r[3] = 0xff;
    r[4] = 0xff;
    r[5] = 0xff;
    r
}
/// `operation_record_init` (`0x00535740`).
fn op_record() -> [u32; 5] {
    [u32::MAX, 0, 0, 0, 0]
}
fn set_u16(r: &mut [u32; 5], at: usize, x: u16) {
    let w = &mut r[at / 4];
    let sh = (at % 4) * 8;
    *w = (*w & !(0xffff << sh)) | (u32::from(x) << sh);
}
fn set_u8(r: &mut [u32; 5], at: usize, x: u8) {
    let w = &mut r[at / 4];
    let sh = (at % 4) * 8;
    *w = (*w & !(0xff << sh)) | (u32::from(x) << sh);
}
fn get_u8(r: &[u32; 5], at: usize) -> u8 {
    (r[at / 4] >> ((at % 4) * 8)) as u8
}

/// Daño de incendio de `0xE5/0xED`: `población / 300` si `nivel·5000 < 8·población`, si no `/ 800`.
fn fire_damage(m: &mut dyn Mem, t: u32) -> u16 {
    let level = i32::from(m.r8(t + 0x789));
    let pop = m.r32(t + 0x2d4) as i32;
    if level.wrapping_mul(5000) < pop.wrapping_mul(8) {
        (pop / 300) as u16
    } else {
        (pop / 800) as u16
    }
}

/// Ejecuta el comando `op` (que debe cumplir [`handles`]); el contador de programa avanza siempre en uno.
pub fn step(f: &mut FlatScript<'_>, op: u8, cmd: &[u8], vars: &mut [u32]) -> Result<(), String> {
    let need = |n: usize| cmd.get(..n).ok_or_else(|| format!("comando {op:#04x} truncado"));
    match op {
        // 0x004EEA6E: días desde el inicio.
        0x02 => {
            let c = need(2)?;
            let days = (game_time(f.mem) >> 8).wrapping_sub(f.mem.r32(WORLD + 0xc));
            set(vars, c[1], days)?
        }
        // 0x004EEAAB: barcos del comerciante de tipo `vars[b]` (todos si `vars[b] > 3`).
        0x03 => {
            let c = need(4)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let head = f.mem.r16(m + 0xe);
            let mut n = 0u8;
            for id in chain(f.mem, head)? {
                let s = ship(f.mem, u32::from(id));
                let kind = v(vars, c[2])? as i32;
                if kind > 3 || kind == i32::from(f.mem.r8(s + 0xe)) {
                    n = n.wrapping_add(1)
                }
            }
            set(vars, c[3], u32::from(n))?
        }
        // 0x004EED7B: humano con más `+0x46C` (con signo) o, con una opción `+0xC <= 1`, el jugador local.
        0x08 => {
            let c = need(2)?;
            let options = f.mem.r32(OPTIONS_PTR);
            let selected = if f.mem.r8(options + 0xc) > 1 {
                let mut best = i32::MIN;
                let mut sel = None;
                for b in 0..u32::from(merchant_count(f.mem)) {
                    let m = merchant(f.mem, b);
                    if f.mem.r16(m + 8) == 0 {
                        let score = f.mem.r32(m + 0x46c) as i32;
                        if score > best {
                            best = score;
                            sel = Some(b as u8)
                        }
                    }
                }
                // Sin candidato el original devuelve un byte de pila sin inicializar.
                sel.ok_or("0x08 sin comerciante humano: el original lee un byte sin inicializar")?
            } else {
                f.mem.r8(LOCAL_PLAYER)
            };
            set(vars, c[1], u32::from(selected))?
        }
        // 0x004EEF59: barcos de las cinco bandas piratas (`count_pirate_group_ships`, 0x00506290).
        0x0f => {
            let c = need(2)?;
            let mut n = 0u32;
            for k in 0..5 {
                let p = f.mem.r32(SHIPS + 0xc + 4 * k);
                if p != 0 {
                    let head = f.mem.r16(p + 0xc);
                    n = n.wrapping_add(chain(f.mem, head)?.len() as u32)
                }
            }
            set(vars, c[1], n & 0xffff)?
        }
        // 0x004EEF8A: primer barco con armamento `+0x120` por encima del umbral pirata o, si no hay, el de mayor
        // armamento (desempate por `+0x10`); 0xFFFF si el índice resultante es la cuenta de barcos.
        0x10 => {
            let c = need(3)?;
            let limit = call(f, 0x00515750, 0, &[0xffff])?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let count = ship_count(f.mem);
            let mut di = f.mem.r16(m + 0xe);
            let mut bp = di;
            let (mut best, mut second) = (0i32, 0i32);
            let mut steps = 0;
            while di < count {
                steps += 1;
                if steps > LOOP_LIMIT {
                    return Err("cadena de barcos cíclica".into());
                }
                let s = ship(f.mem, u32::from(di));
                let arm = f.mem.r32(s + 0x120);
                if arm > limit {
                    bp = di;
                    break;
                }
                if best < arm as i32 {
                    best = arm as i32;
                    bp = di
                } else if best == arm as i32 {
                    let sec = f.mem.r32(s + 0x10) as i32;
                    if second < sec {
                        second = sec;
                        bp = di
                    }
                }
                di = f.mem.r16(s + 4);
            }
            let r = if bp == count { 0xffff } else { bp };
            set(vars, c[2], u32::from(r))?
        }
        // 0x004EF788: barco del comerciante a menos de √0x1B (√0x50 en la tarea 0x134) de la ciudad.
        0x1f => {
            let c = need(4)?;
            let who = v(vars, c[1])? as u16;
            let at = v(vars, c[2])? as u16;
            set(vars, c[3], u32::MAX)?;
            let limit = if f.task_word_10 == 0x134 { 0x50 } else { 0x1b };
            if who < merchant_count(f.mem) {
                let m = merchant(f.mem, u32::from(who));
                let head = f.mem.r16(m + 0xe);
                let xy = TOWN_COORDS + u32::from(at) * 8;
                for id in chain(f.mem, head)? {
                    let s = ship(f.mem, u32::from(id));
                    let dx = i32::from(f.mem.r16(s + 0x1e) as i16).wrapping_sub(f.mem.r32(xy) as i32);
                    let dy = i32::from(f.mem.r16(s + 0x22) as i16).wrapping_sub(f.mem.r32(xy + 4) as i32);
                    if dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)) < limit {
                        set(vars, c[3], u32::from(id))?;
                        break;
                    }
                }
            }
        }
        // 0x004EF70C: carta 0x45/0x46 pendiente en la cadena del comerciante (`has_type_45_or_46_message`).
        0x25 => {
            let c = need(3)?;
            let who = v(vars, c[1])?;
            if who as i32 >= i32::from(merchant_count(f.mem)) {
                set(vars, c[2], 0)?
            } else {
                let m = merchant(f.mem, who);
                let mut at = u32::from(f.mem.r16(m + 0xa));
                let count = u32::from(f.mem.r16(POOL + 6));
                let mut budget = count;
                let mut found = 0;
                while at < count && budget > 0 {
                    budget -= 1;
                    let e = f.mem.r32(POOL).wrapping_add(at << 4);
                    let kind = f.mem.r8(e + 4);
                    if kind == 0x45 || kind == 0x46 {
                        found = 1;
                        break;
                    }
                    at = u32::from(f.mem.r16(e + 6));
                }
                set(vars, c[2], found)?
            }
        }
        // 0x004EFDDF: el barco está en la ciudad con un estado de puerto (`< 2`, `4..=8` o `0x10`).
        0x2a => {
            let c = need(4)?;
            let at = v(vars, c[1])? as u8;
            let id = v(vars, c[2])? as u16;
            let r = if id < ship_count(f.mem) {
                let s = ship(f.mem, u32::from(id));
                if f.mem.r8(s + 0x39) != at {
                    0
                } else {
                    let st = f.mem.r16(s + 0x134);
                    u32::from(st < 2 || (4..=8).contains(&st) || st == 0x10)
                }
            } else {
                0
            };
            set(vars, c[3], r)?
        }
        // 0x004F028D / 0x004F03AC: barco ocioso con más capacidad (`recompute_load`) y esa capacidad en lotes de 2.000.
        0x2b | 0x6e => {
            let c = need(if op == 0x2b { 5 } else { 6 })?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let head = f.mem.r16(m + 0xe);
            let at = v(vars, c[2])? as u8;
            let port = if op == 0x6e { v(vars, c[5])? as u8 } else { 0 };
            set(vars, c[3], u32::MAX)?;
            set(vars, c[4], 0)?;
            let mut best = 0u32;
            for id in chain(f.mem, head)? {
                let s = ship(f.mem, u32::from(id));
                if !idle_in(f.mem, s, at) || (op == 0x6e && !port_ok(f.mem, s, port)) {
                    continue;
                }
                if recompute_load(f, s)? > best {
                    best = recompute_load(f, s)?;
                    set(vars, c[3], u32::from(id))?;
                    set(vars, c[4], best / 2000)?;
                }
            }
        }
        // 0x004F0692: `captain_message_status`.
        0x2c => {
            let c = need(4)?;
            let r = call(f, 0x004d7770, POOL, &[v(vars, c[1])?, v(vars, c[2])?])?;
            set(vars, c[3], r)?
        }
        // 0x004F06C1: `Merchant+0x46C`.
        0x2d => {
            let c = need(3)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let x = f.mem.r32(m + 0x46c);
            set(vars, c[2], x)?
        }
        // 0x004F0711: reputación en la ciudad en punto fijo 16.16 (`0x004F3740`).
        0x2e => {
            let c = need(4)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let at = m.wrapping_add(v(vars, c[2])?.wrapping_mul(4)).wrapping_add(0x2fc);
            let rep = f32::from_bits(f.mem.r32(at));
            let scale = f32::from_bits(f.mem.r32(0x0067_2850));
            set(vars, c[3], crate::numeric::ftol_i32(f64::from(rep) * f64::from(scale)) as u32)?
        }
        // 0x004F0780: mercancía producida más escasa y su factor (`0x00672660`).
        0x2f => {
            let c = need(4)?;
            let t = town(f.mem, v(vars, c[1])?);
            let ware = call(f, 0x0051beb0, t, &[])? as u8;
            set(vars, c[2], u32::from(ware))?;
            let need_factor = if ware < 0x14 { u32::from(f.mem.r8(WARE_NEED + u32::from(ware))) } else { 0 };
            set(vars, c[3], need_factor)?
        }
        // 0x004F0816: `Merchant+0x17` acotado a ±20 (`0x004F36C0`).
        0x31 => {
            let c = need(3)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let x = (f.mem.r8(m + 0x17) as i8).clamp(-20, 20);
            set(vars, c[2], i32::from(x) as u32)?
        }
        // 0x004F086A: primer barco parado (`+0x134 == 0`) fuera de las `n` ciudades excluidas.
        0x32 => {
            let c = need(5)?;
            let n = usize::from(c[1]);
            let excluded = cmd.get(5..5 + n).ok_or("comando 0x32 truncado")?;
            let m = merchant(f.mem, v(vars, c[2])?);
            set(vars, c[3], u32::MAX)?;
            set(vars, c[4], u32::MAX)?;
            let head = f.mem.r16(m + 0xe);
            for id in chain(f.mem, head)? {
                let s = ship(f.mem, u32::from(id));
                if f.mem.r16(s + 0x134) != 0 {
                    continue;
                }
                let at = u32::from(f.mem.r8(s + 0x39));
                let mut k = 0;
                while k < n && at != v(vars, excluded[k])? {
                    k += 1
                }
                if k == n {
                    set(vars, c[3], at)?;
                    set(vars, c[4], u32::from(id))?;
                    break;
                }
            }
        }
        // 0x004F098E: primer barco que puede encabezar un convoy.
        0x33 => {
            let c = need(3)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            set(vars, c[2], u32::MAX)?;
            let head = f.mem.r16(m + 0xe);
            for id in chain(f.mem, head)? {
                let s = ship(f.mem, u32::from(id));
                if can_lead_convoy(f.mem, s) {
                    set(vars, c[2], u32::from(id))?;
                    break;
                }
            }
        }
        // 0x004F0A54: opción de partida `+0x16`.
        0x35 => {
            let c = need(2)?;
            let options = f.mem.r32(OPTIONS_PTR);
            let x = f.mem.r8(options + 0x16);
            set(vars, c[1], u32::from(x))?
        }
        // 0x004F0A6F / 0x004F0C74: barco aleatorio (determinista) que cumple el modo de estado.
        0x36 | 0x70 => {
            let c = need(4)?;
            let who = v(vars, c[1])?;
            let m = merchant(f.mem, who);
            let head = f.mem.r16(m + 0xe);
            let count = ship_count(f.mem);
            let mut result = u32::MAX;
            if head < count {
                let mut eligible = 0u32;
                for id in chain(f.mem, head)? {
                    let s = ship(f.mem, u32::from(id));
                    let convoy_ok = op == 0x70 || f.mem.r16(s + 8) >= convoy_count(f.mem);
                    if convoy_ok && f.mem.r32(s + 0x18) as i32 > 0 {
                        let mode = v(vars, c[3])?;
                        if mode_ok(mode, f.mem.r16(s + 0x134)) && f.mem.r16(s + 0x40) > 0 {
                            eligible = eligible.wrapping_add(1)
                        }
                    }
                }
                let mut bx = eligible as u16;
                if bx != 0 {
                    let sched_head = u32::from(f.mem.r16(SCHED + 8));
                    let towns0 = f.mem.r32(WORLD + 0x68);
                    let pop0 = f.mem.r32(towns0 + 0x2d4);
                    let seed = game_time(f.mem).wrapping_add(sched_head).wrapping_add(pop0);
                    bx = (seed % u32::from(bx)) as u16;
                    let m = merchant(f.mem, v(vars, c[1])?);
                    let mut ebp = u32::from(f.mem.r16(m + 0xe));
                    let mut steps = 0;
                    loop {
                        if ebp >= u32::from(ship_count(f.mem)) {
                            result = ebp;
                            break;
                        }
                        steps += 1;
                        if steps > LOOP_LIMIT {
                            return Err("cadena de barcos cíclica".into());
                        }
                        let own = ship(f.mem, ebp);
                        let mut s = own;
                        let group = f.mem.r16(own + 8);
                        let pass = if op == 0x36 {
                            group > convoy_count(f.mem)
                        } else {
                            if group < convoy_count(f.mem) {
                                let cv = convoy(f.mem, u32::from(group));
                                let leader = f.mem.r16(cv + 0x10);
                                s = ship(f.mem, u32::from(leader));
                            }
                            true
                        };
                        if pass && f.mem.r32(s + 0x18) as i32 > 0 {
                            let mode = v(vars, c[3])?;
                            if mode_ok(mode, f.mem.r16(s + 0x134)) && f.mem.r16(s + 0x40) > 0 {
                                if bx == 0 {
                                    result = ebp;
                                    break;
                                }
                                bx = bx.wrapping_sub(1);
                            }
                        }
                        ebp = u32::from(f.mem.r16(own + 4));
                    }
                }
            }
            set(vars, c[2], result)?
        }
        // 0x004F1001: tripulación `ship+0x40`.
        0x3b => {
            let c = need(3)?;
            let s = ship(f.mem, v(vars, c[1])?);
            let x = f.mem.r16(s + 0x40);
            set(vars, c[2], u32::from(x))?
        }
        // 0x004F1050: tabla de enfriamiento en las variables `[base, base+n)` (horas) y `[base+n, base+2n)` (claves).
        0x3c => {
            let c = need(6)?;
            let base = i32::from(c[1]);
            let n = v(vars, c[2])? as i32;
            let key = v(vars, c[3])?;
            set(vars, c[5], 0)?;
            let end = base.wrapping_add(n);
            let mut free = end;
            let mut i = base;
            while i < end {
                let key_at = n.wrapping_add(i) as u32;
                if vu(vars, i as u32)? == 0 {
                    if i < free {
                        free = i
                    }
                } else {
                    let now = game_time(f.mem);
                    if v(vars, c[4])?.wrapping_add(vu(vars, i as u32)?) > now {
                        if vu(vars, key_at)? == key {
                            break;
                        }
                    } else if vu(vars, key_at)? == key {
                        setu(vars, i as u32, game_time(f.mem))?;
                        set(vars, c[5], 1)?;
                        break;
                    } else {
                        setu(vars, i as u32, 0)?
                    }
                }
                i += 1;
            }
            if i == end && free < end {
                setu(vars, free as u32, game_time(f.mem))?;
                setu(vars, free.wrapping_add(n) as u32, key)?;
                set(vars, c[5], 1)?;
            }
        }
        // 0x004F11A5: unidades de una mercancía a bordo.
        0x3e => {
            let c = need(4)?;
            let s = ship(f.mem, v(vars, c[1])?);
            let ware = v(vars, c[2])?;
            let x = f.mem.r32(s.wrapping_add(ware.wrapping_mul(4)).wrapping_add(0x54));
            // El original guarda el valor bruto y relee la mercancía (puede ser la misma variable).
            set(vars, c[3], x)?;
            let ware = v(vars, c[2])?;
            set(vars, c[3], unscale(f.mem, ware, x))?
        }
        // 0x004F1409: casco al `vars[a]` % si el barco (o su jefe de convoy) está en un estado admitido.
        0x41 => {
            let c = need(4)?;
            let s = ship(f.mem, v(vars, c[2])?);
            let mut st = f.mem.r16(s + 0x134);
            let group = f.mem.r16(s + 8);
            if group < convoy_count(f.mem) {
                let cv = convoy(f.mem, u32::from(group));
                let leader = f.mem.r16(cv + 0x10);
                let l = ship(f.mem, u32::from(leader));
                st = f.mem.r16(l + 0x134);
            }
            if st < 4 || matches!(st, 8 | 9 | 0xf | 0x12 | 0x14) {
                let hull = (v(vars, c[1])?.wrapping_mul(f.mem.r32(s + 0x14)) as i32) / 100;
                f.mem.w32(s + 0x18, hull as u32);
                mark_changed(f.mem, s, 0x100);
                set(vars, c[3], 1)?
            } else {
                set(vars, c[3], 0)?
            }
        }
        // 0x004F1514: porcentaje de casco (1 si redondea a 0 con casco).
        0x42 => {
            let c = need(3)?;
            let s = ship(f.mem, v(vars, c[1])?);
            let hull = f.mem.r32(s + 0x18) as i32;
            let max = f.mem.r32(s + 0x14) as i32;
            let mut p = hull.wrapping_mul(100).checked_div(max).ok_or("0x42: división del casco inválida")?;
            if p == 0 && hull > 0 {
                p = 1
            }
            set(vars, c[2], p as u32)?
        }
        // 0x004F167C: bit 4 de `Town+0x2C8`.
        0x45 => {
            let c = need(3)?;
            let t = town(f.mem, v(vars, c[1])?);
            let x = f.mem.r32(t + 0x2c8) & 0x10 != 0;
            set(vars, c[2], u32::from(x))?
        }
        // 0x004F16B3: tarea 5 (investigación de la Liga) a la hora siguiente.
        0x46 => {
            let c = need(3)?;
            f.mem.w32(SCHED + 4, 0);
            let due = game_time(f.mem).wrapping_add(1);
            let rec = call(f, 0x004d8cf0, SCHED, &[due])?;
            f.mem.w32(SCHED + 4, 1);
            let count = ship_count(f.mem);
            let sea = f.mem.r16(SHIPS + 0xea);
            let docked = f.mem.r16(SHIPS + 0xe8);
            let pick = if sea < count {
                sea
            } else if docked < count {
                docked
            } else {
                0
            };
            let s = ship(f.mem, u32::from(pick));
            f.mem.w16(rec + 6, 5);
            f.mem.w8(rec + 8, v(vars, c[1])? as u8);
            f.mem.w8(rec + 9, v(vars, c[2])? as u8);
            f.mem.w8(rec + 0xa, 0xff);
            f.mem.w8(rec + 0xb, v(vars, c[2])? as u8);
            let now = game_time(f.mem);
            f.mem.w32(rec + 0xc, now);
            f.mem.w8(rec + 0x10, 8);
            f.mem.w8(rec + 0x11, 0);
            let x = f.mem.r16(s + 0x1c);
            f.mem.w16(rec + 0x12, x);
            let y = f.mem.r16(s + 0x20);
            f.mem.w16(rec + 0x14, y);
            f.mem.w16(rec + 0x16, 0xff);
        }
        // 0x004F17C0: unidades de una mercancía en la oficina.
        0x47 => {
            let c = need(5)?;
            let o = find_office(f, v(vars, c[1])?, v(vars, c[2])?, 0)?;
            if o == 0 {
                return Err("0x47 sin oficina: el original lee la dirección nula".into());
            }
            let ware = v(vars, c[3])?;
            let x = f.mem.r32(o.wrapping_add(ware.wrapping_mul(4)).wrapping_add(4));
            // El original guarda el valor bruto y relee la mercancía (puede ser la misma variable).
            set(vars, c[4], x)?;
            let ware = v(vars, c[3])?;
            set(vars, c[4], unscale(f.mem, ware, x))?
        }
        // 0x004F188D: suma a `Office+0x2E8` (el original no indexa por la mercancía).
        0x48 => {
            let c = need(5)?;
            let o = find_office(f, v(vars, c[1])?, v(vars, c[2])?, 0)?;
            if o == 0 {
                return Err("0x48 sin oficina: el original escribe en la dirección nula".into());
            }
            let amount = scaled(f.mem, v(vars, c[3])?, v(vars, c[4])?);
            let old = f.mem.r32(o + 0x2e8);
            f.mem.w32(o + 0x2e8, old.wrapping_add(amount as u32));
        }
        // 0x004F192D: valor base `cantidad · lote · base[mercancía]` en x87 (53 bits) truncado.
        0x49 => {
            let c = need(4)?;
            let ware = v(vars, c[1])?;
            let qty = v(vars, c[2])? as i32 as f32;
            let factor = lot(f.mem, ware) as f32;
            let base = f32::from_bits(f.mem.r32(WARE_BASE.wrapping_add(ware.wrapping_mul(4))));
            let x = f64::from(qty) * f64::from(factor) * f64::from(base);
            set(vars, c[3], crate::numeric::ftol_i32(x) as u32)?
        }
        // 0x004F1987: algún humano tiene una oferta de taberna en la ciudad.
        0x4a => {
            let c = need(3)?;
            let at = v(vars, c[1])? & 0xffff;
            set(vars, c[2], 0)?;
            for b in 0..u32::from(merchant_count(f.mem)) {
                let m = merchant(f.mem, b);
                if f.mem.r16(m + 8) != 0 {
                    continue;
                }
                let head = u32::from(f.mem.r16(m + 0xa));
                let r = call(f, 0x004d7900, POOL, &[head, at])? as u16;
                if r < f.mem.r16(POOL + 6) {
                    set(vars, c[2], 1)?;
                    break;
                }
            }
        }
        // 0x004F1A0F: precio de venta de `cantidad · lote`.
        0x4b => {
            let c = need(5)?;
            let ware = v(vars, c[1])?;
            let amount = scaled(f.mem, ware, v(vars, c[2])?);
            let r = call(f, 0x0052e1d0, PRICES, &[ware, v(vars, c[3])?, amount as u32])?;
            set(vars, c[4], r)?
        }
        // 0x004EE30B / 0x004EE44A: movimiento de existencias de la ciudad o de una oficina (acotado a lo que hay).
        0x4c | 0xee => {
            let c = need(if op == 0x4c { 5 } else { 6 })?;
            let (slot, ware, amount, dst) = if op == 0x4c {
                let t = v(vars, c[1])? & 0xff;
                let ware = v(vars, c[2])? & 0xff;
                let base = f.mem.r32(WORLD + 0x68);
                let at = base.wrapping_add(ware.wrapping_add(t.wrapping_mul(638)).wrapping_mul(4)).wrapping_add(4);
                (Some(at), ware, v(vars, c[3])?, c[4])
            } else {
                let who = v(vars, c[1])? & 0xff;
                let at = v(vars, c[2])? & 0xff;
                let ware = v(vars, c[3])? & 0xff;
                let amount = v(vars, c[4])?;
                let o = find_office(f, who, at, 0)?;
                ((o != 0).then(|| o.wrapping_add(ware * 4).wrapping_add(4)), ware, amount, c[5])
            };
            match slot {
                None => set(vars, dst, 0)?,
                Some(at) => {
                    let amount = scaled(f.mem, ware, amount);
                    let stock = f.mem.r32(at) as i32;
                    if amount < 0 {
                        let want = amount.wrapping_neg();
                        if stock >= want {
                            set(vars, dst, want as u32)?;
                            f.mem.w32(at, stock.wrapping_add(amount) as u32);
                        } else {
                            set(vars, dst, stock as u32)?;
                            f.mem.w32(at, 0);
                        }
                    } else {
                        set(vars, dst, amount as u32)?;
                        f.mem.w32(at, stock.wrapping_add(amount) as u32);
                    }
                }
            }
            let x = v(vars, dst)?;
            set(vars, dst, unscale(f.mem, ware, x))?
        }
        // 0x004F1A96: reputación social local `+= vars[b] / 1000` (`0x004F3770`).
        0x4d => {
            let c = need(4)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let delta = ((v(vars, c[2])? as i32 as f32) as f64 / f64::from(f32::from_bits(f.mem.r32(0x0066_bc78)))) as f32;
            let at = m.wrapping_add(v(vars, c[3])?.wrapping_mul(3).wrapping_add(0x48).wrapping_mul(4));
            let old = f32::from_bits(f.mem.r32(at));
            f.mem.w32(at, ((f64::from(delta) + f64::from(old)) as f32).to_bits());
        }
        // 0x004F1B13: peso de la carga `ship+0x118 / 200`.
        0x4e => {
            let c = need(3)?;
            let s = ship(f.mem, v(vars, c[1])?);
            let x = (f.mem.r32(s + 0x118) as i32) / 200;
            set(vars, c[2], x as u32)?
        }
        // 0x004F1C74: suma de partes de batalla naval (cartas 0x72) desde una fecha.
        0x50 => {
            let c = need(6)?;
            set(vars, c[5], 0)?;
            let key = v(vars, c[2])? as u16;
            let m = merchant(f.mem, v(vars, c[1])?);
            let mut at = f.mem.r16(m + 0xa);
            let mut steps = 0;
            while at < f.mem.r16(POOL + 6) {
                steps += 1;
                if steps > LOOP_LIMIT {
                    return Err("cadena de cartas cíclica".into());
                }
                let e = f.mem.r32(POOL).wrapping_add(u32::from(at) << 4);
                if f.mem.r8(e + 4) == 0x72 && f.mem.r16(e + 8) == key {
                    let when = date_to_ticks(f, e)?;
                    if when >= v(vars, c[3])? & 0xffff_ff00 {
                        let flags = v(vars, c[4])?;
                        for (bit, off) in [(1, 0xe), (2, 0xc), (4, 0xa)] {
                            if flags & bit != 0 {
                                let x = v(vars, c[5])?.wrapping_add(u32::from(f.mem.r16(e + off)));
                                set(vars, c[5], x)?
                            }
                        }
                    }
                }
                at = f.mem.r16(e + 6);
            }
        }
        // 0x004F1DCA: hunde (casco 0) el barco pirata de la banda `vars[a]` (estado 0x12).
        0x51 => {
            let c = need(2)?;
            let key = v(vars, c[1])? as u16;
            for i in 0..u32::from(ship_count(f.mem)) {
                let s = ship(f.mem, i);
                if f.mem.r16(s + 0x42) == key && f.mem.r16(s + 0x134) == 0x12 {
                    f.mem.w32(s + 0x18, 0);
                    mark_changed(f.mem, s, 0x100);
                    break;
                }
            }
        }
        // 0x004F1E38: consumo semanal de la ciudad y sus oficinas, `·7 / lote`.
        0x52 => {
            let c = need(4)?;
            let t = v(vars, c[1])? & 0xff;
            let ware = v(vars, c[2])? & 0xff;
            let tp = town(f.mem, t);
            let mut sum = f.mem.r32(tp + ware * 4 + 0x310).wrapping_add(f.mem.r32(tp + ware * 4 + 0x64));
            let mut at = f.mem.r16(tp + 0x784);
            let mut steps = 0;
            while at < office_count(f.mem) {
                steps += 1;
                if steps > LOOP_LIMIT {
                    return Err("cadena de oficinas cíclica".into());
                }
                let o = office(f.mem, u32::from(at));
                sum = sum.wrapping_add(f.mem.r32(o + ware * 4 + 0x64));
                at = f.mem.r16(o + 0x2ca);
            }
            let x = (sum.wrapping_mul(7) as i32).checked_div(lot(f.mem, ware)).ok_or("0x52: división inválida")?;
            set(vars, c[3], x as u32)?
        }
        // 0x004EFA54: barco en la ciudad listo para encabezar un convoy.
        0x54 => {
            let c = need(4)?;
            set(vars, c[3], u32::MAX)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let head = f.mem.r16(m + 0xe);
            for id in chain(f.mem, head)? {
                let s = ship(f.mem, u32::from(id));
                if u32::from(f.mem.r8(s + 0x39)) == v(vars, c[2])?
                    && can_lead_convoy(f.mem, s)
                    && f.mem.r8(s + 0x136) == 0
                    && f.mem.r16(s + 0x134) < 4
                {
                    set(vars, c[3], u32::from(id))?;
                    break;
                }
            }
        }
        // 0x004EFE91 / 0x004EFF63: barco ocioso con más armamento `+0x120` (con signo); 0x6D comprueba el puerto.
        0x55 | 0x6d => {
            let c = need(if op == 0x55 { 4 } else { 5 })?;
            let who = v(vars, c[1])?;
            let m = merchant(f.mem, who);
            let at = v(vars, c[2])? as u8;
            let port = if op == 0x6d { v(vars, c[4])? as u8 } else { 0 };
            set(vars, c[3], u32::MAX)?;
            // 0x6D relee el índice tras escribir -1 (si coincide con el destino, compara -1).
            if op == 0x6d && v(vars, c[1])? as i32 >= i32::from(merchant_count(f.mem)) {
                return Ok(());
            }
            let head = f.mem.r16(m + 0xe);
            let mut best = -1i32;
            for id in chain(f.mem, head)? {
                let s = ship(f.mem, u32::from(id));
                if !idle_in(f.mem, s, at) || (op == 0x6d && !port_ok(f.mem, s, port)) {
                    continue;
                }
                let arm = f.mem.r32(s + 0x120) as i32;
                if arm > best {
                    best = arm;
                    set(vars, c[3], u32::from(id))?
                }
            }
        }
        // 0x004F1F14: ciudad del asiento del consejo (`0x006DE52E`).
        0x56 => {
            let c = need(2)?;
            let x = f.mem.r8(WORLD + 0x8e);
            set(vars, c[1], u32::from(x))?
        }
        // 0x004F01AA: barco ocioso más rápido (`get_speed`); el original guarda `+0x120` como marca a batir.
        0x57 => {
            let c = need(4)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let head = f.mem.r16(m + 0xe);
            let at = v(vars, c[2])? as u8;
            set(vars, c[3], u32::MAX)?;
            let mut best = 0u32;
            for id in chain(f.mem, head)? {
                let s = ship(f.mem, u32::from(id));
                if !idle_in(f.mem, s, at) {
                    continue;
                }
                if call(f, 0x005170f0, s, &[])? > best {
                    best = f.mem.r32(s + 0x120);
                    set(vars, c[3], u32::from(id))?
                }
            }
        }
        // 0x004F1F2C / 0x004F1F47: lectura y escritura indirectas de variables.
        0x58 => {
            let c = need(3)?;
            let x = vu(vars, v(vars, c[1])?)?;
            set(vars, c[2], x)?
        }
        0x59 => {
            let c = need(3)?;
            let x = v(vars, c[2])?;
            setu(vars, v(vars, c[1])?, x)?
        }
        // 0x004F00AC: barco ocioso con más carga de una mercancía.
        0x5a => {
            let c = need(5)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let head = f.mem.r16(m + 0xe);
            let at = v(vars, c[2])? as u8;
            let ware = v(vars, c[3])? & 0xff;
            set(vars, c[4], u32::MAX)?;
            let mut best = 0u32;
            for id in chain(f.mem, head)? {
                let s = ship(f.mem, u32::from(id));
                if !idle_in(f.mem, s, at) {
                    continue;
                }
                let x = f.mem.r32(s + ware * 4 + 0x54);
                if x > best {
                    best = x;
                    set(vars, c[4], u32::from(id))?
                }
            }
        }
        // 0x004F1F62: ciudad más cercana por ruta (`0x00448610`).
        0x5b => {
            let c = need(3)?;
            let routes = f.mem.r32(ROUTES_PTR);
            let r = call(f, 0x00448610, routes, &[v(vars, c[1])? & 0xffff])?;
            set(vars, c[2], r & 0xff)?
        }
        // 0x004F1FB3: noticia de batalla (cartas 0x3A/0x39) del buzón general, de la más nueva hacia atrás.
        0x5c => {
            let c = need(5)?;
            set(vars, c[4], 0)?;
            let mut at = f.mem.r16(POOL + 8);
            let mut steps = 0;
            while at < f.mem.r16(POOL + 6) {
                steps += 1;
                if steps > LOOP_LIMIT {
                    return Err("cadena de cartas cíclica".into());
                }
                let e = f.mem.r32(POOL).wrapping_add(u32::from(at) << 4);
                let when = date_to_ticks(f, e)?;
                if v(vars, c[3])? as i32 > when as i32 {
                    break;
                }
                let kind = f.mem.r8(e + 4);
                let place = u32::from(f.mem.r8(e + 5));
                if kind == 0x3a && place == v(vars, c[2])? {
                    if u32::from(f.mem.r8(e + 0xf)) == v(vars, c[1])? {
                        set(vars, c[4], 1)?;
                        break;
                    }
                } else if kind == 0x39 && place == v(vars, c[2])? && u32::from(f.mem.r8(e + 0xd)) == v(vars, c[1])? {
                    set(vars, c[4], 1)?;
                    break;
                }
                at = f.mem.r16(e + 6);
            }
        }
        // 0x004F20E6: `transfer_ship_ownership`.
        0x5d => {
            let c = need(4)?;
            let x = v(vars, c[3])?;
            call(f, 0x00509250, SHIPS, &[v(vars, c[1])?, v(vars, c[2])?, x, x, 0xff])?;
        }
        // 0x004F212B: edificios de un tipo del comerciante en la ciudad (cadena desde la oficina).
        0x5e => {
            let c = need(5)?;
            set(vars, c[4], 0)?;
            let o = find_office(f, v(vars, c[1])?, v(vars, c[2])?, 0)?;
            if o != 0 {
                let mut at = f.mem.r16(o + 0x2ce);
                let t = town(f.mem, v(vars, c[2])?);
                let mut steps = 0;
                while at < f.mem.r16(t + 0x776) {
                    steps += 1;
                    if steps > LOOP_LIMIT {
                        return Err("cadena de edificios cíclica".into());
                    }
                    let rec = f.mem.r32(t + 0x75c).wrapping_add(u32::from(at) * 8);
                    if u32::from(f.mem.r8(rec + 3)) == v(vars, c[3])? {
                        let x = v(vars, c[4])?.wrapping_add(1);
                        set(vars, c[4], x)?
                    }
                    at = f.mem.r16(rec + 4);
                }
            }
        }
        // 0x004F2216: traspasa hasta `vars[e]` edificios de un tipo a otro comerciante (crea la oficina).
        0x5f => {
            let c = need(7)?;
            if v(vars, c[5])? != 0 {
                let o = find_office(f, v(vars, c[1])?, v(vars, c[3])?, 1)?;
                if o != 0 {
                    let mut di = f.mem.r16(o + 0x2ce);
                    let t = town(f.mem, v(vars, c[3])?);
                    let mut done = 0u32;
                    let mut steps = 0;
                    while di < f.mem.r16(t + 0x776) {
                        steps += 1;
                        if steps > LOOP_LIMIT {
                            return Err("cadena de edificios cíclica".into());
                        }
                        let rec = f.mem.r32(t + 0x75c).wrapping_add(u32::from(di) * 8);
                        let next = f.mem.r16(rec + 4);
                        if u32::from(f.mem.r8(rec + 3)) == v(vars, c[4])? {
                            let x = v(vars, c[6])?;
                            call(f, 0x005256f0, t, &[u32::from(di), v(vars, c[2])? & 0xff, x, x])?;
                            done = done.wrapping_add(1);
                            if done == v(vars, c[5])? {
                                break;
                            }
                        }
                        di = next;
                    }
                }
            }
        }
        // 0x004F24E3: alcalde (`Town+0x6F1`).
        0x60 => {
            let c = need(3)?;
            let t = town(f.mem, v(vars, c[1])?);
            let x = f.mem.r8(t + 0x6f1);
            set(vars, c[2], u32::from(x))?
        }
        // 0x004F2512: suma saturada a `Merchant+0x16` (`0x004F36E0`).
        0x61 => {
            let c = need(3)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let add = v(vars, c[2])? as u16 as i16;
            let cur = f.mem.r16(m + 0x16) as i16;
            let r = if add > 0 {
                if i32::from(cur) >= 0x7fff - i32::from(add) {
                    0x7fff
                } else {
                    cur.wrapping_add(add) as u16
                }
            } else if i32::from(cur) + 0x8000 > -i32::from(add) {
                cur.wrapping_add(add) as u16
            } else {
                0x8000
            };
            f.mem.w16(m + 0x16, r);
        }
        // 0x004F2565: existencias de la ciudad en lotes.
        0x62 => {
            let c = need(4)?;
            let ware = v(vars, c[1])?;
            let t = v(vars, c[2])?;
            let base = f.mem.r32(WORLD + 0x68);
            let at = base.wrapping_add(ware.wrapping_add(t.wrapping_mul(638)).wrapping_mul(4)).wrapping_add(4);
            // El original escribe primero las existencias en `vars[c3]` y después vuelve a leer `vars[c1]` (lote) y
            // `vars[c3]` (dividendo): si destino y mercancía son la misma variable, el lote se busca con las existencias.
            set(vars, c[3], f.mem.r32(at))?;
            let ware = v(vars, c[1])?;
            let x = (v(vars, c[3])? as i32).checked_div(lot(f.mem, ware)).ok_or("0x62: división inválida")?;
            set(vars, c[3], x as u32)?
        }
        // 0x004F2363: operación 0x28 (obra) para hasta `vars[c]` edificios de un tipo del comerciante.
        0x63 => {
            let c = need(5)?;
            if v(vars, c[3])? != 0 {
                let o = find_office(f, v(vars, c[1])?, v(vars, c[4])?, 0)?;
                if o != 0 {
                    let mut di = f.mem.r16(o + 0x2ce);
                    let t = town(f.mem, v(vars, c[4])?);
                    let mut done = 0u32;
                    let mut steps = 0;
                    while di < f.mem.r16(t + 0x776) {
                        steps += 1;
                        if steps > LOOP_LIMIT {
                            return Err("cadena de edificios cíclica".into());
                        }
                        let rec = f.mem.r32(t + 0x75c).wrapping_add(u32::from(di) * 8);
                        if u32::from(f.mem.r8(rec + 3)) == v(vars, c[2])? {
                            let mut r = op_record();
                            r[0] = 0x28;
                            r[1] = v(vars, c[4])?;
                            r[2] = v(vars, c[1])?;
                            r[3] = u32::from(f.mem.r8(rec));
                            r[4] = u32::from(f.mem.r8(rec + 1));
                            enqueue(f, &r)?;
                            done = done.wrapping_add(1);
                            if done == v(vars, c[3])? {
                                break;
                            }
                        }
                        di = f.mem.r16(rec + 4);
                    }
                }
            }
        }
        // 0x004F25E8: marca 0x20 de `ship+0x136` en una fracción `(100 - vars[a]) %` de los barcos de la IA.
        0x64 => {
            let c = need(2)?;
            let frac = ((100u32.wrapping_sub(v(vars, c[1])?)).wrapping_shl(16) as i32 / 100) as u32;
            let mut acc = 0u32;
            let mut b = 0u8;
            while u16::from(b) < merchant_count(f.mem) {
                let m = merchant(f.mem, u32::from(b));
                let flags = f.mem.r16(m + 8);
                if flags != 0 && flags & 6 == 0 {
                    let head = f.mem.r16(m + 0xe);
                    for id in chain(f.mem, head)? {
                        let s = ship(f.mem, u32::from(id));
                        acc = acc.wrapping_add(frac);
                        let x = f.mem.r8(s + 0x136);
                        if acc >= 0x10000 {
                            f.mem.w8(s + 0x136, x | 0x20);
                            acc &= 0xffff;
                        } else {
                            f.mem.w8(s + 0x136, x & 0xdf);
                        }
                    }
                }
                b = b.wrapping_add(1);
                if b == 0 {
                    return Err("0x64: más de 255 comerciantes".into());
                }
            }
        }
        // 0x004F272E: barco pirata (estado 0x12) de la banda `vars[a]`.
        0x66 => {
            let c = need(3)?;
            set(vars, c[2], u32::MAX)?;
            let key = v(vars, c[1])? as u16;
            for i in 0..u32::from(ship_count(f.mem)) {
                let s = ship(f.mem, i);
                if f.mem.r16(s + 0x42) == key && f.mem.r16(s + 0x134) == 0x12 {
                    set(vars, c[2], i)?;
                    break;
                }
            }
        }
        // 0x004F27BE: `create_docked_ship_for_merchant` en la ciudad `vars[d] mod número de ciudades`.
        0x67 => {
            let c = need(6)?;
            let kind = v(vars, c[2])? & 0xffff;
            let who = v(vars, c[1])? & 0xff;
            let towns = towns_count(f.mem);
            let at = (v(vars, c[4])? & 0xff).checked_rem(towns).ok_or("0x67 sin ciudades: división por cero")?;
            let r = call(f, 0x0050e000, SHIPS, &[kind, v(vars, c[3])?, at, who])?;
            set(vars, c[5], r & 0xffff)?
        }
        // 0x004F286E: carta de mapa 0x22 (dificultad derivada de dos variables y de la hora).
        0x68 => {
            let c = need(9)?;
            let mut r = letter_init();
            let recipient = v(vars, c[3])?;
            let m = merchant(f.mem, recipient);
            let female = f.mem.r8(m + 0x1a) != 0;
            let a = (v(vars, c[7])? as i32) % 3;
            let b = (v(vars, c[8])? as i32) % 3;
            let mut extra: i32 = if b > a {
                if b <= 1 {
                    0
                } else {
                    5
                }
            } else if b >= 3 {
                if game_time(f.mem) & 1 != 0 {
                    0
                } else {
                    10
                }
            } else if a <= 1 {
                5
            } else {
                10
            };
            extra = extra.wrapping_add((game_time(f.mem) % 5) as i32);
            r[0xa] = ((a + b) >> 1) as u8 & 3;
            r[0xb] = v(vars, c[4])? as u8;
            r[0xc] = v(vars, c[5])? as u8;
            if female {
                r[0xc] |= 0x80;
                r[0xe] = 0
            } else {
                r[0xe] = 0xf
            }
            r[0xd] = v(vars, c[6])? as u8;
            r[0xe] = r[0xe].wrapping_add(extra as u8);
            r[8] = v(vars, c[1])? as u8;
            r[9] = v(vars, c[2])? as u8;
            r[0xf] = 0xe;
            r[4] = 0x22;
            r[5] = v(vars, c[4])? as u8;
            add_message(f, recipient, r)?
        }
        // 0x004F2A89: tarea 0xF (denuncia) a la hora siguiente.
        0x69 => {
            let c = need(4)?;
            f.mem.w32(SCHED + 4, 0);
            let due = game_time(f.mem).wrapping_add(1);
            let rec = call(f, 0x004d8cf0, SCHED, &[due])?;
            f.mem.w32(SCHED + 4, 1);
            let now = game_time(f.mem);
            f.mem.w16(rec + 8, now as u16);
            f.mem.w8(rec + 0xa, v(vars, c[1])? as u8);
            f.mem.w8(rec + 0xc, v(vars, c[2])? as u8);
            f.mem.w8(rec + 0xb, v(vars, c[3])? as u8);
            f.mem.w16(rec + 6, 0xf);
        }
        // 0x004F2B3A: `n`-ésimo comerciante con oficina en la ciudad (el último si hay menos).
        0x6a => {
            let c = need(4)?;
            set(vars, c[3], 0)?;
            let mut left = v(vars, c[2])? as i32;
            let mut di = 0u32;
            while left >= 0 {
                if di as u16 >= merchant_count(f.mem) {
                    break;
                }
                if find_office(f, di, v(vars, c[1])?, 0)? != 0 {
                    left -= 1;
                    set(vars, c[3], di & 0xffff)?
                }
                di += 1;
            }
        }
        // 0x004F2BDA: marca 0x40 de `ship+0x136`.
        0x6c => {
            let c = need(3)?;
            let id = v(vars, c[1])? as u8;
            let on = v(vars, c[2])? as u8 != 0;
            if ship_count(f.mem) > u16::from(id) {
                let s = ship(f.mem, u32::from(id));
                let x = f.mem.r8(s + 0x136);
                f.mem.w8(s + 0x136, if on { x | 0x40 } else { x & 0xbf });
            }
        }
        // 0x004EFCB9: siguiente ciudad de una ruta sin repetir las tres últimas (`find_nearest_town_excluding`).
        0x6f => {
            let c = need(7)?;
            let (a, b, d) = (v(vars, c[1])?, v(vars, c[2])?, v(vars, c[3])?);
            let from = v(vars, c[4])?;
            let flag = u32::from(v(vars, c[5])? != 0);
            let x = v(vars, c[6])? & 0xffff;
            let r = call(f, 0x00533210, WORLD, &[from, a, b, d, 3, flag, x])?;
            // El original copia en cadena releyendo variables ya escritas (los índices pueden repetirse).
            let _ = (b, d);
            let t = v(vars, c[2])?;
            set(vars, c[1], t)?;
            let t = v(vars, c[3])?;
            set(vars, c[2], t)?;
            let t = v(vars, c[4])?;
            set(vars, c[3], t)?;
            set(vars, c[4], r & 0xff)?;
        }
        // 0x004ED018: ciudades con ranura asignada (`Town+0x82C < 0x14`).
        0x71 => {
            let c = need(2)?;
            set(vars, c[1], 0)?;
            let mut t = 0;
            while t < towns_count(f.mem) {
                let tp = town(f.mem, t);
                if f.mem.r8(tp + 0x82c) < 0x14 {
                    let x = v(vars, c[1])?.wrapping_add(1);
                    set(vars, c[1], x)?
                }
                t += 1;
                if t as usize > LOOP_LIMIT {
                    return Err("0x71: demasiadas ciudades".into());
                }
            }
        }
        // 0x004EF062: primer barco que no está en un convoy pequeño (< 3) y no tiene estado 0x14.
        0x72 => {
            let c = need(3)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let count = ship_count(f.mem);
            let mut di = f.mem.r16(m + 0xe);
            let mut steps = 0;
            while di < count {
                steps += 1;
                if steps > LOOP_LIMIT {
                    return Err("cadena de barcos cíclica".into());
                }
                let s = ship(f.mem, u32::from(di));
                let group = f.mem.r16(s + 8);
                let small = group < convoy_count(f.mem) && {
                    let cv = convoy(f.mem, u32::from(group));
                    f.mem.r16(cv + 0x38) < 3
                };
                if !small && f.mem.r16(s + 0x134) != 0x14 {
                    break;
                }
                di = f.mem.r16(s + 4);
            }
            set(vars, c[2], u32::from(di))?
        }
        // 0x004EEC65: `Merchant+0x31` es una ciudad válida (cónyuge).
        0x73 => {
            let c = need(3)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let b = u32::from(f.mem.r8(m + 0x31));
            set(vars, c[2], u32::from(towns_count(f.mem) > b))?
        }
        // 0x004F051A: barco con capacidad por encima de `vars[c]·200`; uno en convoy no sustituye al anterior.
        0x74 => {
            let c = need(6)?;
            let m = merchant(f.mem, v(vars, c[1])?);
            let head = f.mem.r16(m + 0xe);
            let at = v(vars, c[2])? as u8;
            let port = v(vars, c[5])? as u8;
            let threshold = v(vars, c[3])?.wrapping_mul(200);
            set(vars, c[4], u32::MAX)?;
            let mut prev = 0xffffu32;
            for id in chain(f.mem, head)? {
                let s = ship(f.mem, u32::from(id));
                if !idle_in(f.mem, s, at) || !port_ok(f.mem, s, port) {
                    continue;
                }
                if recompute_load(f, s)? > threshold {
                    let mut chosen = u32::from(id);
                    if ship_count(f.mem) > prev as u16 && convoy_count(f.mem) > f.mem.r16(s + 8) {
                        chosen = prev
                    }
                    set(vars, c[4], chosen & 0xffff)?;
                    prev = chosen;
                }
            }
        }
        // 0x004ED00A: `assign_town_slots` con el planificador como `this`.
        0xe4 => {
            call(f, 0x004dc800, SCHED, &[])?;
        }
        // 0x004EE698: gran incendio (siete operaciones 0x7D sobre edificios de tipos 0x53…0x97).
        0xe5 => {
            let c = need(2)?;
            let t = v(vars, c[1])? as u8;
            if f.mem.r32(OPS + 0x2c) != 0 || u32::from(t) >= towns_count(f.mem) {
                return Ok(());
            }
            let tp = town(f.mem, u32::from(t));
            let mut r = op_record();
            set_u8(&mut r, 0xe, t);
            set_u16(&mut r, 0xa, fire_damage(f.mem, tp));
            set_u16(&mut r, 8, 0);
            r[1] = game_time(f.mem).wrapping_add(10);
            let mut kind = 0x53u32;
            while kind < 0x9b {
                let (x, y) = (get_u8(&r, 0xc), get_u8(&r, 0xd));
                let out = call(f, 0x00525b90, tp, &[kind, u32::from(x), u32::from(y)])?;
                set_u8(&mut r, 0xc, out as u8);
                set_u8(&mut r, 0xd, (out >> 8) as u8);
                if out as u8 != 0xff {
                    r[0] = 0x7d;
                    enqueue(f, &r)?
                }
                kind += 0xc;
            }
        }
        // 0x004F2C3E: las cuatro satisfacciones de la ciudad (`Town+0x300..0x308`).
        0xe6 => {
            let c = need(3)?;
            let x = v(vars, c[2])? as u16;
            let t = v(vars, c[1])? & 0xff;
            if t < towns_count(f.mem) {
                for k in 0..4 {
                    let tp = town(f.mem, t);
                    f.mem.w16(tp + 0x300 + 2 * k, x);
                }
            }
        }
        // 0x004ED086: notificación 0x26 con la ciudad natal; con un comerciante fuera de la tabla el original repite
        // el comando sin fin.
        0xe8 => {
            let c = need(2)?;
            let who = v(vars, c[1])? as u16;
            if who >= merchant_count(f.mem) {
                return Err("0xE8 con comerciante fuera de la tabla: el original no avanza".into());
            }
            let m = merchant(f.mem, u32::from(who));
            let home = u32::from(f.mem.r8(m + 0x19));
            call(f, 0x00548ca0, OPS, &[u32::from(who), 0, 0, home, 0x26, 0])?;
        }
        // 0x004EE9DB: casco a cero si el barco está en una ciudad distinta de 0.
        0xea => {
            let c = need(2)?;
            let id = v(vars, c[1])? as u16;
            if id < ship_count(f.mem) {
                let s = ship(f.mem, u32::from(id));
                if f.mem.r8(s + 0x39) != 0 {
                    f.mem.w32(s + 0x18, 0);
                    recompute_load(f, s)?;
                }
            }
        }
        // 0x004EE984: tripulación `ship+0x40`.
        0xeb => {
            let c = need(3)?;
            let id = v(vars, c[1])? as u16;
            if id < ship_count(f.mem) {
                let s = ship(f.mem, u32::from(id));
                f.mem.w16(s + 0x40, v(vars, c[2])? as u16);
                recompute_load(f, s)?;
            }
        }
        // 0x004EE7D0: peste (operación 0x7C) con duración y nombre aleatorios (`rand`).
        0xec => {
            let c = need(2)?;
            if f.mem.r32(OPS + 0x2c) != 0 {
                return Ok(());
            }
            let t = v(vars, c[1])? as u8;
            let tp = town(f.mem, u32::from(t));
            if f.mem.r32(tp + 0x2c8) & 8 != 0 {
                return Ok(());
            }
            let mut r = op_record();
            set_u8(&mut r, 0x12, t);
            let a = i32::from(f.mem.r16(tp + 0x790)).wrapping_mul(100);
            let b = i32::from(f.mem.r16(tp + 0x792)).wrapping_mul(75);
            let mut level = i32::from(a < b);
            let care = i32::from(f.mem.r8(tp + 0x771)) + i32::from(f.mem.r8(tp + 0x773));
            let pop = f.mem.r32(tp + 0x2d4) as i32;
            if care.wrapping_mul(50000) < pop.wrapping_mul(8) {
                level += 1
            }
            let d = (call(f, 0x00639c03, 0, &[])? & 3) + 1;
            // El original suma el dword `GameWorld+4`, pero solo guarda la palabra baja (día del año).
            set_u16(&mut r, 0x10, f.mem.r16(WORLD + 4).wrapping_add(d as u16));
            let e = (call(f, 0x00639c03, 0, &[])? as i32 % 10).wrapping_add(d as i32);
            let span = (e.wrapping_add(level * 10) as u32).wrapping_shl(8);
            r[2] = span.wrapping_add(game_time(f.mem));
            for k in 0..4 {
                let ch = (call(f, 0x00639c03, 0, &[])? as u8 & 0x1f).wrapping_add(0x31);
                set_u8(&mut r, 4 + k, ch);
            }
            let tp = town(f.mem, u32::from(get_u8(&r, 0x12)));
            let pop = f.mem.r32(tp + 0x2d4) as i32;
            set_u16(&mut r, 0xc, 0);
            // El original escribe en `[esp+0x9E]` después de un `push`: es `+0xE`; `+0x12` conserva la ciudad.
            set_u16(&mut r, 0xe, (pop / 40) as u16);
            r[0] = 0x7c;
            enqueue(f, &r)?
        }
        // 0x004EE567: incendio (operación 0x7D) en la ciudad de índice literal `cmd[1]`.
        0xed => {
            let c = need(2)?;
            if f.mem.r32(OPS + 0x2c) != 0 {
                return Ok(());
            }
            let tp = town(f.mem, u32::from(c[1]));
            let mut r = op_record();
            set_u8(&mut r, 0xe, v(vars, c[1])? as u8);
            if f.mem.r32(tp + 0x2c8) & 0x8000 != 0 {
                return Ok(());
            }
            set_u16(&mut r, 0xa, fire_damage(f.mem, tp));
            set_u16(&mut r, 8, 0);
            r[1] = game_time(f.mem).wrapping_add(10);
            let out = call(f, 0x00525b90, tp, &[u32::MAX, 0, 0])?;
            set_u8(&mut r, 0xc, out as u8);
            set_u8(&mut r, 0xd, (out >> 8) as u8);
            if out as u8 != 0xff {
                r[0] = 0x7d;
                enqueue(f, &r)?
            }
        }
        // 0x004EE25C: aviso en la ventana de sucesos (solo presentación).
        0xef => {
            let c = need(2)?;
            v(vars, c[1])?;
        }
        // 0x004EE1CD: carga o descarga de mercancía de misión.
        0xf1 => {
            let c = need(4)?;
            let s = ship(f.mem, v(vars, c[1])?);
            let n = v(vars, c[2])?;
            if n as i32 > 0 {
                let r = call(f, 0x00518640, s, &[n])?;
                set(vars, c[3], r & 0xff)?
            } else {
                let arg = (n & !0xff) | u32::from((n as u8).wrapping_neg());
                call(f, 0x005186d0, s, &[arg])?;
            }
        }
        // 0x004EE1A2: `ship_add_counter_0x157` (vuelve a 0 al llegar a 0x7F).
        0xf2 => {
            let c = need(3)?;
            let s = ship(f.mem, v(vars, c[1])?);
            let mut x = f.mem.r8(s + 0x157).wrapping_add(v(vars, c[2])? as u8);
            if x >= 0x7f {
                x = 0
            }
            f.mem.w8(s + 0x157, x);
            mark_changed(f.mem, s, 4);
        }
        // 0x004EE155: modo `ship+0x3F` y bit 1 de `+0x3D` salvo en estado 0x11.
        0xf3 => {
            let c = need(3)?;
            let s = ship(f.mem, v(vars, c[1])?);
            if f.mem.r16(s + 0x134) != 0x11 {
                f.mem.w8(s + 0x3f, v(vars, c[2])? as u8);
                let x = f.mem.r8(s + 0x3d);
                f.mem.w8(s + 0x3d, x | 2);
            }
        }
        // 0x004EE08A: suma o resta carga a bordo, limitada por la capacidad y redondeada al lote.
        0xf4 => {
            let c = need(4)?;
            let s = ship(f.mem, v(vars, c[1])?);
            let ware = v(vars, c[2])?;
            let mut amount = scaled(f.mem, ware, v(vars, c[3])?);
            let at = s.wrapping_add(ware.wrapping_mul(4)).wrapping_add(0x54);
            if amount < 0 {
                let x = f.mem.r32(at) as i32;
                f.mem.w32(at, if x < amount.wrapping_neg() { 0 } else { amount.wrapping_add(x) as u32 });
            } else {
                if (recompute_load(f, s)? as i32) < amount {
                    amount = recompute_load(f, s)? as i32
                }
                let unit = if small_lot(f.mem, ware) { 200 } else { 2000 };
                amount -= amount % unit;
                let x = f.mem.r32(at);
                f.mem.w32(at, x.wrapping_add(amount as u32));
            }
            recompute_load(f, s)?;
            mark_changed(f.mem, s, 4);
        }
        // 0x004EE012: pergamino de texto para el jugador local (solo presentación).
        0xf5 => {
            let c = need(6)?;
            v(vars, c[1])?;
        }
        // 0x004EDF3F: carta de tesoro 0x45/0x46 con las coordenadas de la ciudad.
        0xf6 => {
            let c = need(6)?;
            let mut r = letter_init();
            r[5] = v(vars, c[4])? as u8;
            r[4] = (v(vars, c[5])? as u8 & 1) + 0x45;
            r[8] = 0;
            r[9] = 0;
            r[0xa..0xc].copy_from_slice(&(v(vars, c[3])? as u16).to_le_bytes());
            let xy = TOWN_COORDS.wrapping_add(v(vars, c[1])?.wrapping_mul(8));
            r[0xc..0xe].copy_from_slice(&f.mem.r16(xy).to_le_bytes());
            r[0xe..0x10].copy_from_slice(&f.mem.r16(xy + 4).to_le_bytes());
            add_message(f, v(vars, c[2])?, r)?
        }
        // 0x004ED3A6: asedio (operación 0x8E) si la ciudad tiene muralla.
        0xfa => {
            let c = need(2)?;
            let t = v(vars, c[1])?;
            if t >= towns_count(f.mem) {
                return Ok(());
            }
            let tp = town(f.mem, t);
            if call(f, 0x005252f0, tp, &[])? as u8 == 0xff {
                return Ok(());
            }
            let mut r = op_record();
            r[0] = 0x8e;
            r[1] = t;
            r[4] = 0;
            let year = f.mem.r16(WORLD + 2);
            r[2] = if year < 0x546 {
                0
            } else if year < 0x578 {
                game_time(f.mem) & 1
            } else {
                game_time(f.mem) % 3
            };
            let towns0 = f.mem.r32(WORLD + 0x68);
            let pop0 = f.mem.r32(towns0 + 0x2d4) as i32;
            r[3] = (pop0 % 256) as u32;
            enqueue(f, &r)?
        }
        _ => return Err(format!("comando {op:#04x} no pertenece al módulo plano")),
    }
    Ok(())
}

/// Memoria que anota el valor previo de cada byte escrito para deshacer un guion fallido.
pub struct Journal<'a> {
    inner: &'a mut dyn Mem,
    log: Vec<(u32, u8)>,
}

impl<'a> Journal<'a> {
    pub fn new(inner: &'a mut dyn Mem) -> Self {
        Journal { inner, log: Vec::new() }
    }
    /// Restaura los bytes escritos, del último al primero.
    pub fn undo(&mut self) {
        while let Some((a, b)) = self.log.pop() {
            self.inner.w8(a, b)
        }
    }
}

impl Mem for Journal<'_> {
    fn r8(&mut self, a: u32) -> u8 {
        self.inner.r8(a)
    }
    fn w8(&mut self, a: u32, v: u8) {
        let old = self.inner.r8(a);
        self.log.push((a, old));
        self.inner.w8(a, v)
    }
}
