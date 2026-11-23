//! Caminos rápidos de la composición de la ciudad (`Scene::render`): copias por filas ya recortadas, franjas del lienzo en
//! paralelo y relleno de huecos por tramos. Todo produce exactamente los mismos píxeles que la versión píxel a píxel.
//!
//! El número de hilos sale de `std::thread::available_parallelism` (como mucho 8) o de la variable `P3_RENDER_HILOS`
//! (1 = sin hilos, útil para perfilar).
use crate::Texture;

/// Hilos de trabajo para decodificar texturas y componer franjas.
pub(crate) fn threads() -> usize {
    if let Some(n) = std::env::var("P3_RENDER_HILOS").ok().and_then(|v| v.parse::<usize>().ok()) {
        return n.clamp(1, 64);
    }
    // WASM (`wasm32-unknown-unknown`) no tiene hilos: un solo hilo, composición en línea.
    if cfg!(target_arch = "wasm32") {
        return 1;
    }
    std::thread::available_parallelism().map_or(1, |n| n.get()).min(8)
}

/// Dibujo pendiente de una textura sobre el lienzo: rectángulo `frame` de la textura `tex` con destino `(dx, dy)`.
#[derive(Clone, Copy)]
pub(crate) struct Op {
    pub tex: usize,
    pub frame: [i16; 4],
    pub dx: i32,
    pub dy: i32,
}

/// Tramo `[lo, hi)` de `0..len` cuyo origen `src0 + i` cae en `0..src_len` y cuyo destino `dst0 + i` cae en `dst_lo..dst_hi`.
fn clip(len: i64, src0: i64, src_len: i64, dst0: i64, dst_lo: i64, dst_hi: i64) -> (i64, i64) {
    let lo = 0.max(-src0).max(dst_lo - dst0);
    let hi = len.min(src_len - src0).min(dst_hi - dst0);
    (lo, hi)
}

/// Copia con clave de transparencia (alfa 0 no se dibuja) sobre la franja `band`, que contiene las filas `y0..` del lienzo
/// de ancho `cw`. Equivale a recorrer el rectángulo píxel a píxel descartando lo que cae fuera de la textura o de la franja.
pub(crate) fn blit_band(band: &mut [u8], cw: usize, y0: usize, tex: &Texture, frame: [i16; 4], dx: i32, dy: i32) {
    let rows = band.len() / (cw.max(1) * 4);
    let (fx, fy, fw, fh) = (i64::from(frame[0]), i64::from(frame[1]), i64::from(frame[2]), i64::from(frame[3]));
    let (i0, i1) = clip(fw, fx, tex.width as i64, i64::from(dx), 0, cw as i64);
    let (j0, j1) = clip(fh, fy, tex.height as i64, i64::from(dy), y0 as i64, (y0 + rows) as i64);
    if i0 >= i1 || j0 >= j1 {
        return;
    }
    let n = (i1 - i0) as usize * 4;
    for j in j0..j1 {
        let s = ((fy + j) as usize * tex.width + (fx + i0) as usize) * 4;
        let d = ((i64::from(dy) + j) as usize - y0) * cw * 4 + (i64::from(dx) + i0) as usize * 4;
        let src = &tex.rgba[s..s + n];
        let dst = &mut band[d..d + n];
        for (o, p) in dst.chunks_exact_mut(4).zip(src.chunks_exact(4)) {
            if p[3] != 0 {
                o.copy_from_slice(p);
            }
        }
    }
}

/// Mezcla por el alfa de la textura multiplicado por `fade` (0..=255), con el mismo redondeo que la versión píxel a píxel.
pub(crate) fn blit_faded(canvas: &mut [u8], cw: usize, ch: usize, tex: &Texture, frame: [i16; 4], dx: i32, dy: i32, fade: u8) {
    let (fx, fy, fw, fh) = (i64::from(frame[0]), i64::from(frame[1]), i64::from(frame[2]), i64::from(frame[3]));
    let (i0, i1) = clip(fw, fx, tex.width as i64, i64::from(dx), 0, cw as i64);
    let (j0, j1) = clip(fh, fy, tex.height as i64, i64::from(dy), 0, ch as i64);
    if i0 >= i1 || j0 >= j1 {
        return;
    }
    let n = (i1 - i0) as usize * 4;
    let fade = u32::from(fade);
    for j in j0..j1 {
        let s = ((fy + j) as usize * tex.width + (fx + i0) as usize) * 4;
        let d = ((i64::from(dy) + j) as usize * cw + (i64::from(dx) + i0) as usize) * 4;
        let src = &tex.rgba[s..s + n];
        let dst = &mut canvas[d..d + n];
        for (o, p) in dst.chunks_exact_mut(4).zip(src.chunks_exact(4)) {
            let a = u32::from(p[3]) * fade / 255;
            if a == 0 {
                continue;
            }
            if a == 255 {
                // (c * 0 + t * 255) / 255 = t.
                o[..3].copy_from_slice(&p[..3]);
            } else {
                for k in 0..3 {
                    o[k] = ((u32::from(o[k]) * (255 - a) + u32::from(p[k]) * a) / 255) as u8;
                }
            }
            o[3] = 255;
        }
    }
}

/// Tramos horizontales de píxeles negros (`R = G = B = 0`) de las filas de `band` (filas `y0..` del lienzo): `(fila, x0, x1)`
/// con `x1` excluido, ordenados por fila y columna.
pub(crate) fn black_runs(band: &[u8], cw: usize, y0: usize, out: &mut Vec<(u32, u32, u32)>) {
    if cw == 0 {
        return;
    }
    // Píxel negro: los tres primeros bytes (R, G, B) a cero, sea cual sea el alfa.
    let rgb = |p: &[u8]| u32::from_le_bytes([p[0], p[1], p[2], 0]);
    for (r, row) in band.chunks_exact(cw * 4).enumerate() {
        let y = (y0 + r) as u32;
        let mut start = None;
        for (x, p) in row.chunks_exact(4).enumerate() {
            if rgb(p) == 0 {
                if start.is_none() {
                    start = Some(x as u32);
                }
            } else if let Some(s) = start.take() {
                out.push((y, s, x as u32));
            }
        }
        if let Some(s) = start {
            out.push((y, s, cw as u32));
        }
    }
}

/// Rellena de `[0, 0, 0, 255]` la franja, dibuja en orden las operaciones que la tocan y devuelve sus tramos negros.
fn compose_band(band: &mut [u8], cw: usize, y0: usize, ops: &[Op], texs: &[Option<&Texture>]) -> Vec<(u32, u32, u32)> {
    for px in band.chunks_exact_mut(4) {
        px.copy_from_slice(&[0, 0, 0, 255]);
    }
    let rows = band.len() / (cw.max(1) * 4);
    let (lo, hi) = (y0 as i64, (y0 + rows) as i64);
    for op in ops {
        let top = i64::from(op.dy);
        if top >= hi || top + i64::from(op.frame[3]) <= lo {
            continue;
        }
        if let Some(tex) = texs[op.tex] {
            blit_band(band, cw, y0, tex, op.frame, op.dx, op.dy);
        }
    }
    let mut runs = Vec::new();
    black_runs(band, cw, y0, &mut runs);
    runs
}

/// Compone el lienzo `width × height` (recién reservado, a ceros) con las operaciones en orden y devuelve los tramos negros
/// resultantes. Cada franja horizontal la compone un hilo: como todas aplican las mismas operaciones en el mismo orden,
/// recortadas a sus filas, el resultado no depende del número de franjas.
pub(crate) fn compose(canvas: &mut [u8], width: usize, height: usize, ops: &[Op], texs: &[Option<&Texture>]) -> Vec<(u32, u32, u32)> {
    let n = threads().min(height.max(1));
    if n <= 1 || width == 0 {
        return compose_band(canvas, width, 0, ops, texs);
    }
    let rows_per = height.div_ceil(n);
    let mut parts: Vec<Vec<(u32, u32, u32)>> = Vec::new();
    std::thread::scope(|s| {
        let handles: Vec<_> = canvas
            .chunks_mut(rows_per * width * 4)
            .enumerate()
            .map(|(i, band)| s.spawn(move || compose_band(band, width, i * rows_per, ops, texs)))
            .collect();
        parts = handles.into_iter().map(|h| h.join().expect("franja de la ciudad")).collect();
    });
    parts.concat()
}

/// Ejecuta `job(i)` para `i` en `0..count` repartido entre hilos y devuelve los resultados en orden.
pub(crate) fn parallel_map<T: Send>(count: usize, job: impl Fn(usize) -> T + Sync) -> Vec<T> {
    let n = threads().min(count);
    if n <= 1 {
        return (0..count).map(job).collect();
    }
    let next = std::sync::atomic::AtomicUsize::new(0);
    let mut out: Vec<Option<T>> = (0..count).map(|_| None).collect();
    std::thread::scope(|s| {
        let handles: Vec<_> = (0..n)
            .map(|_| {
                s.spawn(|| {
                    let mut done = Vec::new();
                    loop {
                        let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if i >= count {
                            break;
                        }
                        done.push((i, job(i)));
                    }
                    done
                })
            })
            .collect();
        for h in handles {
            for (i, v) in h.join().expect("trabajo en paralelo") {
                out[i] = Some(v);
            }
        }
    });
    out.into_iter().map(|v| v.expect("resultado")).collect()
}

/// Relleno de huecos de [`crate::fill_holes`] a partir de los tramos negros del lienzo (`black_runs` de todas sus filas).
///
/// Un hueco es una componente de píxeles negros unidos por los lados; dos tramos de filas contiguas pertenecen a la misma
/// si comparten columna. Se rellenan las componentes de hasta `MAX_HOLE` píxeles que no tocan el margen del lienzo.
pub(crate) fn fill_holes_runs(canvas: &mut [u8], width: usize, height: usize, runs: &[(u32, u32, u32)]) {
    const MAX_HOLE: usize = 4000;
    const MARGIN: usize = 80;
    fn find(parent: &mut [u32], mut i: u32) -> u32 {
        while parent[i as usize] != i {
            parent[i as usize] = parent[parent[i as usize] as usize];
            i = parent[i as usize];
        }
        i
    }
    let mut parent: Vec<u32> = (0..runs.len() as u32).collect();
    // Unión de cada tramo con los de la fila anterior que comparten columna (dos punteros sobre tramos ordenados).
    let mut prev = 0..0;
    let mut i = 0;
    while i < runs.len() {
        let y = runs[i].0;
        let mut end = i;
        while end < runs.len() && runs[end].0 == y {
            end += 1;
        }
        if prev.end == i && !prev.is_empty() && runs[prev.start].0 + 1 == y {
            let (mut a, mut b) = (prev.start, i);
            while a < prev.end && b < end {
                let (ra, rb) = (runs[a], runs[b]);
                if ra.1 < rb.2 && rb.1 < ra.2 {
                    let (pa, pb) = (find(&mut parent, a as u32), find(&mut parent, b as u32));
                    if pa != pb {
                        parent[pb as usize] = pa;
                    }
                }
                if ra.2 <= rb.2 {
                    a += 1;
                } else {
                    b += 1;
                }
            }
        }
        prev = i..end;
        i = end;
    }
    // Tamaño de cada componente y si toca el margen.
    let mut size = vec![0usize; runs.len()];
    let mut touches = vec![false; runs.len()];
    for (k, &(y, x0, x1)) in runs.iter().enumerate() {
        let root = find(&mut parent, k as u32) as usize;
        size[root] += (x1 - x0) as usize;
        let (y, x0, x1) = (y as usize, x0 as usize, x1 as usize);
        if y < MARGIN || y + MARGIN >= height || x0 < MARGIN || (x1 - 1) + MARGIN >= width {
            touches[root] = true;
        }
    }
    // El color sale de píxeles que no eran negros antes de rellenar nada (y que por tanto no cambian): primero se eligen
    // todos los orígenes sobre el lienzo intacto y después se copia.
    let mut copies: Vec<(usize, usize)> = Vec::new();
    for (k, &(y, x0, x1)) in runs.iter().enumerate() {
        let root = find(&mut parent, k as u32) as usize;
        if touches[root] || size[root] > MAX_HOLE {
            continue;
        }
        for x in x0..x1 {
            let (x, y) = (x as i32, y as i32);
            'found: for k in 1..=3 {
                for (ox, oy) in [(-34, 0), (34, 0), (0, -22), (0, 22), (-17, -11), (17, -11), (-17, 11), (17, 11)] {
                    let (sx, sy) = (x + ox * k, y + oy * k);
                    if sx < 0 || sy < 0 || sx as usize >= width || sy as usize >= height {
                        continue;
                    }
                    let n = sy as usize * width + sx as usize;
                    if canvas[n * 4..n * 4 + 3] != [0, 0, 0] {
                        copies.push((n, y as usize * width + x as usize));
                        break 'found;
                    }
                }
            }
        }
    }
    for (from, to) in copies {
        canvas.copy_within(from * 4..from * 4 + 4, to * 4);
    }
}
