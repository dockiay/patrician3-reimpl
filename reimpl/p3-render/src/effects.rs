//! Aparición de efectos del pueblo (huellas, humo, chispas...): `0x57A150` (`objeto.método(x, y, rumbo, enlace, parámetro)`).
//!
//! Observado y probado en emulador (`oracle/x86_effect_spawn.py`, 3.000 casos con el x86 real). La elección del juego de sprites entre los que comparten
//! identificador usa muestreo de depósito con `rand`; las llamadas a `0x557730`, `0x559CF0`, `0x57AF60`, `0x559790`, `0x559370`, `0x59B5B0`, `0x59A260` y
//! `0x5986A0` se sustituyen por [`EffectEnv`] y se registran.

#[derive(Clone, Debug, Default)]
pub struct Effect {
    pub x: u32,
    pub y: u32,
    pub px: i16,
    pub py: i16,
    pub heading: u8,
    pub frame: u8,
    /// `+0x16`: enlace.
    pub link: u16,
    /// `+0x24`: instante de fin.
    pub until: u32,
    /// `+0x2C`: juego elegido.
    pub set: u8,
    /// `+0x2E`
    pub extra: u16,
}

/// Una entrada de la tabla de juegos de efectos.
#[derive(Clone, Copy, Debug)]
pub struct EffectSet {
    /// Byte de identificador (`DAT_006E2E74[k]`).
    pub id: u8,
    /// `juego+4`: parámetro de selección.
    pub param: u32,
    /// `juego+0x10`: duración.
    pub duration: u32,
}

pub trait EffectEnv {
    fn rand(&mut self) -> u32;
    /// `0x557730(juego, 2)`.
    fn select(&mut self, set: usize) -> u8;
    /// `0x559CF0`
    fn begin(&mut self, set: usize);
    /// `0x57AF60(juego, activar)`
    fn toggle(&mut self, set: usize, on: bool);
    /// `0x559790` (si `param & 0x40 == 0`) o `0x559370` + `0x59B5B0` + `0x59A260`.
    fn occupy(&mut self, set: usize, dirty: bool);
    /// `0x5986A0(x, y)`
    fn mark(&mut self, x: i16, y: i16);
}

/// `0x57A150`. `flag` es `escena+0xC594`, `now` es `DAT_006DCCF8`; `sets` no puede estar vacía.
pub fn spawn(e: &mut Effect, sets: &[EffectSet], flag: bool, now: u32, x: u32, y: u32, heading: u8, link: u16, param: u32, env: &mut dyn EffectEnv) {
    let id = param as u8;
    let extra = match id {
        4..=7 | 0x40..=0x43 | 0x50 | 0x80 | 0xB0 | 199 => (param >> 8) & 0xFF,
        _ => 0,
    };
    e.set = 0;
    let n = sets.len();
    let mut count: u8 = 0;
    let mut k: u8 = 0;
    while usize::from(k) < n {
        let s = &sets[usize::from(k)];
        if s.id == id && (extra == 0 || extra == s.param) {
            count = count.wrapping_add(1);
            let r = env.rand() as i32;
            if r % i32::from(count) == 0 {
                e.set = k;
                if !flag {
                    k = (n - 1) as u8;
                }
            }
        }
        k = k.wrapping_add(1);
    }
    let set = usize::from(e.set);
    e.frame = env.select(set);
    e.until = sets[set].duration.wrapping_add(now);
    env.begin(set);
    e.link = link;
    e.x = x;
    e.y = y;
    e.heading = heading;
    e.y = match id {
        0x24 | 0x30 | 0x11 | 0xB0 => y.wrapping_add(0x30_0000),
        0x81 => y.wrapping_add(0xB_0000),
        _ => y,
    };
    e.extra = if sets[set].id & 0x20 != 0 { (param >> 16) as u16 } else { 0 };
    e.px = (e.x >> 16) as i16;
    e.py = (e.y >> 16) as i16;
    if flag || !(0x40..=0x43).contains(&id) {
        env.toggle(set, true);
        env.occupy(set, param & 0x40 != 0);
        env.toggle(set, false);
        if id == 0 || id == 2 {
            let (px, py) = (e.px, e.py);
            env.mark(px, py);
            env.mark(px.wrapping_sub(1), py);
            env.mark(px, py.wrapping_sub(1));
            env.mark(px.wrapping_sub(1), py.wrapping_sub(1));
        }
    }
}
