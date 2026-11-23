//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X00460870` p2_scrollbar_ctor
pub fn f_00460870(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65ed82u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6985b4u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x14u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = esi.wrapping_add(0x94u32);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    ecx = edi;
    m.w32(esp.wrapping_add(0x1cu32), ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ebp = esi.wrapping_add(0x17cu32);
    m.w8(esp.wrapping_add(0x1cu32), (0x1u32) as u8);
    ecx = ebp;
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x264u32);
    m.w8(esp.wrapping_add(0x1cu32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x46cc20, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x1cu32), (0x3u32) as u8);
    m.w32(esi, 0x66e120u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    edx = m.r32(ebp);
    ecx = ebp;
    m.w8(esi.wrapping_add(0x360u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x36cu32), ebx);
    m.w32(esi.wrapping_add(0x368u32), ebx);
    m.w32(esi.wrapping_add(0x364u32), ebx);
    m.w32(esi.wrapping_add(0x374u32), ebx);
    m.w32(esi.wrapping_add(0x370u32), ebx);
    m.w32(esi.wrapping_add(0x378u32), ebx);
    m.w32(esi.wrapping_add(0x35cu32), ebx);
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(esi.wrapping_add(0x264u32));
    ecx = esi.wrapping_add(0x264u32);
    { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    edx = m.r32(edi);
    ecx = edi;
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = esi.wrapping_add(0x380u32);
    m.w32(esi.wrapping_add(0x37cu32), ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let t = m.r32(0x66a4d4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(esp.wrapping_add(0x14u32));
    eax = esi;
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebp = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X00460970` p2_scrollbar_ctor_named
pub fn f_00460970(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65edbeu32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(esp.wrapping_add(0x14u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x14u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = esi.wrapping_add(0x94u32);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    ecx = edi;
    m.w32(esp.wrapping_add(0x1cu32), ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ebp = esi.wrapping_add(0x17cu32);
    m.w8(esp.wrapping_add(0x1cu32), (0x1u32) as u8);
    ecx = ebp;
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x264u32);
    m.w8(esp.wrapping_add(0x1cu32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x46cc20, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x1cu32), (0x3u32) as u8);
    m.w32(esi, 0x66e120u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    edx = m.r32(ebp);
    ecx = ebp;
    m.w8(esi.wrapping_add(0x360u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x36cu32), ebx);
    m.w32(esi.wrapping_add(0x368u32), ebx);
    m.w32(esi.wrapping_add(0x364u32), ebx);
    m.w32(esi.wrapping_add(0x374u32), ebx);
    m.w32(esi.wrapping_add(0x370u32), ebx);
    m.w32(esi.wrapping_add(0x378u32), ebx);
    m.w32(esi.wrapping_add(0x35cu32), ebx);
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(esi.wrapping_add(0x264u32));
    ecx = esi.wrapping_add(0x264u32);
    { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    edx = m.r32(edi);
    ecx = edi;
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = esi.wrapping_add(0x380u32);
    m.w32(esi.wrapping_add(0x37cu32), ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let t = m.r32(0x66a4d4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(esp.wrapping_add(0x14u32));
    eax = esi;
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebp = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X00460A70` p2_scrollbar_dtor
pub fn f_00460a70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65edfau32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    m.w32(esp.wrapping_add(0x10u32), esi);
    m.w32(esi, 0x66e120u32);
    m.w32(esp.wrapping_add(0x1cu32), 0x3u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    eax = m.r32(esi.wrapping_add(0x17cu32));
    edi = esi.wrapping_add(0x17cu32);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    ecx = edi;
    m.w8(esi.wrapping_add(0x360u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x36cu32), ebx);
    m.w32(esi.wrapping_add(0x368u32), ebx);
    m.w32(esi.wrapping_add(0x364u32), ebx);
    m.w32(esi.wrapping_add(0x374u32), ebx);
    m.w32(esi.wrapping_add(0x370u32), ebx);
    m.w32(esi.wrapping_add(0x378u32), ebx);
    m.w32(esi.wrapping_add(0x35cu32), ebx);
    { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    edx = m.r32(esi.wrapping_add(0x264u32));
    ebp = esi.wrapping_add(0x264u32);
    ecx = ebp;
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(esi.wrapping_add(0x94u32));
    ecx = esi.wrapping_add(0x94u32);
    { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = esi.wrapping_add(0x380u32);
    m.w32(esi.wrapping_add(0x37cu32), ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    { let t = m.r32(0x66a4d4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = ebp;
    m.w8(esp.wrapping_add(0x1cu32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x46cd00, ecx, &a); }
    ecx = edi;
    m.w8(esp.wrapping_add(0x1cu32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x94u32);
    m.w8(esp.wrapping_add(0x1cu32), ((ebx & 0xff)) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x1cu32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x4b1640, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x14u32));
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebp = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X00460B70` p2_scrollbar_on_open
pub fn f_00460b70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x65ee1cu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                eax = m.r32(0x6c7cccu32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x24u32), ebx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esp.wrapping_add(0x30u32));
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6985b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x30u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x38u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6985c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x2cu32));
                ebp = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6985d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6985e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                edi = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6985f4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 9; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x34u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x38u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x30u32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x17cu32));
                ecx = esi.wrapping_add(0x17cu32);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w8(esp.wrapping_add(0x24u32), (0x1u32) as u8);
                ecx = esp.wrapping_add(0x30u32);
                pc = 8; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x38u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = esi.wrapping_add(0x94u32);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w8(esp.wrapping_add(0x24u32), (0x1u32) as u8);
                ecx = esp.wrapping_add(0x30u32);
                pc = 8; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = esp.wrapping_add(0x30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x264u32));
                ecx = esi.wrapping_add(0x264u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(esp.wrapping_add(0x24u32), (0x1u32) as u8);
                ecx = esp.wrapping_add(0x30u32);
                pc = 8; continue;
            }
            7 => {
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6985fcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esi.wrapping_add(0x35cu32), eax);
                ecx = esp.wrapping_add(0x30u32);
                m.w8(esp.wrapping_add(0x24u32), (0x1u32) as u8);
                if !fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 9; continue;
            }
            9 => {
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x24u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x2cu32);
                m.w32(esp.wrapping_add(0x24u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 11; continue;
            }
            10 => {
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x24u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x2cu32);
                m.w32(esp.wrapping_add(0x24u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x18);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00460EF0` p2_scrollbar_setup
pub fn f_00460ef0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x18);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x2cu32));
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0x34u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0x36cu32), eax);
                m.w32(esi.wrapping_add(0x378u32), eax);
                m.w32(esi.wrapping_add(0x374u32), ecx);
                eax = m.r32(edi.wrapping_add(0xcu32));
                ebx = m.r32(edi.wrapping_add(0x4u32));
                ebp = esi.wrapping_add(0x17cu32);
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                ebx = esi.wrapping_add(0x94u32);
                m.w32(esi.wrapping_add(0x370u32), eax);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edx = m.r32(ebx);
                ecx = ebx;
                m.w32(esi.wrapping_add(0x37cu32), eax);
                eax = m.r32(esp.wrapping_add(0x30u32));
                m.w32(esi.wrapping_add(0x364u32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0xcu32));
                edx = m.r32(edi.wrapping_add(0x4u32));
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(ebp);
                m.w32(esp.wrapping_add(0x34u32), ecx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x370u32));
                m.w32(esi.wrapping_add(0x368u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x364u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w8(esi.wrapping_add(0x360u32), (0x1u32) as u8);
                pc = 3; continue;
            }
            2 => {
                m.w8(esi.wrapping_add(0x360u32), (0x0u32) as u8);
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi.wrapping_add(0x264u32));
                ebp = esi.wrapping_add(0x264u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0x8u32));
                edx = m.r32(ebx);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(eax, m.r32(edi.wrapping_add(0x4u32)), 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esp.wrapping_add(0x38u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                m.w32(esp.wrapping_add(0x38u32), eax);
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                eax = esi.wrapping_add(0x17cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b50f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                eax = esi.wrapping_add(0x17cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xecu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebx);
                ecx = ebx;
                m.w8(esi.wrapping_add(0x2a4u32), (0x1u32) as u8);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = ebx;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(ebx);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x38u32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x38u32));
                edx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edi.wrapping_add(0x8u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edi.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = esi.wrapping_add(0x94u32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x1cu32), 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x17cu32));
                ebx = esi.wrapping_add(0x17cu32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = ebx;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(ebx);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x38u32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x38u32));
                ebx = m.r32(edi.wrapping_add(0x8u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x17cu32));
                ebx = esi.wrapping_add(0x17cu32);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0xcu32));
                edx = m.r32(ebx);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(edi.wrapping_add(0x8u32));
                eax = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x14u32), ecx);
                edx = m.r32(edi.wrapping_add(0x4u32));
                m.w32(esi.wrapping_add(0x18u32), edx);
                edx = m.r32(ebp);
                m.w32(esi.wrapping_add(0x2cu32), eax);
                eax = m.r32(edi.wrapping_add(0xcu32));
                ebx = m.r32(edi.wrapping_add(0x4u32));
                ecx = ebp;
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x30u32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x368u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46cd60, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                edx = esi.wrapping_add(0x380u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(edx, eax);
                ecx = m.r32(edi.wrapping_add(0x4u32));
                m.w32(edx.wrapping_add(0x4u32), ecx);
                eax = m.r32(edi.wrapping_add(0x8u32));
                m.w32(edx.wrapping_add(0x8u32), eax);
                ecx = m.r32(edi.wrapping_add(0xcu32));
                m.w32(edx.wrapping_add(0xcu32), ecx);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x368u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x36cu32));
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                m.w32(esp.wrapping_add(0x38u32), eax);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x38u32)) as i32); fp.push(v); }
                edx = m.r32(esi.wrapping_add(0x364u32));
                eax = m.r32(esi.wrapping_add(0x370u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                lift::fst32(m, esp.wrapping_add(0x34u32), fp.st(0)); fp.pop();
                m.w32(esp.wrapping_add(0x38u32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x38u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x38u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x34u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x38u32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x38u32), fp.st(0)); fp.pop();
                pc = 8; continue;
            }
            7 => {
                m.w32(esp.wrapping_add(0x38u32), 0x0u32);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = esi.wrapping_add(0x94u32);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                { let v = f64::from(m.r32(esp.wrapping_add(0x2cu32)) as i32); fp.push(v); }
                edi = eax;
                lift::fst32(m, esp.wrapping_add(0x34u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x34u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x38u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ebx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(ebp);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                let (r, f) = lift::add(edi, ebx, 32); fl = f;
                edi = r;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                m.w32(esp.wrapping_add(0x24u32), edi);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x18);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00461590` p2_scrollbar_update
pub fn f_00461590(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x2c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi.wrapping_add(0x17cu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x360u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esi.wrapping_add(0x364u32));
                edx = m.r32(esi.wrapping_add(0x370u32));
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x36cu32));
                fl = lift::sub(edx, ecx, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x374u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, ecx, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(esi.wrapping_add(0x36cu32), eax);
                pc = 6; continue;
            }
            5 => {
                m.w32(esi.wrapping_add(0x36cu32), ecx);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x264u32));
                edi = esi.wrapping_add(0x264u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x368u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x36cu32));
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                edx = m.r32(esi.wrapping_add(0x364u32));
                eax = m.r32(esi.wrapping_add(0x370u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                m.w32(esp.wrapping_add(0x10u32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                pc = 9; continue;
            }
            8 => {
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = esi.wrapping_add(0x94u32);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                { let v = f64::from(m.r32(esp.wrapping_add(0x18u32)) as i32); fp.push(v); }
                ebx = eax;
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                edx = m.r32(edi);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                ecx = edi;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                m.w32(esp.wrapping_add(0x28u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x461c70, ecx, &a); }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x264u32));
                ebp = esi.wrapping_add(0x264u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x368u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x36cu32));
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                edx = m.r32(esi.wrapping_add(0x364u32));
                eax = m.r32(esi.wrapping_add(0x370u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x18u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                pc = 13; continue;
            }
            12 => {
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                edi = esi.wrapping_add(0x94u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                pc = 27; continue;
            }
            14 => {
                edi = esi.wrapping_add(0x94u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 28; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x360u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 23; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esi.wrapping_add(0x36cu32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 23; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::sub(eax, m.r32(esi.wrapping_add(0x374u32)), 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x36cu32), eax);
                if !fl.sf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                m.w32(esi.wrapping_add(0x36cu32), ebx);
                pc = 19; continue;
            }
            19 => {
                edx = m.r32(esi.wrapping_add(0x264u32));
                ebp = esi.wrapping_add(0x264u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x368u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x36cu32));
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                edx = m.r32(esi.wrapping_add(0x364u32));
                eax = m.r32(esi.wrapping_add(0x370u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                m.w32(esp.wrapping_add(0x1cu32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                pc = 22; continue;
            }
            21 => {
                m.w32(esp.wrapping_add(0x10u32), ebx);
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                ebx = eax;
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x20u32);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                m.w32(esp.wrapping_add(0x28u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x461c70, ecx, &a); }
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(esi.wrapping_add(0x264u32));
                ebp = esi.wrapping_add(0x264u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x368u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x36cu32));
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                edx = m.r32(esi.wrapping_add(0x364u32));
                eax = m.r32(esi.wrapping_add(0x370u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                m.w32(esp.wrapping_add(0x1cu32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                pc = 26; continue;
            }
            25 => {
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                pc = 27; continue;
            }
            27 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                ebx = eax;
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x20u32);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x20u32);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x28u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 28; continue;
            }
            28 => {
                ebp = esi.wrapping_add(0x264u32);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x4b1bf0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x24u32), edx);
                edx = m.r32(edi);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(ebp);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = ebp;
                { let v = f64::from(m.r32(esp.wrapping_add(0x24u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x368u32));
                edx = m.r32(esi.wrapping_add(0x364u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edi = m.r32(esi.wrapping_add(0x370u32));
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                let (r, f) = lift::sub(edx, edi, 32); fl = f;
                edx = r;
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d / v); }
                m.w32(esp.wrapping_add(0x1cu32), edx);
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esi.wrapping_add(0x36cu32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                m.w32(esp.wrapping_add(0x30u32), eax);
                eax = esp.wrapping_add(0x2cu32);
                let (r, f) = lift::add(esi, 0x4cu32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                m.w32(esp.wrapping_add(0x44u32), edx);
                { let t = m.r32(0x66a4ccu32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                pc = 30; continue;
            }
            30 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x2c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047C2C0` personal_letters_side_menu_dtor
pub fn f_0047c2c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x65fa86u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                m.w32(esp.wrapping_add(0x10u32), edi);
                m.w32(edi, 0x66f0a8u32);
                m.w32(esp.wrapping_add(0x20u32), 0x3u32);
                esi = edi.wrapping_add(0x1c0u32);
                ebx = 0x5u32;
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                ebp = edi.wrapping_add(0x648u32);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x431060, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0xa0u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(edi.wrapping_add(0xa0u32), ebx);
                pc = 4; continue;
            }
            4 => {
                esi = edi.wrapping_add(0x9fcu32);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x409d70, ecx, &a); }
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x440ab0, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb490, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(edi.wrapping_add(0xb8u32), ebx);
                pc = 6; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x14u32), esi);
                ecx = esi.wrapping_add(0x1290u32);
                m.w8(esp.wrapping_add(0x20u32), (0xcu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4ace40, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4ace40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = esi.wrapping_add(0x1008u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w8(esp.wrapping_add(0x30u32), (0xbu32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4ace40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                edx = esi.wrapping_add(0xd80u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x30u32), (0xau32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = esi.wrapping_add(0xc98u32);
                m.w8(esp.wrapping_add(0x20u32), (0x9u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                eax = esi.wrapping_add(0x9e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x30u32), (0x8u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = esi.wrapping_add(0x728u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w8(esp.wrapping_add(0x30u32), (0x7u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = esi.wrapping_add(0x640u32);
                m.w8(esp.wrapping_add(0x20u32), (0x6u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                edx = esi.wrapping_add(0x388u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x30u32), (0x5u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                eax = esi.wrapping_add(0xd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x30u32), (0x4u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = esi;
                m.w8(esp.wrapping_add(0x20u32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x41c3a0, ecx, &a); }
                ecx = ebp;
                m.w8(esp.wrapping_add(0x20u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x430fe0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                ecx = edi.wrapping_add(0xd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w8(esp.wrapping_add(0x30u32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = edi;
                m.w32(esp.wrapping_add(0x20u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x440a20, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x8u32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047C4A0` personal_letters_side_menu_reset_state
pub fn f_0047c4a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = 0x5u32;
                edi = esi.wrapping_add(0x1c0u32);
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = esi.wrapping_add(0x648u32);
                { let a = []; eax = h.call_at(esp, m, 0x431060, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xa0u32), edi);
                pc = 4; continue;
            }
            4 => {
                ecx = esi.wrapping_add(0x9fcu32);
                { let a = []; eax = h.call_at(esp, m, 0x409d70, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x440ab0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb490, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xb8u32), edi);
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047C820` personal_letters_side_menu_set_tab
pub fn f_0047c820(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x4u32)))) & 0xffff);
                esp = esp.wrapping_sub(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xccu32)))) & 0xffff);
                m.w16(esi.wrapping_add(0xd4u32), ((eax & 0xffff)) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x47c520, ecx, &a); }
                edi = eax;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(edi, ebx, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                m.w16(esi.wrapping_add(0xccu32), ((ebx & 0xffff)) as u16);
                pc = 13; continue;
            }
            2 => {
                fl = lift::sub(u32::from(m.r8(esp.wrapping_add(0x28u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 10; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebp & 0xffff), 16).1;
                if !fl.cf { pc = 10; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(4))))) & 0xffff);
                m.w16(esi.wrapping_add(0xd2u32), ((ebx & 0xffff)) as u16);
                fl = lift::sub(u32::from(m.r16(edi)), (edx & 0xffff), 16).1;
                if fl.zf { pc = 12; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                eax = ecx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(eax.wrapping_mul(4))))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if !fl.cf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = ecx.wrapping_add(0x1u32);
                m.w16(esi.wrapping_add(0xd2u32), ((eax & 0xffff)) as u16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(ecx.wrapping_mul(4)))), (edx & 0xffff), 16).1;
                if !fl.zf { pc = 6; continue; }
                pc = 8; continue;
            }
            8 => {
                pc = 12; continue;
            }
            9 => {
                ecx = m.r32(0x6cbb10u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 11; continue;
            }
            10 => {
                ecx = m.r32(0x6cbb10u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 11; continue;
            }
            11 => {
                m.w16(esi.wrapping_add(0xd2u32), (0xffffu32) as u16);
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_add(0x4);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xccu32)))) & 0xffff);
                ebp = esi.wrapping_add(0x648u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebp;
                m.w32(esi.wrapping_add(0xa0u32), edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xccu32)))) & 0xffff);
                eax = m.r32(esi.wrapping_add(0x650u32));
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esi.wrapping_add(0xc8u32))), 16).1;
                edi = esi.wrapping_add(0x650u32);
                ecx = edi;
                edx = (edx & !0xff) | ((u32::from((!fl.cf && !fl.zf))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x461dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x9ecu32)))) & 0xffff);
                m.w16(esi.wrapping_add(0xceu32), ((eax & 0xffff)) as u16);
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0xb4u32), ebx);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047C9B0` personal_letters_side_menu_load_layout
pub fn f_0047c9b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x65faa8u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(0x6beee0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9ba4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9bacu32);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x54u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x44u32));
                eax = m.r32(0x6beee0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9bbcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9bc4u32);
                ecx = 0x6dd524u32;
                m.w8(esp.wrapping_add(0x54u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                m.w8(esp.wrapping_add(0x3cu32), (0x2u32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9bd4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9be4u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = esp.wrapping_add(0x1cu32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4417d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = 0x1u32;
                edi = esi.wrapping_add(0x1c0u32);
                pc = 1; continue;
            }
            1 => {
                eax = ebx.wrapping_add(ebp);
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebp, 0x6u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9bf4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9c00u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x18u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x650u32));
                edi = esi.wrapping_add(0x650u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9c10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9c1cu32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x44u32));
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9c2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9c38u32);
                m.w16(esi.wrapping_add(0xc4u32), ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w16(esi.wrapping_add(0xc6u32), ((eax & 0xffff)) as u16);
                eax = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9c48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9c54u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x1cu32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x9fcu32));
                ecx = esi.wrapping_add(0x9fcu32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc6u32)))) & 0xffff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                edx = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x2cu32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc6u32)))) & 0xffff);
                m.w16(esi.wrapping_add(0xc2u32), ((ecx & 0xffff)) as u16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9c64u32);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(eax, 0x2u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9c68u32);
                m.w16(esi.wrapping_add(0xbcu32), ((eax & 0xffff)) as u16);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc4u32)))) & 0xffff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x2cu32)))) & 0xffff);
                let (r, f) = lift::add((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xbcu32))), 16); fl = f;
                eax = (eax & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::sub((ecx & 0xffff), 0x6u32, 16); fl = f;
                ecx = (ecx & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::add(eax, 0x2u32, 32); fl = f;
                eax = r;
                m.w16(esi.wrapping_add(0xcau32), ((ecx & 0xffff)) as u16);
                ecx = edi;
                m.w16(esi.wrapping_add(0xbeu32), ((eax & 0xffff)) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x461fd0, ecx, &a); }
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x2cu32)))) & 0xffff);
                let (r, f) = lift::sub((edx & 0xffff), (eax & 0xffff), 16); fl = f;
                edx = (edx & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::sub(edx, 0x8u32, 32); fl = f;
                edx = r;
                m.w16(esi.wrapping_add(0xc0u32), ((edx & 0xffff)) as u16);
                eax = m.r32(0x6cbd5cu32);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(0x6cbd54u32);
                ecx = eax;
                eax = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edx = m.r32(esi);
                ecx = esi;
                m.w8(esi.wrapping_add(0x1d64u32), (0x0u32) as u8);
                let (r, f) = lift::sub(eax, 0x3u32, 32); fl = f;
                eax = r;
                m.w16(esi.wrapping_add(0xc8u32), ((eax & 0xffff)) as u16);
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x44u32);
                m.w32(esp.wrapping_add(0x3cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x30);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047CCC0` personal_letters_side_menu_update
pub fn f_0047ccc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x3c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esp.wrapping_add(0x20u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                ecx = m.r32(0x6cbb10u32);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xccu32))), 16).1;
                edx = m.r32(ecx);
                if !fl.cf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w16(esi.wrapping_add(0xd2u32), (0xffffu32) as u16);
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            3 => {
                ecx = m.r32(0x6de4b4u32);
                eax = m.r32(esi.wrapping_add(0xb0u32));
                fl = lift::sub(ecx, eax, 32).1;
                if fl.cf { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x47f3b0, ecx, &a); }
                pc = 9; continue;
            }
            5 => {
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6cbb10u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0x560u32));
                edi = esi.wrapping_add(0x560u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(edi);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(0x6de4b4u32);
                ecx = m.r32(esi.wrapping_add(0xacu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d500u32);
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(0x6dfc14u32);
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x24u32), 0x71u32);
                m.w32(esp.wrapping_add(0x28u32), edx);
                m.w32(esp.wrapping_add(0x2cu32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6de4b4u32);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esi.wrapping_add(0xacu32), ecx);
                pc = 12; continue;
            }
            12 => {
                ecx = esi.wrapping_add(0x1c0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x47c820, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x3c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            14 => {
                ecx = esi.wrapping_add(0x2a8u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x47c820, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x3c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            16 => {
                ecx = esi.wrapping_add(0x390u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x47c820, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x3c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            18 => {
                ecx = esi.wrapping_add(0x478u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 21; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(0x6cbb10u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0xac0u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 51; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esi.wrapping_add(0x9fcu32));
                ecx = esi.wrapping_add(0x9fcu32);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x3c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            21 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6dcbeeu32))) & 0xff);
                edi = esi.wrapping_add(0x560u32);
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                let (r, f) = lift::shr((ecx & 0xff), 0x1u32, 8, fl); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (eax & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) | (eax & 0xff); fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(0x6dcbeeu32, ((ecx & 0xff)) as u8);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = eax | edx; fl = lift::logic(r, 32);
                eax = r;
                if fl.zf { pc = 28; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = esp.wrapping_add(0x34u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ecx = m.r32(0x6cbb10u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 27; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xccu32))), 16).1;
                if !fl.cf { pc = 27; continue; }
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(0x6dfc14u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x38u32), ecx);
                ecx = esp.wrapping_add(0x34u32);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x34u32), 0x6fu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(edx.wrapping_mul(4))))) & 0xffff);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(edx.wrapping_mul(4))))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if !fl.cf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x14u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x5u32);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = edi.wrapping_add(ecx.wrapping_mul(4));
                ecx = edi.wrapping_add(eax.wrapping_mul(4));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                let (r, f) = lift::add(u32::from(m.r16(esi.wrapping_add(0xd2u32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esi.wrapping_add(0xd2u32), (r) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(edx.wrapping_mul(4))))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if fl.cf { pc = 25; continue; }
                pc = 26; continue;
            }
            26 => {
                let (r, f) = lift::sub(u32::from(m.r16(esi.wrapping_add(0xccu32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esi.wrapping_add(0xccu32), (r) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xccu32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = esi.wrapping_add(0x648u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xccu32)))) & 0xffff);
                edx = m.r32(esi.wrapping_add(0x650u32));
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xc8u32))), 16).1;
                ecx = esi.wrapping_add(0x650u32);
                eax = (eax & !0xff) | ((u32::from((!fl.cf && !fl.zf))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w16(esi.wrapping_add(0xceu32), (0x0u32) as u16);
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w16(esi.wrapping_add(0xd2u32), (0xffffu32) as u16);
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x3c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            28 => {
                ecx = esi.wrapping_add(0x648u32);
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w16(esi.wrapping_add(0xd0u32), (0xffffu32) as u16);
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x9ecu32)))) & 0xffff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w16(esi.wrapping_add(0xceu32), ((ecx & 0xffff)) as u16);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x3c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            30 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 51; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 40; continue; }
                pc = 32; continue;
            }
            32 => {
                eax = m.r32(0x6dfc14u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
                pc = 48; continue;
            }
            34 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0xau32)))) & 0xffff);
                edi = (edi & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(eax, edi, 32).1;
                if !fl.cf { pc = 38; continue; }
                pc = 35; continue;
            }
            35 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd4u32)))) & 0xffff);
                ebp = m.r32(0x6dd730u32);
                pc = 36; continue;
            }
            36 => {
                ecx = eax;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x4u32)))) & 0xff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r8(ebx.wrapping_add(0x6c0198u32)))) & 0xffff);
                fl = lift::sub((ebx & 0xffff), (edx & 0xffff), 16).1;
                if fl.zf { pc = 39; continue; }
                pc = 37; continue;
            }
            37 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(eax, edi, 32).1;
                if fl.cf { pc = 36; continue; }
                pc = 38; continue;
            }
            38 => {
                fl = lift::sub(u32::from(m.r16(0x6dd736u32)), 0xffffu32, 16).1;
                ecx = (ecx & !0xff) | ((u32::from((!fl.cf && !fl.zf))) & 0xff);
                m.w8(esp.wrapping_add(0x10u32), ((ecx & 0xff)) as u8);
                pc = 48; continue;
            }
            39 => {
                fl = lift::sub(eax, 0xffffu32, 32).1;
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                pc = 47; continue;
            }
            40 => {
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                eax = m.r32(0x6dfc14u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
                pc = 48; continue;
            }
            42 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xau32)))) & 0xffff);
                edi = (edi & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(eax, edi, 32).1;
                if !fl.cf { pc = 46; continue; }
                pc = 43; continue;
            }
            43 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd4u32)))) & 0xffff);
                pc = 44; continue;
            }
            44 => {
                ebx = m.r32(0x6dd730u32);
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x4u32)))) & 0xff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r8(ebx.wrapping_add(0x6c0198u32)))) & 0xffff);
                fl = lift::sub((ebx & 0xffff), (edx & 0xffff), 16).1;
                if fl.zf { pc = 52; continue; }
                pc = 45; continue;
            }
            45 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(eax, edi, 32).1;
                if fl.cf { pc = 44; continue; }
                pc = 46; continue;
            }
            46 => {
                fl = lift::sub((ebp & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                eax = (eax & !0xff) | ((u32::from(fl.cf)) & 0xff);
                pc = 47; continue;
            }
            47 => {
                m.w8(esp.wrapping_add(0x10u32), ((eax & 0xff)) as u8);
                pc = 48; continue;
            }
            48 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 50; continue; }
                pc = 49; continue;
            }
            49 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 51; continue; }
                pc = 50; continue;
            }
            50 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd4u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x47c820, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 51; continue;
            }
            51 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x3c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            52 => {
                let r = ebp & 0xffffu32; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                m.w8(esp.wrapping_add(0x10u32), ((edx & 0xff)) as u8);
                pc = 48; continue;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047D260` personal_letters_side_menu_layout_buttons
pub fn f_0047d260(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0xc);
                eax = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4417d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(0x6cbd9cu32);
                let (r, f) = lift::add(esi, 0x1c0u32, 32); fl = f;
                esi = r;
                edi = esi;
                ebp = 0x5u32;
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = 0x92492493u32;
                edi = 0x5u32;
                { let r = i64::from(eax as i32) * i64::from(ebx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                ebx = edx;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = esi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047D2F0` personal_letters_side_menu_show
pub fn f_0047d2f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x38);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ecx = esp.wrapping_add(0x30u32);
                m.w32(esp.wrapping_add(0xcu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(0x6dfc14u32);
                ecx = esp.wrapping_add(0x30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x34u32), 0x71u32);
                m.w32(esp.wrapping_add(0x38u32), eax);
                m.w32(esp.wrapping_add(0x3cu32), 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6de4b4u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1d64u32)))) & 0xff);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w32(esi.wrapping_add(0xacu32), edx);
                if !fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = esi.wrapping_add(0x648u32);
                ebp = 0x5u32;
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0xcu32));
                let (r, f) = lift::sub(edi, 0xe8u32, 32); fl = f;
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w8(esi.wrapping_add(0x1d64u32), (0x1u32) as u8);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6cbd5cu32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                ebp = m.r32(0x6cbd54u32);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                eax = eax.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc8u32)))) & 0xffff);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::imul(edx, ecx); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xbcu32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xcau32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x10u32);
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let t = m.r32(0x66a4a4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                edi = esi.wrapping_add(0x650u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x460ef0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1c0u32));
                ecx = esi.wrapping_add(0x1c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w16(esi.wrapping_add(0xceu32), (0x0u32) as u16);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x2a8u32));
                ecx = esi.wrapping_add(0x2a8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x390u32));
                ecx = esi.wrapping_add(0x390u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x478u32));
                ecx = esi.wrapping_add(0x478u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = esi.wrapping_add(0x648u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xccu32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xccu32)))) & 0xffff);
                edx = m.r32(edi);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xc8u32))), 16).1;
                ebp = 0xffffu32;
                m.w16(esi.wrapping_add(0xceu32), (0x0u32) as u16);
                m.w16(esi.wrapping_add(0xd0u32), ((ebp & 0xffff)) as u16);
                ecx = (ecx & !0xff) | ((u32::from((!fl.cf && !fl.zf))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w16(esi.wrapping_add(0xceu32), (0x0u32) as u16);
                m.w16(esi.wrapping_add(0xd0u32), ((ebp & 0xffff)) as u16);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x38);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047D4D0` personal_letters_side_menu_show_buttons
pub fn f_0047d4d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xccu32)))) & 0xffff);
    ecx = esi.wrapping_add(0x648u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xccu32)))) & 0xffff);
    edx = m.r32(esi.wrapping_add(0x650u32));
    fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xc8u32))), 16).1;
    ecx = esi.wrapping_add(0x650u32);
    eax = (eax & !0xff) | ((u32::from((!fl.cf && !fl.zf))) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w16(esi.wrapping_add(0xceu32), (0x0u32) as u16);
    m.w16(esi.wrapping_add(0xd0u32), (0xffffu32) as u16);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X005A28C0` money_lender_side_menu_load_layout
pub fn f_005a28c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6650e2u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x14);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), ebp);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), 0x0u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x66de5cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp.wrapping_add(0xacu32));
                esi = ebp.wrapping_add(0xacu32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = ebp.wrapping_add(0x194u32);
                ebx = 0x5u32;
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                ebx = ebp.wrapping_add(0x620u32);
                edi = 0x69b724u32;
                ebp = 0xff000000u32;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                m.w32(esi.wrapping_add(0xc0u32), ebp);
                { let t = m.r32(eax.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, 0x20u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, 0x69b73cu32, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c5224u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c5230u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax;
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(eax.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x1cu32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x2cu32), 0xffffffffu32);
                m.w16(edx.wrapping_add(0x6e0u32), ((ecx & 0xffff)) as u16);
                ecx = esp.wrapping_add(0x34u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x20);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A2A00` money_lender_side_menu_prerender
pub fn f_005a2a00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x1c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esi = m.r32(edi.wrapping_add(0x1cu32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xa4u32)))) & 0xff);
                edx = m.r32(0x66d504u32);
                ebp = m.r32(edi.wrapping_add(0x14u32));
                ebx = m.r32(edi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(edi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::sub(0, esi, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w8(esp.wrapping_add(0x1bu32), ((ecx & 0xff)) as u8);
                m.w32(edi.wrapping_add(0x94u32), edx);
                m.w8(edi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x30u32), esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ecx = 0x6dce78u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(0x8);
                edx = m.r32(0x6cbd94u32);
                ecx = esp;
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ecx, ebp);
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w32(ecx.wrapping_add(0x8u32), esi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = edi.wrapping_add(0xacu32);
                m.w32(esp.wrapping_add(0x14u32), 0x6u32);
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = esp;
                eax = m.r32(esi);
                m.w32(ecx, ebp);
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w32(ecx.wrapping_add(0x8u32), edx);
                ecx = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(edi.wrapping_add(0x94u32), ecx);
                m.w8(edi.wrapping_add(0xa4u32), ((edx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A2B20` money_lender_side_menu_layout_buttons
pub fn f_005a2b20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4417d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi.wrapping_add(0xacu32));
                esi = edi.wrapping_add(0xacu32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = m.r32(esp.wrapping_add(0xcu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6e0u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                ecx = m.r32(edi.wrapping_add(0x18u32));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esi);
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x10u32), edx);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::sar(ebx, 0x1u32, 32, fl); fl = f;
                ebx = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(esi, 0x5u32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = ebx.wrapping_add(ebx.wrapping_mul(2));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6e0u32)))) & 0xffff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(edi.wrapping_add(0x18u32));
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 5; continue;
            }
            3 => {
                fl = lift::sub(esi, 0x3u32, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x10u32)), ebx, 32); fl = f;
                m.w32(esp.wrapping_add(0x10u32), r);
                pc = 5; continue;
            }
            5 => {
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(eax.wrapping_mul(4));
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(edi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32));
                ecx = edi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x6u32, 32).1;
                m.w32(esp.wrapping_add(0x10u32), edx);
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A2BF0` money_lender_side_menu_show
pub fn f_005a2bf0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                esi = edi.wrapping_add(0xacu32);
                m.w32(esp.wrapping_add(0xcu32), eax);
                ecx = esi;
                m.w8(edi.wrapping_add(0x6e2u32), (0xffu32) as u8);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0x2cu32));
                ebx = m.r32(edi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x18u32), 0x64u32);
                eax = ecx;
                ecx = m.r32(edi.wrapping_add(0x14u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x10u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6e0u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esi);
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x6e3u32)))) & 0xff);
                ecx = ebx;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 8; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(ebp, 0x5u32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6e0u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = ebx.wrapping_add(ebx.wrapping_mul(2));
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(edi.wrapping_add(0x18u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x18u32), edx);
                pc = 6; continue;
            }
            4 => {
                fl = lift::sub(ebp, 0x3u32, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x18u32)), ebx, 32); fl = f;
                m.w32(esp.wrapping_add(0x18u32), r);
                pc = 6; continue;
            }
            6 => {
                ecx = ebp.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, ebp, 32); fl = f;
                ecx = r;
                edx = ebp.wrapping_add(ecx.wrapping_mul(4));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esi = edi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp.wrapping_add(0x31u32);
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, 0x6u32, 32).1;
                m.w32(esp.wrapping_add(0x18u32), edx);
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w8(edi.wrapping_add(0x6e3u32), (0x1u32) as u8);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_add(0x4);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(edi.wrapping_add(0x6e4u32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(0x6e5534u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A2D60` money_lender_side_menu_show_buttons
pub fn f_005a2d60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx.wrapping_add(0xacu32);
                edi = 0x6u32;
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A2E80` money_lender_side_menu_update
pub fn f_005a2e80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                ecx = m.r32(0x6e5534u32);
                edi = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = m.r32(0x66d4fcu32);
                ebx = m.r32(0x66d504u32);
                let (r, f) = lift::sub(0, (eax & 0xff), 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(edi, ebp, 32).1;
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::sub(edi, ebx, 32).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ecx | edx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic((ecx & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_add(0x4);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x6e4u32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e5534u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6e2u32)))) & 0xff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w8(esp.wrapping_add(0x13u32), ((edx & 0xff)) as u8);
                ebp = esi.wrapping_add(0xacu32);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6e4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                eax = m.r32(0x6cc7e0u32);
                m.w8(eax.wrapping_add(0xfcu32), (0x0u32) as u8);
                m.w16(eax.wrapping_add(0xecu32), (0xcdu32) as u16);
                m.w8(esi.wrapping_add(0x6e4u32), (0x0u32) as u8);
                pc = 9; continue;
            }
            9 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6e2u32)))) & 0xff);
                fl = lift::sub(ebx, ecx, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w8(esi.wrapping_add(0x6e2u32), ((ebx & 0xff)) as u8);
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebx, 0x6u32, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 12; continue;
            }
            12 => {
                pc = 16; continue;
            }
            13 => {
                edi = esi.wrapping_add(0xacu32);
                ebp = 0x6u32;
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 14; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w8(esi.wrapping_add(0x6e2u32), ((ebx & 0xff)) as u8);
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6e2u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x6u32, 8).1;
                if !fl.cf { pc = 19; continue; }
                pc = 17; continue;
            }
            17 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32));
                ecx = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32);
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6e2u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6e5534u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5b32d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cc7e0u32);
                m.w8(eax.wrapping_add(0xfcu32), (0x0u32) as u8);
                m.w16(eax.wrapping_add(0xecu32), (0xcdu32) as u16);
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(0x6e5534u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x440f70, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(0x6cc7e0u32);
                ecx = m.r32(eax.wrapping_add(0xe8u32));
                edx = m.r32(eax.wrapping_add(0xd0u32));
                ecx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 25; continue; }
                pc = 22; continue;
            }
            22 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(0x6cc7e0u32);
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(eax.wrapping_add(0xe8u32));
                eax = m.r32(eax.wrapping_add(0xd0u32));
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                fl = lift::sub(u32::from(m.r8(ecx)), 0x0u32, 8).1;
                if !fl.zf { pc = 28; continue; }
                pc = 26; continue;
            }
            26 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 28; continue;
            }
            28 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AC810` market_hall_side_menu_prerender
pub fn f_005ac810(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x1c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esi = m.r32(edi.wrapping_add(0x1cu32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xa4u32)))) & 0xff);
                edx = m.r32(0x66d504u32);
                ebp = m.r32(edi.wrapping_add(0x14u32));
                ebx = m.r32(edi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(edi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::sub(0, esi, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w8(esp.wrapping_add(0x1bu32), ((ecx & 0xff)) as u8);
                m.w32(edi.wrapping_add(0x94u32), edx);
                m.w8(edi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x30u32), esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ecx = 0x6dce78u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(0x8);
                edx = m.r32(0x6cbd94u32);
                ecx = esp;
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ecx, ebp);
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w32(ecx.wrapping_add(0x8u32), esi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = edi.wrapping_add(0xacu32);
                m.w32(esp.wrapping_add(0x14u32), 0x3u32);
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = esp;
                eax = m.r32(esi);
                m.w32(ecx, ebp);
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w32(ecx.wrapping_add(0x8u32), edx);
                ecx = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(edi.wrapping_add(0x94u32), ecx);
                m.w8(edi.wrapping_add(0xa4u32), ((edx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AC930` market_hall_side_menu_layout_buttons
pub fn f_005ac930(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4417d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xacu32));
                edi = esi.wrapping_add(0xacu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                ebx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = m.r32(esp.wrapping_add(0xcu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x3c8u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                ecx = edi;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(edi);
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esi = eax;
                ebx = 0x3u32;
                ecx = esi;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(edi);
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(edx, esi, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AC9C0` market_hall_side_menu_show
pub fn f_005ac9c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x14);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                esi = edi.wrapping_add(0xacu32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = esi;
                m.w8(edi.wrapping_add(0x3cau32), (0xffu32) as u8);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x1cu32), 0x64u32);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = esi;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi.wrapping_add(0x14u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x3c8u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(edi.wrapping_add(0x18u32));
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x3cbu32)))) & 0xff);
                ecx = ebx;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = edi.wrapping_add(0x368u32);
                m.w32(esp.wrapping_add(0x10u32), 0x3u32);
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, 0x20u32, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if !fl.zf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w8(edi.wrapping_add(0x3cbu32), (0x1u32) as u8);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_add(0x4);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(edi.wrapping_add(0x3ccu32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6e5584u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005ACB10` market_hall_side_menu_show_buttons
pub fn f_005acb10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx.wrapping_add(0xacu32);
                edi = 0x3u32;
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005ACC30` market_hall_side_menu_update
pub fn f_005acc30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                ecx = m.r32(0x6e5584u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x94u32));
                edx = m.r32(0x66d4fcu32);
                let (r, f) = lift::sub(0, (eax & 0xff), 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(ecx, edx, 32).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::logic((edx & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_add(0x4);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(esi.wrapping_add(0x3ccu32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e5584u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 30; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3cau32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                m.w8(esp.wrapping_add(0xfu32), ((edx & 0xff)) as u8);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebp = esi.wrapping_add(0xacu32);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3ccu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                m.w8(esi.wrapping_add(0x3ccu32), (0x0u32) as u8);
                pc = 9; continue;
            }
            9 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3cau32)))) & 0xff);
                fl = lift::sub(ebx, ecx, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w8(esi.wrapping_add(0x3cau32), ((ebx & 0xff)) as u8);
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebx, 0x3u32, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 12; continue;
            }
            12 => {
                pc = 16; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = esi.wrapping_add(0xacu32);
                ebp = 0x3u32;
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 14; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w8(esi.wrapping_add(0x3cau32), ((ebx & 0xff)) as u8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3cau32)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((eax & 0xff), 0x3u32, 8).1;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                if !fl.cf { pc = 19; continue; }
                pc = 17; continue;
            }
            17 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32));
                ecx = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32);
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3cau32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x7u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6e5584u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5ddcd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(0x6e5584u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x440f70, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 21; continue;
            }
            21 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x8u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(0x6cc7e0u32);
                ecx = m.r32(eax.wrapping_add(0xe8u32));
                edx = m.r32(eax.wrapping_add(0xd0u32));
                ecx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x2u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 27; continue; }
                pc = 24; continue;
            }
            24 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x8u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(0x6cc7e0u32);
                pc = 27; continue;
            }
            27 => {
                edx = m.r32(eax.wrapping_add(0xe8u32));
                eax = m.r32(eax.wrapping_add(0xd0u32));
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x3u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 30; continue; }
                pc = 28; continue;
            }
            28 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x8u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 30; continue;
            }
            30 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x460870 => f_00460870(m, h, ecx, esp),
        0x460970 => f_00460970(m, h, ecx, esp),
        0x460a70 => f_00460a70(m, h, ecx, esp),
        0x460b70 => f_00460b70(m, h, ecx, esp),
        0x460ef0 => f_00460ef0(m, h, ecx, esp),
        0x461590 => f_00461590(m, h, ecx, esp),
        0x47c2c0 => f_0047c2c0(m, h, ecx, esp),
        0x47c4a0 => f_0047c4a0(m, h, ecx, esp),
        0x47c820 => f_0047c820(m, h, ecx, esp),
        0x47c9b0 => f_0047c9b0(m, h, ecx, esp),
        0x47ccc0 => f_0047ccc0(m, h, ecx, esp),
        0x47d260 => f_0047d260(m, h, ecx, esp),
        0x47d2f0 => f_0047d2f0(m, h, ecx, esp),
        0x47d4d0 => f_0047d4d0(m, h, ecx, esp),
        0x5a28c0 => f_005a28c0(m, h, ecx, esp),
        0x5a2a00 => f_005a2a00(m, h, ecx, esp),
        0x5a2b20 => f_005a2b20(m, h, ecx, esp),
        0x5a2bf0 => f_005a2bf0(m, h, ecx, esp),
        0x5a2d60 => f_005a2d60(m, h, ecx, esp),
        0x5a2e80 => f_005a2e80(m, h, ecx, esp),
        0x5ac810 => f_005ac810(m, h, ecx, esp),
        0x5ac930 => f_005ac930(m, h, ecx, esp),
        0x5ac9c0 => f_005ac9c0(m, h, ecx, esp),
        0x5acb10 => f_005acb10(m, h, ecx, esp),
        0x5acc30 => f_005acc30(m, h, ecx, esp),
        _ => return None,
    }))
}
