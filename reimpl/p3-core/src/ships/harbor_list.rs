//! J05 paso 1: lista de barcos locales de la escena del puerto (`LocalMapShipList`).
//! `ensure_free_ship_slot` 0x0061A100, `local_map_ship_init_motion` 0x00626490, `local_map_ship_list_add_ship` 0x00618A30 y
//! `harbor_scene_add_berthed_ships` 0x00613920. Probadas contra el x86 original en emulación (`oracle/x86_harbor_list.py`).
//! Límites: `operator new` no deja basura (el original deja bytes sin inicializar en los campos que el constructor no escribe);
//! los accesos fuera de rango devuelven error en lugar de leer memoria ajena.

use super::battle::construct_local_ship;
use super::{r16, r32};

const SHIP: usize = 0x180;
const CONVOY: usize = 60;
const LOCAL: usize = 0x164;
const NIL: u16 = 0xFFFF;

/// Lista de barcos locales (campos de `this`: +4 punteros, +8 y +0xC enlaces de `u16`, +0x10 cabeza, +0x12 libre, +0x14 recuento, +0x16 capacidad).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ShipList {
    /// Dirección lógica de la lista (se guarda en `+0x12C` de cada barco local).
    pub address: u32,
    pub ships: Vec<Option<Vec<u8>>>,
    pub link8: Vec<u16>,
    pub link12: Vec<u16>,
    pub head: u16,
    pub free: u16,
    pub count: u16,
    pub capacity: u16,
    /// `+0x5C`: ciudad de la escena.
    pub town: u16,
    /// `+0x68`: altura por ranura (en 1/32).
    pub heights: Vec<u16>,
    /// `+0x2F3C`: `[(ciudad * 256 - ciudad) + ranura]` = barco del mundo.
    pub town_ships: Vec<u16>,
    /// `escena + 0xC658`: máximo de barcos locales.
    pub limit: u32,
    /// `+0x266` y `+0x1256` (palabras), `+0x2246` (bytes), `+0x2A3E` (bytes por ranura): puntos de la ruta de entrada o salida; ocho por ranura.
    pub entry_x: Vec<u16>,
    pub entry_y: Vec<u16>,
    pub entry_flags: Vec<u8>,
    pub entry_len: Vec<u8>,
    /// `+0x60`: objeto del carril (distinto de cero: tras sincronizar se refresca su progreso, `0x5708F0`).
    pub lane: u32,
    /// `+0x64`: reloj de la última sincronización.
    pub last_sync: u32,
    /// `+0x2B40 + ranura * 4`: longitud de la ruta de cada ranura (suma de tramos, y escalado 34/22).
    pub speeds: Vec<i32>,
    /// `+0x7EEC`: se pone a cero al entrar en la ciudad.
    pub field_7eec: u32,
}

/// Rutas de acceso por ranura (`escena + 0xC670`: `{recuento, puntero}` de 8 bytes; cada punto ocupa 8 bytes con `x` en +0 e `y` en +4).
pub struct SceneRoutes<'a> {
    pub routes: &'a [Vec<(u16, u16)>],
    /// `escena + 0xC324`: ciudad que se está viendo (el sonido solo suena en ella).
    pub shown_town: u32,
}

pub trait SoundHost {
    /// `0x59A090` (`town_view_play_ship_sound_at_position`): posición de pantalla del barco.
    fn ship_sound(&mut self, x: i32, y: i32, z: i32);
}

/// Estado global que leen estas funciones.
pub struct HarborWorld<'a> {
    /// `DAT_006DD7A4`: registros de barco de 0x180 bytes (los índices más allá del vector se leen como ceros).
    pub ships: &'a [u8],
    /// `DAT_006DD7A8`: registros de convoy de 60 bytes.
    pub convoys: &'a [u8],
    /// `DAT_006DD894`, `DAT_006DD896`
    pub ship_count: u16,
    pub convoy_count: u16,
    /// `DAT_006DFC14`, `DAT_006DFC18`, `DAT_006DE4AA`
    pub player: u32,
    pub hotseat: u32,
    pub merchant_count: u16,
    /// `*(DAT_006CBDF0 + 0x18C)`
    pub strict_convoy_flag: u32,
    /// Palabra en `+8` del comerciante `n` (0 = humano).
    pub merchant_word: &'a dyn Fn(u32) -> u16,
}

impl HarborWorld<'_> {
    fn ship_byte(&self, id: u32, at: usize) -> u8 {
        self.ships.get(id as usize * SHIP + at).copied().unwrap_or(0)
    }
    fn ship_u16(&self, id: u32, at: usize) -> u16 {
        u16::from(self.ship_byte(id, at)) | u16::from(self.ship_byte(id, at + 1)) << 8
    }
    fn conv_u16(&self, id: u32, at: usize) -> u16 {
        let b = |i: usize| self.convoys.get(id as usize * CONVOY + i).copied().unwrap_or(0);
        u16::from(b(at)) | u16::from(b(at + 1)) << 8
    }
    fn conv_byte(&self, id: u32, at: usize) -> u8 {
        self.convoys.get(id as usize * CONVOY + at).copied().unwrap_or(0)
    }
}

/// `0x61A100`: si no hay ranura libre, amplía en cuatro y encadena las nuevas. Devuelve la cabeza de libres.
pub fn ensure_free_ship_slot(l: &mut ShipList) -> u16 {
    if l.free == NIL && l.capacity < 0xFFFB {
        let old = l.capacity;
        l.free = old;
        l.capacity = old + 4;
        let cap = usize::from(l.capacity);
        l.ships.resize(cap, None);
        l.link8.resize(cap, 0);
        l.link12.resize(cap, 0);
        let mut i = usize::from(old);
        while i < cap {
            l.ships[i] = None;
            l.link8[i] = (i as u16).wrapping_add(1);
            l.link12[i] = (i as u16).wrapping_sub(1);
            i += 1;
        }
        l.link8[usize::from(l.free)] = NIL;
        l.link12[cap - 1] = NIL;
    }
    l.free
}

/// `0x626490` (`this` = barco local + 4): bloque de movimiento inicial. `a1`/`a2` se truncan a palabra como el original.
pub fn init_motion(object: &mut [u8], a1: u32, a2: u32, a3: u8) {
    let w = |o: &mut [u8], at: usize, v: u16| o[at..at + 2].copy_from_slice(&v.to_le_bytes());
    w(object, 4, 0);
    w(object, 6, a1 as u16);
    w(object, 8, 0);
    w(object, 0xA, a2 as u16);
    object[0xD] = a3;
    object[0x22] = 4;
    w(object, 0x20, NIL);
}

/// Lectura de la tabla de punteros: más allá de la capacidad el original encuentra ceros (se modela como ranura vacía).
fn slot_ref(l: &ShipList, i: u16) -> Result<&Option<Vec<u8>>, &'static str> {
    static NONE: Option<Vec<u8>> = None;
    Ok(l.ships.get(usize::from(i)).unwrap_or(&NONE))
}

/// `0x618A30`: añade el barco `id` (o devuelve la ranura que ya lo tiene) y lo pone al frente de la lista. `0xFFFF` = rechazado.
pub fn add_ship(l: &mut ShipList, w: &HarborWorld, id: u32, x: u32) -> Result<u16, &'static str> {
    let wanted = id & 0xFFFF;
    if l.head != NIL {
        let mut cur = l.head;
        loop {
            let found = slot_ref(l, cur)?.as_ref().map_or(0, |o| r32(o, 4) as u32);
            if found == wanted {
                return Ok(cur);
            }
            let next = *l.link12.get(usize::from(cur)).ok_or("enlace fuera de rango")?;
            if next == NIL {
                break;
            }
            cur = next;
        }
    }
    let limit = l.limit;
    if u32::from(l.count) == limit {
        return Ok(NIL);
    }
    let owner = u32::from(w.ship_byte(wanted, 0));
    if w.player != owner && u32::from(l.count) * 4 >= limit.wrapping_mul(3) {
        return Ok(NIL);
    }
    let convoy = w.ship_u16(wanted, 8);
    if convoy < w.convoy_count
        && w.strict_convoy_flag != 0
        && w.conv_byte(u32::from(convoy), 0x14) & 2 != 0
        && u32::from(w.conv_u16(u32::from(convoy), 0x10)) != id
    {
        return Ok(NIL);
    }
    let from_top = if owner == w.player {
        false
    } else if w.hotseat == 0 {
        true
    } else if owner >= u32::from(w.merchant_count) {
        true
    } else {
        (w.merchant_word)(owner) != 0
    };
    let slot: u16;
    if from_top {
        let mut si = (limit as u16).wrapping_sub(1);
        loop {
            if slot_ref(l, si)?.is_none() {
                break;
            }
            if si == NIL {
                return Ok(NIL);
            }
            si = si.wrapping_sub(1);
            if slot_ref(l, si)?.is_none() {
                break;
            }
        }
        if si == NIL {
            return Ok(NIL);
        }
        slot = si;
    } else {
        let mut si: u16 = 0;
        if slot_ref(l, 0)?.is_some() {
            loop {
                if si == NIL {
                    break;
                }
                si = si.wrapping_add(1);
                if u32::from(si) == limit {
                    return Ok(NIL);
                }
                if slot_ref(l, si)?.is_none() {
                    break;
                }
            }
        }
        slot = si;
    }
    let s = usize::from(slot);
    if s >= l.ships.len() {
        return Err("ranura fuera de la lista");
    }
    let mut object = vec![0u8; LOCAL];
    let ws = &w.ships[(wanted as usize * SHIP).min(w.ships.len())..];
    let mut world = vec![0u8; SHIP];
    let n = ws.len().min(SHIP);
    world[..n].copy_from_slice(&ws[..n]);
    construct_local_ship(&mut object, &world, id, l.address, slot, 0)?;
    init_motion(
        &mut object[4..],
        x.wrapping_shl(16),
        x.wrapping_shl(16),
        world[1].wrapping_shl(4),
    );
    l.ships[s] = Some(object);
    let (a8, a12) = (l.link8[s], l.link12[s]);
    if a8 == NIL {
        l.free = a12;
    } else {
        *l.link12.get_mut(usize::from(a8)).ok_or("enlace fuera de rango")? = a12;
    }
    if a12 != NIL {
        *l.link8.get_mut(usize::from(a12)).ok_or("enlace fuera de rango")? = a8;
    }
    l.link8[s] = NIL;
    l.link12[s] = l.head;
    if l.head != NIL {
        *l.link8.get_mut(usize::from(l.head)).ok_or("enlace fuera de rango")? = slot;
    }
    l.count = l.count.wrapping_add(1);
    l.head = slot;
    Ok(slot)
}

/// `0x613920`: recorre la cadena de barcos atracados desde `first` (`+6` de cada registro) y los añade a la escena de su ciudad.
pub fn add_berthed_ships(l: &mut ShipList, w: &HarborWorld, first: u16) -> Result<(), &'static str> {
    let mut id = u32::from(first);
    let conv = w.ship_u16(id, 8);
    if conv < w.convoy_count {
        id = u32::from(w.conv_u16(u32::from(conv), 0xA));
    }
    if u16::from(w.ship_byte(id, 0x39)) != l.town {
        return Ok(());
    }
    loop {
        if u16::from(w.ship_byte(id, 0x39)) != l.town {
            return Ok(());
        }
        let mut found = None;
        if l.head != NIL {
            let mut cur = l.head;
            loop {
                let world = slot_ref(l, cur)?.as_ref().map_or(0, |o| r32(o, 4) as u32);
                if world == id {
                    found = Some(cur);
                    break;
                }
                let next = *l.link12.get(usize::from(cur)).ok_or("enlace fuera de rango")?;
                if next == NIL {
                    break;
                }
                cur = next;
            }
        }
        let slot = match found {
            Some(s) => s,
            None => {
                let s = add_ship(l, w, id, 0)?;
                if s == NIL {
                    return Ok(());
                }
                let index = (usize::from(l.town) << 8) - usize::from(l.town) + usize::from(s);
                *l.town_ships.get_mut(index).ok_or("tabla de ciudad fuera de rango")? = id as u16;
                s
            }
        };
        let s = usize::from(slot);
        // Una ranura sin objeto (cadena de enlaces dañada) escribe y lee la página cero del original: sin efecto, índice 0.
        if let Some(o) = l.ships.get_mut(s).and_then(Option::as_mut) {
            o[0x12A] = 0;
        }
        let world = l.ships.get(s).and_then(Option::as_ref).map_or(0, |o| r32(o, 4) as u32);
        let height = {
            let c = w.ship_u16(world, 8);
            if c < w.convoy_count {
                w.conv_u16(u32::from(c), 0x16) << 5
            } else {
                w.ship_u16(world, 0x138) << 5
            }
        };
        if s >= l.heights.len() {
            l.heights.resize(s + 1, 0);
        }
        l.heights[s] = height;
        if w.ship_u16(id, 8) >= w.convoy_count {
            return Ok(());
        }
        let next = w.ship_u16(id, 6);
        if next >= w.ship_count {
            return Ok(());
        }
        id = u32::from(next);
    }
}

#[allow(dead_code)]
fn _unused(b: &[u8]) -> u16 {
    r16(b, 0)
}

fn mulhi(a: i32, b: i32) -> i32 {
    ((i64::from(a) * i64::from(b)) >> 32) as i32
}

/// Altura de una ranura al incorporarla (`+0x68`): la del convoy (`+0x16`) o la del barco (`+0x138`), por 32.
fn slot_height(l: &mut ShipList, w: &HarborWorld, slot: usize) {
    let world = l
        .ships
        .get(slot)
        .and_then(Option::as_ref)
        .map_or(0, |o| r32(o, 4) as u32);
    let c = w.ship_u16(world, 8);
    let h = if c < w.convoy_count {
        w.conv_u16(u32::from(c), 0x16).wrapping_shl(5)
    } else {
        w.ship_u16(world, 0x138).wrapping_shl(5)
    };
    if slot >= l.heights.len() {
        l.heights.resize(slot + 1, 0);
    }
    l.heights[slot] = h;
}

/// Copia (invertida) la ruta de la ranura a las tablas de entrada/salida. `double`: la entrada extrapola el punto cero (`2*p1 - p2`), la salida repite `p2`.
fn fill_route(l: &mut ShipList, routes: &SceneRoutes, slot: usize, double: bool) -> Result<(), &'static str> {
    let pts = routes.routes.get(slot).ok_or("ruta de la ranura ausente")?;
    let count = pts.len() as u32;
    let base = (slot as u32) * 8;
    let need = (base + count + 16) as usize;
    if l.entry_x.len() < need {
        l.entry_x.resize(need, 0);
        l.entry_y.resize(need, 0);
        l.entry_flags.resize(need, 0);
    }
    for (i, p) in pts.iter().enumerate() {
        let idx = (count - i as u32 + base) as usize;
        l.entry_x[idx] = p.0;
        l.entry_y[idx] = p.1;
        l.entry_flags[idx] = 0;
    }
    l.entry_flags[(count + base) as usize] = 1;
    l.entry_flags[base as usize + 1] = 2;
    l.entry_flags[base as usize] = 0;
    let b = base as usize;
    if double {
        l.entry_x[b] = l.entry_x[b + 1].wrapping_mul(2).wrapping_sub(l.entry_x[b + 2]);
        l.entry_y[b] = l.entry_y[b + 1].wrapping_mul(2).wrapping_sub(l.entry_y[b + 2]);
    } else {
        l.entry_x[b] = l.entry_x[b + 2];
        l.entry_y[b] = l.entry_y[b + 2];
    }
    if l.entry_len.len() <= slot {
        l.entry_len.resize(slot + 1, 0);
    }
    let len = (count as u8).wrapping_add(1);
    l.entry_len[slot] = len;
    if len != 0 {
        let mut counter: u8 = 0;
        loop {
            let at = usize::from(counter) + b;
            let v = i32::from(l.entry_y[at] as i16);
            let mut q = mulhi(34i32.wrapping_mul(v), 0x2E8B_A2E9) >> 2;
            q = q.wrapping_add(((q as u32) >> 31) as i32);
            l.entry_y[at] = q as u16;
            counter = counter.wrapping_add(1);
            if counter >= len {
                break;
            }
        }
    }
    Ok(())
}

/// Busca la ranura que ya tiene el barco `id` en la cadena de la lista.
fn find_slot(l: &ShipList, id: u32) -> Result<Option<u16>, &'static str> {
    if l.head == NIL {
        return Ok(None);
    }
    let mut cur = l.head;
    loop {
        let world = slot_ref(l, cur)?.as_ref().map_or(0, |o| r32(o, 4) as u32);
        if world == id {
            return Ok(Some(cur));
        }
        let next = *l.link12.get(usize::from(cur)).ok_or("enlace fuera de rango")?;
        if next == NIL {
            return Ok(None);
        }
        cur = next;
    }
}

/// `0x6132C0`: incorpora a la escena los barcos que llegan (cadena `+6`) con su ruta de entrada.
pub fn add_entering_ships(
    l: &mut ShipList,
    w: &HarborWorld,
    routes: &SceneRoutes,
    first: u32,
) -> Result<(), &'static str> {
    let mut id = first as u16;
    let c = w.ship_u16(u32::from(id), 8);
    if c < w.convoy_count {
        id = w.conv_u16(u32::from(c), 0xA);
    }
    if u16::from(w.ship_byte(u32::from(id), 0x39)) != l.town {
        return Ok(());
    }
    loop {
        let wanted = u32::from(id);
        let slot = match find_slot(l, wanted)? {
            Some(s) => Some(s),
            None => {
                if u32::from(l.count) >= l.limit {
                    return Ok(());
                }
                let s = add_ship(l, w, wanted, 0)?;
                if s == NIL {
                    None
                } else {
                    let index = (usize::from(l.town) << 8) - usize::from(l.town) + usize::from(s);
                    *l.town_ships.get_mut(index).ok_or("tabla de ciudad fuera de rango")? = id;
                    Some(s)
                }
            }
        };
        if let Some(s) = slot {
            let s = usize::from(s);
            fill_route(l, routes, s, true)?;
            if let Some(o) = l.ships.get_mut(s).and_then(Option::as_mut) {
                o[0x12A] = 1;
            }
            slot_height(l, w, s);
        }
        if w.ship_u16(wanted, 8) >= w.convoy_count {
            return Ok(());
        }
        let next = w.ship_u16(wanted, 6);
        if next >= w.ship_count {
            return Ok(());
        }
        id = next;
    }
}

/// `0x6135C0`: igual para los que zarpan. `allow_add` (segundo argumento) permite crear la ranura si el barco no está.
pub fn add_leaving_ships(
    l: &mut ShipList,
    w: &HarborWorld,
    routes: &SceneRoutes,
    host: &mut dyn SoundHost,
    first: u32,
    allow_add: u8,
) -> Result<(), &'static str> {
    let mut id = first as u16;
    let c = w.ship_u16(u32::from(id), 8);
    if c < w.convoy_count {
        id = w.conv_u16(u32::from(c), 0xA);
    }
    if u16::from(w.ship_byte(u32::from(id), 0x39)) != l.town {
        return Ok(());
    }
    loop {
        let wanted = u32::from(id);
        let mut slot = find_slot(l, wanted)?;
        let mut known = slot.is_some();
        if slot.is_none() {
            if allow_add == 0 {
                known = false;
            } else {
                if u32::from(l.count) >= l.limit {
                    return Ok(());
                }
                let s = add_ship(l, w, wanted, 0)?;
                if s != NIL {
                    let index = (usize::from(l.town) << 8) - usize::from(l.town) + usize::from(s);
                    *l.town_ships.get_mut(index).ok_or("tabla de ciudad fuera de rango")? = id;
                    slot = Some(s);
                }
            }
        }
        if let Some(s) = slot {
            let su = usize::from(s);
            if known {
                let o = slot_ref(l, s)?.as_ref();
                let flag = o.map_or(0, |o| o[0x12A]);
                if flag != 8 && flag != 0x18 && flag != 1 && routes.shown_town == u32::from(l.town) {
                    // Una ranura sin objeto lee la página cero del original: posición (0, 0).
                    let x = o.map_or(0, |o| i32::from(r16(o, 0xA) as i16));
                    let y = o.map_or(0, |o| i32::from(r16(o, 0xE) as i16));
                    let mut q = mulhi(22i32.wrapping_mul(y), 0x7878_7879) >> 4;
                    q = q.wrapping_add(((q as u32) >> 31) as i32);
                    host.ship_sound(x, q, 0);
                }
            }
            fill_route(l, routes, su, false)?;
            if let Some(o) = l.ships.get_mut(su).and_then(Option::as_mut) {
                o[0x12A] = 8;
            }
            slot_height(l, w, su);
        }
        if w.ship_u16(wanted, 8) >= w.convoy_count {
            return Ok(());
        }
        let next = w.ship_u16(wanted, 6);
        if next >= w.ship_count {
            return Ok(());
        }
        id = next;
    }
}

/// Efectos de `0x619330`/`0x613DA0` que quedan fuera de la lista: la escena libera la entidad de vista y se limpia un indicador global.
pub trait ListHost: SoundHost {
    /// `escena+0x3F8` (vtable +4): libera la entidad de vista (`barco+0x134`) del barco que se destruye.
    fn release_view_entity(&mut self, entity: u32);
    /// `DAT_006E59CD = 0` (el destructor lo hace si `barco+0x160 != 0`).
    fn clear_ship_flag(&mut self);
    /// `0x5708F0` (`harbor_lane_ship_type1f_refresh_progress_in_list`) con `this` = `lista+0x60`.
    fn refresh_lane_progress(&mut self, lane: u32);
}

/// `0x619330` (`local_map_ship_list_remove_ship`): destruye el barco local de la ranura (`arg & 0xFF`) y la devuelve a la lista libre.
/// `false` si la ranura estaba vacía.
pub fn remove_ship(l: &mut ShipList, host: &mut dyn ListHost, arg: u32) -> Result<bool, &'static str> {
    let slot = usize::from(arg as u8);
    let Some(object) = l.ships.get(slot).and_then(Option::as_ref) else {
        return Ok(false);
    };
    let view = r32(object, 0x134) as u32;
    if view != 0 {
        host.release_view_entity(view);
    }
    if object[0x160] != 0 {
        host.clear_ship_flag();
    }
    l.ships[slot] = None;
    let bx = slot as u16;
    let at = |v: &Vec<u16>, i: usize| v.get(i).copied().ok_or("enlace fuera de rango");
    let put = |v: &mut Vec<u16>, i: usize, x: u16| -> Result<(), &'static str> {
        *v.get_mut(i).ok_or("enlace fuera de rango")? = x;
        Ok(())
    };
    if l.head == bx {
        l.head = at(&l.link12, slot)?;
    } else {
        let prev = usize::from(at(&l.link8, slot)?);
        let next = at(&l.link12, slot)?;
        put(&mut l.link12, prev, next)?;
    }
    let next = at(&l.link12, slot)?;
    if next != NIL {
        let prev = at(&l.link8, slot)?;
        put(&mut l.link8, usize::from(next), prev)?;
    }
    if l.free != NIL {
        put(&mut l.link8, usize::from(l.free), bx)?;
    }
    put(&mut l.link8, slot, NIL)?;
    put(&mut l.link12, slot, l.free)?;
    l.count = l.count.wrapping_sub(1);
    l.free = bx;
    Ok(true)
}

/// Reloj, ajuste y divisor que lee `0x613DA0` (`DAT_006DCCF8`, `DAT_006DFC1C`, `DAT_006DFBC4`).
pub struct SyncClock {
    pub now: u32,
    pub smoothing_token: u32,
    pub divisor: u32,
}

impl HarborWorld<'_> {
    /// Estado del barco: el del convoy (`+0x12`) si pertenece a uno, si no `+0x134`.
    fn local_state(&self, world: u32) -> u16 {
        let c = self.ship_u16(world, 8);
        if c < self.convoy_count {
            self.conv_u16(u32::from(c), 0x12)
        } else {
            self.ship_u16(world, 0x134)
        }
    }
}

/// `0x613DA0` (`harbor_scene_sync_ships_with_game_state`): suaviza las alturas y despacha cada barco local según su estado.
pub fn sync_ships_with_game_state(
    l: &mut ShipList,
    w: &HarborWorld,
    routes: &SceneRoutes,
    host: &mut dyn ListHost,
    clock: &SyncClock,
) -> Result<(), &'static str> {
    let delta = clock.now.wrapping_sub(l.last_sync);
    l.last_sync = clock.now;
    if l.town >= 0x80 {
        return Ok(());
    }
    let mut flagged = false;
    if l.head != NIL {
        let scaled = delta.wrapping_shl(5);
        let mut cur = l.head;
        loop {
            let s = usize::from(cur);
            let next = *l.link12.get(s).ok_or("enlace fuera de rango")?;
            let (world, h13a) = slot_ref(l, cur)?
                .as_ref()
                .map_or((0, 0), |o| (r32(o, 4) as u32, r16(o, 0x13A)));
            let smoothed = if clock.smoothing_token == 0 {
                if clock.divisor == 0 {
                    return Err("divisor cero");
                }
                ((scaled / clock.divisor) as u16).wrapping_add(h13a)
            } else {
                let hi = ((u64::from(scaled) * 0x36B0_6E71) >> 32) as u32;
                let q = (scaled.wrapping_sub(hi) >> 1).wrapping_add(hi) >> 11;
                h13a.wrapping_add(q as u16)
            };
            if s >= l.heights.len() {
                l.heights.resize(s + 1, 0);
            }
            l.heights[s] = smoothed;
            let state = w.local_state(world);
            let c = w.ship_u16(world, 8);
            let target = if c >= w.convoy_count || state == 9 {
                w.ship_u16(world, 0x138)
            } else {
                w.conv_u16(u32::from(c), 0x16)
            };
            let target = u32::from(target).wrapping_shl(5);
            let cur_h = i32::from(smoothed as i16);
            let new_h = if (cur_h.wrapping_sub(target as i32)).wrapping_abs() > 0x20 {
                target as u16
            } else {
                ((cur_h.wrapping_mul(8).wrapping_sub(cur_h) as u32).wrapping_add(target) >> 3) as u16
            };
            if let Some(o) = l.ships.get_mut(s).and_then(Option::as_mut) {
                o[0x13A..0x13C].copy_from_slice(&new_h.to_le_bytes());
            }
            let first = u32::from(r16_obj(l, s, 4));
            const DISPATCH: [u8; 14] = [0, 0, 1, 2, 0, 0, 0, 0, 3, 3, 0, 1, 0, 0];
            if state > 0xD {
                let index = (usize::from(l.town) << 8) - usize::from(l.town) + s;
                *l.town_ships.get_mut(index).ok_or("tabla de ciudad fuera de rango")? = NIL;
                remove_ship(l, host, u32::from(cur))?;
            } else {
                match DISPATCH[usize::from(state)] {
                    0 => set_flag(l, s, 0),
                    1 => {
                        add_leaving_ships(l, w, routes, host, first, 0)?;
                        set_flag(l, s, 8);
                    }
                    2 => add_entering_ships(l, w, routes, first)?,
                    _ => {
                        add_leaving_ships(l, w, routes, host, first, 0)?;
                        flagged = true;
                    }
                }
            }
            if next == NIL {
                break;
            }
            cur = next;
        }
    }
    if flagged && l.head != NIL {
        let mut cur = l.head;
        loop {
            let s = usize::from(cur);
            let next = *l.link12.get(s).ok_or("enlace fuera de rango")?;
            let world = u32::from(r16_obj(l, s, 4));
            let state = w.local_state(world);
            if state == 8 || state == 9 {
                set_flag(l, s, 0x18);
            }
            if next == NIL {
                break;
            }
            cur = next;
        }
    }
    if l.lane != 0 {
        host.refresh_lane_progress(l.lane);
    }
    Ok(())
}

fn r16_obj(l: &ShipList, s: usize, at: usize) -> u16 {
    l.ships.get(s).and_then(Option::as_ref).map_or(0, |o| r16(o, at))
}

fn set_flag(l: &mut ShipList, s: usize, v: u8) {
    if let Some(o) = l.ships.get_mut(s).and_then(Option::as_mut) {
        o[0x12A] = v;
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------
// J05 paso 3: posición y avance (`0x613B20`, `0x614080`)

const SIN: [i32; 65] = [
    0, 1608, 3216, 4821, 6424, 8022, 9616, 11204, 12785, 14359, 15924, 17479, 19024, 20557, 22078, 23586, 25080, 26558,
    28020, 29466, 30893, 32303, 33692, 35062, 36410, 37736, 39040, 40320, 41576, 42806, 44011, 45190, 46341, 47464,
    48559, 49624, 50660, 51665, 52639, 53581, 54491, 55368, 56212, 57022, 57798, 58538, 59244, 59914, 60547, 61145,
    61705, 62228, 62714, 63162, 63572, 63944, 64277, 64571, 64827, 65043, 65220, 65358, 65457, 65516, 65536,
];
const ATAN: [u8; 65] = [
    0, 1, 1, 2, 3, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 9, 10, 11, 11, 12, 12, 13, 13, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19,
    19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 25, 26, 26, 27, 27, 27, 28, 28, 29, 29, 29, 30, 30, 30, 31, 31,
    31, 32, 32,
];

/// Seno del ángulo de 256 pasos con la tabla de 65 entradas del juego (`0x67ADD8`, `0x67AFD8`, `0x67B1D8`); `0x40` = 65536.
pub fn sin_t(u: u8) -> i32 {
    let u = i32::from(u);
    if u < 0x80 {
        SIN[if u < 0x41 { u } else { 0x80 - u } as usize]
    } else {
        let v = u - 0x80;
        -SIN[if v < 0x41 { v } else { 0x100 - u } as usize]
    }
}

/// Rumbo (0..255) del vector `(dx, dy)` con la tabla de arcotangente `0x67AEE0`: el bloque que el compilador repite en línea por todo el código del puerto.
pub fn heading(mut dx: i32, mut dy: i32) -> u8 {
    while dx > 0x7FFF || dx < -0x7FFF {
        dx >>= 1;
        dy >>= 1;
    }
    if dy == 0 {
        return if dx < 1 { 0xC0 } else { 0x40 };
    }
    let q = ((dx as i64) << 16) as i32 / dy;
    let c: u8 = if q < 0 {
        if q == -0x10000 || -q < 0x10000 {
            ATAN[((0x200 - q) >> 10) as usize].wrapping_neg()
        } else {
            0x40u8
                .wrapping_sub(ATAN[((0x80 - 0x4000_0000 / q) >> 8) as usize])
                .wrapping_neg()
        }
    } else if q < 0x10001 {
        ATAN[((q + 0x200) >> 10) as usize]
    } else {
        0x40u8.wrapping_sub(ATAN[((0x4000_0000 / q + 0x80) >> 8) as usize])
    };
    if dy < 1 {
        c.wrapping_neg()
    } else {
        0x80u8.wrapping_sub(c)
    }
}

/// Argumentos con los que `0x613B20` llama a `local_map_route_place_ship_at_distance` (`0x617380`); las posiciones son desplazamientos dentro de la lista.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalkArgs {
    pub slot: u8,
    pub xs: u32,
    pub ys: u32,
    pub flags: u32,
    pub len: u8,
    pub distance: u16,
    pub reverse: u8,
}

pub trait UpdateHost: ListHost {
    /// `0x617380`: recorre la ruta de la ranura hasta `distance` y escribe la posición del barco.
    fn place_on_route(&mut self, args: WalkArgs);
    /// `0x62018B` (`local_ship_update_view_entities`) con el barco local de la ranura.
    fn update_view(&mut self, slot: u16);
}

/// `0x613B20` (`harbor_scene_update_ship_position`).
pub fn update_ship_position(
    l: &mut ShipList,
    routes: &SceneRoutes,
    host: &mut dyn UpdateHost,
    arg: u32,
) -> Result<(), &'static str> {
    let slot = usize::from(arg as u8);
    if slot as u32 >= l.limit {
        set_flag(l, slot, 0x11);
    }
    let flag = l.ships.get(slot).and_then(Option::as_ref).ok_or("ranura vacía")?[0x12A];
    if flag > 0x18 {
        return Ok(());
    }
    const KIND: [u8; 25] = [
        0, 1, 4, 4, 4, 4, 4, 4, 2, 1, 4, 4, 4, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 3,
    ];
    let h = l.heights.get(slot).copied().unwrap_or(0);
    let (distance, reverse) = match KIND[usize::from(flag)] {
        0 => {
            let pts = routes.routes.get(slot).ok_or("ruta ausente")?;
            if pts.len() < 2 {
                return Err("ruta de un punto");
            }
            let last = (pts.len() as u8).wrapping_sub(1) as usize;
            let ((x, y), (px, py)) = (*pts.get(last).ok_or("ruta corta")?, pts[last - 1]);
            let div22 = |v: i32| mulhi(v, 0x2E8B_A2E9) >> 2;
            let fix = |q: i32| q.wrapping_add(((q as u32) >> 31) as i32);
            let dy = fix(div22((i32::from(y) - i32::from(py)).wrapping_mul(34)));
            let head = heading(i32::from(x) - i32::from(px), dy);
            let yo = fix(div22(i32::from(y).wrapping_mul(34)));
            let o = l.ships[slot].as_mut().unwrap();
            o[8..10].copy_from_slice(&0u16.to_le_bytes());
            o[0xA..0xC].copy_from_slice(&x.to_le_bytes());
            o[0xC..0xE].copy_from_slice(&0u16.to_le_bytes());
            o[0xE..0x10].copy_from_slice(&(yo as u16).to_le_bytes());
            o[0x11] = head;
            o[0x26] = 0;
            return Ok(());
        }
        1 => (0x800u16.wrapping_sub(h), 0),
        2 => (h, 1),
        3 => (if (h as i16) > 0x400 { 0x400 } else { h }, 1),
        _ => return Ok(()),
    };
    let len = l.entry_len.get(slot).copied().unwrap_or(0);
    host.place_on_route(WalkArgs {
        slot: slot as u8,
        xs: 0x266 + slot as u32 * 16,
        ys: 0x1256 + slot as u32 * 16,
        flags: 0x2246 + slot as u32 * 8,
        len,
        distance,
        reverse,
    });
    Ok(())
}

/// `0x614080` (`harbor_scene_update_ships`): retira los barcos marcados 0x11 y avanza el resto con el reloj.
pub fn update_ships(
    l: &mut ShipList,
    routes: &SceneRoutes,
    host: &mut dyn UpdateHost,
    clock: &SyncClock,
    hotseat: u32,
) -> Result<(), &'static str> {
    if l.town >= 0x80 {
        return Ok(());
    }
    let mut delta = if hotseat != 0 {
        0
    } else {
        clock.now.wrapping_sub(l.last_sync)
    };
    let mut cur = l.head;
    while cur != NIL {
        let next = *l.link12.get(usize::from(cur)).ok_or("enlace fuera de rango")?;
        if l.ships
            .get(usize::from(cur))
            .and_then(Option::as_ref)
            .map_or(0, |o| o[0x12A])
            == 0x11
        {
            remove_ship(l, host, u32::from(cur))?;
            let index = usize::from(l.town) * 0xFF + usize::from(cur);
            *l.town_ships.get_mut(index).ok_or("tabla de ciudad fuera de rango")? = NIL;
        }
        cur = next;
    }
    cur = l.head;
    while cur != NIL {
        let s = usize::from(cur);
        let h13a = l.ships.get(s).and_then(Option::as_ref).map_or(0, |o| r16(o, 0x13A));
        let base = if clock.smoothing_token == 0 {
            if clock.divisor == 0 {
                return Err("divisor cero");
            }
            delta = delta.min(clock.divisor);
            (delta.wrapping_shl(5) / clock.divisor) as u16
        } else {
            delta = delta.min(0xD2F);
            ((delta * 0x20) / 0xD2F) as u16
        };
        if s >= l.heights.len() {
            l.heights.resize(s + 1, 0);
        }
        let mut hv = base.wrapping_add(h13a);
        if (hv as i16) < 0 {
            hv = 0;
        } else if (hv as i16) > 0x800 {
            hv = 0x800;
        }
        l.heights[s] = hv;
        update_ship_position(l, routes, host, u32::from(cur))?;
        host.update_view(cur);
        cur = *l.link12.get(s).ok_or("enlace fuera de rango")?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------------------
// J05 paso 3b: entrada en la ciudad (`0x6141D0`)

/// Cabezas de las listas de barcos y convoyes que recorre `0x6141D0` (`DAT_006DD888`, `DAT_006DD88C` y, de la ciudad, `registro+0x81E` y `+0x820`).
#[derive(Clone, Copy, Debug, Default)]
pub struct EnterTown {
    pub first_ship: u16,
    pub first_convoy: u16,
    pub town_ship_head: u16,
    pub town_convoy_head: u16,
}

/// `0x6141D0` (`harbor_scene_enter_town`): vacía la escena de la ciudad anterior, recalcula las longitudes de ruta, rehace la tabla de barcos de la ciudad
/// y recorre los barcos y convoyes presentes (atracados, que llegan o que zarpan). Termina sincronizando y avanzando. `keep_views` es el segundo argumento
/// (si es 0 se anula la entidad de vista de cada barco antes de destruirlo).
#[allow(clippy::too_many_arguments)]
pub fn enter_town(
    l: &mut ShipList,
    w: &HarborWorld,
    routes: &SceneRoutes,
    host: &mut dyn UpdateHost,
    clock: &SyncClock,
    hotseat: u32,
    e: &EnterTown,
    town: u16,
    keep_views: u8,
) -> Result<(), &'static str> {
    if l.town == town {
        return Ok(());
    }
    if l.town < 0x80 {
        let mut cur = l.head;
        while cur != NIL {
            let s = usize::from(cur);
            if keep_views == 0 {
                if let Some(o) = l.ships.get_mut(s).and_then(Option::as_mut) {
                    o[0x134..0x138].copy_from_slice(&0u32.to_le_bytes());
                }
            }
            let next = *l.link12.get(s).ok_or("enlace fuera de rango")?;
            if let Some(o) = l.ships.get(s).and_then(Option::as_ref) {
                let view = r32(o, 0x134) as u32;
                if view != 0 {
                    host.release_view_entity(view);
                }
                if o[0x160] != 0 {
                    host.clear_ship_flag();
                }
            }
            if let Some(slot) = l.ships.get_mut(s) {
                *slot = None;
            }
            cur = next;
        }
    }
    let limit = l.limit;
    if u32::from(l.capacity) < limit {
        l.capacity = limit as u16;
        let cap = usize::from(l.capacity);
        l.ships.resize(cap, None);
        l.link12.resize(cap, 0);
        l.link8.resize(cap, 0);
    }
    l.free = 0;
    l.head = NIL;
    l.count = 0;
    let cap = usize::from(l.capacity);
    l.ships.resize(cap, None);
    l.link8.resize(cap, 0);
    l.link12.resize(cap, 0);
    for i in 0..cap {
        l.ships[i] = None;
        l.link12[i] = (i as u16).wrapping_add(1);
        l.link8[i] = (i as u16).wrapping_sub(1);
    }
    if cap != 0 {
        l.link12[cap - 1] = NIL;
    }
    l.lane = 0;
    l.town = town;
    let slots = limit as usize;
    if l.speeds.len() < slots {
        l.speeds.resize(slots, 0);
    }
    for slot in 0..slots {
        l.speeds[slot] = 0;
        let pts = routes.routes.get(slot).ok_or("ruta de la ranura ausente")?;
        for k in 0..pts.len().saturating_sub(1) {
            let dx = i32::from(pts[k + 1].0) - i32::from(pts[k].0);
            let dy_raw = (i32::from(pts[k + 1].1) - i32::from(pts[k].1)).wrapping_mul(0x22);
            let dy = dy_raw / 0x16;
            let sum = f64::from(dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)));
            l.speeds[slot] = l.speeds[slot].wrapping_add(sum.sqrt() as i32);
        }
    }
    let base = usize::from(l.town) * 0xFF;
    if l.town < 0x80 {
        for i in 0..0xFFusize {
            let entry = *l.town_ships.get(base + i).ok_or("tabla de ciudad fuera de rango")?;
            let mut keep = false;
            if entry < w.ship_count && u16::from(w.ship_byte(u32::from(entry), 0x39)) == l.town {
                let conv = w.ship_u16(u32::from(entry), 8);
                let foreign = conv < w.convoy_count
                    && w.strict_convoy_flag != 0
                    && w.conv_byte(u32::from(conv), 0x14) & 2 != 0
                    && w.conv_u16(u32::from(conv), 0x10) != entry;
                if !foreign && (i as u32) < limit && find_slot(l, u32::from(entry))?.is_none() {
                    let mut object = vec![0u8; LOCAL];
                    let ws = &w.ships[(usize::from(entry) * SHIP).min(w.ships.len())..];
                    let mut world = vec![0u8; SHIP];
                    let n = ws.len().min(SHIP);
                    world[..n].copy_from_slice(&ws[..n]);
                    construct_local_ship(&mut object, &world, u32::from(entry), l.address, i as u16, 0)?;
                    l.ships[i] = Some(object);
                    let (a8, a12) = (l.link8[i], l.link12[i]);
                    if a8 == NIL {
                        l.free = a12;
                    } else {
                        *l.link12.get_mut(usize::from(a8)).ok_or("enlace fuera de rango")? = a12;
                    }
                    if a12 != NIL {
                        *l.link8.get_mut(usize::from(a12)).ok_or("enlace fuera de rango")? = a8;
                    }
                    l.link8[i] = NIL;
                    l.link12[i] = l.head;
                    if l.head != NIL {
                        *l.link8.get_mut(usize::from(l.head)).ok_or("enlace fuera de rango")? = i as u16;
                    }
                    l.count = l.count.wrapping_add(1);
                    l.head = i as u16;
                    keep = true;
                }
            }
            if !keep {
                l.town_ships[base + i] = NIL;
            }
        }
        let mut ship_it = e.first_ship;
        let mut conv_it = e.first_convoy;
        let (mut b4, mut b5) = (false, false);
        if ship_it < w.ship_count {
            'outer: loop {
                loop {
                    loop {
                        if limit <= u32::from(l.count) {
                            break 'outer;
                        }
                        let id = u32::from(ship_it);
                        if u16::from(w.ship_byte(id, 0x39)) == l.town {
                            let state = w.local_state(id);
                            match state {
                                0 | 1 | 4 | 5 | 6 | 7 | 0xC | 0xD => {
                                    let mut slot = find_slot(l, id)?;
                                    if slot.is_none() {
                                        let s = add_ship(l, w, id, 0)?;
                                        if s != NIL {
                                            slot = Some(s);
                                        }
                                    }
                                    if let Some(s) = slot {
                                        let index = (usize::from(l.town) << 8) - usize::from(l.town) + usize::from(s);
                                        *l.town_ships.get_mut(index).ok_or("tabla de ciudad fuera de rango")? = ship_it;
                                        set_flag(l, usize::from(s), 0);
                                    }
                                }
                                2 => add_leaving_ships(l, w, routes, host, id, 1)?,
                                3 => add_entering_ships(l, w, routes, id)?,
                                8 | 9 => {
                                    add_leaving_ships(l, w, routes, host, id, 1)?;
                                    if let Some(s) = find_slot(l, id)? {
                                        set_flag(l, usize::from(s), 0x18);
                                    }
                                }
                                _ => {}
                            }
                        }
                        ship_it = w.ship_u16(id, 6);
                        if ship_it >= w.ship_count {
                            break;
                        }
                    }
                    if !b4 {
                        b4 = true;
                        ship_it = e.town_ship_head;
                    }
                    if ship_it >= w.ship_count {
                        break;
                    }
                }
                if conv_it < w.convoy_count {
                    let c = u32::from(conv_it);
                    ship_it = w.conv_u16(c, 0xA);
                    conv_it = w.conv_u16(c, 8);
                    if conv_it >= w.convoy_count && !b5 {
                        b5 = true;
                        conv_it = e.town_convoy_head;
                    }
                }
                if ship_it >= w.ship_count {
                    break;
                }
            }
        }
    }
    l.field_7eec = 0;
    sync_ships_with_game_state(l, w, routes, host, clock)?;
    update_ships(l, routes, host, clock, hotseat)
}
