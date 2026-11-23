//! `tavern_news_compose` (0x00526B40): compone el registro de noticias de la taberna (tipo 0x55/0x4C, tema, texto y
//! precio) para un comerciante. Traducción 1:1 sobre la memoria del original ([`Mem`]); las cadenas (`CString`) son
//! objetos opacos manejados por el anfitrión.
use crate::flatmem::Mem;

pub const TEMP: u32 = 0x5e00_0000; // cadena temporal local (dirección sintética)

pub trait NewsHost {
    /// `rand` del CRT (0x00639C03).
    fn rand(&mut self) -> u32;
    /// `0x004D75D0` (ECX = 0x6DD730): `(comerciante, &a, &b)`; devuelve los dos dobles palabras escritas.
    fn find_msg(&mut self, merchant: u32) -> (u32, u32);
    /// Constructor de `CString` (0x0064F1DE).
    fn cstr_ctor(&mut self, m: &mut dyn Mem, cstr: u32);
    /// `CString::Format` (0x0064DF80): `(destino, formato, argumentos…)`.
    fn format(&mut self, m: &mut dyn Mem, dst: u32, fmt: u32, args: &[u32]);
    /// `CString::operator+=(LPCSTR)` (0x0064F5E3).
    fn append_lit(&mut self, m: &mut dyn Mem, dst: u32, lit: u32);
    /// `CString::operator+=(CString&)` (0x0064F61F).
    fn append(&mut self, m: &mut dyn Mem, dst: u32, src: u32);
    /// `dst = '"' + dst + '"'` (0x0064CBF0, 0x0064CB60, 0x0064F340 y dos destructores).
    fn quote(&mut self, m: &mut dyn Mem, dst: u32);
    /// Destructor de `CString` (0x0064F253).
    fn cstr_dtor(&mut self, cstr: u32);
    /// `0x005353C0` (ECX = 0x6DE588).
    fn town_5353c0(&mut self, town: u32) -> i32;
    /// `0x00535300` (ECX = 0x6DE588).
    fn town_535300(&mut self, town: u32) -> u32;
    /// `0x00515AE0` sobre la cuadrilla: devuelve los dobles palabras escritos en `(+0x2C, +0x28)`.
    fn band_515ae0(&mut self, band: u32) -> (u32, u32);
    /// `0x004EC694` (ECX = 0x6DD73C).
    fn msg_index(&mut self, merchant: u32) -> u32;
    /// `0x004D8DB0` (ECX = 0x6DD73C): puntero del objeto.
    fn object_at(&mut self, index: u32) -> u32;
    /// `0x00448610` (ECX = [0x6CBDC8]).
    fn ship_448610(&mut self, ship: u32) -> u32;
    /// `0x00512AE0` (ECX = 0x6DDAA0).
    fn text_a(&mut self, v: u32) -> u32;
    /// `0x00512A60` (ECX = 0x6DDAA0).
    fn text_b(&mut self, v: u32) -> u32;
    /// `0x004F6450` sobre un comerciante.
    fn mer_4f6450(&mut self, mer: u32) -> u32;
    /// `0x004F6420` sobre un comerciante.
    fn mer_4f6420(&mut self, mer: u32) -> u32;
}

fn merchant(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(0x6DE4A0 + 0x78).wrapping_add(i.wrapping_mul(0x650))
}

/// `this` = taberna (`+0x2C0` ciudad, `+0x54` modo, `+0x82C` región); `out` = registro de noticia (24 bytes + `CString` en `+0x18`).
/// Los locales sin inicializar del original (`[S+0x28]`, `[S+0x2C]`) se suponen 0; el byte bajo del argumento `out` se
/// reutiliza como variable local (como el original).
pub fn tavern_news_compose(m: &mut dyn Mem, host: &mut dyn NewsHost, this: u32, out: u32) {
    let r = out.wrapping_add(0x18);
    let t = TEMP;
    let mut first = true; // [S+0x13]
    let mut slot54 = out; // [S+0x54]: argumento reutilizado como variable
    let tick = m.r32(0x6DE4B4);
    let sel = ((tick >> 8).wrapping_add(u32::from(m.r8(this + 0x2c0)))) % 3;
    'body: {
        if sel == 0 {
            let cx = u32::from(m.r16(0x6DFC14));
            let (a, b) = host.find_msg(cx);
            if (a as u16) < m.r16(0x6DD736) {
                m.w32(out, 0x55);
                let pm = m.r32(0x6DFC14);
                m.w32(out + 4, pm);
                m.w32(out + 8, a & 0xffff);
                let pm = m.r32(0x6DFC14);
                let mer = merchant(m, pm);
                let home = u32::from(m.r8(mer + 0x19));
                let dl = m.r8(mer.wrapping_add(home).wrapping_add(0x39c));
                slot54 = (slot54 & !0xff) | u32::from(dl);
                let rnd = host.rand() as i32;
                let rem = rnd % 11;
                let k = (u32::from(slot54 as u8) + 1).wrapping_mul(15);
                let v = (rem as u32).wrapping_add(k.wrapping_mul(2));
                m.w32(out + 0xc, v.wrapping_mul(10));
                let e = (b & 0xffff) << 5;
                m.w32(out + 0x10, e);
                let e = e.wrapping_add(u32::from(m.r8(this + 0x2c0)));
                m.w32(out + 0x10, e);
                let fmt = m.r32(0x6C29F8);
                host.format(m, r, fmt, &[]);
                host.quote(m, r);
                break 'body;
            }
        }
        // Noticia del tipo 0x4C.
        m.w32(out, 0x4c);
        let pm = m.r32(0x6DFC14);
        m.w32(out + 4, pm);
        m.w32(out + 8, 0xffff);
        let pm = m.r32(0x6DFC14);
        let mer = merchant(m, pm);
        let home = u32::from(m.r8(mer + 0x19));
        let b = u32::from(m.r8(mer.wrapping_add(home).wrapping_add(0x39c)));
        m.w32(out + 0x10, 0xffff);
        m.w16(out + 0x14, 0);
        m.w16(out + 0x16, 0);
        m.w32(out + 0xc, (b + 1).wrapping_mul(100));
        host.cstr_ctor(m, r);
        let al = m.r8(this + 0x82c);
        let region: u32;
        if al >= 0x14 {
            m.w16(out + 0x14, 0);
            m.w16(out + 0x16, 0);
            region = 0;
        } else {
            let a = u32::from(al) * 4;
            let v = u16::from(m.r8(0x673732 + a));
            m.w16(out + 0x14, v);
            let a = u32::from(m.r8(this + 0x82c)) * 4;
            let v = u16::from(m.r8(0x673733 + a));
            m.w16(out + 0x16, v);
            let a = u32::from(m.r8(this + 0x82c)) * 4;
            region = u32::from(m.r8(0x673731 + a));
        }
        let ntowns = m.r32(0x6DE4B0);
        let town = |m: &mut dyn Mem, i: u32| m.r32(0x6DE508).wrapping_add(i.wrapping_mul(0x9f8));
        let region_of = |m: &mut dyn Mem, tp: u32| {
            let k = u32::from(m.r8(tp + 0x2c1));
            u32::from(m.r8(0x6DDBC0 + k * 0x34))
        };
        // Aviso de ciudades: banderas de 0x2C8 que difieren de las del mercado local.
        let mut i = 0u32;
        while i < ntowns {
            let tp = town(m, i);
            if region_of(m, tp) == region {
                let q = host.town_5353c0(i);
                if q > 3 {
                    let esi = m.r32(tp + 0x2c8);
                    let eax = host.town_535300(i);
                    let name = m.r32(0x6DDA00 + 4 * i);
                    // (máscara, formato si activo, formato si inactivo, separador)
                    let simple: [(u32, u32, u32, u32); 4] = [
                        (0x10, 0x6C25C8, 0x6C25CC, 0x6C07FC),
                        (0x8, 0x6C25C8, 0x6C25CC, 0x6C07FE),
                        (0x8000, 0x6C26C0, 0x6C26C4, 0x6C0800),
                        (0x1000, 0x6C25C8, 0x6C25CC, 0x6C0802),
                    ];
                    let mut done = false;
                    for (mask, fa, fb, sep) in simple {
                        if esi & mask != eax & mask {
                            let f = if esi & mask != 0 { m.r32(fa) } else { m.r32(fb) };
                            host.format(m, t, f, &[name]);
                            add_item(m, host, r, out, t, &mut first, sep);
                            done = true;
                            break;
                        }
                    }
                    if !done {
                        if (eax ^ esi) & 0x7c0000 != 0 {
                            let s = (esi >> 0x12) & 0x1f;
                            if (s as i32) < 0x14 {
                                let label = m.r32(0x6C0BB0 + s * 4);
                                let f = if m.r8(0x66E0D4 + s) != 0 {
                                    m.r32(0x6C276C)
                                } else {
                                    m.r32(0x6C2770)
                                };
                                host.format(m, t, f, &[name, label]);
                                add_item(m, host, r, out, t, &mut first, 0x6C0804);
                            }
                        } else if esi & 0x2000 == 0x2000 {
                            let f = m.r32(0x6C272C);
                            host.format(m, t, f, &[name]);
                            add_item(m, host, r, out, t, &mut first, 0x6C0806);
                        } else if esi & 0x4000 == 0x4000 {
                            let f = m.r32(0x6C2730);
                            host.format(m, t, f, &[name]);
                            add_item(m, host, r, out, t, &mut first, 0x6C0808);
                        }
                    }
                }
            }
            i += 1;
        }
        let data = m.r32(r);
        if m.r32(data.wrapping_sub(8)) == 0 {
            let v = u32::from(m.r8(this + 0x2c0));
            m.w32(out + 8, v);
        }
        // Mejor ciudad de la región por desequilibrio (primera pasada).
        let mut best_town = 0xffu32;
        let mut best_k = 0xffu32;
        let mut best = 0x100i32;
        let mut ebp = ntowns as i32;
        if ebp > 0 {
            loop {
                ebp -= 1;
                let tp = town(m, ebp as u32);
                if region_of(m, tp) == region {
                    for k in 0..20u32 {
                        let a = m.r32(tp + 0x50 - 4 * k) as i32;
                        let c = m.r32(tp + 0x620 - 0x10 * k) as i32;
                        if a < c {
                            let v = if a > 0 {
                                ((a << 2) as i64 / i64::from(c >> 7)) as i32
                            } else {
                                c.wrapping_neg()
                            };
                            if v < best {
                                best = v;
                                best_k = 0x13 - k;
                                best_town = ebp as u32;
                            }
                        }
                    }
                }
                if ebp <= 0 {
                    break;
                }
            }
        }
        if best_town < m.r32(0x6DE4B0) {
            let name = m.r32(0x6DDA00 + 4 * best_town);
            let label = m.r32(0x6C0BB0 + best_k * 4);
            let f = m.r32(0x6C27E0);
            host.format(m, t, f, &[label, name]);
            add_item(m, host, r, out, t, &mut first, 0x6C080A);
        }
        // Segunda pasada: precios fuera de rango.
        let mut best_town = 0xffu32;
        let mut best_k = 0xffu32;
        let mut best = 0i32;
        let mut ebp = m.r32(0x6DE4B0) as i32;
        if ebp > 0 {
            loop {
                ebp -= 1;
                let tp = town(m, ebp as u32);
                if region_of(m, tp) == region && m.r32(tp + 0x76c) & 0x100 == 0x100 {
                    for k in 0..0x18u32 {
                        if (1u32 << k) & 0x4472e != 0 {
                            let lo = m.r32(tp + 0x4f4 + 0x10 * k) as i32;
                            let edx = m.r32(tp + 4 + 4 * k) as i32;
                            if edx > lo.wrapping_add(0x4e20) {
                                let hi = m.r32(tp + 0x4f4 + 0x10 * k + 4) as i32;
                                let v;
                                if edx > hi {
                                    let d = hi.wrapping_sub(lo) >> 7;
                                    v = if d > 1 {
                                        (edx.wrapping_mul(2) / d).wrapping_add(0x100)
                                    } else {
                                        edx.wrapping_add(0x100)
                                    };
                                } else {
                                    v = edx.wrapping_mul(2) / (lo >> 7);
                                }
                                if v > best {
                                    best = v;
                                    best_k = k;
                                    best_town = ebp as u32;
                                }
                            }
                        }
                    }
                }
                if ebp <= 0 {
                    break;
                }
            }
        }
        if best_town < m.r32(0x6DE4B0) {
            let name = m.r32(0x6DDA00 + 4 * best_town);
            let label = m.r32(0x6C0BB0 + best_k * 4);
            let f = m.r32(0x6C28A8);
            host.format(m, t, f, &[label, name]);
            add_item(m, host, r, out, t, &mut first, 0x6C080C);
        }
        // Cuadrillas piratas: la de mayor +0x16.
        slot54 &= !0xff;
        let mut si = slot54 as u16;
        let mut best_v = 0u32;
        let mut s2c = 0u8;
        let mut s28 = 0u8;
        for bnd in 0..5u32 {
            let band = m.r32(0x6DD7AC + 4 * bnd);
            if band != 0 {
                let v = u32::from(m.r16(band + 0x16));
                if v > best_v {
                    best_v = v;
                    slot54 = (slot54 & !0xff) | bnd;
                    let (a, b) = host.band_515ae0(band);
                    s2c = a as u8;
                    s28 = b as u8;
                    let band = m.r32(0x6DD7AC + 4 * bnd);
                    si = m.r16(band + 0xc);
                }
            }
        }
        let mi = m.r32(0x6DFC14);
        let ax = host.msg_index(mi) as u16;
        if ax != 0xffff {
            let obj = host.object_at(u32::from(ax));
            if m.r8(obj + 0xc) == 2 && m.r8(obj + 0x16) == 0 {
                s2c = m.r8(obj + 0x12);
                si = m.r16(obj + 0x10);
                s28 = m.r8(obj + 0x14);
            }
        }
        let band0 = m.r32(0x6DD7AC + 4 * (slot54 & 0xff));
        if band0 != 0 && si < m.r16(0x6DD894) {
            loop {
                let ship = m.r32(0x6DD7A4).wrapping_add(u32::from(si) * 0x180);
                let tr = m.r16(ship + 0x42);
                if tr < m.r16(0x6DD892) {
                    let trp = m.r32(0x6DD7A0).wrapping_add(u32::from(tr) << 4);
                    if m.r8(trp + 2) == s2c && m.r8(trp + 3) == s28 {
                        let ecx = m.r32(0x6CBDC8);
                        let _ = ecx;
                        let al = host.ship_448610(u32::from(si));
                        slot54 = (slot54 & !0xff) | (al & 0xff);
                        let esi = al & 0xff;
                        let which = (u32::from(m.r16(ship + 0x42)) + esi) % 3;
                        let name = m.r32(0x6DDA00 + 4 * esi);
                        match which {
                            0 => {
                                let a = host.text_a(u32::from(s28));
                                let b = host.text_b(u32::from(s2c));
                                let f = m.r32(0x6B1460);
                                host.format(m, t, f, &[name, b, a]);
                            }
                            1 => {
                                let a = host.text_a(u32::from(s28));
                                let b = host.text_b(u32::from(s2c));
                                let f = m.r32(0x6B1464);
                                host.format(m, t, f, &[b, a, name]);
                            }
                            2 => {
                                let a = host.text_a(u32::from(s28));
                                let b = host.text_b(u32::from(s2c));
                                let f = m.r32(0x6B1468);
                                host.format(m, t, f, &[b, a, name]);
                            }
                            _ => {}
                        }
                        add_item(m, host, r, out, t, &mut first, 0x6C080E);
                    }
                }
                si = m.r16(ship + 4);
                if si >= m.r16(0x6DD894) {
                    break;
                }
            }
        }
        let data = m.r32(r);
        if m.r32(data.wrapping_sub(8)) != 0 {
            host.quote(m, r);
            break 'body;
        }
        // Texto de relleno según el modo de la taberna.
        let want = (host.rand() & 3) + 1;
        let mut want = want as i32;
        let mut ecx = 0u32;
        let mut wrapped = false;
        let mut found = 0u32;
        let mut fallback = false;
        while want > 0 {
            let ntown = m.r32(0x6DE4B0);
            let tp = town(m, ecx);
            let reg = {
                let k = u32::from(m.r8(tp + 0x2c1));
                let v = m.r8(0x6DDBC0 + k * 0x34);
                slot54 = (slot54 & !0xff) | u32::from(v);
                u32::from(v)
            };
            if reg == region {
                found = ecx;
                want -= 1;
            }
            ecx += 1;
            if ecx >= ntown {
                if wrapped {
                    fallback = true;
                    break;
                }
                ecx = 0;
                wrapped = true;
            }
        }
        let edi;
        if fallback {
            let n = m.r32(0x6DE4B0);
            edi = u32::from(m.r8(this + 0x82c)) % n;
        } else {
            edi = found;
        }
        let tp = town(m, edi);
        let name = m.r32(0x6DDA00 + 4 * edi);
        match m.r32(this + 0x54) & 3 {
            0 => {
                let a = m.r32(tp + 0x2d4);
                let b = m.r32(tp + 0x2e4);
                let f = m.r32(0x6C29B8);
                host.format(m, r, f, &[a, name, b]);
            }
            1 => {
                let al = m.r8(tp + 0x6f1);
                let f;
                let (x, y);
                if u16::from(al) < m.r16(0x6DE4AA) {
                    let me = merchant(m, u32::from(al));
                    x = host.mer_4f6450(me);
                    let al = m.r8(tp + 0x6f1);
                    let me = merchant(m, u32::from(al));
                    y = host.mer_4f6420(me);
                    f = m.r32(0x6C29BC);
                } else {
                    let a = u32::from(m.r8(tp + 0x6d8));
                    x = host.text_a(a);
                    let b = u32::from(m.r8(tp + 0x6d4));
                    y = host.text_b(b);
                    f = m.r32(0x6C29BC);
                }
                host.format(m, r, f, &[y, x, name]);
            }
            2 => {
                let a = m.r32(tp + 0x31c);
                let f = m.r32(0x6C29C0);
                host.format(m, r, f, &[a, name]);
            }
            _ => {
                let a = u32::from(m.r16(tp + 0x2c6));
                let f = m.r32(0x6C29C4);
                host.format(m, r, f, &[a, name]);
            }
        }
        host.quote(m, r);
        let v = (m.r32(this + 0x54).wrapping_add(edi).wrapping_add(region) & 3) + 7;
        m.w32(out + 0xc, v * 10);
        let v = u32::from(m.r8(this + 0x2c0));
        m.w32(out + 8, v);
    }
    host.cstr_dtor(t);
}

/// Cierre común de un elemento: separador si no es el primero, añadir la temporal y sumar 10 al precio.
fn add_item(m: &mut dyn Mem, host: &mut dyn NewsHost, r: u32, out: u32, t: u32, first: &mut bool, sep: u32) {
    if *first {
        *first = false;
    } else {
        host.append_lit(m, r, sep);
    }
    host.append(m, r, t);
    let v = m.r32(out + 0xc).wrapping_add(10);
    m.w32(out + 0xc, v);
}
