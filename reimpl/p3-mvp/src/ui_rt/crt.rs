//! Formato y lectura de cadenas del CRT de Microsoft (`sprintf`/`CString::Format` y `sscanf`) para el enlazador de interfaz.
//! Cubre lo que usan los textos del juego: `%d %i %u %x %X %o %c %s %f %e %g %p %%` con indicadores, ancho y precisión.

/// Formatea `fmt` tomando los argumentos de pila con `next` (dobles palabras; `%f` toma dos).
///
/// Admite además argumentos posicionales (base 1) en dos grafías: `%$N$s` (la del formateador propio del juego, `0x64D410`) y
/// `%N$s` (la de C). Los usan las traducciones del paquete de idioma cuando la frase del otro idioma pide los argumentos en otro
/// orden («Reúnete antes del 28 de mayo...» frente a «Meet X by 28. May...»). Con posicionales cada argumento ocupa una doble
/// palabra (no vale para `%f` ni `%I64`); sin ellos el comportamiento es el del CRT.
pub fn format(fmt: &[u8], mut next: impl FnMut() -> u32, mut string: impl FnMut(u32) -> Vec<u8>) -> Vec<u8> {
    let positional = positional_index(fmt).is_some();
    let words: Vec<u32> = if positional { (0..32).map(|_| next()).collect() } else { Vec::new() };
    let cursor = std::cell::Cell::new(0usize);
    let mut next = || {
        if positional {
            let k = cursor.get();
            cursor.set(k + 1);
            words.get(k).copied().unwrap_or(0)
        } else {
            next()
        }
    };
    let mut out = Vec::new();
    let mut i = 0;
    while i < fmt.len() {
        let c = fmt[i];
        i += 1;
        if c != b'%' {
            out.push(c);
            continue;
        }
        if positional {
            if let Some((n, len)) = positional_spec(&fmt[i..]) {
                cursor.set(n - 1);
                i += len;
            }
        }
        let (mut left, mut plus, mut space, mut zero, mut alt) = (false, false, false, false, false);
        while i < fmt.len() {
            match fmt[i] {
                b'-' => left = true,
                b'+' => plus = true,
                b' ' => space = true,
                b'0' => zero = true,
                b'#' => alt = true,
                _ => break,
            }
            i += 1;
        }
        let mut width = 0usize;
        if fmt.get(i) == Some(&b'*') {
            let w = next() as i32;
            if w < 0 {
                left = true;
            }
            width = w.unsigned_abs() as usize;
            i += 1;
        } else {
            while let Some(d) = fmt.get(i).filter(|d| d.is_ascii_digit()) {
                width = width * 10 + usize::from(d - b'0');
                i += 1;
            }
        }
        let mut precision: Option<usize> = None;
        if fmt.get(i) == Some(&b'.') {
            i += 1;
            let mut p = 0usize;
            if fmt.get(i) == Some(&b'*') {
                p = (next() as i32).max(0) as usize;
                i += 1;
            } else {
                while let Some(d) = fmt.get(i).filter(|d| d.is_ascii_digit()) {
                    p = p * 10 + usize::from(d - b'0');
                    i += 1;
                }
            }
            precision = Some(p);
        }
        let mut long_long = false;
        while let Some(&m) = fmt.get(i) {
            match m {
                b'h' | b'l' | b'L' | b'w' => i += 1,
                b'I' if fmt.get(i + 1..i + 3) == Some(b"64") => {
                    long_long = true;
                    i += 3;
                }
                _ => break,
            }
        }
        let Some(&conv) = fmt.get(i) else { break };
        i += 1;
        let mut sign = String::new();
        let body: Vec<u8> = match conv {
            b'%' => {
                out.push(b'%');
                continue;
            }
            b'c' => vec![next() as u8],
            b's' => {
                let p = next();
                let mut s = if p == 0 { b"(null)".to_vec() } else { string(p) };
                if let Some(p) = precision {
                    s.truncate(p);
                }
                s
            }
            b'd' | b'i' => {
                let v = if long_long {
                    let lo = next();
                    let hi = next();
                    (u64::from(hi) << 32 | u64::from(lo)) as i64
                } else {
                    i64::from(next() as i32)
                };
                if v < 0 {
                    sign = "-".into();
                } else if plus {
                    sign = "+".into();
                } else if space {
                    sign = " ".into();
                }
                let mut digits = v.unsigned_abs().to_string();
                if let Some(p) = precision {
                    if p == 0 && v == 0 {
                        digits.clear();
                    }
                    while digits.len() < p {
                        digits.insert(0, '0');
                    }
                }
                digits.into_bytes()
            }
            b'u' | b'x' | b'X' | b'o' | b'p' => {
                let v = if long_long {
                    let lo = next();
                    let hi = next();
                    u64::from(hi) << 32 | u64::from(lo)
                } else {
                    u64::from(next())
                };
                let mut digits = match conv {
                    b'u' => v.to_string(),
                    b'x' => format!("{v:x}"),
                    b'o' => format!("{v:o}"),
                    b'p' => format!("{v:08X}"),
                    _ => format!("{v:X}"),
                };
                if let Some(p) = precision {
                    while digits.len() < p {
                        digits.insert(0, '0');
                    }
                }
                if alt && v != 0 {
                    match conv {
                        b'x' => sign = "0x".into(),
                        b'X' => sign = "0X".into(),
                        b'o' => digits.insert(0, '0'),
                        _ => {}
                    }
                }
                digits.into_bytes()
            }
            b'f' | b'e' | b'E' | b'g' | b'G' => {
                let lo = next();
                let hi = next();
                let v = f64::from_bits(u64::from(hi) << 32 | u64::from(lo));
                let p = precision.unwrap_or(6);
                if v.is_sign_negative() {
                    sign = "-".into();
                } else if plus {
                    sign = "+".into();
                } else if space {
                    sign = " ".into();
                }
                let a = v.abs();
                let s = match conv {
                    b'f' => fixed(a, p),
                    b'e' | b'E' => {
                        let s = format!("{a:.p$e}");
                        let (m, e) = s.split_once('e').unwrap_or((&s, "0"));
                        let e: i32 = e.parse().unwrap_or(0);
                        let s = format!("{m}e{}{:03}", if e < 0 { '-' } else { '+' }, e.abs());
                        if conv == b'E' { s.to_uppercase() } else { s }
                    }
                    _ => {
                        let p = p.max(1);
                        let exp = if a == 0.0 { 0 } else { a.log10().floor() as i32 };
                        if exp < -4 || exp >= p as i32 {
                            let s = format!("{a:.*e}", p - 1);
                            let (m, e) = s.split_once('e').unwrap_or((&s, "0"));
                            let m = if alt { m.to_string() } else { m.trim_end_matches('0').trim_end_matches('.').to_string() };
                            let e: i32 = e.parse().unwrap_or(0);
                            format!("{m}e{}{:03}", if e < 0 { '-' } else { '+' }, e.abs())
                        } else {
                            let decimals = (p as i32 - 1 - exp).max(0) as usize;
                            let s = format!("{a:.decimals$}");
                            if alt || !s.contains('.') { s } else { s.trim_end_matches('0').trim_end_matches('.').to_string() }
                        }
                    }
                };
                s.into_bytes()
            }
            b'n' => {
                next();
                Vec::new()
            }
            other => vec![b'%', other],
        };
        let len = sign.len() + body.len();
        let pad = width.saturating_sub(len);
        let numeric = !matches!(conv, b's' | b'c');
        if left {
            out.extend_from_slice(sign.as_bytes());
            out.extend_from_slice(&body);
            out.extend(std::iter::repeat_n(b' ', pad));
        } else if zero && numeric && precision.is_none_or(|_| matches!(conv, b'f' | b'e' | b'E' | b'g' | b'G')) {
            out.extend_from_slice(sign.as_bytes());
            out.extend(std::iter::repeat_n(b'0', pad));
            out.extend_from_slice(&body);
        } else {
            out.extend(std::iter::repeat_n(b' ', pad));
            out.extend_from_slice(sign.as_bytes());
            out.extend_from_slice(&body);
        }
    }
    out
}

/// `$N$` o `N$` tras un `%`: (N, bytes que ocupa). N empieza en 1.
fn positional_spec(rest: &[u8]) -> Option<(usize, usize)> {
    let skip = usize::from(rest.first() == Some(&b'$'));
    let digits = rest[skip..].iter().take_while(|c| c.is_ascii_digit()).count();
    if digits == 0 || rest.get(skip + digits) != Some(&b'$') {
        return None;
    }
    let n: usize = std::str::from_utf8(&rest[skip..skip + digits]).ok()?.parse().ok()?;
    (n >= 1).then_some((n, skip + digits + 1))
}

/// Primera conversión posicional de `fmt`, si hay alguna.
fn positional_index(fmt: &[u8]) -> Option<usize> {
    let mut i = 0;
    while i < fmt.len() {
        if fmt[i] == b'%' {
            if fmt.get(i + 1) == Some(&b'%') {
                i += 2;
                continue;
            }
            if let Some((n, _)) = positional_spec(&fmt[i + 1..]) {
                return Some(n);
            }
        }
        i += 1;
    }
    None
}

/// `%.pf` redondeando la mitad hacia arriba sobre el valor decimal exacto (como el CRT de Visual C++ 6).
fn fixed(a: f64, p: usize) -> String {
    let exact = format!("{a:.60}");
    let (int, frac) = exact.split_once('.').unwrap_or((&exact, ""));
    let mut digits: Vec<u8> = int.bytes().chain(frac.bytes().take(p)).collect();
    let round = frac.as_bytes().get(p).is_some_and(|d| *d >= b'5');
    if round {
        let mut i = digits.len();
        loop {
            if i == 0 {
                digits.insert(0, b'1');
                break;
            }
            i -= 1;
            if digits[i] == b'9' {
                digits[i] = b'0';
            } else {
                digits[i] += 1;
                break;
            }
        }
    }
    let n = digits.len() - p;
    let mut s = String::from_utf8(digits[..n].to_vec()).unwrap_or_default();
    if p > 0 {
        s.push('.');
        s.push_str(std::str::from_utf8(&digits[n..]).unwrap_or(""));
    }
    s
}

/// Valor que `sscanf` escribe en un destino.
#[derive(Debug, PartialEq)]
pub enum Scanned {
    Int(u32),
    Str(Vec<u8>),
    Char(u8),
    F32(f32),
    F64(f64),
}

/// `sscanf(input, fmt, ...)`: lista de asignaciones en orden y su número.
pub fn scan(input: &[u8], fmt: &[u8]) -> Vec<Scanned> {
    let mut out = Vec::new();
    let (mut i, mut k) = (0usize, 0usize);
    let skip_ws = |s: &[u8], mut p: usize| {
        while p < s.len() && s[p].is_ascii_whitespace() {
            p += 1;
        }
        p
    };
    while k < fmt.len() {
        let f = fmt[k];
        if f.is_ascii_whitespace() {
            i = skip_ws(input, i);
            k += 1;
            continue;
        }
        if f != b'%' {
            if input.get(i) != Some(&f) {
                break;
            }
            i += 1;
            k += 1;
            continue;
        }
        k += 1;
        let suppress = fmt.get(k) == Some(&b'*');
        if suppress {
            k += 1;
        }
        let mut width = 0usize;
        while let Some(d) = fmt.get(k).filter(|d| d.is_ascii_digit()) {
            width = width * 10 + usize::from(d - b'0');
            k += 1;
        }
        let mut long = false;
        while let Some(&m) = fmt.get(k) {
            match m {
                b'l' | b'L' => {
                    long = true;
                    k += 1;
                }
                b'h' => k += 1,
                _ => break,
            }
        }
        let Some(&conv) = fmt.get(k) else { break };
        k += 1;
        let limit = |start: usize| if width > 0 { (start + width).min(input.len()) } else { input.len() };
        match conv {
            b'%' => {
                i = skip_ws(input, i);
                if input.get(i) != Some(&b'%') {
                    break;
                }
                i += 1;
            }
            b'd' | b'i' | b'u' | b'x' | b'X' | b'o' => {
                i = skip_ws(input, i);
                let end = limit(i);
                let mut j = i;
                let neg = input.get(j) == Some(&b'-');
                if matches!(input.get(j), Some(b'-' | b'+')) && j < end {
                    j += 1;
                }
                let mut radix = match conv {
                    b'x' | b'X' => 16,
                    b'o' => 8,
                    _ => 10,
                };
                if conv == b'i' && input.get(j) == Some(&b'0') {
                    if matches!(input.get(j + 1), Some(b'x' | b'X')) {
                        radix = 16;
                        j += 2;
                    } else {
                        radix = 8;
                    }
                } else if radix == 16 && input.get(j) == Some(&b'0') && matches!(input.get(j + 1), Some(b'x' | b'X')) {
                    j += 2;
                }
                let start = j;
                let mut v: u32 = 0;
                while j < end {
                    let Some(d) = char::from(input[j]).to_digit(radix) else { break };
                    v = v.wrapping_mul(radix).wrapping_add(d);
                    j += 1;
                }
                if j == start {
                    break;
                }
                i = j;
                if !suppress {
                    out.push(Scanned::Int(if neg { v.wrapping_neg() } else { v }));
                }
            }
            b'f' | b'e' | b'g' | b'E' | b'G' => {
                i = skip_ws(input, i);
                let end = limit(i);
                let mut j = i;
                while j < end && (input[j].is_ascii_digit() || matches!(input[j], b'-' | b'+' | b'.' | b'e' | b'E')) {
                    j += 1;
                }
                let Ok(v) = std::str::from_utf8(&input[i..j]).unwrap_or("").parse::<f64>() else { break };
                i = j;
                if !suppress {
                    out.push(if long { Scanned::F64(v) } else { Scanned::F32(v as f32) });
                }
            }
            b's' => {
                i = skip_ws(input, i);
                let end = limit(i);
                let mut j = i;
                while j < end && !input[j].is_ascii_whitespace() {
                    j += 1;
                }
                if j == i {
                    break;
                }
                if !suppress {
                    out.push(Scanned::Str(input[i..j].to_vec()));
                }
                i = j;
            }
            b'c' => {
                let Some(&ch) = input.get(i) else { break };
                i += 1;
                if !suppress {
                    out.push(Scanned::Char(ch));
                }
            }
            _ => break,
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(fmt: &str, args: &[u32]) -> String {
        let mut it = args.iter().copied();
        String::from_utf8(format(fmt.as_bytes(), || it.next().unwrap_or(0), |_| b"abc".to_vec())).unwrap()
    }

    #[test]
    fn printf_basics() {
        assert_eq!(f("Font%d", &[3]), "Font3");
        assert_eq!(f("%5d|%-4d|%05d|%x|%X|%u", &[42, 7, -3i32 as u32, 255, 255, u32::MAX]), "   42|7   |-0003|ff|FF|4294967295");
        assert_eq!(f("%s %c %%", &[1, b'z' as u32]), "abc z %");
        let v = 3.25f64.to_bits();
        assert_eq!(f("%.1f", &[v as u32, (v >> 32) as u32]), "3.3");
        assert_eq!(f("%.2d", &[5]), "05");
    }

    #[test]
    fn printf_positional() {
        // Frase de misión española sobre los argumentos del EXE inglés (`0x69E748`): fecha delante, persona y lugar detrás.
        assert_eq!(f("%$3$u de %$4$u: %$1$u %$2$u, %$3$u%%", &[1, 2, 28, 1301]), "28 de 1301: 1 2, 28%");
        assert_eq!(f("%2$d-%1$d", &[7, 9]), "9-7");
        assert_eq!(f("%$2$5d|", &[1, 42]), "   42|");
    }

    #[test]
    fn scanf_basics() {
        assert_eq!(scan(b"12 13 1", b"%i %i %i"), vec![Scanned::Int(12), Scanned::Int(13), Scanned::Int(1)]);
        assert_eq!(scan(b"-5,abc", b"%d,%s"), vec![Scanned::Int(-5i32 as u32), Scanned::Str(b"abc".to_vec())]);
        assert_eq!(scan(b"x", b"%d"), vec![]);
    }
}
