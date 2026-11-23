//! Formatos de carta de misión `%R %r %S %s %n %v %w %D %W %F` del intérprete `0x004ECF64`
//! (bloques `0x004ED5D2`..`0x004ED976`). Las tablas y el estado vivo que lee el original llegan en
//! `FormatExtras`; lo que no se puede reproducir con ellas devuelve error en lugar de adivinar.
use crate::ai::reputation::Merchant;

/// Bytes de la visibilidad del comerciante local (`0x006DE508 + 0x9F8·i`): `+0x6F1`, `+0x6D4..`, `+0x6D8..`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocalVisibility {
    pub flag: u8,
    pub first: [u8; 4],
    pub family: [u8; 4],
}

#[derive(Clone, Debug, Default)]
pub struct FormatExtras {
    /// `0x006A0608` (16 punteros): título según oficio y género (`%R`).
    pub titles_a: Vec<Vec<u8>>,
    /// `0x006A0718` (16 punteros): variante `%r`.
    pub titles_b: Vec<Vec<u8>>,
    /// `0x006C0CB4` (4 punteros): tipos de barco (`%S`).
    pub ship_types: Vec<Vec<u8>>,
    /// Nombre (`+0x160`) de cada barco del vector `0x006DD7A4`; su longitud es `u16 0x006DD894`.
    pub ship_records: Vec<Vec<u8>>,
    /// Cadenas apuntadas por `comerciante+0xE4` y `+0xE8` (`%n`, `%v`).
    pub merchant_strings: Vec<[Vec<u8>; 2]>,
    /// `0x006C0BB0` (24 punteros): mercancías (`%W`, `%w`).
    pub goods_names: Vec<Vec<u8>>,
    /// `0x00672C14` (24 bytes): marca de plural de `%w`.
    pub goods_flags: Vec<u8>,
    /// `0x006C0D9C` (13 punteros): meses (`%D`); el 0 es vacío.
    pub month_names: Vec<Vec<u8>>,
    /// u16 desde `0x00672D76` (al menos 258 entradas): tabla acumulada de días.
    pub calendar: Vec<u16>,
    /// Gestor de comerciantes `0x006DE4A0`: bytes `+0x8D`, `+0x8E` y u16 `+0x0A`.
    /// `None` si el anfitrión no conoce el jugador local: `%F` se rechaza.
    pub local_current: Option<u8>,
    pub local_alt: u8,
    pub local_count: u16,
    /// Visibilidad por comerciante local (`%F` cuando el actual ya no es un comerciante válido).
    pub local_visibility: Vec<LocalVisibility>,
}

/// Fuente de `FormatExtras`. Permite que el anfitrión las construya solo si un comando de carta (`0xF7`) llega a
/// pedirlas: montarlas cuesta mucho más que ejecutar un guion que está esperando.
pub trait FormatExtrasSource {
    fn format_extras(&self) -> &FormatExtras;
}
impl FormatExtrasSource for FormatExtras {
    fn format_extras(&self) -> &FormatExtras { self }
}
/// Construcción diferida: `build` se ejecuta como mucho una vez, en el primer uso.
pub struct LazyFormatExtras<F: Fn() -> FormatExtras> {
    cell: std::cell::OnceCell<FormatExtras>,
    build: F,
}
impl<F: Fn() -> FormatExtras> LazyFormatExtras<F> {
    pub fn new(build: F) -> Self { Self { cell: std::cell::OnceCell::new(), build } }
}
impl<F: Fn() -> FormatExtras> FormatExtrasSource for LazyFormatExtras<F> {
    fn format_extras(&self) -> &FormatExtras { self.cell.get_or_init(&self.build) }
}

fn cstr(v: &[u8]) -> &[u8] {
    v.split(|&c| c == 0).next().unwrap_or_default()
}

/// `0x005333C0`: pares (nombre de pila, apellido) del comerciante local.
pub fn local_name_pair(ex: &FormatExtras, merchants: &[Merchant]) -> Result<(u8, u8), String> {
    let current = ex.local_current.ok_or("jugador local no determinado")?;
    let cur = u16::from(current);
    if cur < ex.local_count {
        let m = merchants.get(usize::from(current)).ok_or("comerciante local ausente")?;
        let (a, b) = (m.bytes.get(0x1c), m.bytes.get(0x1d));
        return Ok((*a.ok_or("registro truncado")?, *b.ok_or("registro truncado")?));
    }
    let vis = ex.local_visibility.get(usize::from(ex.local_alt)).ok_or("visibilidad local ausente")?;
    let esi = if u16::from(vis.flag) < ex.local_count {
        0
    } else {
        (usize::from(current).wrapping_neg().wrapping_sub(1)) & 3
    };
    Ok((vis.first[esi], vis.family[esi]))
}

/// `0x005311D0`: (día, mes, año) de una marca de tiempo.
pub fn split_date(ex: &FormatExtras, value: u32) -> Result<(u8, u8, u16), String> {
    let ecx = (value as i32) >> 8;
    let prod = i64::from(ecx) * i64::from(0xb38cf9b1u32 as i32);
    let mut edx = ((prod >> 32) as i32).wrapping_add(ecx);
    edx >>= 8;
    edx = edx.wrapping_add(((edx as u32) >> 31) as i32);
    let year = edx as u16;
    let day = ecx.wrapping_sub(365i32.wrapping_mul(i32::from(year)));
    let cal = |i: usize| ex.calendar.get(i).copied().ok_or_else(|| "tabla de calendario truncada".to_string());
    let mut month = 0u8;
    if !(i32::from(cal(1)?) > day) {
        loop {
            month = month.wrapping_add(1);
            if i32::from(cal(usize::from(month) + 1)?) > day {
                break;
            }
        }
    }
    let low = (cal(usize::from(month))? & 0xff) as u8;
    let d = (day as u8).wrapping_sub(low).wrapping_add(1);
    Ok((d, month, year))
}

/// Texto que el original copia para los formatos de este módulo; `None` si el formato no es de este módulo.
/// `value` es la variable del guion; `F` ignora el operando.
pub fn format_extra(
    kind: u8,
    value: u32,
    ex: &FormatExtras,
    merchants: &[Merchant],
    first_name: &dyn Fn(u8) -> Result<Vec<u8>, String>,
    family_name: &dyn Fn(u8) -> Result<Vec<u8>, String>,
) -> Option<Result<Vec<u8>, String>> {
    let run = || -> Result<Vec<u8>, String> {
        let v16 = value as u16;
        let out = match kind {
            b'R' | b'r' => {
                let table = if kind == b'R' { &ex.titles_a } else { &ex.titles_b };
                if usize::from(v16) >= merchants.len() {
                    Vec::new()
                } else {
                    let m = &merchants[usize::from(v16)];
                    let g = *m.bytes.get(0x19).ok_or("registro truncado")?;
                    let idx = *m.bytes.get(usize::from(g) + 0x39c).ok_or("registro truncado")?;
                    let gender = *m.bytes.get(0x1a).ok_or("registro truncado")?;
                    let at = usize::from(idx) + usize::from(gender) * 8;
                    cstr(table.get(at).ok_or("título fuera de tabla")?).to_vec()
                }
            }
            b'S' => cstr(ex.ship_types.get((value & 3) as usize).ok_or("tipo de barco ausente")?).to_vec(),
            b's' => match ex.ship_records.get(usize::from(v16)) {
                Some(n) => cstr(n).to_vec(),
                None => Vec::new(),
            },
            b'n' | b'v' => {
                if usize::from(v16) >= merchants.len() {
                    Vec::new()
                } else {
                    let pair = ex.merchant_strings.get(usize::from(v16)).ok_or("cadena de comerciante ausente")?;
                    cstr(&pair[usize::from(kind == b'v')]).to_vec()
                }
            }
            b'W' => {
                let r = (value as i32) % 24;
                if r < 0 {
                    return Err("resto negativo en %W no admitido".into());
                }
                cstr(ex.goods_names.get(r as usize).ok_or("mercancía ausente")?).to_vec()
            }
            b'w' => {
                let i = value as usize;
                let name = cstr(ex.goods_names.get(i).ok_or("mercancía ausente")?);
                let flag = *ex.goods_flags.get(i).ok_or("marca de mercancía ausente")?;
                let mut o = if flag != 0 { b"\\B ".to_vec() } else { b"\\L ".to_vec() };
                o.extend_from_slice(name);
                o
            }
            b'D' => {
                let (d, m, y) = split_date(ex, value)?;
                let month = cstr(ex.month_names.get(usize::from(m)).ok_or("mes fuera de tabla")?);
                let mut o = format!("{d}. ").into_bytes();
                o.extend_from_slice(month);
                o.extend_from_slice(format!(" {y}").as_bytes());
                o
            }
            b'F' => {
                let (a, b) = local_name_pair(ex, merchants)?;
                let mut o = first_name(a)?;
                o.push(b' ');
                o.extend_from_slice(&family_name(b)?);
                if o.len() > 31 {
                    return Err("%F supera el búfer de 32 bytes del original".into());
                }
                o
            }
            _ => return Err(String::new()),
        };
        Ok(out)
    };
    if !matches!(kind, b'R' | b'r' | b'S' | b's' | b'n' | b'v' | b'W' | b'w' | b'D' | b'F') {
        return None;
    }
    Some(run())
}

/// Punteros de nombre cacheados del comerciante (`+0xE4` apellido, `+0xE8` nombre de pila).
///
/// El original guarda ahí punteros a las ranuras del banco de nombres `0x006DDAA0` y no los escribe en el guardado
/// (`merchant_read_save_record` no toca `+0xE4..+0xEB`). Se refrescan en `merchant_refresh_first_name_ptr`
/// (`0x004F6420`) y `merchant_refresh_family_name_ptr` (`0x004F6450`), que el cargador del banco (`0x00511E30`)
/// llama para cada comerciante; la quiebra (`0x004F5D80`) cambia `+0x1C/+0x1D` sin refrescarlos, así que `%n/%v`
/// siguen mostrando el nombre anterior hasta el siguiente refresco. Aquí el puntero se representa por el índice
/// que se pasó a `0x00512A60`/`0x00512AE0`: las ranuras del banco tienen posición fija y la búsqueda es la misma
/// en el momento del refresco y en el del formato.
pub mod name_ptr {
    const TAG: u32 = 0x4e50_0000;
    const FIRST: u32 = 0x100;
    const FAMILY: u32 = 0x200;

    /// Valor de `+0xE8` tras `0x00512A60(index)`.
    pub fn first(index: u8) -> u32 {
        TAG | FIRST | u32::from(index)
    }
    /// Valor de `+0xE4` tras `0x00512AE0(index)`.
    pub fn family(index: u8) -> u32 {
        TAG | FAMILY | u32::from(index)
    }
    /// Índice del nombre de pila cacheado; `None` si `+0xE8` no lo escribió un refresco modelado.
    pub fn first_index(value: u32) -> Option<u8> {
        (value & !0xff == TAG | FIRST).then_some(value as u8)
    }
    /// Índice del apellido cacheado; `None` si `+0xE4` no lo escribió un refresco modelado.
    pub fn family_index(value: u32) -> Option<u8> {
        (value & !0xff == TAG | FAMILY).then_some(value as u8)
    }
}

/// `0x00512A60` (ECX = `0x006DDAA0`): bit 7 elige el banco femenino (índice `& 0x7F`); fuera de la tabla
/// (`índice >= cuenta + 0x14`) devuelve la base del búfer, que es la primera ranura.
pub fn first_name_lookup(first_names: &[Vec<Vec<u8>>; 2], value: u32) -> Result<&[u8], String> {
    let (bank, index) = if value & 0x80 != 0 { (1, (value & 0x7f) as usize) } else { (0, value as usize) };
    let names = &first_names[bank];
    let name = names.get(index).or_else(|| names.first()).ok_or("nombres de pila ausentes")?;
    Ok(cstr(name))
}

/// `0x00512AE0` (ECX = `0x006DDAA0`): apellido; fuera de la tabla devuelve la primera ranura.
pub fn family_name_lookup(family_names: &[Vec<u8>], value: u32) -> Result<&[u8], String> {
    let name = family_names.get(value as usize).or_else(|| family_names.first()).ok_or("apellidos ausentes")?;
    Ok(cstr(name))
}

/// `0x004F6420`: normaliza el bit 7 de `+0x1C` según el sexo (`+0x1A`) y cachea el nombre de pila en `+0xE8`.
/// Devuelve el índice resultante (lo que el original busca y devuelve como puntero).
pub fn refresh_first_name_ptr(merchant: &mut Merchant) -> u8 {
    let name = merchant.bytes[0x1c];
    merchant.bytes[0x1c] = if merchant.bytes[0x1a] != 0 { name & 0x7f } else { name | 0x80 };
    let index = merchant.bytes[0x1c];
    merchant.bytes[0xe8..0xec].copy_from_slice(&name_ptr::first(index).to_le_bytes());
    index
}

/// `0x004F6450`: cachea el apellido de `+0x1D` en `+0xE4`.
pub fn refresh_family_name_ptr(merchant: &mut Merchant) -> u8 {
    let index = merchant.bytes[0x1d];
    merchant.bytes[0xe4..0xe8].copy_from_slice(&name_ptr::family(index).to_le_bytes());
    index
}

/// Bucle final de `name_bank_load_from_buffer` (`0x0051203A..0x00512069`): para cada comerciante `i < cuenta`
/// (`u16 0x006DE4AA`), primero el nombre de pila y después el apellido.
pub fn refresh_loaded_merchant_names(merchants: &mut [Merchant], count: u16) -> Result<(), String> {
    let n = usize::from(count);
    if merchants.len() < n {
        return Err("comerciantes del banco de nombres incompletos".into());
    }
    for m in &mut merchants[..n] {
        if m.bytes.len() < 0xec {
            return Err("registro de comerciante truncado".into());
        }
        refresh_first_name_ptr(m);
        refresh_family_name_ptr(m);
    }
    Ok(())
}

/// Cadenas de `%n` (`+0xE4`) y `%v` (`+0xE8`) de cada comerciante, resueltas en el banco de nombres actual.
/// Falla si algún puntero no lo escribió un refresco modelado (no se inventa el texto).
pub fn merchant_name_strings(
    merchants: &[Merchant],
    first_names: &[Vec<Vec<u8>>; 2],
    family_names: &[Vec<u8>],
) -> Result<Vec<[Vec<u8>; 2]>, String> {
    merchants
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let word = |at: usize| -> Result<u32, String> {
                Ok(u32::from_le_bytes(m.bytes.get(at..at + 4).ok_or("registro de comerciante truncado")?.try_into().unwrap()))
            };
            let family = name_ptr::family_index(word(0xe4)?).ok_or_else(|| format!("apellido cacheado del comerciante {i} desconocido"))?;
            let first = name_ptr::first_index(word(0xe8)?).ok_or_else(|| format!("nombre cacheado del comerciante {i} desconocido"))?;
            Ok([family_name_lookup(family_names, u32::from(family))?.to_vec(), first_name_lookup(first_names, u32::from(first))?.to_vec()])
        })
        .collect()
}

/// Visibilidad que lee `0x005333C0` cuando el Ältermann (`GameWorld+0x8D`) no es un comerciante: alcalde (`+0x6F1`),
/// nombres (`+0x6D4`) y apellidos (`+0x6D8`) de los cuatro consejeros de cada ciudad.
pub fn town_alderman_visibility(town: &[u8]) -> Result<LocalVisibility, String> {
    let get = |r: std::ops::Range<usize>| -> Result<[u8; 4], String> {
        Ok(town.get(r).ok_or("ciudad truncada")?.try_into().unwrap())
    };
    Ok(LocalVisibility {
        flag: *town.get(0x6f1).ok_or("ciudad truncada")?,
        first: get(0x6d4..0x6d8)?,
        family: get(0x6d8..0x6dc)?,
    })
}
