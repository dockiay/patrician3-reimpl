//! Generadores de números aleatorios del original (C02).
//!
//! Hay dos familias y ninguna más en el ejecutable:
//!
//! - `rand`/`srand` del CRT de MSVC (0x00639C03 / 0x00639BF6): un flujo por hilo, compartido por las llamadas de
//!   simulación, interfaz y audio que se ejecuten en ese mismo hilo. Se siembra con `timeGetTime()` al arrancar el programa y al
//!   empezar partida nueva; **el guardado no lo contiene** y cargar una partida no lo restaura.
//! - `LcgRandom` (objeto de 8 bytes, 0x0046C320..0x0046C380) y su copia en el objeto de batalla naval
//!   (`+0xADC`, 0x0060D259 y 24 copias en línea): paso `ROR1(0xA205B063 - s * 0x44BF19B3)`. Las
//!   siembras de simulación son deterministas (tick del mundo o campos del estado), y el estado de la
//!   batalla naval sí se guarda.

/// Semilla de `rand` antes de cualquier `srand` (bloque por hilo recién creado).
pub const CRT_DEFAULT_SEED: u32 = 1;
/// Estado que deja el constructor de `LcgRandom`.
pub const LCG_CTOR_SEED: u32 = 0x4996_02D2;

/// `rand`/`srand` del CRT (estado en `ptd+0x14`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CrtRand {
    pub state: u32,
}

impl Default for CrtRand {
    fn default() -> Self {
        CrtRand {
            state: CRT_DEFAULT_SEED,
        }
    }
}

impl CrtRand {
    /// `srand(seed)`: guarda la semilla tal cual.
    pub fn srand(&mut self, seed: u32) {
        self.state = seed;
    }

    /// `rand()`: `s = s * 0x343FD + 0x269EC3`; devuelve `(s >> 16) & 0x7FFF` (0..=32767).
    pub fn rand(&mut self) -> i32 {
        self.state = self.state.wrapping_mul(0x343FD).wrapping_add(0x269EC3);
        ((self.state >> 16) & 0x7FFF) as i32
    }
}

/// Un paso del generador de objeto: `ROR1(0xA205B063 - s * 0x44BF19B3)`.
pub const fn lcg_step(state: u32) -> u32 {
    0xA205_B063u32
        .wrapping_sub(state.wrapping_mul(0x44BF_19B3))
        .rotate_right(1)
}

/// `LcgRandom`: primera palabra = estado; la segunda la pone a cero el constructor y nadie la usa.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LcgRandom {
    pub state: u32,
    pub second: u32,
}

impl Default for LcgRandom {
    fn default() -> Self {
        Self::new()
    }
}

impl LcgRandom {
    /// `lcg_random_ctor` (0x0046C320).
    pub const fn new() -> Self {
        LcgRandom {
            state: LCG_CTOR_SEED,
            second: 0,
        }
    }

    /// Constructor seguido de la resiembra habitual (21 de las 22 construcciones).
    pub const fn seeded(seed: u32) -> Self {
        LcgRandom { state: seed, second: 0 }
    }

    /// `lcg_random_set_seed` (0x0046C340).
    pub fn set_seed(&mut self, seed: u32) {
        self.state = seed;
    }

    /// `lcg_random_next` (0x0046C350): avanza y devuelve el estado nuevo completo.
    pub fn next_u32(&mut self) -> u32 {
        self.state = lcg_step(self.state);
        self.state
    }

    /// `lcg_random_next_below(n)` (0x0046C380): estado nuevo `% n`, sin signo.
    ///
    /// Con `n == 0` el original produce una excepción de división; aquí entra en pánico **después** de
    /// avanzar el estado, como el original (que lo escribe antes del `div`).
    pub fn next_below(&mut self, n: u32) -> u32 {
        let state = self.next_u32();
        assert!(n != 0, "lcg_random_next_below(0): división por cero en el original");
        state % n
    }
}

/// `sea_battle_next_random` (0x0060D259) y sus 24 copias en línea: el mismo paso sobre `battle+0xADC`.
pub fn sea_battle_next(state: &mut u32) -> u32 {
    *state = lcg_step(*state);
    *state
}

/// Semilla de la batalla naval al empezar: `(u16)[+0xAD8] << 16 + (u16)[+0xAD4]`.
pub fn sea_battle_seed(word_ad8: u16, word_ad4: u16) -> u32 {
    ((word_ad8 as u32) << 16).wrapping_add(word_ad4 as u32)
}

/// Estado aleatorio de un hilo del original.
///
/// En modo de paridad el flujo `rand` es por hilo, no por partida: [`SimRandom::load_original`]
/// lo deja como está, igual que el juego al cargar un `.pat`. El formato propio puede guardarlo
/// ([`SimRandom::snapshot`] / [`SimRandom::restore`]) para que una partida recargada repita resultados;
/// eso es una mejora explícita, no paridad. Cada hilo original requiere su propio estado; este
/// snapshot solo representa uno de ellos.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SimRandom {
    pub crt: CrtRand,
}

/// Lo que el formato propio guarda del azar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RandomSnapshot {
    pub crt_state: u32,
}

impl SimRandom {
    /// Partida nueva: `srand(timeGetTime())` en `operations_host_start_new_game`; la hora la da el anfitrión.
    pub fn new_game(&mut self, wall_ms: u32) {
        self.crt.srand(wall_ms);
    }

    /// Carga de un guardado original: el CRT no se toca (no está en el archivo).
    pub fn load_original(&mut self) {}

    pub fn snapshot(&self) -> RandomSnapshot {
        RandomSnapshot {
            crt_state: self.crt.state,
        }
    }

    pub fn restore(&mut self, snapshot: RandomSnapshot) {
        self.crt.state = snapshot.crt_state;
    }
}
