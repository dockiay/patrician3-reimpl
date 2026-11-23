//! Viento de la escena del puerto (`HarborScene`, objeto global `0x006E6220`): `0x00618D10` (`thiscall`, dos
//! punteros de salida: rumbo y fuerza). Lo consultan el puerto (`0x006130A2`), la máquina de asedio (`0x0056594A`),
//! el ariete (`0x00575160`) y el fotograma de la batalla (`0x00632478`) para orientar el humo.
//!
//! El valor objetivo depende del tick del mundo (`0x006DE4B4`), de la ciudad de la escena (`+0x5C`) y de la
//! orientación del mar de esa ciudad (`AusrichtungDesMeeres`, `TownData+0x31` en `0x006DDBC1 + 0x34 * id`, con `id`
//! de `0x006DE4B8[ciudad]`). Si han pasado menos de 16 cuartos de tick desde la última consulta el rumbo y la fuerza
//! se acercan al objetivo con un paso limitado; si no ha pasado ninguno se devuelve lo guardado.
//!
//! Estado en la escena: `+0x7EEC` (tick de la última consulta, múltiplo de 4), `+0x7EF0` (rumbo) y `+0x7EF1`
//! (fuerza). El constructor y los reinicios (`0x006131E8`, `0x0061325B`, `0x006132B1`, `0x00614865`) ponen
//! `+0x7EEC = 0`; rumbo y fuerza no se inician en esos puntos.

/// Estado del viento de la escena del puerto.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HarborWind {
    /// `+0x7EEC`.
    pub last: u32,
    /// `+0x7EF0`.
    pub dir: u8,
    /// `+0x7EF1`.
    pub force: u8,
}

impl HarborWind {
    /// `0x00618D10` con el tick `tick`, la ciudad de la escena `town` (`+0x5C`, palabra) y la orientación del mar
    /// `sea` de esa ciudad. Devuelve `(rumbo, fuerza)`.
    pub fn sample(&mut self, tick: u32, town: u16, sea: u8) -> (u8, u8) {
        let n = tick.wrapping_sub(self.last) >> 2;
        if n == 0 {
            return (self.dir, self.force);
        }
        let mut dir = 0xE3u8.wrapping_sub(((tick >> 8) as u8).wrapping_add(town as u8).wrapping_mul(0x33)) & 0x7F;
        if i32::from(dir) + 0x20 > i32::from(sea) << 5 {
            dir = dir.wrapping_add(0x40);
        }
        if i32::from(dir) > (i32::from(sea) + 3) << 5 {
            dir = dir.wrapping_add(0x40);
        }
        let mut force = (dir.wrapping_sub(1) & 3) + 4;
        if n < 0x10 {
            let step = (n * 8) as i16;
            let mut d = i16::from(dir) - i16::from(self.dir);
            if d > 0x80 {
                d -= 0x100;
            } else if d < -0x80 {
                d += 0x100;
            }
            if d > step {
                dir = self.dir.wrapping_add((n as u8) << 3);
            } else if -i32::from(d) > i32::from(step) {
                dir = self.dir.wrapping_sub((n as u8) << 3);
            }
            let f = i16::from(force) - i16::from(self.force);
            if f > n as i16 {
                force = self.force.wrapping_add(n as u8);
            } else if -i32::from(f) > n as i32 {
                force = self.force.wrapping_sub(n as u8);
            }
        }
        self.dir = dir;
        self.force = force;
        self.last = tick & !3;
        (dir, force)
    }
}
