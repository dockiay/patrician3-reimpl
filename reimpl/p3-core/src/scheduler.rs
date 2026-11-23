//! Planificador de tareas con fecha: objeto global 0x006DD73C.
//!
//! Reproduce `scheduled_tasks_add_task` (0x004D8CF0), `reschedule_first_task` (0x004D8DD0),
//! `scheduled_tasks_get_task` (0x004D8DB0), `scheduled_tasks_remove_by_opcode` (0x004D8E70),
//! `scheduled_tasks_free_all` (0x004D8520), `scheduled_tasks_init_new_game` (0x004D8B10) y el
//! despachador `handle_scheduled_tasks_tick` (0x004D85C0).
//!
//! Los registros forman una lista enlazada ordenada por vencimiento dentro de un array que crece de
//! 128 en 128. Un índice igual o mayor que la capacidad cierra una lista.

/// Tipo de un registro libre.
pub const FREE: u16 = 0xFFFF;
/// Fin de lista escrito al añadir por el final.
pub const END: u16 = 0xFFFF;
/// Crecimiento del array, en registros.
pub const GROWTH: u16 = 0x80;
/// Tipo con guion de misión propio en `+0xC` (se libera al eliminar la tarea).
pub const KIND_MISSION_SCRIPT: u16 = 0x1B;

/// Registro de 24 bytes: `+0` vencimiento, `+4` siguiente, `+6` tipo, `+8..+0x18` carga.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaskRecord {
    pub due: u32,
    pub next: u16,
    pub kind: u16,
    /// Bytes `+8..+0x18` del registro.
    pub payload: [u8; 16],
}

impl TaskRecord {
    fn slot(offset: usize, width: usize) -> std::ops::Range<usize> {
        assert!((8..=24 - width).contains(&offset), "desplazamiento fuera de la carga");
        offset - 8..offset - 8 + width
    }

    /// `u32` en el desplazamiento del registro (8..=0x14).
    pub fn u32_at(&self, offset: usize) -> u32 {
        u32::from_le_bytes(self.payload[Self::slot(offset, 4)].try_into().unwrap())
    }

    pub fn set_u32_at(&mut self, offset: usize, value: u32) {
        self.payload[Self::slot(offset, 4)].copy_from_slice(&value.to_le_bytes());
    }

    pub fn u8_at(&self, offset: usize) -> u8 {
        self.payload[Self::slot(offset, 1)][0]
    }

    pub fn set_u8_at(&mut self, offset: usize, value: u8) {
        self.payload[Self::slot(offset, 1)][0] = value;
    }

    pub fn f32_at(&self, offset: usize) -> f32 {
        f32::from_bits(self.u32_at(offset))
    }
}

/// Qué recibe el manejador de un tipo como argumento de pila.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgKind {
    /// Ninguno: el manejador lee la cabeza por su cuenta.
    None,
    /// Puntero al registro despachado.
    Record,
    /// Puntero a la carga (`registro + 8`).
    Payload,
    /// Índice de la cabeza (`u16`; el original apila un registro de 32 bits cuyos 16 bits altos no
    /// se fijan en el tipo 4).
    HeadIndex,
}

/// Qué hace el despachador tras el manejador.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum After {
    /// Reprograma la cabeza si el manejador devuelve distinto de cero; si no, la elimina.
    KeepIfNonzero,
    /// Reprograma siempre la cabeza.
    Always,
    /// Elimina siempre la cabeza.
    Remove,
    /// Suma un intervalo fijo al registro despachado y reprograma (tipos 0x03 y 0x06).
    AddInterval(u32),
    /// Tipo 0x04: el indicador de despacho vale 0 mientras corre el manejador.
    DispatchFlagOff,
    /// Tipo 0x14: si devuelve 0 se elimina; si no, decide el global 0x006DFC18.
    HotseatGate,
    /// Caso en línea del despachador (0x1A, 0x2C, 0x37): sin manejador.
    Inline,
}

/// Fila de la tabla de saltos 0x004D8A28.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KindEntry {
    pub handler: u32,
    pub arg: ArgKind,
    pub after: After,
}

const fn k(handler: u32, arg: ArgKind, after: After) -> KindEntry {
    KindEntry { handler, arg, after }
}

use After::*;
use ArgKind::{HeadIndex, Payload, Record};

/// Los 58 tipos (0x00..=0x39). Un tipo mayor se elimina sin llamar a nada.
pub const KINDS: [KindEntry; 58] = [
    k(0x4DB330, Record, KeepIfNonzero),             // 0x00
    k(0x4DB4E0, Record, KeepIfNonzero),             // 0x01
    k(0x4DBB00, ArgKind::None, Always),             // 0x02
    k(0x4DDA40, ArgKind::None, AddInterval(0xA00)), // 0x03
    k(0x4DFC94, HeadIndex, DispatchFlagOff),        // 0x04
    k(0x4E5B84, Record, KeepIfNonzero),             // 0x05
    k(0x4E2144, ArgKind::None, AddInterval(0x700)), // 0x06
    k(0x4E23A4, Record, Remove),                    // 0x07
    k(0x4E2634, Record, Always),                    // 0x08
    k(0x4E2824, Record, KeepIfNonzero),             // 0x09
    k(0x4E2CD4, Record, KeepIfNonzero),             // 0x0A
    k(0x4E37F4, Record, KeepIfNonzero),             // 0x0B
    k(0x4E38B4, ArgKind::None, KeepIfNonzero),      // 0x0C
    k(0x4E4984, Record, Always),                    // 0x0D
    k(0x4E4A44, Record, KeepIfNonzero),             // 0x0E
    k(0x4E5664, ArgKind::None, Remove),             // 0x0F
    k(0x4E63E4, Record, KeepIfNonzero),             // 0x10
    k(0x4E6744, Record, KeepIfNonzero),             // 0x11
    k(0x4E6A54, Record, KeepIfNonzero),             // 0x12
    k(0x4E7A14, Record, KeepIfNonzero),             // 0x13
    k(0x4E7BB4, Record, HotseatGate),               // 0x14
    k(0x4E7C74, Record, KeepIfNonzero),             // 0x15
    k(0x4E8234, Record, Always),                    // 0x16
    k(0x4E8684, Payload, Remove),                   // 0x17
    k(0x4E8804, ArgKind::None, KeepIfNonzero),      // 0x18
    k(0x4E8E24, Record, Always),                    // 0x19
    k(0, ArgKind::None, Inline),                    // 0x1A
    k(0x4ECF64, Record, KeepIfNonzero),             // 0x1B
    k(0x4E9094, ArgKind::None, KeepIfNonzero),      // 0x1C
    k(0x4E9564, ArgKind::None, KeepIfNonzero),      // 0x1D
    k(0x4DFD44, Record, KeepIfNonzero),             // 0x1E
    k(0x4E0334, HeadIndex, KeepIfNonzero),          // 0x1F
    k(0x4DBBD0, ArgKind::None, Always),             // 0x20
    k(0x4DC1F0, HeadIndex, KeepIfNonzero),          // 0x21
    k(0x4DB7C0, ArgKind::None, Always),             // 0x22
    k(0x4E09D4, ArgKind::None, KeepIfNonzero),      // 0x23
    k(0x4E09D4, ArgKind::None, KeepIfNonzero),      // 0x24
    k(0x4E1B84, ArgKind::None, KeepIfNonzero),      // 0x25
    k(0x4E1E54, ArgKind::None, KeepIfNonzero),      // 0x26
    k(0x4DDC00, ArgKind::None, KeepIfNonzero),      // 0x27
    k(0x4DDF40, ArgKind::None, KeepIfNonzero),      // 0x28
    k(0x4DDE70, ArgKind::None, KeepIfNonzero),      // 0x29
    k(0x4DE4F0, ArgKind::None, Always),             // 0x2A
    k(0x4DE790, ArgKind::None, Always),             // 0x2B
    k(0, ArgKind::None, Inline),                    // 0x2C
    k(0x4E2BC4, ArgKind::None, KeepIfNonzero),      // 0x2D
    k(0x4E9A94, ArgKind::None, KeepIfNonzero),      // 0x2E
    k(0x4EA0D4, ArgKind::None, KeepIfNonzero),      // 0x2F
    k(0x4EA594, ArgKind::None, KeepIfNonzero),      // 0x30
    k(0x4EAB94, ArgKind::None, Remove),             // 0x31
    k(0x4EAFE4, ArgKind::None, KeepIfNonzero),      // 0x32
    k(0x4EC934, ArgKind::None, Remove),             // 0x33
    k(0x4E09D4, ArgKind::None, KeepIfNonzero),      // 0x34
    k(0x4E94A4, ArgKind::None, Remove),             // 0x35
    k(0x4E2CD4, Record, KeepIfNonzero),             // 0x36
    k(0, ArgKind::None, Inline),                    // 0x37
    k(0x4ECB14, ArgKind::None, KeepIfNonzero),      // 0x38
    k(0x4ECDF4, ArgKind::None, KeepIfNonzero),      // 0x39
];

/// Argumento con el que se llama a un manejador.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandlerArg {
    None,
    /// Índice del registro despachado (el original pasa su puntero).
    Record(u16),
    /// Índice del registro cuya carga se pasa (`puntero + 8`).
    Payload(u16),
    HeadIndex(u16),
}

/// Una llamada del despachador a un manejador de tipo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandlerCall {
    pub kind: u16,
    /// Dirección del manejador original.
    pub handler: u32,
    pub arg: HandlerArg,
}

/// Lo que el planificador necesita del resto de la simulación.
pub trait TaskHandlers {
    /// Un anfitrión parcial puede detenerse antes de consumir una tarea no portada.
    fn supports_task(&self, _kind: u16) -> bool {
        true
    }
    fn can_dispatch(&mut self, _scheduler: &Scheduler, kind: u16) -> bool {
        self.supports_task(kind)
    }
    /// Aviso al anfitrión de que la cabeza vencida no se pudo procesar.
    fn dispatch_blocked(&mut self, _scheduler: &Scheduler, _kind: u16) {}
    /// Ejecuta el manejador del tipo y devuelve su `eax`.
    fn run_task(&mut self, scheduler: &mut Scheduler, call: HandlerCall) -> u32;
    /// Libera el guion de misión (`+0xC` de un tipo 0x1B).
    fn release_mission_script(&mut self, pointer: u32);
    /// Destruye el objeto de `+8` de un tipo 0x34 en estado >= 7 (`scheduled_tasks_free_all`).
    fn release_task_object(&mut self, pointer: u32);
    /// `GameWorld+0xA`: número de comerciantes.
    fn merchant_count(&self) -> u16;
    /// Tipo 0x1A: método 0x004F6C10 del comerciante (se recorren de último a primero).
    fn merchant_task_1a(&mut self, scheduler: &mut Scheduler, merchant: u16);
    /// Tick del mundo (0x006DE4B4), que lee el caso 0x2C.
    fn world_tick(&self) -> u32;
    /// `OPS+0x2C != 0` (cliente de red), global 0x006DF31C.
    fn is_network_client(&self) -> bool;
    /// Tipo 0x2C: recálculo del intervalo (0x004DEA20).
    fn task_2c_recompute(&mut self, scheduler: &mut Scheduler);
    /// Tipo 0x2C: paso fuera de red (0x004DEBA0).
    fn task_2c_step(&mut self, scheduler: &mut Scheduler);
    /// Global 0x006DFC18, que decide si sobrevive el tipo 0x14.
    fn hotseat_gate(&self) -> u32;
    /// Tipo 0x37: `merchant+0x608 &= !(1 << bit)` (el desplazamiento usa `bit & 31`).
    fn clear_merchant_bit_608(&mut self, merchant: u32, bit: u32);
}

/// Situaciones que el original no comprueba y que el modelo registra en lugar de leer memoria ajena.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Diagnostics {
    /// La cabeza quedó fuera del array dentro del bucle del despachador (el original lee fuera).
    pub head_out_of_bounds: u32,
    /// Un caso en línea escribió en su registro después de que el array creciera (el original
    /// escribe en el bloque ya liberado).
    pub stale_record_writes: u32,
}

/// Objeto planificador (0x006DD73C).
#[derive(Clone, Debug, PartialEq)]
pub struct Scheduler {
    /// `+0`: array; su longitud es siempre la capacidad.
    pub records: Vec<TaskRecord>,
    /// `+4`: indicador de despacho.
    pub dispatching: u32,
    /// `+8`: cabeza de la lista de tareas.
    pub head: u16,
    /// `+0xA`: cabeza de la lista libre.
    pub free_head: u16,
    /// `+0xC`: capacidad.
    pub capacity: u16,
    /// Bloques liberados por crecimiento durante el manejador en curso (el primero es al que
    /// apunta el registro despachado).
    freed_blocks: Vec<Vec<TaskRecord>>,
    pub diagnostics: Diagnostics,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    /// 0x004EC864: informa del barco hundido o capturado a la primera misión 0x32/2 correspondiente.
    /// El capitán se consulta antes de que remove_ship lo devuelva a su pool.
    pub fn report_captured_ship(
        &mut self,
        ship: u32,
        credited: u32,
        merchant_count: u16,
        ships: &[u8],
        captains: &[u8],
        captain_count: u16,
    ) -> Result<(), &'static str> {
        let pos = usize::try_from(ship).map_err(|_| "barco inválido")?.checked_mul(0x180).ok_or("barco inválido")?;
        let image = ships.get(pos..pos + 0x180).ok_or("barco truncado")?;
        let cap = u16::from_le_bytes([image[0x42], image[0x43]]);
        let look = if cap < captain_count {
            let base = usize::from(cap) * 16;
            let person = captains.get(base..base + 16).ok_or("capitán truncado")?;
            Some((person[2], person[3]))
        } else {
            None
        };
        self.report_captured_look(ship, credited, merchant_count, look)
    }
    /// 0x004EC864 con los bytes `+2/+3` del capitán del barco leídos en la llamada (o sin capitán).
    pub fn report_captured_look(
        &mut self,
        ship: u32,
        credited: u32,
        merchant_count: u16,
        look: Option<(u8, u8)>,
    ) -> Result<(), &'static str> {
        let mut at = self.head;
        let mut seen = vec![false; usize::from(self.capacity)];
        while at < self.capacity {
            let i = usize::from(at);
            if seen[i] {
                return Err("lista de tareas cíclica");
            }
            seen[i] = true;
            let task = self.records.get_mut(i).ok_or("tarea truncada")?;
            let target = u16::from_le_bytes([task.u8_at(0x10), task.u8_at(0x11)]);
            if task.kind == 0x32 && task.u8_at(0xc) == 2 && u32::from(target) == ship {
                let owner = task.u8_at(0xd);
                if u16::from(owner) >= merchant_count {
                    task.set_u8_at(0xc, 5);
                    return Ok(());
                }
                let mut success = false;
                if u32::from(owner) == credited {
                    if let Some((p2, p3)) = look {
                        let a = u16::from_le_bytes([task.u8_at(0x12), task.u8_at(0x13)]);
                        let b = u16::from_le_bytes([task.u8_at(0x14), task.u8_at(0x15)]);
                        success = u16::from(p2) == a && u16::from(p3) == b;
                    }
                }
                task.set_u8_at(0x16, if success { 1 } else { 2 });
                return Ok(());
            }
            at = task.next;
        }
        Ok(())
    }
    /// `0x004D9F60`: incluye el primer registro fuera del horizonte, como el original.
    pub fn council_meeting_mask(&self, town: u8, tick: u32) -> u32 {
        let mut at = self.head;
        let mut mask = 0u32;
        let end = tick.wrapping_add(0x1e00);
        let mut seen = 0usize;
        while at < self.capacity && (at as usize) < self.records.len() && seen < self.records.len() {
            let r = &self.records[at as usize];
            seen += 1;
            if r.kind == 0x2e && r.u8_at(0xc) == town {
                mask |= 1u32.wrapping_shl(u32::from(r.u8_at(0xd)));
            }
            if r.due >= end {
                break;
            }
            at = r.next;
        }
        mask
    }
    /// `0x004D9FC0`: usa la ventana inicial durante todo el recorrido.
    pub fn next_free_day_for_town(&self, due: u32, town: u8) -> u32 {
        let mut result = due & 0xffff_ff00;
        let lower = result.wrapping_sub(0x100);
        let upper = result.wrapping_add(0x100);
        let mut at = self.head;
        let mut seen = 0usize;
        while at < self.capacity && (at as usize) < self.records.len() && seen < self.records.len() {
            let r = &self.records[at as usize];
            seen += 1;
            let relevant =
                r.kind == 0x20 || (r.kind == 0x21 && r.u8_at(8) == town) || (r.kind == 0x2e && r.u8_at(0xc) == town);
            if relevant {
                if r.due >= lower && r.due <= upper {
                    result = (r.due & 0xffff_ff00).wrapping_add(0x200);
                }
            } else if !matches!(r.kind, 0x21 | 0x2e) && r.due > upper {
                break;
            }
            at = r.next;
        }
        result
    }
    /// Estado tras `scheduled_tasks_free_all`: sin array, cabeza 0xFFFF.
    pub fn new() -> Self {
        Scheduler {
            records: Vec::new(),
            dispatching: 0,
            head: END,
            free_head: 0,
            capacity: 0,
            freed_blocks: Vec::new(),
            diagnostics: Diagnostics::default(),
        }
    }

    fn grow(&mut self) {
        let old_free = self.free_head;
        let capacity = self.capacity.wrapping_add(GROWTH);
        assert!(
            capacity != 0,
            "capacidad desbordada: el original reserva 0 bytes y copia fuera"
        );
        self.capacity = capacity;
        let mut block = vec![TaskRecord::default(); capacity as usize];
        if !self.records.is_empty() {
            // memcpy de `free_head` registros (el bloque viejo estaba lleno).
            let n = old_free as usize;
            block[..n].copy_from_slice(&self.records[..n]);
            self.freed_blocks.push(std::mem::take(&mut self.records));
        }
        self.records = block;
        let mut i = old_free;
        while i < self.capacity {
            self.records[i as usize].next = i.wrapping_add(1);
            i = i.wrapping_add(1);
        }
    }

    /// `scheduled_tasks_add_task`: toma un registro libre, lo pone en cabeza con `due` y lo coloca
    /// detrás de todos los que vencen en su mismo tick o antes. Devuelve su índice; el tipo y la
    /// carga los escribe quien llama.
    pub fn add_task(&mut self, due: u32) -> u16 {
        if self.free_head >= self.capacity {
            self.grow();
        }
        let index = self.free_head;
        let head = self.head;
        let record = &mut self.records[index as usize];
        self.free_head = record.next;
        record.next = head;
        self.head = index;
        record.due = due;
        self.reschedule_first();
        index
    }

    /// `reschedule_first_task`: mueve la cabeza a su sitio; queda detrás de los de igual vencimiento.
    pub fn reschedule_first(&mut self) {
        let first = self.head;
        if first >= self.capacity {
            self.diagnostics.head_out_of_bounds += 1;
            return;
        }
        let r = &mut self.records;
        let due = r[first as usize].due;
        let n = r[first as usize].next;
        if n >= self.capacity || due < r[n as usize].due {
            return;
        }
        self.head = n;
        let mut link = n;
        let mut cur = r[n as usize].next;
        while cur < self.capacity {
            if due < r[cur as usize].due {
                r[first as usize].next = cur;
                r[link as usize].next = first;
                return;
            }
            link = cur;
            cur = r[cur as usize].next;
        }
        r[first as usize].next = END;
        r[link as usize].next = first;
    }

    /// `scheduled_tasks_get_task`: índice válido si `index <= capacity` (el original admite el
    /// índice igual a la capacidad y devuelve un puntero justo fuera del array).
    pub fn get_task(&self, index: u32) -> Option<u32> {
        (index <= self.capacity as u32).then_some(index)
    }

    /// Elimina la cabeza (camino común del despachador, 0x004D869B).
    fn remove_head<H: TaskHandlers + ?Sized>(&mut self, handlers: &mut H) {
        let index = self.head;
        if index >= self.capacity {
            self.diagnostics.head_out_of_bounds += 1;
            return;
        }
        let record = &mut self.records[index as usize];
        if record.kind == KIND_MISSION_SCRIPT {
            let script = record.u32_at(0xC);
            if script != 0 {
                handlers.release_mission_script(script);
                self.records[index as usize].set_u32_at(0xC, 0);
            }
        }
        let record = &mut self.records[index as usize];
        self.head = record.next;
        record.next = self.free_head;
        record.kind = FREE;
        self.free_head = index;
    }

    /// `scheduled_tasks_remove_by_opcode`: quita de la lista todas las tareas del tipo. Libera el
    /// guion de un 0x1B pero, a diferencia del despachador, no pone `+0xC` a cero.
    pub fn remove_by_kind<H: TaskHandlers + ?Sized>(&mut self, kind: u16, handlers: &mut H) {
        // `link == None` es el campo de cabeza; `Some(i)` el `next` del registro i.
        let mut link: Option<u16> = None;
        let mut cur = self.head;
        while cur < self.capacity {
            let record = self.records[cur as usize];
            if record.kind == kind {
                match link {
                    None => self.head = record.next,
                    Some(i) => self.records[i as usize].next = record.next,
                }
                if kind == KIND_MISSION_SCRIPT && record.u32_at(0xC) != 0 {
                    handlers.release_mission_script(record.u32_at(0xC));
                }
                let r = &mut self.records[cur as usize];
                r.kind = FREE;
                r.next = self.free_head;
                self.free_head = cur;
            } else {
                link = Some(cur);
            }
            cur = match link {
                None => self.head,
                Some(i) => self.records[i as usize].next,
            };
        }
    }

    /// `scheduled_tasks_free_all`: libera cargas propias y deja el objeto vacío.
    pub fn free_all<H: TaskHandlers + ?Sized>(&mut self, handlers: &mut H) {
        for i in 0..self.records.len() {
            let record = self.records[i];
            if record.kind == KIND_MISSION_SCRIPT {
                let script = record.u32_at(0xC);
                if script != 0 {
                    handlers.release_mission_script(script);
                    self.records[i].set_u32_at(0xC, 0);
                }
            } else if record.kind == 0x34 && record.u8_at(0x15) >= 7 {
                let object = record.u32_at(8);
                if object != 0 {
                    handlers.release_task_object(object);
                    self.records[i].set_u32_at(8, 0);
                }
            }
        }
        let diagnostics = self.diagnostics;
        *self = Scheduler::new();
        self.diagnostics = diagnostics;
    }

    /// `scheduled_tasks_init_new_game(start, seed)`: vacía el array (sin liberar cargas ni tocar
    /// el indicador de despacho) y crea las 13 tareas iniciales en el orden original.
    pub fn init_new_game(&mut self, start: u32, seed: u32) {
        self.records.clear();
        self.head = END;
        self.free_head = 0;
        self.capacity = 0;
        let s = start;
        let add = |sched: &mut Scheduler, due: u32, kind: u16| -> usize {
            let i = sched.add_task(due) as usize;
            sched.records[i].kind = kind;
            i
        };
        add(
            self,
            ((seed & 0xFFFF).wrapping_add(s).wrapping_add(0x5A00)) & !0xFF,
            0x20,
        );
        let i = add(self, s.wrapping_add(3), 0x00);
        self.records[i].set_u32_at(0xC, 0);
        self.records[i].set_u32_at(8, 0);
        self.records[i].set_u8_at(0x10, 0xFF);
        let i = add(self, s.wrapping_add(5), 0x03);
        self.records[i].set_u32_at(0xC, 0);
        self.records[i].set_u32_at(8, 0);
        self.records[i].set_u32_at(0x10, 0);
        add(self, s.wrapping_add(0x121), 0x06);
        let i = add(self, s.wrapping_add(0xA), 0x19);
        self.records[i].set_u8_at(8, 0);
        let i = add(self, s.wrapping_add(0xC), 0x08);
        self.records[i].set_u32_at(8, seed & 0xF);
        self.records[i].set_u32_at(0xC, seed >> 1);
        let i = add(self, s.wrapping_add(0x14), 0x0D);
        self.records[i].set_u32_at(8, 0);
        let i = add(self, s.wrapping_add(0x80), 0x16);
        // fild de (seed >> 2) como entero de 64 bits, fstp a f32 y división por 1987.0f (0x00672828).
        let quotient = ((seed >> 2) as f32) / 1987.0f32;
        self.records[i].set_u32_at(0xC, s.wrapping_add(seed).wrapping_mul(1987) & 0xFFFF);
        self.records[i].set_u32_at(8, quotient.to_bits());
        add(self, (s | 0xFF).wrapping_add(0x100), 0x1A);
        let i = add(self, s.wrapping_add(0x5A00), 0x22);
        self.records[i].set_u32_at(8, (s & 0x3FF) % 0x28);
        self.records[i].set_u32_at(0xC, seed % 0x34 + 1);
        add(self, s.wrapping_add(0x10E00), 0x2A);
        // El original reutiliza la ranura del argumento `start` para guardar `start + 0x5A00`
        // (0x004D8C62) y la vuelve a leer: 0x2B y 0x2C vencen en `start + 0x5A00`, no en `start`.
        add(self, s.wrapping_add(0x5A00), 0x2B);
        let i = add(self, s.wrapping_add(0x5A00), 0x2C);
        self.records[i].set_u32_at(8, 0);
        self.records[i].set_u32_at(0x10, 0);
        self.records[i].set_u32_at(0xC, 0x5D00);
    }

    /// Registro al que apunta el puntero guardado por el despachador: si el array creció durante
    /// el manejador, el original sigue usando el bloque liberado.
    fn dispatched(&mut self, index: u16) -> &mut TaskRecord {
        match self.freed_blocks.first_mut() {
            Some(block) => &mut block[index as usize],
            None => &mut self.records[index as usize],
        }
    }

    fn write_dispatched_due(&mut self, index: u16, due: u32) {
        if !self.freed_blocks.is_empty() {
            self.diagnostics.stale_record_writes += 1;
        }
        self.dispatched(index).due = due;
    }

    /// `handle_scheduled_tasks_tick(tick)`: despacha mientras la cabeza venza en `tick` o antes.
    /// Tras cada manejador vuelve a leer la cabeza, así que una tarea creada con vencimiento
    /// <= `tick` se ejecuta en la misma llamada.
    pub fn dispatch<H: TaskHandlers + ?Sized>(&mut self, tick: u32, handlers: &mut H) {
        if self.head > self.capacity {
            return;
        }
        self.dispatching = 1;
        loop {
            let index = self.head;
            if index >= self.capacity {
                // El original lee un registro fuera del array; aquí se termina el despacho.
                self.diagnostics.head_out_of_bounds += 1;
                break;
            }
            if self.records[index as usize].due > tick {
                break;
            }
            self.freed_blocks.clear();
            let kind = self.records[index as usize].kind;
            if !handlers.can_dispatch(self, kind) {
                handlers.dispatch_blocked(self, kind);
                break;
            }
            let Some(entry) = KINDS.get(kind as usize).copied() else {
                self.remove_head(handlers);
                continue;
            };
            let arg = match entry.arg {
                ArgKind::None => HandlerArg::None,
                ArgKind::Record => HandlerArg::Record(index),
                ArgKind::Payload => HandlerArg::Payload(index),
                ArgKind::HeadIndex => HandlerArg::HeadIndex(index),
            };
            let call = HandlerCall {
                kind,
                handler: entry.handler,
                arg,
            };
            match entry.after {
                KeepIfNonzero => {
                    let result = handlers.run_task(self, call);
                    self.keep_if(result, handlers);
                }
                Always => {
                    handlers.run_task(self, call);
                    self.reschedule_first();
                }
                Remove => {
                    handlers.run_task(self, call);
                    self.remove_head(handlers);
                }
                AddInterval(interval) => {
                    handlers.run_task(self, call);
                    let due = self.dispatched(index).due.wrapping_add(interval);
                    self.write_dispatched_due(index, due);
                    self.reschedule_first();
                }
                DispatchFlagOff => {
                    self.dispatching = 0;
                    let result = handlers.run_task(self, call);
                    self.dispatching = 1;
                    self.keep_if(result, handlers);
                }
                HotseatGate => {
                    let result = handlers.run_task(self, call);
                    if result == 0 {
                        self.remove_head(handlers);
                    } else {
                        let gate = handlers.hotseat_gate();
                        self.keep_if(gate, handlers);
                    }
                }
                Inline => self.inline_case(kind, index, handlers),
            }
        }
        self.dispatching = 0;
    }

    fn keep_if<H: TaskHandlers + ?Sized>(&mut self, result: u32, handlers: &mut H) {
        if result != 0 {
            self.reschedule_first();
        } else {
            self.remove_head(handlers);
        }
    }

    fn inline_case<H: TaskHandlers + ?Sized>(&mut self, kind: u16, index: u16, handlers: &mut H) {
        match kind {
            0x1A => {
                let count = handlers.merchant_count();
                for merchant in (0..count).rev() {
                    handlers.merchant_task_1a(self, merchant);
                }
                let due = self.dispatched(index).due.wrapping_add(0x40) & !0xF;
                self.write_dispatched_due(index, due);
                self.reschedule_first();
            }
            0x2C => {
                let last = self.dispatched(index).u32_at(0x10);
                if handlers.world_tick().wrapping_sub(last) >= 0x5D00 {
                    handlers.task_2c_recompute(self);
                    let now = handlers.world_tick();
                    if !self.freed_blocks.is_empty() {
                        self.diagnostics.stale_record_writes += 1;
                    }
                    self.dispatched(index).set_u32_at(0x10, now);
                }
                if !handlers.is_network_client() {
                    handlers.task_2c_step(self);
                }
                let record = *self.dispatched(index);
                self.write_dispatched_due(index, record.due.wrapping_add(record.u32_at(0xC)));
                self.reschedule_first();
            }
            0x37 => {
                let record = *self.dispatched(index);
                handlers.clear_merchant_bit_608(record.u32_at(0xC), record.u32_at(8));
                self.remove_head(handlers);
            }
            _ => unreachable!("tipo en línea desconocido"),
        }
    }

    /// Índices de la lista de tareas, en orden de despacho.
    pub fn queue(&self) -> Vec<u16> {
        let mut out = Vec::new();
        let mut cur = self.head;
        while cur < self.capacity && out.len() <= self.capacity as usize {
            out.push(cur);
            cur = self.records[cur as usize].next;
        }
        out
    }
}
