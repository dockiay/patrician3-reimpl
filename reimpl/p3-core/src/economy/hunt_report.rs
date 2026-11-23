//! `send_pirate_hunt_report_message` (0x004F3AF0) y su render recursivo `0x004F3CE0`: componen el texto del informe de caza
//! pirata y lo publican. Traducción 1:1 sobre la memoria del original ([`Mem`]).
use crate::flatmem::Mem;

/// Cadenas locales (direcciones sintéticas).
pub const L1: u32 = 0x5e20_0000;
pub const L2: u32 = 0x5e20_0004;
pub const TMP: u32 = 0x5e20_0100;

pub trait HuntHost {
    /// `operator new` (0x0064F7B9).
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    /// `CString::Format` (0x0064DF80): `(destino, formato, argumentos…)`.
    fn format(&mut self, m: &mut dyn Mem, dst: u32, fmt: u32, args: &[u32]);
    /// `CString::operator+=(LPCSTR)` (0x0064F5E3).
    fn append_lit(&mut self, m: &mut dyn Mem, dst: u32, lit: u32);
    /// `CString::operator+=(CString&)` (0x0064F61F).
    fn append(&mut self, m: &mut dyn Mem, dst: u32, src: u32);
    /// Destructor de `CString` (0x0064F253).
    fn dtor(&mut self, c: u32);
    /// `0x00512B20` (ECX = 0x6DDAA0): texto de un id de 16 bits.
    fn text(&mut self, m: &mut dyn Mem, v: u32) -> u32;
    /// `0x00512AE0`.
    fn text_a(&mut self, m: &mut dyn Mem, v: u32) -> u32;
    /// `0x00512A60`.
    fn text_b(&mut self, m: &mut dyn Mem, v: u32) -> u32;
    /// `0x004D6530` (ECX = 0x6DD730).
    fn post(&mut self, m: &mut dyn Mem, code: u32, msg: &mut [u8; 16]);
    /// Destructor del mensaje (0x0051A880).
    fn msg_dtor(&mut self);
}

fn msg_ctor() -> [u8; 16] {
    let mut o = [0u8; 16];
    o[1] = 0x80;
    o[2] = 0xff;
    o[3] = 0xff;
    o[4] = 0xff;
    o[5] = 0xff;
    o
}

/// 0x004F3CE0: `node` = nodo de la expresión (`+0` hijo, `+4` clase, `+5` marca, `+6` id de texto); `a1` = marca buscada; `a2` = bandera.
pub fn render_node(m: &mut dyn Mem, h: &mut dyn HuntHost, node: u32, a1: u32, a2: u32, dst: u32, depth: u32) -> u32 {
    let mut local = a2 as u8;
    let tmp = TMP + 8 * depth;
    if m.r8(node + 5) == a1 as u8 {
        let k = m.r8(node + 4);
        if k == 3 {
            let e = m.r32(0x6C7CCC);
            m.w32(tmp, e);
            let id = u32::from(m.r16(node + 6));
            let t = h.text(m, id);
            let f = m.r32(0x6B1360);
            h.format(m, tmp, f, &[t]);
            h.append(m, dst, tmp);
            h.dtor(tmp);
        } else if k < 3 {
            local = 1;
        }
    }
    let child = m.r32(node);
    let bl: u8 = if child != 0 {
        render_node(m, h, child, a1, u32::from(local), dst, depth + 1) as u8
    } else {
        0
    };
    let k = u32::from(m.r8(node + 4));
    if k < 3 {
        if bl != 0 {
            let id = u32::from(m.r16(node + 6));
            let t = h.text(m, id);
            h.append_lit(m, dst, t);
            let lit = if k == 1 {
                m.r32(0x6B136C)
            } else {
                m.r32(0x6B1364 + 4 * k)
            };
            h.append_lit(m, dst, lit);
            return u32::from(bl);
        } else if a2 as u8 != 0 {
            let l = m.r32(0x6B135C);
            h.append_lit(m, dst, l);
            let id = u32::from(m.r16(node + 6));
            let t = h.text(m, id);
            h.append_lit(m, dst, t);
            let l = m.r32(0x6B136C + 4 * k);
            h.append_lit(m, dst, l);
            return 1;
        } else {
            let e = m.r32(0x6C7CCC);
            m.w32(tmp, e);
            let id = u32::from(m.r16(node + 6));
            let t = h.text(m, id);
            let f = m.r32(0x6B136C + 4 * k);
            h.format(m, tmp, f, &[t]);
            h.append(m, dst, tmp);
            h.dtor(tmp);
        }
    }
    u32::from(bl)
}

pub fn send_pirate_hunt_report_message(
    m: &mut dyn Mem,
    h: &mut dyn HuntHost,
    this: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
) {
    let e = m.r32(0x6C7CCC);
    m.w32(L1, e);
    m.w32(L2, e);
    let mut msg = msg_ctor();
    let idx = (a4 & 3) * 4;
    let ta = h.text_a(m, a3 & 0xff);
    let tb = h.text_b(m, a2 & 0xff);
    let f = m.r32(0x6B1338 + idx);
    h.format(m, L1, f, &[tb, ta]);
    render_node(m, h, this, a1, 0, L1, 0);
    let f = m.r32(0x6B1358);
    let player = u32::from(m.r8(0x6DE52E));
    let name = m.r32(0x6DDA00 + 4 * player);
    h.format(m, L2, f, &[name]);
    h.append(m, L1, L2);
    msg[4] = 0x3d;
    msg[5] = m.r8(0x6DE52E);
    let data = m.r32(L1);
    let len = m.r32(data.wrapping_sub(8));
    let buf1 = h.alloc(m, len.wrapping_add(0x64));
    msg[12..16].copy_from_slice(&buf1.to_le_bytes());
    let obj = h.alloc(m, 0x10);
    msg[8..12].copy_from_slice(&obj.to_le_bytes());
    m.w32(obj, 0);
    m.w32(obj + 4, 0);
    m.w16(obj + 8, 0xffff);
    m.w8(obj + 0xa, 0);
    m.w8(obj + 0xb, 0);
    m.w8(obj + 0xc, 0);
    let pl = m.r8(0x6DE52E);
    m.w8(obj + 0xd, pl);
    m.w8(obj + 0xe, 0xff);
    m.w8(obj + 0xf, 0xff);
    let mut s = m.r32(0x6B1348 + idx);
    let mut buf = buf1;
    loop {
        let c = m.r8(s);
        m.w8(buf, c);
        buf += 1;
        let n = m.r32(obj).wrapping_add(1);
        m.w32(obj, n);
        s += 1;
        if c == 0 {
            break;
        }
    }
    let data = m.r32(L1);
    let l = m.r32(data.wrapping_sub(8)) as i32;
    if l > 0 {
        let mut i = 0u32;
        loop {
            let c = m.r8(data + i);
            m.w8(buf, c);
            buf += 1;
            let n = m.r32(obj).wrapping_add(1);
            m.w32(obj, n);
            i += 1;
            let data2 = m.r32(L1);
            if (i as i32) >= m.r32(data2.wrapping_sub(8)) as i32 {
                break;
            }
        }
    }
    for _ in 0..4 {
        m.w8(buf, 0);
        buf += 1;
        let n = m.r32(obj).wrapping_add(1);
        m.w32(obj, n);
    }
    h.post(m, a1 & 0xff, &mut msg);
    h.msg_dtor();
    h.dtor(L2);
    h.dtor(L1);
}
