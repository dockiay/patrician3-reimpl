//! Texto redactado de las cartas (rama por tipo del despacho `0x47F5EA..0x48653B` de la ventana de mensajes), con las plantillas del
//! ejecutable inglés verificado. Cada tipo sigue su rama; `sprintf` (`0x64D410`) se reproduce con `%s %i %u %c`.
//! Excepción deliberada: la 0x42 sin su texto de ejecución se compone de forma legible; el original se cae leyendo el puntero nulo.
//! Incluye el encabezado común. Primer lote (20 tipos): solo plantillas, tablas del ejecutable, bancos de nombres y fecha. Los demás tipos devuelven `None`.
//! Verificado contra el x86 con `oracle/x86_letter_text.py` (fixture `p3-core/tests/fixtures/letter_text.json`).
use sha2::{Digest, Sha256};

/// Datos de `.rdata` y `.data` del ejecutable (direcciones virtuales), copiados una vez.
#[derive(Clone)]
pub struct LetterTextBank {
    ranges: Vec<(u32, Vec<u8>)>,
}

/// Entorno de una carta: nombres que el original toma de tablas vivas.
pub trait LetterContext {
    /// Nombre de ciudad `0x6DDA00[i]`.
    fn town_name(&self, id: usize) -> Option<Vec<u8>>;
    /// Nombre de barco `ship+0x160` con el índice del original (la rama usa el barco 0 si el índice no es válido).
    fn ship_name(&self, id: usize) -> Option<Vec<u8>>;
    /// `0x512A60` sobre el banco `0x6DDAA0` (nombre de pila).
    fn first_name(&self, id: u32) -> Option<Vec<u8>>;
    /// `0x512AE0` sobre el banco `0x6DDAA0` (apellido).
    fn family_name(&self, id: u32) -> Option<Vec<u8>>;
    /// `0x512B20` sobre el banco `0x6DDAA0` (tercera tabla: títulos).
    fn bank_c_name(&self, id: u32) -> Option<Vec<u8>>;
    /// Entrada `idx` del array `0x6DD7A0` (16 bytes): los bytes `+2` y `+3` (ids de nombre de pila y apellido del capitán).
    fn captain_name_ids(&self, idx: usize) -> Option<(u8, u8)>;
    /// Tercera tabla de textos urbanos (`0x6DD8C0`, entradas 80.. del banco de nombres de ciudad).
    fn town_text3(&self, id: usize) -> Option<Vec<u8>>;
    /// Identificador del objeto de imagen de mapa que crea `0x51AC60` para las cartas 0x46/0x6E (objeto de interfaz; sin él no se compone).
    fn map_image_handle(&self) -> Option<u32>;
    /// Texto de mercancías de `0x636620` (`format_supply_request_goods_text`) para los bytes `+8..+0xC` de la carta; `None` si no está disponible.
    fn goods_text(&self, args: [u8; 5]) -> Option<Vec<u8>>;
    /// Ficha del comerciante `idx` (`0x5303C0`); `None` si no existe.
    fn merchant(&self, idx: u32) -> Option<PlayerInfo>;
    /// Comerciante local.
    fn player(&self) -> Option<PlayerInfo>;
    /// Memoria del x86 (`len` bytes en `addr`): cuerpo de texto libre de las cartas 0x29/0x47/0x63/0x83 y bloques de mensaje de las 0x3D/0x3E/0x3F; `None` si no se puede leer.
    fn memory(&self, addr: u32, len: u32) -> Option<Vec<u8>>;
    /// Valor global del x86 que las ramas consultan: `0x6DE4AA` (u16: comerciantes), `0x6DE584` (lista de ofertas), `0x6DE4B4` (reloj), `0x6CBDD8` (ventana enlazada) y el resultado de `0x4B2200` (clave `0x4B2200`).
    fn global(&self, addr: u32) -> u32;
    /// `0x533EE0`: texto de oferta que coincide con la clave de 8 bytes (`carta+8`); `None` si no hay coincidencia.
    fn offer_text(&self, key: [u8; 8]) -> Option<Vec<u8>>;
    /// Nodo de 16 bytes `idx` de la lista de libro mayor del x86 (`[0x6DD730] + idx*16`, `0x6DD736` entradas); `None` si no existe.
    fn ledger_node(&self, idx: u32) -> Option<[u8; 16]>;
    /// Número de nodos del libro mayor (`0x6DD736`).
    fn ledger_count(&self) -> u32;
    /// Palabra `+off` de la ficha del comerciante local (`0x5303C0`).
    fn player_u16(&self, off: u32) -> u16;
    /// Texto de una tabla inicializada o de un banco vivo. El constructor CRT
    /// `0x46BD80` rellena `0x6A0C18/0x6A0C54`; `0x6A0B38` ya contiene punteros
    /// en el EXE. `0x6B7614` no se inicializó antes de dos cierres nativos de la 0x42;
    /// su ausencia activa la composición segura de esa carta. `None` si no está disponible.
    fn runtime_text(&self, slot: u32) -> Option<Vec<u8>>;
    /// Registro `idx` (24 bytes) de la lista de deudas del x86 (`0x4D8DB0` sobre `0x6DD73C`; recuento en `0x6DD748`); `None` si no existe.
    fn debt_record(&self, idx: u32) -> Option<[u8; 24]>;
    /// Palabra inicial de la fila seleccionada de la lista de mensajes.
    fn selected_row_word(&self) -> u16;
    /// Byte inicial (marcas) de la fila seleccionada de la lista de mensajes.
    fn selected_row_flags(&self) -> u8 {
        (self.selected_row_word() & 0xff) as u8
    }
    /// Número de ciudades (`0x6DE4B0`).
    fn town_count(&self) -> usize;
    /// Número de barcos (`0x6DD894`).
    fn ship_count(&self) -> usize;
    /// Texto de la fila de la lista de mensajes (`ebp+idx*0x14+8`): la fecha «DD.MM.AAAA» de la carta (`row_date_text`, `0x47C520`).
    fn row_text(&self) -> Vec<u8>;
}

/// Comerciante local (`0x6DFC14` y su ficha `get_merchant`): índice, byte `+0x1A` (bit 0: sexo) y los ids de nombre `+0x1C`/`+0x1D`.
#[derive(Debug, Clone, Copy)]
pub struct PlayerInfo {
    pub index: u32,
    pub flags: u8,
    pub first: u8,
    pub family: u8,
}

/// Efecto lateral de una rama de carta fuera del texto.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    /// Llamada a una función de la interfaz (dirección x86 y argumentos).
    Call(u32, Vec<u32>),
    /// Escritura en el objeto de la ventana de mensajes (`ebx+offset`).
    Win(u32, Vec<u8>),
    /// Escritura en un local de la rutina (`esp+offset`).
    Local(u32, Vec<u8>),
    /// Reserva de botones: valores de las ranuras de punteros de texto (`[esp+0x18]` apunta a ellas).
    Buttons(Vec<u32>),
}

/// Resultado de componer una carta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composed {
    /// Texto completo (prefijo previo + rama).
    pub text: Vec<u8>,
    /// Valor del local de interfaz `[esp+0x20]` si la rama lo escribe (decimal de float de transparencia).
    pub ui_float: Option<u32>,
    /// Efectos sobre la ventana, los locales de interfaz y llamadas a la interfaz, en orden de ejecución.
    pub effects: Vec<Effect>,
}

enum Arg<'a> {
    S(&'a [u8]),
    I(i32),
    U(u32),
    C(u8),
}

fn sprintf(template: &[u8], args: &[Arg]) -> Result<Vec<u8>, String> {
    // `0x64D410` admite `%$N$s` (argumento N, base 1); sin `$N$` cada conversión toma el argumento de su número de orden.
    let (mut out, mut i, mut n) = (Vec::new(), 0, 0);
    while i < template.len() {
        let c = template[i];
        if c != b'%' {
            out.push(c);
            i += 1;
            continue;
        }
        let mut spec = *template.get(i + 1).ok_or("plantilla truncada")?;
        i += 2;
        if spec == b'%' {
            out.push(b'%');
            continue;
        }
        let mut index = n;
        if spec == b'$' {
            let start = i;
            while template.get(i).is_some_and(u8::is_ascii_digit) {
                i += 1;
            }
            let number: usize = std::str::from_utf8(&template[start..i]).unwrap().parse().map_err(|_| "índice posicional inválido")?;
            if template.get(i) != Some(&b'$') || number == 0 {
                return Err("plantilla posicional mal formada".into());
            }
            index = number - 1;
            spec = *template.get(i + 1).ok_or("plantilla truncada")?;
            i += 2;
        }
        let arg = args.get(index).ok_or("faltan argumentos de la plantilla")?;
        n += 1;
        match (spec, arg) {
            (b's', Arg::S(s)) => out.extend_from_slice(s),
            (b'i' | b'd', Arg::I(v)) => out.extend_from_slice(v.to_string().as_bytes()),
            (b'i' | b'd', Arg::U(v)) => out.extend_from_slice((*v as i32).to_string().as_bytes()),
            (b'u', Arg::U(v)) => out.extend_from_slice(v.to_string().as_bytes()),
            (b'u', Arg::I(v)) => out.extend_from_slice((*v as u32).to_string().as_bytes()),
            (b'c', Arg::C(v)) => out.push(*v),
            _ => return Err(format!("formato %{} no compatible", spec as char)),
        }
    }
    Ok(out)
}

impl LetterTextBank {
    pub fn from_english_exe(bytes: &[u8]) -> Result<Self, String> {
        let hash = format!("{:x}", Sha256::digest(bytes));
        if hash != "fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece" {
            return Err("ejecutable inglés no reconocido para textos de carta".into());
        }
        let image = p3_assets::exe::PeImage::parse(bytes).map_err(str::to_string)?;
        let mut ranges = Vec::new();
        for (start, len) in [(0x66a000u32, 0x28000usize), (0x692000, 0x3a000)] {
            ranges.push((start, image.bytes_at(start, len).map_err(str::to_string)?.to_vec()));
        }
        // La cola sin inicializar de `.data` (hasta su tamaño virtual) vale cero: varias ranuras apuntan a cadenas vacías allí.
        if let Some((_, data)) = ranges.last_mut() {
            data.resize(0x5fcdc, 0);
        }
        let mut bank = Self { ranges };
        // CRT constructor 0x46BD80, registered at 0x6921E4. These are
        // addresses of built-in text, not entries loaded from a save or INI.
        // Decode its small MOV-only body from the verified executable so no
        // commercial strings have to be copied into the Rust sources.
        let code = image.bytes_at(0x46bd80, 0x23b).map_err(str::to_string)?;
        let (mut pc, mut eax) = (0usize, 0u32);
        loop {
            let word = |at: usize| u32::from_le_bytes(code[at..at+4].try_into().unwrap());
            let (slot, value) = match code[pc] {
                0xa1 => { eax = bank.u32(word(pc+1))?; pc += 5; continue; }
                0xa3 => { let slot=word(pc+1); pc+=5; (slot,eax) }
                0xc7 if code[pc+1] == 0x05 => { let pair=(word(pc+2),word(pc+6)); pc+=10; pair }
                0xc3 => break,
                _ => return Err("unexpected text-table constructor instruction".into()),
            };
            if !(0x6a0bc8..=0x6a0ca8).contains(&slot) {
                return Err("text-table constructor destination outside its table".into());
            }
            let (base,data)=bank.ranges.iter_mut().find(|(base,data)|slot>=*base && (slot-*base) as usize+4<=data.len()).ok_or("text-table destination unavailable")?;
            let at=(slot-*base) as usize;
            data[at..at+4].copy_from_slice(&value.to_le_bytes());
        }
        Ok(bank)
    }
    fn at(&self, address: u32) -> Option<&[u8]> {
        self.ranges.iter().find(|(s, d)| address >= *s && ((address - s) as usize) < d.len()).map(|(s, d)| &d[(address - s) as usize..])
    }
    fn u32(&self, address: u32) -> Result<u32, String> {
        let b = self.at(address).and_then(|b| b.get(..4)).ok_or_else(|| format!("dirección {address:#x} fuera de los datos"))?;
        Ok(u32::from_le_bytes(b.try_into().unwrap()))
    }
    fn hull(&self, i: u32) -> Result<&[u8], String> {
        self.slot(0x6c0e80 + 4 * i)
    }
    fn month_name(&self, i: u32) -> Result<&[u8], String> {
        self.slot(0x6c0d00 + 4 * i)
    }
    fn cstr(&self, address: u32) -> Result<&[u8], String> {
        if let Some(t) = p3_assets::exe::translation(address) {
            return Ok(t);
        }
        let b = self.at(address).ok_or_else(|| format!("cadena {address:#x} fuera de los datos"))?;
        let end = b.iter().position(|&c| c == 0).ok_or("cadena sin terminador")?;
        Ok(&b[..end])
    }
    /// Cadena apuntada por la ranura `slot`.
    fn slot(&self, slot: u32) -> Result<&[u8], String> {
        let p = self.u32(slot)?;
        if p == 0 {
            return Err(format!("ranura {slot:#x} vacía"));
        }
        self.cstr(p)
    }

    /// Text pointer after the verified CRT constructor. A null pointer remains
    /// unavailable; in particular, 0x6B7614 is not assigned by 0x46BD80.
    pub fn initialized_text(&self, slot: u32) -> Option<Vec<u8>> {
        self.slot(slot).ok().map(<[u8]>::to_vec)
    }

    /// `0x5311D0`: (año, mes 0-based, día) de un tick, con las tablas de inicio de mes de `0x672D76`.
    fn ticks_to_date(&self, tick: u32) -> Result<(u16, u8, u8), String> {
        let h = |i: u32| -> Result<i32, String> { Ok(i32::from(u16::from_le_bytes(self.at(0x672d76 + 2 * i).and_then(|b| b.get(..2)).ok_or("tabla de meses fuera de rango")?.try_into().unwrap()))) };
        let days = (tick as i32) >> 8;
        let q = ((i64::from(days) * i64::from(0xb38cf9b1u32 as i32)) >> 32) as i32;
        let q = q.wrapping_add(days) >> 8;
        let q = q.wrapping_add(((q as u32) >> 31) as i32);
        let year = q as u16;
        let doy = days.wrapping_sub(i32::from(year).wrapping_mul(365));
        let mut month = 0u8;
        if h(1)? <= doy {
            loop {
                month = month.wrapping_add(1);
                if h(1 + u32::from(month))? > doy {
                    break;
                }
            }
        }
        let start = h(u32::from(month))? as u8;
        Ok((year, month, (doy as u8).wrapping_sub(start).wrapping_add(1)))
    }

    /// Tabla fija `0x6C0CB4[(-1-id)&3]` del tercer banco (`0x512B20` fuera de rango).
    pub fn bank_c_fallback(&self, id: u32) -> Option<Vec<u8>> {
        let k = 0xffff_ffffu32.wrapping_sub(id) & 3;
        self.slot(0x6c0cb4 + 4 * k).ok().map(<[u8]>::to_vec)
    }

    /// Fecha de una carta como `d. Mmm aaaa` (formato de `0x6B597C`); es la aproximación del texto de fila que usa el tipo 0x03.
    pub fn date_text(&self, day: u8, month0: u8, year: u16) -> Option<Vec<u8>> {
        let month = self.month_name(u32::from(month0 & 0xf) + 1).ok()?;
        let mut out = format!("{day}. ").into_bytes();
        out.extend_from_slice(month);
        out.extend(format!(" {year}").bytes());
        Some(out)
    }

    /// Compone el texto de una carta. `prefix` es el texto previo del buffer (el original ya escribió el encabezado). La carta puede
    /// modificarse (los tipos 0x12/0x26 normalizan mes y día). `Ok(None)` si el tipo aún no está portado.
    pub fn compose(&self, letter: &mut [u8; 16], ctx: &dyn LetterContext) -> Result<Option<Composed>, String> {
        let kind = letter[4];
        let w = |l: &[u8; 16], o: usize| u16::from_le_bytes([l[o], l[o + 1]]);
        let town = |i: usize| ctx.town_name(i).ok_or_else(|| format!("ciudad {i} sin nombre"));
        let t5 = town(usize::from(letter[5]))?;
        // Encabezado (`0x47F509..0x47F5C7`): marca de color, ciudad (si el índice es válido) y fecha; el cuarto carácter de color pasa a '1'
        // para las clases 0 y 3 de la tabla `0x6C0198`.
        let mut text = self.cstr(0x6b9cac)?.to_vec();
        if usize::from(letter[5]) < ctx.town_count() {
            text.extend(sprintf(self.cstr(0x6b9cb4)?, &[Arg::S(&t5)])?);
        }
        // El búfer temporal `[esp+0x64]` conserva la fecha: las ramas que no lo reescriben la copian de nuevo en la cola.
        let stale_tmp = sprintf(self.cstr(0x6b9cbc)?, &[Arg::U(u32::from(letter[0])), Arg::U(u32::from(letter[1] & 0xf) + 1), Arg::U(u32::from(w(letter, 2)))])?;
        text.extend_from_slice(&stale_tmp);
        let class = *self.at(0x6c0198 + u32::from(letter[4])).and_then(|b| b.first()).ok_or("tabla de clases fuera de rango")?;
        if class == 0 || class == 3 {
            text[5] = b'1';
        }
        let mut ui_float = None;
        let mut effects: Vec<Effect> = Vec::new();
        let mut mark = |text: &mut Vec<u8>| {
            if text.len() > 5 {
                text[5] = b'1'
            }
        };
        // Los tipos 0x12/0x26 corrigen el par mes/día antes de escribir la fecha.
        let normalize_date = |this: &Self, l: &mut [u8; 16]| -> Result<(), String> {
            let month = w(l, 0xc);
            if month == 0 {
                l[0xc..0xe].copy_from_slice(&1u16.to_le_bytes());
                l[0xe..0x10].copy_from_slice(&1u16.to_le_bytes());
            } else {
                let b = this.at(0x672d92 + 2 * u32::from(month)).and_then(|b| b.get(..2)).ok_or("tabla de días fuera de rango")?;
                if u32::from(w(l, 0xe)) == u32::from(u16::from_le_bytes([b[0], b[1]])) + 1 {
                    l[0xe..0x10].copy_from_slice(&1u16.to_le_bytes());
                    l[0xc..0xe].copy_from_slice(&month.wrapping_add(1).to_le_bytes());
                }
            }
            Ok(())
        };
        let first = |i: u32| ctx.first_name(i).ok_or_else(|| format!("nombre de pila {i} ausente"));
        let bankc = |i: u32| ctx.bank_c_name(i).ok_or_else(|| format!("entrada {i} del tercer banco ausente"));
        let family = |i: u32| ctx.family_name(i).ok_or_else(|| format!("apellido {i} ausente"));
        let piece = match kind {
            0x00 => sprintf(self.slot(0x6b2e90)?, &[Arg::S(&t5), Arg::S(self.hull(u32::from(w(letter, 0xa)))?)])?,
            0x01 => sprintf(
                self.slot(0x6b2f20)?,
                &[Arg::S(&t5), Arg::S(self.hull(u32::from(w(letter, 8)))?), Arg::U(u32::from(w(letter, 0xa))), Arg::C(0x25), Arg::U(u32::from_le_bytes(letter[0xc..0x10].try_into().unwrap()))],
            )?,
            0x03 => {
                mark(&mut text);
                ui_float = Some(0x3f40_0000);
                let building = self.slot(0x6a57c8 + 4 * u32::from(w(letter, 8)))?;
                let row = ctx.row_text();
                sprintf(self.slot(0x6b30d8)?, &[Arg::S(&t5), Arg::S(&row), Arg::S(self.slot(0x6b30dc)?), Arg::S(building)])?
            }
            0x12 => {
                normalize_date(self, letter)?;
                let mut s = sprintf(
                    self.slot(0x6b3aa0)?,
                    &[Arg::U(u32::from(w(letter, 8))), Arg::U(u32::from(w(letter, 0xe))), Arg::U(u32::from(w(letter, 0xc))), Arg::U(u32::from(w(letter, 0xa))), Arg::S(&t5)],
                )?;
                s.extend_from_slice(self.slot(0x6b3aa4)?);
                s
            }
            0x14 => {
                let refund = i32::from_le_bytes(letter[8..12].try_into().unwrap());
                let (year, month) = (u32::from(w(letter, 0xe)) / 12, u32::from(w(letter, 0xe)) % 12);
                let day = u32::from(letter[0xd]) >> 2;
                let hull_name = self.hull(u32::from(letter[0xd]) & 3)?;
                let month_name = self.slot(0x6c0d04 + 4 * month)?;
                if refund >= 0 {
                    sprintf(self.slot(0x6b3e94)?, &[Arg::U(day), Arg::S(month_name), Arg::U(year), Arg::S(hull_name), Arg::S(&t5), Arg::I(refund)])?
                } else {
                    sprintf(self.slot(0x6b3e98)?, &[Arg::U(day), Arg::S(month_name), Arg::U(year), Arg::S(hull_name), Arg::S(&t5)])?
                }
            }
            0x17 => {
                let idx = usize::from(w(letter, 0xe));
                let ship = ctx.ship_name(if idx < ctx.ship_count() { idx } else { 0 }).ok_or("barco sin nombre")?;
                let date = u32::from(w(letter, 0xa));
                let dest = town(usize::from(w(letter, 8)))?;
                sprintf(
                    self.slot(0x6b41f8)?,
                    &[Arg::S(&t5), Arg::S(&ship), Arg::U(date & 0x1f), Arg::S(self.month_name(date >> 5)?), Arg::S(&dest), Arg::U(u32::from(w(letter, 0xc)))],
                )?
            }
            0x18 => {
                let dest = town(usize::from(w(letter, 0xc)))?;
                sprintf(self.slot(0x6b43d0 + 4 * u32::from(letter[8] & 3))?, &[Arg::S(&dest), Arg::U(u32::from(w(letter, 0xa)))])?
            }
            0x1d => {
                mark(&mut text);
                self.cstr(0x6b9cd8)?.to_vec()
            }
            0x20 => {
                mark(&mut text);
                ui_float = Some(0x3f66_6666);
                let mut s = sprintf(self.slot(0x6b50c8)?, &[Arg::S(&t5)])?;
                if w(letter, 0xe) > 0 {
                    s.extend(sprintf(self.slot(0x6b50cc)?, &[Arg::U(u32::from(w(letter, 0xe)))])?);
                }
                if w(letter, 0xa) > 0 && w(letter, 0xc) > 0 {
                    s.extend(sprintf(self.slot(0x6b50d0)?, &[Arg::U(u32::from(w(letter, 0xc)))])?);
                    for k in 0..16u32 {
                        if u32::from(w(letter, 0xa)) & (1 << k) != 0 {
                            s.push(b' ');
                            s.extend_from_slice(self.slot(0x6c0bb0 + 4 * k)?);
                        }
                    }
                }
                s
            }
            0x24 => sprintf(self.slot(0x6b57fc + 4 * u32::from(w(letter, 0xc) & 3))?, &[Arg::U(u32::from(w(letter, 0xa))), Arg::S(&t5)])?,
            0x26 => {
                normalize_date(self, letter)?;
                sprintf(
                    self.slot(0x6b597c)?,
                    &[Arg::U(u32::from(w(letter, 8))), Arg::U(u32::from(w(letter, 0xe))), Arg::S(self.month_name(u32::from(w(letter, 0xc)))?), Arg::U(u32::from(w(letter, 0xa))), Arg::S(&t5)],
                )?
            }
            0x43 => {
                let t8 = town(usize::from(w(letter, 8)))?;
                sprintf(self.slot(0x6b76f8)?, &[Arg::S(&t8), Arg::S(&t8)])?
            }
            0x60 => {
                mark(&mut text);
                sprintf(self.slot(0x6b9168)?, &[Arg::S(&t5)])?
            }
            0x61 => {
                mark(&mut text);
                sprintf(self.slot(0x6b916c)?, &[Arg::S(&t5), Arg::U(u32::from(w(letter, 8)))])?
            }
            0x66 | 0x67 | 0x68 => {
                let a = town(usize::from(w(letter, 8)))?;
                let b = town(usize::from(w(letter, 0xa)))?;
                let slot = match kind {
                    0x66 => 0x6aadd8,
                    0x67 => 0x6aaddc,
                    _ => 0x6aadd8 + 4 * (2 + 2 * u32::from(w(letter, 8) == w(letter, 0xa))),
                };
                if kind == 0x66 {
                    ui_float = Some(0x3f60_0000);
                }
                sprintf(self.slot(slot)?, &[Arg::S(&a), Arg::S(&b)])?
            }
            0x69 => sprintf(self.slot(0x6aade4)?, &[Arg::S(&t5)])?,
            0x7c => sprintf(self.slot(0x6b1e04)?, &[Arg::S(&t5), Arg::U(u32::from(w(letter, 8)))])?,
            0x81 => {
                let mut s = sprintf(self.slot(0x6b19e8)?, &[Arg::S(&t5)])?;
                s.extend(sprintf(self.slot(0x6b19ec)?, &[Arg::S(&t5)])?);
                s
            }
            0x04 | 0x05 => {
                if kind == 0x04 {
                    mark(&mut text);
                    ui_float = Some(0x3f40_0000);
                }
                let (first_id, family_id) = ctx.captain_name_ids(usize::from(w(letter, 8))).ok_or("capitán sin ficha")?;
                let ship = ctx.ship_name(usize::from(w(letter, 0xa))).ok_or("barco sin nombre")?;
                let (first, family) = (first(u32::from(first_id))?, family(u32::from(family_id))?);
                sprintf(self.slot(if kind == 0x04 { 0x6b317c } else { 0x6b31e0 })?, &[Arg::S(&first), Arg::S(&family), Arg::S(&t5), Arg::S(&ship)])?
            }
            0x16 => {
                let (a, b) = (first(u32::from(w(letter, 8)))?, family(u32::from(w(letter, 0xa)))?);
                sprintf(self.slot(0x6b3f44)?, &[Arg::S(&a), Arg::S(&b)])?
            }
            0x2b => {
                let (a, b) = (first(u32::from(w(letter, 8)))?, family(u32::from(w(letter, 0xa)))?);
                sprintf(self.slot(0x6b5bac)?, &[Arg::U(u32::from(w(letter, 0xc))), Arg::S(&a), Arg::S(&b)])?
            }
            0x33 => {
                let mut s = self.slot(0x6b63b0)?.to_vec();
                let (a, b) = (first(u32::from(letter[0xb]))?, family(u32::from(letter[0xa]))?);
                let month = self.slot(0x6c0da0 + 4 * u32::from(letter[0xc]))?;
                s.extend(sprintf(self.slot(0x6b63b4)?, &[Arg::S(&a), Arg::S(&b), Arg::U(u32::from(letter[0xd])), Arg::S(month), Arg::U(u32::from(w(letter, 0xe)))])?);
                let t8 = town(usize::from(w(letter, 8)))?;
                s.extend(sprintf(self.slot(0x6b63b8)?, &[Arg::S(&t8)])?);
                s
            }
            0x36 => {
                let mut s = self.slot(0x6b65e0)?.to_vec();
                let honorific = 1 + u32::from(w(letter, 8) > 0x80);
                let (a, b) = (first(u32::from(w(letter, 8)))?, family(u32::from(w(letter, 0xa)))?);
                s.extend(sprintf(self.slot(0x6b65e0 + 4 * honorific)?, &[Arg::S(&a), Arg::S(&b)])?);
                s.extend_from_slice(self.slot(0x6b65ec)?);
                let amount = (u32::from(w(letter, 0xc)) << 16).wrapping_add(u32::from(w(letter, 0xe)));
                s.extend(sprintf(self.slot(0x6b65f0)?, &[Arg::U(amount), Arg::S(&t5)])?);
                s
            }
            0x3b => {
                let (a, b) = (first(u32::from(w(letter, 0xa)))?, family(u32::from(w(letter, 0xc)))?);
                let mut s = sprintf(self.slot(if w(letter, 8) != 0 { 0x6b6f48 } else { 0x6b6f4c })?, &[Arg::S(&a), Arg::S(&b)])?;
                s.extend_from_slice(self.slot(0x6b6f50)?);
                s
            }
            0x4f => {
                let mut s = self.slot(0x6b8324)?.to_vec();
                s.extend(first(u32::from(w(letter, 0xc)))?);
                s.push(b' ');
                s.extend(family(u32::from(w(letter, 0xe)))?);
                s.extend_from_slice(self.slot(0x6b8328)?);
                let (a, b) = (first(u32::from(w(letter, 8)))?, family(u32::from(w(letter, 0xa)))?);
                s.extend(sprintf(self.cstr(0x6b9d34)?, &[Arg::S(&a), Arg::S(&b)])?);
                s
            }
            0x31 => {
                let mut s = self.slot(0x6b61e8)?.to_vec();
                s.extend(bankc(u32::from(w(letter, 0xe)))?);
                s.extend_from_slice(self.slot(0x6b61ec)?);
                s.extend_from_slice(&t5);
                s.extend_from_slice(self.slot(0x6b61f0)?);
                s.extend(sprintf(self.slot(0x6b61f4)?, &[Arg::U(u32::from(w(letter, 8)))])?);
                s
            }
            0x32 => {
                let mut s = self.slot(0x6b62a4)?.to_vec();
                s.extend_from_slice(self.slot(0x6a0608 + 4 * (u32::from(letter[0xd] & 7) + 8 * u32::from(letter[0xc] & 1)))?);
                s.push(b' ');
                s.extend(first(u32::from(letter[0xf]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xe]))?);
                s.extend_from_slice(self.slot(0x6b62a8)?);
                s.extend_from_slice(&t5);
                s.extend_from_slice(self.slot(0x6b62ac)?);
                s.extend(sprintf(self.slot(0x6b62b0)?, &[Arg::U(u32::from(w(letter, 8)))])?);
                s
            }
            0x4d => {
                let mut s = self.slot(0x6b8194)?.to_vec();
                s.extend(first(u32::from(w(letter, 8)))?);
                s.push(b' ');
                s.extend(family(u32::from(w(letter, 0xa)))?);
                s.extend_from_slice(self.slot(0x6b8198)?);
                s.extend(bankc(u32::from(w(letter, 0xc)))?);
                s.extend_from_slice(self.slot(0x6b819c)?);
                s.extend(sprintf(self.slot(0x6b81a0)?, &[Arg::U(u32::from(w(letter, 0xe)))])?);
                s
            }
            0x4e => {
                let mut s = self.slot(0x6b8288)?.to_vec();
                s.extend(first(u32::from(letter[0xf]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xe]))?);
                s.extend_from_slice(self.slot(0x6b828c)?);
                s.extend(first(u32::from(w(letter, 8)))?);
                s.push(b' ');
                s.extend(family(u32::from(w(letter, 0xa)))?);
                s.extend_from_slice(self.slot(0x6b8290)?);
                s.extend(bankc(u32::from(w(letter, 0xc)))?);
                s
            }
            0x50 => {
                let mut s = first(u32::from(letter[0xf]))?;
                s.push(b' ');
                s.extend(family(u32::from(letter[0xe]))?);
                s.extend_from_slice(self.slot(0x6b83d0)?);
                s.extend(bankc(u32::from(w(letter, 0xc)))?);
                s.extend_from_slice(self.slot(0x6b83d4)?);
                let (a, b) = (first(u32::from(w(letter, 8)))?, family(u32::from(w(letter, 0xa)))?);
                s.extend(sprintf(self.cstr(0x6b9d3c)?, &[Arg::S(&a), Arg::S(&b)])?);
                s
            }
            0x7a => {
                let a = first(u32::from(w(letter, 8)))?;
                sprintf(self.slot(if letter[8] & 0x80 != 0 { 0x6aed88 } else { 0x6aed8c })?, &[Arg::S(&a)])?
            }
            0x02 => {
                let ship = ctx.ship_name(usize::from(w(letter, 8))).ok_or("barco ausente")?;
                let hull = self.slot(0x6c0e38 + 4 * u32::from(w(letter, 0xa)))?;
                let bank = bankc(u32::from(w(letter, 8)))?;
                let amount = i32::from_le_bytes(letter[0xc..0x10].try_into().unwrap());
                if amount > 0 {
                    sprintf(self.slot(0x6b304c)?, &[Arg::S(hull), Arg::S(&bank), Arg::S(&t5), Arg::I(amount)])?
                } else {
                    sprintf(self.slot(0x6b3048)?, &[Arg::S(hull), Arg::S(&bank), Arg::S(&ship), Arg::S(&t5)])?
                }
            }
            0x07 => {
                let w8 = w(letter, 8);
                let bank = bankc(u32::from(w8 & 0x7fff))?;
                let mut s = if w8 >= 0x8000 {
                    sprintf(self.slot(0x6b33a0)?, &[Arg::S(&bank), Arg::S(&t5)])?
                } else {
                    sprintf(self.slot(0x6b339c)?, &[Arg::S(&bank), Arg::S(&t5)])?
                };
                let amount = i32::from_le_bytes(letter[0xc..0x10].try_into().unwrap());
                s.extend(match amount {
                    0 => sprintf(self.slot(0x6b33ac)?, &[])?,
                    1.. => sprintf(self.slot(0x6b33a4)?, &[Arg::I(amount)])?,
                    _ => sprintf(self.slot(0x6b33a8)?, &[Arg::I(amount.wrapping_neg())])?,
                });
                let (a, b) = (first(u32::from(letter[0xb]))?, family(u32::from(letter[0xa]))?);
                s.extend(sprintf(self.cstr(0x6b9ccc)?, &[Arg::S(&a), Arg::S(&b)])?);
                s
            }
            0x41 => {
                let t8 = ctx.town_name(usize::from(w(letter, 8))).ok_or("ciudad ausente")?;
                let (c, e) = (u32::from(w(letter, 0xc)), u32::from(w(letter, 0xe)));
                let (f, fam) = (first(c)?, family(e)?);
                    let mut s = sprintf(self.slot(0x6b74ec)?, &[Arg::S(&t8)])?;
                    s.extend(sprintf(self.slot(0x6b74ec + 4 * if c > 0x7f { 1 } else { 2 })?, &[Arg::S(&f), Arg::S(&fam)])?);
                    s.extend_from_slice(self.slot(0x6b74f8)?);
                    s.extend(bankc(u32::from(w(letter, 0xa)))?);
                    s.extend_from_slice(self.slot(0x6b74fc)?);
                    s.extend_from_slice(&t8);
                    s.extend_from_slice(self.slot(0x6b7500)?);
                    s.extend_from_slice(&t8);
                    s.extend_from_slice(self.slot(0x6b7504)?);
                    s.extend_from_slice(&t5);
                    s
            }
            0x65 => {
                let m = ctx.player().ok_or("comerciante local ausente")?;
                let (mf, mfam) = (first(m.first_id())?, family(u32::from(m.family))?);
                let goods = ctx.goods_text([letter[8], letter[9], letter[0xa], letter[0xb], letter[0xc]]).ok_or("texto de mercancías (0x636620) no disponible")?;
                sprintf(
                    self.slot(0x6a99f8)?,
                    &[Arg::S(self.slot(if m.flags != 0 { 0x6a99fc } else { 0x6a9a00 })?), Arg::S(&mf), Arg::S(&mfam), Arg::U(u32::from(letter[0xd])), Arg::S(self.slot(0x6c0d00 + 4 * u32::from(letter[0xe]))?), Arg::S(&goods)],
                )?
            }
            0x46 | 0x6e => {
                let handle = ctx.map_image_handle().ok_or("objeto de imagen de mapa (0x51AC60) no disponible")?;
                if kind == 0x6e {
                    // el original conserva la ciudad del encabezado en un local
                }
                sprintf(self.cstr(if kind == 0x46 { 0x6b9d1c } else { 0x6b9d84 })?, &[Arg::U(handle)])?
            }
            0x27 => {
                let m = ctx.merchant(u32::from(w(letter, 8))).ok_or("comerciante ausente")?;
                let (mf, mfam) = (first(m.first_id())?, family(u32::from(m.family))?);
                let amount = i32::from_le_bytes(letter[0xc..0x10].try_into().unwrap());
                let wa = u32::from(w(letter, 0xa));
                let pct = (wa as i32).wrapping_mul(amount).wrapping_div(400);
                if amount >= 0 {
                    let mut s = sprintf(self.slot(0x6b5a84)?, &[Arg::S(&mf), Arg::S(&mfam), Arg::I(amount)])?;
                    s.extend(sprintf(self.slot(0x6b5a88)?, &[Arg::I(pct), Arg::U(wa)])?);
                    s
                } else {
                    let mut s = sprintf(self.slot(0x6b5a8c)?, &[Arg::S(&mf), Arg::S(&mfam), Arg::I(amount.wrapping_neg())])?;
                    s.extend(sprintf(self.slot(0x6b5a90)?, &[Arg::U(wa)])?);
                    s
                }
            }
            0x22 => {
                let m = ctx.player().ok_or("comerciante local ausente")?;
                let (mf, mfam) = (first(m.first_id())?, family(u32::from(m.family))?);
                let n = ctx.town_count() as u32;
                if n == 0 {
                    return Err("sin ciudades".into());
                }
                let town = ctx.town_name((u32::from(letter[0xb]) % n) as usize).ok_or("ciudad ausente")?;
                let (fc, fd) = (first(u32::from(letter[0xc]))?, family(u32::from(letter[0xd]))?);
                let year = u32::from(letter[0xe]) + 0x88b8;
                let mut s = if m.flags != 0 {
                    sprintf(self.slot(0x6b5420)?, &[Arg::S(&mfam), Arg::S(&fc), Arg::S(&fd), Arg::S(&town), Arg::U(year)])?
                } else {
                    sprintf(self.slot(0x6b5424)?, &[Arg::S(&mf), Arg::S(&mfam), Arg::S(&fc), Arg::S(&fd), Arg::S(&town), Arg::U(year)])?
                };
                let mut month = u32::from(letter[1] & 0xf);
                let mut day = u32::from(letter[0]) + 0xe;
                let dim = u32::from(u16::from_le_bytes(self.at(0x672d94 + 2 * month).and_then(|b| b.get(..2)).ok_or("tabla de días fuera de rango")?.try_into().unwrap()));
                if day as i32 > dim as i32 {
                    day -= dim;
                    month += 1;
                }
                let y = u32::from(w(letter, 2));
                s.extend(if month < 12 {
                    sprintf(self.cstr(0x6b9cdc)?, &[Arg::U(day), Arg::U(month + 1), Arg::U(y)])?
                } else {
                    sprintf(self.cstr(0x6b9ce8)?, &[Arg::U(day), Arg::U(month.wrapping_sub(11)), Arg::U(y + 1)])?
                });
                let (a, b) = (first(u32::from(letter[8]))?, family(u32::from(letter[9]))?);
                s.extend(sprintf(self.slot(0x6b5428)?, &[Arg::S(&a), Arg::S(&b)])?);
                let months = u32::from(u16::from_le_bytes(self.at(0x672d78 + 2 * u32::from(letter[1] & 0xf)).and_then(|b| b.get(..2)).ok_or("tabla de meses fuera de rango")?.try_into().unwrap()));
                let due = (months + u32::from(w(letter, 2)) * 365 + u32::from(letter[0xf]) + u32::from(letter[0])) << 8;
                if due > ctx.global(0x6de4b4) {
                    effects.push(Effect::Win(0xb0, due.to_le_bytes().to_vec()));
                    effects.push(Effect::Local(0x16, 1u16.to_le_bytes().to_vec()));
                    effects.push(Effect::Buttons(vec![self.u32(0x6b52a0)?]));
                }
                s
            }
            0x6a => {
                let m = ctx.player().ok_or("comerciante local ausente")?;
                let mut s = self.slot(0x6b612c)?.to_vec();
                s.extend(first(m.first_id())?);
                s.push(b' ');
                s.extend(family(u32::from(m.family))?);
                s.extend_from_slice(self.slot(0x6b6130)?);
                s.extend_from_slice(self.slot(0x6aa18c)?);
                s.extend(ctx.town_name(usize::from(letter[0xa])).ok_or("ciudad ausente")?);
                s.extend_from_slice(self.slot(0x6aa190)?);
                let (a, b, c) = (first(u32::from(letter[8]))?, family(u32::from(letter[9] & 0x7f))?, bankc(u32::from(letter[0xb]))?);
                s.extend(sprintf(self.slot(0x6b6140)?, &[Arg::U(u32::from_le_bytes(letter[0xc..0x10].try_into().unwrap())), Arg::S(&a), Arg::S(&b), Arg::S(&c)])?);
                s
            }
            0x85 => {
                let m = ctx.player().ok_or("comerciante local ausente")?;
                let (mf, mfam) = (first(m.first_id())?, family(u32::from(m.family))?);
                sprintf(
                    self.slot(0x6a9a08 + 4 * u32::from(w(letter, 8)))?,
                    &[Arg::S(self.slot(if m.flags != 0 { 0x6a99fc } else { 0x6a9a00 })?), Arg::S(&mf), Arg::S(&mfam), Arg::U(u32::from(w(letter, 0xa)))],
                )?
            }
            0x4b => {
                let mut s = first(u32::from(letter[0xd]))?;
                s.push(b' ');
                s.extend(family(u32::from(letter[0xe]))?);
                let c = u32::from(letter[0xc]);
                s.extend_from_slice(self.slot(match c {
                    0..=7 => 0x6b8080 + 4 * c,
                    8 => 0x6ae634,
                    _ => 0x6ae638,
                })?);
                let wf = usize::from(letter[0xf]);
                match c {
                    0 => {
                        s.extend(ctx.town_text3(wf).ok_or("texto de ciudad ausente")?);
                        s.extend_from_slice(self.slot(0x6b80a0)?);
                        s.extend(ctx.town_name(wf).ok_or("ciudad ausente")?);
                        s.extend_from_slice(self.slot(0x6b80a4)?);
                    }
                    1 => {
                        s.extend(ctx.town_name(wf).ok_or("ciudad ausente")?);
                        s.extend_from_slice(self.slot(0x6b80a8)?);
                    }
                    3 => {
                        s.extend(first(u32::from(letter[9]))?);
                        s.push(b' ');
                        s.extend(family(wf as u32)?);
                        s.extend_from_slice(self.slot(0x6b80ac)?);
                    }
                    5..=7 => s.extend_from_slice(self.slot(0x6b80b0)?),
                    _ => {}
                }
                match letter[8] {
                    0 => {
                        s.extend_from_slice(self.slot(0x6b80b4)?);
                        s.extend(sprintf(self.slot(0x6b80b8)?, &[Arg::U(u32::from(w(letter, 0xa)) * 500)])?);
                    }
                    1 => {
                        s.extend_from_slice(self.slot(0x6ae63c)?);
                        let h = u32::from(letter[0xa]);
                        let flag = *self.at(0x672c14 + h).and_then(|b| b.first()).ok_or("tabla de horas fuera de rango")?;
                        let tail = self.cstr(if flag != 0 { 0x6b9d2c } else { 0x6b9d30 })?;
                        s.extend(sprintf(self.slot(0x6ae640)?, &[Arg::U(u32::from(letter[0xb])), Arg::S(tail), Arg::S(self.slot(0x6c0bb0 + 4 * h)?)])?);
                    }
                    2 => {
                        s.extend_from_slice(self.slot(0x6ae63c)?);
                        s.extend(sprintf(self.slot(0x6ae644)?, &[Arg::S(&ctx.town_name(usize::from(letter[0xa])).ok_or("ciudad ausente")?), Arg::U(u32::from(letter[0xb]))])?);
                    }
                    _ => return Ok(None),
                }
                s
            }
            0x4a => {
                let mut s = b"\\l".to_vec();
                s.extend_from_slice(self.slot(0x6c3054 - 4 * u32::from(w(letter, 8) >> 15))?);
                s.extend(first(u32::from(letter[9]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[8]))?);
                let e = u32::from(w(letter, 0xe));
                let wc = usize::from(w(letter, 0xc));
                let town_c = || ctx.town_name(wc).ok_or("ciudad ausente");
                match e {
                    0 => {
                        s.extend_from_slice(self.slot(0x6b7cbc)?);
                        s.extend(ctx.town_text3(wc).ok_or("texto de ciudad ausente")?);
                        s.extend_from_slice(self.slot(0x6b7cdc)?);
                        s.extend(town_c()?);
                        s.extend_from_slice(self.slot(0x6b7ce0)?);
                    }
                    1 => {
                        s.extend_from_slice(self.slot(0x6b7cc0)?);
                        s.extend(town_c()?);
                        s.extend_from_slice(self.slot(0x6b7ce4)?);
                    }
                    2 => s.extend_from_slice(self.slot(0x6b7cc4)?),
                    3 => {
                        s.extend_from_slice(self.slot(0x6b7cc8)?);
                        s.extend(first(u32::from(letter[0xb]))?);
                        s.push(b' ');
                        s.extend(family(u32::from(letter[0xa]))?);
                        s.extend_from_slice(self.slot(0x6b7ce8)?);
                    }
                    4..=7 => s.extend_from_slice(self.slot(0x6b7cbc + 4 * e)?),
                    8 => s.extend_from_slice(self.slot(0x6ae648)?),
                    _ => s.extend_from_slice(self.slot(0x6b7cc4)?),
                }
                s.extend_from_slice(self.slot(if (5..=7).contains(&e) { 0x6b7cf0 } else { 0x6b7cec })?);
                s
            }
            0x6b => {
                let mut s = self.slot(0x6ab238)?.to_vec();
                let (a, c, e) = (letter[0xd], letter[0xc], w(letter, 0xe));
                if c < a {
                    s.extend(match e {
                        0 => sprintf(self.slot(0x6ab24c)?, &[Arg::U(u32::from_le_bytes(letter[8..12].try_into().unwrap()))])?,
                        1 => sprintf(self.slot(0x6ab23c)?, &[])?,
                        2 => sprintf(self.slot(0x6ab240)?, &[])?,
                        3 => sprintf(self.slot(if letter[8] > letter[9] { 0x6ab244 } else { 0x6ab248 })?, &[Arg::U(u32::from(letter[8]))])?,
                        _ => return Ok(None),
                    });
                } else {
                    s.extend_from_slice(self.slot(0x6ab250 + 4 * u32::from(e))?);
                }
                s.extend(sprintf(self.slot(0x6ab260)?, &[Arg::S(&t5)])?);
                s
            }
            0x5d => {
                mark(&mut text);
                let mut s = self.slot(0x6b9008)?.to_vec();
                s.extend_from_slice(&t5);
                let (f, fam, bank) = (first(u32::from(letter[9]))?, family(u32::from(letter[8]))?, bankc(u32::from(w(letter, 0xa)))?);
                s.extend(sprintf(
                    self.slot(0x6b900c)?,
                    &[Arg::U(u32::from(w(letter, 0xe)) * 10), Arg::S(self.slot(0x6a171c + 4 * u32::from(letter[0xc] & 3))?), Arg::S(&bank), Arg::S(&f), Arg::S(&fam)],
                )?);
                s
            }
            0x5e => {
                ui_float = Some(0x3f40_0000);
                let mut s = self.slot(0x6b9088)?.to_vec();
                s.extend_from_slice(&t5);
                s.extend_from_slice(self.slot(0x6b908c)?);
                s
            }
            0x4c => {
                let base = if letter[9] > 0x7f { 0x6a0608 } else { 0x6a0628 };
                let mut s = self.slot(base + 4 * u32::from(letter[8] & 7))?.to_vec();
                s.push(b' ');
                s.extend(first(u32::from(letter[9]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xa]))?);
                s.extend_from_slice(b"\n\n");
                let tc = ctx.town_name(usize::from(letter[0xc])).ok_or("ciudad ausente")?;
                match letter[0xb] {
                    0 => s.extend(sprintf(self.slot(0x6ac7c8)?, &[Arg::S(&tc), Arg::S(&tc)])?),
                    1 => s.extend(sprintf(self.slot(0x6add4c)?, &[Arg::S(&tc), Arg::S(&tc)])?),
                    2 => {
                        s.extend(sprintf(self.slot(if letter[0xd] > 1 { 0x6ac7cc } else { 0x6ac7d0 })?, &[Arg::S(&tc)])?);
                        s.extend_from_slice(self.slot(0x6ac7d4)?);
                    }
                    _ => return Ok(None),
                }
                s
            }
            0x5b => {
                let mut s = self.slot(if letter[0xf] & 0x80 != 0 { 0x6b8e5c } else { 0x6b8e58 })?.to_vec();
                s.extend(first(u32::from(letter[0xf]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xe]))?);
                s.extend_from_slice(self.slot(0x6b8e60)?);
                let h = u32::from(w(letter, 0xa));
                let flag = *self.at(0x672c14 + h).and_then(|b| b.first()).ok_or("tabla de horas fuera de rango")?;
                let hour = self.slot(0x6c0bb0 + 4 * h)?;
                s.extend(sprintf(self.cstr(if flag != 0 { 0x6b9d60 } else { 0x6b9d68 })?, &[Arg::U(u32::from(w(letter, 8))), Arg::S(hour)])?);
                s.extend_from_slice(self.slot(0x6b8e64)?);
                s.extend_from_slice(&t5);
                s
            }
            0x5c => {
                let w8 = w(letter, 8);
                let (f, fam) = (first(u32::from(w8))?, family(u32::from(w(letter, 0xa)))?);
                let mut s = sprintf(self.slot(if w8 > 0x7f { 0x6b8fbc } else { 0x6b8fb8 })?, &[Arg::S(&f), Arg::S(&fam)])?;
                let c = usize::from(w(letter, 0xc));
                s.extend(if c < ctx.town_count() {
                    sprintf(self.slot(0x6b8fc0)?, &[Arg::S(&ctx.town_name(c).ok_or("ciudad ausente")?)])?
                } else {
                    sprintf(self.slot(0x6b8fc4)?, &[Arg::S(&ctx.town_name(c & 0xff).ok_or("ciudad ausente")?)])?
                });
                if w(letter, 0xe) != 0 {
                    s.extend_from_slice(self.slot(0x6b8fc8)?);
                }
                s
            }
            0x49 => {
                let w8 = w(letter, 8);
                let mut s = self.slot(0x6b77cc)?.to_vec();
                s.extend_from_slice(self.slot(if w8 > 0x7f { 0x6b77d4 } else { 0x6b77d0 })?);
                s.extend(first(u32::from(w8))?);
                s.push(b' ');
                s.extend(family(u32::from(w(letter, 0xa)))?);
                s.extend_from_slice(self.slot(0x6b77d8)?);
                s.extend(sprintf(self.cstr(0x6b9d28)?, &[Arg::U(u32::from_le_bytes(letter[0xc..0x10].try_into().unwrap()))])?);
                s.extend_from_slice(self.slot(0x6b77dc)?);
                s.extend_from_slice(&t5);
                s
            }
            0x52 => {
                let mut s = self.slot(0x6b8748 + 4 * u32::from(letter[8] & 1))?.to_vec();
                s.extend(first(u32::from(letter[0xf]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xe]))?);
                if letter[0xa] & 1 != 0 {
                    s.extend_from_slice(self.slot(0x6b8750)?);
                    s.extend(bankc(u32::from(w(letter, 0xc)))?);
                    s.extend_from_slice(self.slot(0x6b8754)?);
                } else {
                    s.extend_from_slice(self.slot(0x6b8758)?);
                }
                let (a, b, c) = (first(u32::from(w(letter, 8)))?, family(u32::from(w(letter, 0xa)))?, bankc(u32::from(w(letter, 0xc)))?);
                s.extend(sprintf(self.slot(0x6b875c)?, &[Arg::S(&a), Arg::S(&b), Arg::S(&c)])?);
                s
            }
            0x51 => {
                let mut s = self.slot(0x6b845c)?.to_vec();
                s.extend(first(u32::from(w(letter, 0xc)))?);
                s.push(b' ');
                s.extend(family(u32::from(w(letter, 0xe)))?);
                s.extend_from_slice(self.slot(0x6b8460)?);
                s.extend(ctx.town_text3(usize::from(letter[5])).ok_or("texto de ciudad ausente")?);
                s.extend_from_slice(self.slot(0x6b8464)?);
                s.extend_from_slice(&t5);
                s.extend_from_slice(self.slot(0x6b8468)?);
                let (a, b) = (first(u32::from(w(letter, 8)))?, family(u32::from(w(letter, 0xa)))?);
                s.extend(sprintf(self.cstr(0x6b9d44)?, &[Arg::S(&a), Arg::S(&b)])?);
                s
            }
            0x6c => {
                let long = u32::from_le_bytes(letter[8..12].try_into().unwrap());
                let amount = u32::from_le_bytes(letter[0xc..0x10].try_into().unwrap());
                let mut s = self.slot(0x6ab32c)?.to_vec();
                s.extend_from_slice(&t5);
                s.extend_from_slice(self.slot(0x6ab330)?);
                s.extend(sprintf(self.cstr(0x6b9d7c)?, &[Arg::U(amount)])?);
                s.extend_from_slice(self.slot(0x6ab334)?);
                s.extend(sprintf(self.cstr(0x6b9d80)?, &[Arg::U(long & 0x7fff_ffff)])?);
                s.extend_from_slice(self.slot(0x6ab338)?);
                if long & 0x8000_0000 != 0 {
                    s.extend_from_slice(self.slot(0x6ab33c)?);
                }
                s
            }
            0x1a => {
                let x = u32::from(w(letter, 0xe)) >> 5;
                let (month, year) = (x % 12, u32::from(w(letter, 2)) + x / 12);
                let h = u32::from(w(letter, 0xa)) % 24;
                let flag = *self.at(0x672c14 + h).and_then(|b| b.first()).ok_or("tabla de horas fuera de rango")?;
                let alt = ctx.town_text3(usize::from(letter[5])).ok_or("texto de ciudad ausente")?;
                let (f, fam) = (first(u32::from(letter[9]))?, family(u32::from(letter[8]))?);
                sprintf(
                    self.slot(if flag != 0 { 0x6b4bb0 } else { 0x6b4bb4 })?,
                    &[Arg::U(month + 1), Arg::U(u32::from(letter[0xe] & 0x1f)), Arg::U(year), Arg::S(&f), Arg::S(&fam), Arg::S(&alt), Arg::S(&t5), Arg::U(u32::from(w(letter, 0xc))), Arg::S(self.slot(0x6c0bb0 + 4 * h)?)],
                )?
            }
            0x19 => {
                let sub = u32::from(letter[0xa]);
                let town_b = || ctx.town_name(usize::from(letter[0xb])).ok_or("ciudad ausente");
                let mut s = first(u32::from(letter[8]))?;
                s.push(b' ');
                s.extend(family(u32::from(letter[9]))?);
                s.extend_from_slice(self.slot(if sub < 8 { 0x6b48b8 + 12 * sub } else { 0x6ae5c8 + 12 * (sub - 8) })?);
                let bank = || bankc(u32::from(w(letter, 0xc)));
                let piece = match sub {
                    0 => {
                        let alt = ctx.town_text3(usize::from(letter[0xb])).ok_or("texto de ciudad ausente")?;
                        sprintf(self.slot(0x6b48bc)?, &[Arg::S(&alt), Arg::S(&town_b()?)])?
                    }
                    1 => sprintf(self.slot(0x6b48c8)?, &[Arg::S(&town_b()?)])?,
                    2 => sprintf(self.slot(0x6b48d4)?, &[Arg::S(&bank()?)])?,
                    3 => sprintf(self.slot(0x6b48e0)?, &[Arg::S(&town_b()?), Arg::S(&first(u32::from(letter[0xc]))?), Arg::S(&family(u32::from(letter[0xd]))?)])?,
                    4 => sprintf(self.slot(0x6b48ec)?, &[])?,
                    8 => sprintf(self.slot(0x6ae5cc)?, &[])?,
                    9 => sprintf(self.slot(0x6ae5d8)?, &[Arg::S(&bank()?)])?,
                    10 => sprintf(self.slot(0x6ae5e4)?, &[Arg::S(&bank()?)])?,
                    11 => sprintf(self.slot(0x6ae5f0)?, &[Arg::S(&bank()?)])?,
                    12 => sprintf(self.slot(0x6ae5fc)?, &[Arg::S(&bank()?)])?,
                    13 => sprintf(self.slot(0x6ae608)?, &[Arg::S(&town_b()?), Arg::S(&bank()?)])?,
                    14 => sprintf(self.slot(0x6ae614)?, &[Arg::S(&town_b()?), Arg::S(&bank()?)])?,
                    15 => sprintf(self.slot(0x6ae620)?, &[Arg::S(&town_b()?), Arg::S(&bank()?)])?,
                    _ => return Ok(None),
                };
                s.extend(piece);
                s.extend_from_slice(self.slot(if sub < 8 { 0x6b48c0 + 12 * sub } else { 0x6ae570 + 12 * sub })?);
                s.extend_from_slice(self.slot(0x6b48f4)?);
                s
            }
            0x15 => {
                let (a, c, e) = (w(letter, 0xa), w(letter, 0xc), w(letter, 0xe));
                let (f, fam) = (first(u32::from(letter[9]))?, family(u32::from(letter[8]))?);
                if a != 0 && c != 0 {
                    let mut s = if e == 0 {
                        sprintf(self.slot(0x6b4180)?, &[Arg::S(&f), Arg::S(&fam), Arg::U(u32::from(c))])?
                    } else {
                        sprintf(self.slot(0x6b4184)?, &[Arg::S(&f), Arg::S(&fam), Arg::U(u32::from(e)), Arg::U(u32::from(c))])?
                    };
                    for k in 0..16u32 {
                        if a & (1 << k) != 0 {
                            s.push(b' ');
                            s.extend_from_slice(self.slot(0x6c0bb0 + 4 * k)?);
                            s.push(b',');
                        }
                    }
                    s.pop();
                    s
                } else if e == 0 {
                    sprintf(self.slot(0x6b4178)?, &[Arg::S(&f), Arg::S(&fam)])?
                } else {
                    sprintf(self.slot(0x6b417c)?, &[Arg::S(&f), Arg::S(&fam), Arg::U(u32::from(e))])?
                }
            }
            0x0b => {
                if text.len() > 5 {
                    text[5] = b'3';
                }
                let w8 = usize::from(w(letter, 8));
                let town8 = ctx.town_name(w8);
                let mut s = if w8 < ctx.town_count() {
                    let t8 = town8.clone().ok_or("ciudad ausente")?;
                    sprintf(self.slot(0x6b70f4)?, &[Arg::S(&t5), Arg::S(&t8), Arg::S(self.slot(0x6add54 + 4 * u32::from(letter[0xa]))?), Arg::S(&t5)])?
                } else {
                    sprintf(self.slot(0x6adddc)?, &[Arg::S(&t5), Arg::S(&t5)])?
                };
                let (a, c, e) = (u32::from(w(letter, 0xa)), u32::from(w(letter, 0xc)), u32::from(w(letter, 0xe)));
                let (idx, x, y) = if c <= e { ((a >> 7) & 0xfe, e, c) } else { (((a >> 7) & 0xff) | 1, c, e) };
                let tmpl = self.slot(0x6add10 + 4 * idx)?;
                s.extend(if idx < 2 {
                    sprintf(tmpl, &[Arg::U(x), Arg::U(y)])?
                } else {
                    sprintf(tmpl, &[Arg::S(&town8.ok_or("ciudad ausente")?), Arg::U(x), Arg::U(y)])?
                });
                s
            }
            0x0a => {
                let mut s = first(u32::from(w(letter, 8)))?;
                s.push(b' ');
                s.extend(family(u32::from(w(letter, 0xa)))?);
                s.extend_from_slice(self.slot(0x6b4a18)?);
                let kind_slot = if w(letter, 0xc) == 8 { 0x6ae5c8 } else { 0x6b4a0c + 4 * u32::from(w(letter, 0xc)) };
                s.extend_from_slice(self.slot(kind_slot)?);
                s.extend_from_slice(self.slot(0x6b4a1c)?);
                s
            }
            0x13 => {
                let amount = u32::from_le_bytes(letter[0xc..0x10].try_into().unwrap());
                let mut s = match letter[0xa] {
                    0 => sprintf(self.slot(0x6b3d70)?, &[Arg::S(&t5), Arg::U(amount)])?,
                    1 => sprintf(self.slot(0x6b3d74)?, &[Arg::S(&t5), Arg::U(amount)])?,
                    2 => sprintf(self.slot(0x6b3d78)?, &[Arg::S(&t5), Arg::S(&bankc(u32::from(w(letter, 8)))?), Arg::U(amount)])?,
                    _ => return Ok(None),
                };
                if letter[0xb] == 0 {
                    s.extend_from_slice(self.slot(0x6b3d7c)?);
                }
                s
            }
            0x28 => sprintf(self.slot(0x6b5b10)?, &[Arg::U(u32::from(w(letter, 8))), Arg::U(u32::from(w(letter, 0xa)))])?,
            0x34 => {
                mark(&mut text);
                let (year, month, day) = self.ticks_to_date(u32::from_le_bytes(letter[0xc..0x10].try_into().unwrap()))?;
                let bank = bankc(u32::from(w(letter, 8)))?;
                let hull = self.slot(0x6c0e38 + 4 * u32::from(letter[0xa] & 3))?;
                sprintf(self.slot(0x6b6444)?, &[Arg::S(hull), Arg::S(&bank), Arg::U(u32::from(day)), Arg::S(self.slot(0x6c0d9c + 4 * u32::from(month))?), Arg::U(u32::from(year))])?
            }
            0x2a => {
                let sex = u32::from(letter[8] != 0);
                let tmpl = match letter[0xf] {
                    0 => 0x6c14c4,
                    0xff => 0x6aa184,
                    1 => 0x6c14c8,
                    _ => 0x6c14cc,
                };
                let (fb, fd) = (first(u32::from(letter[0xb]))?, first(u32::from(letter[0xd]))?);
                let (fc, fe) = (family(u32::from(letter[0xc]))?, family(u32::from(letter[0xe]))?);
                let (a, b, c) = (
                    self.slot(0x6c0ef8 + 4 * sex)?,
                    self.slot(0x69ee48 + 4 * u32::from(letter[9]))?,
                    self.slot(0x6a0608 + 4 * (u32::from(letter[0xa]) + 8 * sex))?,
                );
                let mut args = vec![Arg::S(a), Arg::S(b), Arg::S(c), Arg::S(&fb), Arg::S(&fc), Arg::S(&fd), Arg::S(&fe)];
                if !matches!(letter[0xf], 0 | 0xff | 1) {
                    args.push(Arg::U(u32::from(letter[0xf])));
                }
                sprintf(self.slot(tmpl)?, &args)?
            }
            0x2c | 0x2d | 0x2e => {
                let pl = ctx.player().ok_or("comerciante local ausente")?;
                let mut s = self.slot(0x6b5e80)?.to_vec();
                s.extend_from_slice(self.slot(if pl.flags & 1 != 0 { 0x6b5e84 } else { 0x6b5e88 })?);
                s.extend(first(u32::from(pl.first))?);
                s.push(b' ');
                s.extend(family(u32::from(pl.family))?);
                match kind {
                    0x2c => {
                        s.extend_from_slice(self.slot(0x6b5e8c)?);
                        s.extend_from_slice(&t5);
                        s.extend_from_slice(self.slot(0x6b5e90)?);
                        s.extend(bankc(u32::from(w(letter, 0xa)))?);
                        s.extend_from_slice(self.slot(0x6b5e94)?);
                        s.extend(sprintf(self.cstr(0x6b9cf4)?, &[Arg::U(u32::from(w(letter, 0xc))), Arg::U(pl.index)])?);
                        s.extend_from_slice(self.slot(0x6b5e98)?);
                        s.extend(first(u32::from(letter[8]))?);
                        s.push(b' ');
                        s.extend(family(u32::from(letter[9]))?);
                        s.extend_from_slice(self.slot(0x6b5e9c)?);
                        s.extend(sprintf(self.cstr(0x6b9cf8)?, &[Arg::U(u32::from(w(letter, 0xe)))])?);
                        s.extend_from_slice(self.slot(0x6b5ea0)?);
                    }
                    0x2d => {
                        s.extend_from_slice(self.slot(0x6b5eb0)?);
                        s.extend(bankc(u32::from(w(letter, 0xa)))?);
                        s.extend_from_slice(self.slot(0x6b5eb4)?);
                        s.extend(sprintf(self.cstr(0x6b9cfc)?, &[Arg::U(u32::from(w(letter, 0xc)) + u32::from(w(letter, 0xe))), Arg::U(pl.index)])?);
                        s.extend_from_slice(self.slot(0x6b5eb8)?);
                        s.extend(sprintf(self.cstr(0x6b9d00)?, &[Arg::U(u32::from(w(letter, 0xe)))])?);
                        s.extend_from_slice(self.slot(0x6b5ebc)?);
                    }
                    _ => {
                        s.extend_from_slice(self.slot(0x6b5ea4)?);
                        s.extend(bankc(u32::from(w(letter, 0xc)))?);
                        s.extend_from_slice(self.slot(0x6b5ea8)?);
                        s.extend_from_slice(&t5);
                        s.extend_from_slice(self.slot(0x6b5eac)?);
                    }
                }
                s
            }
            0x2f => {
                let mut s = self.slot(0x6b6064)?.to_vec();
                let tmpl = if w(letter, 8) & 0x8000 == 0x8000 { 0x6b606c } else { 0x6b6068 };
                let (a, b) = (first(u32::from(letter[9]))?, family(u32::from(w(letter, 8) & 0xff))?);
                s.extend(sprintf(self.slot(tmpl)?, &[Arg::S(&a), Arg::S(&b)])?);
                s.extend_from_slice(self.slot(0x6b6070)?);
                s.extend(first(u32::from(letter[0xb]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xa]))?);
                s.extend(sprintf(self.slot(0x6b6074)?, &[Arg::U(u32::from(w(letter, 0xc)).wrapping_mul(100)), Arg::S(&t5)])?);
                s
            }
            0x29 | 0x47 | 0x63 | 0x83 => {
                if kind == 0x63 {
                    ui_float = Some(0x3f59_999a);
                }
                if kind == 0x29 || kind == 0x63 {
                    mark(&mut text);
                }
                let (ptr, len) = (u32::from_le_bytes(letter[8..12].try_into().unwrap()), u32::from_le_bytes(letter[12..16].try_into().unwrap()));
                if len == 0 {
                    Vec::new()
                } else {
                    ctx.memory(ptr, len).ok_or("cuerpo de texto de la carta no disponible")?
                }
            }
            0x1b => {
                let hull = self.slot(0x6c0e38 + 4 * u32::from(letter[8] & 3))?.to_vec();
                let tmpl = if letter[8] & 1 == 0 { (0x6b4d2c, 0x6b4d28) } else { (0x6b4d34, 0x6b4d30) };
                let ship = bankc(u32::from(w(letter, 0xc) & 0x3fff))?;
                let sailors = u32::from(letter[0xf]);
                let s = if letter[0xe] < 0xff {
                    let fam = family(u32::from(letter[0xe]))?;
                    let fst = first(u32::from(w(letter, 0xa) >> 11))?;
                    sprintf(self.slot(tmpl.0)?, &[Arg::S(&hull), Arg::S(&ship), Arg::U(sailors), Arg::S(&fst), Arg::S(&fam)])?
                } else {
                    sprintf(self.slot(tmpl.1)?, &[Arg::S(&hull), Arg::S(&ship), Arg::U(sailors)])?
                };
                if ctx.global(0x6cbdd8) != 0 && ctx.global(0x4b2200) & 0xff != 0 && w(letter, 8) > 3 {
                    effects.push(Effect::Call(0x44f490, vec![u32::from(w(letter, 8) >> 2), u32::from(w(letter, 0xa) & 0x7ff)]));
                }
                s
            }
            0x1e | 0x1f => {
                let count = ctx.global(0x6de4aa) & 0xffff;
                let mut s = Vec::new();
                if u32::from(w(letter, 0xe)) < count {
                    let m = ctx.merchant(u32::from(w(letter, 0xe))).ok_or("comerciante ausente")?;
                    let (f, fam) = (first(m.first_id())?, family(u32::from(m.family))?);
                    s.extend(sprintf(self.slot(0x6b4f78 + 4 * u32::from(m.flags))?, &[Arg::S(&f), Arg::S(&fam), Arg::C(10), Arg::C(10), Arg::C(10)])?);
                }
                if ctx.global(0x6de584) != 0 {
                    let mut key = [0u8; 8];
                    key.copy_from_slice(&letter[8..16]);
                    if let Some(t) = ctx.offer_text(key) {
                        s.extend(t);
                    }
                }
                let mut sig = Vec::new();
                if u32::from(w(letter, 0xc)) < count {
                    s.extend_from_slice(b"\n\n\\c");
                    let m = ctx.merchant(u32::from(w(letter, 0xc))).ok_or("comerciante ausente")?;
                    let (f, fam) = (first(m.first_id())?, family(u32::from(m.family))?);
                    sig = sprintf(self.slot(0x6b4f9c)?, &[Arg::C(10), Arg::C(0x5c), Arg::C(0x72), Arg::S(&f), Arg::S(&fam)])?;
                    effects.push(Effect::Local(0x1c, vec![letter[0xc]]));
                }
                let slots: Vec<u32> = if kind == 0x1e {
                    effects.push(Effect::Local(0x16, 1u16.to_le_bytes().to_vec()));
                    vec![self.u32(0x6b503c)?]
                } else {
                    effects.push(Effect::Local(0x16, 2u16.to_le_bytes().to_vec()));
                    effects.push(Effect::Local(0x1c, vec![letter[0xc]]));
                    vec![self.u32(0x6b5028)?, self.u32(0x6b502c)?]
                };
                effects.push(Effect::Buttons(slots));
                s.extend(sig);
                s
            }
            0x21 => {
                let m = ctx.player().ok_or("comerciante local ausente")?;
                let (f, fam) = (first(m.first_id())?, family(u32::from(m.family))?);
                let amount = u32::from(w(letter, 0xc));
                let mut s = if m.flags != 0 {
                    sprintf(self.slot(0x6b5298)?, &[Arg::S(&fam), Arg::U(amount)])?
                } else {
                    sprintf(self.slot(0x6b529c)?, &[Arg::S(&f), Arg::S(&fam), Arg::U(amount)])?
                };
                let (a, b) = (first(u32::from(w(letter, 8)))?, family(u32::from(w(letter, 0xa)))?);
                s.extend(sprintf(self.slot(0x6b52a4)?, &[Arg::S(&a), Arg::S(&b)])?);
                let due = ((u32::from(w(letter, 2)) * 365 + u32::from(w(letter, 0xe))) << 8) as i32;
                if (ctx.global(0x6de4b4) as i32) < due {
                    effects.push(Effect::Win(0xb0, (due + 0x100).to_le_bytes().to_vec()));
                    effects.push(Effect::Local(0x16, 1u16.to_le_bytes().to_vec()));
                    effects.push(Effect::Buttons(vec![self.u32(0x6b52a0)?]));
                } else {
                    effects.push(Effect::Local(0x18, vec![0; 4]));
                }
                s
            }
            0x11 => {
                mark(&mut text);
                ui_float = Some(0x3f40_0000);
                let count = ctx.ledger_count() & 0xffff;
                let head = u32::from(ctx.player_u16(0xa));
                let find = |kind: u8| -> Result<Option<[u8; 16]>, String> {
                    if head >= count {
                        return Ok(None);
                    }
                    let mut cx = head;
                    for _ in 0..0x20000 {
                        let n = ctx.ledger_node(cx).ok_or("nodo del libro mayor ausente")?;
                        if n[4] == kind {
                            return Ok(Some(n));
                        }
                        let next = u32::from(u16::from_le_bytes([n[6], n[7]]));
                        if next >= count {
                            return Ok(None);
                        }
                        cx = next;
                    }
                    Err("cadena del libro mayor cíclica".into())
                };
                let (n37, n36) = (find(0xf)?, find(0x10)?);
                let d = |n: &[u8; 16], o: usize| u32::from_le_bytes(n[o..o + 4].try_into().unwrap());
                let mut s = Vec::new();
                let a8 = u32::from_le_bytes(letter[8..12].try_into().unwrap());
                if a8 != 0 {
                    s.extend(sprintf(self.slot(0x6b39f0)?, &[Arg::U(a8)])?);
                }
                if let Some(n) = n37.filter(|n| d(n, 0xc) != 0) {
                    s.extend(sprintf(self.slot(0x6b38c0)?, &[Arg::U(d(&n, 0xc))])?);
                }
                if let Some(n) = n36.filter(|n| d(n, 0xc) != 0) {
                    s.extend(sprintf(self.slot(0x6b3950)?, &[Arg::U(d(&n, 0xc))])?);
                }
                if let Some(n) = n37.filter(|n| d(n, 8) != 0) {
                    s.extend(sprintf(self.slot(0x6b38bc)?, &[Arg::U(d(&n, 8))])?);
                }
                if let Some(n) = n36.filter(|n| d(n, 8) != 0) {
                    s.extend(sprintf(self.slot(0x6b39f4)?, &[Arg::U(d(&n, 8)), Arg::U(u32::from_le_bytes(letter[12..16].try_into().unwrap()))])?);
                }
                s
            }
            0x25 => {
                if w(letter, 8) > 3 {
                    return Err("casco fuera de rango en la carta 0x25".into());
                }
                let loaded = |this: &Self, odd: bool, amount: u16| sprintf(this.slot(if odd { 0x6b58e0 } else { 0x6b58d4 })?, &[Arg::U(u32::from(amount))]);
                let mut s = sprintf(self.slot(0x6b58d0)?, &[Arg::S(self.hull(u32::from(w(letter, 8)))?)])?;
                let mut tag = 2;
                if w(letter, 0xc) != 0 {
                    s.extend(loaded(self, w(letter, 8) & 1 == 1, w(letter, 0xc))?);
                    tag = 3;
                }
                if w(letter, 0xa) <= 3 {
                    s.extend(sprintf(self.slot(0x6b58d0 + 4 * tag)?, &[Arg::S(self.hull(u32::from(w(letter, 0xa)))?)])?);
                    if w(letter, 0xe) != 0 {
                        s.extend(loaded(self, w(letter, 0xa) & 1 == 1, w(letter, 0xe))?);
                    }
                }
                s.push(b'.');
                s
            }
            0x30 => {
                let m = ctx.player().ok_or("comerciante local ausente")?;
                let mut s = self.slot(0x6b612c)?.to_vec();
                s.extend(first(m.first_id())?);
                s.push(b' ');
                s.extend(family(u32::from(m.family))?);
                s.push(b'?');
                s.extend_from_slice(self.slot(0x6b6130)?);
                s.extend_from_slice(self.hull(u32::from(w(letter, 0xe) & 3))?);
                s.extend_from_slice(self.slot(if letter[0xa] > 0x7f { 0x6b6138 } else { 0x6b6134 })?);
                s.extend(first(u32::from(letter[0xa]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xb]))?);
                s.extend_from_slice(self.slot(if letter[9] & 0x80 != 0 { 0x6aa188 } else { 0x6b613c })?);
                let (a, b, c) = (first(u32::from(letter[8]))?, family(u32::from(letter[9] & 0x7f))?, bankc(u32::from(w(letter, 0xe) >> 2))?);
                s.extend(sprintf(self.slot(0x6b6140)?, &[Arg::U(u32::from(w(letter, 0xc))), Arg::S(&a), Arg::S(&b), Arg::S(&c)])?);
                s
            }
            0x37 => {
                if text.len() > 5 {
                    text[5] = b'3';
                }
                let m = ctx.player().ok_or("comerciante local ausente")?;
                let (f, fam) = (first(m.first_id())?, family(u32::from(m.family))?);
                let mut s = self.slot(0x6b674c)?.to_vec();
                s.extend(sprintf(self.cstr(0x6b9d04)?, &[Arg::S(self.slot(0x6c3048 + 4 * u32::from(m.flags & 1))?), Arg::S(&f), Arg::S(&fam)])?);
                s.extend_from_slice(self.slot(0x6b6750)?);
                s.extend_from_slice(&t5);
                s.extend_from_slice(self.slot(0x6b6754)?);
                s.extend(sprintf(self.cstr(0x6b9d10)?, &[Arg::U(u32::from_le_bytes(letter[8..12].try_into().unwrap()))])?);
                s.extend_from_slice(self.slot(0x6b6758)?);
                s.extend_from_slice(&t5);
                s
            }
            0x3d | 0x3e | 0x3f => {
                let (desc_addr, block) = (u32::from_le_bytes(letter[8..12].try_into().unwrap()), u32::from_le_bytes(letter[12..16].try_into().unwrap()));
                let desc = ctx.memory(desc_addr, 16).ok_or("descriptor de mensaje no disponible")?;
                let cstr_at = |mut a: u32| -> Result<Vec<u8>, String> {
                    let mut v = Vec::new();
                    loop {
                        let b = ctx.memory(a, 1).ok_or("bloque de mensaje no disponible")?[0];
                        if b == 0 {
                            return Ok(v);
                        }
                        v.push(b);
                        a += 1;
                        if v.len() > 0x10000 {
                            return Err("cadena de mensaje demasiado larga".into());
                        }
                    }
                };
                let first_str = cstr_at(block)?;
                let mut cur = block + first_str.len() as u32 + 1;
                let s = cstr_at(cur)?;
                cur += s.len() as u32 + 1;
                let due = u32::from_le_bytes(desc[4..8].try_into().unwrap());
                if ctx.global(0x6de4b4) < due {
                    effects.push(Effect::Win(0xb0, due.to_le_bytes().to_vec()));
                    let count = desc[0xb];
                    if count != 0 {
                        effects.push(Effect::Local(0x16, u16::from(count).to_le_bytes().to_vec()));
                        let mut ptrs = Vec::new();
                        for _ in 0..count {
                            ptrs.push(cur);
                            cur += cstr_at(cur)?.len() as u32 + 1;
                        }
                        effects.push(Effect::Buttons(ptrs));
                    }
                }
                effects.push(Effect::Local(0x1d, vec![desc[0xe]]));
                effects.push(Effect::Local(0x1e, vec![desc[0xf]]));
                effects.push(Effect::Local(0x1f, vec![desc[0xd]]));
                s
            }
            0x5a => {
                let mut s = self.slot(if letter[0xf] & 0x80 != 0 { 0x6b8d4c } else { 0x6b8d48 })?.to_vec();
                s.extend(first(u32::from(letter[0xf]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xe]))?);
                s.extend_from_slice(self.slot(0x6b8d50)?);
                let good = u32::from(w(letter, 0xa));
                let flag = *self.at(0x672c14 + good).and_then(|b| b.first()).ok_or("tabla de horas fuera de rango")?;
                s.extend(sprintf(self.cstr(if flag != 0 { 0x6b9d4c } else { 0x6b9d54 })?, &[Arg::U(u32::from(w(letter, 0xc))), Arg::S(self.slot(0x6c0bb0 + 4 * good)?)])?);
                s.extend_from_slice(self.slot(0x6b8d54)?);
                let bonus = (u32::from(w(letter, 0xc)).wrapping_mul(u32::from(w(letter, 8)))) as i32;
                if bonus > 0 {
                    s.extend_from_slice(self.slot(0x6b8d58)?);
                    s.extend(sprintf(self.cstr(0x6b9d5c)?, &[Arg::U(bonus as u32)])?);
                    s.extend_from_slice(self.slot(0x6b8d5c)?);
                }
                s.extend_from_slice(self.slot(0x6b8d60)?);
                s.extend_from_slice(&t5);
                s
            }
            0x62 => {
                let value = (u32::from(w(letter, 0xa)) << 16).wrapping_add(u32::from(w(letter, 0xc)));
                let mut s = self.slot(0x6b9224)?.to_vec();
                s.extend(sprintf(self.cstr(0x6b9d70)?, &[Arg::U(value)])?);
                s.extend_from_slice(self.slot(0x6b9228)?);
                if w(letter, 8) == 0xffff {
                    s.extend_from_slice(self.slot(0x6b9234)?);
                } else {
                    s.extend(first(u32::from(letter[9]))?);
                    s.push(b' ');
                    s.extend(first(u32::from(letter[8]))?);
                    s.extend_from_slice(self.slot(if letter[9] & 0x80 != 0 { 0x6b9230 } else { 0x6b922c })?);
                }
                s.extend(sprintf(self.cstr(0x6b9d74)?, &[Arg::I(i32::from(w(letter, 0xe)))])?);
                s.extend_from_slice(self.slot(0x6b9238)?);
                let share = u32::from(w(letter, 0xe)).wrapping_mul(value);
                s.extend(sprintf(self.cstr(0x6b9d78)?, &[Arg::U(((u64::from(share) * 0x51eb_851f) >> 32) as u32 >> 7)])?);
                s.extend_from_slice(self.slot(0x6b923c)?);
                s
            }
            0x64 => {
                let m = ctx.player().ok_or("comerciante local ausente")?;
                let (mf, mfam) = (first(m.first_id())?, family(u32::from(m.family))?);
                let goods = ctx.goods_text([letter[8], letter[9], letter[0xa], letter[0xb], letter[0xc]]).ok_or("texto de mercancías (0x636620) no disponible")?;
                let title = self.slot(if m.flags != 0 { 0x6a99fc } else { 0x6a9a00 })?;
                let s = sprintf(
                    self.slot(0x6a99f4)?,
                    &[Arg::S(title), Arg::S(&mf), Arg::S(&mfam), Arg::S(&goods), Arg::U(u32::from(letter[0xd])), Arg::S(self.slot(0x6c0d00 + 4 * u32::from(letter[0xe]))?)],
                )?;
                let months = u32::from(u16::from_le_bytes(self.at(0x672d78 + 2 * u32::from(letter[1] & 0x7f)).and_then(|b| b.get(..2)).ok_or("tabla de meses fuera de rango")?.try_into().unwrap()));
                let due = (months + u32::from(w(letter, 2)) * 365 + u32::from(letter[0]) + 0xd) << 8;
                if letter[0xf] == 0xfe || (ctx.global(0x6de4b4) & !0xff) > due {
                    effects.push(Effect::Local(0x18, vec![0; 4]));
                } else {
                    effects.push(Effect::Win(0xb0, (due + 0x100).to_le_bytes().to_vec()));
                    effects.push(Effect::Local(0x16, 1u16.to_le_bytes().to_vec()));
                    effects.push(Effect::Buttons(vec![self.u32(0x6a9a04)?]));
                }
                s
            }
            0x73 => {
                let mut s = first(u32::from(letter[0xf]))?;
                s.push(b' ');
                s.extend(family(u32::from(letter[0xe]))?);
                s.extend_from_slice(self.slot(0x6abb78)?);
                s.extend_from_slice(&t5);
                s.extend_from_slice(self.slot(0x6abb7c)?);
                let mut count = 0;
                for k in 0..3 {
                    let item = w(letter, 8 + 2 * k);
                    if item >= 0x1800 {
                        continue;
                    }
                    if count == 0 {
                        s.extend_from_slice(self.slot(0x6abb80)?);
                    }
                    count += 1;
                    let good = u32::from(item >> 8);
                    let flag = *self.at(0x672c14 + good).and_then(|b| b.first()).ok_or("tabla de mercancías fuera de rango")?;
                    s.extend(sprintf(self.cstr(0x6b9d9c)?, &[Arg::U(u32::from(item & 0xff)), Arg::S(self.slot(0x6c0dec + 4 * u32::from(flag))?), Arg::S(self.slot(0x6c0bb0 + 4 * good)?)])?);
                }
                s.extend_from_slice(self.slot(if count == 0 { 0x6abb88 } else { 0x6abb84 })?);
                s
            }
            0x74 => {
                let mut s = self.slot((if letter[9] & 0x80 != 0 { 0x6a0608 } else { 0x6a0628 }) + 4 * u32::from(letter[0xa]))?.to_vec();
                s.push(b' ');
                s.extend(first(u32::from(letter[9]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[8]))?);
                s.extend(sprintf(self.slot(0x6abd0c)?, &[Arg::S(&t5)])?);
                s.extend_from_slice(self.slot(if w(letter, 0xa) > 0xff { 0x6abd10 } else { 0x6abd1c })?);
                let young = u32::from(w(letter, 0xc) < 0x8000);
                let (a, b) = (family(u32::from(letter[0xc]))?, first(u32::from(letter[0xd]))?);
                s.extend(sprintf(
                    self.slot(0x6abd14)?,
                    &[
                        Arg::S(self.slot(0x6c0ee8 + 4 * young)?),
                        Arg::S(self.slot(0x69ef2c + 4 * u32::from(letter[0xf]))?),
                        Arg::S(self.slot(0x6a0608 + 4 * (u32::from(letter[0xe]) + 8 * young))?),
                        Arg::S(&b),
                        Arg::S(&a),
                    ],
                )?);
                s.extend_from_slice(self.slot(0x6abd18)?);
                s
            }
            0x75 => {
                let mut s = first(u32::from(letter[0xf]))?;
                s.push(b' ');
                s.extend(family(u32::from(letter[0xe]))?);
                s.extend_from_slice(self.slot(0x6abdb4)?);
                let mut count = 0;
                for k in 0..3 {
                    let item = w(letter, 8 + 2 * k);
                    if item >= 0x1800 {
                        continue;
                    }
                    if count == 0 {
                        s.extend_from_slice(self.slot(0x6abdb8)?);
                    }
                    count += 1;
                    let good = u32::from(item >> 8);
                    let flag = *self.at(0x672c14 + good).and_then(|b| b.first()).ok_or("tabla de mercancías fuera de rango")?;
                    s.extend(sprintf(self.cstr(0x6b9da8)?, &[Arg::U(u32::from(item & 0xff)), Arg::S(self.slot(0x6c0dec + 4 * u32::from(flag))?), Arg::S(self.slot(0x6c0bb0 + 4 * good)?)])?);
                }
                if count == 0 {
                    s.extend_from_slice(self.slot(0x6abdbc)?);
                }
                s
            }
            0x76 | 0x7d | 0x7e | 0x7f => {
                let (a, b) = (first(u32::from(letter[8]))?, family(u32::from(letter[9]))?);
                let mut s = sprintf(self.slot(if letter[8] & 0x80 != 0 { 0x6ae924 } else { 0x6ae920 })?, &[Arg::S(&a), Arg::S(&b)])?;
                let building = self.slot(0x6a57c8 + 4 * u32::from(letter[0xd]))?;
                let amount = u32::from(w(letter, 0xe));
                let (fa, fam) = (first(u32::from(letter[0xa]))?, family(u32::from(letter[0xb]))?);
                s.extend(match kind {
                    0x76 => sprintf(self.slot(0x6ae928)?, &[Arg::S(&t5), Arg::S(building), Arg::U(amount), Arg::S(&fa), Arg::S(&fam), Arg::U(amount)])?,
                    0x7d => sprintf(self.slot(0x6ae930)?, &[Arg::S(&t5), Arg::S(building), Arg::U(amount), Arg::U(amount)])?,
                    0x7e => sprintf(self.slot(0x6ae92c)?, &[Arg::S(building), Arg::S(&t5)])?,
                    _ => sprintf(self.slot(0x6ae934)?, &[Arg::S(&t5), Arg::S(building), Arg::U(amount), Arg::S(&fa), Arg::S(&fam)])?,
                });
                s
            }
            0x77 => {
                let mut s = Vec::new();
                match letter[0xe] {
                    0 if letter[9] >= 5 => {
                        let (a, b) = (first(u32::from(letter[0xc]))?, family(u32::from(letter[0xd]))?);
                        s.extend(sprintf(self.slot(if letter[0xc] & 0x80 != 0 { 0x6addc4 } else { 0x6addc0 })?, &[Arg::S(&a), Arg::S(&b)])?);
                        let town8 = ctx.town_name(usize::from(letter[8])).ok_or("ciudad ausente")?;
                        s.extend(sprintf(self.slot(0x6addb4 + 4 * u32::from(letter[9]))?, &[Arg::S(&town8), Arg::S(&town8), Arg::S(&town8)])?);
                        let (a, b) = (first(u32::from(letter[0xa]))?, family(u32::from(letter[0xb]))?);
                        s.extend(sprintf(self.slot(0x6addd0)?, &[Arg::S(&a), Arg::S(&b)])?);
                    }
                    0 => {
                        if usize::from(letter[8]) >= ctx.town_count() {
                            s.extend_from_slice(self.slot(0x6add90)?);
                            s.extend_from_slice(self.slot(0x6adda8)?);
                        } else {
                            let town8 = ctx.town_name(usize::from(letter[8])).ok_or("ciudad ausente")?;
                            let head = sprintf(self.slot(0x6add84 + 4 * u32::from(letter[9]))?, &[Arg::S(&town8)])?;
                            s.extend_from_slice(&head);
                            // Los casos 0/1/2/4 sobrescriben el búfer temporal; el 3 lo deja y la cadena se copia por segunda vez.
                            let body = match letter[9] {
                                0 => sprintf(self.slot(0x6add98)?, &[Arg::S(&town8), Arg::S(self.slot(0x6c0bb0 + 4 * u32::from(letter[0xa]))?)])?,
                                1 => sprintf(self.slot(0x6add9c)?, &[Arg::S(&town8)])?,
                                2 => sprintf(self.slot(0x6adda0)?, &[Arg::S(&town8), Arg::S(&family(u32::from(letter[0xa]))?), Arg::S(&town8)])?,
                                3 => head,
                                _ => sprintf(self.slot(0x6adda4)?, &[Arg::S(&town8)])?,
                            };
                            s.extend(body);
                        }
                        s.extend_from_slice(self.slot(0x6addac + 4 * u32::from(letter[0xb] & 3))?);
                        s.extend_from_slice(self.slot(0x6addbc)?);
                    }
                    1 => s.extend(sprintf(self.slot(0x6addd4)?, &[Arg::S(&t5)])?),
                    2 => s.extend_from_slice(self.slot(0x6addd8)?),
                    _ => s.extend_from_slice(&stale_tmp),
                }
                s
            }
            0x78 => {
                let mut s = self.slot(0x6add70)?.to_vec();
                // El año pasa al siguiente si la fecha de la carta (mes `+9`, día `+8`) es anterior a la de emisión (`+1`/`+0`).
                let next_year = letter[9] < (letter[1] & 0x7f) || (letter[9] == (letter[1] & 0x7f) && letter[8] < letter[0]);
                let year = u32::from(w(letter, 2)) + u32::from(next_year);
                s.extend(sprintf(self.slot(0x6add74)?, &[Arg::S(&t5), Arg::U(u32::from(letter[8])), Arg::S(self.slot(0x6c0d00 + 4 * u32::from(letter[9]))?), Arg::U(year)])?);
                if letter[0xb] & 0xf0 == 0 {
                    s.extend(sprintf(self.slot(0x6add78)?, &[Arg::S(self.slot(0x6a96ac)?), Arg::S(b""), Arg::S(self.slot(0x6a96d0)?)])?);
                } else {
                    // CString local: se inicializa vacío y se asigna o formatea según el nibble bajo (`0x64F390` / `0x64DF80`).
                    let cstring = match letter[0xb] & 0xf {
                        0 => sprintf(self.slot(0x6a95b8)?, &[Arg::S(self.slot(0x6c0bb0 + 4 * u32::from(letter[0xc]))?)])?,
                        1 => self.slot(0x6a95bc)?.to_vec(),
                        2 | 5 => self.slot(0x6a95c0)?.to_vec(),
                        4 | 6 => self.slot(0x6a95c4)?.to_vec(),
                        _ => Vec::new(),
                    };
                    let town_a = ctx.town_name(usize::from(letter[0xa])).ok_or("ciudad ausente")?;
                    s.extend(sprintf(self.slot(0x6add78)?, &[Arg::S(self.slot(0x6a95b4 - 4 * u32::from(letter[0xb] >> 4))?), Arg::S(&town_a), Arg::S(&cstring)])?);
                }
                let (a, b) = (first(u32::from(letter[0xe]))?, family(u32::from(letter[0xf]))?);
                s.extend(sprintf(
                    self.slot(if letter[0xe] & 0x80 != 0 { 0x6add80 } else { 0x6add7c })?,
                    &[Arg::S(self.slot(0x69ee48 + 4 * u32::from(letter[0xd]))?), Arg::S(&a), Arg::S(&b)],
                )?);
                s
            }
            0x80 => {
                let m = ctx.merchant(u32::from(letter[0xc])).ok_or("comerciante ausente")?;
                let (a, b) = (first(u32::from(m.first))?, family(u32::from(m.family))?);
                let mut s = sprintf(self.cstr(0x6b9dcc)?, &[Arg::S(&a), Arg::S(&b)])?;
                let (ta, tb) = (ctx.town_name(usize::from(letter[0xa])).ok_or("ciudad ausente")?, ctx.town_name(usize::from(letter[0xb])).ok_or("ciudad ausente")?);
                s.extend(sprintf(self.slot(0x6b23a0)?, &[Arg::S(&ta), Arg::S(&tb)])?);
                s.extend_from_slice(self.slot(0x6b23a4)?);
                let months = u32::from(w(letter, 0xe));
                s.extend(sprintf(self.slot(0x6b23a8)?, &[Arg::U(u32::from(letter[0xd])), Arg::U(months % 12 + 1), Arg::U(months / 12)])?);
                let (c, d) = (first(u32::from(letter[8]))?, family(u32::from(letter[9]))?);
                s.extend(sprintf(self.cstr(0x6b9dd8)?, &[Arg::S(self.slot(0x6a0644 - 0x20 * u32::from(letter[8] >> 7))?), Arg::S(&c), Arg::S(&d)])?);
                s
            }
            0x82 => {
                let bit = u32::from(letter[0xc] >> 7);
                let runtime = ctx.runtime_text(0x6a0c54 + 4 * u32::from(w(letter, 8)).wrapping_sub(15 * bit)).ok_or("texto cargado en ejecución no disponible")?;
                let title = self.slot(0x6a0628 + 4 * u32::from(w(letter, 0xa)).wrapping_sub(8 * bit))?;
                let (a, b) = (first(u32::from(w(letter, 0xc)))?, family(u32::from(w(letter, 0xe)))?);
                let mut s = self.slot(0x6b19f0)?.to_vec();
                s.extend(sprintf(self.slot(0x6b19fc + 4 * bit)?, &[Arg::S(&runtime), Arg::S(title), Arg::S(&a), Arg::S(&b)])?);
                s.extend(sprintf(self.slot(0x6b19f4)?, &[Arg::S(&t5)])?);
                s.extend_from_slice(self.slot(0x6b19f8)?);
                s.extend_from_slice(&t5);
                s
            }
            0x84 => {
                let (intro, base) = if w(letter, 0xa) & 0x8000 != 0 { (0x6ab638, 0x38) } else { (0x6ab630, 0x47) };
                let mut s = self.slot(intro)?.to_vec();
                s.extend(ctx.runtime_text(0x6a0b38 + 4 * (u32::from(letter[8] & 0x1f) + base)).ok_or("texto cargado en ejecución no disponible")?);
                s.push(b' ');
                s.extend_from_slice(self.slot(0x6a0628 + 4 * u32::from(letter[9]).wrapping_sub(8 * u32::from(w(letter, 0xa) >> 15)))?);
                s.push(b' ');
                s.extend(first(u32::from(letter[0xb]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xa]))?);
                s.extend_from_slice(self.slot(0x6aafc0)?);
                s.extend(sprintf(self.slot(0x6aafc4)?, &[Arg::U(u32::from(letter[0xc] & 0x3f)), Arg::S(self.slot(0x6c0da0 + 4 * u32::from(letter[0xd]))?), Arg::U(u32::from(w(letter, 0xe)))])?);
                s.extend_from_slice(self.slot(0x6aafc8)?);
                s
            }
            0x42 => {
                let town8 = ctx.town_name(usize::from(w(letter, 8))).ok_or("ciudad ausente")?;
                let mut s = sprintf(self.slot(0x6b7600)?, &[Arg::S(&town8)])?;
                let (a, b) = (first(u32::from(w(letter, 0xc)))?, family(u32::from(w(letter, 0xe)))?);
                s.extend(sprintf(self.slot(0x6b7608)?, &[Arg::S(&a), Arg::S(&b)])?);
                let amount = sprintf(self.cstr(0x6b9d14)?, &[Arg::U(u32::from(w(letter, 0xa)))])?;
                if let Some(runtime) = ctx.runtime_text(0x6b7614) {
                    // Preserve the x86 contract when the caller supplies this text.
                    s.extend_from_slice(self.slot(0x6b760c)?);
                    s.extend_from_slice(&town8);
                    s.extend_from_slice(self.slot(0x6b7610)?);
                    s.extend(amount);
                    s.extend(runtime);
                } else {
                    // The verified original crashes at 0x482659 (ECX=0). Its
                    // preceding fragments also put town/amount in the wrong
                    // gaps. Repair only this absent-text case: fill the double
                    // space in the paragraph with the town, then the reward;
                    // omit the incomplete sentence after the signature marker.
                    let body = self.slot(0x6b760c)?;
                    let gap = body.windows(2).position(|b| b == b"  ").ok_or("hueco de ciudad de recompensa ausente")?;
                    let closing = self.slot(0x6b7610)?;
                    let signature = closing.windows(3).position(|b| b == b"\\r\n").ok_or("firma de recompensa ausente")?;
                    s.extend_from_slice(b"\n\n");
                    s.extend_from_slice(&body[..gap + 1]);
                    s.extend_from_slice(&town8);
                    s.extend_from_slice(&body[gap + 1..]);
                    s.extend(amount);
                    s.extend_from_slice(&closing[..signature + 3]);
                }
                s.extend_from_slice(&t5);
                s
            }
            0x6d => {
                let mut s = if letter[0xc] >= 0x80 {
                    let mut v = self.slot(0x6ab638)?.to_vec();
                    v.extend(ctx.runtime_text(0x6a0c18 + 4 * u32::from(letter[0xb])).ok_or("texto cargado en ejecución no disponible")?);
                    v.extend_from_slice(self.slot(0x6ab63c)?);
                    v
                } else {
                    let mut v = self.slot(0x6ab630)?.to_vec();
                    v.extend(ctx.runtime_text(0x6a0c54 + 4 * u32::from(letter[0xb])).ok_or("texto cargado en ejecución no disponible")?);
                    v.extend_from_slice(self.slot(0x6ab634)?);
                    v
                };
                s.extend(first(u32::from(letter[0xc]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xd]))?);
                s.extend_from_slice(self.slot(0x6ab640)?);
                s.extend_from_slice(&t5);
                s.extend_from_slice(self.slot(0x6ab644)?);
                let (pre, post) = if letter[9] >= 0x80 { (0x6ab650, 0x6ab654) } else { (0x6ab648, 0x6ab64c) };
                s.extend_from_slice(self.slot(pre)?);
                s.extend_from_slice(self.slot(0x69ee48 + 4 * u32::from(letter[8]))?);
                s.extend_from_slice(self.slot(post)?);
                s.extend(first(u32::from(letter[9]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xa]))?);
                s
            }
            0x7b => {
                let mut s = self.cstr(0x6b9dc4)?.to_vec();
                let runtime = ctx.runtime_text(0x6a0c18 + 4 * u32::from(w(letter, 8))).ok_or("texto cargado en ejecución no disponible")?;
                let (a, b) = (first(u32::from(w(letter, 0xa)))?, family(u32::from(w(letter, 0xc)))?);
                s.extend(sprintf(self.slot(0x6b1d6c + 4 * u32::from(w(letter, 0xa) >> 7))?, &[Arg::S(&runtime), Arg::S(&a), Arg::S(&b)])?);
                s.extend(sprintf(self.slot(0x6b1d74)?, &[Arg::S(&t5)])?);
                s
            }
            0x6f | 0x70 => {
                let (a, b) = (first(u32::from(letter[0xa]))?, family(u32::from(letter[0xb]))?);
                let mut s = sprintf(self.cstr(0x6b9d90)?, &[Arg::S(self.slot(0x6ab630 + 4 * u32::from((letter[0xa] >> 6) & 2))?), Arg::S(&a), Arg::S(&b)])?;
                let town = |i: u8| ctx.town_name(usize::from(i)).ok_or("ciudad ausente");
                let hideout = || -> Result<Vec<u8>, String> {
                    let slot = if letter[0xe] < 0x28 { 0x6ddaa8 + 4 * u32::from(letter[0xe]) } else { 0x6ddaa0 };
                    ctx.runtime_text(slot).ok_or_else(|| "texto cargado en ejecución no disponible".to_string())
                };
                let (fe, fam_e) = (first(u32::from(letter[0xe]))?, family(u32::from(letter[0xe]))?);
                if kind == 0x6f {
                    let idx = letter[8] & 0x7f;
                    let body = match idx {
                        0 => sprintf(self.slot(0x6aba78)?, &[Arg::S(&town(letter[0xe])?)])?,
                        1 => sprintf(self.slot(0x6aba7c)?, &[Arg::S(&town(letter[0xe])?), Arg::S(&town(letter[0xf])?)])?,
                        2 => sprintf(self.slot(0x6aba80)?, &[Arg::S(&fe), Arg::S(&family(u32::from(letter[0xf]))?)])?,
                        3 => sprintf(self.slot(0x6aba84)?, &[Arg::S(&hideout()?)])?,
                        4 => sprintf(self.slot(0x6aba88)?, &[Arg::S(&town(letter[0xe])?)])?,
                        _ => Vec::new(),
                    };
                    s.extend(body);
                    s.extend_from_slice(self.slot(0x6aba8c)?);
                    if letter[8] & 0x80 == 0x80 {
                        s.extend_from_slice(self.slot(0x6aba90)?);
                    }
                } else {
                    s.extend_from_slice(self.slot(0x6aba94)?);
                    let body = match letter[8] {
                        0 => sprintf(self.slot(0x6aba98)?, &[Arg::S(&town(letter[0xe])?)])?,
                        1 => sprintf(self.slot(0x6aba9c)?, &[Arg::S(&town(letter[0xe])?), Arg::S(&town(letter[0xf])?)])?,
                        2 => sprintf(self.slot(0x6abaa0)?, &[Arg::S(&fe), Arg::S(&fam_e)])?,
                        3 => sprintf(self.slot(0x6abaa4)?, &[Arg::S(&hideout()?)])?,
                        4 => sprintf(self.slot(0x6abaa8)?, &[Arg::S(&town(letter[0xe])?)])?,
                        _ => Vec::new(),
                    };
                    s.extend(body);
                    s.extend_from_slice(self.slot(0x6abaac)?);
                }
                s.extend_from_slice(self.slot(0x6ab64c + 4 * u32::from((letter[0xc] >> 6) & 2))?);
                s.extend(first(u32::from(letter[0xc]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xd]))?);
                s
            }
            0x48 => {
                let months = u32::from(u16::from_le_bytes(self.at(0x672d78 + 2 * u32::from(letter[1] & 0xf)).and_then(|b| b.get(..2)).ok_or("tabla de meses fuera de rango")?.try_into().unwrap()));
                let tick = (months + u32::from(w(letter, 2)) * 365 + u32::from(letter[0]) + 9) << 8;
                let (year, month, day) = self.ticks_to_date(tick)?;
                let town_c = ctx.town_name(usize::from(letter[0xc])).ok_or("ciudad ausente")?;
                let date_args = [Arg::U(u32::from(day)), Arg::U(u32::from(month)), Arg::U(u32::from(year)), Arg::S(&town_c)];
                let mut s = sprintf(self.slot(0x6ac490 + 4 * u32::from(letter[0xb]))?, &[Arg::S(&town_c)])?;
                s.extend_from_slice(self.slot(if letter[9] > 0x7f { 0x6a0608 } else { 0x6a0628 } + 4 * u32::from(letter[8] & 7))?);
                s.push(b' ');
                s.extend(first(u32::from(letter[9]))?);
                s.push(b' ');
                s.extend(family(u32::from(letter[0xa]))?);
                s.extend_from_slice(b"\n\n");
                let ld = letter[0xd];
                match letter[0xb] {
                    0 => {
                        s.extend(sprintf(self.slot(0x6ac4dc)?, &[Arg::S(&town_c)])?);
                        if ld != 0 {
                            s.extend(sprintf(self.slot(0x6ac4c8)?, &[Arg::S(&town_c)])?);
                            s.extend(sprintf(self.slot(0x6ac4cc)?, &date_args)?);
                        }
                        effects.push(Effect::Local(0x1f, vec![letter[5]]));
                    }
                    1 => {
                        s.extend(sprintf(self.slot(0x6ac49c)?, &[Arg::S(&town_c)])?);
                        s.extend_from_slice(self.slot(0x6ac4a0)?);
                        if ld > 1 {
                            s.extend_from_slice(self.slot(0x6ac4d4)?);
                            s.extend(sprintf(self.slot(0x6ac4d8)?, &date_args)?);
                        } else if ld == 1 {
                            s.extend_from_slice(self.slot(0x6ac4a4)?);
                            s.extend(sprintf(self.slot(0x6ac4a8)?, &date_args)?);
                        }
                    }
                    2 => {
                        s.extend_from_slice(self.slot(0x6ac4ac)?);
                        let (a, b) = (first(u32::from(letter[0xe]))?, family(u32::from(letter[0xf]))?);
                        s.extend(sprintf(self.slot(0x6ac4b0)?, &[Arg::S(&a), Arg::S(&b)])?);
                        if ld == 1 {
                            s.extend_from_slice(self.slot(0x6ac4b4)?);
                            s.extend(sprintf(self.slot(0x6ac4bc)?, &date_args)?);
                        } else if ld > 1 {
                            s.extend_from_slice(self.slot(0x6ac4b8)?);
                            s.extend(sprintf(self.slot(0x6ac4c0)?, &date_args)?);
                        }
                    }
                    _ => {}
                }
                s.extend_from_slice(self.slot(0x6ac4d0)?);
                s
            }
            0x79 => {
                let count = ctx.global(0x6dd748) & 0xffff;
                let idx = u32::from(w(letter, 0xa));
                if idx < count {
                    let rec = ctx.debt_record(idx).ok_or("registro de deuda ausente")?;
                    let members = ctx.global(0x6de4aa) & 0xffff;
                    let out_of_range = u32::from(rec[0x11]) < members || u32::from(rec[0x10]) >= members;
                    if u16::from_le_bytes([rec[6], rec[7]]) != 1 || out_of_range || rec[0x12] != 0 {
                        // `0x535740` deja el mensaje en blanco salvo el primer campo (-1); la rutina fija los demás.
                        effects.push(Effect::Call(0x54aa70, vec![0x77, 0xffff, u32::from(ctx.selected_row_word()), 1, 0]));
                    } else {
                        effects.push(Effect::Local(0x16, 2u16.to_le_bytes().to_vec()));
                        effects.push(Effect::Buttons(vec![self.u32(0x6aebfc)?, self.u32(0x6aec00)?]));
                    }
                }
                let flag = u32::from(ctx.selected_row_flags() & 2);
                let (a, b) = (first(u32::from(letter[0xe]))?, family(u32::from(letter[0xf]))?);
                let mut s = sprintf(self.cstr(0x6b9db4)?, &[Arg::S(self.slot(0x6c3054 - 4 * u32::from(letter[0xe] >> 7))?), Arg::S(&a), Arg::S(&b)])?;
                let number = (u32::from(letter[0xc]) & 0x3c0) << 10;
                let number = number + u32::from(w(letter, 8));
                s.extend(sprintf(
                    self.slot(0x6aec04 + 4 * flag)?,
                    &[Arg::S(self.slot(0x6a0628 + 4 * u32::from(letter[0xc] & 0x2f))?), Arg::S(&family(u32::from(letter[0xd]))?), Arg::S(&t5), Arg::U(number)],
                )?);
                s.extend_from_slice(self.slot(0x6aec08 + 4 * flag)?);
                s.extend_from_slice(self.slot(0x6aec14)?);
                s.extend_from_slice(&t5);
                s
            }
            _ => return Ok(None),
        };
        if matches!(kind, 0xa | 0xb | 0x19 | 0x2f | 0x36 | 0x4a | 0x4b | 0x4c | 0x5b | 0x5c | 0x6b | 0x6e | 0x37 | 0x5a | 0x80 | 0x84 | 0x6d | 0x6f | 0x70) {
            effects.push(Effect::Local(0x1f, vec![letter[5]]));
        }
        text.extend(piece);
        Ok(Some(Composed { text, ui_float, effects }))
    }
}

impl PlayerInfo {
    /// Id de nombre de pila tras `0x4F6420`: bit 7 según el sexo.
    pub fn first_id(&self) -> u32 {
        u32::from(if self.flags != 0 { self.first & 0x7f } else { self.first | 0x80 })
    }
}

impl crate::Sim {
    /// `0x512A60` sobre los bancos cargados (para el compositor de cartas).
    pub fn letter_first_name(&self, id: u32) -> Option<Vec<u8>> {
        p3_core::mission_format::first_name_lookup(self.host.first_names.as_ref()?, id).ok().map(<[u8]>::to_vec)
    }
    /// `0x512AE0` sobre los bancos cargados.
    pub fn letter_family_name(&self, id: u32) -> Option<Vec<u8>> {
        p3_core::mission_format::family_name_lookup(self.host.family_names.as_ref()?, id).ok().map(<[u8]>::to_vec)
    }
    /// `0x512B20` con el banco base (`ship_names`): id de 16 bits; `None` si cae fuera del banco base (rango propio o tabla fija).
    pub fn letter_ship_bank_name(&self, id: u32) -> Option<Vec<u8>> {
        let r = self.host.expedition_resources.as_ref()?;
        r.ship_names.get((id & 0xffff) as usize).map(|n| n.iter().copied().take_while(|&c| c != 0).collect())
    }
    /// Bytes `+2` y `+3` de la entrada `idx` del array de capitanes (`0x6DD7A0`, entradas de 16 bytes).
    pub fn captain_name_ids(&self, idx: usize) -> Option<(u8, u8)> {
        let entry = self.host.expedition_factory.as_ref()?.captains.get(idx)?;
        Some((*entry.get(2)?, *entry.get(3)?))
    }
}

/// Lista de cartas del jugador (`personal_letters_side_menu_build_row_model`, `0x47C520`).
/// Recorre la cadena de nodos (16 bytes; siguiente en +6) desde `head`, conserva los de la pestaña activa y escribe filas
/// de 20 bytes: índice, tipo, subtipo, marca (bit 7 del byte +1) y fecha «DD.MM.AAAA» en +8; cierra con una fila `0xFFFF`.
/// `tab` se recorta a 3 como en el original. Devuelve `(filas, recuento)`; sin filas si `selected >= members`
/// (`0x6DFC14` frente a `0x6DE4AA`) o la cadena está vacía.
pub fn build_letter_rows(
    node: &dyn Fn(u32) -> Option<[u8; 16]>,
    node_count: u16,
    head: u16,
    tab: &mut u16,
    class_of: &dyn Fn(u8) -> u8,
    selected: u32,
    members: u16,
) -> (Vec<u8>, u16) {
    if selected >= u32::from(members) {
        return (Vec::new(), 0);
    }
    if *tab > 3 {
        *tab = 3;
    }
    let mut n: u32 = 0;
    if head < node_count {
        let mut at = u32::from(head);
        loop {
            n += 1;
            let next = node(at).map_or(0xffff, |x| u32::from(u16::from_le_bytes([x[6], x[7]])));
            if next < u32::from(node_count) {
                at = next;
            } else {
                break;
            }
        }
    }
    let n = n & 0xffff;
    if n == 0 {
        return (Vec::new(), 0);
    }
    let mut rows = vec![0u8; 20 * (n as usize + 1)];
    for r in 0..=n as usize {
        rows[20 * r..20 * r + 2].copy_from_slice(&0xffffu16.to_le_bytes());
    }
    let (mut at, mut filled) = (u32::from(head), 0usize);
    while (filled as u32) < n && at < u32::from(node_count) {
        let Some(x) = node(at) else { break };
        if u16::from(class_of(x[4])) == *tab {
            let r = &mut rows[20 * filled..20 * filled + 20];
            r[0..2].copy_from_slice(&(at as u16).to_le_bytes());
            r[2..4].copy_from_slice(&u16::from(x[4]).to_le_bytes());
            r[4..6].copy_from_slice(&u16::from(x[5]).to_le_bytes());
            r[6..8].copy_from_slice(&u16::from(x[1] >> 7).to_le_bytes());
            r[8..18].copy_from_slice(&row_date_text(&x));
            filled += 1;
        }
        at = u32::from(u16::from_le_bytes([x[6], x[7]]));
    }
    rows[20 * filled..20 * filled + 2].copy_from_slice(&0xffffu16.to_le_bytes());
    (rows, filled as u16)
}

/// Día, mes (1..=16) y año de una carta tal como los muestra su fila (`0x47C691`: `and eax,0xf; inc eax`): el byte 1 guarda
/// el mes con base 0 en el nibble bajo y la marca de no leída en el bit 7, igual que el encabezado del cuerpo (`date_text`).
/// WIN-05: la lista del cliente mostraba el byte crudo y quedaba un mes por detrás del cuerpo.
pub fn row_date(x: &[u8]) -> (u8, u8, u16) {
    (x[0], (x[1] & 0xf) + 1, u16::from_le_bytes([x[2], x[3]]))
}

/// Fecha de una fila de la lista: «DD.MM.AAAA» (día = byte 0, mes = (byte 1 & 15) + 1, año = palabra +2), 10 bytes más dos ceros.
/// Los dígitos usan aritmética de byte como el x86 (el millar del año puede salirse de '0'..'9').
pub fn row_date_text(x: &[u8; 16]) -> [u8; 10] {
    let (day, month, year) = row_date(x);
    let (day, month, year) = (u32::from(day), u32::from(month), u32::from(year));
    let digit = |v: u32| (v as u8).wrapping_add(0x30);
    [
        digit(day / 10),
        digit(day % 10),
        b'.',
        digit(month / 10),
        digit(month % 10),
        b'.',
        digit(year / 1000),
        digit(year / 100 % 10),
        digit(year / 10 % 10),
        digit(year % 10),
    ]
}
