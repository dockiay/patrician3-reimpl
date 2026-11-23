//! Avistamientos de los barcos humanos: `update_ship_spotted_contacts` 0x0050D2B0 y
//! `update_spotting_for_human_merchants` 0x0050DC50.
//!
//! Cada barco humano guarda dos contactos en `ship+0x50/+0x52`. La pasada por barco revisa los dos
//! contactos anteriores (pierde los que salen del alcance `gestor+0xFC` o cambian de estado), recorre el
//! orden X de la flota (`+0xA` hacia atrás, `+0xC` hacia delante) dentro de la ventana `gestor+0xFE` y
//! elige los nuevos: piratas detenidos (`0x12`) primero, después los barcos que se acercan según la
//! dirección de los dos (tablas de 16 rumbos). Los fenómenos (`0x15`) solo encienden su marca
//! `gestor+0x24 + 12*w` (`+6 = 0xFF`). La interfaz recibe tres avisos: contacto perdido 0x00452F60,
//! pulso de la carta 0x0046F340 y sonido 0x00459790.
//!
//! Precondiciones del original (fuera de ellas lee memoria ajena): los rumbos `ship+1` de los barcos
//! recorridos son `< 16` y el líder de cada convoy con barcos de otro dueño es un índice válido.
//! Paridad con el código real en `p3-core/tests/c10_spotting.rs` (1.200 casos x86).

pub const SHIP_SIZE: usize = 0x180;
pub const CONVOY_SIZE: usize = 0x3C;
pub const NONE: u16 = 0xFFFF;
const MAX: i32 = 0x7FFF_FFFF;
const MIN: i32 = i32::MIN;
const DX: [i8; 16] = [-9, -10, -9, -7, -4, 0, 4, 7, 9, 10, 9, 7, 4, 0, -4, -7];
const DY: [i8; 16] = [4, 0, -4, -7, -9, -10, -9, -7, -4, 0, 4, 7, 9, 10, 9, 7];

/// Avisos de interfaz que emite el original (punteros `[0x6CBDD8]` y `[0x6CC940]`).
pub trait SpotHost {
    /// 0x00452F60: contacto `ship` perdido.
    fn lost(&mut self, ship: u16);
    /// 0x0046F340: pulso en la carta.
    fn pulse(&mut self, x: u16, y: u16);
    /// 0x00459790 `(2, barco, x, y, 0)`: sonido de avistamiento.
    fn sound(&mut self, ship: u16, x: i32, y: i32);
}

/// Campos del gestor de barcos que se leen o escriben.
pub struct SpotManager<'a> {
    /// Bloque del gestor desde `+0` (al menos `0x100` bytes): fenómenos `+0x24`, recuentos `+0xF4/+0xF6`,
    /// alcance `+0xFC` y ventana `+0xFE`.
    pub raw: &'a mut [u8],
}

fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn i16_at(b: &[u8], o: usize) -> i32 {
    i32::from(i16::from_le_bytes([b[o], b[o + 1]]))
}
fn set16(b: &mut [u8], o: usize, v: u16) {
    b[o..o + 2].copy_from_slice(&v.to_le_bytes());
}

fn ship(i: u16) -> usize {
    usize::from(i) * SHIP_SIZE
}

/// 0x0050D2B0. `ships` es la tabla completa (`gestor+4`), `count = gestor+0xF4`.
pub fn update_ship_spotted_contacts(mgr: &mut [u8], ships: &mut [u8], s: u16, sound: bool, host: &mut impl SpotHost) {
    let count = u16_at(mgr, 0xF4);
    let at = ship(s);
    let (sx, sy) = (i16_at(ships, at + 0x1E), i16_at(ships, at + 0x22));
    let mut d = [0i32; 2];
    for (k, slot) in [0x50usize, 0x52].into_iter().enumerate() {
        let c = u16_at(ships, at + slot);
        if c >= count {
            d[k] = MIN;
            continue;
        }
        let t = ship(c);
        let state = u16_at(ships, t + 0x134);
        if state == 0x12 && u16_at(ships, t + 0x138) > 0 {
            host.lost(c);
        } else if state == 0xF || state == 0x12 {
            let dx = i16_at(ships, t + 0x1E) - sx;
            let dy = i16_at(ships, t + 0x22) - sy;
            let linked = u16_at(ships, t + 0xA) < count || u16_at(ships, t + 0xC) < count;
            if linked && i32::from(u16_at(mgr, 0xFC)) >= dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)) {
                d[k] = MAX;
                continue;
            }
            host.lost(c);
        }
        set16(ships, at + slot, NONE);
        d[k] = MIN;
    }
    let (mut d0, mut d1) = (d[0], d[1]);
    if d0 < d1 {
        let v = u16_at(ships, at + 0x52);
        set16(ships, at + 0x50, v);
        set16(ships, at + 0x52, NONE);
        d0 = d1;
        d1 = MIN;
    } else if d1 == MAX {
        return;
    }
    let dir = usize::from(ships[at + 1]);
    let (ox, oy) = (i32::from(DX[dir]), i32::from(DY[dir]));
    let r = i32::from(u16_at(mgr, 0xFE));
    let (ylo, yhi) = (sy - r, sy + r);
    let owner = ships[at];
    for forward in [false, true] {
        let mut t = u16_at(ships, at + if forward { 0xC } else { 0xA });
        while t < count {
            let tt = ship(t);
            let (tx, ty) = (i16_at(ships, tt + 0x1E), i16_at(ships, tt + 0x22));
            if if forward { tx > sx + r } else { tx < sx - r } {
                break;
            }
            let next = u16_at(ships, tt + if forward { 0xC } else { 0xA });
            if ships[tt] == owner || ty < ylo || ty > yhi {
                t = next;
                continue;
            }
            let state = u16_at(ships, tt + 0x134);
            if state == 0x15 {
                let w = usize::from(ships[tt + 0x39]);
                if w < 16 {
                    set16(mgr, 0x2A + 12 * w, 0xFF);
                    host.pulse(u16_at(mgr, 0x24 + 12 * w), u16_at(mgr, 0x26 + 12 * w));
                    if sound {
                        host.sound(t, tx, ty);
                    }
                }
                t = next;
                continue;
            }
            if t == u16_at(ships, at + 0x52) || t == u16_at(ships, at + 0x50) {
                t = next;
                continue;
            }
            if state == 0x12 && (forward || u16_at(ships, tt + 0x138) == 0) {
                let hit = if d0 < MAX {
                    d1 = d0;
                    d0 = MAX;
                    let old = u16_at(ships, at + 0x50);
                    set16(ships, at + 0x50, t);
                    set16(ships, at + 0x52, old);
                    true
                } else if d1 < MAX {
                    set16(ships, at + 0x52, t);
                    d1 = MAX;
                    true
                } else {
                    false
                };
                if hit {
                    host.pulse(tx as u16, ty as u16);
                    if sound {
                        host.sound(t, tx, ty);
                    }
                }
            } else if state == 0xF || (forward && state != 0x12) {
                let (ex, ey) = (tx - sx, ty - sy);
                let base = ex.wrapping_mul(ex).wrapping_add(ey.wrapping_mul(ey));
                let tdir = usize::from(ships[tt + 1]);
                let fx = ex.wrapping_add(ox - i32::from(DX[tdir]));
                let fy = ey.wrapping_add(oy - i32::from(DY[tdir]));
                let score = fy.wrapping_mul(fy).wrapping_add(fx.wrapping_mul(fx)).wrapping_sub(base);
                if score > d1 {
                    set16(ships, at + 0x52, t);
                    if score > d0 {
                        d1 = d0;
                        d0 = score;
                    } else {
                        d1 = score;
                    }
                }
            }
            t = next;
        }
    }
    if ships[at + 0x15C] != 0 {
        for slot in [0x50usize, 0x52] {
            let c = u16_at(ships, at + slot);
            if c < count {
                let t = ship(c);
                host.pulse(u16_at(ships, t + 0x1E), u16_at(ships, t + 0x22));
                if sound {
                    host.sound(c, i16_at(ships, t + 0x1E), i16_at(ships, t + 0x22));
                }
            }
        }
    }
    for slot in [0x50usize, 0x52] {
        let c = u16_at(ships, at + slot);
        if c < count && u16_at(ships, ship(c) + 0x134) == 0x15 {
            set16(ships, at + slot, NONE);
        }
    }
}

/// Comerciante visto por la pasada de humanos.
pub struct SpotMerchant {
    /// `+8`: 0 para humano.
    pub kind: u16,
    /// `+0x3B`: sonido de avistamientos activado.
    pub sound: bool,
    /// `+0xE`: primer barco de la flota (cadena `ship+4`).
    pub first_ship: u16,
}

/// Globales de interfaz y partida que lee 0x0050DC50.
pub struct SpotGlobals {
    /// `[0x6CBDD8]` y `[0x6CC940]` no nulos (carta y ventana presentes).
    pub ui: bool,
    /// `byte [0x6DF2F0] & 2`: todos los humanos (partida por turnos), no solo el jugador.
    pub all_humans: bool,
    /// `[0x6DFC14]`.
    pub player: u32,
}

/// 0x0050DC50. `merchants` es la tabla de `get_merchant` (`[0x6DE4AA]` entradas).
pub fn update_spotting_for_human_merchants(mgr: &mut [u8], ships: &mut [u8], convoys: &[u8], merchants: &[SpotMerchant], g: &SpotGlobals, host: &mut impl SpotHost) {
    if !g.ui {
        return;
    }
    let n = merchants.len() as u32;
    let range = if g.all_humans {
        0..n
    } else if g.player < n {
        g.player..g.player + 1
    } else {
        return;
    };
    for m in range {
        let merchant = &merchants[m as usize];
        if merchant.kind != 0 {
            continue;
        }
        let flag = m == g.player && merchant.sound;
        let mut s = merchant.first_ship;
        while s < u16_at(mgr, 0xF4) {
            let at = ship(s);
            let cv = u16_at(ships, at + 8);
            let state = |ships: &[u8], i: u16| u16_at(ships, usize::from(i) * SHIP_SIZE + 0x134);
            let mut clear = false;
            if cv >= u16_at(mgr, 0xF6) {
                if state(ships, s) == 0xF {
                    update_ship_spotted_contacts(mgr, ships, s, flag, host);
                } else {
                    clear = true;
                }
            } else {
                let c = usize::from(cv) * CONVOY_SIZE;
                let leader = u16_at(convoys, c + 0x10);
                if leader == s {
                    if state(ships, leader) == 0xF {
                        update_ship_spotted_contacts(mgr, ships, s, flag, host);
                    } else {
                        clear = true;
                    }
                } else if convoys[c] == ships[at] {
                    clear = true;
                } else {
                    update_ship_spotted_contacts(mgr, ships, leader, flag, host);
                    let la = ship(leader);
                    let slots: [u8; 4] = ships[la + 0x50..la + 0x54].try_into().unwrap();
                    ships[at + 0x50..at + 0x54].copy_from_slice(&slots);
                }
            }
            if clear {
                set16(ships, at + 0x50, NONE);
                set16(ships, at + 0x52, NONE);
            }
            s = u16_at(ships, at + 4);
        }
    }
}

/// Comprueba las precondiciones del original antes de [`update_spotting_for_human_merchants`]: rumbos `< 16`, líderes
/// de convoy válidos y tablas del tamaño que declara el gestor (fuera de ellas el original lee memoria ajena).
pub fn check_preconditions(mgr: &[u8], ships: &[u8], convoys: &[u8]) -> Result<(), String> {
    if mgr.len() < 0x100 {
        return Err("gestor de barcos truncado".into());
    }
    let (count, ncv) = (u16_at(mgr, 0xF4), u16_at(mgr, 0xF6));
    if ships.len() < usize::from(count) * SHIP_SIZE || convoys.len() < usize::from(ncv) * CONVOY_SIZE {
        return Err("tablas de barcos o convoyes más cortas que el gestor".into());
    }
    for i in 0..count {
        let at = ship(i);
        if ships[at + 1] >= 16 {
            return Err(format!("barco {i}: rumbo {} fuera de la tabla", ships[at + 1]));
        }
        let cv = u16_at(ships, at + 8);
        if cv < ncv && u16_at(convoys, usize::from(cv) * CONVOY_SIZE + 0x10) >= count {
            return Err(format!("convoy {cv}: líder fuera de la flota"));
        }
    }
    Ok(())
}

/// Pulsos del minimapa: lista de entradas de 12 bytes (`+0` x, `+2` y, `+4` nivel, `+8` clase) en
/// `minimapa+0xC4` (cuenta en `+0xC8`).
///
/// - Alta `0x0046F340(x, y)`: solo con el minimapa visible (`[this+0x48]`, `0x004B2200`) y la señal de
///   velocidad `[0x006DFC1C]` distinta de 1 (`[0x00673CA8]`). Convierte con `x / 9,23077 + 0,5` e
///   `y / 7,92952 + 0,5` (`0x00639C28` trunca) y no añade la entrada si ya hay una con las mismas
///   coordenadas. La nueva entrada lleva nivel `0xFF` y clase 1; se añade al final.
/// - Dibujo (`0x0046E004..0x0046E0AB`): por entrada, color gris `0xFF` + nivel en los tres canales y cinco
///   segmentos que forman un rombo de radio 2 (puntos en `(x, y±2)`, líneas `(x±1, y±1)` y `(x±2, y)`);
///   después resta 10 al nivel.
/// - Limpieza (`0x0046E644`): una entrada con nivel `< 0` se elimina con `0x0046F4C0` (siempre la primera:
///   las más viejas mueren antes).
///
/// **Supuesto**: el dibujo y la limpieza van seguidos en cada repintado del minimapa. Sin juego no se ha
/// podido comprobar el orden relativo; con él, una entrada se dibuja 26 veces (255, 245, …, 5).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MinimapPulses {
    /// `(x, y, nivel, clase)` en coordenadas del minimapa.
    pub entries: Vec<(i16, i16, i32, u8)>,
}

/// Escalas del minimapa (`0x0066EA90`, `0x0066EA98`) y redondeo (`0x0066A7D0`).
pub const MINIMAP_SCALE: (f64, f64) = (9.23077, 7.92952);

impl MinimapPulses {
    /// `0x0046F340`. `visible` es `[minimapa+0x48]`; `speed_token` es `[0x006DFC1C]`.
    pub fn add(&mut self, x: u16, y: u16, visible: bool, speed_token: u32) -> bool {
        if !visible || speed_token == 1 {
            return false;
        }
        let px = (f64::from(x) / MINIMAP_SCALE.0 + 0.5) as i32 as i16;
        let py = (f64::from(y) / MINIMAP_SCALE.1 + 0.5) as i32 as i16;
        if self.entries.iter().any(|e| e.0 == px && e.1 == py) {
            return false;
        }
        self.entries.push((px, py, 0xFF, 1));
        true
    }

    /// Un repintado: devuelve lo que se dibuja `(x, y, nivel)` y envejece/limpia las entradas.
    pub fn paint(&mut self) -> Vec<(i32, i32, i32)> {
        let drawn = self.entries.iter().map(|e| (i32::from(e.0), i32::from(e.1), e.2 & 0xFF)).collect();
        for e in &mut self.entries {
            e.2 -= 10;
        }
        self.entries.retain(|e| e.2 >= 0);
        drawn
    }
}

#[cfg(test)]
mod minimap_pulse_tests {
    use super::*;

    #[test]
    fn add_converts_coordinates_and_skips_duplicates() {
        let mut p = MinimapPulses::default();
        assert!(p.add(1200, 900, true, 0));
        assert_eq!(p.entries, vec![(130, 113, 0xFF, 1)]);
        assert!(!p.add(1203, 898, true, 0), "mismas coordenadas de minimapa");
        assert!(!p.add(10, 10, false, 0), "minimapa oculto");
        assert!(!p.add(10, 10, true, 1), "señal de velocidad 1");
        assert_eq!(p.entries.len(), 1);
    }

    #[test]
    fn pulse_is_drawn_26_times_then_removed() {
        let mut p = MinimapPulses::default();
        p.add(100, 100, true, 0);
        let mut levels = vec![];
        while !p.entries.is_empty() {
            levels.extend(p.paint().into_iter().map(|d| d.2));
        }
        assert_eq!(levels.len(), 26);
        assert_eq!((levels[0], levels[25]), (255, 5));
    }
}
