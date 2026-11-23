//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X004726B0` ship_list_window_ctor_default
pub fn f_004726b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65f56eu32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6b9870u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0xcu32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x41c360, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = edi ^ edi; fl = lift::logic(r, 32);
    edi = r;
    ecx = esi.wrapping_add(0xd0u32);
    m.w32(esp.wrapping_add(0x14u32), edi);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = esi.wrapping_add(0x484u32);
    m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = esi.wrapping_add(0x838u32);
    m.w8(esp.wrapping_add(0x14u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a440, ecx, &a); }
    ecx = esi.wrapping_add(0x874u32);
    m.w8(esp.wrapping_add(0x14u32), (0x3u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a440, ecx, &a); }
    ecx = esi.wrapping_add(0x910u32);
    m.w8(esp.wrapping_add(0x14u32), (0x4u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = esi.wrapping_add(0xa04u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x28u32), (0x5u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0xbd8u32);
    m.w8(esp.wrapping_add(0x14u32), (0x6u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0xcd4u32);
    m.w8(esp.wrapping_add(0x14u32), (0x7u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0xdbcu32);
    m.w8(esp.wrapping_add(0x14u32), (0x8u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x478660u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x478590u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    edx = esi.wrapping_add(0xeacu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x28u32), (0x9u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi;
    m.w8(esp.wrapping_add(0x14u32), (0xau32) as u8);
    m.w32(esi, 0x66ecf8u32);
    m.w32(esi.wrapping_add(0x8f4u32), edi);
    m.w32(esi.wrapping_add(0xbd4u32), edi);
    m.w32(esi.wrapping_add(0x8fcu32), edi);
    { let a = []; eax = h.call_at(esp, m, 0x472a10, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    eax = esi;
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X004727E0` ship_list_window_ctor_named
pub fn f_004727e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65f626u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(esp.wrapping_add(0x14u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0xcu32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x41c360, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = edi ^ edi; fl = lift::logic(r, 32);
    edi = r;
    ecx = esi.wrapping_add(0xd0u32);
    m.w32(esp.wrapping_add(0x14u32), edi);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = esi.wrapping_add(0x484u32);
    m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = esi.wrapping_add(0x838u32);
    m.w8(esp.wrapping_add(0x14u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a440, ecx, &a); }
    ecx = esi.wrapping_add(0x874u32);
    m.w8(esp.wrapping_add(0x14u32), (0x3u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a440, ecx, &a); }
    ecx = esi.wrapping_add(0x910u32);
    m.w8(esp.wrapping_add(0x14u32), (0x4u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = esi.wrapping_add(0xa04u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x28u32), (0x5u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0xbd8u32);
    m.w8(esp.wrapping_add(0x14u32), (0x6u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0xcd4u32);
    m.w8(esp.wrapping_add(0x14u32), (0x7u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0xdbcu32);
    m.w8(esp.wrapping_add(0x14u32), (0x8u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x478660u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x478590u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    edx = esi.wrapping_add(0xeacu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x28u32), (0x9u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi;
    m.w8(esp.wrapping_add(0x14u32), (0xau32) as u8);
    m.w32(esi, 0x66ecf8u32);
    m.w32(esi.wrapping_add(0x8f4u32), edi);
    m.w32(esi.wrapping_add(0xbd4u32), edi);
    m.w32(esi.wrapping_add(0x8fcu32), edi);
    { let a = []; eax = h.call_at(esp, m, 0x472a10, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    eax = esi;
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X00472910` ship_list_window_dtor
pub fn f_00472910(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65f6c3u32);
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
    m.w32(esi, 0x66ecf8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x478660u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    eax = esi.wrapping_add(0xeacu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x20u32), 0x9u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    ecx = esi.wrapping_add(0xdbcu32);
    m.w8(esp.wrapping_add(0x10u32), (0x8u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0xcd4u32);
    m.w8(esp.wrapping_add(0x10u32), (0x7u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0xbd8u32);
    m.w8(esp.wrapping_add(0x10u32), (0x6u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = esi.wrapping_add(0xa04u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x20u32), (0x5u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    ecx = esi.wrapping_add(0x910u32);
    m.w8(esp.wrapping_add(0x10u32), (0x4u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x874u32);
    m.w8(esp.wrapping_add(0x10u32), (0x3u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a480, ecx, &a); }
    ecx = esi.wrapping_add(0x838u32);
    m.w8(esp.wrapping_add(0x10u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a480, ecx, &a); }
    ecx = esi.wrapping_add(0x484u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430fe0, ecx, &a); }
    ecx = esi.wrapping_add(0xd0u32);
    m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430fe0, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x41c3a0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X00472A10` ship_list_window_reset
pub fn f_00472a10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = esi.wrapping_add(0x910u32);
                m.w32(esi.wrapping_add(0x904u32), ebx);
                m.w32(esi.wrapping_add(0x908u32), ebx);
                m.w32(esi.wrapping_add(0x90cu32), ebx);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esi.wrapping_add(0x900u32), ebx);
                m.w32(esi.wrapping_add(0xb4u32), ebx);
                m.w32(esi.wrapping_add(0xb8u32), ebx);
                m.w32(esi.wrapping_add(0xbcu32), ebx);
                m.w32(esi.wrapping_add(0xc0u32), ebx);
                m.w32(esi.wrapping_add(0xb0u32), ebx);
                m.w32(esi.wrapping_add(0xa8u32), ebx);
                m.w32(esi.wrapping_add(0xacu32), ebx);
                m.w32(esi.wrapping_add(0xc4u32), ebx);
                m.w32(esi.wrapping_add(0xc8u32), ebx);
                m.w32(esi.wrapping_add(0xccu32), ebx);
                m.w32(esi.wrapping_add(0x8f8u32), ebx);
                m.w8(esi.wrapping_add(0xcc4u32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0xcccu32), ((ebx & 0xff)) as u8);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(eax.wrapping_mul(4).wrapping_add(0x66e930u32)), ebx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x5u32, 32).1;
                if fl.cf { pc = 1; continue; }
                pc = 3; continue;
            }
            3 => {
                pc = 5; continue;
            }
            4 => {
                m.w8(esi.wrapping_add(0xcccu32), ((eax & 0xff)) as u8);
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0xcd4u32));
                ecx = esi.wrapping_add(0xcd4u32);
                m.w32(esi.wrapping_add(0xcc0u32), ebx);
                m.w32(esi.wrapping_add(0xea8u32), 0xffffffffu32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0xdbcu32));
                ecx = esi.wrapping_add(0xdbcu32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0xeacu32));
                ecx = esi.wrapping_add(0xeacu32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0xfa4u32));
                ecx = esi.wrapping_add(0xfa4u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0xbd4u32));
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                eax = 0xffffu32;
                m.w32(esi.wrapping_add(0xbd4u32), ebx);
                m.w32(esi.wrapping_add(0x8e0u32), eax);
                m.w32(esi.wrapping_add(0x8e4u32), eax);
                eax = m.r32(esi.wrapping_add(0x8fcu32));
                m.w32(esi.wrapping_add(0x8d8u32), ebx);
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esi.wrapping_add(0x8d4u32), ebx);
                m.w32(esi.wrapping_add(0xcd0u32), ebx);
                m.w8(esi.wrapping_add(0x8dcu32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0xcc8u32), ebx);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                m.w32(esi.wrapping_add(0x8fcu32), ebx);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63a1b6, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x8f4u32), ebx);
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::sub(eax, ebx, 32).1;
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
                edx = m.r32(esi.wrapping_add(0xa04u32));
                ecx = esi.wrapping_add(0xa04u32);
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0xaecu32));
                ecx = esi.wrapping_add(0xaecu32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0xbd8u32));
                ecx = esi.wrapping_add(0xbd8u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi;
                m.w32(esi.wrapping_add(0x9f8u32), ebx);
                m.w32(esi.wrapping_add(0x9fcu32), ebx);
                m.w32(esi.wrapping_add(0xa00u32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x41c3e0, ecx, &a); }
                ecx = esi.wrapping_add(0xd0u32);
                m.w32(esi.wrapping_add(0x8f0u32), ebx);
                m.w32(esi.wrapping_add(0x8ecu32), ebx);
                m.w32(esi.wrapping_add(0x8e8u32), ebx);
                m.w32(esi.wrapping_add(0x8b4u32), 0xffffffffu32);
                m.w32(esi.wrapping_add(0x8b0u32), 0xffffffffu32);
                m.w32(esi.wrapping_add(0xea4u32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x431060, ecx, &a); }
                ecx = esi.wrapping_add(0x484u32);
                { let a = []; eax = h.call_at(esp, m, 0x431060, ecx, &a); }
                ecx = esi.wrapping_add(0x838u32);
                { let a = []; eax = h.call_at(esp, m, 0x40a500, ecx, &a); }
                ecx = esi.wrapping_add(0x874u32);
                { let a = []; eax = h.call_at(esp, m, 0x40a500, ecx, &a); }
                m.w8(esi.wrapping_add(0x48u32), ((ebx & 0xff)) as u8);
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

/// `0X00472C60` ship_list_window_load_layout
pub fn f_00472c60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65f6ddu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x34);
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
                m.w32(esp.wrapping_add(0x4cu32), 0x0u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esp.wrapping_add(0x58u32));
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9874u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x58u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x60u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9880u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x54u32));
                ebp = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b988cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b989cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x58u32), eax);
                eax = m.r32(esp.wrapping_add(0x54u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b98a8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x54u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b98b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b98c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x50u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esp.wrapping_add(0x54u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b98ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0x18u32), eax);
                edx = m.r32(esp.wrapping_add(0x54u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b98d8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b98e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                edi = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(esp.wrapping_add(0x54u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b98f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x54u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9900u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b990cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x3cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esp.wrapping_add(0x54u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9918u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x54u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9924u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9934u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(esp.wrapping_add(0x54u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9940u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x54u32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b994cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9958u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x4cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(esp.wrapping_add(0x54u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9964u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x58u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0xcd0u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = 0x6da820u32;
                m.w32(esi.wrapping_add(0xa8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = 0x6da820u32;
                m.w32(esi.wrapping_add(0xc4u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0xacu32), eax);
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9974u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi.wrapping_add(0x8c8u32);
                m.w32(esi.wrapping_add(0xb0u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esp.wrapping_add(0x54u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9980u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x5cu32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x60u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x8d4u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xd8u32));
                ecx = esi.wrapping_add(0xd8u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x5cu32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x60u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x48cu32));
                ecx = esi.wrapping_add(0x48cu32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x54u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b998cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi.wrapping_add(0x904u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = esi.wrapping_add(0x910u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de59u32))) & 0xff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x18u32));
                edx = esp.wrapping_add(0x54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x60u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xdbcu32));
                ecx = esi.wrapping_add(0xdbcu32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = esp.wrapping_add(0x54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x60u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xcd4u32));
                ecx = esi.wrapping_add(0xcd4u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edx = esp.wrapping_add(0x54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x60u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xa04u32));
                ecx = esi.wrapping_add(0xa04u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = esp.wrapping_add(0x54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x60u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xaecu32));
                ecx = esi.wrapping_add(0xaecu32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b999cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi.wrapping_add(0x9f8u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esp.wrapping_add(0x28u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(esi.wrapping_add(0xb4u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x30u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0xb8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0xbcu32), eax);
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                m.w32(esi.wrapping_add(0xc0u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0xc8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0xccu32), eax);
                eax = m.r32(esp.wrapping_add(0x54u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cc954u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b99acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x60u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x838u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x40a560, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x60u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x874u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x40a560, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esi.wrapping_add(0xacu32));
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                edx = esi.wrapping_add(0x8b8u32);
                ecx = m.r32(eax);
                m.w32(edx, ecx);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(edx.wrapping_add(0x4u32), ecx);
                ecx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(edx.wrapping_add(0x8u32), ecx);
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(edx.wrapping_add(0xcu32), eax);
                pc = 4; continue;
            }
            4 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(esi, 0xbd8u32, 32); fl = f;
                esi = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 5; continue;
            }
            5 => {
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x4cu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x54u32);
                m.w32(esp.wrapping_add(0x4cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x44u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x40);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00473220` ship_list_window_draw
pub fn f_00473220(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65f6efu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x30);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                edi = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 19; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 19; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                eax = esp.wrapping_add(0x20u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x41caa0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6998ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x58u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8dcu32)))) & 0xff);
                m.w32(esp.wrapping_add(0x48u32), 0x0u32);
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x20u32);
                ecx = esp.wrapping_add(0x58u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f60a, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x8e4u32));
                ecx = esp.wrapping_add(0x54u32);
                eax = m.r32(edx.wrapping_mul(4).wrapping_add(0x6dda00u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esp.wrapping_add(0x54u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dcf20u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ebp = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edi = eax;
                let (r, f) = lift::sar(edi, 0x1u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ebp, 32); fl = f;
                edi = r;
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
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                edx = m.r32(esi.wrapping_add(0x8d4u32));
                eax = m.r32(esi.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edx.wrapping_add(eax).wrapping_add(0x1u32);
                edx = edi.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x8d4u32));
                ebp = m.r32(esi.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x6cu32));
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                esp = esp.wrapping_add(0x24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x473650, ecx, &a); }
                edi = esi.wrapping_add(0x838u32);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ebx = m.r32(esp.wrapping_add(0x18u32));
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x3cu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                edx = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x54u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb820, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x47cu32));
                ebp = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esi.wrapping_add(0x864u32));
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_add(0x8);
                m.w32(esp.wrapping_add(0x18u32), ebp);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                ebx = m.r32(esi.wrapping_add(0x474u32));
                eax = m.r32(esi.wrapping_add(0x478u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.cf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(0xc);
                edx = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(edx, ecx);
                ecx = esi;
                m.w32(edx.wrapping_add(0x4u32), eax);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x473b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebp = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x478u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, ecx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if fl.cf { pc = 7; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 14; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4738c0, ecx, &a); }
                edi = esi.wrapping_add(0x874u32);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ebx = m.r32(esp.wrapping_add(0x18u32));
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x3cu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                edx = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x54u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb820, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x830u32));
                ebp = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esi.wrapping_add(0x864u32));
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_add(0x8);
                m.w32(esp.wrapping_add(0x18u32), ebp);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                ebx = m.r32(esi.wrapping_add(0x828u32));
                eax = m.r32(esi.wrapping_add(0x82cu32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.cf { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(0xc);
                edx = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(edx, ecx);
                ecx = esi;
                m.w32(edx.wrapping_add(0x4u32), eax);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4743f0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebp = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x82cu32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, ecx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if fl.cf { pc = 12; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = esp.wrapping_add(0x14u32);
                ecx = esi.wrapping_add(0x838u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x699990u32);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 18; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edi = m.r32(esi.wrapping_add(0x83cu32));
                eax = m.r32(0x6b2aa4u32);
                edx = m.r32(esi.wrapping_add(0x910u32));
                let (r, f) = lift::add(esi, 0x910u32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_add(0x8);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x70u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                pc = 18; continue;
            }
            18 => {
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x54u32);
                m.w32(esp.wrapping_add(0x48u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x3c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00473650` ship_list_window_draw_own_ships_header
pub fn f_00473650(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                eax = esp.wrapping_add(0xcu32);
                esi = edi.wrapping_add(0x838u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(edi.wrapping_add(0x860u32));
                esp = esp.wrapping_add(0x4);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(0x6998acu32);
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xcu32));
                edx = m.r32(0x6998f0u32);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xcu32));
                edx = m.r32(0x6998fcu32);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = esi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = m.r32(0x6cc3a4u32);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = esi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, m.r32(esp.wrapping_add(0xcu32)), 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                eax = m.r32(edi.wrapping_add(0xcc0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(edi.wrapping_add(0xc0u32));
                ebx = m.r32(eax.wrapping_add(0x4u32));
                pc = 3; continue;
            }
            2 => {
                ecx = m.r32(edi.wrapping_add(0xb4u32));
                ebx = m.r32(ecx.wrapping_add(0x4u32));
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                ecx = esi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, m.r32(esp.wrapping_add(0xcu32)), 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                eax = m.r32(edi.wrapping_add(0xcc0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(edi.wrapping_add(0xbcu32));
                edi = m.r32(eax.wrapping_add(0x4u32));
                pc = 6; continue;
            }
            5 => {
                ecx = m.r32(edi.wrapping_add(0xb8u32));
                edi = m.r32(ecx.wrapping_add(0x4u32));
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x24u32));
                edi = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
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

/// `0X004738C0` ship_list_window_draw_foreign_ships_header
pub fn f_004738c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = ecx;
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esi = edi.wrapping_add(0x874u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(edi.wrapping_add(0x89cu32));
                esp = esp.wrapping_add(0x4);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(0x6998acu32);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(0x6998f0u32);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(0x6998e4u32);
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = esi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = esp.wrapping_add(0x20u32);
                eax = m.r32(0x6cc3a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_add(0x10);
                ebx = edi.wrapping_add(0x838u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = ebx;
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, m.r32(esp.wrapping_add(0x10u32)), 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(edi.wrapping_add(0xcc0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(edi.wrapping_add(0xc0u32));
                ebp = m.r32(eax.wrapping_add(0x4u32));
                pc = 3; continue;
            }
            2 => {
                ecx = m.r32(edi.wrapping_add(0xb4u32));
                ebp = m.r32(ecx.wrapping_add(0x4u32));
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                ecx = ebx;
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(edi.wrapping_add(0xbcu32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                edx = m.r32(ecx.wrapping_add(0x4u32));
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edi = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
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

/// `0X004743F0` ship_list_window_draw_foreign_ship_row
pub fn f_004743f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x65f713u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x1c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64ffe6cau32);
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                ecx = m.r32(esi.wrapping_add(0x8f4u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                ebx = ebx.wrapping_add(ebx.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edi = esi.wrapping_add(0x874u32);
                esp = esp.wrapping_add(0x4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb430, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_add(0x10);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xcccu32)))) & 0xff);
                fl = lift::sub(m.r32(ecx.wrapping_mul(4).wrapping_add(0x66e930u32)), 0x2u32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi.wrapping_add(0x8ecu32));
                eax = m.r32(esi.wrapping_add(0x8e8u32));
                ecx = edx.wrapping_add(ebp).wrapping_add(0x1u32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.cf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi.wrapping_add(0x8f4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                edx = m.r32(0x6dd7a4u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x8u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(eax.wrapping_add(edx).wrapping_add(0x8u32))), 16).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edi = esi.wrapping_add(0x838u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                edx = eax.wrapping_add(ecx).wrapping_add(0xfffffffeu32);
                eax = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bb430, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                edx = eax.wrapping_add(ecx).wrapping_add(0xfffffffeu32);
                eax = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bb430, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                edx = eax.wrapping_add(ecx).wrapping_add(0xfffffffeu32);
                eax = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bb430, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x8b4u32));
                edi = m.r32(esi.wrapping_add(0x828u32));
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                edx = 0xff000000u32;
                fl = lift::sub(ecx, ebp, 32).1;
                m.w32(esp.wrapping_add(0x14u32), edx);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(0x66e104u32);
                m.w32(esp.wrapping_add(0x14u32), edx);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x900u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                edi = m.r32(esi.wrapping_add(0x8f4u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ebp = m.r32(0x66e108u32);
                edi = (edi & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                eax = m.r32(esi.wrapping_add(0x8f8u32));
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub((edi & 0xffff), u32::from(m.r16(eax)), 16).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = ebp;
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::add(eax, 0x2u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                if !fl.zf { pc = 8; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w32(esp.wrapping_add(0x14u32), edx);
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x4cu32));
                eax = m.r32(esp.wrapping_add(0x48u32));
                ecx = ebx.wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                edi = esi.wrapping_add(0x838u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x40u32), eax);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0xeu32)))) & 0xff);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0cb4u32));
                ecx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(0x6c7cccu32);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x40u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx))) & 0xff);
                let r = (ecx & 0xffff) ^ (ecx & 0xffff); fl = lift::logic(r, 16);
                ecx = (ecx & !0xffff) | ((r) & 0xffff);
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                m.w32(esp.wrapping_add(0x34u32), 0x0u32);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if !fl.cf { pc = 17; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 17; continue; }
                pc = 14; continue;
            }
            14 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(eax.wrapping_add(0xe8u32));
                fl = lift::sub(u32::from(m.r8(ecx)), 0x0u32, 8).1;
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(eax.wrapping_add(0xe4u32));
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b99f4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 19; continue;
            }
            16 => {
                ecx = m.r32(eax.wrapping_add(0xe4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 18; continue;
            }
            17 => {
                edx = m.r32(0x69c98cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 18; continue;
            }
            18 => {
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = edi;
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x44u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x495340, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(eax, 0x28u32, 32).1;
                if !fl.cf { pc = 22; continue; }
                pc = 20; continue;
            }
            20 => {
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x134u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub((ebp & 0xffff), 0x13u32, 16).1;
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub((ebp & 0xffff), 0xeu32, 16).1;
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = eax & ecx; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((ebp & 0xffff), 0xdu32, 16).1;
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = eax & edx; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub((ebp & 0xffff), 0xcu32, 16).1;
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                fl = lift::logic(ecx & eax, 32);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffff0000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x495340, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a00u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x58u32));
                esp = esp.wrapping_add(0x14);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, m.r32(esp.wrapping_add(0x48u32)), 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = edi;
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x44u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0xcc0u32));
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                if !fl.zf { pc = 25; continue; }
                pc = 23; continue;
            }
            23 => {
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(0x134u32))), 0x13u32, 16).1;
                if fl.zf { pc = 31; continue; }
                pc = 24; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4954c0, ecx, &a); }
                ecx = eax;
                eax = 0x51eb851fu32;
                { let r = u64::from(eax) * u64::from(ecx); eax = r as u32; edx = (r >> 32) as u32; fl.cf = edx != 0; fl.of = edx != 0; }
                let (r, f) = lift::shr(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a04u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 31; continue;
            }
            25 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4953c0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9a08u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(esp.wrapping_add(0x54u32));
                eax = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x42u32)))) & 0xffff);
                esp = esp.wrapping_add(0x1c);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                if !fl.cf { pc = 31; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(esi.wrapping_add(0xb0u32));
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 29; continue; }
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ebp = m.r32(edx.wrapping_add(0x8u32));
                edx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x28u32), edx);
                pc = 30; continue;
            }
            29 => {
                ebp = m.r32(esp.wrapping_add(0x24u32));
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xb0u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                ecx = eax.wrapping_add(ebp);
                eax = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x44u32), eax);
                eax = m.r32(edx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = m.r32(esp.wrapping_add(0x50u32));
                edx = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x40u32));
                eax = m.r32(esp.wrapping_add(0x68u32));
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_add(0x28);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x40u32), eax);
                pc = 31; continue;
            }
            31 => {
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x44u32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x495450, ecx, &a); }
                ecx = m.r32(0x6ce6d0u32);
                esp = esp.wrapping_add(0x4);
                edx = esp.wrapping_add(0x48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4901c0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                ecx = edi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp.wrapping_add(0x40u32));
                ecx = m.r32(esi.wrapping_add(0x8ecu32));
                edi = m.r32(esp.wrapping_add(0x3cu32));
                edx = m.r32(esi.wrapping_add(0x8f4u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x40u32), eax);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(0xc);
                ecx = esp;
                m.w32(ecx, eax);
                eax = m.r32(esp.wrapping_add(0x58u32));
                m.w32(ecx.wrapping_add(0x4u32), edx);
                m.w32(ecx.wrapping_add(0x8u32), eax);
                ecx = m.r32(0x6ce6d0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x490f80, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x34u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00474A20` ship_list_window_close
pub fn f_00474a20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(m.r32(esi.wrapping_add(0xa0u32)), ebx, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = []; eax = h.call_at(esp, m, 0x41ccf0, ecx, &a); }
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                edi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x910u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0xbd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esi.wrapping_add(0xa04u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xaecu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0xeacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esi.wrapping_add(0xfa4u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.cf || fl.zf) { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(ebp, eax, 32).1;
                if fl.cf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0xbd4u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(ecx, ebx, 32).1;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                ecx = esi.wrapping_add(0xd8u32);
                m.w32(esi.wrapping_add(0xbd4u32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x461480, ecx, &a); }
                ecx = esi.wrapping_add(0x48cu32);
                { let a = []; eax = h.call_at(esp, m, 0x461480, ecx, &a); }
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0x900u32), ebx);
                m.w32(esi.wrapping_add(0x8b0u32), eax);
                m.w32(esi.wrapping_add(0x8b4u32), eax);
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                m.w8(esi.wrapping_add(0x48u32), (0x0u32) as u8);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x8fcu32));
                m.w32(esi.wrapping_add(0x8f4u32), ebx);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 10; continue;
            }
            10 => {
                m.w32(esi.wrapping_add(0x8fcu32), ebx);
                m.w32(esi.wrapping_add(0x8f8u32), ebx);
                m.w32(esi.wrapping_add(0x8f0u32), ebx);
                m.w32(esi.wrapping_add(0x8ecu32), ebx);
                m.w32(esi.wrapping_add(0x8e8u32), ebx);
                pc = 11; continue;
            }
            11 => {
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

/// `0X00474B70` ship_list_window_refresh_ship_lists
pub fn f_00474b70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8dcu32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x1u32, 8).1;
                if !fl.zf { pc = 32; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esi.wrapping_add(0x8e8u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(edi, ebx, 32).1;
                m.w8(esp.wrapping_add(0x12u32), (0x0u32) as u8);
                if (fl.cf || fl.zf) { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi.wrapping_add(0x8f4u32));
                ebp = m.r32(0x6dd7a4u32);
                pc = 3; continue;
            }
            3 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ebp).wrapping_add(0x3cu32)))) & 0xffff);
                let r = eax & 0x400u32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, 0x400u32, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edi, 32).1;
                if fl.cf { pc = 3; continue; }
                pc = 5; continue;
            }
            5 => {
                pc = 7; continue;
            }
            6 => {
                m.w8(esp.wrapping_add(0x12u32), (0x1u32) as u8);
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x8ecu32));
                edx = m.r32(esi.wrapping_add(0x8f0u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(ecx, edi, 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 18; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0x8e8u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esi.wrapping_add(0x8ecu32), ebx);
                m.w32(esi.wrapping_add(0x8f0u32), ebx);
                if (fl.cf || fl.zf) { pc = 15; continue; }
                pc = 10; continue;
            }
            10 => {
                edi = m.r32(esi.wrapping_add(0x8e0u32));
                ecx = m.r32(esi.wrapping_add(0x8f4u32));
                pc = 11; continue;
            }
            11 => {
                ebp = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ebp)))) & 0xff);
                fl = lift::sub(ebx, edi, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(m.r32(esi.wrapping_add(0x8ecu32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0x8ecu32), r);
                pc = 14; continue;
            }
            13 => {
                let (r, f) = lift::add(m.r32(esi.wrapping_add(0x8f0u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0x8f0u32), r);
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0x8e8u32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(edx, eax, 32).1;
                if fl.cf { pc = 11; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x477c10, ecx, &a); }
                pc = 17; continue;
            }
            17 => {
                m.w8(esp.wrapping_add(0x12u32), (0x1u32) as u8);
                pc = 18; continue;
            }
            18 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xcc4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 31; continue; }
                pc = 19; continue;
            }
            19 => {
                ebx = m.r32(esi.wrapping_add(0x8ecu32));
                eax = m.r32(esi.wrapping_add(0x8e8u32));
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.cf { pc = 31; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(0x6dfc14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xeu32)))) & 0xffff);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(0x6dd894u32))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebp & 0xffff), 16).1;
                if !fl.cf { pc = 27; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(esi.wrapping_add(0x8f4u32));
                edi = m.r32(0x6dd7a4u32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ebx.wrapping_mul(2))))) & 0xffff);
                pc = 22; continue;
            }
            22 => {
                ecx = eax;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(0x50u32))), (edx & 0xffff), 16).1;
                if fl.zf { pc = 26; continue; }
                pc = 23; continue;
            }
            23 => {
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(0x52u32))), (edx & 0xffff), 16).1;
                if fl.zf { pc = 26; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebp & 0xffff), 16).1;
                if fl.cf { pc = 22; continue; }
                pc = 25; continue;
            }
            25 => {
                pc = 27; continue;
            }
            26 => {
                fl = lift::sub((eax & 0xffff), (ebp & 0xffff), 16).1;
                if fl.cf { pc = 30; continue; }
                pc = 27; continue;
            }
            27 => {
                edi = m.r32(esi.wrapping_add(0x8f4u32));
                eax = esi.wrapping_add(0x8e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ebx.wrapping_mul(2))))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x48fdf0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                m.w32(esi.wrapping_add(0x8f4u32), eax);
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                m.w8(esp.wrapping_add(0x12u32), (0x1u32) as u8);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0x8f0u32), eax);
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(esi.wrapping_add(0x8e8u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if fl.cf { pc = 20; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
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
            32 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 55; continue; }
                pc = 33; continue;
            }
            33 => {
                ecx = m.r32(esi.wrapping_add(0x8e0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = eax;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x18u32), ebp);
                m.w8(esp.wrapping_add(0x13u32), ((ebx & 0xff)) as u8);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xeu32)))) & 0xffff);
                edi = 0x17u32;
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 46; continue; }
                pc = 34; continue;
            }
            34 => {
                ebp = m.r32(0x6dd7a4u32);
                eax = edx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), (edi & 0xffff), 16).1;
                if fl.zf { pc = 44; continue; }
                pc = 35; continue;
            }
            35 => {
                ebp = m.r32(esi.wrapping_add(0x8e8u32));
                fl = lift::sub(ebx, ebp, 32).1;
                if (!fl.cf && !fl.zf) { pc = 42; continue; }
                pc = 36; continue;
            }
            36 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(ebp & ebp, 32);
                if (fl.cf || fl.zf) { pc = 42; continue; }
                pc = 37; continue;
            }
            37 => {
                edi = m.r32(esi.wrapping_add(0x8f4u32));
                pc = 38; continue;
            }
            38 => {
                fl = lift::sub(u32::from(m.r16(edi)), (edx & 0xffff), 16).1;
                if fl.zf { pc = 41; continue; }
                pc = 39; continue;
            }
            39 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(edi, 0x2u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ecx, ebp, 32).1;
                if fl.cf { pc = 38; continue; }
                pc = 40; continue;
            }
            40 => {
                pc = 42; continue;
            }
            41 => {
                fl = lift::sub(ecx, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 43; continue; }
                pc = 42; continue;
            }
            42 => {
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                pc = 43; continue;
            }
            43 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                edi = 0x17u32;
                pc = 44; continue;
            }
            44 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if fl.cf { pc = 34; continue; }
                pc = 45; continue;
            }
            45 => {
                ebp = m.r32(esp.wrapping_add(0x18u32));
                pc = 46; continue;
            }
            46 => {
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(0x8e8u32)), 32).1;
                if !fl.zf { pc = 48; continue; }
                pc = 47; continue;
            }
            47 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 115; continue; }
                pc = 48; continue;
            }
            48 => {
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 50; continue; }
                pc = 49; continue;
            }
            49 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 50; continue;
            }
            50 => {
                edx = ebx.wrapping_add(ebx);
                m.w32(esi.wrapping_add(0x8f4u32), 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                m.w32(esi.wrapping_add(0x8f4u32), eax);
                m.w32(esi.wrapping_add(0x8e8u32), ebx);
                m.w32(esi.wrapping_add(0x8ecu32), ebx);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xeu32)))) & 0xffff);
                esp = esp.wrapping_add(0x4);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                edx = eax;
                if !fl.cf { pc = 54; continue; }
                pc = 51; continue;
            }
            51 => {
                ebx = m.r32(0x6dd7a4u32);
                eax = ecx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), (edi & 0xffff), 16).1;
                if fl.zf { pc = 53; continue; }
                pc = 52; continue;
            }
            52 => {
                m.w16(edx, ((ecx & 0xffff)) as u16);
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                pc = 53; continue;
            }
            53 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if fl.cf { pc = 51; continue; }
                pc = 54; continue;
            }
            54 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            55 => {
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 115; continue; }
                pc = 56; continue;
            }
            56 => {
                eax = m.r32(esi.wrapping_add(0x8e0u32));
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xeu32)))) & 0xffff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w8(esp.wrapping_add(0x13u32), ((ebx & 0xff)) as u8);
                if !fl.cf { pc = 71; continue; }
                pc = 57; continue;
            }
            57 => {
                ecx = m.r32(esi.wrapping_add(0x8e4u32));
                m.w32(esp.wrapping_add(0x18u32), ecx);
                pc = 58; continue;
            }
            58 => {
                ebp = m.r32(0x6dd7a4u32);
                eax = edx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp.wrapping_add(0x18u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(ecx, edi, 32).1;
                if !fl.zf { pc = 69; continue; }
                pc = 59; continue;
            }
            59 => {
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), 0x4u32, 16).1;
                if !fl.cf { pc = 69; continue; }
                pc = 60; continue;
            }
            60 => {
                ebp = m.r32(esi.wrapping_add(0x8e8u32));
                fl = lift::sub(ebx, ebp, 32).1;
                if !fl.cf { pc = 67; continue; }
                pc = 61; continue;
            }
            61 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(ebp & ebp, 32);
                if (fl.cf || fl.zf) { pc = 67; continue; }
                pc = 62; continue;
            }
            62 => {
                edi = m.r32(esi.wrapping_add(0x8f4u32));
                pc = 63; continue;
            }
            63 => {
                fl = lift::sub(u32::from(m.r16(edi)), (edx & 0xffff), 16).1;
                if fl.zf { pc = 66; continue; }
                pc = 64; continue;
            }
            64 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(edi, 0x2u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ecx, ebp, 32).1;
                if fl.cf { pc = 63; continue; }
                pc = 65; continue;
            }
            65 => {
                pc = 67; continue;
            }
            66 => {
                fl = lift::sub(ecx, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 68; continue; }
                pc = 67; continue;
            }
            67 => {
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                pc = 68; continue;
            }
            68 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                pc = 69; continue;
            }
            69 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if fl.cf { pc = 58; continue; }
                pc = 70; continue;
            }
            70 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 71; continue;
            }
            71 => {
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(0x8ecu32)), 32).1;
                if fl.zf { pc = 73; continue; }
                pc = 72; continue;
            }
            72 => {
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                pc = 74; continue;
            }
            73 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 82; continue; }
                pc = 74; continue;
            }
            74 => {
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                m.w32(esi.wrapping_add(0x8e8u32), ebx);
                fl = lift::sub(eax, edi, 32).1;
                m.w32(esi.wrapping_add(0x8ecu32), edi);
                m.w32(esi.wrapping_add(0x8f0u32), edi);
                if fl.zf { pc = 76; continue; }
                pc = 75; continue;
            }
            75 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 76; continue;
            }
            76 => {
                edx = ebx.wrapping_add(ebx);
                m.w32(esi.wrapping_add(0x8f4u32), edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                m.w32(esi.wrapping_add(0x8f4u32), eax);
                edi = eax;
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_add(0x4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xeu32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 82; continue; }
                pc = 77; continue;
            }
            77 => {
                ebp = 0x4u32;
                pc = 78; continue;
            }
            78 => {
                edx = m.r32(0x6dd7a4u32);
                eax = ecx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x8e4u32));
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(edx, ebx, 32).1;
                if !fl.zf { pc = 81; continue; }
                pc = 79; continue;
            }
            79 => {
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), (ebp & 0xffff), 16).1;
                if !fl.cf { pc = 81; continue; }
                pc = 80; continue;
            }
            80 => {
                edx = m.r32(esi.wrapping_add(0x8ecu32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::add(edi, 0x2u32, 32); fl = f;
                edi = r;
                m.w32(esi.wrapping_add(0x8ecu32), edx);
                m.w16(edi.wrapping_add(0xfffffffeu32), ((ecx & 0xffff)) as u16);
                pc = 81; continue;
            }
            81 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if fl.cf { pc = 78; continue; }
                pc = 82; continue;
            }
            82 => {
                eax = m.r32(esi.wrapping_add(0x8d8u32));
                m.w8(esp.wrapping_add(0x12u32), (0x0u32) as u8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 114; continue; }
                pc = 83; continue;
            }
            83 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                edi = (edi & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(esp.wrapping_add(0x18u32), edi);
                if fl.sf { pc = 100; continue; }
                pc = 84; continue;
            }
            84 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0x8e0u32)), 32).1;
                if fl.zf { pc = 99; continue; }
                pc = 85; continue;
            }
            85 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xeu32)))) & 0xffff);
                fl = lift::sub((ebp & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 99; continue; }
                pc = 86; continue;
            }
            86 => {
                ebx = m.r32(esi.wrapping_add(0x8e4u32));
                pc = 87; continue;
            }
            87 => {
                ecx = m.r32(0x6dd7a4u32);
                eax = ebp;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(ecx, ebx, 32).1;
                if !fl.zf { pc = 98; continue; }
                pc = 88; continue;
            }
            88 => {
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), 0x4u32, 16).1;
                if !fl.cf { pc = 98; continue; }
                pc = 89; continue;
            }
            89 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::sub(edx, ecx, 32).1;
                if !fl.cf { pc = 96; continue; }
                pc = 90; continue;
            }
            90 => {
                edi = m.r32(esi.wrapping_add(0x8e8u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(edi & edi, 32);
                if (fl.cf || fl.zf) { pc = 96; continue; }
                pc = 91; continue;
            }
            91 => {
                edx = m.r32(esi.wrapping_add(0x8f4u32));
                pc = 92; continue;
            }
            92 => {
                fl = lift::sub(u32::from(m.r16(edx)), (ebp & 0xffff), 16).1;
                if fl.zf { pc = 95; continue; }
                pc = 93; continue;
            }
            93 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edi, 32).1;
                if fl.cf { pc = 92; continue; }
                pc = 94; continue;
            }
            94 => {
                pc = 96; continue;
            }
            95 => {
                fl = lift::sub(ecx, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 97; continue; }
                pc = 96; continue;
            }
            96 => {
                m.w8(esp.wrapping_add(0x12u32), (0x1u32) as u8);
                pc = 97; continue;
            }
            97 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edi = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                pc = 98; continue;
            }
            98 => {
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((ebp & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if fl.cf { pc = 87; continue; }
                pc = 99; continue;
            }
            99 => {
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(esp.wrapping_add(0x18u32), edi);
                if !fl.sf { pc = 84; continue; }
                pc = 100; continue;
            }
            100 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 102; continue; }
                pc = 101; continue;
            }
            101 => {
                m.w8(esp.wrapping_add(0x12u32), (0x1u32) as u8);
                pc = 103; continue;
            }
            102 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 114; continue; }
                pc = 103; continue;
            }
            103 => {
                ecx = m.r32(esi.wrapping_add(0x8ecu32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esi.wrapping_add(0x8f0u32), 0x0u32);
                eax = ecx.wrapping_add(edx);
                m.w32(esi.wrapping_add(0x8e8u32), eax);
                let (r, f) = lift::add(eax, eax, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x8ecu32));
                edx = m.r32(esi.wrapping_add(0x8f4u32));
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                edi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                esp = esp.wrapping_add(0x10);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 105; continue; }
                pc = 104; continue;
            }
            104 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 105; continue;
            }
            105 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                m.w32(esi.wrapping_add(0x8f4u32), edi);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                edi = edi.wrapping_add(eax.wrapping_mul(2));
                if fl.sf { pc = 114; continue; }
                pc = 106; continue;
            }
            106 => {
                ebx = 0x4u32;
                pc = 107; continue;
            }
            107 => {
                fl = lift::sub(ebp, m.r32(esi.wrapping_add(0x8e0u32)), 32).1;
                if fl.zf { pc = 113; continue; }
                pc = 108; continue;
            }
            108 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xeu32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 113; continue; }
                pc = 109; continue;
            }
            109 => {
                edx = m.r32(0x6dd7a4u32);
                eax = ecx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(edx, m.r32(esi.wrapping_add(0x8e4u32)), 32).1;
                if !fl.zf { pc = 112; continue; }
                pc = 110; continue;
            }
            110 => {
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), (ebx & 0xffff), 16).1;
                if !fl.cf { pc = 112; continue; }
                pc = 111; continue;
            }
            111 => {
                edx = m.r32(esi.wrapping_add(0x8f0u32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::add(edi, 0x2u32, 32); fl = f;
                edi = r;
                m.w32(esi.wrapping_add(0x8f0u32), edx);
                m.w16(edi.wrapping_add(0xfffffffeu32), ((ecx & 0xffff)) as u16);
                pc = 112; continue;
            }
            112 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if fl.cf { pc = 109; continue; }
                pc = 113; continue;
            }
            113 => {
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.sf { pc = 107; continue; }
                pc = 114; continue;
            }
            114 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                ecx = m.r32(esp.wrapping_add(0x13u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax | ecx; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            115 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004751D0` ship_list_window_layout_controls
pub fn f_004751d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x68);
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
                ecx = 0x6da5f0u32;
                edi = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esi.wrapping_add(0xcc8u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.cf || fl.zf) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xcc8u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebx, eax, 32).1;
                if fl.cf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xbd4u32));
                m.w32(esi.wrapping_add(0xcc8u32), 0x0u32);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 5; continue;
            }
            5 => {
                m.w32(esi.wrapping_add(0xbd4u32), 0x0u32);
                ecx = m.r32(0x6e51acu32);
                edx = esp.wrapping_add(0x40u32);
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                edx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0x838u32);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebp;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                ecx = ebp;
                let (r, f) = lift::imul(ebx, m.r32(esi.wrapping_add(0x8ecu32))); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = esi.wrapping_add(0x874u32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = esi.wrapping_add(0x874u32);
                ebp = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                ecx = esi.wrapping_add(0x874u32);
                let (r, f) = lift::imul(ebp, m.r32(esi.wrapping_add(0x8f0u32))); fl = f;
                ebp = r;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x908u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x8ccu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = esi.wrapping_add(0x874u32);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = esi.wrapping_add(0x838u32);
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(0x838u32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0xdbcu32));
                ecx = esi.wrapping_add(0xdbcu32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = esi.wrapping_add(0x838u32);
                ebx = eax;
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = esi.wrapping_add(0xd8u32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x461fd0, ecx, &a); }
                ecx = m.r32(0x6cbb00u32);
                edx = ecx;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x2cu32), eax);
                eax = m.r32(esp.wrapping_add(0x38u32));
                ecx = m.r32(0x6cbb00u32);
                fl = lift::sub(edx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = ecx.wrapping_add(edx);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x30u32), eax);
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x8f0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 19; continue;
            }
            9 => {
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x30u32), eax);
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 16; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 16; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ebx = m.r32(esi.wrapping_add(0x908u32));
                edx = m.r32(esi.wrapping_add(0x8ccu32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::sub(ebx, edx, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                ecx = esi.wrapping_add(0x874u32);
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = ebp;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebp;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = ebp;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebp;
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebp = esi.wrapping_add(0x838u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ebx;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ecx = m.r32(esi.wrapping_add(0x8ecu32));
                ebx = eax;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.cf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(ebx, m.r32(esi.wrapping_add(0x8ecu32)), 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                pc = 19; continue;
            }
            13 => {
                ecx = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.cf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                let (r, f) = lift::sub(ebx, ecx, 32); fl = f;
                ebx = r;
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x8f0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 19; continue;
            }
            15 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(ebx, m.r32(esp.wrapping_add(0x14u32)), 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                pc = 19; continue;
            }
            16 => {
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebp;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = ebp;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x908u32));
                edx = m.r32(esi.wrapping_add(0x8ccu32));
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = ebp;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ebx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ebx = eax;
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                pc = 19; continue;
            }
            18 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 19; continue;
            }
            19 => {
                ecx = esi.wrapping_add(0x874u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esi.wrapping_add(0x2cu32));
                eax = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x40u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0x14u32), eax);
                if (fl.sf == fl.of) { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                m.w32(esi.wrapping_add(0x14u32), 0x0u32);
                pc = 21; continue;
            }
            21 => {
                ecx = m.r32(esi.wrapping_add(0x30u32));
                eax = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(0x6cbb00u32);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub(edx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esi.wrapping_add(0x18u32), edx);
                pc = 24; continue;
            }
            23 => {
                edx = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x18u32), eax);
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                fl = lift::logic((eax & 0xff) & 0x3u32, 8);
                if fl.zf { pc = 28; continue; }
                pc = 25; continue;
            }
            25 => {
                edx = eax;
                let r = edx & 0x80000003u32; fl = lift::logic(r, 32);
                edx = r;
                if !fl.sf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let r = edx | 0xfffffffcu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                pc = 27; continue;
            }
            27 => {
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esi.wrapping_add(0x14u32), edx);
                pc = 28; continue;
            }
            28 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x41c5a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x910u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xbd8u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = esi.wrapping_add(0xa04u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xaecu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x1cu32));
                let (r, f) = lift::add(edx, m.r32(esi.wrapping_add(0x34u32)), 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                m.w32(esp.wrapping_add(0x54u32), edx);
                m.w32(esp.wrapping_add(0x24u32), edx);
                edx = m.r32(esi.wrapping_add(0xcc0u32));
                fl = lift::logic(edx & edx, 32);
                edx = m.r32(esi.wrapping_add(0x904u32));
                m.w8(esp.wrapping_add(0x10u32), (u32::from(fl.zf)) as u8);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x908u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x90cu32));
                let (r, f) = lift::add(ecx, m.r32(esp.wrapping_add(0x54u32)), 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x4cu32), edx);
                m.w32(esp.wrapping_add(0x50u32), eax);
                eax = m.r32(esi.wrapping_add(0x910u32));
                m.w32(esp.wrapping_add(0x54u32), ecx);
                ecx = esi.wrapping_add(0x910u32);
                edx = esp.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x9f8u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x9fcu32));
                m.w32(esp.wrapping_add(0x4cu32), eax);
                eax = m.r32(ebx);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0xa00u32));
                m.w32(esp.wrapping_add(0x50u32), ecx);
                let (r, f) = lift::add(edx, m.r32(esp.wrapping_add(0x24u32)), 32); fl = f;
                edx = r;
                ecx = esp.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x58u32), edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x9f8u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x9fcu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                m.w32(esp.wrapping_add(0x4cu32), eax);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0xa00u32));
                let (r, f) = lift::add(edx, m.r32(esp.wrapping_add(0x24u32)), 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x50u32), ecx);
                ecx = esi.wrapping_add(0xaecu32);
                eax = esp.wrapping_add(0x4cu32);
                m.w32(esp.wrapping_add(0x54u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x10u32)))) & 0xff);
                edx = m.r32(esi.wrapping_add(0xaecu32));
                ecx = esi.wrapping_add(0xaecu32);
                let r = (eax & 0xff) ^ 0x1u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xaecu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x910u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x14u32));
                ebx = m.r32(esi.wrapping_add(0x8c8u32));
                ecx = m.r32(esi.wrapping_add(0x8ccu32));
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x8d0u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = esp.wrapping_add(0x1cu32);
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                m.w32(esp.wrapping_add(0x28u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a4e0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = ebp;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                let (r, f) = lift::add(eax, m.r32(esp.wrapping_add(0x20u32)), 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                pc = 30; continue;
            }
            30 => {
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x874u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a4e0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = esi.wrapping_add(0xd8u32);
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x461480, ecx, &a); }
                ecx = esi.wrapping_add(0x48cu32);
                { let a = []; eax = h.call_at(esp, m, 0x461480, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x83cu32));
                ecx = m.r32(0x6cbb00u32);
                eax = 0x55555556u32;
                m.w32(esp.wrapping_add(0x58u32), edx);
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                ecx = m.r32(esi.wrapping_add(0x14u32));
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x60u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                ebx = m.r32(esi.wrapping_add(0x840u32));
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ecx = ebp;
                m.w32(esp.wrapping_add(0x5cu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = ebp;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                edx = eax;
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x68u32);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x6cu32), edx);
                { let t = m.r32(0x66a4a4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebp;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                edx = esp.wrapping_add(0x6cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = esi.wrapping_add(0xd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x460ef0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x8ecu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0xd0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 33; continue;
            }
            32 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = esp.wrapping_add(0x70u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x78u32), eax);
                m.w32(esp.wrapping_add(0x7cu32), eax);
                m.w32(esp.wrapping_add(0x80u32), eax);
                m.w32(esp.wrapping_add(0x84u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x460ef0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 33; continue;
            }
            33 => {
                ecx = m.r32(esi.wrapping_add(0x8f0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                ecx = m.r32(esi.wrapping_add(0x87cu32));
                ebx = esi.wrapping_add(0x874u32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = ebx;
                m.w32(esp.wrapping_add(0x5cu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = ebx;
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = eax;
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x68u32);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x6cu32), edx);
                { let t = m.r32(0x66a4a4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = esi.wrapping_add(0x874u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebx;
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = esp.wrapping_add(0x6cu32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                ebx = esi.wrapping_add(0x48cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x460ef0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x8f0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x484u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 36; continue;
            }
            35 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = esp.wrapping_add(0x70u32);
                ebx = esi.wrapping_add(0x48cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x78u32), eax);
                m.w32(esp.wrapping_add(0x7cu32), eax);
                m.w32(esp.wrapping_add(0x80u32), eax);
                m.w32(esp.wrapping_add(0x84u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x460ef0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 36; continue;
            }
            36 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x475c10, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x476dc0, ecx, &a); }
                pc = 38; continue;
            }
            38 => {
                ecx = m.r32(esi.wrapping_add(0x83cu32));
                edx = m.r32(esi.wrapping_add(0x864u32));
                eax = m.r32(esi.wrapping_add(0xbd8u32));
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = esi.wrapping_add(0x910u32);
                m.w32(esp.wrapping_add(0x14u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x70u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6b2aa4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x18u32));
                ebx = esi.wrapping_add(0xbd8u32);
                ecx = ebx;
                eax = eax.wrapping_add(edx).wrapping_add(0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x24u32));
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xcccu32)))) & 0xff);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6cc91cu32));
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xeacu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xfa4u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esp.wrapping_add(0x1cu32);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x40aa40, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebp;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xcd4u32));
                ecx = esi.wrapping_add(0xcd4u32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = m.r32(esi.wrapping_add(0xcd0u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esp.wrapping_add(0x20u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(0x6cbaf4u32);
                m.w32(esp.wrapping_add(0x30u32), ecx);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                edx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::sub(edx, 0x1eu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 41; continue; }
                pc = 39; continue;
            }
            39 => {
                fl = lift::sub(m.r32(eax.wrapping_add(0x4u32)), ecx, 32).1;
                if fl.zf { pc = 41; continue; }
                pc = 40; continue;
            }
            40 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                let (r, f) = lift::add(edx, 0x1e0u32, 32); fl = f;
                edx = r;
                ebx = m.r32(edx);
                m.w32(esp.wrapping_add(0x68u32), ebx);
                ebx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x6cu32), ebx);
                ebx = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x70u32), ebx);
                edx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x74u32), edx);
                pc = 41; continue;
            }
            41 => {
                eax = m.r32(eax.wrapping_add(0x4u32));
                ebx = esi.wrapping_add(0xeacu32);
                m.w32(esp.wrapping_add(0x54u32), ecx);
                m.w32(ebx.wrapping_add(0xe8u32), eax);
                eax = m.r32(esp.wrapping_add(0x68u32));
                edx = ebx.wrapping_add(0xecu32);
                m.w32(ebx.wrapping_add(0xecu32), eax);
                eax = m.r32(esp.wrapping_add(0x6cu32));
                m.w32(edx.wrapping_add(0x4u32), eax);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(edx.wrapping_add(0x8u32), ecx);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x448u32));
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                ebp = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(0x6cbaf4u32);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x20u32), ebp);
                eax = m.r32(edx.wrapping_add(0x4u32));
                edx = m.r32(esp.wrapping_add(0x68u32));
                ebp = esi.wrapping_add(0xfa4u32);
                ecx = ebp.wrapping_add(0xecu32);
                m.w32(ebp.wrapping_add(0xe8u32), eax);
                eax = m.r32(esp.wrapping_add(0x6cu32));
                m.w32(ecx, edx);
                edx = m.r32(esp.wrapping_add(0x54u32));
                m.w32(ecx.wrapping_add(0x4u32), eax);
                m.w32(ecx.wrapping_add(0x8u32), edx);
                eax = m.r32(ebp);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x8b0u32));
                edx = m.r32(esi.wrapping_add(0x8ecu32));
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, edx, 32).1;
                if fl.cf { pc = 43; continue; }
                pc = 42; continue;
            }
            42 => {
                m.w32(esi.wrapping_add(0x8b0u32), eax);
                pc = 43; continue;
            }
            43 => {
                edx = m.r32(esi.wrapping_add(0x8b4u32));
                ecx = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::sub(edx, ecx, 32).1;
                if fl.cf { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                m.w32(esi.wrapping_add(0x8b4u32), eax);
                pc = 45; continue;
            }
            45 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x68);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00475C10` ship_list_window_create_row_buttons
pub fn f_00475c10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x65f728u32);
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
                esi = ecx;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::sub(m.r32(esi.wrapping_add(0x8ecu32)), ebp, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                edi = m.r32(esi.wrapping_add(0x8ecu32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = edi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                ecx = edi.wrapping_add(eax.wrapping_mul(4));
                edx = ecx.wrapping_mul(8).wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x14u32), eax);
                fl = lift::sub(eax, ebp, 32).1;
                m.w32(esp.wrapping_add(0x20u32), ebp);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6910u32);
                ebx = eax.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(eax, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 4; continue;
            }
            3 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                m.w32(esp.wrapping_add(0x20u32), 0xffffffffu32);
                fl = lift::sub(eax, ebp, 32).1;
                m.w32(esi.wrapping_add(0xbd4u32), ebx);
                m.w32(esi.wrapping_add(0xcc8u32), eax);
                if (fl.cf || fl.zf) { pc = 11; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x8f4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(ebp.wrapping_mul(2))))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(0x134u32))), 0x11u32, 16).1;
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0xbd4u32));
                ecx = esi.wrapping_add(0xcd4u32);
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                ecx = esi.wrapping_add(0xdbcu32);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(0x134u32))), 0x13u32, 16).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(esi.wrapping_add(0xbd4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(edi.wrapping_add(eax));
                ecx = edi.wrapping_add(eax);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebp, eax, 32).1;
                if fl.cf { pc = 6; continue; }
                pc = 11; continue;
            }
            11 => {
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

/// `0X00475D70` ship_list_window_recreate_row_buttons_if_needed
pub fn f_00475d70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edi = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::logic(edi & edi, 32);
                if (fl.cf || fl.zf) { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x8f4u32));
                ebx = m.r32(0x6dd7a4u32);
                ebp = 0x20u32;
                pc = 2; continue;
            }
            2 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x3cu32))), (ebp & 0xffff), 16).1;
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x11u32, 16).1;
                if fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub((eax & 0xffff), 0xdu32, 16).1;
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(edx, edi, 32).1;
                if fl.cf { pc = 2; continue; }
                pc = 6; continue;
            }
            6 => {
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
            7 => {
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ebp = eax;
                eax = m.r32(esi.wrapping_add(0xcc8u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.cf || fl.zf) { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                ecx = ebp;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xcc8u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(edi, eax, 32).1;
                if fl.cf { pc = 9; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0xbd4u32));
                m.w32(esi.wrapping_add(0xcc8u32), 0x0u32);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            12 => {
                ecx = esi;
                m.w32(esi.wrapping_add(0xbd4u32), 0x0u32);
                { let a = []; eax = h.call_at(esp, m, 0x475c10, ecx, &a); }
                ecx = esi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x476dc0, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00476050` ship_list_window_update
pub fn f_00476050(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x34);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x474b70, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4779d0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x18u32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x40u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4751d0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x18u32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x40u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                ecx = esi.wrapping_add(0xbd8u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xcccu32)))) & 0xff);
                let (r, f) = lift::add((ecx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = (ecx & !0xff) | ((r) & 0xff);
                eax = (eax & !0xff) | (((ecx & 0xff)) & 0xff);
                m.w8(esi.wrapping_add(0xcccu32), ((ecx & 0xff)) as u8);
                fl = lift::sub((eax & 0xff), 0x5u32, 8).1;
                if fl.cf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w8(esi.wrapping_add(0xcccu32), (0x0u32) as u8);
                pc = 5; continue;
            }
            5 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4779d0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x18u32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x40u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ebp = eax;
                eax = m.r32(esi.wrapping_add(0xcc8u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.cf || fl.zf) { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0xbd4u32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xcc8u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(edi, eax, 32).1;
                if fl.cf { pc = 7; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esi.wrapping_add(0xbd4u32));
                m.w32(esi.wrapping_add(0xcc8u32), 0x0u32);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                ecx = esi;
                m.w32(esi.wrapping_add(0xbd4u32), 0x0u32);
                { let a = []; eax = h.call_at(esp, m, 0x475c10, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x476dc0, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xcccu32)))) & 0xff);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6cc91cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0xbd8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                edi = eax;
                ebx = esi.wrapping_add(0xeacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0xfa4u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            11 => {
                edi = m.r32(esi.wrapping_add(0x8ecu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic(edi & edi, 32);
                if (fl.cf || fl.zf) { pc = 19; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esi.wrapping_add(0x8f4u32));
                ebx = m.r32(0x6dd7a4u32);
                ebp = 0x20u32;
                pc = 13; continue;
            }
            13 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x3cu32))), (ebp & 0xffff), 16).1;
                if !fl.zf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x11u32, 16).1;
                if fl.zf { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub((eax & 0xffff), 0xdu32, 16).1;
                if fl.zf { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(edx, edi, 32).1;
                if fl.cf { pc = 13; continue; }
                pc = 17; continue;
            }
            17 => {
                pc = 19; continue;
            }
            18 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x475b90, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x475c10, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x476dc0, ecx, &a); }
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 26; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = esi.wrapping_add(0xd0u32);
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                edi = esi.wrapping_add(0x838u32);
                edx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x38u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x2cu32));
                ecx = edi;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x40u32), eax);
                eax = m.r32(esp.wrapping_add(0x28u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xcd0u32));
                edi = m.r32(esp.wrapping_add(0x34u32));
                ebx = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(esi.wrapping_add(0xdbcu32));
                let (r, f) = lift::sub(edi, ecx, 32); fl = f;
                edi = r;
                ecx = esi.wrapping_add(0xdbcu32);
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x34u32), edi);
                m.w32(esp.wrapping_add(0x3cu32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = 0x6da5f0u32;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x38u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x476dc0, ecx, &a); }
                pc = 26; continue;
            }
            22 => {
                ebx = m.r32(esi.wrapping_add(0x474u32));
                eax = m.r32(esi.wrapping_add(0x478u32));
                fl = lift::sub(ebx, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if (fl.sf == fl.of) { pc = 26; continue; }
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(esi.wrapping_add(0x8f4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(ebx.wrapping_mul(2))))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                fl = lift::sub(u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x3cu32))), 0x0u32, 16).1;
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(esi.wrapping_add(0x474u32));
                edi = esi.wrapping_add(0x838u32);
                ecx = edi;
                let (r, f) = lift::sub(ebx, edx, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebp = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = esp.wrapping_add(0x28u32);
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = ebp;
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::imul(ecx, ebx); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x40u32), ecx);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x14u32));
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(esi.wrapping_add(0x478u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if (fl.sf != fl.of) { pc = 23; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 35; continue; }
                pc = 27; continue;
            }
            27 => {
                ecx = esi.wrapping_add(0x484u32);
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 31; continue; }
                pc = 28; continue;
            }
            28 => {
                edi = esi.wrapping_add(0x874u32);
                edx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x38u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x2cu32));
                ecx = edi;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x40u32), eax);
                eax = m.r32(esp.wrapping_add(0x28u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(0x838u32);
                fl = lift::sub(edi, ecx, 32).1;
                m.w32(esp.wrapping_add(0x3cu32), eax);
                if !fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                edx = m.r32(esi.wrapping_add(0xcd0u32));
                edi = m.r32(esp.wrapping_add(0x34u32));
                eax = m.r32(esi.wrapping_add(0xdbcu32));
                ecx = esi.wrapping_add(0xdbcu32);
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x34u32), edi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(m.r32(esp.wrapping_add(0x34u32)), eax, 32); fl = f;
                m.w32(esp.wrapping_add(0x34u32), r);
                pc = 30; continue;
            }
            30 => {
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 35; continue;
            }
            31 => {
                ebx = m.r32(esi.wrapping_add(0x828u32));
                eax = m.r32(esi.wrapping_add(0x82cu32));
                fl = lift::sub(ebx, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if (fl.sf == fl.of) { pc = 35; continue; }
                pc = 32; continue;
            }
            32 => {
                edx = m.r32(esi.wrapping_add(0x8ecu32));
                ecx = m.r32(esi.wrapping_add(0x8f4u32));
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                fl = lift::sub(u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x3cu32))), 0x0u32, 16).1;
                if fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                edx = m.r32(esi.wrapping_add(0x828u32));
                edi = esi.wrapping_add(0x874u32);
                ecx = edi;
                let (r, f) = lift::sub(ebx, edx, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebp = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = esp.wrapping_add(0x28u32);
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = ebp;
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::imul(ecx, ebx); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x40u32), ecx);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x14u32));
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(esi.wrapping_add(0x82cu32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if (fl.sf != fl.of) { pc = 32; continue; }
                pc = 35; continue;
            }
            35 => {
                ecx = m.r32(0x6ce6d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x24u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4885a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x8f8u32));
                fl = lift::sub(ecx, edx, 32).1;
                if fl.zf { pc = 37; continue; }
                pc = 36; continue;
            }
            36 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4773a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 37; continue;
            }
            37 => {
                eax = esp.wrapping_add(0x20u32);
                ecx = 0x6da5f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b77f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 45; continue; }
                pc = 38; continue;
            }
            38 => {
                ecx = m.r32(esi.wrapping_add(0x47cu32));
                eax = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x838u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x40aad0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(esi.wrapping_add(0x8b0u32));
                fl = lift::sub(eax, ecx, 32).1;
                edi = ecx;
                if fl.zf { pc = 44; continue; }
                pc = 39; continue;
            }
            39 => {
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 41; continue; }
                pc = 40; continue;
            }
            40 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4766f0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 41; continue;
            }
            41 => {
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                if fl.zf { pc = 43; continue; }
                pc = 42; continue;
            }
            42 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4766f0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 43; continue;
            }
            43 => {
                m.w32(esi.wrapping_add(0x8b0u32), edi);
                pc = 44; continue;
            }
            44 => {
                edx = m.r32(esi.wrapping_add(0x47cu32));
                eax = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                pc = 45; continue;
            }
            45 => {
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 52; continue; }
                pc = 46; continue;
            }
            46 => {
                eax = m.r32(esi.wrapping_add(0x830u32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x1cu32);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = esp.wrapping_add(0x10u32);
                edi = esi.wrapping_add(0x874u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x40aad0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esp.wrapping_add(0x10u32));
                ebp = m.r32(esi.wrapping_add(0x8b4u32));
                ebx = eax;
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if fl.zf { pc = 52; continue; }
                pc = 47; continue;
            }
            47 => {
                fl = lift::sub(ebp, 0xffffffffu32, 32).1;
                if fl.zf { pc = 49; continue; }
                pc = 48; continue;
            }
            48 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = esp.wrapping_add(0x28u32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = ebx;
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::imul(ecx, ebp); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x40u32), ecx);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x14u32));
                pc = 49; continue;
            }
            49 => {
                fl = lift::sub(ebx, 0xffffffffu32, 32).1;
                if fl.zf { pc = 51; continue; }
                pc = 50; continue;
            }
            50 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4766f0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 51; continue;
            }
            51 => {
                m.w32(esi.wrapping_add(0x8b4u32), ebx);
                pc = 52; continue;
            }
            52 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x476790, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x34);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00476790` ship_list_window_handle_buttons
pub fn f_00476790(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = esi.wrapping_add(0x910u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x4u32), eax);
                pc = 3; continue;
            }
            3 => {
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xa04u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xa04u32));
                ecx = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xa04u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let r = eax ^ 0x1u32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xa04u32));
                ecx = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xa04u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xcc0u32));
                ecx = esi;
                let r = eax ^ 0x1u32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0xcc0u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x476cc0, ecx, &a); }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x2u32, 32).1;
                m.w32(esp.wrapping_add(0x4u32), eax);
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x474u32));
                ecx = m.r32(esi.wrapping_add(0x478u32));
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x4u32), eax);
                if (fl.sf == fl.of) { pc = 12; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                ecx = eax.wrapping_add(edx.wrapping_mul(8));
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                ecx = m.r32(esi.wrapping_add(0x478u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x4u32), eax);
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 10; continue;
            }
            10 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            11 => {
                ecx = esp.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4768b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            12 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004768B0` ship_list_window_goto_ship_row
pub fn f_004768b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(ebp);
                edx = m.r32(esi.wrapping_add(0x8f4u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                edx = m.r32(0x6dd7a4u32);
                edi = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x13u32, 16).1;
                if fl.zf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub((eax & 0xffff), 0x11u32, 16).1;
                if !fl.zf { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = esp.wrapping_add(0x20u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(ebp);
                ecx = m.r32(esi.wrapping_add(0x8f4u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x20u32), 0xau32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x28u32), edx);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6ce6d0u32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                eax = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x44u32), ebx);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                m.w32(esp.wrapping_add(0x24u32), ebx);
                m.w8(esp.wrapping_add(0x28u32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4885a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if fl.zf { pc = 10; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ebp);
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x48fdf0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x6ce6d0u32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                edx = esp.wrapping_add(0x14u32);
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
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
            5 => {
                fl = lift::sub((eax & 0xffff), 0xeu32, 16).1;
                if fl.zf { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub((eax & 0xffff), 0xdu32, 16).1;
                if fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub((eax & 0xffff), 0xcu32, 16).1;
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                m.w8(esp.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                m.w32(esp.wrapping_add(0x18u32), 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ecx = m.r32(ebp);
                edx = m.r32(esi.wrapping_add(0x8f4u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                esp = esp.wrapping_add(0x4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                m.w16(eax, ((ecx & 0xffff)) as u16);
                ecx = m.r32(0x6ce6d0u32);
                eax = esp.wrapping_add(0x14u32);
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x476a30, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 10; continue;
            }
            10 => {
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

/// `0X00476DC0` ship_list_window_place_row_buttons
pub fn f_00476dc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                eax = esp.wrapping_add(0x14u32);
                ebx = esi.wrapping_add(0x838u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x40aa40, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esi.wrapping_add(0xcd4u32));
                ecx = esi.wrapping_add(0xcd4u32);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esi.wrapping_add(0xcd0u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = ebx;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebx;
                edi = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                ebp = m.r32(esi.wrapping_add(0x47cu32));
                let (r, f) = lift::imul(edi, m.r32(esi.wrapping_add(0x474u32))); fl = f;
                edi = r;
                edx = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esi.wrapping_add(0x8ecu32));
                let (r, f) = lift::add(edi, ebp, 32); fl = f;
                edi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(edx, edi, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                fl = lift::logic(ecx & ecx, 32);
                m.w32(esp.wrapping_add(0x18u32), edx);
                if (fl.cf || fl.zf) { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x474u32)), 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x478u32)), 32).1;
                if (fl.sf == fl.of) { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                edx = m.r32(eax.wrapping_add(edi));
                ecx = eax.wrapping_add(edi);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xbd4u32));
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 8; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                edx = m.r32(eax.wrapping_add(edi));
                ecx = eax.wrapping_add(edi);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                edx = m.r32(eax.wrapping_add(edi));
                ecx = eax.wrapping_add(edi);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xbd4u32));
                edx = m.r32(eax.wrapping_add(edi));
                ecx = eax.wrapping_add(edi);
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 9; continue;
            }
            9 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebx;
                ebp = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x8ecu32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if fl.cf { pc = 2; continue; }
                pc = 10; continue;
            }
            10 => {
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

/// `0X00477150` ship_list_window_on_mouse_event
pub fn f_00477150(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(0xc);
                fl = lift::sub(eax, 0xfffffffeu32, 32).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                if !fl.zf { pc = 28; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x8b0u32));
                let r = edx | 0xffffffffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x8b4u32)), edx, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6dcbd1u32))) & 0xff);
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                let (r, f) = lift::shr((edx & 0xff), 0x1u32, 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (eax & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) | (eax & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                m.w8(0x6dcbd1u32, ((edx & 0xff)) as u8);
                if !fl.zf { pc = 28; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x6ce6d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
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
            5 => {
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x8b4u32)), edx, 32).1;
                if fl.zf { pc = 28; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(0x6dccf8u32);
                ebp = m.r32(esi.wrapping_add(0xea4u32));
                edi = eax;
                let (r, f) = lift::sub(edi, ebp, 32); fl = f;
                edi = r;
                fl = lift::sub(edi, 0x1f4u32, 32).1;
                if !fl.cf { pc = 11; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(ecx, edx, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(ecx, m.r32(esi.wrapping_add(0xea8u32)), 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x474u32));
                eax = esp.wrapping_add(0x24u32);
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x28u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4768b0, ecx, &a); }
                esp = esp.wrapping_add(4);
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
            11 => {
                fl = lift::sub(ecx, edx, 32).1;
                m.w32(esi.wrapping_add(0xea4u32), eax);
                m.w32(esi.wrapping_add(0xea8u32), ecx);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0x474u32));
                pc = 14; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                edx = m.r32(esi.wrapping_add(0x8b4u32));
                ecx = m.r32(esi.wrapping_add(0x828u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 14; continue;
            }
            14 => {
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x8f4u32));
                ebp = m.r32(0x6dd7a4u32);
                edi = (edi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                eax = edi;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), 0xeu32, 16).1;
                if fl.zf { pc = 28; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub((ecx & 0xffff), 0x13u32, 16).1;
                if fl.zf { pc = 28; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub((ecx & 0xffff), 0xdu32, 16).1;
                if fl.zf { pc = 28; continue; }
                pc = 17; continue;
            }
            17 => {
                fl = lift::sub((ecx & 0xffff), 0xcu32, 16).1;
                if fl.zf { pc = 28; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dcbd1u32))) & 0xff);
                m.w8(esp.wrapping_add(0x18u32), (0x0u32) as u8);
                ebx = (ebx & !0xff) | (((ecx & 0xff)) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let (r, f) = lift::shr((ebx & 0xff), 0x1u32, 8, fl); fl = f;
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let r = (ebx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (ebx & 0xff) | (ecx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                m.w8(0x6dcbd1u32, ((ebx & 0xff)) as u8);
                if fl.zf { pc = 26; continue; }
                pc = 19; continue;
            }
            19 => {
                edx = m.r32(0x6dfc14u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 26; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(esi.wrapping_add(0x8f8u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ebp)))) & 0xff);
                fl = lift::sub(ebx, edx, 32).1;
                if !fl.zf { pc = 26; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esi.wrapping_add(0x900u32));
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x48fdf0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x8f8u32));
                esp = esp.wrapping_add(0xc);
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.zf { pc = 27; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(esi.wrapping_add(0x900u32));
                fl = lift::logic(ecx & ecx, 32);
                if !fl.zf { pc = 27; continue; }
                pc = 25; continue;
            }
            25 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = eax.wrapping_add(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x900u32));
                ecx = m.r32(esi.wrapping_add(0x8f8u32));
                let (r, f) = lift::add(edx, edx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                edx = m.r32(esi.wrapping_add(0x900u32));
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_add(0x10);
                m.w16(eax.wrapping_add(edx.wrapping_mul(2)), ((edi & 0xffff)) as u16);
                pc = 27; continue;
            }
            26 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                m.w32(esp.wrapping_add(0x14u32), 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w16(eax, ((edi & 0xffff)) as u16);
                ecx = m.r32(0x6ce6d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x495190, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esp.wrapping_add(0x18u32), ((eax & 0xff)) as u8);
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(0x6ce6d0u32);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
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
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004773A0` ship_list_window_set_selected_ships
pub fn f_004773a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x44);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(esi.wrapping_add(0x900u32));
                m.w32(esp.wrapping_add(0x18u32), eax);
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 13; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                ebp = m.r32(esi.wrapping_add(0x474u32));
                eax = m.r32(esi.wrapping_add(0x478u32));
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebp);
                if (fl.sf == fl.of) { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                ecx = m.r32(esi.wrapping_add(0x8fcu32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ebp.wrapping_mul(2))))) & 0xffff);
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(eax.wrapping_add(ecx))), 16).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0x474u32));
                edi = esi.wrapping_add(0x838u32);
                ecx = edi;
                let (r, f) = lift::sub(ebp, edx, 32); fl = f;
                ebp = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = esp.wrapping_add(0x1cu32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = ebx;
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::imul(ecx, ebp); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x40u32), ecx);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x478u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebp);
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 12; continue; }
                pc = 8; continue;
            }
            8 => {
                ebp = m.r32(esi.wrapping_add(0x828u32));
                eax = m.r32(esi.wrapping_add(0x82cu32));
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebp);
                if (fl.sf == fl.of) { pc = 12; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0x8ecu32));
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                ecx = m.r32(esi.wrapping_add(0x8fcu32));
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(eax.wrapping_add(ecx))), 16).1;
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x828u32));
                edi = esi.wrapping_add(0x874u32);
                ecx = edi;
                let (r, f) = lift::sub(ebp, edx, 32); fl = f;
                ebp = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = esp.wrapping_add(0x28u32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = ebx;
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::imul(ecx, ebp); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x44u32), eax);
                m.w32(esp.wrapping_add(0x48u32), ecx);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x50u32), ecx);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x50u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0x82cu32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebp);
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esi.wrapping_add(0x900u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x10u32), edx);
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 13; continue;
            }
            13 => {
                ebx = m.r32(esp.wrapping_add(0x5cu32));
                fl = lift::logic(ebx & ebx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 27; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(esp.wrapping_add(0x58u32));
                m.w32(esp.wrapping_add(0x14u32), ebx);
                m.w32(esp.wrapping_add(0x10u32), edx);
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x8ecu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 20; continue; }
                pc = 16; continue;
            }
            16 => {
                ebp = m.r32(esi.wrapping_add(0x474u32));
                eax = m.r32(esi.wrapping_add(0x478u32));
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x18u32), ebp);
                if (fl.sf == fl.of) { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ebp.wrapping_mul(2))))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(edx)), 16).1;
                if !fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                edi = m.r32(esi.wrapping_add(0x474u32));
                let (r, f) = lift::sub(ebp, edi, 32); fl = f;
                ebp = r;
                edi = esi.wrapping_add(0x838u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = ebx;
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::imul(ecx, ebp); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x44u32), eax);
                m.w32(esp.wrapping_add(0x48u32), ecx);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x50u32), ecx);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x50u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp.wrapping_add(0x18u32));
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0x478u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x18u32), ebp);
                if (fl.sf != fl.of) { pc = 17; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                ebp = m.r32(esi.wrapping_add(0x828u32));
                eax = m.r32(esi.wrapping_add(0x82cu32));
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x18u32), ebp);
                if (fl.sf == fl.of) { pc = 25; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esi.wrapping_add(0x8ecu32));
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                edx = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(edx)), 16).1;
                if !fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                edi = m.r32(esi.wrapping_add(0x828u32));
                let (r, f) = lift::sub(ebp, edi, 32); fl = f;
                ebp = r;
                edi = esi.wrapping_add(0x874u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40ac40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = ebx;
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::imul(ecx, ebp); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x40u32), ecx);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp.wrapping_add(0x18u32));
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esi.wrapping_add(0x82cu32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x18u32), ebp);
                if (fl.sf != fl.of) { pc = 22; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.zf { pc = 15; continue; }
                pc = 26; continue;
            }
            26 => {
                ebx = m.r32(esp.wrapping_add(0x5cu32));
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(esi.wrapping_add(0x8fcu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 29; continue;
            }
            29 => {
                edi = ebx.wrapping_add(ebx);
                m.w32(esi.wrapping_add(0x8fcu32), 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x60u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi.wrapping_add(0x8fcu32), eax);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0x10);
                m.w32(esi.wrapping_add(0x8f8u32), edi);
                m.w32(esi.wrapping_add(0x900u32), ebx);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x44);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004778A0` ship_list_window_remove_ship_on_event
pub fn f_004778a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x8e8u32));
                ebp = m.r32(esp.wrapping_add(0x20u32));
                ebx = esi.wrapping_add(0x8e8u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(ecx & ecx, 32);
                if (fl.cf || fl.zf) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi.wrapping_add(0x8f4u32));
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(u32::from(m.r16(edx)), (ebp & 0xffff), 16).1;
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 3; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x24u32))), 0x17u32, 16).1;
                if !fl.zf { pc = 14; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esi.wrapping_add(0x8f4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x48fdf0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                m.w32(esi.wrapping_add(0x8f4u32), eax);
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(0x6dd7a4u32);
                let r = ebp & 0xffffu32; fl = lift::logic(r, 32);
                ebp = r;
                eax = ebp;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ecx)))) & 0xff);
                ecx = m.r32(esi.wrapping_add(0x8e0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::sub(m.r32(esi.wrapping_add(0x8ecu32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0x8ecu32), r);
                pc = 13; continue;
            }
            12 => {
                let (r, f) = lift::sub(m.r32(esi.wrapping_add(0x8f0u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0x8f0u32), r);
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x18u32), edx);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4751d0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x18u32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0xcu32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            14 => {
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

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x4726b0 => f_004726b0(m, h, ecx, esp),
        0x4727e0 => f_004727e0(m, h, ecx, esp),
        0x472910 => f_00472910(m, h, ecx, esp),
        0x472a10 => f_00472a10(m, h, ecx, esp),
        0x472c60 => f_00472c60(m, h, ecx, esp),
        0x473220 => f_00473220(m, h, ecx, esp),
        0x473650 => f_00473650(m, h, ecx, esp),
        0x4738c0 => f_004738c0(m, h, ecx, esp),
        0x4743f0 => f_004743f0(m, h, ecx, esp),
        0x474a20 => f_00474a20(m, h, ecx, esp),
        0x474b70 => f_00474b70(m, h, ecx, esp),
        0x4751d0 => f_004751d0(m, h, ecx, esp),
        0x475c10 => f_00475c10(m, h, ecx, esp),
        0x475d70 => f_00475d70(m, h, ecx, esp),
        0x476050 => f_00476050(m, h, ecx, esp),
        0x476790 => f_00476790(m, h, ecx, esp),
        0x4768b0 => f_004768b0(m, h, ecx, esp),
        0x476dc0 => f_00476dc0(m, h, ecx, esp),
        0x477150 => f_00477150(m, h, ecx, esp),
        0x4773a0 => f_004773a0(m, h, ecx, esp),
        0x4778a0 => f_004778a0(m, h, ecx, esp),
        _ => return None,
    }))
}
