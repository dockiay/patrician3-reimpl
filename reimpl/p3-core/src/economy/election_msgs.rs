//! `post_alderman_election_messages` (0x005299F0): compone y publica los dos mensajes de la elección de concejales
//! (texto con los candidatos y aviso resumido). Traducción 1:1 sobre la memoria del original ([`Mem`]).
use crate::flatmem::Mem;

pub trait ElectionHost {
    /// `operator new` (0x0064F7B9).
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    /// `0x00512AE0` (ECX = 0x6DDAA0): puntero de texto.
    fn text_a(&mut self, m: &mut dyn Mem, v: u32) -> u32;
    /// `0x00512A60` (ECX = 0x6DDAA0): puntero de texto.
    fn text_b(&mut self, m: &mut dyn Mem, v: u32) -> u32;
    /// `sprintf(buf, fmt, b, a, ciudad, q)` (0x0064D410): bytes escritos (sin el NUL).
    fn sprintf(&mut self, m: &mut dyn Mem, fmt: u32, b: u32, a: u32, town_name: u32, q: u32) -> Vec<u8>;
    /// `0x004D6530` (ECX = 0x6DD730): publica el mensaje (`code`, estructura de 16 bytes). Puede modificarla.
    fn post(&mut self, m: &mut dyn Mem, code: i32, msg: &mut [u8; 16]);
    /// `operator delete` (0x0064F7E2).
    fn free(&mut self, p: u32);
    /// Destructor del mensaje (`0x0051A880`).
    fn msg_dtor(&mut self);
    /// `0x004F9430` sobre un comerciante.
    fn merchant_flag(&mut self, mer: u32) -> u8;
}

fn copy_cstr(m: &mut dyn Mem, mut dst: u32, mut src: u32) -> u32 {
    loop {
        let c = m.r8(src);
        if c == 0 {
            return dst;
        }
        m.w8(dst, c);
        dst += 1;
        src += 1;
    }
}

fn merchant(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(0x6DE4A0 + 0x78).wrapping_add(i.wrapping_mul(0x650))
}
fn town(m: &mut dyn Mem, t: u32) -> u32 {
    m.r32(0x6DE508).wrapping_add(t.wrapping_mul(0x9f8))
}

/// `ctor` (0x0051A850) de la estructura de mensaje; los bytes 6 y 7 no se escriben (residuo, 0 aquí).
fn msg_ctor() -> [u8; 16] {
    let mut o = [0u8; 16];
    o[1] = 0x80;
    o[2] = 0xff;
    o[3] = 0xff;
    o[4] = 0xff;
    o[5] = 0xff;
    o
}

/// `p`/`q`: dos tablas de cuatro bytes (candidato con bit alto = comerciante, votos); `kind` y `town_arg` son bytes.
pub fn post_alderman_election_messages(
    m: &mut dyn Mem,
    host: &mut dyn ElectionHost,
    p: u32,
    q: u32,
    kind: u32,
    town_arg: u32,
) {
    let kind = kind & 0xff;
    let town_arg = town_arg & 0xff;
    let mut msg = msg_ctor();
    msg[4] = 0x47;
    msg[5] = kind as u8;
    let buf = host.alloc(m, 0x800);
    msg[8..12].copy_from_slice(&buf.to_le_bytes());
    let mut esi = buf;
    let s = m.r32(0x6B72F0);
    esi = copy_cstr(m, esi, s);
    let s = m.r32(0x6DDA00 + 4 * kind);
    esi = copy_cstr(m, esi, s);
    let s = m.r32(0x6B72F4);
    esi = copy_cstr(m, esi, s);
    for i in 0..4u32 {
        let al = m.r8(p + i);
        let (ebp, ebx, ecx);
        if al & 0x80 != 0 {
            let idx = u32::from(al & 0x7f);
            let mer = merchant(m, idx);
            ebp = u32::from(m.r8(mer + 0x1c));
            ebx = u32::from(m.r8(mer + 0x1d));
            ecx = u32::from(m.r8(mer + 0x19));
        } else {
            let t = u32::from(al);
            let tp = town(m, t);
            let sel = 0xffff_ffffu32.wrapping_sub(u32::from(m.r8(tp + 0x6f1))) & 3;
            ebp = u32::from(m.r8(tp + sel + 0x6d4));
            ebx = u32::from(m.r8(tp + sel + 0x6d8));
            ecx = t;
        }
        let votes = u32::from(m.r8(q + i));
        let name = m.r32(0x6DDA00 + 4 * ecx);
        let a = host.text_a(m, ebx);
        let b = host.text_b(m, ebp);
        let fmt = m.r32(0x6B7300);
        let out = host.sprintf(m, fmt, b, a, name, votes);
        for c in out {
            if c == 0 {
                break;
            }
            m.w8(esi, c);
            esi += 1;
        }
    }
    let s = m.r32(0x6B72F8);
    esi = copy_cstr(m, esi, s);
    let own = u32::from(m.r8(0x6DE52D));
    let n = u32::from(m.r16(0x6DE4AA));
    let (ebp, edi);
    if (own as i32) < (n as i32) {
        let mer = merchant(m, own);
        ebp = u32::from(m.r8(mer + 0x1c));
        edi = u32::from(m.r8(mer + 0x1d));
    } else {
        let sel = 0xffff_ffffu32.wrapping_sub(own) & 3;
        let tp = town(m, town_arg);
        ebp = u32::from(m.r8(tp + sel + 0x6d4));
        edi = u32::from(m.r8(tp + sel + 0x6d8));
    }
    let s = host.text_b(m, ebp);
    esi = copy_cstr(m, esi, s);
    m.w8(esi, 0x20);
    esi += 1;
    let s = host.text_a(m, edi);
    esi = copy_cstr(m, esi, s);
    let s = m.r32(0x6B72FC);
    esi = copy_cstr(m, esi, s);
    m.w8(esi, 0);
    esi += 1;
    m.w8(esi, 0);
    esi += 1;
    let len = esi.wrapping_sub(u32::from_le_bytes([msg[8], msg[9], msg[10], msg[11]]));
    msg[12..16].copy_from_slice(&len.to_le_bytes());
    host.post(m, -1, &mut msg);
    let ptr = u32::from_le_bytes([msg[8], msg[9], msg[10], msg[11]]);
    if ptr != 0 {
        host.free(ptr);
        msg[8..12].copy_from_slice(&0u32.to_le_bytes());
    }
    msg[4] = 0x57;
    msg[5] = kind as u8;
    let own = u32::from(m.r8(0x6DE52D));
    let n = u32::from(m.r16(0x6DE4AA));
    if (own as i32) >= (n as i32) {
        msg[9] = 1;
        let sel = 0xffff_ffffu32.wrapping_sub(own) & 3;
        let tp = town(m, town_arg);
        msg[10] = m.r8(tp + sel + 0x6e0);
        msg[13] = m.r8(tp + sel + 0x6e4);
        msg[8] = town_arg as u8;
        msg[11] = m.r8(tp + sel + 0x6d4);
        msg[12] = m.r8(tp + sel + 0x6d8);
    } else {
        let mer = merchant(m, own);
        msg[9] = m.r8(mer + 0x1a);
        msg[10] = host.merchant_flag(mer);
        let home = u32::from(m.r8(mer + 0x19));
        msg[13] = m.r8(mer + home + 0x39c);
        msg[8] = m.r8(mer + 0x19);
        msg[11] = m.r8(mer + 0x1c);
        msg[12] = m.r8(mer + 0x1d);
    }
    host.post(m, -2, &mut msg);
    host.msg_dtor();
}
