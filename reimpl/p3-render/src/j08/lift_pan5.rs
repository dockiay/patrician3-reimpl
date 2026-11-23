//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X004786F0` build_side_menu_ctor
pub fn f_004786f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65f7a8u32);
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
    { let a = []; eax = h.call_at(esp, m, 0x440900, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x5u32);
    eax = esi.wrapping_add(0xacu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x24u32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x8u32);
    ecx = esi.wrapping_add(0x534u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x24u32), (0x1u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0xc94u32);
    m.w8(esp.wrapping_add(0x10u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0xd7cu32);
    m.w8(esp.wrapping_add(0x10u32), (0x3u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0xe64u32);
    m.w8(esp.wrapping_add(0x10u32), (0x4u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0xf4cu32);
    m.w8(esp.wrapping_add(0x10u32), (0x5u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    edx = m.r32(0x6c7cccu32);
    ecx = m.r32(esp.wrapping_add(0x8u32));
    m.w32(esi.wrapping_add(0x1078u32), edx);
    m.w32(esi, 0x66ef70u32);
    m.w32(esi.wrapping_add(0x1050u32), 0x0u32);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X004787D0` build_side_menu_dtor
pub fn f_004787d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65f828u32);
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
    m.w32(esi, 0x66ef70u32);
    ecx = esi.wrapping_add(0x1078u32);
    m.w32(esp.wrapping_add(0x10u32), 0x6u32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esi.wrapping_add(0xf4cu32);
    m.w8(esp.wrapping_add(0x10u32), (0x5u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0xe64u32);
    m.w8(esp.wrapping_add(0x10u32), (0x4u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0xd7cu32);
    m.w8(esp.wrapping_add(0x10u32), (0x3u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0xc94u32);
    m.w8(esp.wrapping_add(0x10u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x8u32);
    eax = esi.wrapping_add(0x534u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x20u32), (0x1u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x5u32);
    ecx = esi.wrapping_add(0xacu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x20u32), (0x0u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x440a20, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X004788B0` build_side_menu_release_buttons
pub fn f_004788b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0xf4cu32));
                ecx = esi.wrapping_add(0xf4cu32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0xe64u32));
                ecx = esi.wrapping_add(0xe64u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0xc94u32));
                ecx = esi.wrapping_add(0xc94u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0xd7cu32));
                ecx = esi.wrapping_add(0xd7cu32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(esi, 0x534u32, 32); fl = f;
                esi = r;
                edi = 0x8u32;
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
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

/// `0X00478920` build_side_menu_load_layout
pub fn f_00478920(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65f84au32);
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
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x3cu32), 0x0u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x18u32), eax);
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
                m.w32(esp, 0x6b9a9cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a8cu32);
                ecx = 0x6dd524u32;
                m.w8(esp.wrapping_add(0x54u32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
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
                m.w32(esp, 0x6b9aa4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a8cu32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x1cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x66de5cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xf4cu32));
                edi = esi.wrapping_add(0xf4cu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = esi.wrapping_add(0xe64u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esi.wrapping_add(0x100cu32), 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xc94u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xd7cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0x5f4u32);
                m.w32(esp.wrapping_add(0x14u32), 0x8u32);
                pc = 1; continue;
            }
            1 => {
                eax = ebp.wrapping_add(0xffffff40u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x14u32));
                m.w32(ebp, 0xff000000u32);
                ecx = m.r32(0x66e104u32);
                m.w32(ebp.wrapping_add(0x4u32), ecx);
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(0x6c2280u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6c2298u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6c22acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0xc94u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6c22bcu32);
                ecx = esi.wrapping_add(0xd7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9aacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a8cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                m.w32(esi.wrapping_add(0x1034u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9ab8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a8cu32);
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
                m.w32(esp, 0x6b9ac4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a8cu32);
                m.w32(esi.wrapping_add(0x1044u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esi.wrapping_add(0x1048u32), eax);
                eax = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9ad4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a8cu32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                m.w32(esi.wrapping_add(0x1064u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9ae0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a8cu32);
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
                m.w32(esp, 0x6b9aecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a8cu32);
                m.w32(esi.wrapping_add(0x1068u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esi.wrapping_add(0x106cu32), eax);
                eax = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9af8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a8cu32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = esp.wrapping_add(0x1cu32);
                ebx = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4417d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                edi = esi.wrapping_add(0xacu32);
                pc = 3; continue;
            }
            3 => {
                edx = ebx.wrapping_add(ebp);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(edi);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebp, 0x5u32, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6dcf20u32);
                edx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dcf20u32;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x1034u32));
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x28u32);
                m.w32(esi.wrapping_add(0x1038u32), ecx);
                eax = m.r32(0x6dcd28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dcd28u32;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ebx = m.r32(esi.wrapping_add(0x1038u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x9cu32);
                edx = esi.wrapping_add(0x107cu32);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let (r, f) = lift::sar(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x1040u32), eax);
                ecx = eax.wrapping_add(eax.wrapping_mul(8));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x103cu32), ecx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi);
                esp = esp.wrapping_add(0xc);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x130u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esp.wrapping_add(0x18u32);
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

/// `0X00478C90` build_side_menu_layout_buttons
pub fn f_00478c90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4417d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(0x6cbd9cu32);
                ebx = esi.wrapping_add(0xacu32);
                ebp = ebx;
                m.w32(esp.wrapping_add(0x10u32), 0x5u32);
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = 0x2aaaaaabu32;
                ecx = ebx;
                { let r = i64::from(eax as i32) * i64::from(edi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x14u32);
                edi = edx;
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x18u32), edx);
                edx = m.r32(ebx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0x194u32));
                m.w32(esp.wrapping_add(0x14u32), ebx);
                ebx = esi.wrapping_add(0x194u32);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0x364u32));
                m.w32(esp.wrapping_add(0x14u32), ebx);
                ebx = esi.wrapping_add(0x364u32);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0x44cu32));
                m.w32(esp.wrapping_add(0x14u32), ebx);
                ebx = esi.wrapping_add(0x44cu32);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(0x27cu32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                edx = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0x14u32), ebx);
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x534u32));
                edi = esi.wrapping_add(0x534u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edi);
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x1038u32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x1cu32), 0x64u32);
                m.w32(esp.wrapping_add(0x18u32), edx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esi = eax.wrapping_add(eax.wrapping_mul(4));
                ebx = 0x8u32;
                let (r, f) = lift::sar(esi, 0x2u32, 32, fl); fl = f;
                esi = r;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(edi);
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
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
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00478E10` build_side_menu_show
pub fn f_00478e10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                ebx = eax;
                eax = m.r32(0x6e51acu32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                edi = m.r32(eax.wrapping_add(0xc324u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = edi.wrapping_add(edi.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(0x6de508u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, edi, 32); fl = f;
                edx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x6f1u32)))) & 0xff);
                eax = eax.wrapping_add(edx.wrapping_mul(8));
                edx = m.r32(0x6dfc14u32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esi.wrapping_add(0x1050u32));
                fl = lift::sub(ecx, edx, 32).1;
                m.w8(esp.wrapping_add(0x13u32), (u32::from(fl.zf)) as u8);
                fl = lift::sub(eax, ebp, 32).1;
                m.w32(esi.wrapping_add(0x1054u32), ebp);
                if !fl.zf { pc = 19; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = esi.wrapping_add(0xacu32);
                ebp = 0x5u32;
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                edi = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(edi.wrapping_add(0x76cu32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::logic(u32::from(m.r8(edi.wrapping_add(0x2c8u32))) & 0xe0u32, 8);
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 8; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esi.wrapping_add(0xacu32));
                ecx = esi.wrapping_add(0xacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 12; continue;
            }
            11 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esi.wrapping_add(0x194u32));
                ecx = esi.wrapping_add(0x194u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub(m.r32(edi.wrapping_add(0x76cu32)), 0x8u32, 32).1;
                if fl.cf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 16; continue;
            }
            15 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(esi.wrapping_add(0x27cu32));
                ecx = esi.wrapping_add(0x27cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x364u32));
                ecx = esi.wrapping_add(0x364u32);
                fl = lift::logic(eax & eax, 32);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x1cu32), ((eax & 0xff)) as u8);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esi.wrapping_add(0x44cu32));
                ecx = esi.wrapping_add(0x44cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x534u32));
                edi = esi.wrapping_add(0x534u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edi);
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x1038u32));
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x20u32), edx);
                m.w32(esp.wrapping_add(0x24u32), 0x64u32);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax.wrapping_add(eax.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x18u32), 0x8u32);
                let (r, f) = lift::sar(ebp, 0x2u32, 32, fl); fl = f;
                ebp = r;
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x20u32));
                eax = esp.wrapping_add(0x1cu32);
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x24u32), edx);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if !fl.zf { pc = 17; continue; }
                pc = 18; continue;
            }
            18 => {
                edi = esi.wrapping_add(0xf4cu32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, eax, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x1cu32);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(edi);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0xe64u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(ebp);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let (r, f) = lift::sar(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = esi.wrapping_add(0xc94u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = esi.wrapping_add(0xd7cu32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x1cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x1050u32), 0x1u32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 19; continue;
            }
            19 => {
                ecx = esi;
                m.w32(esi.wrapping_add(0x104cu32), ebp);
                { let a = []; eax = h.call_at(esp, m, 0x47ab20, ecx, &a); }
                fl = lift::sub(m.r32(esi.wrapping_add(0x104cu32)), 0x1u32, 32).1;
                if !fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x47ab20, ecx, &a); }
                pc = 21; continue;
            }
            21 => {
                m.w32(esi.wrapping_add(0x1058u32), ebp);
                m.w32(esi.wrapping_add(0x105cu32), ebp);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x58cc90, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x593800, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                esp = esp.wrapping_add(0x4);
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

/// `0X00479150` build_side_menu_show_buttons
pub fn f_00479150(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx.wrapping_add(0x534u32);
                edi = ecx.wrapping_add(0xc74u32);
                ebx = 0x8u32;
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(edi);
                fl = lift::logic(eax & eax, 32);
                if (fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 8; continue;
            }
            3 => {
                fl = lift::sub(eax, 0x32u32, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(eax, 0x33u32, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(eax, 0x36u32, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 8; continue;
            }
            7 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6e51acu32);
                eax = m.r32(eax.wrapping_add(0xc324u32));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(0x6de508u32);
                ecx = m.r32(eax.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2c8u32));
                let (r, f) = lift::shr(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                ecx = (ecx & !0xff) | ((!(ecx & 0xff)) & 0xff);
                let r = (ecx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 9; continue;
            }
            9 => {
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

/// `0X004791F0` build_side_menu_hide
pub fn f_004791f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = 0x8u32;
                edi = esi.wrapping_add(0x534u32);
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0xf4cu32));
                edi = esi.wrapping_add(0xf4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xe64u32));
                edi = esi.wrapping_add(0xe64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc94u32));
                edi = esi.wrapping_add(0xc94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xd7cu32));
                edi = esi.wrapping_add(0xd7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x58cc90, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x1058u32), 0x0u32);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x593800, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w32(esi.wrapping_add(0x105cu32), 0x0u32);
                m.w32(esi.wrapping_add(0x1054u32), 0x0u32);
                ecx = esi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x440ed0, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004792E0` build_side_menu_close
pub fn f_004792e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = ecx;
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                esi = eax;
                eax = m.r32(edi.wrapping_add(0x1050u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(0x6cbd8cu32), edi, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebx = edi.wrapping_add(0xacu32);
                ebp = 0x5u32;
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                ebx = edi.wrapping_add(0x534u32);
                ebp = 0x8u32;
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 5; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = edi.wrapping_add(0xf4cu32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi.wrapping_add(0xe64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = edi.wrapping_add(0xc94u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = edi.wrapping_add(0xd7cu32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(edi.wrapping_add(0x1050u32), 0x0u32);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                ecx = edi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x4410f0, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00479390` build_side_menu_prerender
pub fn f_00479390(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                ecx = m.r32(0x66d504u32);
                ebp = m.r32(esi.wrapping_add(0x14u32));
                ebx = m.r32(esi.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(0x8);
                m.w32(esi.wrapping_add(0x94u32), ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                edx = m.r32(esi);
                m.w32(ecx, ebp);
                m.w8(esp.wrapping_add(0x1fu32), ((eax & 0xff)) as u8);
                eax = m.r32(esi.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w8(esi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(ecx.wrapping_add(0x8u32), eax);
                eax = m.r32(0x6cbd94u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1054u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 9; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi.wrapping_add(0xf4cu32));
                edi = esi.wrapping_add(0xf4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = esp;
                eax = m.r32(edi);
                m.w32(ecx, ebp);
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w32(ecx.wrapping_add(0x8u32), edx);
                ecx = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1070u32));
                fl = lift::sub(eax, 0x36u32, 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, 0x3au32, 32).1;
                if (fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edi = esi.wrapping_add(0xe64u32);
                pc = 6; continue;
            }
            4 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                edi = esi.wrapping_add(0xc94u32);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                eax = esp;
                edx = m.r32(edi);
                m.w32(eax, ebp);
                m.w32(eax.wrapping_add(0x4u32), ebx);
                m.w32(eax.wrapping_add(0x8u32), ecx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 13; continue;
            }
            7 => {
                fl = lift::sub(eax, 0x28u32, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 8; continue;
            }
            8 => {
                edi = esi.wrapping_add(0xd7cu32);
                pc = 6; continue;
            }
            9 => {
                eax = esi.wrapping_add(0xc74u32);
                edi = esi.wrapping_add(0x534u32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x18u32), 0x8u32);
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::sub(m.r32(ecx), 0x0u32, 32).1;
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = esp;
                eax = m.r32(edi);
                m.w32(ecx, ebp);
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w32(ecx.wrapping_add(0x8u32), edx);
                ecx = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edx, 0x4u32, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                if !fl.zf { pc = 10; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w8(esi.wrapping_add(0xa4u32), ((eax & 0xff)) as u8);
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

/// `0X00479550` build_side_menu_draw
pub fn f_00479550(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65f85cu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x2c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x50u32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ebp = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 12; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 12; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x64u32));
                edx = esp;
                ecx = esi;
                m.w32(edx, edi);
                edi = m.r32(esp.wrapping_add(0x58u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(edx.wrapping_add(0x4u32), ebx);
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x440b20, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                edi = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ebx = eax;
                eax = m.r32(esi.wrapping_add(0x1054u32));
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = esp.wrapping_add(0x10u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4797c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            5 => {
                ecx = m.r32(esi.wrapping_add(0x104cu32));
                eax = 0x4u32;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w32(esi.wrapping_add(0x104cu32), eax);
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = 0x6dcf20u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                eax = m.r32(0x66e0c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x104cu32));
                eax = m.r32(esi.wrapping_add(0x1034u32));
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c2380u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = eax.wrapping_add(ecx).wrapping_add(0x1u32);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                edx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x104cu32));
                edx = m.r32(esi.wrapping_add(0x1034u32));
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6c2380u32));
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x1060u32));
                esp = esp.wrapping_add(0x2c);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = esp.wrapping_add(0x20u32);
                { let a = []; eax = h.call_at(esp, m, 0x4209e0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x1040u32));
                ecx = m.r32(esi.wrapping_add(0x1044u32));
                ebp = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                m.w32(esp.wrapping_add(0x48u32), 0x0u32);
                let (r, f) = lift::add(edx, edx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ecx.wrapping_mul(4);
                let (r, f) = lift::sub(ebp, edx, 32); fl = f;
                ebp = r;
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sub(ebp, edx, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esi.wrapping_add(0x1038u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = edx.wrapping_add(ecx.wrapping_mul(2));
                edx = m.r32(0x6b20a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x64u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x38u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x420a10, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = esp.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x44u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x420a00, ecx, &a); }
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(0x6cbd64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(0x6cbd5cu32);
                edx = m.r32(0x6cbd58u32);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 11; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x38);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00479F50` build_side_menu_update
pub fn f_00479f50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esp.wrapping_add(0x34u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x20u32), edi);
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                eax = m.r32(0x6e51acu32);
                ebx = m.r32(eax.wrapping_add(0xc324u32));
                eax = m.r32(0x6de4b0u32);
                fl = lift::sub(ebx, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w32(esp.wrapping_add(0x14u32), edi);
                ebx = edi;
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(0x6de508u32);
                ecx = ebx.wrapping_add(ebx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
                ecx = r;
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2c8u32));
                ebp = edx.wrapping_add(ecx.wrapping_mul(8));
                let r = eax & 0xe0u32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), ebp);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi.wrapping_add(0xacu32));
                edi = esi.wrapping_add(0xacu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0x364u32));
                edi = esi.wrapping_add(0x364u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0x194u32));
                ecx = esi.wrapping_add(0x194u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x44cu32));
                ecx = esi.wrapping_add(0x44cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(edi);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x6f1u32)))) & 0xff);
                ecx = edi;
                ebp = eax;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                eax = m.r32(0x6dfc14u32);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ebp, eax, 32).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::sub(edx, ecx, 32).1;
                if !fl.zf { pc = 24; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0x194u32));
                ecx = esi.wrapping_add(0x194u32);
                fl = lift::sub(ebp, eax, 32).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 24; continue;
            }
            10 => {
                eax = m.r32(0x6dfc14u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x6f1u32)))) & 0xff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 36; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(esi.wrapping_add(0x194u32));
                edi = esi.wrapping_add(0x194u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esi.wrapping_add(0x27cu32));
                edi = esi.wrapping_add(0x27cu32);
                ebx = m.r32(ecx.wrapping_add(0x76cu32));
                ecx = edi;
                let (r, f) = lift::shr(ebx, 0x3u32, 32, fl); fl = f;
                ebx = r;
                let r = (ebx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x1cu32), ((ebx & 0xff)) as u8);
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esi.wrapping_add(0xacu32));
                edi = esi.wrapping_add(0xacu32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 30; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                ebp = m.r32(ecx.wrapping_add(0x76cu32));
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(ebp & ebp, 32);
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(edi);
                ecx = m.r32(eax.wrapping_add(0x76cu32));
                fl = lift::logic(ecx & ecx, 32);
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(esi.wrapping_add(0x364u32));
                edi = esi.wrapping_add(0x364u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 20; continue;
            }
            20 => {
                edx = m.r32(esi.wrapping_add(0x44cu32));
                edi = esi.wrapping_add(0x44cu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 23; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 22; continue;
            }
            22 => {
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 23; continue;
            }
            23 => {
                ebx = m.r32(esp.wrapping_add(0x14u32));
                pc = 24; continue;
            }
            24 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x18u32), edi);
                pc = 25; continue;
            }
            25 => {
                ecx = edi.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, edi, 32); fl = f;
                ecx = r;
                edx = edi.wrapping_add(ecx.wrapping_mul(4));
                ecx = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 70; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = esi;
                m.w32(esi.wrapping_add(0x104cu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x47ab20, ecx, &a); }
                ebx = 0x8u32;
                m.w32(esp.wrapping_add(0x20u32), 0x1u32);
                edi = esi.wrapping_add(0x534u32);
                ebp = esi.wrapping_add(0xc74u32);
                m.w32(esp.wrapping_add(0x18u32), ebx);
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(ebp);
                fl = lift::sub(eax, 0x32u32, 32).1;
                if (fl.sf != fl.of) { pc = 59; continue; }
                pc = 28; continue;
            }
            28 => {
                fl = lift::sub(eax, 0x36u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 59; continue; }
                pc = 29; continue;
            }
            29 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(edi);
                edx = m.r32(ecx.wrapping_add(0x2c8u32));
                ecx = edi;
                let (r, f) = lift::shr(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                edx = (edx & !0xff) | ((!(edx & 0xff)) & 0xff);
                let r = (edx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 63; continue;
            }
            30 => {
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            32 => {
                edx = m.r32(esi.wrapping_add(0x364u32));
                edi = esi.wrapping_add(0x364u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 34; continue;
            }
            34 => {
                edx = m.r32(esi.wrapping_add(0x44cu32));
                edi = esi.wrapping_add(0x44cu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 23; continue; }
                pc = 35; continue;
            }
            35 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 22; continue;
            }
            36 => {
                eax = m.r32(ebp.wrapping_add(0x76cu32));
                edx = m.r32(esi.wrapping_add(0xacu32));
                edi = esi.wrapping_add(0xacu32);
                fl = lift::logic(((eax >> 8) & 0xff) & 0x1u32, 8);
                ecx = edi;
                if !fl.zf { pc = 48; continue; }
                pc = 37; continue;
            }
            37 => {
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 39; continue;
            }
            39 => {
                edx = m.r32(esi.wrapping_add(0x194u32));
                edi = esi.wrapping_add(0x194u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 41; continue; }
                pc = 40; continue;
            }
            40 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 41; continue;
            }
            41 => {
                edx = m.r32(esi.wrapping_add(0x27cu32));
                edi = esi.wrapping_add(0x27cu32);
                ecx = edi;
                pc = 42; continue;
            }
            42 => {
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 44; continue; }
                pc = 43; continue;
            }
            43 => {
                eax = m.r32(edi);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 44; continue;
            }
            44 => {
                edx = m.r32(esi.wrapping_add(0x364u32));
                edi = esi.wrapping_add(0x364u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 46; continue; }
                pc = 45; continue;
            }
            45 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 46; continue;
            }
            46 => {
                edx = m.r32(esi.wrapping_add(0x44cu32));
                edi = esi.wrapping_add(0x44cu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 24; continue; }
                pc = 47; continue;
            }
            47 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 24; continue;
            }
            48 => {
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 50; continue; }
                pc = 49; continue;
            }
            49 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 50; continue;
            }
            50 => {
                edx = m.r32(esi.wrapping_add(0x27cu32));
                edi = esi.wrapping_add(0x27cu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 52; continue; }
                pc = 51; continue;
            }
            51 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 52; continue;
            }
            52 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esi.wrapping_add(0x194u32));
                edi = esi.wrapping_add(0x194u32);
                fl = lift::logic(eax & eax, 32);
                ecx = edi;
                if fl.zf { pc = 42; continue; }
                pc = 53; continue;
            }
            53 => {
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 55; continue; }
                pc = 54; continue;
            }
            54 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 55; continue;
            }
            55 => {
                edx = m.r32(esi.wrapping_add(0x364u32));
                edi = esi.wrapping_add(0x364u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 57; continue; }
                pc = 56; continue;
            }
            56 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 57; continue;
            }
            57 => {
                edx = m.r32(esi.wrapping_add(0x44cu32));
                edi = esi.wrapping_add(0x44cu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 24; continue; }
                pc = 58; continue;
            }
            58 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 24; continue;
            }
            59 => {
                fl = lift::logic(eax & eax, 32);
                eax = m.r32(edi);
                ecx = edi;
                if (fl.sf != fl.of) { pc = 61; continue; }
                pc = 60; continue;
            }
            60 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 62; continue;
            }
            61 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 62; continue;
            }
            62 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 63; continue;
            }
            63 => {
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 27; continue; }
                pc = 64; continue;
            }
            64 => {
                eax = m.r32(esi.wrapping_add(0x1054u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 69; continue; }
                pc = 65; continue;
            }
            65 => {
                eax = m.r32(esi.wrapping_add(0x1058u32));
                m.w32(esi.wrapping_add(0x1054u32), 0x0u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 67; continue; }
                pc = 66; continue;
            }
            66 => {
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x58cc90, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x1058u32), 0x0u32);
                pc = 67; continue;
            }
            67 => {
                eax = m.r32(esi.wrapping_add(0x105cu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 69; continue; }
                pc = 68; continue;
            }
            68 => {
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x593800, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w32(esi.wrapping_add(0x105cu32), 0x0u32);
                pc = 69; continue;
            }
            69 => {
                eax = m.r32(esi.wrapping_add(0xe64u32));
                edi = esi.wrapping_add(0xe64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xf4cu32));
                edi = esi.wrapping_add(0xf4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc94u32));
                edi = esi.wrapping_add(0xc94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xd7cu32));
                edi = esi.wrapping_add(0xd7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x18u32));
                ebx = m.r32(esp.wrapping_add(0x14u32));
                pc = 70; continue;
            }
            70 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, 0x5u32, 32).1;
                m.w32(esp.wrapping_add(0x18u32), edi);
                if (fl.sf != fl.of) { pc = 25; continue; }
                pc = 71; continue;
            }
            71 => {
                eax = m.r32(esi.wrapping_add(0x1054u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 100; continue; }
                pc = 72; continue;
            }
            72 => {
                edi = esi.wrapping_add(0xf4cu32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 86; continue; }
                pc = 73; continue;
            }
            73 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                m.w32(esi.wrapping_add(0x1054u32), 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xe64u32));
                edi = esi.wrapping_add(0xe64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc94u32));
                edi = esi.wrapping_add(0xc94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xd7cu32));
                edi = esi.wrapping_add(0xd7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = esi.wrapping_add(0x534u32);
                ebp = esi.wrapping_add(0xc74u32);
                ebx = 0x8u32;
                pc = 74; continue;
            }
            74 => {
                eax = m.r32(ebp);
                fl = lift::sub(eax, 0x32u32, 32).1;
                if fl.zf { pc = 80; continue; }
                pc = 75; continue;
            }
            75 => {
                fl = lift::sub(eax, 0x33u32, 32).1;
                if fl.zf { pc = 80; continue; }
                pc = 76; continue;
            }
            76 => {
                fl = lift::sub(eax, 0x36u32, 32).1;
                if fl.zf { pc = 80; continue; }
                pc = 77; continue;
            }
            77 => {
                fl = lift::logic(eax & eax, 32);
                eax = m.r32(edi);
                ecx = edi;
                if (fl.sf != fl.of) { pc = 79; continue; }
                pc = 78; continue;
            }
            78 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 81; continue;
            }
            79 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 81; continue;
            }
            80 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(eax.wrapping_add(0x2c8u32));
                let (r, f) = lift::shr(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                ecx = (ecx & !0xff) | ((!(ecx & 0xff)) & 0xff);
                let r = (ecx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 81; continue;
            }
            81 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 74; continue; }
                pc = 82; continue;
            }
            82 => {
                eax = m.r32(esi.wrapping_add(0x1058u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 84; continue; }
                pc = 83; continue;
            }
            83 => {
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x58cc90, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x1058u32), 0x0u32);
                pc = 84; continue;
            }
            84 => {
                eax = m.r32(esi.wrapping_add(0x105cu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 98; continue; }
                pc = 85; continue;
            }
            85 => {
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x593800, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w32(esi.wrapping_add(0x105cu32), 0x0u32);
                pc = 98; continue;
            }
            86 => {
                ecx = esi.wrapping_add(0xe64u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 89; continue; }
                pc = 87; continue;
            }
            87 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x766u32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(eax.wrapping_add(0x776u32))), 16).1;
                if !fl.cf { pc = 97; continue; }
                pc = 88; continue;
            }
            88 => {
                edx = m.r32(esi.wrapping_add(0x1070u32));
                eax = m.r32(0x6dfc14u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(edx, 0x36u32, 32).1;
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                edx = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x38u32), 0x24u32);
                m.w32(esp.wrapping_add(0x3cu32), eax);
                m.w32(esp.wrapping_add(0x40u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 98; continue;
            }
            89 => {
                ecx = esi.wrapping_add(0xc94u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 91; continue; }
                pc = 90; continue;
            }
            90 => {
                eax = m.r32(0x6dfc14u32);
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x38u32), 0x25u32);
                m.w32(esp.wrapping_add(0x3cu32), ebx);
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 98; continue;
            }
            91 => {
                ecx = esi.wrapping_add(0xd7cu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 93; continue; }
                pc = 92; continue;
            }
            92 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(0x6dfc14u32))) & 0xff);
                eax = esp.wrapping_add(0x3bu32);
                ecx = esp.wrapping_add(0x3au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x3cu32), 0x22u32);
                m.w8(esp.wrapping_add(0x40u32), ((edx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x41u32), ((ebx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x44u32), (0x28u32) as u8);
                m.w8(esp.wrapping_add(0x45u32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x5250d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = esp.wrapping_add(0x34u32);
                ecx = 0x6df2f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 98; continue;
            }
            93 => {
                eax = m.r32(0x6dfc18u32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 96; continue; }
                pc = 94; continue;
            }
            94 => {
                eax = m.r32(esi.wrapping_add(0x107cu32));
                ecx = m.r32(0x6de4b4u32);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 96; continue; }
                pc = 95; continue;
            }
            95 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(ecx.wrapping_add(0x7a4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 97; continue; }
                pc = 96; continue;
            }
            96 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = esi;
                m.w32(edx.wrapping_add(0x7a4u32), 0x0u32);
                { let a = []; eax = h.call_at(esp, m, 0x47b3c0, ecx, &a); }
                let r = m.r32(esp.wrapping_add(0x20u32)) | eax; fl = lift::logic(r, 32);
                m.w32(esp.wrapping_add(0x20u32), r);
                pc = 97; continue;
            }
            97 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 99; continue; }
                pc = 98; continue;
            }
            98 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x24u32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x34u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 99; continue;
            }
            99 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x38);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            100 => {
                ecx = esi.wrapping_add(0x534u32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                edi = esi.wrapping_add(0xc74u32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                pc = 101; continue;
            }
            101 => {
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 103; continue; }
                pc = 102; continue;
            }
            102 => {
                fl = lift::sub(m.r32(edi), 0x0u32, 32).1;
                if (fl.sf == fl.of) { pc = 105; continue; }
                pc = 103; continue;
            }
            103 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ecx, 0xe8u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebp, 0x8u32, 32).1;
                m.w32(esp.wrapping_add(0x18u32), ecx);
                if (fl.sf != fl.of) { pc = 101; continue; }
                pc = 104; continue;
            }
            104 => {
                pc = 97; continue;
            }
            105 => {
                edi = esi.wrapping_add(0x534u32);
                ebx = 0x8u32;
                pc = 106; continue;
            }
            106 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 106; continue; }
                pc = 107; continue;
            }
            107 => {
                eax = m.r32(esi.wrapping_add(0xf4cu32));
                edi = esi.wrapping_add(0xf4cu32);
                ebx = 0x1u32;
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(0xc74u32));
                ecx = (ecx & !0xffff) | (((eax & 0xffff)) & 0xffff);
                m.w32(esi.wrapping_add(0x1070u32), eax);
                m.w16(esi.wrapping_add(0x1154u32), ((ecx & 0xffff)) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                ecx = eax;
                m.w16(esi.wrapping_add(0x1156u32), ((edx & 0xffff)) as u16);
                fl = lift::sub(ecx, 0x36u32, 32).1;
                if (fl.sf == fl.of) { pc = 141; continue; }
                pc = 108; continue;
            }
            108 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(ecx, ebp, 32).1;
                if !fl.zf { pc = 110; continue; }
                pc = 109; continue;
            }
            109 => {
                eax = m.r32(0x6c22e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 114; continue;
            }
            110 => {
                fl = lift::sub(ecx, 0x34u32, 32).1;
                if fl.zf { pc = 113; continue; }
                pc = 111; continue;
            }
            111 => {
                fl = lift::sub(ecx, 0x35u32, 32).1;
                if fl.zf { pc = 113; continue; }
                pc = 112; continue;
            }
            112 => {
                ecx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6a57c8u32));
                edx = m.r32(0x6c22ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esi.wrapping_add(0x1078u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 115; continue;
            }
            113 => {
                ecx = m.r32(0x6b23bcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 114; continue;
            }
            114 => {
                ecx = esi.wrapping_add(0x1078u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 115; continue;
            }
            115 => {
                edi = m.r32(esp.wrapping_add(0x10u32));
                edx = esi.wrapping_add(0x1118u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f3a0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x1070u32));
                fl = lift::logic(eax & eax, 32);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                fl = lift::sub(ecx, 0x2u32, 32).1;
                if !fl.zf { pc = 120; continue; }
                pc = 116; continue;
            }
            116 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w32(esi.wrapping_add(0x1058u32), ebp);
                if fl.zf { pc = 119; continue; }
                pc = 117; continue;
            }
            117 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x792u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(edi.wrapping_add(0x790u32))), 16).1;
                if (fl.cf || fl.zf) { pc = 119; continue; }
                pc = 118; continue;
            }
            118 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                edi = esi.wrapping_add(0xc94u32);
                pc = 126; continue;
            }
            119 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                edi = esi.wrapping_add(0xc94u32);
                pc = 126; continue;
            }
            120 => {
                fl = lift::sub(ecx, 0x28u32, 32).1;
                if !fl.zf { pc = 127; continue; }
                pc = 121; continue;
            }
            121 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w32(esi.wrapping_add(0x1058u32), ebp);
                if fl.zf { pc = 124; continue; }
                pc = 122; continue;
            }
            122 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x789u32)))) & 0xff);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(edi.wrapping_add(0x2d4u32));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 124; continue; }
                pc = 123; continue;
            }
            123 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 125; continue;
            }
            124 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 125; continue;
            }
            125 => {
                edi = esi.wrapping_add(0xd7cu32);
                pc = 126; continue;
            }
            126 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x47b270, ecx, &a); }
                m.w32(esi.wrapping_add(0x1054u32), ebx);
                pc = 98; continue;
            }
            127 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 145; continue; }
                pc = 128; continue;
            }
            128 => {
                m.w32(esi.wrapping_add(0x1058u32), ebx);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x672fc0u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x2cu32, 8).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                if !fl.zf { pc = 130; continue; }
                pc = 129; continue;
            }
            129 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(0x6dfc14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4f74f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2cu32);
                pc = 139; continue;
            }
            130 => {
                fl = lift::sub(ecx, 0x1fu32, 32).1;
                if !fl.zf { pc = 132; continue; }
                pc = 131; continue;
            }
            131 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                pc = 138; continue;
            }
            132 => {
                fl = lift::sub(ecx, 0x1eu32, 32).1;
                if !fl.zf { pc = 134; continue; }
                pc = 133; continue;
            }
            133 => {
                ecx = lift::sx8((eax & 0xff));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 139; continue;
            }
            134 => {
                fl = lift::sub((eax & 0xff), 0x29u32, 8).1;
                if !fl.zf { pc = 136; continue; }
                pc = 135; continue;
            }
            135 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x29u32);
                pc = 139; continue;
            }
            136 => {
                fl = lift::sub(ecx, 0x34u32, 32).1;
                if !fl.zf { pc = 140; continue; }
                pc = 137; continue;
            }
            137 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(0x677929u32))) & 0xff);
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 138; continue;
            }
            138 => {
                eax = lift::sx8((eax & 0xff));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 139; continue;
            }
            139 => {
                ecx = m.r32(0x6e51acu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5928a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x58cc90, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x47b270, ecx, &a); }
                m.w32(esi.wrapping_add(0x1054u32), ebx);
                pc = 98; continue;
            }
            140 => {
                ecx = lift::sx8((eax & 0xff));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x200000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 139; continue;
            }
            141 => {
                edi = m.r32(esp.wrapping_add(0x10u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x766u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(edi.wrapping_add(0x776u32))), 16).1;
                if !fl.cf { pc = 143; continue; }
                pc = 142; continue;
            }
            142 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi.wrapping_add(0x1074u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f1f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                eax = (eax & !0xffff) | (((edx & 0xffff)) & 0xffff);
                ecx = edx;
                m.w32(esi.wrapping_add(0x1070u32), edx);
                m.w16(esi.wrapping_add(0x1154u32), ((eax & 0xffff)) as u16);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6a57c8u32));
                eax = m.r32(0x6c22ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi.wrapping_add(0x1078u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                edx = esi.wrapping_add(0x1118u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f3a0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(edi.wrapping_add(0x2c8u32));
                fl = lift::logic(eax & eax, 32);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let (r, f) = lift::shr(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                ecx = (ecx & !0xff) | ((!(ecx & 0xff)) & 0xff);
                let r = (ecx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (eax & 0xff) & (ecx & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ecx = edi;
                m.w8(esp.wrapping_add(0x1cu32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x522a00, ecx, &a); }
                pc = 144; continue;
            }
            143 => {
                edx = esi.wrapping_add(0x1118u32);
                ebp = ecx.wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                let r = ebp & 0x3u32; fl = lift::logic(r, 32);
                ebp = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5226c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6c230cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esi.wrapping_add(0x1078u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                ecx = edi.wrapping_add(0x7a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4fcfe0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esp.wrapping_add(0x1cu32), ((eax & 0xff)) as u8);
                pc = 144; continue;
            }
            144 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esi.wrapping_add(0xe64u32));
                edi = esi.wrapping_add(0xe64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                m.w32(esi.wrapping_add(0x105cu32), ebx);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x58cc90, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x1058u32), 0x0u32);
                pc = 145; continue;
            }
            145 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x47b270, ecx, &a); }
                m.w32(esi.wrapping_add(0x1054u32), ebx);
                pc = 98; continue;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047B270` build_side_menu_format_cost_strings
pub fn f_0047b270(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                esi = ebx.wrapping_add(0x1128u32);
                edi = ebx.wrapping_add(0x10a0u32);
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xfffffff0u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = edi.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9b10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 4; continue;
            }
            3 => {
                m.w32(edi.wrapping_add(0xffffffe0u32), 0x0u32);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9b14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 7; continue;
            }
            6 => {
                m.w32(edi, 0x0u32);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x10u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9b18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 10; continue;
            }
            9 => {
                m.w32(edi.wrapping_add(0x20u32), 0x0u32);
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x10u32));
                eax = m.r32(esi);
                ecx = m.r32(esi.wrapping_add(0xfffffff0u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                eax = (eax & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                m.w8(ebx.wrapping_add(ebp).wrapping_add(0x1110u32), ((eax & 0xff)) as u8);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edi, 0x8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(esi, 0x4u32, 32); fl = f;
                esi = r;
                fl = lift::sub(ebp, 0x4u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(ebx.wrapping_add(0x1148u32));
                edx = ebx.wrapping_add(0x10e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9b1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                eax = m.r32(ebx.wrapping_add(0x114cu32));
                ecx = ebx.wrapping_add(0x10f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9b20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                eax = m.r32(ebx.wrapping_add(0x1150u32));
                esp = esp.wrapping_add(0x18);
                fl = lift::sub(eax, 0xau32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(0x6c2318u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 14; continue;
            }
            13 => {
                ecx = eax.wrapping_add(0x6u32);
                eax = 0x92492493u32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6c2328u32);
                pc = 14; continue;
            }
            14 => {
                eax = ebx.wrapping_add(0x1100u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                eax = (eax & !0xffff) | ((u32::from(m.r8(ebx.wrapping_add(0x115bu32)))) & 0xffff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x1158u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x115au32)))) & 0xff);
                esp = esp.wrapping_add(0xc);
                m.w8(ebx.wrapping_add(0x1114u32), ((ecx & 0xff)) as u8);
                m.w8(ebx.wrapping_add(0x1115u32), ((edx & 0xff)) as u8);
                m.w16(ebx.wrapping_add(0x1116u32), ((eax & 0xffff)) as u16);
                ecx = m.r32(0x6de4b4u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(ebx.wrapping_add(0x107cu32), ecx);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047B830` build_side_menu_build_placement_site_mask
pub fn f_0047b830(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                edx = eax.wrapping_add(edx.wrapping_mul(8));
                eax = m.r32(ecx.wrapping_add(0x1070u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = edx.wrapping_add(0x7a4u32);
                { let a = []; eax = h.call_at(esp, m, 0x4fd5e0, ecx, &a); }
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                fl = lift::sub(eax, 0x15u32, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(eax, 0x1eu32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = edx.wrapping_add(0x7a4u32);
                { let a = []; eax = h.call_at(esp, m, 0x4fd4a0, ecx, &a); }
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            5 => {
                fl = lift::sub(eax, 0x31u32, 32).1;
                if (fl.sf == fl.of) { pc = 10; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(edx.wrapping_add(0x2c8u32));
                let r = ecx & 0x10u32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(eax, 0x5u32, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = 0x1u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edx.wrapping_add(0x7a4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4fd0d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            9 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edx.wrapping_add(0x7a4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4fd0d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            10 => {
                m.w32(esp.wrapping_add(0x4u32), eax);
                ecx = edx;
                { let a = [m.r32(esp.wrapping_add(4))]; return h.call_at(esp, m, 0x5241e0, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047C120` personal_letters_side_menu_ctor
pub fn f_0047c120(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65f977u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(0x8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    edi = ecx;
    m.w32(esp.wrapping_add(0x8u32), edi);
    { let a = []; eax = h.call_at(esp, m, 0x440900, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6u32);
    eax = edi.wrapping_add(0xd8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x2cu32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = edi.wrapping_add(0x648u32);
    m.w8(esp.wrapping_add(0x18u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    esi = edi.wrapping_add(0x9fcu32);
    m.w8(esp.wrapping_add(0x18u32), (0x2u32) as u8);
    ecx = esi;
    m.w32(esp.wrapping_add(0xcu32), esi);
    { let a = []; eax = h.call_at(esp, m, 0x41c320, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    ecx = esi.wrapping_add(0xd0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x2cu32), (0x3u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    edx = esi.wrapping_add(0x388u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x2cu32), (0x4u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0x640u32);
    m.w8(esp.wrapping_add(0x18u32), (0x5u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    eax = esi.wrapping_add(0x728u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x2cu32), (0x6u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    ecx = esi.wrapping_add(0x9e0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x2cu32), (0x7u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0xc98u32);
    m.w8(esp.wrapping_add(0x18u32), (0x8u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4ace40u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4acdd0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    edx = esi.wrapping_add(0xd80u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xd8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x2cu32), (0x9u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4ace40u32);
    m.w8(esp.wrapping_add(0x1cu32), (0xau32) as u8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4acdd0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    eax = esi.wrapping_add(0x1008u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xd8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0x1290u32);
    m.w8(esp.wrapping_add(0x18u32), (0xbu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4acdd0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x10u32));
    m.w32(esi, 0x66f1e0u32);
    m.w32(edi, 0x66f0a8u32);
    m.w32(edi.wrapping_add(0xb8u32), 0x0u32);
    m.w32(edi.wrapping_add(0xb4u32), 0x1u32);
    eax = edi;
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x14);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
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

/// `0X0047C520` personal_letters_side_menu_build_row_model
pub fn f_0047c520(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x6dfc14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                m.w32(esp.wrapping_add(0x14u32), esi);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d6780, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6dfc14u32);
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                eax = 0x3u32;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xd4u32))), (eax & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w16(esi.wrapping_add(0xd4u32), ((eax & 0xffff)) as u16);
                pc = 4; continue;
            }
            4 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub((edi & 0xffff), (edx & 0xffff), 16).1;
                eax = edi;
                if !fl.cf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                esi = m.r32(0x6dd730u32);
                pc = 6; continue;
            }
            6 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(esi).wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (edx & 0xffff), 16).1;
                if fl.cf { pc = 6; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ebx = ecx;
                m.w32(esp.wrapping_add(0x18u32), ebx);
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            9 => {
                esi = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                eax = esi.wrapping_add(esi.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x14u32), eax);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 23; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::logic(ebx & ebx, 32);
                if (fl.sf != fl.of) { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = eax;
                pc = 12; continue;
            }
            12 => {
                m.w16(ecx, (0xffffu32) as u16);
                let (r, f) = lift::add(ecx, 0x14u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                if !fl.zf { pc = 12; continue; }
                pc = 13; continue;
            }
            13 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::logic(ebx & ebx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 22; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = eax.wrapping_add(0x4u32);
                pc = 15; continue;
            }
            15 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(edi, edx, 32).1;
                if (fl.sf == fl.of) { pc = 22; continue; }
                pc = 16; continue;
            }
            16 => {
                if (!fl.cf && !fl.zf) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(0x6dd730u32);
                esi = edi;
                let (r, f) = lift::shl(esi, 0x4u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                pc = 19; continue;
            }
            18 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 19; continue;
            }
            19 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xff);
                eax = (eax & !0xffff) | ((u32::from(m.r8(edx.wrapping_add(0x6c0198u32)))) & 0xffff);
                edx = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(edx.wrapping_add(0xd4u32))), 16).1;
                if !fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                m.w16(ecx.wrapping_add(0xfffffffcu32), ((edi & 0xffff)) as u16);
                edi = 0xau32;
                eax = (eax & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xffff);
                m.w16(ecx.wrapping_add(0xfffffffeu32), ((eax & 0xffff)) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x5u32)))) & 0xffff);
                m.w16(ecx, ((edx & 0xffff)) as u16);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1u32)))) & 0xff);
                let (r, f) = lift::shr((eax & 0xff), 0x7u32, 8, fl); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = (edx & 0xffff) ^ (edx & 0xffff); fl = lift::logic(r, 16);
                edx = (edx & !0xffff) | ((r) & 0xffff);
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                eax = 0x66666667u32;
                m.w16(ecx.wrapping_add(0x2u32), ((edx & 0xffff)) as u16);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add((edx & 0xff), 0x30u32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(ecx.wrapping_add(0x4u32), ((edx & 0xff)) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(edi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w8(ecx.wrapping_add(0x6u32), (0x2eu32) as u8);
                let (r, f) = lift::add((edx & 0xff), 0x30u32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(ecx.wrapping_add(0x5u32), ((edx & 0xff)) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1u32)))) & 0xff);
                let r = eax & 0xfu32; fl = lift::logic(r, 32);
                eax = r;
                m.w8(ecx.wrapping_add(0x9u32), (0x2eu32) as u8);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(edi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::add((edx & 0xff), 0x30u32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::add((eax & 0xff), 0x30u32, 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(ecx.wrapping_add(0x8u32), ((edx & 0xff)) as u8);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w8(ecx.wrapping_add(0x7u32), ((eax & 0xff)) as u8);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x2u32)))) & 0xffff);
                eax = 0x66666667u32;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                ebx = edx;
                edx = (edx & !0xff) | ((0xau32) & 0xff);
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                { let r = i32::from(eax as u8 as i8) * i32::from(edx as u8 as i8); eax = (eax & !0xffff) | (r as u32 & 0xffff); fl.cf = r != i32::from(r as i8); fl.of = fl.cf; }
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2u32)))) & 0xff);
                let (r, f) = lift::sub((edx & 0xff), (eax & 0xff), 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                eax = 0x66666667u32;
                let (r, f) = lift::add((edx & 0xff), 0x30u32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(ecx.wrapping_add(0xdu32), ((edx & 0xff)) as u8);
                { let r = i64::from(eax as i32) * i64::from(ebx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                edi = edx;
                edx = (edx & !0xff) | ((0xau32) & 0xff);
                let (r, f) = lift::sar(edi, 0x2u32, 32, fl); fl = f;
                edi = r;
                eax = edi;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = edi;
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let r = i32::from(eax as u8 as i8) * i32::from(edx as u8 as i8); eax = (eax & !0xffff) | (r as u32 & 0xffff); fl.cf = r != i32::from(r as i8); fl.of = fl.cf; }
                let (r, f) = lift::sub((ebx & 0xff), (eax & 0xff), 8); fl = f;
                ebx = (ebx & !0xff) | ((r) & 0xff);
                eax = 0x66666667u32;
                { let r = i64::from(eax as i32) * i64::from(edi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::add((ebx & 0xff), 0x30u32, 8); fl = f;
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w8(ecx.wrapping_add(0xcu32), ((ebx & 0xff)) as u8);
                eax = (eax & !0xff) | (((edx & 0xff)) & 0xff);
                ebx = (ebx & !0xff) | ((0xau32) & 0xff);
                { let r = i32::from(eax as u8 as i8) * i32::from(ebx as u8 as i8); eax = (eax & !0xffff) | (r as u32 & 0xffff); fl.cf = r != i32::from(r as i8); fl.of = fl.cf; }
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x10u32)))) & 0xff);
                let (r, f) = lift::sub((eax & 0xff), (ebx & 0xff), 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::add((eax & 0xff), 0x30u32, 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add((edx & 0xff), 0x30u32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(ecx.wrapping_add(0xbu32), ((eax & 0xff)) as u8);
                m.w8(ecx.wrapping_add(0xau32), ((edx & 0xff)) as u8);
                m.w8(ecx.wrapping_add(0xeu32), (0x0u32) as u8);
                m.w8(ecx.wrapping_add(0xfu32), (0x0u32) as u8);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(ecx, 0x14u32, 32); fl = f;
                ecx = r;
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(ebp, ebx, 32).1;
                if (fl.sf != fl.of) { pc = 15; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                ecx = ebp.wrapping_add(ebp.wrapping_mul(4));
                m.w16(eax.wrapping_add(ecx.wrapping_mul(4)), (0xffffu32) as u16);
                m.w16(edx.wrapping_add(0xccu32), ((ebp & 0xffff)) as u16);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            23 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(edx.wrapping_add(0xccu32), ((ebp & 0xffff)) as u16);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
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

/// `0X0047D520` personal_letters_side_menu_hide
pub fn f_0047d520(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x440ed0, ecx, &a); }
    ecx = m.r32(0x6cbb10u32);
    eax = m.r32(ecx);
    { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    edx = m.r32(esi.wrapping_add(0x9fcu32));
    ecx = esi.wrapping_add(0x9fcu32);
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(esi.wrapping_add(0x650u32));
    ecx = esi.wrapping_add(0x650u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w16(esi.wrapping_add(0xceu32), (0x0u32) as u16);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X0047D570` personal_letters_side_menu_close
pub fn f_0047d570(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                edi = esi.wrapping_add(0x650u32);
                ebx = eax;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ecx = edi;
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                m.w16(esi.wrapping_add(0xceu32), ((ebp & 0xffff)) as u16);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x461480, ecx, &a); }
                fl = lift::sub(esi, m.r32(0x6cbd90u32), 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::sub(eax, ebp, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 3; continue;
            }
            3 => {
                m.w32(esi.wrapping_add(0xa0u32), ebp);
                pc = 4; continue;
            }
            4 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1d64u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(m.r32(0x6cbd8cu32), esi, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                edi = esi.wrapping_add(0x648u32);
                ebp = 0x5u32;
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::sub(edi, 0xe8u32, 32); fl = f;
                edi = r;
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 7; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w8(esi.wrapping_add(0x1d64u32), (0x0u32) as u8);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, ebp, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(m.r32(0x6cbd8cu32), esi, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb490, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xb8u32), ebp);
                pc = 12; continue;
            }
            12 => {
                ecx = esi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x4410f0, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047D630` personal_letters_side_menu_draw
pub fn f_0047d630(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x30);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x48u32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                ebp = m.r32(esi.wrapping_add(0x30u32));
                ebx = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 44; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 44; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(0xc);
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                edx = esp;
                m.w32(edx, edi);
                edi = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(edx.wrapping_add(0x4u32), eax);
                m.w32(edx.wrapping_add(0x8u32), ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x440b20, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 44; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(0x6cbd5cu32);
                edx = ((eax as i32) >> 31) as u32;
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(0x6cbd54u32);
                edi = eax;
                let (r, f) = lift::sar(edi, 0x1u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                esp = esp.wrapping_add(0x8);
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xd4u32))), 0x4u32, 16).1;
                if !fl.cf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = 0x6dcf20u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                eax = m.r32(0x66e0c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = edi.wrapping_add(0x1u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd4u32)))) & 0xffff);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6b2dd8u32));
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x38u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd4u32)))) & 0xffff);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6b2dd8u32));
                ecx = m.r32(esp.wrapping_add(0x40u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x2c);
                pc = 5; continue;
            }
            5 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                fl = lift::logic(ebx & ebx, 32);
                ebp = edi.wrapping_add(edx.wrapping_mul(2));
                if fl.zf { pc = 41; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(u32::from(m.r16(ebx)), 0xffffu32, 16).1;
                if !fl.cf { pc = 10; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xceu32)))) & 0xffff);
                pc = 8; continue;
            }
            8 => {
                edi = eax;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::add(ebx, 0x14u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(u32::from(m.r16(ebx)), 0xffffu32, 16).1;
                if fl.cf { pc = 8; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xceu32)))) & 0xffff);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                m.w16(esp.wrapping_add(0x44u32), ((eax & 0xffff)) as u16);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xbeu32)))) & 0xffff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xbcu32)))) & 0xffff);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc0u32)))) & 0xffff);
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc8u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x48u32), eax);
                let (r, f) = lift::imul(ecx, edx); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x9f4u32));
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x30u32), edi);
                m.w32(esp.wrapping_add(0x38u32), eax);
                m.w32(esp.wrapping_add(0x34u32), ebp);
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx))) & 0xffff);
                let (r, f) = lift::sub(ebp, edx, 32); fl = f;
                ebp = r;
                fl = lift::sub((eax & 0xffff), 0xffffu32, 16).1;
                m.w32(esp.wrapping_add(0x14u32), edi);
                if !fl.cf { pc = 41; continue; }
                pc = 11; continue;
            }
            11 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 14; continue;
            }
            13 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x4u32))), 0x86u32, 8).1;
                if !fl.cf { pc = 39; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esp.wrapping_add(0x44u32))), 16).1;
                if !fl.zf { pc = 21; continue; }
                pc = 16; continue;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64ffe6cau32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(ebp, edx, 32).1;
                if (fl.sf != fl.of) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = ebp;
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                let (r, f) = lift::sub(eax, 0x2u32, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = ecx;
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::sub(ecx, edi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb430, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 21; continue;
            }
            21 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x44u32)))) & 0xffff);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xd0u32))), (edx & 0xffff), 16).1;
                if !fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 26; continue;
            }
            23 => {
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(0x6u32))), 0x1u32, 16).1;
                if !fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                pc = 26; continue;
            }
            25 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff5a2406u32);
                pc = 26; continue;
            }
            26 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc4u32)))) & 0xffff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edi = ebx.wrapping_add(0x8u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = esp.wrapping_add(0x38u32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xfffffffcu32)))) & 0xffff);
                eax = m.r32(edx.wrapping_mul(4).wrapping_add(0x6dda00u32));
                edx = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x30u32));
                eax = esp.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_add(0x2c);
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(0x6u32))), 0x1u32, 16).1;
                if !fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc6u32)))) & 0xffff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 28; continue;
            }
            28 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebx))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                if (!fl.cf && !fl.zf) { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                edx = m.r32(0x6dd730u32);
                eax = ecx;
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 31; continue;
            }
            30 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 31; continue;
            }
            31 => {
                eax = (eax & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x3cu32, 16).1;
                m.w16(ebx.wrapping_add(0x2u32), ((eax & 0xffff)) as u16);
                if fl.cf { pc = 36; continue; }
                pc = 32; continue;
            }
            32 => {
                fl = lift::sub((eax & 0xffff), 0x40u32, 16).1;
                if (!fl.cf && !fl.zf) { pc = 36; continue; }
                pc = 33; continue;
            }
            33 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(ecx, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(ecx.wrapping_add(0xcu32));
                pc = 37; continue;
            }
            35 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = m.r32(ecx.wrapping_add(0xcu32));
                pc = 37; continue;
            }
            36 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = m.r32(eax.wrapping_mul(4).wrapping_add(0x6a52d0u32));
                pc = 37; continue;
            }
            37 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = esp.wrapping_add(0x30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x10);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esp.wrapping_add(0x3cu32));
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf == fl.of) { pc = 41; continue; }
                pc = 38; continue;
            }
            38 => {
                edi = m.r32(esp.wrapping_add(0x14u32));
                pc = 40; continue;
            }
            39 => {
                m.w32(esi.wrapping_add(0xb4u32), 0x1u32);
                pc = 40; continue;
            }
            40 => {
                edx = m.r32(esp.wrapping_add(0x44u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x14u32)))) & 0xffff);
                let (r, f) = lift::add(ebx, 0x14u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub((eax & 0xffff), 0xffffu32, 16).1;
                m.w32(esp.wrapping_add(0x44u32), edx);
                if fl.cf { pc = 11; continue; }
                pc = 41; continue;
            }
            41 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 43; continue; }
                pc = 42; continue;
            }
            42 => {
                ecx = m.r32(0x6cbd64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(0x6cbd5cu32);
                eax = m.r32(0x6cbd58u32);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 43; continue;
            }
            43 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 44; continue;
            }
            44 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x30);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0047DAA0` personal_letters_side_menu_prerender
pub fn f_0047daa0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(0xc);
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
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
    ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
    edi = m.r32(esi.wrapping_add(0x14u32));
    ebp = m.r32(esi.wrapping_add(0x18u32));
    edx = m.r32(0x66d504u32);
    ebx = m.r32(esi.wrapping_add(0x1cu32));
    esp = esp.wrapping_sub(0x8);
    m.w8(esp.wrapping_add(0x1fu32), ((ecx & 0xff)) as u8);
    ecx = esp;
    let (r, f) = lift::sub(0, edi, 32); fl = f;
    edi = r;
    m.w32(ecx, edi);
    m.w32(esp.wrapping_add(0x24u32), eax);
    eax = m.r32(esi.wrapping_add(0x94u32));
    m.w32(esi.wrapping_add(0x94u32), edx);
    let (r, f) = lift::sub(0, ebp, 32); fl = f;
    ebp = r;
    m.w32(ecx.wrapping_add(0x4u32), ebp);
    m.w8(esi.wrapping_add(0xa4u32), (0x0u32) as u8);
    edx = m.r32(0x6cbd94u32);
    m.w32(esp.wrapping_add(0x20u32), eax);
    eax = m.r32(esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    let (r, f) = lift::sub(0, ebx, 32); fl = f;
    ebx = r;
    m.w32(ecx.wrapping_add(0x8u32), ebx);
    ecx = esi;
    { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(0x6cbd94u32);
    esp = esp.wrapping_sub(0xc);
    eax = esp;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi.wrapping_add(0x650u32);
    m.w32(eax, edi);
    m.w32(eax.wrapping_add(0x4u32), ebp);
    m.w32(eax.wrapping_add(0x8u32), ebx);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x461fe0, ecx, &a); }
    esp = esp.wrapping_add(16);
    ecx = m.r32(esp.wrapping_add(0x18u32));
    edx = m.r32(esp.wrapping_add(0x14u32));
    eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w32(esi.wrapping_add(0x94u32), edx);
    m.w8(esi.wrapping_add(0xa4u32), ((eax & 0xff)) as u8);
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
    esp = esp.wrapping_add(0xc);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X0047DB50` personal_letters_side_menu_on_mouse_event
pub fn f_0047db50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x10u32), ((eax & 0xffff)) as u16);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1d64u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 20; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(0x6cbd5cu32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(0x6cbd54u32);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                edi = edx.wrapping_add(ecx.wrapping_mul(2));
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ebp = m.r32(ecx.wrapping_add(0x4u32));
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                m.w32(ecx.wrapping_add(0x4u32), ebp);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xbcu32)))) & 0xffff);
                ecx = m.r32(ecx);
                eax = ebp;
                ebp = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x28u32), edx);
                let (r, f) = lift::add(edx, 0xau32, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc0u32)))) & 0xffff);
                edx = edx.wrapping_add(ebp).wrapping_add(0xfffffff6u32);
                fl = lift::sub(ecx, edx, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x30u32)), 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xceu32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add((ebx & 0xffff), (eax & 0xffff), 16); fl = f;
                ebx = (ebx & !0xffff) | ((r) & 0xffff);
                fl = lift::sub(ecx, 0xfffffffeu32, 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x10u32), ebx);
                pc = 8; continue;
            }
            7 => {
                eax = 0xffffu32;
                ebx = eax;
                pc = 8; continue;
            }
            8 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x10u32)))) & 0xffff);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xd2u32))), (edx & 0xffff), 16).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(esi.wrapping_add(0xd0u32))), 16).1;
                if fl.zf { pc = 20; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc0u32)))) & 0xffff);
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = edx.wrapping_add(ebp).wrapping_add(0x1u32);
                edx = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x14u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd0u32)))) & 0xffff);
                fl = lift::sub((ebx & 0xffff), (eax & 0xffff), 16).1;
                if fl.zf { pc = 15; continue; }
                pc = 11; continue;
            }
            11 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xceu32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, edx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 12; continue;
            }
            12 => {
                ebp = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.sf == fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            14 => {
                m.w16(esi.wrapping_add(0xd0u32), ((ebx & 0xffff)) as u16);
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                ebx = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub((eax & 0xffff), (ebx & 0xffff), 16).1;
                if fl.zf { pc = 20; continue; }
                pc = 16; continue;
            }
            16 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xceu32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xc2u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, edx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 19; continue; }
                pc = 17; continue;
            }
            17 => {
                edi = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                fl = lift::sub(eax, edi, 32).1;
                if (fl.sf == fl.of) { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 19; continue;
            }
            19 => {
                ecx = esi;
                m.w16(esi.wrapping_add(0xd2u32), ((ebx & 0xffff)) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x47f3b0, ecx, &a); }
                pc = 20; continue;
            }
            20 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004869A0` personal_letters_window_jump_to_first_unread_letter
pub fn f_004869a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ecx);
                eax = m.r32(0x6dfc14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebp & 0xffff), 16).1;
                if !fl.cf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | (((ebp & 0xffff)) & 0xffff);
                pc = 2; continue;
            }
            2 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, ecx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esi = eax;
                pc = 5; continue;
            }
            4 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x1u32))), 0x80u32, 8).1;
                if fl.cf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x6c0198u32)))) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((eax & 0xff), 0x4u32, 8).1;
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ebx = (ebx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                fl = lift::logic(ebx & edx, 32);
                if !fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebp & 0xffff), 16).1;
                if fl.cf { pc = 2; continue; }
                pc = 8; continue;
            }
            8 => {
                pc = 11; continue;
            }
            9 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x6c0198u32)))) & 0xffff);
                m.w16(edi.wrapping_add(0xd4u32), ((ecx & 0xffff)) as u16);
                pc = 11; continue;
            }
            10 => {
                esi = m.r32(esp.wrapping_add(0x10u32));
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(edi.wrapping_add(0xa0u32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(eax, ebp, 32).1;
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 13; continue;
            }
            13 => {
                ecx = edi;
                m.w32(edi.wrapping_add(0xa0u32), ebp);
                { let a = []; eax = h.call_at(esp, m, 0x47c520, ecx, &a); }
                fl = lift::sub(eax, ebp, 32).1;
                m.w32(edi.wrapping_add(0xa0u32), eax);
                if fl.zf { pc = 19; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(u32::from(m.r16(eax)), 0xffffu32, 16).1;
                if !fl.cf { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                esi = (esi & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xffff);
                ecx = 0x1u32;
                pc = 16; continue;
            }
            16 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((esi & 0xffff), u32::from(m.r16(eax.wrapping_add(0x2u32))), 16).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x6u32))), (ecx & 0xffff), 16).1;
                ebx = (ebx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::logic(ebx & edx, 32);
                if !fl.zf { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::add(eax, 0x14u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(u32::from(m.r16(eax)), 0xffffu32, 16).1;
                if fl.cf { pc = 16; continue; }
                pc = 18; continue;
            }
            18 => {
                m.w16(edi.wrapping_add(0xceu32), (0x0u32) as u16);
                m.w16(edi.wrapping_add(0xd2u32), (0xffffu32) as u16);
                pc = 19; continue;
            }
            19 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = edi.wrapping_add(0x648u32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xccu32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            20 => {
                fl = lift::sub(u32::from(m.r16(eax)), 0xffffu32, 16).1;
                if !fl.cf { pc = 18; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = edi;
                m.w16(edi.wrapping_add(0xd2u32), ((ebp & 0xffff)) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x47f3b0, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0x94u32));
                ecx = m.r32(0x66d504u32);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 19; continue; }
                pc = 22; continue;
            }
            22 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esi = edi.wrapping_add(0x648u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xccu32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xc2u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xd2u32)))) & 0xffff);
                let (r, f) = lift::imul(edx, eax); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi.wrapping_add(0x650u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x461dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x9ecu32)))) & 0xffff);
                m.w16(edi.wrapping_add(0xceu32), ((ecx & 0xffff)) as u16);
                pc = 19; continue;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004958F0` statistics_side_menu_dtor
pub fn f_004958f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66081du32);
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
                m.w32(edi, 0x66f4c8u32);
                ebx = 0x4u32;
                esi = edi.wrapping_add(0x2ab8u32);
                m.w32(esp.wrapping_add(0x20u32), ebx);
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
                ebp = edi.wrapping_add(0x2e58u32);
                ebx = 0x7u32;
                esi = ebp;
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                esi = edi.wrapping_add(0xc0u32);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x49dfe0, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0xa0u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(edi.wrapping_add(0xa0u32), ebx);
                pc = 6; continue;
            }
            6 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x440ab0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4b0440u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x7u32);
                eax = edi.wrapping_add(0x34b0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x30u32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x7u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                m.w8(esp.wrapping_add(0x30u32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                eax = edi.wrapping_add(0x2ab8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x30u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0x14u32), esi);
                ecx = esi.wrapping_add(0x262cu32);
                m.w8(esp.wrapping_add(0x20u32), (0x12u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x430fe0, ecx, &a); }
                ecx = esi.wrapping_add(0x2358u32);
                m.w8(esp.wrapping_add(0x20u32), (0x11u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0x2270u32);
                m.w8(esp.wrapping_add(0x20u32), (0x10u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0x2188u32);
                m.w8(esp.wrapping_add(0x20u32), (0xfu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0x20a0u32);
                m.w8(esp.wrapping_add(0x20u32), (0xeu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0xfc8u32);
                m.w8(esp.wrapping_add(0x20u32), (0xdu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x46f860, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                ecx = esi.wrapping_add(0xc28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w8(esp.wrapping_add(0x30u32), (0xcu32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4b0440u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                edx = esi.wrapping_add(0xba8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x30u32), (0xbu32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4b0440u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                eax = esi.wrapping_add(0xb28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x30u32), (0xau32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                m.w8(esp.wrapping_add(0x28u32), (0x9u32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                ecx = esi.wrapping_add(0x784u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                edx = esi.wrapping_add(0x3e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x30u32), (0x8u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = esi.wrapping_add(0x2fcu32);
                m.w8(esp.wrapping_add(0x20u32), (0x7u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0x214u32);
                m.w8(esp.wrapping_add(0x20u32), (0x6u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0x20cu32);
                m.w8(esp.wrapping_add(0x20u32), (0x5u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esi;
                m.w8(esp.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x41c3a0, ecx, &a); }
                ecx = edi;
                m.w32(esp.wrapping_add(0x20u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x440a20, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00495B30` statistics_side_menu_release_resources
pub fn f_00495b30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                ebx = 0x4u32;
                esi = edi.wrapping_add(0x2ab8u32);
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
                esi = edi.wrapping_add(0x2e58u32);
                ebx = 0x7u32;
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = edi.wrapping_add(0xc0u32);
                { let a = []; eax = h.call_at(esp, m, 0x49dfe0, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0xa0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(edi.wrapping_add(0xa0u32), 0x0u32);
                pc = 6; continue;
            }
            6 => {
                ecx = edi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x440ab0, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00495C10` statistics_side_menu_load_layout
pub fn f_00495c10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66083fu32);
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
                eax = m.r32(0x6c7cccu32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                m.w32(esp.wrapping_add(0x3cu32), ebp);
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x10u32), eax);
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
                m.w32(esp, 0x6ba528u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba514u32);
                ecx = 0x6dd524u32;
                m.w8(esp.wrapping_add(0x54u32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
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
                m.w32(esp, 0x6ba530u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba514u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba538u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba514u32);
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
                edi = esi.wrapping_add(0x2ab8u32);
                pc = 1; continue;
            }
            1 => {
                eax = ebx.wrapping_add(ebp);
                edx = esp.wrapping_add(0x14u32);
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
                fl = lift::sub(ebp, 0x4u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x66de5cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x2e58u32));
                edi = esi.wrapping_add(0x2e58u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = esi.wrapping_add(0x2f40u32);
                ebx = 0x6u32;
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                ebp = esi.wrapping_add(0x34b0u32);
                ebx = 0x7u32;
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(edi);
                m.w32(edi.wrapping_add(0xc0u32), 0xff000000u32);
                eax = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                m.w32(edi.wrapping_add(0xc4u32), eax);
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0x20u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 5; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(0x6dcf20u32);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dcf20u32;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x2cu32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba548u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba514u32);
                ecx = 0x6dd524u32;
                m.w16(esi.wrapping_add(0xb2u32), ((eax & 0xffff)) as u16);
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
                eax = m.r32(esi.wrapping_add(0xc0u32));
                ecx = esi.wrapping_add(0xc0u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi);
                ecx = esi;
                m.w16(esi.wrapping_add(0xb0u32), (0x2u32) as u16);
                m.w8(esi.wrapping_add(0x3597u32), (0x0u32) as u8);
                m.w8(esi.wrapping_add(0xb4u32), (0x0u32) as u8);
                m.w8(esi.wrapping_add(0xb5u32), (0x6u32) as u8);
                m.w8(esi.wrapping_add(0xb6u32), (0xbu32) as u8);
                m.w8(esi.wrapping_add(0xb7u32), (0x10u32) as u8);
                m.w8(esi.wrapping_add(0xb8u32), (0x13u32) as u8);
                m.w8(esi.wrapping_add(0xb9u32), (0x16u32) as u8);
                m.w8(esi.wrapping_add(0xbau32), (0x1cu32) as u8);
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
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

/// `0X00495E90` statistics_side_menu_layout_buttons
pub fn f_00495e90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4417d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(0x6cbd9cu32);
                ebp = esi.wrapping_add(0x2ab8u32);
                ebx = ebp;
                m.w32(esp.wrapping_add(0x10u32), 0x4u32);
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = 0x66666667u32;
                ebx = esi.wrapping_add(0x2c88u32);
                { let r = i64::from(eax as i32) * i64::from(edi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                ecx = ebx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x14u32);
                edi = edx;
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x18u32), edx);
                edx = m.r32(ebx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0x2d70u32));
                m.w32(esp.wrapping_add(0x14u32), ebx);
                ebx = esi.wrapping_add(0x2d70u32);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x14u32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                m.w32(esp.wrapping_add(0x18u32), ebx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(0x2ba0u32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                edx = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0x14u32), ebx);
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2e58u32));
                edi = esi.wrapping_add(0x2e58u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                ebp = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xb2u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x14u32), ebp);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(edi);
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esi = eax.wrapping_add(eax.wrapping_mul(2));
                ebx = 0x7u32;
                let (r, f) = lift::sar(esi, 0x1u32, 32, fl); fl = f;
                esi = r;
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(edx, esi, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esp.wrapping_add(0x18u32), edx);
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
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00495FF0` statistics_side_menu_show
pub fn f_00495ff0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(edi.wrapping_add(0xacu32), 0xffffffffu32);
                m.w16(edi.wrapping_add(0xb0u32), (0x2u32) as u16);
                m.w8(edi.wrapping_add(0x3590u32), (0x1u32) as u8);
                m.w32(edi.wrapping_add(0x3591u32), eax);
                esi = edi.wrapping_add(0x2e58u32);
                m.w16(edi.wrapping_add(0x3595u32), ((eax & 0xffff)) as u16);
                ecx = esi;
                edx = m.r32(esi);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0x2cu32));
                ebp = m.r32(edi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), 0x64u32);
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
                m.w32(esp.wrapping_add(0x1cu32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xb2u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esi);
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = eax;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xb0u32)))) & 0xffff);
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(edi).wrapping_add(0xb4u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x3597u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 9; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebp = edi.wrapping_add(0x2ab8u32);
                ebx = 0x4u32;
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebx = edi.wrapping_add(0x34b0u32);
                edx = ecx.wrapping_mul(8).wrapping_add(0x6cc830u32);
                m.w32(esp.wrapping_add(0x10u32), edx);
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
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
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                fl = lift::sub(edx, 0x1bu32, 32).1;
                if (fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 7; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba554u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba556u32);
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(eax, 0x8u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, 0x20u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(ebp, 0x7u32, 32).1;
                m.w32(esp.wrapping_add(0x24u32), edx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w8(edi.wrapping_add(0x3597u32), (0x1u32) as u8);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            9 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                edx = m.r32(edi.wrapping_add(0xc0u32));
                ecx = edi.wrapping_add(0xc0u32);
                esp = esp.wrapping_add(0x4);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004961C0` statistics_side_menu_show_buttons
pub fn f_004961c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, edi);
                edi = ecx;
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xb0u32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xb4u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xb5u32)))) & 0xff);
                ebp = ecx;
                eax = edx;
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0xcu32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = edi.wrapping_add(0x2e58u32);
                pc = 2; continue;
            }
            2 => {
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
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x17eu32)))) & 0xffff);
                fl = lift::sub(ebp, ecx, 32).1;
                ecx = esi;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x17eu32)))) & 0xffff);
                fl = lift::sub(ebp, eax, 32).1;
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                m.w8(edi.wrapping_add(ebx).wrapping_add(0x3590u32), ((ecx & 0xff)) as u8);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                fl = lift::logic(u32::from(m.r8(0x6df2f0u32)) & 0x1u32, 8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(0x6cc3e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                eax = m.r32(edx.wrapping_add(0x4u32));
                m.w32(edi.wrapping_add(0xacu32), eax);
                ecx = m.r32(0x6cbdf0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x460540, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
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

/// `0X00496290` statistics_side_menu_hide
pub fn f_00496290(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = esp.wrapping_add(0xcu32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x440ed0, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0xc0u32));
                ecx = edi.wrapping_add(0xc0u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esi = edi.wrapping_add(0x2e58u32);
                ebx = 0x7u32;
                pc = 1; continue;
            }
            1 => {
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
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::logic(u32::from(m.r8(0x6df2f0u32)) & 0x1u32, 8);
                if fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(edi.wrapping_add(0x94u32));
                eax = m.r32(0x66d500u32);
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x66de90u32);
                ecx = m.r32(0x6dfbc4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(edi.wrapping_add(0xacu32));
                ecx = m.r32(0x6cbdf0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x460540, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00496340` statistics_side_menu_close
pub fn f_00496340(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ebp = eax;
                eax = m.r32(edi.wrapping_add(0xa0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x3597u32)))) & 0xff);
                m.w32(edi.wrapping_add(0xa0u32), 0x0u32);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(m.r32(0x6cbd8cu32), edi, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                esi = edi.wrapping_add(0x2ab8u32);
                ebx = 0x4u32;
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 5; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(edi.wrapping_add(0x3597u32), (0x0u32) as u8);
                pc = 7; continue;
            }
            7 => {
                esi = edi.wrapping_add(0x2e58u32);
                ebx = 0x7u32;
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 8; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = edi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x4410f0, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004963D0` statistics_side_menu_draw
pub fn f_004963d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0xcu32), eax);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x38u32));
                edx = esp;
                ecx = esi;
                m.w32(edx, edi);
                edi = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(edx.wrapping_add(0x4u32), ebx);
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x440b20, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                edx = m.r32(0x66e0c8u32);
                edi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = 0x6dcf20u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xb0u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xb2u32)))) & 0xffff);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6c151cu32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edx.wrapping_add(eax).wrapping_add(0xffffffffu32);
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xb0u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xb2u32)))) & 0xffff);
                eax = m.r32(edx.wrapping_mul(4).wrapping_add(0x6c151cu32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_add(edx).wrapping_add(0xfffffffeu32);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                esp = esp.wrapping_add(0x24);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6cbd64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(0x6cbd5cu32);
                edx = m.r32(0x6cbd58u32);
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
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

/// `0X00496560` statistics_side_menu_update
pub fn f_00496560(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = ecx;
                m.w8(esp.wrapping_add(0x12u32), (0x0u32) as u8);
                m.w8(esp.wrapping_add(0x13u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                ebx = edi.wrapping_add(0x2ab8u32);
                pc = 1; continue;
            }
            1 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(esi, 0x4u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 3; continue;
            }
            3 => {
                pc = 10; continue;
            }
            4 => {
                m.w16(edi.wrapping_add(0xb0u32), ((esi & 0xffff)) as u16);
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                let (r, f) = lift::add(esi, edi, 32); fl = f;
                esi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb5u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb4u32)))) & 0xff);
                ebp = ecx;
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                fl = lift::logic(ebp & ebp, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = eax.wrapping_mul(8).wrapping_add(0x6cc830u32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                esi = edi.wrapping_add(0x2e58u32);
                m.w32(esp.wrapping_add(0x14u32), edx);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esi);
                ecx = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esi);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x17eu32)))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x18u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x17eu32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                m.w8(edi.wrapping_add(ebx).wrapping_add(0x3590u32), ((ecx & 0xff)) as u8);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ecx, 0x8u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(ebx, ebp, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(ebp, 0x7u32, 32).1;
                if (fl.sf == fl.of) { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = ebp.wrapping_mul(8);
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                eax = ebp.wrapping_add(edx.wrapping_mul(4));
                esi = edi.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x2e58u32);
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba558u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba55au32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(edi.wrapping_add(ebp).wrapping_add(0x3590u32), (0x0u32) as u8);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(ebp, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebx = edi.wrapping_add(0x2e58u32);
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(ebx);
                ecx = ebx;
                m.w8(esp.wrapping_add(0x12u32), (0x1u32) as u8);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(edi.wrapping_add(ebp).wrapping_add(0x3590u32), (0x0u32) as u8);
                pc = 15; continue;
            }
            14 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ebp).wrapping_add(0x3590u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 21; continue; }
                pc = 15; continue;
            }
            15 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(ebp, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 11; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 33; continue; }
                pc = 17; continue;
            }
            17 => {
                ebp = 0xffffca70u32;
                ebx = edi.wrapping_add(0x3590u32);
                esi = edi.wrapping_add(0x2e58u32);
                let (r, f) = lift::sub(ebp, edi, 32); fl = f;
                ebp = r;
                pc = 18; continue;
            }
            18 => {
                fl = lift::sub(u32::from(m.r8(ebx)), 0x0u32, 8).1;
                if fl.zf { pc = 28; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 27; continue; }
                pc = 20; continue;
            }
            20 => {
                m.w8(ebx, (0x0u32) as u8);
                pc = 28; continue;
            }
            21 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esi = edi.wrapping_add(0x2e58u32);
                pc = 22; continue;
            }
            22 => {
                fl = lift::sub(ebp, ebx, 32).1;
                ecx = esi;
                if !fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(edi.wrapping_add(ebx).wrapping_add(0x3590u32), (0x1u32) as u8);
                pc = 25; continue;
            }
            24 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(edi.wrapping_add(ebx).wrapping_add(0x3590u32), (0x0u32) as u8);
                pc = 25; continue;
            }
            25 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(ebx, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 22; continue; }
                pc = 26; continue;
            }
            26 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xb0u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(edi).wrapping_add(0xb4u32)))) & 0xffff);
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                ecx = edi.wrapping_add(0xc0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x49e300, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 33; continue;
            }
            27 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esp.wrapping_add(0x12u32), (0x1u32) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                pc = 28; continue;
            }
            28 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                ecx = ebx.wrapping_add(ebp);
                fl = lift::sub(ecx, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 18; continue; }
                pc = 29; continue;
            }
            29 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 33; continue; }
                pc = 30; continue;
            }
            30 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xb0u32)))) & 0xffff);
                esi = (esi & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x17eu32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(edi).wrapping_add(0xb4u32)))) & 0xff);
                ecx = edx.wrapping_add(edi);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 33; continue; }
                pc = 31; continue;
            }
            31 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xb5u32)))) & 0xff);
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf == fl.of) { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                let (r, f) = lift::sub(esi, eax, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(eax.wrapping_mul(4));
                edx = m.r32(edi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2e58u32));
                ecx = edi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2e58u32);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(edi).wrapping_add(0x3590u32), (0x1u32) as u8);
                pc = 33; continue;
            }
            33 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(edi.wrapping_add(0x14u32));
                ecx = m.r32(edi.wrapping_add(0x2cu32));
                edx = m.r32(edi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x1cu32);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x2cu32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 35; continue;
            }
            35 => {
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

/// `0X00496880` statistics_side_menu_prerender
pub fn f_00496880(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = ecx;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xb0u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xb5u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xb4u32)))) & 0xff);
                esi = ecx;
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x18u32), esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xa4u32)))) & 0xff);
                edx = m.r32(0x66d504u32);
                ebp = m.r32(edi.wrapping_add(0x14u32));
                ebx = m.r32(edi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(edi.wrapping_add(0x94u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(edi.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w8(esp.wrapping_add(0x1bu32), ((ecx & 0xff)) as u8);
                m.w32(edi.wrapping_add(0x94u32), edx);
                m.w8(edi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x30u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x8);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xb0u32)))) & 0xffff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(0x8);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                eax = esp;
                edx = m.r32(edi);
                m.w32(eax, ebp);
                m.w32(eax.wrapping_add(0x4u32), ebx);
                m.w32(eax.wrapping_add(0x8u32), ecx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(esi & esi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esi = edi.wrapping_add(0x2e58u32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                eax = esp;
                edx = m.r32(esi);
                m.w32(eax, ebp);
                m.w32(eax.wrapping_add(0x4u32), ebx);
                m.w32(eax.wrapping_add(0x8u32), ecx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
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
                if !fl.zf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
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

/// `0X00496A00` statistics_side_menu_ctor
pub fn f_00496a00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66059cu32);
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
    { let a = []; eax = h.call_at(esp, m, 0x440900, ecx, &a); }
    ecx = esi.wrapping_add(0xc0u32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x49d1e0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4u32);
    eax = esi.wrapping_add(0x2ab8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x24u32), (0x1u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x7u32);
    ecx = esi.wrapping_add(0x2e58u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x24u32), (0x2u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b0440u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b03e0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x7u32);
    edx = esi.wrapping_add(0x34b0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x24u32), (0x3u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = m.r32(esp.wrapping_add(0x8u32));
    m.w32(esi, 0x66f4c8u32);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X005A5EB0` tavern_side_menu_draw
pub fn f_005a5eb0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0xcu32), eax);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x38u32));
                edx = esp;
                ecx = esi;
                m.w32(edx, edi);
                edi = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(edx.wrapping_add(0x4u32), ebx);
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x440b20, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                edi = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = 0x6dcf20u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                edx = m.r32(0x66e0c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xb04u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edx, 0xbu32, 32); fl = f;
                edx = r;
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6dd8c0u32));
                eax = m.r32(esi.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                edx = eax.wrapping_add(ecx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xb04u32));
                eax = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6dd8c0u32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                esp = esp.wrapping_add(0x24);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x6cbd64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(0x6cbd5cu32);
                eax = m.r32(0x6cbd58u32);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
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

/// `0X005A6030` tavern_side_menu_load_layout
pub fn f_005a6030(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6652e2u32);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebp = ecx;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), 0x0u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x18u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x66de5cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp.wrapping_add(0xacu32));
                esi = ebp.wrapping_add(0xacu32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = ebp.wrapping_add(0x194u32);
                ebx = 0x9u32;
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
                edi = ebp.wrapping_add(0x9c0u32);
                ebx = 0xau32;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                m.w32(esi.wrapping_add(0xc0u32), 0xff000000u32);
                { let t = m.r32(eax.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0x20u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                esi = 0x1u32;
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(0x6b2ca0u32);
                fl = lift::sub(esi, 0x9u32, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_mul(4).wrapping_add(0x69c9d4u32));
                pc = 7; continue;
            }
            7 => {
                ecx = esi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::sub(ecx, esi, 32); fl = f;
                ecx = r;
                edx = esi.wrapping_add(ecx.wrapping_mul(4));
                ecx = ebp.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0xau32, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c54a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c54b0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax;
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x18u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x28u32), 0xffffffffu32);
                m.w16(ebp.wrapping_add(0xb00u32), ((ecx & 0xffff)) as u16);
                ecx = esp.wrapping_add(0x30u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
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
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A6180` tavern_side_menu_render_buttons_to_cache
pub fn f_005a6180(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                esi = edi.wrapping_add(0xacu32);
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esi);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(eax).wrapping_add(0xb0cu32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                ecx = esi;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                eax = esp;
                edx = m.r32(esi);
                m.w32(eax, ebp);
                m.w32(eax.wrapping_add(0x4u32), ebx);
                m.w32(eax.wrapping_add(0x8u32), ecx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
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
                pc = 4; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0xau32, 32).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(edi.wrapping_add(0x94u32), eax);
                m.w8(edi.wrapping_add(0xa4u32), ((ecx & 0xff)) as u8);
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

/// `0X005A62C0` tavern_side_menu_draw_button_labels
pub fn f_005a62c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = esp.wrapping_add(0xcu32);
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
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0xb08u32));
                m.w32(esp.wrapping_add(0x10u32), edx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xb00u32)))) & 0xffff);
                ecx = eax;
                eax = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::imul(ecx, edx); fl = f;
                ecx = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(edi);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax;
                ecx = ebp;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(ebx).wrapping_add(0xb0cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x14u32)), ebp, 32); fl = f;
                m.w32(esp.wrapping_add(0x14u32), r);
                pc = 3; continue;
            }
            3 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebx, 0xau32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
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

/// `0X005A6370` tavern_side_menu_on_show
pub fn f_005a6370(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
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
                m.w8(edi.wrapping_add(0xb16u32), (0xffu32) as u8);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0x2cu32));
                ebp = m.r32(edi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), 0x64u32);
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi.wrapping_add(0x14u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(edi.wrapping_add(0xb08u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xb00u32)))) & 0xffff);
                ecx = eax;
                eax = m.r32(edi.wrapping_add(0x30u32));
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::imul(ecx, edx); fl = f;
                ecx = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esi;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = eax;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xb17u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebx = edi.wrapping_add(0x9c0u32);
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ebp).wrapping_add(0xb0cu32)))) & 0xff);
                edx = m.r32(esi);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
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
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                pc = 5; continue;
            }
            4 => {
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(ebx, 0x20u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(ebp, 0xau32, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(edi.wrapping_add(0xb17u32), (0x1u32) as u8);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                ecx = edi;
                m.w16(edi.wrapping_add(0xb1cu32), (0xffffu32) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_add(0x4);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(edi.wrapping_add(0xb18u32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(0x6e5574u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w8(edi.wrapping_add(0xb19u32), (0x0u32) as u8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A6530` tavern_side_menu_show_buttons
pub fn f_005a6530(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                esi = ebx.wrapping_add(0xacu32);
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(edi).wrapping_add(0xb0cu32)))) & 0xff);
                ecx = esi;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
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
                pc = 4; continue;
            }
            3 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, 0xau32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 5; continue;
            }
            5 => {
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

/// `0X005A6590` tavern_side_menu_on_hide
pub fn f_005a6590(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x440ed0, ecx, &a); }
                esi = edi.wrapping_add(0xacu32);
                ebx = 0xau32;
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
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
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x466b40, ecx, &a); }
                m.w8(edi.wrapping_add(0xb16u32), (0xffu32) as u8);
                ecx = m.r32(0x6e5574u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e5574u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 4; continue;
            }
            4 => {
                ecx = esp.wrapping_add(0xcu32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ecx = m.r32(0x6dfc14u32);
                edx = m.r32(edi.wrapping_add(0xb04u32));
                eax = esp.wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x10u32), 0x52u32);
                m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                m.w32(esp.wrapping_add(0x20u32), 0xau32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(edi.wrapping_add(0xb19u32), (0x1u32) as u8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A66C0` tavern_side_menu_update
pub fn f_005a66c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = m.r32(0x6e5574u32);
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
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(esi.wrapping_add(0xb18u32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e5574u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 58; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb16u32)))) & 0xff);
                edi = esi.wrapping_add(0xacu32);
                ecx = edi;
                m.w8(esp.wrapping_add(0x14u32), ((edx & 0xff)) as u8);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb18u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                eax = m.r32(0x6cc7e0u32);
                m.w8(eax.wrapping_add(0xfcu32), (0x0u32) as u8);
                m.w16(eax.wrapping_add(0xecu32), (0xe1u32) as u16);
                m.w8(esi.wrapping_add(0xb18u32), (0x0u32) as u8);
                pc = 9; continue;
            }
            9 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb16u32)))) & 0xff);
                fl = lift::sub(ebx, ecx, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w8(esi.wrapping_add(0xb16u32), ((ebx & 0xff)) as u8);
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ecx, 0xe8u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebx, 0xau32, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 12; continue;
            }
            12 => {
                pc = 16; continue;
            }
            13 => {
                ebp = edi;
                m.w32(esp.wrapping_add(0x10u32), 0xau32);
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if !fl.zf { pc = 14; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w8(esi.wrapping_add(0xb16u32), ((ebx & 0xff)) as u8);
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb16u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0xau32, 8).1;
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb16u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x14u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6e5574u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5ced00, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5a7a60, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5a7ba0, ecx, &a); }
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(0x6e5574u32);
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb19u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 32; continue; }
                pc = 22; continue;
            }
            22 => {
                eax = esi.wrapping_add(0xb0cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb16u32)))) & 0xff);
                esp = esp.wrapping_add(0xc);
                fl = lift::sub((eax & 0xff), 0xau32, 8).1;
                if !fl.cf { pc = 25; continue; }
                pc = 23; continue;
            }
            23 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub((eax & 0xff), 0x4u32, 8).1;
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                fl = lift::logic(edx & ecx, 32);
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w8(eax.wrapping_add(esi).wrapping_add(0xb0cu32), (0x1u32) as u8);
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(0x6dfc14u32);
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xb08u32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5a6e60, ecx, &a); }
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                ebp = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ebx = m.r32(esi.wrapping_add(0x18u32));
                eax = ecx;
                m.w32(esp.wrapping_add(0x20u32), 0x64u32);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xb00u32)))) & 0xffff);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ecx = edx;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(esi.wrapping_add(0xb08u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x30u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                let (r, f) = lift::sar(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 26; continue;
            }
            26 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(ebp).wrapping_add(0xb0cu32)))) & 0xff);
                edx = m.r32(edi);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ecx = edi;
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x1cu32)), ebx, 32); fl = f;
                m.w32(esp.wrapping_add(0x1cu32), r);
                pc = 29; continue;
            }
            28 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 29; continue;
            }
            29 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebp, 0xau32, 32).1;
                if (fl.sf != fl.of) { pc = 26; continue; }
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0xb08u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x24u32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x34u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            32 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x7u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x7u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 34; continue;
            }
            34 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x8u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 36; continue;
            }
            36 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                ecx = m.r32(0x6cc7e0u32);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                edx = m.r32(ecx.wrapping_add(0xe8u32));
                eax = m.r32(ecx.wrapping_add(0xd0u32));
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                edx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0xcu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                if !fl.zf { pc = 39; continue; }
                pc = 37; continue;
            }
            37 => {
                { let v = lift::fld64(m, 0x678498u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 42; continue; }
                pc = 38; continue;
            }
            38 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x9u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 41; continue;
            }
            39 => {
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 42; continue; }
                pc = 40; continue;
            }
            40 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x9u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xcu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 41; continue;
            }
            41 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x9u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                pc = 42; continue;
            }
            42 => {
                eax = m.r32(ecx.wrapping_add(0xe8u32));
                edx = m.r32(ecx.wrapping_add(0xd0u32));
                eax = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x9u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 45; continue; }
                pc = 43; continue;
            }
            43 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                ecx = m.r32(0x6cc7e0u32);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xcu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xcu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                pc = 45; continue;
            }
            45 => {
                edx = m.r32(ecx.wrapping_add(0xe8u32));
                eax = m.r32(ecx.wrapping_add(0xd0u32));
                edx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0xbu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 48; continue; }
                pc = 46; continue;
            }
            46 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                ecx = m.r32(0x6cc7e0u32);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677d28u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 48; continue; }
                pc = 47; continue;
            }
            47 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xbu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xbu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                pc = 48; continue;
            }
            48 => {
                eax = m.r32(ecx.wrapping_add(0xe8u32));
                edx = m.r32(ecx.wrapping_add(0xd0u32));
                esi = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5u32)))) & 0xff);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = eax & edx; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::logic(edx & eax, 32);
                if fl.zf { pc = 51; continue; }
                pc = 49; continue;
            }
            49 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                ecx = m.r32(0x6cc7e0u32);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 51; continue; }
                pc = 50; continue;
            }
            50 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xdu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xdu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                pc = 51; continue;
            }
            51 => {
                eax = m.r32(ecx.wrapping_add(0xe8u32));
                edx = m.r32(ecx.wrapping_add(0xd0u32));
                esi = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xdu32)))) & 0xff);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::logic(edx & eax, 32);
                if fl.zf { pc = 55; continue; }
                pc = 52; continue;
            }
            52 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 55; continue; }
                pc = 53; continue;
            }
            53 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                ecx = m.r32(0x6cc7e0u32);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 55; continue; }
                pc = 54; continue;
            }
            54 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                pc = 55; continue;
            }
            55 => {
                eax = m.r32(ecx.wrapping_add(0xe8u32));
                ecx = m.r32(ecx.wrapping_add(0xd0u32));
                edx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 58; continue; }
                pc = 56; continue;
            }
            56 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x66db88u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 58; continue; }
                pc = 57; continue;
            }
            57 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 58; continue;
            }
            58 => {
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
            _ => unreachable!(),
        }
    }
}

/// `0X005A6DA0` tavern_side_menu_open
pub fn f_005a6da0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(eax.wrapping_add(0xc324u32));
                edx = m.r32(esi.wrapping_add(0x94u32));
                m.w32(esi.wrapping_add(0xb04u32), ecx);
                fl = lift::sub(edx, m.r32(0x66d4f8u32), 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5a6e60, ecx, &a); }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x8u32));
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x441180, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A6E20` tavern_side_menu_clear_selection_and_invalidate
pub fn f_005a6e20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ebx.wrapping_add(0xacu32);
                edi = 0xau32;
                pc = 1; continue;
            }
            1 => {
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
                m.w16(ebx.wrapping_add(0xb1cu32), (0xffffu32) as u16);
                m.w8(ebx.wrapping_add(0xb16u32), (0xffu32) as u8);
                m.w8(ebx.wrapping_add(0xb18u32), (0x1u32) as u8);
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

/// `0X005A7320` tavern_side_menu_get_current_letter_slot
pub fn f_005a7320(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xb1cu32)))) & 0xffff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, 0x8u32, 32); fl = f;
                eax = r;
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = 0x8u32;
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A74B0` tavern_side_menu_find_next_offer_letter
pub fn f_005a74b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                ebp = ecx;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb1cu32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                if fl.cf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(0x6dfc14u32);
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = eax;
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if !fl.cf { pc = 9; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = ebx;
                ecx = 0x6dd730u32;
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4f38b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(esi, 0x8u32, 32); fl = f;
                esi = r;
                ecx = 0x6dd73cu32;
                edx = m.r32(esi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x8u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x6u32))), 0x1bu32, 16).1;
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi);
                eax = m.r32(eax.wrapping_add(0xcu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xcu32)))) & 0xff);
                ecx = m.r32(0x6dfc14u32);
                eax = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(edi, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                pc = 8; continue;
            }
            7 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 8; continue;
            }
            8 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = eax;
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if fl.cf { pc = 2; continue; }
                pc = 9; continue;
            }
            9 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | (((ebx & 0xffff)) & 0xffff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            10 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xffff) | (((ecx & 0xffff)) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 13; continue;
            }
            12 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 13; continue;
            }
            13 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = eax;
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub((edi & 0xffff), (eax & 0xffff), 16).1;
                if !fl.cf { pc = 24; continue; }
                pc = 14; continue;
            }
            14 => {
                esi = edi;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                edx = (edx & !0xffff) | (((eax & 0xffff)) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(0x6dd730u32);
                ebx = esi;
                let (r, f) = lift::shl(ebx, 0x4u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                pc = 17; continue;
            }
            16 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(ebx.wrapping_add(0x8u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd73cu32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x6u32))), 0x1bu32, 16).1;
                if !fl.zf { pc = 20; continue; }
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(ebx.wrapping_add(0x8u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0xcu32)))) & 0xff);
                edx = m.r32(eax.wrapping_add(0xcu32));
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                ecx = m.r32(0x6dfc14u32);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 27; continue; }
                pc = 19; continue;
            }
            19 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 27; continue; }
                pc = 20; continue;
            }
            20 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(esi, 0x4u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                pc = 23; continue;
            }
            22 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 23; continue;
            }
            23 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = eax;
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub((edi & 0xffff), (eax & 0xffff), 16).1;
                if fl.cf { pc = 14; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(0x6dfc14u32);
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = eax;
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub((edi & 0xffff), (eax & 0xffff), 16).1;
                if !fl.cf { pc = 37; continue; }
                pc = 25; continue;
            }
            25 => {
                esi = edi;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                edx = (edx & !0xffff) | (((eax & 0xffff)) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 29; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(0x6dd730u32);
                ebx = esi;
                let (r, f) = lift::shl(ebx, 0x4u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                pc = 30; continue;
            }
            27 => {
                fl = lift::sub((edi & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if !fl.cf { pc = 24; continue; }
                pc = 28; continue;
            }
            28 => {
                eax = (eax & !0xffff) | (((edi & 0xffff)) & 0xffff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            29 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(ebx.wrapping_add(0x8u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd73cu32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x6u32))), 0x1bu32, 16).1;
                if !fl.zf { pc = 33; continue; }
                pc = 31; continue;
            }
            31 => {
                edx = m.r32(ebx.wrapping_add(0x8u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0xcu32)))) & 0xff);
                edx = m.r32(eax.wrapping_add(0xcu32));
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                ecx = m.r32(0x6dfc14u32);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 37; continue; }
                pc = 32; continue;
            }
            32 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 37; continue; }
                pc = 33; continue;
            }
            33 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(esi, 0x4u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                pc = 36; continue;
            }
            35 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 36; continue;
            }
            36 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = eax;
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub((edi & 0xffff), (eax & 0xffff), 16).1;
                if fl.cf { pc = 25; continue; }
                pc = 37; continue;
            }
            37 => {
                fl = lift::sub((edi & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xb1cu32))), 16).1;
                if fl.zf { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                eax = (eax & !0xffff) | (((edi & 0xffff)) & 0xffff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            39 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xffff) | 0xffffu32; fl = lift::logic(r, 16);
                eax = (eax & !0xffff) | ((r) & 0xffff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A77A0` tavern_side_menu_show_next_offer_letter
pub fn f_005a77a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                ebp = ecx;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb1cu32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                if fl.cf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(0x6dfc14u32);
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = eax;
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if !fl.cf { pc = 33; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = ebx;
                ecx = 0x6dd730u32;
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4f38b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(esi, 0x8u32, 32); fl = f;
                esi = r;
                ecx = 0x6dd73cu32;
                edx = m.r32(esi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x8u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x6u32))), 0x1bu32, 16).1;
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi);
                eax = m.r32(eax.wrapping_add(0xcu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xcu32)))) & 0xff);
                ecx = m.r32(0x6dfc14u32);
                eax = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 31; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 31; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(edi, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                pc = 8; continue;
            }
            7 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 8; continue;
            }
            8 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = eax;
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if fl.cf { pc = 2; continue; }
                pc = 9; continue;
            }
            9 => {
                pc = 31; continue;
            }
            10 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xffff) | (((ecx & 0xffff)) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 13; continue;
            }
            12 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 13; continue;
            }
            13 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = eax;
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if !fl.cf { pc = 23; continue; }
                pc = 14; continue;
            }
            14 => {
                edi = ebx;
                ecx = 0x6dd730u32;
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4f38b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(esi, 0x8u32, 32); fl = f;
                esi = r;
                ecx = 0x6dd73cu32;
                edx = m.r32(esi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x8u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x6u32))), 0x1bu32, 16).1;
                if !fl.zf { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = m.r32(esi);
                eax = m.r32(eax.wrapping_add(0xcu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xcu32)))) & 0xff);
                ecx = m.r32(0x6dfc14u32);
                eax = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 16; continue;
            }
            16 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 22; continue; }
                pc = 17; continue;
            }
            17 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(edi, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                pc = 20; continue;
            }
            19 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 20; continue;
            }
            20 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = eax;
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if fl.cf { pc = 14; continue; }
                pc = 21; continue;
            }
            21 => {
                pc = 23; continue;
            }
            22 => {
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if fl.cf { pc = 32; continue; }
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(0x6dfc14u32);
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                esi = eax;
                fl = lift::sub((esi & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if !fl.cf { pc = 29; continue; }
                pc = 24; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5a7350, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 29; continue; }
                pc = 25; continue;
            }
            25 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(esi, 0x4u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                pc = 28; continue;
            }
            27 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 28; continue;
            }
            28 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xb04u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d7900, ecx, &a); }
                esp = esp.wrapping_add(8);
                esi = eax;
                fl = lift::sub((esi & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if fl.cf { pc = 24; continue; }
                pc = 29; continue;
            }
            29 => {
                fl = lift::sub((esi & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xb1cu32))), 16).1;
                ebx = esi;
                if !fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                ebx = 0xffffu32;
                pc = 31; continue;
            }
            31 => {
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(0x6dd736u32)), 16).1;
                if !fl.cf { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xb16u32)))) & 0xff);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5a7a60, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w16(ebp.wrapping_add(0xb1cu32), ((ebx & 0xffff)) as u16);
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xb16u32)))) & 0xff);
                ecx = m.r32(0x6e5574u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5ced00, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = ebp;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x5a7ba0, ecx, &a); }
            }
            33 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A7A60` tavern_side_menu_send_leave_page_operation
pub fn f_005a7a60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ebx;
                esi = ecx;
                let r = edi & 0xffu32; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0xcu32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xb04u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub((ebx & 0xff), 0x3u32, 8).1;
                m.w32(esp.wrapping_add(0xcu32), 0x52u32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                m.w32(esp.wrapping_add(0x1cu32), edi);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esi.wrapping_add(0xb04u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = edx.wrapping_add(ecx.wrapping_mul(8));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5269a0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x10u32)), eax, 32); fl = f;
                m.w32(esp.wrapping_add(0x10u32), r);
                pc = 13; continue;
            }
            3 => {
                fl = lift::sub((ebx & 0xff), 0x8u32, 8).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esi.wrapping_add(0xb04u32));
                edx = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = edx.wrapping_add(ecx.wrapping_mul(8));
                { let a = []; eax = h.call_at(esp, m, 0x5261d0, ecx, &a); }
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x10u32)), eax, 32); fl = f;
                m.w32(esp.wrapping_add(0x10u32), r);
                pc = 13; continue;
            }
            5 => {
                fl = lift::sub((ebx & 0xff), 0x6u32, 8).1;
                if fl.zf { pc = 12; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub((ebx & 0xff), 0x7u32, 8).1;
                if fl.zf { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub((ebx & 0xff), 0x9u32, 8).1;
                if !fl.zf { pc = 13; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xb1cu32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 11; continue;
            }
            10 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(eax.wrapping_add(0x8u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x8u32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                eax = m.r32(eax.wrapping_add(0x8u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xcu32)))) & 0xff);
                m.w16(esi.wrapping_add(0xb1cu32), (0xffffu32) as u16);
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let r = ecx | edx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 13; continue;
            }
            12 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 13; continue;
            }
            13 => {
                eax = esp.wrapping_add(0xcu32);
                ecx = 0x6df2f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            14 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A7BA0` tavern_side_menu_send_page_operation
pub fn f_005a7ba0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                ecx = esp.wrapping_add(0x4u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(0x6dfc14u32);
                ecx = m.r32(esi.wrapping_add(0xb04u32));
                m.w32(esp.wrapping_add(0xcu32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb16u32)))) & 0xff);
                edx = eax;
                m.w32(esp.wrapping_add(0x4u32), 0x52u32);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((eax & 0xff), 0x3u32, 8).1;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                m.w32(esp.wrapping_add(0x14u32), edx);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x8u32), eax);
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0xb04u32));
                edx = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = edx.wrapping_add(ecx.wrapping_mul(8));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5269a0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x8u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x4u32);
                m.w32(esp.wrapping_add(0x8u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                fl = lift::sub((eax & 0xff), 0x8u32, 8).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x8u32), eax);
                eax = m.r32(esi.wrapping_add(0xb04u32));
                edx = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = edx.wrapping_add(ecx.wrapping_mul(8));
                { let a = []; eax = h.call_at(esp, m, 0x5261d0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x8u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x4u32);
                m.w32(esp.wrapping_add(0x8u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            4 => {
                fl = lift::sub((eax & 0xff), 0x6u32, 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub((eax & 0xff), 0x7u32, 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub((eax & 0xff), 0x9u32, 8).1;
                if !fl.zf { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xb1cu32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6dd730u32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 10; continue;
            }
            9 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(eax.wrapping_add(0x8u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x8u32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x8u32), ecx);
                eax = m.r32(eax.wrapping_add(0x8u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xcu32)))) & 0xff);
                eax = esp.wrapping_add(0x4u32);
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let r = ecx | edx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            11 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x8u32), eax);
                pc = 12; continue;
            }
            12 => {
                eax = esp.wrapping_add(0x4u32);
                ecx = 0x6df2f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x4786f0 => f_004786f0(m, h, ecx, esp),
        0x4787d0 => f_004787d0(m, h, ecx, esp),
        0x4788b0 => f_004788b0(m, h, ecx, esp),
        0x478920 => f_00478920(m, h, ecx, esp),
        0x478c90 => f_00478c90(m, h, ecx, esp),
        0x478e10 => f_00478e10(m, h, ecx, esp),
        0x479150 => f_00479150(m, h, ecx, esp),
        0x4791f0 => f_004791f0(m, h, ecx, esp),
        0x4792e0 => f_004792e0(m, h, ecx, esp),
        0x479390 => f_00479390(m, h, ecx, esp),
        0x479550 => f_00479550(m, h, ecx, esp),
        0x479f50 => f_00479f50(m, h, ecx, esp),
        0x47b270 => f_0047b270(m, h, ecx, esp),
        0x47b830 => f_0047b830(m, h, ecx, esp),
        0x47c120 => f_0047c120(m, h, ecx, esp),
        0x47c2c0 => f_0047c2c0(m, h, ecx, esp),
        0x47c4a0 => f_0047c4a0(m, h, ecx, esp),
        0x47c520 => f_0047c520(m, h, ecx, esp),
        0x47c820 => f_0047c820(m, h, ecx, esp),
        0x47c9b0 => f_0047c9b0(m, h, ecx, esp),
        0x47ccc0 => f_0047ccc0(m, h, ecx, esp),
        0x47d260 => f_0047d260(m, h, ecx, esp),
        0x47d2f0 => f_0047d2f0(m, h, ecx, esp),
        0x47d4d0 => f_0047d4d0(m, h, ecx, esp),
        0x47d520 => f_0047d520(m, h, ecx, esp),
        0x47d570 => f_0047d570(m, h, ecx, esp),
        0x47d630 => f_0047d630(m, h, ecx, esp),
        0x47daa0 => f_0047daa0(m, h, ecx, esp),
        0x47db50 => f_0047db50(m, h, ecx, esp),
        0x4869a0 => f_004869a0(m, h, ecx, esp),
        0x4958f0 => f_004958f0(m, h, ecx, esp),
        0x495b30 => f_00495b30(m, h, ecx, esp),
        0x495c10 => f_00495c10(m, h, ecx, esp),
        0x495e90 => f_00495e90(m, h, ecx, esp),
        0x495ff0 => f_00495ff0(m, h, ecx, esp),
        0x4961c0 => f_004961c0(m, h, ecx, esp),
        0x496290 => f_00496290(m, h, ecx, esp),
        0x496340 => f_00496340(m, h, ecx, esp),
        0x4963d0 => f_004963d0(m, h, ecx, esp),
        0x496560 => f_00496560(m, h, ecx, esp),
        0x496880 => f_00496880(m, h, ecx, esp),
        0x496a00 => f_00496a00(m, h, ecx, esp),
        0x5a5eb0 => f_005a5eb0(m, h, ecx, esp),
        0x5a6030 => f_005a6030(m, h, ecx, esp),
        0x5a6180 => f_005a6180(m, h, ecx, esp),
        0x5a62c0 => f_005a62c0(m, h, ecx, esp),
        0x5a6370 => f_005a6370(m, h, ecx, esp),
        0x5a6530 => f_005a6530(m, h, ecx, esp),
        0x5a6590 => f_005a6590(m, h, ecx, esp),
        0x5a66c0 => f_005a66c0(m, h, ecx, esp),
        0x5a6da0 => f_005a6da0(m, h, ecx, esp),
        0x5a6e20 => f_005a6e20(m, h, ecx, esp),
        0x5a7320 => f_005a7320(m, h, ecx, esp),
        0x5a74b0 => f_005a74b0(m, h, ecx, esp),
        0x5a77a0 => f_005a77a0(m, h, ecx, esp),
        0x5a7a60 => f_005a7a60(m, h, ecx, esp),
        0x5a7ba0 => f_005a7ba0(m, h, ecx, esp),
        _ => return None,
    }))
}
