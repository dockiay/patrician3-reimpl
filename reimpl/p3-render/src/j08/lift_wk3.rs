//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

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

/// `0X00488460` ship_side_menu_hide
pub fn f_00488460(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = m.r32(0x6cbb40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42d6d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cba98u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6cba98u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                edx = m.r32(0x66d504u32);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc8u32)), 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                m.w32(esi.wrapping_add(0x3868u32), eax);
                pc = 9; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x5238u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                m.w32(esi.wrapping_add(0x3868u32), eax);
                pc = 9; continue;
            }
            8 => {
                m.w32(esi.wrapping_add(0x3868u32), 0xffffffffu32);
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(0x6d9584u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cbb10u32);
                fl = lift::sub(m.r32(ecx.wrapping_add(0x5acu32)), 0x4950d0u32, 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 11; continue;
            }
            11 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x440ed0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00488600` ship_side_menu_load_layout
pub fn f_00488600(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6603cfu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0xd0);
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
                m.w32(esp.wrapping_add(0xe8u32), 0x0u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esp.wrapping_add(0xf4u32));
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9edcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0xf4u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0xfcu32));
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9eecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x1cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9ef4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w8(esp.wrapping_add(0x100u32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x24u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f04u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                m.w8(esp.wrapping_add(0xf8u32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                edi = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0xd4u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f2cu32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0xdcu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x48u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0xc8u32), eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                ebx = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f70u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x40u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f84u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x50u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f90u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fa0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x58u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0xb8u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fb0u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x90u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fc0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x68u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0xd8u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fdcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x98u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9ff8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x70u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0xa0u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba008u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x68u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba01cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba028u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xd0u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0x70u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba034u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0xa8u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba044u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba054u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x78u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba068u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x80u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba078u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba084u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xc0u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x88u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba094u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xdcu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0x44u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0c4u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x3cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0dcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x5cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x54u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x5cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0f4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba100u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x74u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x6cu32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba110u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x74u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba11cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba128u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x8cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0x84u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba138u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x8cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba144u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba154u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xa4u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x9cu32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0xbcu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba170u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba184u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xbcu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0xb4u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba194u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0xa4u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xd4u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0xd0u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1c4u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w16(esi.wrapping_add(0x582cu32), ((eax & 0xffff)) as u16);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w16(esi.wrapping_add(0x582eu32), ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x3864u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1ecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esi.wrapping_add(0x3874u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba200u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba214u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esi.wrapping_add(0x386cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x3870u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba220u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x20u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3878u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2bcu32));
                edi = esi.wrapping_add(0x2bcu32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x20u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xecu32));
                ebp = esi.wrapping_add(0xecu32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x20u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1d4u32));
                ebx = esi.wrapping_add(0x1d4u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0xd0u32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2258u32));
                ecx = esi.wrapping_add(0x2258u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0xd4u32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x574u32));
                ecx = esi.wrapping_add(0x574u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0xdcu32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x3a4u32));
                ecx = esi.wrapping_add(0x3a4u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x38u32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x48cu32));
                ecx = esi.wrapping_add(0x48cu32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0xc8u32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x65cu32));
                ecx = esi.wrapping_add(0x65cu32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x3938u32));
                ecx = esi.wrapping_add(0x3938u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                edx = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x3cecu32));
                ecx = esi.wrapping_add(0x3cecu32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x5428u32));
                ecx = esi.wrapping_add(0x5428u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = esp.wrapping_add(0xf0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x387cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x40a560, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x90u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x48u32));
                m.w32(esi.wrapping_add(0x3810u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xb8u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3814u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x3818u32), eax);
                eax = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x98u32));
                m.w32(esi.wrapping_add(0x380cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x58u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3824u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x381cu32), eax);
                eax = m.r32(esp.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x60u32));
                m.w32(esi.wrapping_add(0x3820u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xa0u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3828u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x382cu32), eax);
                eax = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xc0u32));
                m.w32(esi.wrapping_add(0x3830u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x70u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3834u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x3838u32), eax);
                eax = m.r32(esp.wrapping_add(0xa8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0xf8u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x38b8u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x40a560, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x78u32));
                edx = esp.wrapping_add(0xf0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x38f4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x40a560, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x80u32));
                m.w32(esi.wrapping_add(0x383cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xb0u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3840u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x3844u32), eax);
                eax = m.r32(esp.wrapping_add(0x88u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                m.w32(esi.wrapping_add(0x3848u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x384cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x3854u32), eax);
                eax = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                m.w32(esi.wrapping_add(0x3850u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x54u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x5824u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5858u32), eax);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x64u32));
                m.w32(esi.wrapping_add(0x5854u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x6cu32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x585cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5850u32), eax);
                eax = m.r32(esp.wrapping_add(0x74u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x7cu32));
                m.w32(esi.wrapping_add(0x5870u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x84u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x5868u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5864u32), eax);
                eax = m.r32(esp.wrapping_add(0x8cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x94u32));
                m.w32(esi.wrapping_add(0x586cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x9cu32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x5860u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5880u32), eax);
                eax = m.r32(esp.wrapping_add(0xa4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xacu32));
                m.w32(esi.wrapping_add(0x588cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xb4u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x5884u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5888u32), eax);
                eax = m.r32(esp.wrapping_add(0xbcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xc4u32));
                m.w32(esi.wrapping_add(0x5890u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esi.wrapping_add(0x385cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba238u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba244u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(edi);
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba250u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(ebx);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba25cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esi.wrapping_add(0x3a4u32));
                ecx = esi.wrapping_add(0x3a4u32);
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba270u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esi.wrapping_add(0x48cu32));
                ecx = esi.wrapping_add(0x48cu32);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba284u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esi.wrapping_add(0x65cu32));
                ecx = esi.wrapping_add(0x65cu32);
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba294u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esi.wrapping_add(0x574u32));
                ecx = esi.wrapping_add(0x574u32);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi.wrapping_add(0x5818u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2b0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x580cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2bcu32);
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = esi.wrapping_add(0x57ecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi.wrapping_add(0x57e0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x57f8u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x744u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = eax.wrapping_add(edx.wrapping_mul(4));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = esi.wrapping_add(0x82cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0x914u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x3724u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esi.wrapping_add(0x36a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                eax = esi.wrapping_add(0x36c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                eax = esi.wrapping_add(0x36e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x3724u32));
                ecx = esi.wrapping_add(0x3724u32);
                eax = esi.wrapping_add(0x3700u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x1d04u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de57u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = esi.wrapping_add(0x1decu32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de57u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2ecu32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2eeu32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xccu32));
                edx = esp.wrapping_add(0xf0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1ed8u32));
                edi = esi.wrapping_add(0x1ed8u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                eax = esi.wrapping_add(0x2238u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x10u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de58u32))) & 0xff);
                pc = 8; continue;
            }
            2 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x20u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de57u32))) & 0xff);
                pc = 8; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x30u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de56u32))) & 0xff);
                pc = 8; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x60u32, 32).1;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de55u32))) & 0xff);
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                pc = 8; continue;
            }
            8 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x2068u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6999b4u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x10u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de58u32))) & 0xff);
                pc = 16; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x20u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de57u32))) & 0xff);
                pc = 16; continue;
            }
            12 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x30u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de56u32))) & 0xff);
                pc = 16; continue;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x60u32, 32).1;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de55u32))) & 0xff);
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                pc = 16; continue;
            }
            16 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x1c1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(ecx.wrapping_mul(4));
                ebx = eax.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6999c8u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = esi.wrapping_add(0x2150u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x69c98cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba308u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba318u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xbu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba328u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2340u32));
                edi = esi.wrapping_add(0x2340u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xecu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x2380u32), (0x5u32) as u8);
                ebx = esi.wrapping_add(0x2438u32);
                ebp = 0x13u32;
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46cdc0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebx, 0xf8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 17; continue; }
                pc = 18; continue;
            }
            18 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x9fcu32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de55u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x69b338u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = esi.wrapping_add(0xae4u32);
                ebp = 0x13u32;
                pc = 19; continue;
            }
            19 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 19; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba354u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba340u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0xf8u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x4170u32));
                ecx = esi.wrapping_add(0x4170u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de58u32))) & 0xff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1du32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba360u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x5348u32));
                ecx = esi.wrapping_add(0x5348u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = esi.wrapping_add(0x6d50u32);
                ebp = esi.wrapping_add(0x7f70u32);
                m.w32(esp.wrapping_add(0x1cu32), 0x14u32);
                pc = 21; continue;
            }
            21 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = edi.wrapping_add(0xffffeb4cu32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de58u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de58u32))) & 0xff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebx);
                ecx = ebp.wrapping_add(0xffffeb60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebp, 0x20u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if !fl.zf { pc = 21; continue; }
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1du32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba378u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x81f0u32));
                ecx = esi.wrapping_add(0x81f0u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x66de56u32))) & 0xff);
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
                edx = m.r32(esi.wrapping_add(0x82d0u32));
                let (r, f) = lift::add(esi, 0x82d0u32, 32); fl = f;
                esi = r;
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4e57u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x104u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = esp.wrapping_add(0x20u32);
                m.w8(esp.wrapping_add(0xe8u32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0xe8u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0xe8u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0xf0u32);
                m.w32(esp.wrapping_add(0xe8u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0xe0u32));
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
                esp = esp.wrapping_add(0xdc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048AC30` ship_side_menu_fit_table_rows
pub fn f_0048ac30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6603f3u32);
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
    ebx = m.r32(esi.wrapping_add(0x38c0u32));
    edi = esi.wrapping_add(0x38b8u32);
    ecx = edi;
    { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
    ecx = edi;
    ebp = eax;
    { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
    ecx = edi;
    let (r, f) = lift::add(ebp, eax, 32); fl = f;
    ebp = r;
    { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
    ecx = m.r32(esi.wrapping_add(0x30u32));
    let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
    ecx = r;
    let (r, f) = lift::sub(ecx, eax, 32); fl = f;
    ecx = r;
    eax = ecx;
    ecx = edi;
    edx = ((eax as i32) >> 31) as u32;
    { let d = i64::from(ebp as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
    esp = esp.wrapping_add(4);
    ebx = m.r32(esi.wrapping_add(0x3884u32));
    edi = esi.wrapping_add(0x387cu32);
    ecx = edi;
    { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
    ecx = edi;
    ebp = eax;
    { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
    ecx = edi;
    let (r, f) = lift::add(ebp, eax, 32); fl = f;
    ebp = r;
    { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
    edx = m.r32(esi.wrapping_add(0x30u32));
    ecx = edi;
    let (r, f) = lift::sub(edx, ebx, 32); fl = f;
    edx = r;
    let (r, f) = lift::sub(edx, eax, 32); fl = f;
    edx = r;
    eax = edx;
    edx = ((eax as i32) >> 31) as u32;
    { let d = i64::from(ebp as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
    esp = esp.wrapping_add(4);
    ebx = m.r32(esi.wrapping_add(0x38fcu32));
    edi = esi.wrapping_add(0x38f4u32);
    ecx = edi;
    { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
    ecx = edi;
    ebp = eax;
    { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
    ecx = edi;
    let (r, f) = lift::add(ebp, eax, 32); fl = f;
    ebp = r;
    { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
    ecx = m.r32(esi.wrapping_add(0x30u32));
    let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
    ecx = r;
    let (r, f) = lift::sub(ecx, eax, 32); fl = f;
    ecx = r;
    eax = ecx;
    ecx = edi;
    edx = ((eax as i32) >> 31) as u32;
    { let d = i64::from(ebp as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(0x6c7cccu32);
    m.w32(esp.wrapping_add(0x10u32), edx);
    eax = m.r32(0x699874u32);
    ecx = m.r32(0x69988cu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    edx = esp.wrapping_add(0x18u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6ba394u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w32(esp.wrapping_add(0x2cu32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
    eax = m.r32(esp.wrapping_add(0x20u32));
    esp = esp.wrapping_add(0x10);
    ecx = esi.wrapping_add(0x744u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(0x699934u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi.wrapping_add(0x3724u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w16(esi.wrapping_add(0x57dcu32), (0xffffu32) as u16);
    edx = m.r32(0x6b04f8u32);
    let (r, f) = lift::add(esi, 0x82d0u32, 32); fl = f;
    esi = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    ecx = esi;
    { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esp.wrapping_add(0x10u32);
    m.w32(esp.wrapping_add(0x1cu32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
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

/// `0X0048ADA0` ship_side_menu_prerender
pub fn f_0048ada0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                edx = m.r32(0x66d504u32);
                edi = m.r32(esi.wrapping_add(0x14u32));
                ebp = m.r32(esi.wrapping_add(0x18u32));
                ebx = m.r32(esi.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                m.w8(esp.wrapping_add(0x17u32), ((ecx & 0xff)) as u8);
                esp = esp.wrapping_add(0x4);
                ecx = esi;
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esi.wrapping_add(0x94u32), edx);
                m.w8(esi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, edi, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x492bc0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x493d10, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x5238u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                edx = (edx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(ecx)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 3; continue;
            }
            2 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 3; continue;
            }
            3 => {
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492150, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi);
                eax = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(0x6cbd94u32);
                edx = esp;
                ecx = esi.wrapping_add(0x3938u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(edx, edi);
                m.w32(edx.wrapping_add(0x4u32), ebp);
                m.w32(edx.wrapping_add(0x8u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x461fe0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(0x6cbd94u32);
                ecx = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ecx, edi);
                m.w32(ecx.wrapping_add(0x4u32), ebp);
                m.w32(ecx.wrapping_add(0x8u32), ebx);
                ecx = esi.wrapping_add(0x3cecu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x461fe0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(0xc);
                eax = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x5428u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x461fe0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x744u32));
                eax = esp;
                ecx = esi.wrapping_add(0x744u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x82cu32));
                eax = esp;
                ecx = esi.wrapping_add(0x82cu32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x914u32));
                eax = esp;
                ecx = esi.wrapping_add(0x914u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x1decu32));
                eax = esp;
                ecx = esi.wrapping_add(0x1decu32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1d04u32));
                ecx = esi.wrapping_add(0x1d04u32);
                esp = esp.wrapping_sub(0xc);
                eax = esp;
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x1ed8u32));
                eax = esp;
                ecx = esi.wrapping_add(0x1ed8u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x2068u32));
                eax = esp;
                ecx = esi.wrapping_add(0x2068u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x1c1cu32));
                eax = esp;
                ecx = esi.wrapping_add(0x1c1cu32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x2150u32));
                eax = esp;
                ecx = esi.wrapping_add(0x2150u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x2258u32));
                eax = esp;
                ecx = esi.wrapping_add(0x2258u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi.wrapping_add(0x94u32), ecx);
                m.w8(esi.wrapping_add(0xa4u32), ((edx & 0xff)) as u8);
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
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048D8C0` ship_side_menu_draw_ship_picture
pub fn f_0048d8c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = m.r32(0x6dd7a4u32);
                esp = esp.wrapping_sub(0x28);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x8u32)))) & 0xffff);
                esi = ecx;
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x14u32)))) & 0xff);
                let r = (edx & 0xff) & 0x10u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), 0x10u32, 8).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = 0x4u32;
                pc = 4; continue;
            }
            3 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 4; continue;
            }
            4 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0xeu32)))) & 0xff);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x5870u32));
                edx = m.r32(esi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x5850u32));
                ebp = m.r32(eax.wrapping_add(0x4u32));
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x40u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                edi = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = m.r32(edi.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x40u32), eax);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x38u32), eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi);
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0x34u32), ecx);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                m.w32(esp.wrapping_add(0x48u32), eax);
                eax = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb820, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x50u32));
                edx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x58u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x58u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                eax = ecx;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x5878u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x5874u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x48u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edi);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x50u32));
                edx = m.r32(edi.wrapping_add(0xcu32));
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebp = eax;
                eax = ecx;
                ecx = m.r32(edi);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(ebp, 0x1u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x54);
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(0x134u32))), 0x12u32, 16).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64ffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x5880u32));
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ebx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x30u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                eax = ecx;
                ebp = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ebp = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = eax;
                eax = ebx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x28);
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc8u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esi.wrapping_add(0x5894u32));
                eax = m.r32(ecx.wrapping_add(0xfffffff8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                esp = esp.wrapping_add(0x4);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x5894u32));
                edx = m.r32(esi.wrapping_add(0x3864u32));
                esi = m.r32(edi);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::add(edx, esi, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x14);
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
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048DB50` ship_side_menu_animate_ship_picture
pub fn f_0048db50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x6dccf8u32);
                esp = esp.wrapping_sub(0x28);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x587cu32)), 32).1;
                if fl.cf { pc = 27; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 27; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi.wrapping_add(0x5870u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esp.wrapping_add(0xcu32);
                eax = m.r32(edx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x30u32));
                edi = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x3cu32), eax);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edi = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                esp = esp.wrapping_add(0x8);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = eax;
                eax = edi;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x28u32), ecx);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x30u32), ecx);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x3cu32));
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x5878u32));
                edi = eax;
                eax = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                ebp = m.r32(esi.wrapping_add(0x5874u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_add(0x8);
                let (r, f) = lift::sar(edi, 0x1u32, 32, fl); fl = f;
                edi = r;
                ebx = eax;
                eax = edi;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(ebx, 0x1u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = ebx;
                edx = ((eax as i32) >> 31) as u32;
                let r = edx & 0x3u32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = lift::sx8(u32::from(m.r8(0x6b9e78u32)));
                let (r, f) = lift::sar(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                m.w32(esi.wrapping_add(0x5874u32), ebp);
                ebp = edi;
                eax = lift::sx8(u32::from(m.r8(0x6b9e79u32)));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x5878u32), ecx);
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x5874u32));
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(eax, edi, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = ebx;
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(ecx, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 6; continue;
            }
            6 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::sub((edx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(0x6b9e78u32, ((edx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::sub((edx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(0x6b9e79u32, ((edx & 0xff)) as u8);
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w8(0x6b9e79u32, (0x1u32) as u8);
                pc = 8; continue;
            }
            8 => {
                eax = edi;
                ecx = m.r32(esi.wrapping_add(0x5874u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = edi;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                edx = (edx & !0xff) | ((u32::from(m.r8(0x6b9e78u32))) & 0xff);
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if (!fl.zf && fl.sf == fl.of) { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::sub(0, (edx & 0xff), 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(0x6b9e78u32, ((edx & 0xff)) as u8);
                pc = 13; continue;
            }
            13 => {
                eax = ebx;
                ecx = m.r32(esi.wrapping_add(0x5878u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ebx;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                edx = (edx & !0xff) | ((u32::from(m.r8(0x6b9e79u32))) & 0xff);
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if (!fl.zf && fl.sf == fl.of) { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if (fl.sf == fl.of) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::sub(0, (edx & 0xff), 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(0x6b9e79u32, ((edx & 0xff)) as u8);
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0x5874u32));
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.sf == fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                m.w32(esi.wrapping_add(0x5874u32), ebp);
                pc = 22; continue;
            }
            20 => {
                fl = lift::sub(eax, edi, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                m.w32(esi.wrapping_add(0x5874u32), edi);
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(esi.wrapping_add(0x5878u32));
                eax = ebx;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                m.w32(esi.wrapping_add(0x5878u32), eax);
                pc = 26; continue;
            }
            24 => {
                fl = lift::sub(ecx, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                m.w32(esi.wrapping_add(0x5878u32), ebx);
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(0x6dccf8u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 0x96u32, 32); fl = f;
                eax = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x587cu32), eax);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 27; continue;
            }
            27 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
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

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x472a10 => f_00472a10(m, h, ecx, esp),
        0x472c60 => f_00472c60(m, h, ecx, esp),
        0x473650 => f_00473650(m, h, ecx, esp),
        0x4738c0 => f_004738c0(m, h, ecx, esp),
        0x4743f0 => f_004743f0(m, h, ecx, esp),
        0x4751d0 => f_004751d0(m, h, ecx, esp),
        0x475c10 => f_00475c10(m, h, ecx, esp),
        0x476050 => f_00476050(m, h, ecx, esp),
        0x4768b0 => f_004768b0(m, h, ecx, esp),
        0x476dc0 => f_00476dc0(m, h, ecx, esp),
        0x4773a0 => f_004773a0(m, h, ecx, esp),
        0x4778a0 => f_004778a0(m, h, ecx, esp),
        0x47d520 => f_0047d520(m, h, ecx, esp),
        0x47d570 => f_0047d570(m, h, ecx, esp),
        0x47daa0 => f_0047daa0(m, h, ecx, esp),
        0x488460 => f_00488460(m, h, ecx, esp),
        0x488600 => f_00488600(m, h, ecx, esp),
        0x48ac30 => f_0048ac30(m, h, ecx, esp),
        0x48ada0 => f_0048ada0(m, h, ecx, esp),
        0x48d8c0 => f_0048d8c0(m, h, ecx, esp),
        0x48db50 => f_0048db50(m, h, ecx, esp),
        0x4ba050 => f_004ba050(m, h, ecx, esp),
        _ => return None,
    }))
}
