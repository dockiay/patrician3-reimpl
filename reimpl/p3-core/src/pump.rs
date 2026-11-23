//! Bombeo de tiempo real a ticks: `operations_pump_time_advance_and_autosave` (0x00546620).
//!
//! Convierte los milisegundos del reloj de fotogramas (0x006DCCF8) en una operación `0xC4`
//! («avanza hasta el tick N») y encola el autoguardado `0xC2`. Solo lo llama el anfitrión o la
//! partida local; los clientes de red reciben los `0xC4` del anfitrión.

/// Tabla de velocidad 0x0066DE7C (niveles 0–5).
pub const SPEED_TABLE: [u32; 6] = [0x78, 0x5A, 0x3C, 0x1E, 0x14, 0x384];
/// Divisor global del modo 2 (combate lento), 0x00673CF8.
pub const SLOW_COMBAT_DIVISOR: u32 = 0xD2F;
/// Operación de avance de tiempo.
pub const OP_ADVANCE_TIME: u32 = 0xC4;
/// Operación de guardado.
pub const OP_SAVE: u32 = 0xC2;
/// Avance de todas las batallas navales, despachado en la pasada siguiente.
pub const OP_ADVANCE_BATTLES: u32 = 0x94;
pub const OP_SET_TIME_MODE: u32 = 0xC8;
/// Etiqueta «AUTO» del autoguardado.
pub const AUTOSAVE_TAG: u32 = 0x4F54_5541;
/// Con este número de registros en el paquete de cabeza ya no se encola nada.
pub const QUEUE_FULL: u16 = 0x34;
/// En modo normal el tick deja de encolarse un registro antes.
pub const QUEUE_FULL_FOR_TICK: u16 = 0x33;

/// Milisegundos por tick de un nivel: `valor · 1000 >> 8` (468, 351, 234, 117, 78 y 3515).
pub const fn ms_per_tick(level: usize) -> u32 {
    (SPEED_TABLE[level] * 1000) >> 8
}

/// Cadencia de `queue_battles_time_advance` (0x00612DB0). El anfitrión debe
/// ejecutar primero el tick sin avance del gestor naval y encolar la operación
/// devuelta; no debe ejecutar el combate dentro del bombeo que la genera.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BattleTimePump {
    pub enabled: bool,
    pub base: u32,
}
impl BattleTimePump {
    pub fn schedule(
        &mut self,
        now: u32,
        mode: u32,
        hotseat: u32,
        ms_per_tick: u32,
        mode1_divisor: u32,
    ) -> Result<Option<Operation>, &'static str> {
        if !self.enabled {
            return Ok(None);
        }
        let raw = now.wrapping_sub(self.base) as i32;
        let elapsed = if mode != 1 && raw > 100 {
            100
        } else {
            raw.clamp(0, 100_000)
        } as u32;
        let mut ticks = (elapsed / 40) as u16;
        if mode == 1 || mode == 0 && hotseat == 0 {
            let divisor = if mode == 1 { mode1_divisor } else { ms_per_tick };
            if divisor == 0 {
                return Err("división por cero en reloj de batalla");
            }
            ticks = ((u32::from(ticks) * 3375) / divisor) as u16;
        }
        ticks = ticks.min(0x5460);
        if ticks != 0 {
            self.enabled = false;
        }
        self.base = now.wrapping_add(u32::from(ticks) * 40).wrapping_sub(elapsed);
        Ok((ticks != 0).then_some(Operation([OP_ADVANCE_BATTLES, u32::from(ticks), 0, 0, 0])))
    }
}

/// Comprobación posterior al despacho 0x94 (0x00612C90/0x00612D20).
/// Cada entrada presente indica si la batalla está en modo directo; los
/// huecos vacíos de la tabla se ignoran. La petición se encola para después.
pub fn battle_time_mode_request(
    mode: u32,
    network_client: bool,
    force: bool,
    battles: &[Option<bool>],
) -> Option<Operation> {
    if network_client && !force {
        return None;
    }
    let any_manual = battles.iter().flatten().any(|&direct| !direct);
    let requested = if mode == 2 && !any_manual {
        Some(0)
    } else if mode != 2 && any_manual {
        Some(2)
    } else {
        None
    };
    requested.map(|value| Operation([OP_SET_TIME_MODE, u32::MAX, u32::MAX, value, u32::MAX]))
}

/// Registro de operación de 20 bytes tal como se encola.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Operation(pub [u32; 5]);

/// Paquetes locales de `execute_operations`: cada pasada captura hasta 52
/// registros. Lo que se encola durante su ejecución espera a otra pasada.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OperationQueue {
    pending: std::collections::VecDeque<Operation>,
}
impl OperationQueue {
    pub fn enqueue(&mut self, op: Operation) {
        self.pending.push_back(op);
    }
    pub fn len(&self) -> usize {
        self.pending.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    /// Si el anfitrión falla, conserva la operación no ejecutada y las siguientes
    /// delante de las nuevas; los efectos ya ejecutados permanecen publicados.
    pub fn dispatch<E>(&mut self, mut execute: impl FnMut(Operation, &mut Self) -> Result<(), E>) -> Result<usize, E> {
        let mut packet = self
            .pending
            .drain(..self.pending.len().min(usize::from(QUEUE_FULL)))
            .collect::<std::collections::VecDeque<_>>();
        let mut done = 0;
        while let Some(op) = packet.pop_front() {
            if let Err(error) = execute(op, self) {
                packet.push_front(op);
                while let Some(left) = packet.pop_back() {
                    self.pending.push_front(left)
                }
                return Err(error);
            }
            done += 1;
        }
        Ok(done)
    }
}

/// Cola de operaciones y escena de batallas.
pub trait PumpSink {
    /// `OPS+0x56`: registros en el paquete de cabeza (se vuelve a leer en cada comprobación).
    fn queue_len(&self) -> u16;
    /// `operations_enqueue` (0x0054AA70).
    fn enqueue(&mut self, op: Operation);
    /// `queue_battles_time_advance` (0x00612DB0).
    fn battles_time_advance(&mut self);
}

/// Campos de tiempo del objeto de operaciones (`OPS`, 0x006DF2F0).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TimePump {
    /// `OPS+0`: indicadores; el bit 0x10 indica partida en marcha para el autoguardado.
    pub flags: u32,
    /// `+0x8D4`: milisegundos por tick (modo 0).
    pub ms_per_tick: u32,
    /// `+0x8D8`: divisor del modo 1.
    pub mode1_divisor: u32,
    /// `+0x914`: indicador de marcha.
    pub running: u32,
    /// `+0x928`: hotseat.
    pub hotseat: u32,
    /// `+0x92C`: modo de tiempo (0 normal, 1, 2 o más combate lento) y ficha de `advance_time`.
    pub mode: u32,
    /// `+0x938`: base del reloj de pared.
    pub base: u32,
    /// `+0x93C`: hay un `0xC4` encolado sin ejecutar.
    pub pending_tick: u32,
    /// `+0x940`: plazo del siguiente autoguardado.
    pub autosave_deadline: u32,
    /// `+0x944`: intervalo de autoguardado.
    pub autosave_interval: u32,
}

impl TimePump {
    fn autosave<K: PumpSink + ?Sized>(&mut self, wall_ms: u32, sink: &mut K) {
        sink.enqueue(Operation([OP_SAVE, 0xFFFF_FFFF, AUTOSAVE_TAG, 0, 0]));
        self.autosave_deadline = self.autosave_interval.wrapping_add(wall_ms);
    }

    /// Una llamada del bombeo con el reloj de pared `wall_ms` y el tick actual del mundo.
    ///
    /// Divisor cero: el original provoca una excepción de división; aquí entra en pánico.
    pub fn pump<K: PumpSink + ?Sized>(&mut self, wall_ms: u32, tick: u32, sink: &mut K) {
        if self.hotseat != 0 && sink.queue_len() < QUEUE_FULL {
            sink.battles_time_advance();
        }
        if self.running == 0 {
            return;
        }
        if self.hotseat == 1 {
            // Un día entero por vez, con ficha 1; sin batallas al final.
            if self.pending_tick == 0 && sink.queue_len() < QUEUE_FULL {
                self.pending_tick = 1;
                sink.enqueue(Operation([OP_ADVANCE_TIME, tick, tick.wrapping_add(0x100), 1, 0]));
            }
            if self.autosave_deadline < wall_ms
                && sink.queue_len() < QUEUE_FULL
                && self.flags & 0x10 != 0
                && self.mode != 2
            {
                self.autosave(wall_ms, sink);
            }
            return;
        }
        if self.pending_tick == 0 {
            let elapsed = wall_ms.saturating_sub(self.base);
            let (divisor, cap) = match self.mode {
                0 => (self.ms_per_tick, 8),
                1 => (self.mode1_divisor, 0x100),
                _ => (SLOW_COMBAT_DIVISOR, 1),
            };
            let ticks = elapsed / divisor;
            // El resto se calcula antes del tope: con tope, el tiempo sobrante se pierde.
            let remainder = elapsed - divisor.wrapping_mul(ticks);
            self.base = wall_ms.wrapping_sub(remainder);
            let ticks = ticks.min(cap);
            if ticks > 0 && sink.queue_len() < QUEUE_FULL_FOR_TICK {
                self.pending_tick = 1;
                sink.enqueue(Operation([OP_ADVANCE_TIME, tick, tick.wrapping_add(ticks), 0, 0]));
            }
            if self.autosave_deadline < wall_ms {
                if sink.queue_len() >= QUEUE_FULL {
                    return;
                }
                if self.flags & 0x10 != 0 && self.mode != 2 {
                    self.autosave(wall_ms, sink);
                }
            }
        }
        if sink.queue_len() < QUEUE_FULL {
            sink.battles_time_advance();
        }
    }
}

/// Campos de `OPS` que fija el constructor `0x00546450` (resto a cero): `+0x8D4` y `+0x8DC` con
/// `ms_per_tick(2)` (234, nivel normal), `+0x8D8 = 0x19`, `+0x940 = 0xFFFFFFFF` y `+0x91C = 1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeMode {
    pub pump: TimePump,
    /// `+0x8DC`: último intervalo normal (no más lento que el nivel 0) que pidió `0xC8`.
    pub normal_ms: u32,
    /// `+0x91C`: valor del quinto campo de `0xC8`.
    pub field_91c: u32,
}

impl Default for TimeMode {
    fn default() -> Self {
        let ms = ms_per_tick(2);
        TimeMode {
            pump: TimePump { ms_per_tick: ms, mode1_divisor: 0x19, autosave_deadline: u32::MAX, ..TimePump::default() },
            normal_ms: ms,
            field_91c: 1,
        }
    }
}

impl TimeMode {
    /// Opcode `0xC8` de `execute_operations` (caso 7 de la tabla `0x00547290`, `0x00546DCF..0x00546E7C`): en
    /// hotseat para el reloj (`+0x914 = 0`, `+0x92C = 0`); si no, lo pone en marcha y cada campo distinto de
    /// `0xFFFFFFFF` fija intervalo (`+0x8D4`, y `+0x8DC` si no supera el del nivel 0), divisor del modo 1, modo (si
    /// cambia, la base pasa a ser el reloj de fotogramas `0x006DCCF8`) y `+0x91C`.
    pub fn apply(&mut self, op: [u32; 5], frame_clock: u32) {
        let p = &mut self.pump;
        if p.hotseat != 0 {
            p.running = 0;
            p.mode = 0;
            return;
        }
        p.running = 1;
        if op[1] != u32::MAX {
            if ms_per_tick(0) >= op[1] {
                self.normal_ms = op[1];
            }
            p.ms_per_tick = op[1];
        }
        if op[2] != u32::MAX {
            p.mode1_divisor = op[2];
        }
        if op[3] != u32::MAX {
            if p.mode != op[3] {
                p.base = frame_clock;
            }
            p.mode = op[3];
        }
        if op[4] != u32::MAX {
            self.field_91c = op[4];
        }
    }
}

/// Operación que encola la ventana de opciones al elegir la velocidad `level` (`0x0045DF6D` y `0x00460590`):
/// `0xC8` con `ms_per_tick(level)` y el resto de campos sin cambio. La ventana guarda además el nivel en los
/// ajustes de partida (`[0x006CC3E8]+4`).
pub fn speed_op(level: usize) -> Operation {
    Operation([OP_SET_TIME_MODE, ms_per_tick(level), u32::MAX, u32::MAX, u32::MAX])
}

impl TimeMode {
    /// Campos de tiempo de `read_savegame_from_buffer` (`0x00549F40`, tramo `0x0054A04A..0x0054A0C5`) sobre el
    /// estado actual: `speed` son los tres dwords que siguen a los ajustes y `version` la palabra alta del primer
    /// dword del fichero. `+0x8D4` y `+0x8D8` se leen siempre (un divisor mayor que 4 pasa a 2); `+0x8DC` solo con
    /// versión `>= 0x5F` (antes vale `ms_per_tick(2)`).
    pub fn load(&mut self, version: u32, speed: [u32; 3]) {
        self.pump.ms_per_tick = speed[0];
        self.pump.mode1_divisor = if speed[1] > 4 { 2 } else { speed[1] };
        self.normal_ms = if version < 0x5F { ms_per_tick(2) } else { speed[2] };
    }

    /// Los tres dwords que escribe `write_savegame_to_buffer` (`0x00549CC0`, desde `0x00549E0C`) tras los ajustes.
    pub fn saved(&self) -> [u32; 3] {
        [self.pump.ms_per_tick, self.pump.mode1_divisor, self.normal_ms]
    }
}
