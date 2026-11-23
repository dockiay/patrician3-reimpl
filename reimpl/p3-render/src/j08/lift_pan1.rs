//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X00430FE0` list_scrollbar_dtor
pub fn f_00430fe0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65d5dbu32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    m.w32(esp.wrapping_add(0x4u32), ecx);
    m.w32(ecx, 0x66cfc8u32);
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    esi = ecx.wrapping_add(0x8u32);
    m.w8(ecx.wrapping_add(0x398u32), ((eax & 0xff)) as u8);
    m.w32(ecx.wrapping_add(0x39cu32), eax);
    m.w32(ecx.wrapping_add(0x3a0u32), eax);
    m.w32(ecx.wrapping_add(0x3a4u32), eax);
    m.w32(ecx.wrapping_add(0x3a8u32), eax);
    m.w32(ecx.wrapping_add(0x3acu32), eax);
    m.w32(ecx.wrapping_add(0x3b0u32), eax);
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), eax);
    { let a = []; eax = h.call_at(esp, m, 0x461bd0, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x460a70, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

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

/// `0X00460DA0` p2_scrollbar_draw
pub fn f_00460da0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x20);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                edi = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x4b2200, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi.wrapping_add(0x35cu32));
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = m.r32(edx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x35cu32));
                eax = m.r32(ecx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x40u32));
                m.w32(esp.wrapping_add(0x44u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esp.wrapping_add(0x48u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x28u32));
                ecx = m.r32(esp.wrapping_add(0x34u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(esp.wrapping_add(0x3cu32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                ebp = esi.wrapping_add(0x17cu32);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_add(0x14);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edi = eax;
                eax = m.r32(ebp);
                let (r, f) = lift::sar(edi, 0x1u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esi = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                ebx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sub(esi, ecx, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(esi, eax, 32); fl = f;
                esi = r;
                fl = lift::sub(ebx, esi, 32).1;
                if (fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_add(0x18);
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                fl = lift::sub(ebx, esi, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x24);
                pc = 5; continue;
            }
            5 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x20);
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

/// `0X00461EF0` scrollbar_set_visible
pub fn f_00461ef0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                fl = lift::sub((ebx & 0xff), u32::from(m.r8(esi.wrapping_add(0x48u32))), 8).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = esi.wrapping_add(0x94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x17cu32));
                ecx = esi.wrapping_add(0x17cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x264u32));
                ecx = esi.wrapping_add(0x264u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                m.w32(esp.wrapping_add(0x8u32), eax);
                m.w32(esp.wrapping_add(0xcu32), eax);
                ecx = esp.wrapping_add(0x8u32);
                eax = esi.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let t = m.r32(0x66a4ccu32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b21a0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
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

/// `0X004B9840` sgl_scrollbar_ctor
pub fn f_004b9840(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661cd8u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bbc00u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x8u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    m.w32(esi, 0x670cb0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b9930, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X004B98E0` sgl_scrollbar_dtor
pub fn f_004b98e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661cfcu32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    m.w32(esp.wrapping_add(0x4u32), esi);
    m.w32(esi, 0x670cb0u32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b9930, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x4b1640, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X004B9A90` sgl_scrollbar_on_mouse_event
pub fn f_004b9a90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x24);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1720, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(edi);
                ebp = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(edi.wrapping_add(0x4u32));
                ebx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esi);
                let (r, f) = lift::sub(ecx, ebp, 32); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x98u32));
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = esp.wrapping_add(0x1cu32);
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi.wrapping_add(0x4u32));
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(edi);
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xfcu32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(edx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                ebx = m.r32(esi.wrapping_add(0xe4u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                fl = lift::sub(ecx, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x100u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                ebx = m.r32(esi.wrapping_add(0xe8u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                fl = lift::sub(ecx, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 13; continue;
            }
            5 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x108u32));
                fl = lift::sub(edx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                ebx = m.r32(esi.wrapping_add(0xe4u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                fl = lift::sub(edx, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 11; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x10cu32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0xe8u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                if (fl.sf == fl.of) { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = 0x1u32;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 13; continue;
            }
            11 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 12; continue;
            }
            12 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 13; continue;
            }
            13 => {
                m.w32(esi.wrapping_add(0xf0u32), eax);
                eax = m.r32(esp.wrapping_add(0x3cu32));
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 22; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x2u32, 32).1;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x12du32)))) & 0xff);
                if !fl.zf { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = esi.wrapping_add(0x120u32);
                edx = m.r32(esi.wrapping_add(0x120u32));
                m.w32(esp.wrapping_add(0x1cu32), edx);
                edx = esi.wrapping_add(0x13cu32);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                ecx = m.r32(ecx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(edx, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                m.w32(edx.wrapping_add(0x4u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                m.w32(edx.wrapping_add(0x8u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x140u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x140u32), ecx);
                pc = 18; continue;
            }
            17 => {
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0xe8u32));
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, edx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x140u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w8(esi.wrapping_add(0x12du32), (0x1u32) as u8);
                let (r, f) = lift::imul(eax, m.r32(esi.wrapping_add(0x94u32))); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ebp = eax;
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0xf0u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 32; continue; }
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xf6u32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 32; continue; }
                pc = 21; continue;
            }
            21 => {
                m.w8(esi.wrapping_add(0xf6u32), (0x1u32) as u8);
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                pc = 31; continue;
            }
            22 => {
                fl = lift::sub(eax, 0xfffffffeu32, 32).1;
                if !fl.zf { pc = 32; continue; }
                pc = 23; continue;
            }
            23 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x12du32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 29; continue; }
                pc = 24; continue;
            }
            24 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xf6u32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 29; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf4u32)))) & 0xffff);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                m.w8(esi.wrapping_add(0x12du32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0xf0u32), ebx);
                m.w8(esi.wrapping_add(0xf6u32), ((ebx & 0xff)) as u8);
                m.w16(esi.wrapping_add(0xf8u32), ((eax & 0xffff)) as u16);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x3u32, 32).1;
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                let (r, f) = lift::sub(ebp, m.r32(esi.wrapping_add(0x9cu32)), 32); fl = f;
                ebp = r;
                pc = 30; continue;
            }
            27 => {
                fl = lift::sub(eax, 0x4u32, 32).1;
                if !fl.zf { pc = 30; continue; }
                pc = 28; continue;
            }
            28 => {
                let (r, f) = lift::add(ebp, m.r32(esi.wrapping_add(0x9cu32)), 32); fl = f;
                ebp = r;
                pc = 30; continue;
            }
            29 => {
                m.w32(esi.wrapping_add(0xf0u32), ebx);
                m.w8(esi.wrapping_add(0xf6u32), ((ebx & 0xff)) as u8);
                m.w16(esi.wrapping_add(0xf8u32), (0x64u32) as u16);
                m.w8(esi.wrapping_add(0x12du32), ((ebx & 0xff)) as u8);
                pc = 30; continue;
            }
            30 => {
                m.w32(esi.wrapping_add(0xf0u32), ebx);
                m.w8(esi.wrapping_add(0xf6u32), ((ebx & 0xff)) as u8);
                pc = 31; continue;
            }
            31 => {
                m.w16(esi.wrapping_add(0xf8u32), (0x64u32) as u16);
                pc = 32; continue;
            }
            32 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 39; continue; }
                pc = 33; continue;
            }
            33 => {
                ebp = eax;
                fl = lift::sub(ebp, ebx, 32).1;
                if (fl.sf != fl.of) { pc = 38; continue; }
                pc = 34; continue;
            }
            34 => {
                fl = lift::sub(ebp, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 38; continue; }
                pc = 35; continue;
            }
            35 => {
                edi = m.r32(esi.wrapping_add(0x98u32));
                ecx = esi;
                m.w32(esi.wrapping_add(0x98u32), ebp);
                { let a = []; eax = h.call_at(esp, m, 0x4ba810, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x148u32));
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 38; continue; }
                pc = 36; continue;
            }
            36 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0x98u32)), 32).1;
                if fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                eax = m.r32(ecx);
                { let t = m.r32(eax); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 38; continue;
            }
            38 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x24);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            39 => {
                fl = lift::sub(ebp, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 34; continue; }
                pc = 40; continue;
            }
            40 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 34; continue;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004BA050` sgl_scrollbar_draw
pub fn f_004ba050(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x50);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = esp.wrapping_add(0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x68u32));
                ecx = m.r32(esp.wrapping_add(0x48u32));
                ebp = m.r32(esp.wrapping_add(0x6cu32));
                eax = m.r32(esp.wrapping_add(0x4cu32));
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                ecx = m.r32(esp.wrapping_add(0x64u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x70u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xccu32));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 34; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xa8u32)), eax, 32).1;
                if fl.zf { pc = 34; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xacu32)), eax, 32).1;
                if fl.zf { pc = 34; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xb0u32)), eax, 32).1;
                if fl.zf { pc = 34; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x100u32));
                edx = m.r32(esi.wrapping_add(0xc0u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                edi = m.r32(esi.wrapping_add(0xb4u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esi.wrapping_add(0xfcu32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x68u32), edx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esp.wrapping_add(0x64u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x70u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                esp = esp.wrapping_add(0x8);
                fl = lift::sub(edi, m.r32(ecx.wrapping_add(0x14u32)), 32).1;
                if (fl.sf == fl.of) { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(ecx.wrapping_add(0xcu32));
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                ecx = edi;
                fl = lift::logic(eax & eax, 32);
                edi = m.r32(ecx);
                edx = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x3cu32), edx);
                edx = m.r32(ecx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x40u32), edx);
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x44u32), ecx);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x48u32));
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0x118u32));
                edi = m.r32(esi.wrapping_add(0x10cu32));
                edx = m.r32(esi.wrapping_add(0xe8u32));
                ecx = m.r32(esi.wrapping_add(0x100u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esi.wrapping_add(0x114u32));
                edx = m.r32(esi.wrapping_add(0xb8u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esi.wrapping_add(0xc4u32));
                let (r, f) = lift::sub(edi, ecx, 32); fl = f;
                edi = r;
                ecx = m.r32(esp.wrapping_add(0x64u32));
                m.w32(esp.wrapping_add(0x18u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(esp.wrapping_add(0x20u32), edx);
                m.w32(esp.wrapping_add(0x6cu32), eax);
                m.w32(esp.wrapping_add(0x3cu32), eax);
                m.w32(esp.wrapping_add(0x40u32), eax);
                m.w32(esp.wrapping_add(0x44u32), eax);
                m.w32(esp.wrapping_add(0x48u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_add(0x8);
                fl = lift::sub(eax, m.r32(ecx.wrapping_add(0x14u32)), 32).1;
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, m.r32(ecx.wrapping_add(0xcu32)), 32); fl = f;
                eax = r;
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                ecx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x40u32), ecx);
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x44u32), eax);
                pc = 14; continue;
            }
            13 => {
                edx = m.r32(esp.wrapping_add(0x68u32));
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                edi = m.r32(esp.wrapping_add(0x44u32));
                pc = 16; continue;
            }
            16 => {
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(edi, ecx, 32).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if !fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 20; continue;
            }
            19 => {
                eax = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24)), m.r32(esp.wrapping_add(28))]; eax = h.call_at(esp, m, 0x4bba80, ecx, &a); }
                esp = esp.wrapping_add(0x20);
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(esi.wrapping_add(0x10cu32));
                edx = m.r32(esi.wrapping_add(0xc8u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                edi = m.r32(esi.wrapping_add(0xbcu32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(esi.wrapping_add(0x108u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x68u32), edx);
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(esp.wrapping_add(0x64u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x70u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                esp = esp.wrapping_add(0x8);
                fl = lift::sub(edi, m.r32(ecx.wrapping_add(0x14u32)), 32).1;
                if (fl.sf == fl.of) { pc = 26; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 26; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(ecx.wrapping_add(0xcu32));
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                ecx = edi;
                fl = lift::logic(eax & eax, 32);
                edi = m.r32(ecx);
                edx = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x3cu32), edx);
                edx = m.r32(ecx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x40u32), edx);
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x44u32), ecx);
                if fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x48u32));
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 28; continue;
            }
            28 => {
                edi = m.r32(esi.wrapping_add(0x124u32));
                edx = m.r32(esp.wrapping_add(0x64u32));
                let (r, f) = lift::add(edi, ebp, 32); fl = f;
                edi = r;
                ebp = m.r32(esi.wrapping_add(0x120u32));
                let (r, f) = lift::add(ebp, ebx, 32); fl = f;
                ebp = r;
                ebx = m.r32(esi.wrapping_add(0xd0u32));
                esi = m.r32(esi.wrapping_add(0xd4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esp.wrapping_add(0x70u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                m.w32(esp.wrapping_add(0x6cu32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                esp = esp.wrapping_add(0x8);
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 32; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 32; continue; }
                pc = 30; continue;
            }
            30 => {
                edx = m.r32(esi.wrapping_add(0xcu32));
                let (r, f) = lift::shl(ebx, 0x4u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                ecx = ebx;
                fl = lift::logic(eax & eax, 32);
                esi = m.r32(ecx);
                ebx = m.r32(ecx.wrapping_add(0x4u32));
                edx = m.r32(ecx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x40u32), edx);
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x44u32), ecx);
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x48u32));
                eax = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 32; continue;
            }
            32 => {
                ecx = m.r32(esp.wrapping_add(0x64u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x6cu32));
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 44; continue; }
                pc = 33; continue;
            }
            33 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 44; continue;
            }
            34 => {
                edx = m.r32(esp.wrapping_add(0x54u32));
                eax = m.r32(esp.wrapping_add(0x58u32));
                ecx = m.r32(0x6dcd0cu32);
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x30u32), edx);
                edi = eax.wrapping_add(ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                edx = m.r32(0x6dcd0cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                ecx = m.r32(0x6dcd0cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x58u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                edx = m.r32(0x6dcd0cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x6cu32));
                esp = esp.wrapping_add(0x40);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xe4u32));
                esp = esp.wrapping_add(0x10);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 37; continue; }
                pc = 35; continue;
            }
            35 => {
                eax = m.r32(esi.wrapping_add(0xe8u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 37; continue; }
                pc = 36; continue;
            }
            36 => {
                ecx = m.r32(esp.wrapping_add(0x70u32));
                edx = m.r32(esp.wrapping_add(0x2cu32));
                eax = m.r32(esp.wrapping_add(0x34u32));
                m.w32(esp.wrapping_add(0x38u32), ebx);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                m.w32(esp.wrapping_add(0x38u32), edx);
                ecx = edx;
                edx = m.r32(0x6dcd0cu32);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x44u32), eax);
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xe8u32));
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x4cu32));
                ecx = ebx.wrapping_add(0x1u32);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0xe8u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x88u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                eax = m.r32(0x6dcd10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xe8u32));
                ecx = m.r32(esp.wrapping_add(0x8cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                ebp = m.r32(esp.wrapping_add(0x64u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                eax = m.r32(0x6dcd0cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xe8u32));
                eax = edi;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                ecx = m.r32(0x6dcd10u32);
                esp = esp.wrapping_add(0x4c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x68u32));
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esi.wrapping_add(0xe8u32));
                eax = edi;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x78u32));
                eax = m.r32(esi.wrapping_add(0xe8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x90u32));
                esp = esp.wrapping_add(0x24);
                pc = 37; continue;
            }
            37 => {
                eax = esi.wrapping_add(0x120u32);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                edx = eax;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                eax = m.r32(edx);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                ecx = esi;
                edx = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x70u32), edx);
                edx = esi.wrapping_add(0xd8u32);
                edi = m.r32(esi.wrapping_add(0xd8u32));
                m.w32(esp.wrapping_add(0x68u32), edi);
                edi = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x6cu32), edi);
                edx = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x70u32), edx);
                edx = m.r32(esp.wrapping_add(0x68u32));
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                m.w32(esp.wrapping_add(0x68u32), edx);
                { let t = m.r32(eax.wrapping_add(0xc4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                ecx = m.r32(esi.wrapping_add(0x130u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 40; continue;
            }
            39 => {
                edx = m.r32(esi.wrapping_add(0x134u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 40; continue;
            }
            40 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x70u32));
                edi = m.r32(esp.wrapping_add(0x6cu32));
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb430, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x80u32));
                let (r, f) = lift::add(edi, ebx, 32); fl = f;
                edi = r;
                edx = edi;
                m.w32(esp.wrapping_add(0x44u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x7cu32));
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                esi = ebp;
                let (r, f) = lift::sub(edx, 0x4u32, 32); fl = f;
                edx = r;
                esp = esp.wrapping_add(0x10);
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                fl = lift::logic(edx & edx, 32);
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                if (fl.zf || fl.sf != fl.of) { pc = 43; continue; }
                pc = 41; continue;
            }
            41 => {
                eax = esi;
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, 0x4u32, 32); fl = f;
                eax = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 43; continue; }
                pc = 42; continue;
            }
            42 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                pc = 43; continue;
            }
            43 => {
                ecx = m.r32(0x6dcd0cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                edx = m.r32(0x6dcd10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                eax = m.r32(0x6dcd10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                ecx = m.r32(0x6dcd0cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x40);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 44; continue;
            }
            44 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x50);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CB770` list_box_load_from_ini
pub fn f_004cb770(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6625aau32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x3c);
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
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                m.w32(esp.wrapping_add(0x54u32), ebp);
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w8(esp.wrapping_add(0x54u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4cb060, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                fl = lift::sub(m.r32(ecx.wrapping_add(0xfffffff8u32)), 0x5u32, 32).1;
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(0x6beee0u32);
                ecx = esp.wrapping_add(0x5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x60u32)), 0xffffffffu32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w32(esp.wrapping_add(0x60u32), ebp);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x60u32));
                ecx = m.r32(0x6bfae0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x68u32));
                ecx = m.r32(0x6bfaecu32);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x200u32), eax);
                m.w32(esi.wrapping_add(0x20cu32), eax);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfb04u32);
                ebx = 0x1u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x204u32), eax);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfaf8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x250u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2a20, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                edx = m.r32(0x6bfb14u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                m.w32(esi.wrapping_add(0x210u32), eax);
                edx = m.r32(0x6bfb28u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esi.wrapping_add(0xb0u32), eax);
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                edx = m.r32(esp.wrapping_add(0x5cu32));
                eax = m.r32(0x6dd5e4u32);
                ecx = m.r32(0x6bfb98u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w8(esp.wrapping_add(0x6cu32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e2f0, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e1e0, ecx, &a); }
                eax = m.r32(0x6bec38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63c47b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(0x6bec40u32);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63c47b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(esi.wrapping_add(0x228u32), (0x0u32) as u8);
                pc = 8; continue;
            }
            7 => {
                m.w8(esi.wrapping_add(0x228u32), (0x1u32) as u8);
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esp.wrapping_add(0x5cu32));
                eax = m.r32(0x6bfb7cu32);
                ecx = m.r32(0x6bfb74u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = esp.wrapping_add(0x2cu32);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bfc14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                m.w32(esp.wrapping_add(0x38u32), ebp);
                m.w32(esp.wrapping_add(0x3cu32), ebp);
                m.w32(esp.wrapping_add(0x40u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x6cu32));
                eax = m.r32(0x6bfb8cu32);
                ecx = m.r32(0x6bfb84u32);
                esp = esp.wrapping_add(0x10);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                m.w32(esp.wrapping_add(0x24u32), ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = esp.wrapping_add(0x24u32);
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bfc1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                m.w32(esp.wrapping_add(0x34u32), edx);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x40u32), ebp);
                { let t = m.r32(edx.wrapping_add(0x7cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                edx = m.r32(0x6bfba8u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esi.wrapping_add(0xb4u32));
                edi = esi.wrapping_add(0xb4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                m.w32(esi.wrapping_add(0x234u32), eax);
                { let t = m.r32(edx.wrapping_add(0x58u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfb38u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w8(esi.wrapping_add(0x220u32), ((eax & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfb68u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x224u32), eax);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfbe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ebx = eax;
                fl = lift::sub(ebx, 0x2u32, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ebx = 0x2u32;
                pc = 12; continue;
            }
            12 => {
                fl = lift::logic(ebx & ebx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = esi.wrapping_add(0x248u32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                edx = m.r32(0x6bfbecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x60u32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x68u32));
                edx = m.r32(0x6dd5e0u32);
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x54u32), (0x2u32) as u8);
                m.w32(edx, eax);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(ecx, 0x4u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebp, ebx, 32).1;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                if (fl.sf != fl.of) { pc = 14; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfbfcu32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0x218u32), eax);
                if (fl.sf == fl.of) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                m.w32(esi.wrapping_add(0x218u32), 0x0u32);
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfb58u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4ba6e0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfb48u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4ba710, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfbbcu32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = esp.wrapping_add(0x28u32);
                ebx = eax;
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esi.wrapping_add(0x238u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                edx = m.r32(0x6bfbd0u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esi);
                ecx = esi;
                ebx = eax;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(ebx, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                fl = lift::logic(ebx & ebx, 32);
                if (fl.sf == fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                ebp = eax;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                eax = esi.wrapping_add(0x23cu32);
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::logic(ebx & ebx, 32);
                m.w32(eax, ebp);
                if (fl.zf || fl.sf != fl.of) { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                let (r, f) = lift::add(ebx, 0xfffffffeu32, 32); fl = f;
                ebx = r;
                m.w32(esi.wrapping_add(0x240u32), ebx);
                pc = 23; continue;
            }
            22 => {
                m.w32(esi.wrapping_add(0x240u32), 0x0u32);
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esp.wrapping_add(0x10u32);
                ecx = esp.wrapping_add(0x5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1ec0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = esp.wrapping_add(0x10u32);
                eax = esp.wrapping_add(0x5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2080, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x60u32));
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x54u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x54u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x5cu32);
                m.w32(esp.wrapping_add(0x54u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x48);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CCAA0` list_box_add_column
pub fn f_004ccaa0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6625d4u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
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
                ebp = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                m.w32(esp.wrapping_add(0x18u32), ebp);
                ebx = ebp.wrapping_mul(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                edi = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x100u32);
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x28u32), 0x0u32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4d1fe0, ecx, &a); }
                pc = 3; continue;
            }
            2 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                m.w32(esp.wrapping_add(0x28u32), 0xffffffffu32);
                m.w32(edi.wrapping_add(ecx.wrapping_mul(4)), eax);
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                m.w32(eax.wrapping_add(edx.wrapping_mul(4)), 0x0u32);
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xa4u32));
                ecx = m.r32(esi.wrapping_add(0xacu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xa4u32));
                ecx = m.r32(esi.wrapping_add(0xa8u32));
                edx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xacu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xa8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esi.wrapping_add(0x204u32));
                m.w32(esi.wrapping_add(0xacu32), eax);
                eax = m.r32(esi.wrapping_add(0xa4u32));
                esp = esp.wrapping_add(0x30);
                m.w32(esi.wrapping_add(0xa0u32), edi);
                m.w32(esi.wrapping_add(0xa8u32), ecx);
                ecx = m.r32(edi.wrapping_add(eax.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2d70, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x200u32));
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2d80, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x204u32));
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x210u32));
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2e20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x9cu32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(esi.wrapping_add(0x98u32));
                ebp = eax;
                eax = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x98u32));
                esp = esp.wrapping_add(0x14);
                m.w32(eax.wrapping_add(edi.wrapping_mul(4)), ebp);
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(eax.wrapping_add(edx.wrapping_mul(4)), 0x0u32);
                eax = m.r32(esi.wrapping_add(0x9cu32));
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 5; continue;
            }
            5 => {
                ebp = m.r32(esp.wrapping_add(0x18u32));
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                edx = m.r32(esp.wrapping_add(0x38u32));
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                m.w32(esi.wrapping_add(0xa4u32), ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4ccff0, ecx, &a); }
                esp = esp.wrapping_add(20);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x430fe0 => f_00430fe0(m, h, ecx, esp),
        0x460870 => f_00460870(m, h, ecx, esp),
        0x460a70 => f_00460a70(m, h, ecx, esp),
        0x460b70 => f_00460b70(m, h, ecx, esp),
        0x460da0 => f_00460da0(m, h, ecx, esp),
        0x460ef0 => f_00460ef0(m, h, ecx, esp),
        0x461590 => f_00461590(m, h, ecx, esp),
        0x461ef0 => f_00461ef0(m, h, ecx, esp),
        0x4b9840 => f_004b9840(m, h, ecx, esp),
        0x4b98e0 => f_004b98e0(m, h, ecx, esp),
        0x4b9a90 => f_004b9a90(m, h, ecx, esp),
        0x4ba050 => f_004ba050(m, h, ecx, esp),
        0x4cb770 => f_004cb770(m, h, ecx, esp),
        0x4ccaa0 => f_004ccaa0(m, h, ecx, esp),
        _ => return None,
    }))
}
