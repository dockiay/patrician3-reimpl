//! `SeaBattle::finish(bando)` (0x0060D299, método virtual 7): cierre del combate naval. Sortea el siguiente rival del
//! convoy, completa la lista de participantes, resume cada barco (hundido, huido, saqueado, capturado) en seis
//! búferes de texto, ajusta el valor de gloria `+0x1E94`, avisa a los participantes y deja `+0x5C`, `+0x15C`, `+0x130C` y
//! `+0x130E` listos.
//!
//! Los textos del juego viven en la sección de datos del ejecutable. El módulo no los contiene: pide cada uno al
//! anfitrión por la dirección de su puntero ([`FinishHost::text`]) y reproduce el formato (`%s`, `%d`, `%%`), de modo que
//! el resultado coincide byte a byte con el original sea cual sea el idioma. Fixture: `oracle/x86_battle_finish.py`.

use super::{r16, r32, w16, w32};

type E = &'static str;

/// Direcciones de los punteros a texto que lee la función.
pub mod slot {
    pub const SHIP_LINE: u32 = 0x006A_226C;
    pub const SHIP_FLED: u32 = 0x006A_2278;
    pub const SHIP_PLUNDERED: u32 = 0x006A_A180;
    pub const SHIP_CAPTURED: u32 = 0x006A_2274;
    pub const SHIP_SUNK: u32 = 0x006A_2270;
    /// Tabla de nombres de tipo de barco (`+0x0E` del barco mundial).
    pub const SHIP_TYPE: u32 = 0x006A_174C;
    pub const ESCAPED: u32 = 0x006A_2254;
    pub const DEFEATED: u32 = 0x006A_2248;
    pub const SUCCESS: u32 = 0x006A_2250;
    pub const LOST: u32 = 0x006A_224C;
    pub const TOTAL_FAILURE: u32 = 0x006A_225C;
    pub const DEFEATED_ENEMY: u32 = 0x006A_2244;
    pub const FAILURE: u32 = 0x006A_2258;
    pub const PUT_ATTACKERS: u32 = 0x006A_2240;
    pub const OWN: u32 = 0x006A_2260;
    pub const CAPTURED: u32 = 0x006A_2264;
    pub const ENEMY: u32 = 0x006A_2268;
    pub const PIRATE_HIDEOUT_A: u32 = 0x006A_A1A8;
    pub const PIRATE_HIDEOUT_B: u32 = 0x006A_A1AC;
    pub const PIRATE_HIDEOUT_C: u32 = 0x006A_A1A4;
    pub const OF_THE_MERCHANT: [u32; 2] = [0x006A_16EC, 0x006A_16E8];
    pub const OF_UNKNOWN: u32 = 0x006A_16F8;
    pub const OF_PIRATE: u32 = 0x006A_16F0;
    pub const THE_MERCHANT: [u32; 2] = [0x006A_1700, 0x006A_16FC];
    pub const THE_UNKNOWN: u32 = 0x006A_1708;
    pub const THE_PIRATE: u32 = 0x006A_1704;
    pub const LOWER_MERCHANT: [u32; 2] = [0x006A_1710, 0x006A_170C];
    pub const LOWER_UNKNOWN: u32 = 0x006A_1718;
    pub const LOWER_PIRATE: u32 = 0x006A_1714;
    /// Tablas de tratamientos indexadas por la categoría del comerciante (cuatro bytes por entrada).
    pub const TITLE_UPPER: [u32; 2] = [0x006A_0718, 0x006A_0738];
    pub const TITLE_LOWER: [u32; 2] = [0x006A_0608, 0x006A_0628];
    pub const SHIP_PREFIX_A: u32 = 0x006A_172C;
    pub const SHIP_PREFIX_B: u32 = 0x006A_173C;
    pub const MSG_ATTACK: u32 = 0x006A_1D80;
    pub const MSG_FLED: u32 = 0x006A_1D84;
    pub const MSG_BOARDED: u32 = 0x006A_1D88;
    pub const MSG_FULL_OF: u32 = 0x006A_1D8C;
    pub const MSG_USING: u32 = 0x006A_1D90;
    pub const MSG_BEATEN: u32 = 0x006A_1D94;
    /// Funciones del original que devuelven el nombre de pirata.
    pub const PIRATE_NAME_A: u32 = 0x0051_2A60;
    pub const PIRATE_NAME_B: u32 = 0x0051_2AE0;
}

/// Globales que lee o escribe `finish` fuera del objeto de combate.
pub struct FinishGlobals<'a> {
    /// Tabla de convoyes (0x006DD7A8, 60 bytes por entrada).
    pub fleets: &'a [u8],
    /// Palabra baja de 0x006DD894: número de barcos mundiales.
    pub world_count: u16,
    /// Palabra alta de 0x006DD894: número de convoyes (el original también la compara con identificadores de barco).
    pub fleet_count: u16,
    /// 0x006DD892: número de capitanes.
    pub captain_count: u16,
    /// 0x006DE4AA: número de comerciantes.
    pub merchant_count: u16,
    /// Registros de comerciante, 1616 bytes cada uno (0x005303C0).
    pub merchants: &'a [u8],
    /// 0x006DFC14: jugador activo.
    pub player: u32,
    /// 0x006E59CD: hay un mensaje abierto.
    pub dialog_open: u8,
    /// 0x006E59D8: reloj de la última noticia.
    pub news_stamp: u32,
    /// 0x006DE4B4: reloj del mundo.
    pub clock: u32,
}

/// Lo externo que llama `finish`, en el orden del original.
pub trait FinishHost {
    /// Cadena a la que apunta el puntero guardado en `slot`.
    fn text(&mut self, slot: u32) -> Vec<u8>;
    /// Cadena a la que apunta `pointer` (nombres de los registros de comerciante).
    fn cstr(&mut self, pointer: u32) -> Vec<u8>;
    /// Nombre de pirata: `which` es 0x00512A60 o 0x00512AE0.
    fn pirate_name(&mut self, which: u32, index: u8) -> Vec<u8>;
    /// 0x004F9430 sobre el registro del comerciante.
    fn merchant_flag(&mut self, merchant: u32) -> u8;
    /// 0x004D6530: `kind`/`byte5` son los bytes `+4`/`+5` del registro, `d8` y `d12` sus palabras `+8` y `+12`;
    /// `text` va solo con el tipo 0x29 (su puntero y longitud se reducen a ese texto).
    fn event(&mut self, target: u32, kind: u8, byte5: u8, d8: u32, d12: u32, text: Option<&[u8]>);
    /// 0x0041D9B0 con la estructura de mensaje (`battle[0x12EC..]`).
    fn show_message(&mut self, message: &[u8]);
    /// 0x00469380(0).
    fn news_open(&mut self);
    /// 0x00469AF0(0x14, nombre, 0).
    fn news(&mut self, name: u8);
    /// Cada cadena que el original deja escrita (strcpy, strcat, sprintf), en orden.
    fn trace(&mut self, text: &[u8]);
}

pub(super) enum Arg<'a> {
    S(&'a [u8]),
    D(i32),
}

pub(super) fn format(fmt: &[u8], args: &[Arg]) -> Result<Vec<u8>, E> {
    let mut out = Vec::new();
    let mut next = args.iter();
    let mut i = 0;
    while i < fmt.len() {
        if fmt[i] != b'%' {
            out.push(fmt[i]);
            i += 1;
            continue;
        }
        i += 1;
        match fmt.get(i) {
            Some(b'%') => out.push(b'%'),
            Some(b's') => match next.next() {
                Some(Arg::S(s)) => out.extend_from_slice(s),
                _ => return Err("argumento de formato ausente"),
            },
            Some(b'd') => match next.next() {
                Some(Arg::D(v)) => out.extend_from_slice(v.to_string().as_bytes()),
                _ => return Err("argumento de formato ausente"),
            },
            _ => return Err("formato no modelado"),
        }
        i += 1;
    }
    Ok(out)
}

fn cstring(bytes: &[u8], at: usize) -> Result<Vec<u8>, E> {
    let tail = bytes.get(at..).ok_or("cadena fuera de rango")?;
    let end = tail.iter().position(|&b| b == 0).ok_or("cadena sin terminador")?;
    Ok(tail[..end].to_vec())
}

fn put_text(battle: &mut [u8], at: usize, text: &[u8]) -> Result<(), E> {
    let end = at + text.len() + 1;
    if end > battle.len() {
        return Err("texto fuera del objeto de combate");
    }
    battle[at..at + text.len()].copy_from_slice(text);
    battle[at + text.len()] = 0;
    Ok(())
}

struct Ctx<'a, 'g, H: FinishHost> {
    ships: &'a [Vec<u8>],
    worlds: &'a [u8],
    g: &'a mut FinishGlobals<'g>,
    host: &'a mut H,
}

impl<'a, 'g, H: FinishHost> Ctx<'a, 'g, H> {
    fn world(&self, id: u32) -> Result<&'a [u8], E> {
        let at = (id as usize).checked_mul(0x180).ok_or("barco mundial fuera de rango")?;
        self.worlds.get(at..at + 0x180).ok_or("barco mundial fuera de rango")
    }

    fn merchant(&self, id: u32) -> Result<&'g [u8], E> {
        let at = (id as usize).checked_mul(1616).ok_or("comerciante fuera de rango")?;
        self.g.merchants.get(at..at + 1616).ok_or("comerciante fuera de rango")
    }

    fn fleet_word(&self, fleet: u16, offset: usize) -> Result<u16, E> {
        let at = usize::from(fleet) * 0x3C + offset;
        self.g
            .fleets
            .get(at..at + 2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .ok_or("convoy inexistente")
    }

    fn text(&mut self, slot: u32) -> Vec<u8> {
        self.host.text(slot)
    }

    fn sprintf(&mut self, slot: u32, args: &[Arg]) -> Result<Vec<u8>, E> {
        let fmt = self.host.text(slot);
        let out = format(&fmt, args)?;
        self.host.trace(&out);
        Ok(out)
    }

    fn strcat(&mut self, dest: &mut Vec<u8>, src: &[u8]) {
        dest.extend_from_slice(src);
        self.host.trace(dest);
    }

    fn strcpy(&mut self, dest: &mut Vec<u8>, src: &[u8]) {
        dest.clear();
        dest.extend_from_slice(src);
        self.host.trace(dest);
    }

    /// `tabla[índice]` de punteros a texto.
    fn indexed(&mut self, table: u32, index: u8) -> Vec<u8> {
        self.host.text(table + 4 * u32::from(index))
    }

    /// Nombre del comerciante/tratamiento de una línea "el/del/la <categoría> <nombre> <apellido>".
    fn merchant_phrase(&mut self, slot_pair: [u32; 2], titles: [u32; 2], owner: u32) -> Result<Vec<u8>, E> {
        let rec = self.merchant(owner)?;
        let pick = usize::from(rec[0x1A] != 0);
        let title_index = rec[0x39C + usize::from(rec[0x19])];
        let title = self.indexed(titles[pick], title_index);
        let surname = self.host.cstr(r32(rec, 0xE8) as u32);
        let name = self.host.cstr(r32(rec, 0xE4) as u32);
        self.sprintf(slot_pair[pick], &[Arg::S(&title), Arg::S(&surname), Arg::S(&name)])
    }

    fn pirate_phrase(&mut self, slot: u32, b: &[u8]) -> Result<Vec<u8>, E> {
        let second = self.host.pirate_name(slot::PIRATE_NAME_B, b[0x1E91]);
        let first = self.host.pirate_name(slot::PIRATE_NAME_A, b[0x1E90]);
        self.sprintf(slot, &[Arg::S(&first), Arg::S(&second)])
    }
}

/// `SeaBattle::finish(side)`. `base` es la dirección del objeto de combate (el mensaje guarda un puntero a su texto).
pub fn sea_finish<H: FinishHost>(
    battle: &mut [u8],
    ships: &[Vec<u8>],
    worlds: &[u8],
    next: &[u16],
    base: u32,
    g: &mut FinishGlobals,
    host: &mut H,
    side: u8,
) -> Result<(), E> {
    let flag = r16(battle, 0x15C);
    if flag != 0 && flag < 0xF000 {
        return Ok(());
    }
    let mut c = Ctx { ships, worlds, g, host };
    let (owner_a, owner_b) = (r16(battle, 0x35A), r16(battle, 0x35C));
    // Más abajo los participantes nuevos pueden reescribir esas dos palabras: allí se vuelven a leer.

    // Siguiente rival: sorteo ponderado por la capacidad de los barcos del convoy del bando 1.
    let mut opponent: u16 = 0xFFFF;
    if side == 0 && battle[0x12E8] == 0 {
        let fleet = r16(battle, 0x660);
        if fleet < c.g.fleet_count {
            let first = c.fleet_word(fleet, 10)?;
            if first < c.g.world_count {
                let mut total: u32 = 0;
                let mut id = first;
                let mut guard = 0;
                loop {
                    let w = c.world(u32::from(id))?;
                    if u16::from(w[0]) != owner_b && u16::from(w[0]) != owner_a {
                        total = total.wrapping_add(r32(w, 0x10) as u32);
                    }
                    id = r16(w, 6);
                    guard += 1;
                    if id >= c.g.world_count {
                        break;
                    }
                    if guard > 70_000 {
                        return Err("lista de convoy cíclica");
                    }
                }
                if (total as i32) >= 1 {
                    let mut seed = (r32(battle, 0xADC) as u32)
                        .wrapping_mul(0xBB40_E64D)
                        .wrapping_add(0xA205_B063);
                    seed = seed.rotate_right(1);
                    w32(battle, 0xADC, seed as i32);
                    let mut remaining = (seed % total) as i32 + 1;
                    let mut pick = first;
                    guard = 0;
                    loop {
                        let w = c.world(u32::from(pick))?;
                        if u16::from(w[0]) != owner_b && u16::from(w[0]) != owner_a {
                            remaining = remaining.wrapping_sub(r32(w, 0x10));
                        }
                        if remaining < 1 {
                            break;
                        }
                        pick = r16(w, 6);
                        guard += 1;
                        if guard > 70_000 {
                            return Err("lista de convoy cíclica");
                        }
                    }
                    opponent = pick;
                }
            }
        }
    }

    // Participantes: se añaden los dueños de los convoyes que aún no constan.
    let mut count = u32::from(battle[0x558]);
    if battle[0x12E8] == 0 {
        for (fleet_word, party_word, map) in [(0x65Eusize, 0x35Ausize, 0u8), (0x660, 0x35C, 1)] {
            let other = r16(battle, party_word);
            let fleet = r16(battle, fleet_word);
            if fleet >= c.g.fleet_count || other == 0xFE {
                continue;
            }
            let mut id = c.fleet_word(fleet, 10)?;
            if id >= c.g.world_count {
                continue;
            }
            let mut guard = 0;
            loop {
                let w = c.world(u32::from(id))?;
                let owner = u16::from(w[0]);
                let known = (0..count as usize).any(|j| r16(battle, 0x35A + 2 * j) == owner);
                if count == 0 || !known {
                    let k = count as usize;
                    if 0x35A + 2 * k + 2 > battle.len() || 0x5D + k >= battle.len() || 0x559 + k >= battle.len() {
                        return Err("demasiados participantes");
                    }
                    w16(battle, 0x35A + 2 * k, owner);
                    battle[0x5D + k] = 1;
                    battle[0x559 + k] = map;
                    count += 1;
                }
                id = r16(w, 6);
                guard += 1;
                if id >= c.g.world_count {
                    break;
                }
                if guard > 70_000 {
                    return Err("lista de convoy cíclica");
                }
            }
        }
    }

    // Resumen de cada barco en los seis búferes.
    let mut result_a: Vec<u8> = Vec::new(); // piVar7
    let mut result_b: Vec<u8> = Vec::new(); // piVar8
    let mut lines_0: Vec<u8> = Vec::new(); // local_244
    let mut lines_1: Vec<u8> = Vec::new(); // local_238
    let mut extra_1: Vec<u8> = Vec::new(); // local_23c
    let mut extra_0: Vec<u8> = Vec::new(); // pcVar11
    let (mut n248, mut n24c, mut n250, mut n254, mut n25c, mut n258) = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8);
    let (mut n25e, mut n260, mut n276, mut n278, mut n27a, mut n27c) = (0u16, 0u16, 0u16, 0u16, 0u16, 0u16);

    let mut first_pass = r16(battle, 0x10) != 0xFFFF;
    let mut list = if first_pass {
        r16(battle, 0x10)
    } else {
        r16(battle, 0x65C)
    };
    while list != 0xFFFF {
        loop {
            let id = list;
            let ship = c.ships.get(usize::from(id)).ok_or("barco local ausente")?;
            let w = c.world(r32(ship, 4) as u32)?;
            let mut line = Vec::new();
            let mut captured_line = Vec::new();
            if r32(w, 0x14) != 0 {
                let name = cstring(w, 0x160)?;
                c.strcpy(&mut line, &name);
                let flags = ship[0x12A];
                let kind: u8;
                let weight: i32;
                let type_name = c.indexed(slot::SHIP_TYPE, w[0xE]);
                let percent = r32(w, 0x18)
                    .wrapping_mul(100)
                    .checked_div(r32(w, 0x14))
                    .ok_or("división inválida")?;
                if flags & 4 != 0 {
                    line = c.sprintf(slot::SHIP_SUNK, &[Arg::S(&type_name), Arg::S(&name)])?;
                    weight = 100;
                    kind = 4;
                } else if ship[0x13D] == ship[0x142] {
                    if flags & 2 == 0 {
                        if flags & 0x40 == 0 {
                            line = c.sprintf(slot::SHIP_LINE, &[Arg::S(&type_name), Arg::S(&name), Arg::D(percent)])?;
                            kind = 0;
                        } else {
                            line = c.sprintf(
                                slot::SHIP_PLUNDERED,
                                &[Arg::S(&type_name), Arg::S(&name), Arg::D(percent)],
                            )?;
                            kind = 0x40;
                        }
                    } else {
                        line = c.sprintf(slot::SHIP_FLED, &[Arg::S(&type_name), Arg::S(&name), Arg::D(percent)])?;
                        kind = 0;
                    }
                    weight = 50;
                    if ship[0x142] == 0 {
                        n248 = n248.wrapping_add(1);
                        if flags & 0x60 != 0 {
                            n24c = n24c.wrapping_add(1);
                        }
                    } else {
                        n250 = n250.wrapping_add(1);
                        if flags & 0x60 != 0 {
                            n254 = n254.wrapping_add(1);
                        }
                    }
                } else {
                    line = c.sprintf(
                        slot::SHIP_CAPTURED,
                        &[Arg::S(&type_name), Arg::S(&name), Arg::D(percent)],
                    )?;
                    captured_line =
                        c.sprintf(slot::SHIP_LINE, &[Arg::S(&type_name), Arg::S(&name), Arg::D(percent)])?;
                    weight = 200;
                    kind = 8;
                }
                let glory = r16(battle, 0x1E94);
                let adjusted = |glory: u16| -> u16 {
                    let product = (1000 - i32::from(glory & 0x3FF)).wrapping_mul(1000 - weight) + 500;
                    (glory & 0xFC00).wrapping_add((1000 - product / 1000) as u16)
                };
                if ship[0x142] == 0 {
                    n25c = n25c.wrapping_add(1);
                    c.strcat(&mut lines_0, &line);
                    c.strcat(&mut extra_0, &captured_line);
                    if battle[0xAD2] == 2 {
                        w16(battle, 0x1E94, adjusted(glory));
                    }
                    match kind {
                        4 => n25e = n25e.wrapping_add(1),
                        8 => n276 = n276.wrapping_add(1),
                        0x40 => n27a = n27a.wrapping_add(1),
                        _ => {}
                    }
                } else {
                    n258 = n258.wrapping_add(1);
                    c.strcat(&mut lines_1, &line);
                    c.strcat(&mut extra_1, &captured_line);
                    if battle[0xAD2] == 1 {
                        w16(battle, 0x1E94, adjusted(glory));
                    }
                    match kind {
                        4 => n260 = n260.wrapping_add(1),
                        8 => n278 = n278.wrapping_add(1),
                        0x40 => n27c = n27c.wrapping_add(1),
                        _ => {}
                    }
                }
            }
            list = *next.get(usize::from(id)).ok_or("lista de barcos incompleta")?;
            if list == 0xFFFF {
                break;
            }
        }
        if !first_pass {
            break;
        }
        first_pass = false;
        list = r16(battle, 0x65C);
    }

    // Textos de resultado por bando.
    let (head_b, head_a);
    if side == 0 {
        if i32::from(n258) < (i32::from(n250) - i32::from(n254)) * 2 {
            head_b = c.text(slot::ESCAPED);
            head_a = c.text(slot::DEFEATED);
        } else {
            head_b = c.text(slot::SUCCESS);
            head_a = c.text(slot::LOST);
        }
    } else if (i32::from(n248) - i32::from(n24c)) * 2 <= i32::from(n25c) {
        head_b = c.text(slot::TOTAL_FAILURE);
        head_a = c.text(slot::DEFEATED_ENEMY);
    } else {
        head_b = c.text(slot::FAILURE);
        head_a = c.text(slot::PUT_ATTACKERS);
    }
    c.strcpy(&mut result_b, &head_b);
    c.strcpy(&mut result_a, &head_a);
    let (own, captured, enemy) = (c.text(slot::OWN), c.text(slot::CAPTURED), c.text(slot::ENEMY));
    c.strcat(&mut result_b, &own);
    c.strcat(&mut result_b, &lines_0);
    if !extra_1.is_empty() {
        c.strcat(&mut result_b, &captured);
        c.strcat(&mut result_b, &extra_1);
    }
    c.strcat(&mut result_b, &enemy);
    c.strcat(&mut result_b, &lines_1);
    c.strcat(&mut result_a, &own);
    c.strcat(&mut result_a, &lines_1);
    if !extra_0.is_empty() {
        c.strcat(&mut result_a, &captured);
        c.strcat(&mut result_a, &extra_0);
    }
    c.strcat(&mut result_a, &enemy);
    c.strcat(&mut result_a, &lines_0);

    // Aviso a cada participante.
    if n25c != 0 && count != 0 {
        for k in 0..count as usize {
            let id = r16(battle, 0x35A + 2 * k);
            if id >= c.g.merchant_count {
                continue;
            }
            if battle[0x5D + k] == 0 {
                if c.g.player == u32::from(id) && battle[0xAC4] != 2 {
                    announce(&mut c, battle, base, side, k)?;
                }
            } else {
                let rec = c.merchant(u32::from(id))?;
                let text = if battle[0x559 + k] != 0 { &result_a } else { &result_b };
                c.host.event(
                    u32::from(id),
                    0x29,
                    rec[0x19],
                    0,
                    text.len() as u32 & 0xFFFF,
                    Some(text),
                );
            }
        }
    }

    let diff = if side == 0 {
        i32::from(n258) - i32::from(n250)
    } else {
        i32::from(n25c) - i32::from(n248)
    };
    let mut gain = diff as u32;
    let mut gain16 = gain as u16;
    let mut calm = c.g.news_stamp.wrapping_add(0x1E00) < c.g.clock;
    if !calm {
        let party = r16(battle, ((0x1AE - usize::from(side)) * 2) as usize);
        if party < c.g.merchant_count && r16(c.merchant(u32::from(party))?, 8) == 0 {
            calm = true;
        }
    }
    if gain16 == 0 && calm {
        let had_losses = if side == 0 { n254 } else { n24c };
        if had_losses != 0 {
            gain = 0xFF;
            gain16 = 0xFF;
        }
    }
    if r16(battle, 0x35A + 2 * usize::from(side)) == 0xFF && gain16 != 0 {
        let fleet_ship = if side == 1 {
            let f = c.fleet_word(r16(battle, 0x660), 0x10)?;
            if c.g.fleet_count <= f {
                None
            } else {
                Some(f)
            }
        } else {
            Some(c.fleet_word(r16(battle, 0x65E), 0x10)?)
        };
        if let Some(f) = fleet_ship {
            let w = c.world(u32::from(f))?;
            if r32(w, 0x18) != 0 && r16(w, 0x42) < c.g.captain_count {
                let party_at = (0x1AE - usize::from(side)) * 2;
                let party = u32::from(r16(battle, party_at));
                let rec = c.merchant(party)?;
                let stat = c.host.merchant_flag(party);
                let d8 = u32::from(rec[0x1A])
                    | u32::from(stat) << 8
                    | u32::from(rec[0x39C + usize::from(rec[0x19])]) << 16
                    | u32::from(rec[0x1C]) << 24;
                let d12 = u32::from(rec[0x1D])
                    | u32::from(battle[0x1E90]) << 8
                    | u32::from(battle[0x1E91]) << 16
                    | (gain as u8 as u32) << 24;
                c.host.event(0xFFFF_FFFE, 0x2A, rec[0x19], d8, d12, None);
                if c.g.player != party {
                    c.host.news_open();
                    c.host.news(battle[0x1E91]);
                    c.g.news_stamp = c.g.clock;
                }
            }
        }
    }
    let (owner_a, owner_b) = (r16(battle, 0x35A), r16(battle, 0x35C));
    if owner_a != 0xFF && (n260 != 0 || n278 != 0 || n27c != 0) {
        let d8 = u32::from(owner_b) | u32::from(n260) << 16;
        c.host.event(
            u32::from(owner_a),
            0x72,
            0,
            d8,
            u32::from(n278) | u32::from(n27c) << 16,
            None,
        );
    }
    if owner_b != 0xFF && (n25e != 0 || n276 != 0 || n27a != 0) {
        let d8 = u32::from(owner_a) | u32::from(n25e) << 16;
        c.host.event(
            u32::from(owner_b),
            0x72,
            0,
            d8,
            u32::from(n276) | u32::from(n27a) << 16,
            None,
        );
    }
    battle[0x130E] = side;
    w16(battle, 0x130C, opponent);
    battle[0x5C] = 1;
    w16(battle, 0x15C, 0xFA);
    if battle[0xAC8] != 0 {
        w16(battle, 0x15C, 1);
    }
    Ok(())
}

/// Texto y ventana del aviso al jugador humano (rama `player == id`).
fn announce<H: FinishHost>(c: &mut Ctx<H>, battle: &mut [u8], base: u32, side: u8, k: usize) -> Result<(), E> {
    let kind = battle[0xAC4];
    let tied = side == battle[0x559 + k];
    let ship_slot = usize::from(r16(battle, 0xAC6));
    let ship = c.ships.get(ship_slot).ok_or("barco local ausente")?;
    let world = c.world(r32(ship, 4) as u32)?;
    let owner = world[0];
    let ship_type = world[0xE];
    let flags = battle[0xAD2];
    let party_at = |index: usize| 0x35A + 2 * index;
    if tied {
        if kind == 0 {
            let phrase;
            if side == 0 {
                if owner == 0xFE {
                    phrase = c.text(slot::PIRATE_HIDEOUT_A);
                    c.host.trace(&phrase.clone());
                } else if flags & 2 == 0 {
                    phrase = c.merchant_phrase(slot::OF_THE_MERCHANT, slot::TITLE_UPPER, u32::from(owner))?;
                } else {
                    let f = c.fleet_word(r16(battle, 0x660), 0x10)?;
                    let unknown = owner != 0xFF || c.g.world_count <= f || {
                        let w = c.world(u32::from(f))?;
                        r32(w, 0x18) == 0 || c.g.captain_count <= r16(w, 0x42)
                    };
                    if unknown {
                        phrase = c.text(slot::OF_UNKNOWN);
                        c.host.trace(&phrase.clone());
                    } else {
                        phrase = c.pirate_phrase(slot::OF_PIRATE, battle)?;
                    }
                }
                let prefix = c.indexed(slot::SHIP_PREFIX_A, ship_type);
                let message = c.sprintf(slot::MSG_ATTACK, &[Arg::S(&prefix), Arg::S(&phrase)])?;
                put_text(battle, 0x1310, &message)?;
            } else {
                if owner == 0xFE {
                    phrase = c.text(slot::PIRATE_HIDEOUT_A);
                    c.host.trace(&phrase.clone());
                } else if flags & 1 == 0 {
                    phrase = c.merchant_phrase(slot::THE_MERCHANT, slot::TITLE_UPPER, u32::from(owner))?;
                } else {
                    let unknown = owner != 0xFF || {
                        let f = c.fleet_word(r16(battle, 0x65E), 0x10)?;
                        let w = c.world(u32::from(f))?;
                        r32(w, 0x18) == 0 || c.g.captain_count <= r16(w, 0x42)
                    };
                    if unknown {
                        phrase = c.text(slot::THE_UNKNOWN);
                        c.host.trace(&phrase.clone());
                    } else {
                        phrase = c.pirate_phrase(slot::THE_PIRATE, battle)?;
                    }
                }
                let prefix = c.indexed(slot::SHIP_PREFIX_B, ship_type);
                let message = c.sprintf(slot::MSG_FLED, &[Arg::S(&phrase), Arg::S(&prefix)])?;
                put_text(battle, 0x1310, &message)?;
            }
        } else if kind == 1 {
            let phrase;
            if r16(battle, party_at(usize::from(owner))) == 0xFE {
                phrase = c.text(slot::PIRATE_HIDEOUT_A);
                c.host.trace(&phrase.clone());
            } else if (1u32 << ((1u32.wrapping_sub(u32::from(side))) & 31)) as u8 & flags == 0 {
                phrase = c.merchant_phrase(slot::LOWER_MERCHANT, slot::TITLE_LOWER, u32::from(owner))?;
            } else if owner != 0xFF || battle[0x1E90] == 0xFF || battle[0x1E91] == 0xFF {
                phrase = c.text(slot::LOWER_UNKNOWN);
                c.host.trace(&phrase.clone());
            } else {
                phrase = c.pirate_phrase(slot::LOWER_PIRATE, battle)?;
            }
            let message = c.sprintf(slot::MSG_BOARDED, &[Arg::S(&phrase)])?;
            put_text(battle, 0x1310, &message)?;
        }
    } else if kind == 0 {
        if side == 0 || flags & 2 != 0 {
            let text = c.text(slot::MSG_USING);
            put_text(battle, 0x1310, &text)?;
            c.host.trace(&text);
        } else {
            let party = r16(battle, party_at(usize::from(side)));
            if party != 0xFE {
                let rec_a = c.merchant(u32::from(party))?;
                let name = c.host.cstr(r32(rec_a, 0xE4) as u32);
                let surname = c.host.cstr(r32(c.merchant(u32::from(party))?, 0xE8) as u32);
                let message = c.sprintf(slot::MSG_FULL_OF, &[Arg::S(&surname), Arg::S(&name)])?;
                put_text(battle, 0x1310, &message)?;
            } else {
                let text = c.text(slot::PIRATE_HIDEOUT_C);
                put_text(battle, 0x1310, &text)?;
                c.host.trace(&text);
            }
        }
    } else if kind == 1 {
        let pirate_ship = if side == 0 {
            let f = c.fleet_word(r16(battle, 0x660), 0x10)?;
            if f < c.g.world_count {
                Some(f)
            } else {
                None
            }
        } else {
            Some(c.fleet_word(r16(battle, 0x65E), 0x10)?)
        };
        let party = r16(battle, party_at(usize::from(side)));
        let phrase;
        if party == 0xFE {
            phrase = c.text(slot::PIRATE_HIDEOUT_B);
            c.host.trace(&phrase.clone());
        } else if (1u32 << (u32::from(side) & 31)) as u8 & flags == 0 {
            phrase = c.merchant_phrase(slot::THE_MERCHANT, slot::TITLE_LOWER, u32::from(party))?;
        } else {
            let unknown = party != 0xFF
                || match pirate_ship {
                    None => true,
                    Some(f) => {
                        let w = c.world(u32::from(f))?;
                        r32(w, 0x18) == 0 || c.g.captain_count <= r16(w, 0x42)
                    }
                };
            if unknown {
                phrase = c.text(slot::THE_UNKNOWN);
                c.host.trace(&phrase.clone());
            } else {
                phrase = c.pirate_phrase(slot::THE_PIRATE, battle)?;
            }
        }
        let message = c.sprintf(slot::MSG_BEATEN, &[Arg::S(&phrase)])?;
        put_text(battle, 0x1310, &message)?;
    }
    // Estructura del mensaje y apertura de la ventana.
    w32(battle, 0x12EC, 0);
    battle[0x12F0] = 0;
    w16(battle, 0x12F2, 0);
    w32(battle, 0x12F4, 0);
    battle[0x12F8] = 0xFF;
    w32(battle, 0x12FC, 0x3FB0_0000);
    battle[0x1300] = 1;
    w16(battle, 0x1302, 0);
    w32(battle, 0x1304, 0);
    w32(battle, 0x1308, 0);
    battle[0x12FB] = 0xFF;
    battle[0x12F9] = 0;
    battle[0x12FA] = 0;
    w32(battle, 0x12EC, (base + 0x1310) as i32);
    w16(battle, 0x12F2, 0);
    w32(battle, 0x12F4, 0);
    battle[0x12F0] = 1;
    battle[0x12F8] = 0xFF;
    w32(battle, 0x12FC, 0x3F35_04F4);
    battle[0x1300] = 1;
    if c.g.dialog_open == 0 {
        c.host.show_message(&battle[0x12EC..]);
        battle[0x12E9] = 0;
        battle[0x12EA] = 1;
        c.g.dialog_open = 1;
    } else {
        battle[0x12E9] = 1;
    }
    Ok(())
}
