//! `CString::Format` (0x0064DF80) del juego: subconjunto de `printf` del CRT original que usan los textos de C08
//! (`%s %i %d %u %x %X %c %%`, banderas `-+ #0`, ancho, precisión, `h/l`) y la forma posicional `%$N$s` de los textos de pirata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FArg {
    Int(u32),
    Str(Vec<u8>),
}

fn pad(out: &mut Vec<u8>, body: &[u8], width: usize, left: bool, zero: bool, sign_len: usize) {
    if body.len() >= width {
        out.extend_from_slice(body);
    } else if left {
        out.extend_from_slice(body);
        out.resize(out.len() + width - body.len(), b' ');
    } else if zero {
        out.extend_from_slice(&body[..sign_len]);
        out.resize(out.len() + width - body.len(), b'0');
        out.extend_from_slice(&body[sign_len..]);
    } else {
        out.resize(out.len() + width - body.len(), b' ');
        out.extend_from_slice(body);
    }
}

pub fn format(fmt: &[u8], args: &[FArg]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut i = 0;
    let mut next = 0usize;
    while i < fmt.len() {
        let c = fmt[i];
        i += 1;
        if c != b'%' {
            out.push(c);
            continue;
        }
        // Forma posicional `%$N$s`.
        if fmt.get(i) == Some(&b'$') {
            let mut j = i + 1;
            let mut n = 0usize;
            while j < fmt.len() && fmt[j].is_ascii_digit() {
                n = n * 10 + usize::from(fmt[j] - b'0');
                j += 1;
            }
            if j > i + 1 && fmt.get(j) == Some(&b'$') && fmt.get(j + 1) == Some(&b's') && n >= 1 {
                match args.get(n - 1) {
                    Some(FArg::Str(s)) => out.extend_from_slice(s),
                    _ => return Err("argumento posicional no es cadena".into()),
                }
                i = j + 2;
                continue;
            }
            return Err("forma posicional no portada".into());
        }
        let (mut left, mut plus, mut space, mut alt, mut zero) = (false, false, false, false, false);
        while let Some(&f) = fmt.get(i) {
            match f {
                b'-' => left = true,
                b'+' => plus = true,
                b' ' => space = true,
                b'#' => alt = true,
                b'0' => zero = true,
                _ => break,
            }
            i += 1;
        }
        let mut width = 0usize;
        while let Some(&d) = fmt.get(i).filter(|d| d.is_ascii_digit()) {
            width = width * 10 + usize::from(d - b'0');
            i += 1;
        }
        let mut prec: Option<usize> = None;
        if fmt.get(i) == Some(&b'.') {
            i += 1;
            let mut p = 0usize;
            while let Some(&d) = fmt.get(i).filter(|d| d.is_ascii_digit()) {
                p = p * 10 + usize::from(d - b'0');
                i += 1;
            }
            prec = Some(p);
        }
        let mut short = false;
        match fmt.get(i) {
            Some(b'h') => {
                short = true;
                i += 1;
            }
            Some(b'l') => i += 1,
            _ => {}
        }
        let t = *fmt.get(i).ok_or("formato truncado")?;
        i += 1;
        if t == b'%' {
            out.push(b'%');
            continue;
        }
        let arg = args.get(next).ok_or("faltan argumentos")?;
        next += 1;
        match (t, arg) {
            (b's', FArg::Str(s)) => {
                let s = &s[..prec.map_or(s.len(), |p| p.min(s.len()))];
                pad(&mut out, s, width, left, false, 0);
            }
            (b'c', FArg::Int(v)) => pad(&mut out, &[*v as u8], width, left, false, 0),
            (b'd' | b'i', FArg::Int(v)) => {
                let n = if short { i32::from(*v as i16) } else { *v as i32 };
                let digits = n.unsigned_abs().to_string();
                let digits = match prec {
                    Some(p) if digits.len() < p => format!("{}{}", "0".repeat(p - digits.len()), digits),
                    _ => digits,
                };
                let sign = if n < 0 {
                    "-"
                } else if plus {
                    "+"
                } else if space {
                    " "
                } else {
                    ""
                };
                let body = format!("{sign}{digits}");
                pad(
                    &mut out,
                    body.as_bytes(),
                    width,
                    left,
                    zero && prec.is_none(),
                    sign.len(),
                );
            }
            (b'u' | b'x' | b'X', FArg::Int(v)) => {
                let n = if short { u32::from(*v as u16) } else { *v };
                let digits = match t {
                    b'u' => n.to_string(),
                    b'x' => format!("{n:x}"),
                    _ => format!("{n:X}"),
                };
                let digits = match prec {
                    Some(p) if digits.len() < p => format!("{}{}", "0".repeat(p - digits.len()), digits),
                    _ => digits,
                };
                let prefix = if alt && n != 0 && t == b'x' {
                    "0x"
                } else if alt && n != 0 && t == b'X' {
                    "0X"
                } else {
                    ""
                };
                let body = format!("{prefix}{digits}");
                pad(
                    &mut out,
                    body.as_bytes(),
                    width,
                    left,
                    zero && prec.is_none(),
                    prefix.len(),
                );
            }
            _ => return Err("tipo de argumento no válido".into()),
        }
    }
    // `ReleaseBuffer` mide la cadena con `strlen`: un NUL intermedio la corta.
    if let Some(z) = out.iter().position(|&b| b == 0) {
        out.truncate(z);
    }
    Ok(out)
}
