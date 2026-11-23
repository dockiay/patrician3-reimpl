//! `ShipListWindow` (`0x475B90`–`0x477790`): regiones sucias de las dos tablas (`+0x838` y `+0x874`) y destrucción de los botones de fila.

use super::{call, Host, Mem, Ret};
use crate::j07::vcall;

const DIRTY: u32 = 0x006D_A5F0;

/// Llama a un método de `TableLayout` (el porte si existe) con una pila de llamado inventada por debajo de `esp`.
fn tcall(m: &mut dyn Mem, h: &mut dyn Host, f: u32, t: u32, a: &[u32], esp: u32) -> u32 {
    call(m, h, f, t, a, esp.wrapping_sub(0x60))
}


const SHIPS: u32 = 0x006D_D7A4;

/// Ordenación por rango de `ShipListWindow` (`0x477DA0` salud, `0x477F70` artillería): selección por clave (función del original,
/// sentido en el byte bajo de `a[2]`), grupos de clave igual y orden por nombre dentro de cada grupo.
fn range_sort(m: &mut dyn Mem, h: &mut dyn Host, t: u32, a: &[u32], health: bool) {
    let (lo, hi, dir) = (a[0], a[1], a[2] & 0xFF);
    if lo >= hi {
        return;
    }
    let arr = m.r32(t + 0x8F4);
    let keyfn = if health { 0x495340 } else { 0x495450 };
    let rec = |m: &mut dyn Mem, k: u32| -> u32 {
        let id = u32::from(m.r16(arr.wrapping_add(k.wrapping_mul(2))));
        m.r32(SHIPS).wrapping_add(id.wrapping_mul(0x180))
    };
    for i in lo..hi {
        for j in i + 1..hi {
            let (ri, rj) = (rec(m, i), rec(m, j));
            let ecx = if health { arr } else { rj };
            let kj = h.call(m, keyfn, ecx, &[rj]);
            let ki = h.call(m, keyfn, ecx, &[ri]);
            if (u32::from(kj < ki) ^ dir) != 0 {
                let (x, y) = (m.r16(arr + 2 * i), m.r16(arr + 2 * j));
                m.w16(arr + 2 * i, y);
                m.w16(arr + 2 * j, x);
            }
        }
    }
    let gkey = |m: &mut dyn Mem, r: u32| -> u32 {
        if health {
            let num = m.r32(r + 0x18).wrapping_mul(100) as i32;
            let den = m.r32(r + 0x14) as i32;
            num.wrapping_div(den) as u32
        } else {
            m.r32(r + 0x120)
        }
    };
    let mut e = lo;
    while e < hi {
        let start = e;
        let r0 = rec(m, e);
        let v0 = gkey(m, r0);
        while e < hi {
            let r = rec(m, e);
            if gkey(m, r) != v0 {
                break;
            }
            e += 1;
        }
        for i in start..e {
            for j in i + 1..e {
                let (pa, pb) = (rec(m, i).wrapping_add(0x160), rec(m, j).wrapping_add(0x160));
                let ecx = if health { pb } else { m.r32(SHIPS) };
                if (h.call(m, 0x639CF0, ecx, &[pb, pa]) as i32) < 0 {
                    let (x, y) = (m.r16(arr + 2 * i), m.r16(arr + 2 * j));
                    m.w16(arr + 2 * i, y);
                    m.w16(arr + 2 * j, x);
                }
            }
        }
    }
}

fn swap_ids(m: &mut dyn Mem, arr: u32, i: u32, j: u32) {
    let (x, y) = (m.r16(arr + 2 * i), m.r16(arr + 2 * j));
    m.w16(arr + 2 * i, y);
    m.w16(arr + 2 * j, x);
}

/// Ordena por nombre `[start, end)` (`strcmp` `0x639CF0` sobre `+0x160`); `ecx_of` da el ECX que el original deja en cada llamada.
fn name_sort(m: &mut dyn Mem, h: &mut dyn Host, arr: u32, start: u32, end: u32, convoy_ecx: bool) {
    for i in start..end {
        for j in i + 1..end {
            let id = |m: &mut dyn Mem, k: u32| u32::from(m.r16(arr + 2 * k));
            let (ii, ij) = (id(m, i), id(m, j));
            let base = m.r32(SHIPS);
            let (pa, pb) = (base.wrapping_add(ii.wrapping_mul(0x180)).wrapping_add(0x160), base.wrapping_add(ij.wrapping_mul(0x180)).wrapping_add(0x160));
            let ecx = if convoy_ecx { ij.wrapping_mul(0x180) } else { pb };
            if (h.call(m, 0x639CF0, ecx, &[pb, pa]) as i32) < 0 {
                swap_ids(m, arr, i, j);
            }
        }
    }
}

/// `0x478120` (por convoy) y `0x478340` (por tipo): `(lo, hi)`.
fn range_sort2(m: &mut dyn Mem, h: &mut dyn Host, t: u32, a: &[u32], convoy: bool) {
    let (lo, hi) = (a[0], a[1]);
    if lo >= hi {
        return;
    }
    let arr = m.r32(t + 0x8F4);
    let rec = |m: &mut dyn Mem, k: u32| -> u32 {
        let id = u32::from(m.r16(arr.wrapping_add(k.wrapping_mul(2))));
        m.r32(SHIPS).wrapping_add(id.wrapping_mul(0x180))
    };
    for i in lo..hi {
        for j in i + 1..hi {
            let (ri, rj) = (rec(m, i), rec(m, j));
            if convoy {
                let lim = m.r16(0x6D_D896);
                let cj = m.r16(rj + 8);
                if cj >= lim {
                    continue;
                }
                let ci = m.r16(ri + 8);
                let swap = if ci >= lim {
                    true
                } else {
                    let tab = m.r32(0x6D_D7A8);
                    let gi = u32::from(m.r16(tab.wrapping_add(u32::from(ci) * 60).wrapping_add(0x10)));
                    let gj = u32::from(m.r16(tab.wrapping_add(u32::from(cj) * 60).wrapping_add(0x10)));
                    let base = m.r32(SHIPS);
                    let pi = base.wrapping_add(gi.wrapping_mul(0x180)).wrapping_add(0x160);
                    let pj = base.wrapping_add(gj.wrapping_mul(0x180)).wrapping_add(0x160);
                    (h.call(m, 0x639CF0, pj, &[pj, pi]) as i32) < 0
                };
                if swap {
                    swap_ids(m, arr, i, j);
                }
            } else if m.r8(rj + 0xE) < m.r8(ri + 0xE) {
                swap_ids(m, arr, i, j);
            }
        }
    }
    if convoy {
        let mut e = lo;
        while e < hi {
            let start = e;
            let r0 = rec(m, e);
            let v0 = m.r16(r0 + 8);
            while e < hi {
                let r = rec(m, e);
                if m.r16(r + 8) != v0 {
                    break;
                }
                e += 1;
            }
            name_sort(m, h, arr, start, e, true);
        }
    } else {
        let mut e = lo;
        for k in 0..=3u8 {
            let start = e;
            while e < hi {
                let r = rec(m, e);
                if m.r8(r + 0xE) != k {
                    break;
                }
                e += 1;
            }
            name_sort(m, h, arr, start, e, true);
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, t: u32, a: &[u32], esp: u32) -> Option<Ret> {
    let r = match f {
        0x4766F0 => {
            let tab = if a[1] as u8 != 0 { t + 0x838 } else { t + 0x874 };
            let edi = tcall(m, h, 0x40ABF0, tab, &[], esp).wrapping_add(tcall(m, h, 0x40ABE0, tab, &[], esp));
            tcall(m, h, 0x40AC40, tab, &[esp.wrapping_sub(0x1C)], esp);
            let o = tcall(m, h, 0x40AC00, tab, &[], esp);
            let y = edi
                .wrapping_mul(a[0])
                .wrapping_add(m.r32(esp.wrapping_sub(0x18)).wrapping_add(o));
            let x = m.r32(esp.wrapping_sub(0x1C));
            m.w32(esp.wrapping_sub(0x10), x);
            m.w32(esp.wrapping_sub(0xC), y);
            m.w32(esp.wrapping_sub(4), y.wrapping_add(edi));
            let w = tcall(m, h, 0x40ABC0, tab, &[], esp);
            m.w32(esp.wrapping_sub(8), w.wrapping_add(x));
            h.call(m, 0x4B9650, DIRTY, &[esp.wrapping_sub(0x10)]);
            Ret::None
        }
        0x477790 => {
            let tab = a[0];
            tcall(m, h, 0x40AC40, tab, &[esp.wrapping_sub(0x1C)], esp);
            let o = tcall(m, h, 0x40AC00, tab, &[], esp);
            let v = o.wrapping_add(m.r32(esp.wrapping_sub(0x18)));
            m.w32(esp.wrapping_sub(0xC), v);
            let b = tcall(m, h, 0x40ABD0, tab, &[], esp);
            let x = m.r32(esp.wrapping_sub(0x1C));
            m.w32(esp.wrapping_sub(0x10), x);
            let v = b.wrapping_add(m.r32(esp.wrapping_sub(0x18)));
            m.w32(esp.wrapping_sub(4), v);
            let w = tcall(m, h, 0x40ABC0, tab, &[], esp);
            let v = w.wrapping_add(m.r32(esp.wrapping_sub(0x1C)));
            m.w32(esp.wrapping_sub(8), v);
            if tab == t + 0x838 {
                let v = m.r32(esp.wrapping_sub(0x10)).wrapping_sub(m.r32(t + 0xCD0));
                m.w32(esp.wrapping_sub(0x10), v);
                let obj = t + 0xDBC;
                let d = vcall(m, h, obj, 0x44, &[]);
                let v = m.r32(esp.wrapping_sub(0x10)).wrapping_sub(d);
                m.w32(esp.wrapping_sub(0x10), v);
            }
            h.call(m, 0x4B9650, DIRTY, &[esp.wrapping_sub(0x10)]);
            Ret::None
        }
        0x476CC0 => {
            let sp = |o: u32| esp.wrapping_sub(o);
            if m.r32(t + 0x8EC) != 0 {
                let tab = t + 0x838;
                tcall(m, h, 0x40AC60, tab, &[4, sp(0x1C)], esp);
                let (x, y) = (m.r32(sp(0x1C)), m.r32(sp(0x18)));
                m.w32(sp(0xC), y);
                m.w32(sp(0x10), x);
                let mut e = tcall(m, h, 0x40A940, tab, &[5], esp);
                e = e.wrapping_add(tcall(m, h, 0x40A960, tab, &[4], esp));
                let r = tcall(m, h, 0x40A940, tab, &[4], esp);
                m.w32(sp(8), r.wrapping_add(x.wrapping_add(e)));
                let b = tcall(m, h, 0x40ABD0, tab, &[], esp);
                m.w32(sp(4), b.wrapping_add(y));
                h.call(m, 0x4B9650, DIRTY, &[sp(0x10)]);
            }
            if m.r32(t + 0x8F0) != 0 {
                let tab = t + 0x874;
                tcall(m, h, 0x40AC60, tab, &[4, sp(0x1C)], esp);
                let (x, y) = (m.r32(sp(0x1C)), m.r32(sp(0x18)));
                m.w32(sp(0xC), y);
                m.w32(sp(0x10), x);
                let e = tcall(m, h, 0x40A960, tab, &[4], esp);
                let r = tcall(m, h, 0x40A940, tab, &[4], esp);
                m.w32(sp(8), r.wrapping_add(x.wrapping_add(e)));
                let b = tcall(m, h, 0x40ABD0, tab, &[], esp);
                m.w32(sp(4), b.wrapping_add(y));
                h.call(m, 0x4B9650, DIRTY, &[sp(0x10)]);
            }
            Ret::None
        }
        0x475B90 => {
            let ebp = h.call(m, 0x4B9730, DIRTY, &[]);
            let mut n = 0u32;
            if m.r32(t + 0xCC8) > 0 {
                loop {
                    let p = m.r32(t + 0xBD4).wrapping_add(n.wrapping_mul(0xE8));
                    h.call(m, 0x4B4EB0, ebp, &[p]);
                    n += 1;
                    if n >= m.r32(t + 0xCC8) {
                        break;
                    }
                }
            }
            m.w32(t + 0xCC8, 0);
            let arr = m.r32(t + 0xBD4);
            if arr != 0 {
                vcall(m, h, arr, 0, &[3]);
                m.w32(t + 0xBD4, 0);
            } else {
                m.w32(t + 0xBD4, 0);
            }
            Ret::None
        }
        0x4CE490 => {
            let l = t.wrapping_sub(0x94);
            let sub = esp.wrapping_sub(0x80);
            let mut pos = call(m, h, 0x4B9EC0, t + 0x20, &[], sub);
            if pos == m.r32(l + 0x230) {
                return Some(Ret::None);
            }
            let sb = l + 0xB4;
            let vis = call(m, h, 0x4BA040, sb, &[], sub) as i32;
            let rows = m.r32(l + 0x9C) as i32;
            if rows > vis {
                let lim = rows.wrapping_sub(call(m, h, 0x4BA040, sb, &[], sub) as i32);
                if (pos as i32) > lim {
                    pos = lim as u32;
                }
            }
            let cols = m.r32(l + 0xA4) as i32;
            let cell = |m: &mut dyn Mem, r: u32, c: u32| -> u32 {
                let tab = m.r32(l + 0x98);
                let row = m.r32(tab.wrapping_add(r.wrapping_mul(4)));
                if row == 0 {
                    0
                } else {
                    m.r32(row.wrapping_add(c.wrapping_mul(4)))
                }
            };
            let (px, py) = (esp.wrapping_sub(0xC), esp.wrapping_sub(8));
            m.w32(px, 0);
            m.w32(py, 0);
            m.w32(esp.wrapping_sub(4), 0);
            let mut col = 0i32;
            while col < cols {
                let y0 = if m.r8(l + 0x215) != 0 { m.r32(l + 0xB0) } else { 0 };
                m.w32(py, y0);
                let v = call(m, h, 0x4BA040, sb, &[], sub);
                let mut end = v.wrapping_add(pos) as i32;
                let nrows = m.r32(l + 0x9C) as i32;
                if end > nrows {
                    end = nrows;
                }
                let mut r = 0i32;
                while (r as i32) < (pos as i32) && (pos as i32) > 0 {
                    let c = cell(m, r as u32, col as u32);
                    if c != 0 {
                        vcall(m, h, c, 0xCC, &[0]);
                    }
                    r += 1;
                }
                while r < end {
                    let c = cell(m, r as u32, col as u32);
                    if c != 0 {
                        vcall(m, h, c, 0x68, &[px]);
                        vcall(m, h, c, 0xCC, &[1]);
                        vcall(m, h, c, 0xD4, &[]);
                    }
                    let y = m.r32(py).wrapping_add(m.r32(l + 0xB0));
                    m.w32(py, y);
                    r += 1;
                }
                while r < (m.r32(l + 0x9C) as i32) {
                    let c = cell(m, r as u32, col as u32);
                    if c != 0 {
                        vcall(m, h, c, 0xCC, &[0]);
                    }
                    r += 1;
                }
                let wt = m.r32(l + 0xAC);
                let w = m.r32(wt.wrapping_add((col as u32).wrapping_mul(4)));
                let x = m.r32(px).wrapping_add(w);
                m.w32(px, x);
                col += 1;
                if col >= (m.r32(l + 0xA4) as i32) {
                    break;
                }
            }
            if m.r32(l + 0x230) != pos {
                m.w8(l + 0x313, 1);
            }
            m.w32(l + 0x230, pos);
            vcall(m, h, l, 0xD4, &[]);
            Ret::None
        }
        0x4CDC70 => {
            let p0 = m.r32(a[0]);
            let o0 = m.r32(p0);
            let edi = if o0 != 0 { vcall(m, h, o0, 0x14, &[]) } else { 0 };
            let q = m.r32(a[1]);
            let esi = if q != 0 {
                let o1 = m.r32(q);
                vcall(m, h, o1, 0x14, &[])
            } else {
                0
            };
            let (s1, s2) = (esp.wrapping_sub(0x10), esp.wrapping_add(4));
            h.call(m, 0x64EFC8, s1, &[edi.wrapping_add(8)]);
            h.call(m, 0x64EFC8, s2, &[esi.wrapping_add(8)]);
            let (p2, p1) = (m.r32(s2), m.r32(s1));
            let r = h.call(m, 0x639CF0, p1, &[p1, p2]);
            h.call(m, 0x64F253, s2, &[]);
            h.call(m, 0x64F253, s1, &[]);
            Ret::U32(r)
        }
        0x477C10 => {
            let n = m.r32(t + 0x8E8);
            if n > 1 {
                let mut i = 0u32;
                loop {
                    let own = m.r32(t + 0x8E0);
                    let arr = m.r32(t + 0x8F4);
                    let base = m.r32(SHIPS);
                    let owner = |m: &mut dyn Mem, k: u32| -> u32 {
                        let id = u32::from(m.r16(arr.wrapping_add(k.wrapping_mul(2))));
                        u32::from(m.r8(base.wrapping_add(id.wrapping_mul(0x180))))
                    };
                    if owner(m, i) != own {
                        let c = i + 1;
                        if c >= n {
                            break;
                        }
                        match (c..n).find(|&j| owner(m, j) == own) {
                            Some(j) => swap_ids(m, arr, i, j),
                            None => break,
                        }
                    }
                    i += 1;
                    if i >= n {
                        break;
                    }
                }
            }
            Ret::None
        }
        0x478120 | 0x478340 => {
            range_sort2(m, h, t, a, f == 0x478120);
            Ret::None
        }
        0x477DA0 | 0x477F70 => {
            range_sort(m, h, t, a, f == 0x477DA0);
            Ret::None
        }
        0x4779D0 => {
            let (n1, n2) = (m.r32(t + 0x8EC), m.r32(t + 0x8F0));
            let end = n2.wrapping_add(n1);
            let idx = u32::from(m.r8(t + 0xCCC));
            let sel = m.r32(0x66E930u32.wrapping_add(idx.wrapping_mul(4))).wrapping_sub(1);
            if sel <= 3 {
                match sel {
                    3 => {
                        h.call(m, 0x477F70, t, &[0, n1, 0]);
                        let (n1, n2) = (m.r32(t + 0x8EC), m.r32(t + 0x8F0));
                        h.call(m, 0x477F70, t, &[n1, n2.wrapping_add(n1), 0]);
                    }
                    0 => {
                        h.call(m, 0x477DA0, t, &[0, n1, 0]);
                        let (n1, n2) = (m.r32(t + 0x8EC), m.r32(t + 0x8F0));
                        h.call(m, 0x477DA0, t, &[n1, n2.wrapping_add(n1), 0]);
                    }
                    _ => {
                        let f2 = if sel == 2 { 0x478340 } else { 0x478120 };
                        h.call(m, f2, t, &[0, n1]);
                        let (n1, n2) = (m.r32(t + 0x8EC), m.r32(t + 0x8F0));
                        h.call(m, f2, t, &[n1, n2.wrapping_add(n1)]);
                    }
                }
                return Some(Ret::None);
            }
            let arr = m.r32(t + 0x8F4);
            let name = |m: &mut dyn Mem, k: u32| -> u32 {
                let id = u32::from(m.r16(arr.wrapping_add(k.wrapping_mul(2))));
                m.r32(0x6D_D7A4)
                    .wrapping_add(id.wrapping_mul(0x180))
                    .wrapping_add(0x160)
            };
            for (ph, (lo, hi)) in [(0u32, n1), (n1, end)].into_iter().enumerate() {
                let mut i = lo;
                while i < hi {
                    let mut j = i + 1;
                    while j < hi {
                        let (pa, pb) = (name(m, i), name(m, j));
                        let ecx = if ph == 0 { m.r32(0x6D_D7A4) } else { pb };
                        if (h.call(m, 0x639CF0, ecx, &[pb, pa]) as i32) < 0 {
                            let a = m.r16(arr + 2 * i);
                            let b = m.r16(arr + 2 * j);
                            m.w16(arr + 2 * i, b);
                            m.w16(arr + 2 * j, a);
                        }
                        j += 1;
                    }
                    i += 1;
                }
            }
            Ret::None
        }
        _ => return None,
    };
    Some(r)
}
