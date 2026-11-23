//! Ficheros de ruta comercial `.rou` (A03): carga `0x004D5EE0`, guardado `0x004D5C50` y descompresor `0x0054C480`.
//!
//! Un `.rou` es un bloque comprimido de registros de 220 bytes. La carga rehace el primer `u16` de cada registro como índice
//! encadenado; el guardado recorre la cadena desde un registro inicial y deja el primero de cada registro a cero.
//! Los ficheros del disco y el compresor son ganchos del anfitrión. Referencia: x86 original en emulación
//! (`reimpl/oracle/x86_route.py`).

/// Tamaño en bytes de un registro de ruta (`0xDC`).
pub const RECORD_SIZE: usize = 220;

/// Bit 2 del byte `+3` de un registro: marca el primero de la ruta al guardar.
const FIRST_FLAG: u8 = 4;

struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
}

impl Bits<'_> {
    fn take(&mut self, n: usize) -> Option<u32> {
        if self.pos + n > self.data.len() * 8 {
            return None;
        }
        let mut v = 0u32;
        for b in 0..n {
            let p = self.pos + b;
            v |= (((self.data[p >> 3] >> (p & 7)) & 1) as u32) << b;
        }
        self.pos += n;
        Some(v)
    }
}

/// `0x0054C480` (descompresor): cabecera `i32` con el tamaño descomprimido (positivo, flujo de bits; negativo, bytes sin
/// comprimir) y tamaño anunciado. Devuelve `None` ante un flujo mal formado (el original lee o escribe fuera del búfer).
pub fn decompress(src: &[u8]) -> Option<(Vec<u8>, u32)> {
    let header = i32::from_le_bytes(src.get(..4)?.try_into().ok()?);
    if header == 0 {
        return Some((Vec::new(), 0));
    }
    if header < 0 {
        let n = header.unsigned_abs() as usize;
        return Some((src.get(4..4 + n)?.to_vec(), n as u32));
    }
    let length = header as usize;
    let mut r = Bits { data: src, pos: 32 };
    let mut out: Vec<u8> = Vec::with_capacity(length);
    while out.len() < length {
        if r.take(1)? == 0 {
            out.push(r.take(8)? as u8);
            continue;
        }
        let bucket = r.take(3)? as usize;
        let distance = (1usize << (bucket + 1)) - 1 + r.take(bucket + 1)? as usize;
        if distance > out.len() {
            return None;
        }
        let mut count = 2usize;
        let mut width = 2;
        loop {
            let extra = r.take(width)? as usize;
            count += extra;
            if extra < (1 << width) - 1 {
                break;
            }
            width += 1;
            if width > 31 || count > length {
                return None;
            }
        }
        for _ in 0..count {
            let b = out[out.len() - distance];
            out.push(b);
        }
    }
    Some((out, header as u32))
}

/// Lectura de ficheros del anfitrión (`0x006389EE`): se intenta primero con el contenedor del juego y después con el disco.
pub trait RouLoadHost {
    /// `second` es `false` en el primer intento y `true` en el segundo; `None` si el fichero no existe.
    fn open(&mut self, second: bool) -> Option<Vec<u8>>;
    /// `0x006389F4`: libera el fichero abierto en el intento indicado.
    fn release(&mut self, second: bool);
}

/// Resultado de la carga: registros y bytes reservados (para la traza de pruebas).
#[derive(Debug, PartialEq)]
pub struct Loaded {
    pub records: Vec<u8>,
}

/// `0x004D5EE0`: carga un `.rou`. `None` si no existe, el tamaño no es múltiplo de 220 o el descompresor no coincide con la cabecera.
///
/// Límites del original no reproducidos: una cabecera con tamaño cero reserva 0 bytes y escribe fuera; con el bit 30 o 31 puesto
/// reserva hasta 1 GiB (y un flujo mal formado lee o escribe fuera). Aquí devuelven `None`.
pub fn load_route_file(host: &mut dyn RouLoadHost) -> Option<Loaded> {
    let (data, second) = match host.open(false) {
        Some(d) => (d, false),
        None => (host.open(true)?, true),
    };
    let result = load_from_bytes(&data);
    host.release(second);
    result
}

pub fn load_from_bytes(data: &[u8]) -> Option<Loaded> {
    let header = u32::from_le_bytes(data.get(..4)?.try_into().ok()?);
    let size = (header & 0x3FFF_FFFF) as usize;
    if size % RECORD_SIZE != 0 || size == 0 || header >= 0x4000_0000 {
        return None;
    }
    let (mut records, announced) = decompress(data)?;
    if announced as usize != size || records.len() != size {
        return None;
    }
    let count = size / RECORD_SIZE;
    for i in 0..count {
        let next = if i + 1 < count { (i + 1) as u16 } else { 0 };
        records[i * RECORD_SIZE..i * RECORD_SIZE + 2].copy_from_slice(&next.to_le_bytes());
    }
    Some(Loaded { records })
}

/// Anfitrión del guardado: compresor `0x0054C410` y escritura de `save/AutoRoute/<nombre>.rou`.
pub trait RouSaveHost {
    /// Comprime el bloque de registros y devuelve el tamaño producido.
    fn compress(&mut self, records: &[u8]) -> usize;
    /// Crea la carpeta (`CreateDirectory`) y abre el fichero; `false` si no se puede abrir.
    fn open_file(&mut self, name: &str) -> bool;
    /// Escribe `size` bytes del bloque comprimido.
    fn write(&mut self, size: usize);
    /// Libera uno de los dos búferes de trabajo (`0`, bloque de registros; `1`, destino comprimido).
    fn free(&mut self, buffer: u8);
}

fn next_of(table: &[u8], i: usize) -> Option<usize> {
    let o = i * RECORD_SIZE;
    Some(u16::from_le_bytes(table.get(o..o + 2)?.try_into().ok()?) as usize)
}

/// `0x004D5C50`: guarda la ruta que empieza en el registro `start` de la tabla (`count` registros).
///
/// Cuenta los registros de la cadena (incluido el inicial; falla si llega a `count` aunque cierre el ciclo), busca la cabeza
/// (primer registro con la marca `+3 & 4` siguiendo la cadena, o el inicial si vuelve a él), copia la cadena desde la cabeza con el
/// primer `u16` a cero, comprime y escribe. Devuelve `true` si el fichero se abrió.
///
/// Límite: con `start >= count`, o si la cadena sale de la tabla antes de la marca, el original lee fuera de ella; aquí devuelve `false`.
pub fn save_route_file(table: &[u8], count: u16, start: u16, name: &str, host: &mut dyn RouSaveHost) -> bool {
    let count = count as usize;
    let start = start as usize;
    if start >= count || table.len() < count * RECORD_SIZE {
        return false;
    }
    let mut length = 0usize;
    let mut cur = start;
    loop {
        length += 1;
        let Some(n) = next_of(table, cur) else { return false };
        if length >= count {
            return false;
        }
        if n == start || n >= count {
            break;
        }
        cur = n;
    }
    let mut head = None;
    let mut cur = start;
    for _ in 0..2 * count + 2 {
        if table[cur * RECORD_SIZE + 3] & FIRST_FLAG != 0 {
            head = Some(cur);
            break;
        }
        let n = next_of(table, cur).unwrap_or(usize::MAX);
        if n == start {
            head = Some(start);
            break;
        }
        if n >= count {
            return false;
        }
        cur = n;
    }
    let Some(head) = head else { return false };
    let mut buffer = Vec::with_capacity(length * RECORD_SIZE);
    let mut cur = head;
    for k in 0..length {
        if cur >= count {
            return false;
        }
        let rec = &table[cur * RECORD_SIZE..(cur + 1) * RECORD_SIZE];
        buffer.extend_from_slice(rec);
        let o = buffer.len() - RECORD_SIZE;
        buffer[o] = 0;
        buffer[o + 1] = 0;
        cur = next_of(table, cur).unwrap_or(usize::MAX);
        if k + 1 < length && cur >= count {
            return false;
        }
    }
    let size = host.compress(&buffer);
    host.free(0);
    if !host.open_file(name) {
        host.free(1);
        return false;
    }
    host.write(size);
    true
}

/// Parada de una ruta comercial: registro de 220 bytes sin la cadena (que añade el exportador).
#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
    pub town_index: u8,
    pub flags: u8,
    pub order: [u8; 24],
    pub price: [i32; 24],
    pub amount: [i32; 24],
}

impl Stop {
    fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&[0, 0, self.town_index, self.flags]);
        out.extend_from_slice(&self.order);
        for v in self.price.iter().chain(self.amount.iter()) {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
}

/// Exporta una ruta como lo deja el guardado original (`0x004D5C50`): registros en orden con el primer `u16` a cero, la primera
/// parada con la marca de cabeza (`+3 & 4`), comprimidos con `0x0054C410`. El cargador original la acepta si el compresor produjo un
/// bloque comprimido (cabecera positiva); un bloque sin comprimir (cabecera negativa) lo rechaza siempre. `None` sin paradas, o con
/// 65.535 o más. Devuelve el bloque y si el cargador original lo aceptaría.
pub fn export_route_file(stops: &[Stop]) -> Option<(Vec<u8>, bool)> {
    if stops.is_empty() || stops.len() >= 0xFFFF {
        return None;
    }
    let mut table = Vec::with_capacity(stops.len() * RECORD_SIZE);
    for (i, st) in stops.iter().enumerate() {
        st.write(&mut table);
        let o = i * RECORD_SIZE;
        let next = if i + 1 < stops.len() { (i + 1) as u16 } else { 0 };
        table[o..o + 2].copy_from_slice(&next.to_le_bytes());
    }
    table[3] |= FIRST_FLAG;
    // El guardado original falla si la cadena abarca toda la tabla: se deja un registro libre al final.
    table.extend_from_slice(&[0; RECORD_SIZE]);
    struct Mem(Vec<u8>, Vec<u8>);
    impl RouSaveHost for Mem {
        fn compress(&mut self, records: &[u8]) -> usize {
            self.1 = crate::route::compress::compress(records);
            self.1.len()
        }
        fn open_file(&mut self, _: &str) -> bool {
            true
        }
        fn write(&mut self, size: usize) {
            self.0 = self.1[..size].to_vec();
        }
        fn free(&mut self, _: u8) {}
    }
    let mut host = Mem(Vec::new(), Vec::new());
    if !save_route_file(&table, stops.len() as u16 + 1, 0, "", &mut host) {
        return None;
    }
    let accepted = load_from_bytes(&host.0).is_some();
    Some((host.0, accepted))
}
