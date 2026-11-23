//! Formato del mensaje del original (PRE-SPEC-NET §2): registro de 20 bytes,
//! paquete de 16 bytes de cabecera más hasta `0x34` registros y el marco de 8
//! bytes que escribe `SchnellTransfer.dll` (NET-BUFFERS).
//!
//! Funciones portadas con contraste x86 (`oracle/x86_net_packets.py`):
//! `operation_packet_clear` 0x00543DA0, `operation_packet_append_from_packet`
//! 0x00543DD0, `operation_packet_init` 0x00543EA0 y `schedule_operation` 0x00543F10.

/// Registro de operación: `u32` opcode y cuatro `u32` de carga (`+4..+0x13`).
pub type Record = [u32; 5];

/// Registro recién inicializado por `operation_record_init` (0x00535740).
pub const EMPTY_RECORD: Record = [0xFFFF_FFFF, 0, 0, 0, 0];
/// Registros por paquete.
pub const MAX_RECORDS: usize = 0x34;
/// Bytes de cabecera (los 8 primeros son de la DLL).
pub const HEADER_LEN: usize = 0x10;
/// Bytes de un paquete completo en memoria y del búfer de recepción.
pub const PACKET_LEN: usize = HEADER_LEN + MAX_RECORDS * 20;
/// Versión del protocolo enviada en `0xC3` y en el bloque de información (+0x66).
pub const PROTOCOL_VERSION: u32 = 0x85;
/// Puerto de juego y de búsqueda LAN.
pub const GAME_PORT: u16 = 0x4890;
pub const LAN_SEARCH_PORT: u16 = 0x4893;
/// Bit 15 del recuento: paquete de transferencia.
pub const TRANSFER_FLAG: u16 = 0x8000;
pub const MARK_GAME: u16 = 0xFFFE;
pub const MARK_STATISTICS: u16 = 0xFFFF;
pub const MARK_SETTINGS: u16 = 0xFFFD;
/// Ranura de un paquete vacío.
pub const NO_SLOT: u16 = 0xFFFF;

/// Paquete de operaciones (`0x424` bytes en memoria: `+0x420` enlaza el siguiente).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Packet {
    /// `+0` y `+4`: los escribe la DLL en el cable; `operation_packet_init` pone `+0` a cero.
    pub dll: [u32; 2],
    /// `+8`: tick del mundo (cliente→anfitrión) o contador por conexión (anfitrión→cliente).
    pub stamp: u32,
    /// `+0xC`: ranura.
    pub slot: u16,
    /// `+0xE`: registros válidos (bit 15 = transferencia).
    pub count: u16,
    pub records: [Record; MAX_RECORDS],
    /// `+0x420`: paquete encadenado por `schedule_operation` cuando este está lleno.
    pub next: Option<Box<Packet>>,
}

impl Default for Packet {
    fn default() -> Self {
        Packet::new()
    }
}

impl Packet {
    /// `operation_packet_init` (0x00543EA0): registros a `EMPTY_RECORD`, `+0`/`+8` a cero, vaciado y sin enlace.
    /// `+4` queda como estaba en memoria; aquí se fija a cero.
    pub fn new() -> Packet {
        Packet { dll: [0; 2], stamp: 0, slot: NO_SLOT, count: 0, records: [EMPTY_RECORD; MAX_RECORDS], next: None }
    }

    /// `operation_packet_clear` (0x00543DA0): recuento 0, ranura `0xFFFF` y opcode `-1` en los 52 registros
    /// (la carga se conserva).
    pub fn clear(&mut self) {
        self.count = 0;
        self.slot = NO_SLOT;
        for r in &mut self.records {
            r[0] = 0xFFFF_FFFF;
        }
    }

    /// Registros válidos (sin el bit de transferencia).
    pub fn len(&self) -> usize {
        usize::from(self.count).min(MAX_RECORDS)
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn valid(&self) -> &[Record] {
        &self.records[..self.len()]
    }

    /// `schedule_operation` (0x00543F10): añade el registro; con 52 ya ocupados lo
    /// programa en el paquete encadenado, que se crea (inicializado y vaciado) si falta.
    pub fn schedule(&mut self, record: Record) {
        if usize::from(self.count) < MAX_RECORDS {
            self.records[usize::from(self.count)] = record;
            self.count += 1;
            return;
        }
        // Con el recuento a 0x34 o más (también con el bit 15) el original encadena.
        self.next.get_or_insert_with(|| Box::new(Packet::new())).schedule(record);
    }

    /// `operation_packet_append_from_packet` (0x00543DD0): copia registros de `src` mientras
    /// `self` tenga sitio, desplaza los no copiados al principio de `src` y deja en `src`
    /// su número. Devuelve si quedan registros en `src`.
    ///
    /// Con `self` lleno no toca nada y devuelve `src.count != 0` (con el bit 15 incluido).
    pub fn append_from(&mut self, src: &mut Packet) -> bool {
        if usize::from(self.count) >= MAX_RECORDS {
            return src.count != 0;
        }
        let total = usize::from(src.count);
        let mut taken = 0usize;
        // Bucle del original: compara con el recuento de 16 bits de `src`, que puede
        // superar 0x34 (NET-Q3); aquí se acota a los 52 registros que existen.
        while taken < total && usize::from(self.count) < MAX_RECORDS {
            self.records[usize::from(self.count)] = src.records[taken.min(MAX_RECORDS - 1)];
            self.count += 1;
            taken += 1;
        }
        let mut moved = 0usize;
        while taken + moved < total {
            let from = (taken + moved).min(MAX_RECORDS - 1);
            src.records[moved.min(MAX_RECORDS - 1)] = src.records[from];
            moved += 1;
        }
        src.count = moved as u16;
        moved != 0
    }

    /// `0x00544040`: vacía el paquete y le pasa los registros del encadenado, que se
    /// libera (su propio enlace pasa a este paquete).
    pub fn promote_chain(&mut self) {
        self.count = 0;
        if let Some(mut next) = self.next.take() {
            self.append_from(&mut next);
            self.next = next.next.take();
        }
    }

    /// Bytes en el cable: cabecera de 16 y `n` registros (`16 + 20·n`). Los 8 primeros
    /// bytes los rellena [`frame`].
    pub fn to_wire(&self) -> Vec<u8> {
        let n = self.len();
        let mut out = Vec::with_capacity(HEADER_LEN + 20 * n);
        out.extend_from_slice(&self.dll[0].to_le_bytes());
        out.extend_from_slice(&self.dll[1].to_le_bytes());
        out.extend_from_slice(&self.stamp.to_le_bytes());
        out.extend_from_slice(&self.slot.to_le_bytes());
        out.extend_from_slice(&self.count.to_le_bytes());
        for r in &self.records[..n] {
            for w in r {
                out.extend_from_slice(&w.to_le_bytes());
            }
        }
        out
    }

    /// Paquete completo de `0x420` bytes (envío de transferencias y recepción).
    pub fn to_full(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(PACKET_LEN);
        out.extend_from_slice(&self.dll[0].to_le_bytes());
        out.extend_from_slice(&self.dll[1].to_le_bytes());
        out.extend_from_slice(&self.stamp.to_le_bytes());
        out.extend_from_slice(&self.slot.to_le_bytes());
        out.extend_from_slice(&self.count.to_le_bytes());
        for r in &self.records {
            for w in r {
                out.extend_from_slice(&w.to_le_bytes());
            }
        }
        out
    }

    /// Lee un paquete recibido. Como el original, no valida el recuento frente a los
    /// bytes recibidos: los registros que faltan quedan como en un paquete nuevo y un
    /// recuento mayor que 0x34 se conserva (lo acotan los consumidores).
    pub fn from_wire(bytes: &[u8]) -> Result<Packet, &'static str> {
        if bytes.len() < HEADER_LEN {
            return Err("paquete más corto que la cabecera");
        }
        let u32_at = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
        let mut p = Packet::new();
        p.dll = [u32_at(0), u32_at(4)];
        p.stamp = u32_at(8);
        p.slot = u16::from_le_bytes([bytes[0xC], bytes[0xD]]);
        p.count = u16::from_le_bytes([bytes[0xE], bytes[0xF]]);
        for (i, r) in p.records.iter_mut().enumerate() {
            let at = HEADER_LEN + 20 * i;
            if at + 20 > bytes.len() {
                break;
            }
            for (k, w) in r.iter_mut().enumerate() {
                *w = u32_at(at + 4 * k);
            }
        }
        Ok(p)
    }

    /// Bytes de carga de un paquete de transferencia (desde `+0x10`).
    pub fn transfer_payload(bytes: &[u8]) -> &[u8] {
        bytes.get(HEADER_LEN..).unwrap_or(&[])
    }
}

/// Marco de `Network_SendMessage`: bytes 0..2 = tres primeros bytes de la firma del
/// contexto, byte 3 = `0xFF`, bytes 4..7 = tamaño total. Un tamaño menor que 8 se
/// rechaza (código `0x29A`).
pub fn frame(signature: [u8; 3], message: &mut [u8]) -> Result<(), u32> {
    if message.len() < 8 {
        return Err(0x29A);
    }
    message[..3].copy_from_slice(&signature);
    message[3] = 0xFF;
    let size = message.len() as u32;
    message[4..8].copy_from_slice(&size.to_le_bytes());
    Ok(())
}

/// Comprobación de `Network_ReceiveMessage`: la firma baja de 16 bits debe coincidir,
/// el byte 3 ser `0xFF` y la longitud de la cabecera igual a la recibida.
pub fn check_frame(signature: [u8; 3], message: &[u8]) -> bool {
    message.len() >= 8
        && message[..2] == signature[..2]
        && message[3] == 0xFF
        && u32::from_le_bytes(message[4..8].try_into().unwrap()) as usize == message.len()
}

/// Divide un flujo TCP en mensajes con el marco de la DLL. Devuelve los mensajes
/// completos y deja el resto en `buffer` (la DLL lo guarda en `conn+4`).
pub fn split_frames(buffer: &mut Vec<u8>) -> Result<Vec<Vec<u8>>, &'static str> {
    let mut out = Vec::new();
    loop {
        if buffer.len() < 8 {
            return Ok(out);
        }
        if buffer[3] != 0xFF {
            return Err("marco sin marcador 0xFF");
        }
        let size = u32::from_le_bytes(buffer[4..8].try_into().unwrap()) as usize;
        if !(8..=0x20000).contains(&size) {
            return Err("tamaño de marco fuera de rango");
        }
        if buffer.len() < size {
            return Ok(out);
        }
        out.push(buffer.drain(..size).collect());
    }
}
