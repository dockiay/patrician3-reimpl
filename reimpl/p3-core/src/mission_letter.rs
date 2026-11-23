//! Construcción de cartas de misión: subconjunto contrastado de `0xF7`.
//! La entrega al depósito y los formatos aún no cubiertos quedan al llamante.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptLetter {
    pub kind: u8,
    pub town: u8,
    pub recipient: u32,
    pub descriptor: [u8; 16],
    pub text: Vec<u8>,
    pub notification: Option<[u32; 6]>,
}

pub struct GrammarContext<'a> {
    pub merchants: &'a [crate::ai::reputation::Merchant],
    pub gender_words: &'a [[Vec<u8>; 2]; 8],
    /// Tablas y estado de los formatos `R r S s n v w D W F` (`mission_format`).
    pub extras: Option<&'a crate::mission_format::FormatExtras>,
}

/// Prepara las clases 0..5 y conserva la notificación como efecto explícito. Reconoce las sustituciones `%Q` (nombre
/// de barco), `%t` (ciudad), `%N` (apellido), `%V` (nombre de pila), `%B` (entero), `%E` (marca), `%c` (entero y sufijo de moneda), `%+`/`%-` (bytes del descriptor) y `%%`. Si falta un recurso o aparece
/// otro formato devuelve error sin modificar las variables.
fn prepare_letter_inner<'a>(
    allow_broadcast: bool,
    blob: &[u8],
    cmd: &[u8],
    vars: &mut [u32],
    head: u16,
    ship_name: impl Fn(u16) -> Option<&'a [u8]>,
    town_names: &[Vec<u8>],
    family_names: &[Vec<u8>],
    first_names: &[Vec<Vec<u8>>; 2],
    grammar: Option<GrammarContext<'_>>,
) -> Result<ScriptLetter, String> {
    if cmd.len() < 12 || cmd[0] != 0xf7 || cmd[1] > 5 {
        return Err("clase de carta no admitida".into());
    }
    let get = |i: u8| {
        vars.get(usize::from(i))
            .copied()
            .ok_or_else(|| format!("variable {i} fuera de rango"))
    };
    let town = get(cmd[2])? as u8;
    let deadline = get(cmd[4])?;
    let offset = u32::from_le_bytes(cmd[8..12].try_into().unwrap()) as usize;
    let mut at = offset;
    let mut text = Vec::new();
    let copy_z = |at: &mut usize, text: &mut Vec<u8>| -> Result<(), String> {
        loop {
            let b = *blob.get(*at).ok_or("texto de carta truncado")?;
            *at += 1;
            text.push(b);
            if b == 0 {
                return Ok(());
            }
            if text.len() > 4096 {
                return Err("texto de carta demasiado largo".into());
            }
        }
    };
    copy_z(&mut at, &mut text)?;
    // `%+v` fija el byte `+0xD` del descriptor con `vars[v]`; `%-?ab` fija `+0xE`/`+0xF` con los dos bytes literales.
    let (mut plus, mut minus) = (None::<u8>, None::<(u8, u8)>);
    loop {
        let b = *blob.get(at).ok_or("cuerpo de carta truncado")?;
        at += 1;
        if b == 0 {
            text.push(0);
            break;
        }
        if b != b'%' {
            text.push(b)
        } else {
            let kind = *blob.get(at).ok_or("formato de carta truncado")?;
            let index = *blob.get(at + 1).ok_or("operando de formato truncado")?;
            at += 2;
            if kind == b'E' {
                text.extend_from_slice(if index == 0 { b"\\B" } else { b"\\L" });
                if text.len() > 4096 {
                    return Err("texto de carta demasiado largo".into());
                }
                continue;
            }
            match kind {
                b'-' => {
                    let a = *blob.get(at).ok_or("operando de formato truncado")?;
                    let b = *blob.get(at + 1).ok_or("operando de formato truncado")?;
                    at += 2;
                    minus = Some((a, b));
                    continue;
                }
                b'%' => {
                    text.push(b'%');
                    continue;
                }
                _ => {}
            }
            if kind == b'F' {
                let env = grammar.as_ref().ok_or("tabla gramatical del ejecutable ausente")?;
                let ex = env.extras.ok_or("tablas de formato del ejecutable ausentes")?;
                let first = |a: u8| -> Result<Vec<u8>, String> {
                    let (bank, i) = if a & 0x80 != 0 { (1, usize::from(a & 0x7f)) } else { (0, usize::from(a)) };
                    let names = &first_names[bank];
                    let n = names.get(i).or_else(|| names.first()).ok_or("nombres de pila ausentes")?;
                    Ok(n.split(|&c| c == 0).next().unwrap_or_default().to_vec())
                };
                let family = |b: u8| -> Result<Vec<u8>, String> {
                    let n = family_names.get(usize::from(b)).or_else(|| family_names.first()).ok_or("apellidos ausentes")?;
                    Ok(n.split(|&c| c == 0).next().unwrap_or_default().to_vec())
                };
                let r = crate::mission_format::format_extra(kind, 0, ex, env.merchants, &first, &family)
                    .expect("F es de mission_format")?;
                text.extend_from_slice(&r);
                if text.len() > 4096 {
                    return Err("texto de carta demasiado largo".into());
                }
                continue;
            }
            if index >= 128 {
                return Err("índice de formato con signo no admitido".into());
            }
            let value = get(index)?;
            if matches!(kind, b'R' | b'r' | b'S' | b's' | b'n' | b'v' | b'w' | b'D' | b'W') {
                let env = grammar.as_ref().ok_or("tabla gramatical del ejecutable ausente")?;
                let ex = env.extras.ok_or("tablas de formato del ejecutable ausentes")?;
                let none = |_: u8| -> Result<Vec<u8>, String> { Err("sin nombres".into()) };
                let r = crate::mission_format::format_extra(kind, value, ex, env.merchants, &none, &none)
                    .expect("formato de mission_format")?;
                text.extend_from_slice(&r);
                if text.len() > 4096 {
                    return Err("texto de carta demasiado largo".into());
                }
                continue;
            }
            if kind == b'+' {
                plus = Some(value as u8);
                continue;
            }
            if (b'0'..=b'7').contains(&kind) {
                let env = grammar.as_ref().ok_or("tabla gramatical del ejecutable ausente")?;
                if let Some(merchant) = env.merchants.get(usize::from(value as u16)) {
                    let gender = *merchant.bytes.get(0x1a).ok_or("comerciante gramatical truncado")? & 1;
                    text.extend_from_slice(&env.gender_words[usize::from(kind - b'0')][usize::from(gender)])
                }
                if text.len() > 4096 {
                    return Err("texto de carta demasiado largo".into());
                }
                continue;
            }
            match kind {
                b'Q' => {
                    let name = ship_name(value as u16).ok_or("nombre de barco ausente")?;
                    text.extend_from_slice(name.split(|&c| c == 0).next().unwrap_or_default())
                }
                b't' => {
                    if town_names.is_empty() {
                        return Err("nombres urbanos ausentes".into());
                    }
                    let name = &town_names[(value as usize) % town_names.len()];
                    text.extend_from_slice(name.split(|&c| c == 0).next().unwrap_or_default())
                }
                b'N' => {
                    let name = family_names
                        .get(value as usize)
                        .or_else(|| family_names.first())
                        .ok_or("apellidos ausentes")?;
                    text.extend_from_slice(name.split(|&c| c == 0).next().unwrap_or_default())
                }
                b'V' => {
                    let (bank, index) = if value & 0x80 != 0 {
                        (1, (value & 0x7f) as usize)
                    } else {
                        (0, value as usize)
                    };
                    let names = &first_names[bank];
                    let name = names
                        .get(index)
                        .or_else(|| names.first())
                        .ok_or("nombres de pila ausentes")?;
                    text.extend_from_slice(name.split(|&c| c == 0).next().unwrap_or_default())
                }
                b'B' => text.extend_from_slice((value as i32).to_string().as_bytes()),
                b'c' => {
                    text.extend_from_slice((value as i32).to_string().as_bytes());
                    text.extend_from_slice(b"\\C")
                }
                _ => return Err(format!("formato de carta %{kind:02x} no implementado")),
            }
        }
        if text.len() > 4096 {
            return Err("texto de carta demasiado largo".into());
        }
    }
    for _ in 0..cmd[6] {
        copy_z(&mut at, &mut text)?
    }
    if text.len() > 4096 {
        return Err("texto de carta demasiado largo".into());
    }
    let mut descriptor = [0u8; 16];
    descriptor[0..4].copy_from_slice(&(text.len() as u32).to_le_bytes());
    descriptor[4..8].copy_from_slice(&deadline.to_le_bytes());
    descriptor[8..10].copy_from_slice(&head.to_le_bytes());
    descriptor[10] = cmd[3];
    descriptor[11] = cmd[6];
    descriptor[12] = cmd[5];
    descriptor[13] = plus.unwrap_or(0xff);
    if let Some((a, b)) = minus {
        descriptor[14] = a;
        descriptor[15] = b;
    }
    if !allow_broadcast && cmd[6] != 0 && cmd[1] != 5 && (get(cmd[5])? as i32) < 0 {
        return Err("difusión de carta con respuestas requiere contexto humano".into());
    }
    let mut staged = vars.to_vec();
    *staged
        .get_mut(usize::from(cmd[3]))
        .ok_or("variable de respuesta ausente")? = u32::from(cmd[6]);
    let recipient = *staged
        .get(usize::from(cmd[5]))
        .ok_or("variable de destinatario ausente")?;
    let notification = if cmd[7] != 0 {
        Some([
            u32::from(recipient as u16),
            0,
            0,
            u32::from(*staged.get(usize::from(cmd[2])).ok_or("ciudad de aviso ausente")? as u16),
            0x12,
            0,
        ])
    } else {
        None
    };
    vars.copy_from_slice(&staged);
    Ok(ScriptLetter {
        kind: if cmd[1] <= 4 { 0x3c + cmd[1] } else { 0x71 },
        town,
        recipient,
        descriptor,
        text,
        notification,
    })
}

/// Conserva la API de una sola carta: la difusión requiere el contexto del intérprete.
pub fn prepare_letter<'a>(
    blob: &[u8],
    cmd: &[u8],
    vars: &mut [u32],
    head: u16,
    ship_name: impl Fn(u16) -> Option<&'a [u8]>,
    town_names: &[Vec<u8>],
    family_names: &[Vec<u8>],
    first_names: &[Vec<Vec<u8>>; 2],
    grammar: Option<GrammarContext<'_>>,
) -> Result<ScriptLetter, String> {
    prepare_letter_inner(
        false,
        blob,
        cmd,
        vars,
        head,
        ship_name,
        town_names,
        family_names,
        first_names,
        grammar,
    )
}

/// Devuelve las cartas del intérprete, y la notificación final si hay difusión.
/// El x86 inicializa primero ocho respuestas, después ocho destinatarios a -1;
/// solo entonces lee el destinatario, de modo que -3 también sigue la ruta -1.
pub fn prepare_letters<'a>(
    blob: &[u8],
    cmd: &[u8],
    vars: &mut [u32],
    head: u16,
    ship_name: impl Fn(u16) -> Option<&'a [u8]>,
    town_names: &[Vec<u8>],
    family_names: &[Vec<u8>],
    first_names: &[Vec<Vec<u8>>; 2],
    grammar: Option<GrammarContext<'_>>,
    merchant_flags: &[u16],
) -> Result<(Vec<ScriptLetter>, Option<([u32; 6], u32)>), String> {
    if cmd.len() < 12 {
        return Err("comando de carta truncado".into());
    }
    let mut staged = vars.to_vec();
    let broadcast = cmd[6] != 0
        && cmd[1] != 5
        && (*vars
            .get(usize::from(cmd[5]))
            .ok_or("variable de destinatario ausente")? as i32)
            < 0;
    let mut letter = prepare_letter_inner(
        true,
        blob,
        cmd,
        &mut staged,
        head,
        ship_name,
        town_names,
        family_names,
        first_names,
        grammar,
    )?;
    if !broadcast {
        vars.copy_from_slice(&staged);
        return Ok((vec![letter], None));
    }
    staged.clone_from_slice(vars);
    let replies = usize::from(cmd[3]);
    let recipients = usize::from(cmd[5]);
    for i in 0..8 {
        *staged
            .get_mut(replies + i)
            .ok_or("variables de respuesta insuficientes")? = u32::from(cmd[6]);
    }
    for i in 0..8 {
        *staged
            .get_mut(recipients + i)
            .ok_or("variables de destinatario insuficientes")? = u32::MAX;
    }
    let mut out = Vec::new();
    for (id, &flags) in merchant_flags.iter().enumerate() {
        if flags != 0 {
            continue;
        }
        let at = recipients
            .checked_add(out.len())
            .ok_or("índice de destinatario fuera de rango")?;
        *staged.get_mut(at).ok_or("variables de destinatario insuficientes")? = id as u32;
        let mut copy = letter.clone();
        copy.recipient = id as u32;
        copy.notification = None;
        out.push(copy);
        letter.descriptor[10] = letter.descriptor[10].wrapping_add(1);
    }
    let recipient = *staged.get(recipients).ok_or("variable de destinatario ausente")?;
    let notification = if cmd[7] != 0 {
        let town = *staged.get(usize::from(cmd[2])).ok_or("ciudad de aviso ausente")?;
        Some((
            [u32::from(recipient as u16), 0, 0, u32::from(town as u16), 0x12, 0],
            recipient,
        ))
    } else {
        None
    };
    vars.copy_from_slice(&staged);
    Ok((out, notification))
}
