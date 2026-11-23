//! Mezcla del controlador gráfico SGL en superficies de memoria de 32 bits (`ddraw_Dll.dll`).
//!
//! `Patrician3.exe` dibuja con `0x004BB330` (`sgl_DrawBitmapRect(sx, sy, dx, dy, w, h, w2, h2)`), que salta por la
//! ranura `0x006DAA28` al controlador cargado por `0x004BAD00` (`gl.cfg` sin clave `DLL` → `ddraw_dll.dll`). En una
//! superficie de memoria el controlador copia la textura activa (`sgl_SetTexture`) con el rasterizador `0x05014050`:
//! recorta contra el rectángulo de recorte (`0x0508D0C8`), elige rutina según el modo de `sgl_SetBlendMode`
//! (`0x0004BB7A0` → byte `0x0508D085`), el color de `sgl_SetColor` (`0x0506E93C`, bytes B,G,R,A en `0x0508D080`) y los
//! indicadores de la textura (`+0x34`), y mezcla píxel a píxel. Las rutinas son las variantes SSE que `0x0508D1E1`
//! instala en cualquier CPU con MMX y SSE; en ellas la tabla de 22 entradas se reduce a ocho mezclas distintas.
//!
//! Probado contra la DLL original en emulación (`oracle/x86_sgl_blit.py`, `tests/sgl_blit.rs`). Fuera de alcance: el
//! camino Direct3D (`0x05022600`, cuadros con texturas en la tarjeta: el resultado depende del dispositivo), las
//! texturas de 8 bits con paleta (rutinas 11–21) y el escalado (`w2 ≠ w` o `h2 ≠ h`, `0x05014317`).

/// Superficie de 32 bits (`0xAARRGGBB` en memoria como B, G, R, A). `pitch` en píxeles.
pub struct Surface<'a> {
    pub pixels: &'a mut [u32],
    pub pitch: usize,
}

/// Textura de origen: píxeles, paso, color clave (`tex+8`) e indicadores (`tex+0x34`).
pub struct Texture<'a> {
    pub pixels: &'a [u32],
    pub pitch: usize,
    pub key: u32,
    pub flags: u32,
}

/// Estado del controlador para una copia: modo de mezcla, color y recorte `[izq, arriba, der, abajo]` (`None` si el byte
/// `0x0508D084` está a cero: el original no recorta y escribiría fuera de la superficie).
#[derive(Clone, Copy, Debug)]
pub struct BlitState {
    pub mode: u8,
    pub color: u32,
    pub clip: Option<[i32; 4]>,
}

/// Rutina elegida por `0x05014050` (índice de su tabla `0x0501449C`, sin el +11 de las texturas con paleta).
pub fn select_routine(mode: u8, color: u32, flags: u32) -> u8 {
    let white = color & 0xFF_FFFF == 0xFF_FFFF;
    let opaque_white = color == 0xFFFF_FFFF;
    match mode {
        3 => 5 + u8::from(white),
        2 => 7 + u8::from(white),
        5 => 9 + u8::from(white),
        1 => {
            if flags & 7 == 7 {
                if opaque_white {
                    1
                } else {
                    2
                }
            } else if flags & 3 == 3 || flags & 0x8000 != 0 {
                if opaque_white {
                    4
                } else {
                    2
                }
            } else if flags & 1 != 0 {
                if opaque_white {
                    0
                } else if color >> 24 == 0xFF {
                    3
                } else if white {
                    4
                } else {
                    2
                }
            } else if opaque_white {
                0
            } else {
                3
            }
        }
        _ => 0,
    }
}

fn bytes(v: u32) -> [u16; 4] {
    [(v & 0xFF) as u16, (v >> 8 & 0xFF) as u16, (v >> 16 & 0xFF) as u16, (v >> 24) as u16]
}

fn pack(b: [u16; 4]) -> u32 {
    let s = |x: u16| u32::from(x.min(255));
    s(b[0]) | s(b[1]) << 8 | s(b[2]) << 16 | s(b[3]) << 24
}

/// Mezcla de un píxel por la rutina `routine` (variantes SSE). `c` es el color de `sgl_SetColor`.
pub fn blend_pixel(routine: u8, s: u32, d: u32, c: u32, key: u32) -> Option<u32> {
    let (sb, db, cb) = (bytes(s), bytes(d), bytes(c));
    let per = |f: &dyn Fn(u16, u16, u16) -> u16| pack([0, 1, 2, 3].map(|i| f(sb[i], db[i], cb[i])));
    Some(match routine {
        // 0x0508D636 (copia; menos de 32 píxeles por fila, 0x0508D58A).
        0 => s,
        // 0x0508D7CA: copia salvo el color clave sin alfa (se compara el píxel entero con `clave & 0xFFFFFF`).
        1 => {
            if s == key & 0xFF_FFFF {
                return None;
            }
            s
        }
        // 0x0508DEBA (rutinas 2, 3 y 4): textura × color y mezcla por su alfa, en palabras con signo de 16 bits
        // (`pmullw` conserva los 16 bits bajos del producto y `psraw` desplaza con signo).
        2..=4 => {
            let m = [0, 1, 2, 3].map(|i| sb[i] * cb[i] >> 8);
            let a = m[3] as i16;
            pack([0, 1, 2, 3].map(|i| {
                let t = (m[i] as i16).wrapping_sub(db[i] as i16).wrapping_mul(a) >> 8;
                (t.wrapping_add(db[i] as i16) as u16) & 0xFF
            }))
        }
        // 0x0508ECFF: suma saturada de textura × color.
        5 => per(&|s, d, c| (s * c >> 8) + d),
        // 0x0508EFBD: suma saturada.
        6 => per(&|s, d, _| s + d),
        // 0x0508F9A8: textura × color × destino.
        7 => per(&|s, d, c| (s * c >> 8) * d >> 8),
        // 0x0508FCD9: textura × destino.
        8 => per(&|s, d, _| s * d >> 8),
        // 0x05090028 (rutinas 9 y 10): textura × destino × 2, saturado.
        9 | 10 => per(&|s, d, _| s * d >> 7),
        _ => return None,
    })
}

/// `sgl_DrawBitmapRect(sx, sy, dx, dy, w, h, w, h)` sobre una superficie de memoria de 32 bits sin paleta. Devuelve la
/// rutina usada, o `None` si el recorte deja el rectángulo vacío.
#[allow(clippy::too_many_arguments)]
pub fn draw_bitmap_rect(
    dst: &mut Surface,
    tex: &Texture,
    st: &BlitState,
    src: (i32, i32),
    at: (i32, i32),
    size: (i32, i32),
) -> Option<u8> {
    draw_with_routine(dst, tex, st, select_routine(st.mode, st.color, tex.flags), src, at, size)
}

#[allow(clippy::too_many_arguments)]
fn draw_with_routine(
    dst: &mut Surface,
    tex: &Texture,
    st: &BlitState,
    routine: u8,
    (mut sx, mut sy): (i32, i32),
    (mut dx, mut dy): (i32, i32),
    (mut w, mut h): (i32, i32),
) -> Option<u8> {
    if let Some([l, t, r, b]) = st.clip {
        // 0x0508D0C8: el ancho del recorte es `der − izq` y se compara con `dx − izq + w`.
        let mut axis = |d: &mut i32, s: &mut i32, n: &mut i32, lo: i32, span: i32| -> bool {
            let mut e = d.wrapping_sub(lo);
            if e < 0 {
                *n = n.wrapping_add(e);
                if *n <= 0 {
                    return false;
                }
                *s = s.wrapping_sub(e);
                *d = lo;
                e = *n;
            } else {
                e = e.wrapping_add(*n);
            }
            let over = e.wrapping_sub(span);
            if over > 0 {
                *n = n.wrapping_sub(over);
                if *n <= 0 {
                    return false;
                }
            }
            true
        };
        if !axis(&mut dx, &mut sx, &mut w, l, r.wrapping_sub(l)) || !axis(&mut dy, &mut sy, &mut h, t, b.wrapping_sub(t)) {
            return None;
        }
    }
    if w <= 0 || h <= 0 {
        return None;
    }
    for y in 0..h as usize {
        let so = (sy as usize + y) * tex.pitch + sx as usize;
        let d0 = (dy as usize + y) * dst.pitch + dx as usize;
        for x in 0..w as usize {
            let d = &mut dst.pixels[d0 + x];
            if let Some(v) = blend_pixel(routine, tex.pixels[so + x], *d, st.color, tex.key) {
                *d = v;
            }
        }
    }
    Some(routine)
}

/// Textura de 8 bits con paleta (`0x0508D034` no nulo): el rasterizador suma 11 a la rutina (`0x050141AD`; las 20 y 21,
/// de los modos 5, caen en la copia 11).
#[derive(Clone, Copy, Debug)]
pub struct PaletteTexture<'a> {
    pub indices: &'a [u8],
    /// Bytes por fila.
    pub pitch: usize,
    pub palette: &'a [u32],
    pub key: u32,
    pub flags: u32,
}

/// `sgl_DrawBitmapRect` sobre una textura con paleta: cada índice se convierte con la paleta y se mezcla con la rutina sin
/// paleta equivalente (contraste en `tests/sgl_blit.rs`). Devuelve la rutina original (`+11`).
pub fn draw_palette_rect(
    dst: &mut Surface,
    tex: &PaletteTexture,
    st: &BlitState,
    src: (i32, i32),
    at: (i32, i32),
    size: (i32, i32),
) -> Option<u8> {
    let expanded: Vec<u32> = tex.indices.iter().map(|&i| tex.palette.get(usize::from(i)).copied().unwrap_or(0)).collect();
    let t = Texture { pixels: &expanded, pitch: tex.pitch, key: tex.key, flags: tex.flags };
    // Las ranuras 20 y 21 (modo 5 con paleta) apuntan a la copia 11: el modo 5 no se aplica.
    let base = select_routine(st.mode, st.color, tex.flags);
    let base = if base >= 9 { 0 } else { base };
    draw_with_routine(dst, &t, st, base, src, at, size).map(|r| r + 11)
}

/// Mezcla de un píxel en las rutinas de la copia escalada (`0x05014317` → `0x050932xx`), en las variantes MMX/SSE que
/// `0x05092FDF` instala al arrancar en cualquier CPU actual (`0x05093432`…`0x05096DC2`). Los cuatro canales, alfa
/// incluida, se tratan igual en palabras de 16 bits: `s = S·C >> 8`; las rutinas 2, 3 y 4 comparten código
/// (`0x05093F36`): `((s − D)·s_alfa` en 16 bits con signo `>> 8) + D`, recortado a 8 bits.
pub fn blend_scaled(routine: u8, s: u32, d: u32, c: u32, key: u32) -> Option<u32> {
    let (sb, db, cb) = (s.to_le_bytes(), d.to_le_bytes(), c.to_le_bytes());
    let m = |a: u8, b: u8| (u16::from(a) * u16::from(b)) >> 8;
    let sat = |v: u16| v.min(0xFF) as u8;
    let mut out = [0u8; 4];
    match routine {
        0 => return Some(s),
        1 => return (s != key & 0xFF_FFFF).then_some(s),
        2..=4 => {
            let sc: [u16; 4] = std::array::from_fn(|k| m(sb[k], cb[k]));
            let a = i32::from(sc[3]);
            for k in 0..4 {
                let t = ((i32::from(sc[k]) - i32::from(db[k])) * a) as i16 >> 8;
                out[k] = (i32::from(t) + i32::from(db[k])) as u8;
            }
        }
        5 => (0..4).for_each(|k| out[k] = sat(u16::from(db[k]) + m(sb[k], cb[k]))),
        6 => (0..4).for_each(|k| out[k] = sat(u16::from(sb[k]) + u16::from(db[k]))),
        7 => (0..4).for_each(|k| out[k] = m(m(sb[k], cb[k]) as u8, db[k]) as u8),
        8 => (0..4).for_each(|k| out[k] = m(sb[k], db[k]) as u8),
        9 | 10 => (0..4).for_each(|k| out[k] = sat((u16::from(sb[k]) * u16::from(db[k])) >> 7)),
        _ => return Some(s),
    }
    Some(u32::from_le_bytes(out))
}

/// `sgl_DrawBitmapRect(sx, sy, dx, dy, w, h, w2, h2)` con `w2×h2` distinto de `w×h` (`0x05014317`), sobre una textura de
/// `size` píxeles. Sin filtrado es el vecino más cercano: el origen avanza en 16.16 con paso `(w << 16) / w2` (truncado)
/// desde medio paso; la columna `x` del destino lee `sx + ((x·paso + paso/2) >> 16)`, y lo mismo en vertical. Con
/// `UseFiltering` (0x10) las copias 0, 8 y 9/10 usan el bilineal de [`filtered`] al ampliar (las demás no
/// filtran). Con `st.clip` recorta como `0x05092E18`/`0x05097A1C` ([`scaled_clip`]); sin él el destino debe caber.
#[allow(clippy::too_many_arguments)]
pub fn draw_scaled_rect(
    dst: &mut Surface,
    tex: &Texture,
    st: &BlitState,
    size: (u32, u32),
    (sx, sy): (i32, i32),
    (dx, dy): (i32, i32),
    (w, h): (i32, i32),
    (w2, h2): (i32, i32),
) -> Option<u8> {
    if w < 0 || h < 0 || w2 <= 0 || h2 <= 0 {
        return None;
    }
    let grows = ((w2 - w) | (h2 - h)) >= 0;
    let routine = select_routine(st.mode, st.color, tex.flags);
    let (stx, sty) = ((i64::from(w) << 16) / i64::from(w2), (i64::from(h) << 16) / i64::from(h2));
    let Some(([dx, dy, w2, h2], [skx, sky])) = scaled_clip(st.clip, (dx, dy), (w2, h2)) else {
        return Some(routine);
    };
    // 0x05092F32: inicio en 16.16 = origen + paso·saltadas + medio paso.
    let start = |s: i32, step: i64, skip: i32| ((s as u32) << 16).wrapping_add((step as u32).wrapping_mul(skip as u32)).wrapping_add((step as u32) >> 1);
    let (x0, y0) = (start(sx, stx, skx), start(sy, sty, sky));
    // 0x0501435B/0x050143DB/0x05014400: filtra solo si el destino no encoge en ningún eje ((w2 − w) | (h2 − h) ≥ 0),
    // con los tamaños de la llamada (antes del recorte).
    if tex.flags & 0x10 != 0 && grows && matches!(routine, 0 | 8..=10) {
        filtered(dst, tex, routine, size, (x0, y0), (dx, dy), (w2, h2), (stx, sty));
        return Some(routine);
    }
    for y in 0..h2 {
        let row = y0.wrapping_add((sty as u32).wrapping_mul(y as u32)) >> 16;
        let srow = row as usize * tex.pitch;
        let drow = (dy + y) as usize * dst.pitch;
        for x in 0..w2 {
            let col = x0.wrapping_add((stx as u32).wrapping_mul(x as u32)) >> 16;
            let s = tex.pixels[srow + col as usize];
            let d = &mut dst.pixels[drow + (dx + x) as usize];
            if let Some(v) = blend_scaled(routine, s, *d, st.color, tex.key) {
                *d = v;
            }
        }
    }
    Some(routine)
}

/// Recorte de la copia escalada (`0x05092E18`/`0x05097A1C`, con el byte `0x0508D084`): primero filas y luego columnas;
/// devuelve el destino recortado `[dx, dy, w2, h2]` y cuántas filas y columnas del destino se saltan. `None` si no queda
/// nada que dibujar.
fn scaled_clip(clip: Option<[i32; 4]>, (dx, dy): (i32, i32), (w2, h2): (i32, i32)) -> Option<([i32; 4], [i32; 2])> {
    let Some([l, t, r, b]) = clip else {
        return Some(([dx, dy, w2, h2], [0, 0]));
    };
    let axis = |d: i32, n: i32, lo: i32, span: i32| -> Option<(i32, i32, i32)> {
        let rel = d.wrapping_sub(lo);
        let (start, skip) = if rel < 0 { (lo, rel.wrapping_neg()) } else { (d, 0) };
        let room = span.wrapping_sub(rel);
        if room <= 0 {
            return None;
        }
        let mut n = n;
        if room.wrapping_sub(n) < 0 {
            n = room;
            if n <= 0 {
                return None;
            }
        }
        if rel < 0 {
            n = n.wrapping_add(rel);
            if n <= 0 {
                return None;
            }
        }
        Some((start, n, skip))
    };
    let (y, h2, sky) = axis(dy, h2, t, b.wrapping_sub(t))?;
    let (x, w2, skx) = axis(dx, w2, l, r.wrapping_sub(l))?;
    Some(([x, y, w2, h2], [skx, sky]))
}

/// Bilineal de las copias filtradas (versión SSE `0x05098F48`, que el controlador instala con MMX+SSE en `0x05097BD6`;
/// preparación en `0x05097A1C`). Las posiciones van en 17.15 (`((s << 16) + paso/2) >> 1`); cada canal se lleva a 15 bits
/// (`c << 7`), se interpola en vertical con `pmulhw` por la fracción de fila y en horizontal por la de columna, y se
/// toma `v = (pmulhw(D − I, fx) + (I >> 1)) >> 5`, alfa incluida: la copia (`0x05098F48`) lo satura; la 8 (`0x050981A0`)
/// y la 9/10 (`0x05098897`) lo multiplican antes por el destino (`>> 8` y `>> 7`). La última columna y fila de la
/// textura se repiten. El par vertical se recalcula solo cuando la fracción de columna no crece: con pasos de más de una
/// columna el original reutiliza el par anterior, y aquí también.
#[allow(clippy::too_many_arguments)]
fn filtered(
    dst: &mut Surface,
    tex: &Texture,
    routine: u8,
    (tw, th): (u32, u32),
    (x16, y16): (u32, u32),
    (dx, dy): (i32, i32),
    (w2, h2): (i32, i32),
    (stx, sty): (i64, i64),
) {
    let mulhw = |a: i32, b: i32| (a * b) >> 16;
    let chan = |p: u32, c: u32| ((p >> (8 * c)) & 0xFF) as i32;
    let (last_col, last_row) = (i64::from(tw) - 1, i64::from(th) - 1);
    let xstep = ((stx as u32) >> 1) as i64;
    let ystep = ((sty as u32) >> 1) as i64;
    let x0 = i64::from(x16 >> 1);
    let mut yacc = i64::from(y16 >> 1);
    for y in 0..i64::from(h2) {
        let fy = (yacc & 0x7FFF) as i32;
        let row = yacc >> 15;
        yacc += ystep;
        let r0 = row as usize * tex.pitch;
        let r1 = if row < last_row { r0 + tex.pitch } else { r0 };
        let drow = (i64::from(dy) + y) as usize * dst.pitch;
        let mut xacc = x0;
        let mut pair = [[0i32; 4]; 2];
        let mut refetch = true;
        for x in 0..i64::from(w2) {
            if refetch {
                let col = xacc >> 15;
                let c1 = if col == last_col { col } else { col + 1 } as usize;
                let col = col as usize;
                let at = |r: usize, c: usize| tex.pixels.get(r + c).copied().unwrap_or(0);
                for c in 0..4 {
                    let side = |cc: usize| {
                        let (a, b) = (chan(at(r0, cc), c) << 7, chan(at(r1, cc), c) << 7);
                        mulhw(b - a, fy) + (a >> 1)
                    };
                    pair[0][c as usize] = side(col);
                    pair[1][c as usize] = side(c1);
                }
            }
            let fx = (xacc & 0x7FFF) as i32;
            let d = &mut dst.pixels[drow + (i64::from(dx) + x) as usize];
            let db = d.to_le_bytes();
            let mut out = 0u32;
            for c in 0..4 {
                let (l, r) = (pair[0][c], pair[1][c]);
                let v = (mulhw(r - l, fx) + (l >> 1)) as i16 >> 5;
                // Copia: `packuswb` directo; 8 y 9/10: `pmullw` por el destino y `psrlw` 8 o 7 antes de saturar.
                let w = match routine {
                    0 => v,
                    8 => ((v as u16).wrapping_mul(u16::from(db[c])) >> 8) as i16,
                    _ => ((v as u16).wrapping_mul(u16::from(db[c])) >> 7) as i16,
                };
                out |= (w.clamp(0, 255) as u32) << (8 * c);
            }
            *d = out;
            xacc += xstep;
            refetch = (xacc & 0x7FFF) as i32 <= fx;
        }
    }
}

/// Preparación de una línea (`0x05091490`): primer píxel, paso 0.32 del eje menor, acumulador, avances con y sin acarreo
/// (en bytes, con el `pitch` de la superficie) y número de píxeles. `None` si el original dividiría con desbordamiento.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LineSetup {
    x: i32,
    y: i32,
    step: u32,
    acc: u32,
    carry: i32,
    plain: i32,
    count: i32,
}

/// `div` de 64 entre 32 bits del x86 (excepción si el cociente no cabe).
fn div64(hi: i32, lo: u32, d: u32) -> Option<u32> {
    let n = (u64::from(hi as u32) << 32) | u64::from(lo);
    let q = n.checked_div(u64::from(d))?;
    u32::try_from(q).ok()
}

/// `mul` seguido de `add 0x80000000`/`adc`: (bajo, alto) de `k·paso + 2³¹`.
fn mul_half(k: i32, step: u32) -> (u32, i32) {
    let v = u64::from(k as u32) * u64::from(step) + 0x8000_0000;
    (v as u32, (v >> 32) as i32)
}

/// Transcripción de `0x05091490` con recorte (`left`, `top`, `right`, `bottom` exclusivos) o sin él.
fn line_setup(mut p: [i32; 4], clip: Option<[i32; 4]>, pitch: i32) -> Option<Option<LineSetup>> {
    if p[0] > p[2] {
        p = [p[2], p[3], p[0], p[1]];
    }
    let [x1, y1, x2, y2] = p;
    let up = y1 > y2;
    let dx = x2.wrapping_sub(x1);
    let dy = if up { y1.wrapping_sub(y2) } else { y2.wrapping_sub(y1) };
    let vstep = if up { pitch.wrapping_neg() } else { pitch };
    let Some([l, t, r, b]) = clip else {
        // 0x05091B6F: sin recorte, acumulador a cero.
        let (step, carry, plain, count) = match dx.cmp(&dy) {
            std::cmp::Ordering::Equal => (0, 0, vstep.wrapping_add(4), dx.wrapping_add(1)),
            std::cmp::Ordering::Greater => (div64(dy, 0, dx as u32)?, vstep.wrapping_add(4), 4, dx.wrapping_add(1)),
            std::cmp::Ordering::Less => (div64(dx, 0, dy as u32)?, vstep.wrapping_add(4), vstep, dy.wrapping_add(1)),
        };
        return Some(Some(LineSetup { x: x1, y: y1, step, acc: 0, carry, plain, count }));
    };
    let (r1, b1) = (r.wrapping_sub(1), b.wrapping_sub(1));
    let neg = |v: i32| v < 0;
    // Rechazo por signo de las diferencias (`lahf` + bit 15).
    let reject = neg(r1.wrapping_sub(x1))
        || neg(x2.wrapping_sub(l))
        || if up { neg(b1.wrapping_sub(y2)) || neg(y1.wrapping_sub(t)) } else { neg(b1.wrapping_sub(y1)) || neg(y2.wrapping_sub(t)) };
    if reject {
        return Some(None);
    }
    let (mut a14, mut a18, mut a1c, mut a20) = (0i32, 0i32, 0i32, 0i32);
    let pos = |v: i32| v > 0;
    let s = match (up, dx.cmp(&dy)) {
        (true, std::cmp::Ordering::Equal) => {
            if pos(l.wrapping_sub(x1)) {
                a14 = l.wrapping_sub(x1);
                a18 = a14;
            }
            if pos(t.wrapping_sub(y2)) {
                a1c = t.wrapping_sub(y2);
                a20 = a1c;
            }
            if pos(x2.wrapping_sub(r1).wrapping_sub(a1c)) {
                a1c = x2.wrapping_sub(r1);
                a20 = a1c;
            }
            if pos(y1.wrapping_sub(b1).wrapping_sub(a18)) {
                a14 = y1.wrapping_sub(b1);
                a18 = a14;
            }
            LineSetup { x: x1.wrapping_add(a14), y: y1.wrapping_sub(a18), step: 0, acc: 0, carry: 0, plain: 4 - pitch, count: dx - a14 - a1c + 1 }
        }
        (true, std::cmp::Ordering::Greater) => {
            let step = div64(dy, 0, dx as u32)?;
            let mut acc = 0x8000_0000u32;
            if pos(l.wrapping_sub(x1)) {
                a14 = l.wrapping_sub(x1);
                (acc, a18) = mul_half(a14, step);
            }
            if pos(y1.wrapping_sub(b1).wrapping_sub(a18)) {
                a14 = div64(y1.wrapping_sub(b1).wrapping_sub(1), 0x8000_0000, step)?.wrapping_add(1) as i32;
                (acc, a18) = mul_half(a14, step);
            }
            if pos(t.wrapping_sub(y2)) {
                a20 = t.wrapping_sub(y2);
                let q = div64(y1.wrapping_sub(t), 0x7FFF_FFFF, step)?;
                a1c = x2.wrapping_sub(x1).wrapping_sub(q as i32);
            }
            if pos(x2.wrapping_sub(r1).wrapping_sub(a1c)) {
                a1c = x2.wrapping_sub(r1);
            }
            LineSetup { x: x1.wrapping_add(a14), y: y1.wrapping_sub(a18), step, acc, carry: 4 - pitch, plain: 4, count: dx - a14 - a1c + 1 }
        }
        (true, std::cmp::Ordering::Less) => {
            let step = div64(dx, 0, dy as u32)?;
            let mut acc = 0x8000_0000u32;
            if pos(l.wrapping_sub(x1)) {
                a18 = div64(l.wrapping_sub(x1).wrapping_sub(1), 0x8000_0000, step)?.wrapping_add(1) as i32;
                (acc, a14) = mul_half(a18, step);
            }
            if pos(y1.wrapping_sub(b1).wrapping_sub(a18)) {
                a18 = y1.wrapping_sub(b1);
                (acc, a14) = mul_half(a18, step);
            }
            if pos(x2.wrapping_sub(r1)) {
                a1c = x2.wrapping_sub(r1);
                let q = div64(r1.wrapping_sub(x1), 0x7FFF_FFFF, step)?;
                a20 = y1.wrapping_sub(y2).wrapping_sub(q as i32);
            }
            if pos(t.wrapping_sub(y2).wrapping_sub(a20)) {
                a20 = t.wrapping_sub(y2);
            }
            LineSetup { x: x1.wrapping_add(a14), y: y1.wrapping_sub(a18), step, acc, carry: 4 - pitch, plain: -pitch, count: dy - a18 - a20 + 1 }
        }
        (false, std::cmp::Ordering::Equal) => {
            if pos(l.wrapping_sub(x1)) {
                a14 = l.wrapping_sub(x1);
                a18 = a14;
            }
            if pos(t.wrapping_sub(y1).wrapping_sub(a18)) {
                a14 = t.wrapping_sub(y1);
                a18 = a14;
            }
            if pos(x2.wrapping_sub(r1)) {
                a1c = x2.wrapping_sub(r1);
                a20 = a1c;
            }
            if pos(y2.wrapping_sub(b1).wrapping_sub(a20)) {
                a1c = y2.wrapping_sub(b1);
                a20 = a1c;
            }
            LineSetup { x: x1.wrapping_add(a14), y: y1.wrapping_add(a18), step: 0, acc: 0, carry: 0, plain: pitch + 4, count: dx - a1c - a14 + 1 }
        }
        (false, std::cmp::Ordering::Greater) => {
            let step = div64(dy, 0, dx as u32)?;
            let mut acc = 0x8000_0000u32;
            if pos(l.wrapping_sub(x1)) {
                a14 = l.wrapping_sub(x1);
                (acc, a18) = mul_half(a14, step);
            }
            if pos(t.wrapping_sub(y1).wrapping_sub(a18)) {
                a14 = div64(t.wrapping_sub(y1).wrapping_sub(1), 0x8000_0000, step)?.wrapping_add(1) as i32;
                (acc, a18) = mul_half(a14, step);
            }
            if pos(y2.wrapping_sub(b1).wrapping_sub(a20)) {
                a20 = y2.wrapping_sub(b1);
                let q = div64(b1.wrapping_sub(y1), 0x7FFF_FFFF, step)?;
                a1c = x2.wrapping_sub(x1).wrapping_sub(q as i32);
            }
            if pos(x2.wrapping_sub(r1).wrapping_sub(a1c)) {
                a1c = x2.wrapping_sub(r1);
            }
            LineSetup { x: x1.wrapping_add(a14), y: y1.wrapping_add(a18), step, acc, carry: pitch + 4, plain: 4, count: dx - a14 - a1c + 1 }
        }
        (false, std::cmp::Ordering::Less) => {
            let step = div64(dx, 0, dy as u32)?;
            let mut acc = 0x8000_0000u32;
            if pos(t.wrapping_sub(y1)) {
                a18 = t.wrapping_sub(y1);
                (acc, a14) = mul_half(a18, step);
            }
            if pos(l.wrapping_sub(x1).wrapping_sub(a14)) {
                a18 = div64(l.wrapping_sub(x1).wrapping_sub(1), 0x8000_0000, step)?.wrapping_add(1) as i32;
                (acc, a14) = mul_half(a18, step);
            }
            if pos(x2.wrapping_sub(r1)) {
                a1c = x2.wrapping_sub(r1);
                let q = div64(r1.wrapping_sub(x1), 0x7FFF_FFFF, step)?;
                a20 = y2.wrapping_sub(y1).wrapping_sub(q as i32);
            }
            if pos(y2.wrapping_sub(b1).wrapping_sub(a20)) {
                a20 = y2.wrapping_sub(b1);
            }
            LineSetup { x: x1.wrapping_add(a14), y: y1.wrapping_add(a18), step, acc, carry: pitch + 4, plain: pitch, count: dy - a18 - a20 + 1 }
        }
    };
    Some(Some(s))
}

/// Mezcla de un píxel de línea (`0x05091154`, variantes MMX): modo 3 suma saturada con el alfa a 0xFF, modo 2 producto
/// por canal (`>> 8`, alfa incluida), color opaco copiado y, si no, `D·(255 − a) >> 8 + C·a >> 8` saturado, alfa a 0xFF.
pub fn blend_line(mode: u8, color: u32, d: u32) -> u32 {
    let (cb, db) = (color.to_le_bytes(), d.to_le_bytes());
    let a = u16::from(cb[3]);
    let mut out = [0u8; 4];
    match mode {
        3 => (0..4).for_each(|k| out[k] = db[k].saturating_add(if k == 3 { 0xFF } else { cb[k] })),
        2 => (0..4).for_each(|k| out[k] = ((u16::from(db[k]) * u16::from(cb[k])) >> 8) as u8),
        _ if a == 0xFF => return color,
        _ => {
            for k in 0..4 {
                let c = if k == 3 { 0xFF } else { (u16::from(cb[k]) * a) >> 8 };
                out[k] = (((u16::from(db[k]) * (0xFF - a)) >> 8) + c).min(0xFF) as u8;
            }
        }
    }
    u32::from_le_bytes(out)
}

/// Línea entera en una región de recorte (`0x05015080` → `0x05091154`): `mode` es `sgl_SetBlendMode`, `color` ARGB y
/// `clip` el rectángulo activo (`None` = byte `0x0508D084` a cero: el original no recorta). Devuelve `false` si el
/// original habría fallado al dividir.
pub fn draw_line_int(dst: &mut Surface, p: [i32; 4], mode: u8, color: u32, clip: Option<[i32; 4]>) -> bool {
    let pitch = (dst.pitch * 4) as i32;
    let Some(setup) = line_setup(p, clip, pitch) else {
        return false;
    };
    let Some(s) = setup else {
        return true;
    };
    if s.count <= 0 {
        return true;
    }
    let mut at = i64::from(s.x) * 4 + i64::from(s.y) * i64::from(pitch);
    let mut acc = s.acc;
    for _ in 0..s.count {
        if let Some(px) = usize::try_from(at / 4).ok().filter(|_| at % 4 == 0).and_then(|i| dst.pixels.get_mut(i)) {
            *px = blend_line(mode, color, *px);
        }
        let (n, c) = acc.overflowing_add(s.step);
        acc = n;
        at += i64::from(if c { s.carry } else { s.plain });
    }
    true
}

/// `sgl_DrawLine` (`0x0500F400` → `0x05015130`) con una sola región de recorte: las coordenadas se redondean como
/// `fistp` (al par más cercano).
pub fn draw_line(dst: &mut Surface, (x1, y1, x2, y2): (f32, f32, f32, f32), mode: u8, color: u32, clip: Option<[i32; 4]>) -> bool {
    let r = |v: f32| v.round_ties_even() as i32;
    draw_line_int(dst, [r(x1), r(y1), r(x2), r(y2)], mode, color, clip)
}
