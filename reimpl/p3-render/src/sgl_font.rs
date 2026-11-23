//! Fuentes de mapa de bits del controlador SGL (`ddraw_Dll.dll`): `sgl_MemoryFont_CreateProportional` (`0x0500F4A0`) y
//! `sgl_MemoryFont_CreateKerning` (`0x0500F500`), que el EXE crea en `0x004C1440` con las líneas `[FontN]` de
//! `scripts/fonts.ini` (`Params=ancho alto espaciado`, `Type=Kerning`) sobre la textura `.aim` de la fuente y el juego de
//! caracteres de `0x006BF340`; el ancho de un texto (`0x0500F1E0`) y su dibujo (`sgl_DrawText` `0x0500F140` →
//! `0x0500EDB0`). Contrastado con `oracle/x86_sgl_font.py`.
//!
//! Objeto de fuente: `+4` ancho de celda, `+8` alto, `+0xC` −1 (proporcional/kerning), `+0x10` glifos, `+0x14` anchos,
//! `+0x18` tabla de kerning (`glifos × glifos`), `+0x20` carácter → glifo, `+0x120` espaciado, `+0x124` ancho por defecto.

/// Juego de caracteres que el EXE pasa a las fuentes (`0x006BF340` → `0x006BF2C8`): el glifo `i` de la tira es el carácter
/// `i` de esta cadena (latín-1).
pub const GAME_CHARSET: &[u8] = b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\xb0\xe4\xf6\xfc\xc4\xd6\xdc\xdf\xe1\xe9\xed\xf3\xfa\xe0\xe8\xec\xf2\xf9\xe2\xea\xee\xf4\xfb";

/// Caracteres sin kerning (`[0x0505F2C8]`): con ellos el avance es el ancho más el espaciado.
pub const NO_KERNING: &[u8] = b"+-*/\\#~!\"\xa7$%%&/()= ";

/// Textura bloqueada (`vtable+4` de la textura): BGRA de 32 bits.
#[derive(Clone, Copy, Debug)]
pub struct Locked<'a> {
    pub pixels: &'a [u8],
    /// Ancho en píxeles (campo `+8` del bloqueo): fija cuántos glifos caben.
    pub width: u32,
    /// Bytes por fila (`+0x10`).
    pub pitch: usize,
}

impl Locked<'_> {
    fn byte(&self, at: usize) -> u8 {
        self.pixels.get(at).copied().unwrap_or(0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Font {
    pub cell: (u32, u32),
    pub count: u32,
    pub widths: Vec<u8>,
    pub kerning: Option<Vec<u8>>,
    pub map: [u8; 256],
}

/// `0x0500EC60`: glifo de cada carácter de `charset`, como mucho `ancho de textura / ancho de celda` (división sin signo);
/// los demás caracteres quedan en el glifo 0.
fn char_map(charset: &[u8], tex_width: u32, cell_w: u32) -> [u8; 256] {
    let mut map = [0u8; 256];
    let n = tex_width.checked_div(cell_w).unwrap_or(0);
    let mut k = 0u8;
    while let Some(&c) = charset.get(usize::from(k)) {
        if c == 0 || u32::from(k) >= n {
            break;
        }
        map[usize::from(c)] = k;
        k = k.wrapping_add(1);
    }
    map
}

/// `sgl_MemoryFont_CreateProportional`/`CreateKerning` (`0x0500E190`/`0x0500E210` con `0x0500EC60` y `0x0500E7E0`).
pub fn create(charset: &[u8], tex: &Locked, cell_w: u32, cell_h: u32, spacing: i32, default_width: u8, kerning: bool) -> Font {
    let map = char_map(charset, tex.width, cell_w);
    let count = u32::from(*map.iter().max().unwrap()) + 1;
    let n = count as usize;
    let sp = spacing as u8;
    let mut widths = vec![0u8; n];
    // Anchos: columna más a la derecha con G+R+A (suma de 8 bits) distinta de cero.
    for (g, w) in widths.iter_mut().enumerate() {
        let base = g * cell_w as usize * 4;
        let mut right = 0u32;
        for row in 0..cell_h as usize {
            let p = base + row * tex.pitch;
            for x in 0..cell_w {
                let at = p + 4 * x as usize;
                let s = tex.byte(at + 1).wrapping_add(tex.byte(at + 3)).wrapping_add(tex.byte(at + 2));
                if s != 0 && x > right {
                    right = x;
                }
            }
        }
        *w = sp.wrapping_add(right as u8);
    }
    for w in widths.iter_mut() {
        if i32::from(*w) <= spacing {
            *w = default_width;
        }
    }
    let kern = kerning.then(|| {
        let stride = 2 * cell_h as usize;
        let mut profile = vec![0u8; n * stride];
        let mut kernable = [1u8; 256];
        for &c in NO_KERNING {
            kernable[usize::from(map[usize::from(c)])] = 0;
        }
        // Perfil de cada fila: primera columna con algún byte (izquierda) y distancia de la última al ancho (derecha).
        let mut k = 0;
        for (g, &w) in widths.iter().enumerate() {
            let mut row_at = g * cell_w as usize * 4;
            for _ in 0..cell_h {
                let (mut left, mut right) = (w, w);
                for x in 0..=u32::from(w) {
                    let at = row_at + 4 * x as usize;
                    let s = u32::from(tex.byte(at)) + u32::from(tex.byte(at + 1)) + u32::from(tex.byte(at + 2)) + u32::from(tex.byte(at + 3));
                    if s != 0 {
                        if x < u32::from(left) {
                            left = x as u8;
                        }
                        if u32::from(w) - x < u32::from(right) {
                            right = w.wrapping_sub(x as u8);
                        }
                    }
                }
                profile[k] = left;
                profile[k + 1] = right;
                k += 2;
                row_at += tex.pitch;
            }
        }
        let mut table = vec![0u8; n * n];
        for a in 0..n {
            for b in 0..n {
                table[a * n + b] = if kernable[a] + kernable[b] == 2 {
                    let wa = widths[a];
                    let mut gap = wa;
                    for row in 0..cell_h as usize {
                        let v = profile[b * stride + 2 * row].wrapping_add(profile[a * stride + 2 * row + 1]);
                        if v < gap {
                            gap = v;
                        }
                    }
                    sp.wrapping_sub(gap).wrapping_add(wa).wrapping_add(1)
                } else {
                    widths[a].wrapping_add(sp)
                };
            }
        }
        table
    });
    for w in widths.iter_mut() {
        *w = w.wrapping_add(1);
    }
    Font { cell: (cell_w, cell_h), count, widths, kerning: kern, map }
}

impl Font {
    /// Glifo y avance de cada carácter (el avance mira el glifo siguiente en la tabla de kerning si no es el 0).
    fn steps<'s>(&'s self, s: &'s [u8]) -> impl Iterator<Item = (u8, u8)> + 's {
        let n = self.count as usize;
        (0..s.len()).map(move |i| {
            let g = self.map[usize::from(s[i])];
            let next = self.map[usize::from(s.get(i + 1).copied().unwrap_or(0))];
            let adv = match (&self.kerning, next) {
                (Some(k), nx) if nx != 0 => k.get(usize::from(g) * n + usize::from(nx)).copied().unwrap_or(0),
                _ => self.widths.get(usize::from(g)).copied().unwrap_or(0),
            };
            (g, adv)
        })
    }

    /// `0x0500F1E0`: ancho del texto (hasta el primer cero).
    pub fn text_width(&self, s: &[u8]) -> i32 {
        let s = s.split(|&c| c == 0).next().unwrap_or(&[]);
        self.steps(s).fold(0u32, |acc, (_, adv)| acc.wrapping_add(u32::from(adv))) as i32
    }

    /// `sgl_DrawText(x, y, texto)` con el modo de `sgl_SetTextMode` (2 derecha, 3 centrado, otro izquierda): copias de la
    /// textura de la fuente `(sx, sy, dx, dy, ancho, alto)` en orden, cada glifo con su ancho y el alto de celda.
    pub fn draw(&self, x: i32, y: i32, s: &[u8], mode: i32) -> Vec<[i32; 6]> {
        let s = s.split(|&c| c == 0).next().unwrap_or(&[]);
        if s.is_empty() {
            return Vec::new();
        }
        let width = self.text_width(s);
        let mut x = match mode {
            2 => x.wrapping_sub(width),
            3 => x.wrapping_sub(width >> 1),
            _ => x,
        };
        let mut out = Vec::with_capacity(s.len());
        for (g, adv) in self.steps(s) {
            let w = i32::from(self.widths.get(usize::from(g)).copied().unwrap_or(0));
            out.push([(self.cell.0 as i32).wrapping_mul(i32::from(g)), 0, x, y, w, self.cell.1 as i32]);
            x = x.wrapping_add(i32::from(adv));
        }
        out
    }
}
