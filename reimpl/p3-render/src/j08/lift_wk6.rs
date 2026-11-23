//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

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

/// `0X005A4690` trading_office_side_menu_prerender
pub fn f_005a4690(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                edi = m.r32(esi.wrapping_add(0x18u32));
                ebp = m.r32(esi.wrapping_add(0x1cu32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                edx = m.r32(0x66d504u32);
                ebx = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::sub(0, edi, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w8(esp.wrapping_add(0x1bu32), ((ecx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x94u32), edx);
                m.w8(esi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x28u32), edi);
                m.w32(esp.wrapping_add(0x2cu32), ebp);
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
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ecx, ebx);
                m.w32(ecx.wrapping_add(0x4u32), edi);
                m.w32(ecx.wrapping_add(0x8u32), ebp);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x80au32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 2; continue;
            }
            2 => {
                eax = ebp.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                ecx = ebp.wrapping_add(eax.wrapping_mul(4));
                edx = m.r32(esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32));
                edi = esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(ebp, 0x6u32, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x80cu32));
                edx = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                fl = lift::logic(u32::from(m.r8(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2c8u32))) & 0x40u32, 8);
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x61cu32));
                ecx = esi.wrapping_add(0x61cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            5 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(0xc);
                ecx = esp;
                eax = m.r32(edi);
                m.w32(ecx, ebx);
                m.w32(ecx.wrapping_add(0x4u32), edx);
                edx = m.r32(esp.wrapping_add(0x30u32));
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
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 7; continue;
            }
            7 => {
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
                esp = esp.wrapping_add(0x18);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A4800` trading_office_side_menu_layout_buttons
pub fn f_005a4800(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x804u32)))) & 0xffff);
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
                ebx = 0x7u32;
                ecx = esi;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
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

/// `0X005A4890` trading_office_side_menu_show
pub fn f_005a4890(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w8(edi.wrapping_add(0x806u32), (0xffu32) as u8);
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
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x804u32)))) & 0xffff);
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
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x808u32)))) & 0xff);
                ecx = ebx;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
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
                ebp = edi.wrapping_add(0x708u32);
                m.w32(esp.wrapping_add(0x10u32), 0x7u32);
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
                m.w8(edi.wrapping_add(0x808u32), (0x1u32) as u8);
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
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(edi.wrapping_add(0x809u32), (0x1u32) as u8);
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
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6e557cu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A4AD0` trading_office_side_menu_update
pub fn f_005a4ad0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x80cu32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6e557cu32);
                edi = m.r32(esi.wrapping_add(0x94u32));
                ebp = eax;
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x66d4fcu32);
                ebx = m.r32(0x66d504u32);
                let (r, f) = lift::sub(0, (eax & 0xff), 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(edi, ecx, 32).1;
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
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x809u32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e557cu32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 30; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x807u32)))) & 0xff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x2f0u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                m.w8(esi.wrapping_add(0x807u32), ((eax & 0xff)) as u8);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x807u32)))) & 0xff);
                let r = (edx & 0xff) ^ (edx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), (eax & 0xff), 8).1;
                let (r, f) = lift::sbb(ecx, ecx, fl.cf, 32); fl = f;
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x80au32)))) & 0xff);
                let (r, f) = lift::sub(0, ecx, 32); fl = f;
                ecx = r;
                fl = lift::sub(ecx, edx, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                eax = (eax & !0xff) | ((u32::from((!fl.cf && !fl.zf))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                m.w8(esi.wrapping_add(0x80au32), ((eax & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5a4e00, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x80au32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 24; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x806u32)))) & 0xff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w8(esp.wrapping_add(0x13u32), ((ecx & 0xff)) as u8);
                ebp = esi.wrapping_add(0xacu32);
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 16; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x809u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                m.w8(esi.wrapping_add(0x809u32), (0x0u32) as u8);
                pc = 14; continue;
            }
            14 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x806u32)))) & 0xff);
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.zf { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w8(esi.wrapping_add(0x806u32), ((ebx & 0xff)) as u8);
                pc = 16; continue;
            }
            16 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebx, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 11; continue; }
                pc = 17; continue;
            }
            17 => {
                pc = 21; continue;
            }
            18 => {
                edi = esi.wrapping_add(0xacu32);
                ebp = 0x7u32;
                pc = 19; continue;
            }
            19 => {
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
                if !fl.zf { pc = 19; continue; }
                pc = 20; continue;
            }
            20 => {
                m.w8(esi.wrapping_add(0x806u32), ((ebx & 0xff)) as u8);
                pc = 21; continue;
            }
            21 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x806u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x7u32, 8).1;
                if !fl.cf { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x806u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(0x6e557cu32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5d9a20, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 24; continue;
            }
            24 => {
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
                if !fl.zf { pc = 27; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(0x6cc7e0u32);
                edx = m.r32(ecx.wrapping_add(0xe8u32));
                eax = m.r32(ecx.wrapping_add(0xd0u32));
                edx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x1u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
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
                pc = 27; continue;
            }
            27 => {
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
                if !fl.zf { pc = 30; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(0x6cc7e0u32);
                eax = m.r32(ecx.wrapping_add(0xe8u32));
                edx = m.r32(ecx.wrapping_add(0xd0u32));
                eax = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                fl = lift::sub(u32::from(m.r8(eax)), 0x0u32, 8).1;
                if !fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
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
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A4DA0` trading_office_side_menu_open
pub fn f_005a4da0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    eax = m.r32(esp.wrapping_add(0x4u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x441180, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(0x6e51acu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    edx = m.r32(ecx.wrapping_add(0xc324u32));
    m.w32(esi.wrapping_add(0x80cu32), edx);
    ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
    eax = (eax & !0xffff) | (((edx & 0xffff)) & 0xffff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = 0x6de4a0u32;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
    esp = esp.wrapping_add(12);
    eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2f0u32)))) & 0xffff);
    fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
    edx = (edx & !0xff) | ((u32::from((!fl.cf && !fl.zf))) & 0xff);
    m.w8(esi.wrapping_add(0x807u32), ((eax & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x80au32), ((edx & 0xff)) as u8);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X005A8DB0` town_hall_side_menu_load_layout
pub fn f_005a8db0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6653d2u32);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = ecx;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), 0x0u32);
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
                edx = m.r32(ebx.wrapping_add(0xacu32));
                esi = ebx.wrapping_add(0xacu32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = ebx.wrapping_add(0x194u32);
                ebp = 0x7u32;
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
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                ebp = 0xff000000u32;
                pc = 3; continue;
            }
            3 => {
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(eax.wrapping_mul(4));
                eax = ebx.wrapping_add(ecx.wrapping_mul(8));
                m.w32(eax.wrapping_add(0x16cu32), ebp);
                edi = eax.wrapping_add(0xacu32);
                eax = esi;
                edx = m.r32(edi);
                let (r, f) = lift::shl(eax, 0x5u32, 32, fl); fl = f;
                eax = r;
                ecx = eax.wrapping_add(ebx).wrapping_add(0x7f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(esi, 0x2u32, 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esi.wrapping_mul(4).wrapping_add(0x69ddb4u32));
                pc = 10; continue;
            }
            5 => {
                fl = lift::sub(esi, 0x6u32, 32).1;
                if (fl.sf == fl.of) { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_mul(4).wrapping_add(0x69ddb8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 11; continue;
            }
            7 => {
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6a94f8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx.wrapping_add(0x61cu32);
                pc = 12; continue;
            }
            9 => {
                edx = m.r32(0x6a9630u32);
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 11; continue;
            }
            11 => {
                ecx = edi;
                pc = 12; continue;
            }
            12 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = esp.wrapping_add(0x24u32);
                m.w32(esp.wrapping_add(0x1cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
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
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A8ED0` town_hall_side_menu_prerender
pub fn f_005a8ed0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                edi = m.r32(esi.wrapping_add(0x18u32));
                ebp = m.r32(esi.wrapping_add(0x1cu32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                edx = m.r32(0x66d504u32);
                ebx = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::sub(0, edi, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w8(esp.wrapping_add(0x1bu32), ((ecx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x94u32), edx);
                m.w8(esi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x28u32), edi);
                m.w32(esp.wrapping_add(0x2cu32), ebp);
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
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ecx, ebx);
                m.w32(ecx.wrapping_add(0x4u32), edi);
                m.w32(ecx.wrapping_add(0x8u32), ebp);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(edi, 0x7u32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f5u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6de52eu32))) & 0xff);
                fl = lift::sub((eax & 0xff), (ecx & 0xff), 8).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = edi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::sub(ecx, edi, 32); fl = f;
                ecx = r;
                edx = edi.wrapping_add(ecx.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32));
                ecx = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                ecx = edi.wrapping_mul(8);
                esp = esp.wrapping_sub(0xc);
                let (r, f) = lift::sub(ecx, edi, 32); fl = f;
                ecx = r;
                edx = edi.wrapping_add(ecx.wrapping_mul(4));
                ecx = esp;
                ebp = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32);
                edx = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(ecx, ebx);
                eax = m.r32(ebp);
                m.w32(ecx.wrapping_add(0x4u32), edx);
                edx = m.r32(esp.wrapping_add(0x30u32));
                m.w32(ecx.wrapping_add(0x8u32), edx);
                ecx = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x94u32), eax);
                m.w8(esi.wrapping_add(0xa4u32), ((ecx & 0xff)) as u8);
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
                esp = esp.wrapping_add(0x18);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A9010` town_hall_side_menu_layout_buttons
pub fn f_005a9010(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = esp.wrapping_add(0x18u32);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(0x6dcdd0u32);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dcdd0u32;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ebx = m.r32(edi.wrapping_add(0x18u32));
                edx = m.r32(esi);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = eax;
                ebx = 0x8u32;
                let (r, f) = lift::sar(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(edx, edi, 32); fl = f;
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
                esp = esp.wrapping_add(0x18);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A90B0` town_hall_side_menu_show
pub fn f_005a90b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w8(edi.wrapping_add(0x8f4u32), (0xffu32) as u8);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = m.r32(edi.wrapping_add(0x14u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x20u32);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(0x6dcdd0u32);
                ecx = 0x6dcdd0u32;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x24u32));
                ebx = m.r32(edi.wrapping_add(0x18u32));
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x1cu32), 0x64u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                let (r, f) = lift::sar(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x8f6u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = edi.wrapping_add(0x7f0u32);
                m.w32(esp.wrapping_add(0x10u32), 0x8u32);
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
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
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
                m.w8(edi.wrapping_add(0x8f6u32), (0x1u32) as u8);
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
                m.w32(esp, 0x7u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(edi.wrapping_add(0x8f7u32), (0x1u32) as u8);
                ecx = m.r32(0x6e558cu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cbdb8u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x7u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_add(0x20);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A91F0` town_hall_side_menu_show_buttons
pub fn f_005a91f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(esi, 0x7u32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x8f5u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6de52eu32))) & 0xff);
                fl = lift::sub((eax & 0xff), (ecx & 0xff), 8).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = edi.wrapping_add(0x704u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 5; continue;
            }
            4 => {
                eax = esi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(eax.wrapping_mul(4));
                ecx = edi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32);
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(eax.wrapping_mul(4));
                edx = m.r32(edi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32));
                ecx = edi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 6; continue;
            }
            6 => {
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

/// `0X005A9340` town_hall_side_menu_update
pub fn f_005a9340(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5a9780, ecx, &a); }
                ecx = m.r32(0x6e558cu32);
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
                m.w32(esp, 0x7u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x8f7u32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x7u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e558cu32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 40; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f4u32)))) & 0xff);
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f7u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                m.w8(esi.wrapping_add(0x8f7u32), (0x0u32) as u8);
                pc = 9; continue;
            }
            9 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f4u32)))) & 0xff);
                fl = lift::sub(ebx, ecx, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w8(esi.wrapping_add(0x8f4u32), ((ebx & 0xff)) as u8);
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebx, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 12; continue;
            }
            12 => {
                pc = 16; continue;
            }
            13 => {
                edi = esi.wrapping_add(0xacu32);
                ebp = 0x8u32;
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
                m.w8(esi.wrapping_add(0x8f4u32), ((ebx & 0xff)) as u8);
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f4u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x8u32, 8).1;
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f4u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6e558cu32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5e0ba0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(0x6e558cu32);
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
                ecx = m.r32(eax.wrapping_add(0xd0u32));
                edx = m.r32(ecx.wrapping_add(edx.wrapping_mul(4)));
                fl = lift::sub(u32::from(m.r8(edx)), 0x0u32, 8).1;
                if !fl.zf { pc = 29; continue; }
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
                eax = m.r32(0x6cc7e0u32);
                pc = 29; continue;
            }
            29 => {
                ecx = m.r32(eax.wrapping_add(0xe8u32));
                edx = m.r32(eax.wrapping_add(0xd0u32));
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 32; continue; }
                pc = 30; continue;
            }
            30 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x66a7d0u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
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
                pc = 32; continue;
            }
            32 => {
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
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x2u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                if !fl.zf { pc = 35; continue; }
                pc = 33; continue;
            }
            33 => {
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 38; continue; }
                pc = 34; continue;
            }
            34 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 37; continue;
            }
            35 => {
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 38; continue; }
                pc = 36; continue;
            }
            36 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 37; continue;
            }
            37 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                pc = 38; continue;
            }
            38 => {
                eax = m.r32(esi.wrapping_add(0x8f0u32));
                edx = m.r32(0x6dccf8u32);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.cf { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb38u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = m.r32(0x6dccf8u32);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, 0x2710u32, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x8f0u32), ecx);
                pc = 40; continue;
            }
            40 => {
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

/// `0X005A9710` town_hall_side_menu_open
pub fn f_005a9710(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    eax = m.r32(esp.wrapping_add(0x4u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x441180, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(0x6e51acu32);
    edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xc324u32)))) & 0xff);
    ecx = esi;
    m.w8(esi.wrapping_add(0x8f5u32), ((edx & 0xff)) as u8);
    eax = m.r32(0x6dccf8u32);
    let (r, f) = lift::add(eax, 0x1388u32, 32); fl = f;
    eax = r;
    m.w32(esi.wrapping_add(0x8f0u32), eax);
    { let a = []; eax = h.call_at(esp, m, 0x5a9780, ecx, &a); }
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X005AA970` weapons_smith_side_menu_load_layout
pub fn f_005aa970(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x665502u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                edi = 0x6aa33cu32;
                esi = ecx.wrapping_add(0xacu32);
                ebx = ecx.wrapping_add(0x280u32);
                pc = 1; continue;
            }
            1 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                m.w32(esi.wrapping_add(0xc0u32), 0xff000000u32);
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
                fl = lift::sub(edi, 0x6aa344u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = esp.wrapping_add(0x1cu32);
                m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0xcu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AAA30` weapons_smith_side_menu_prerender
pub fn f_005aaa30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esi.wrapping_add(0x1cu32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                edx = m.r32(0x66d504u32);
                ebp = m.r32(esi.wrapping_add(0x14u32));
                ebx = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::sub(0, edi, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w8(esp.wrapping_add(0x1bu32), ((ecx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x94u32), edx);
                m.w8(esi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x30u32), edi);
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
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ecx, ebp);
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w32(ecx.wrapping_add(0x8u32), edi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x2c1u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                edi = m.r32(esi.wrapping_add(0xacu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi.wrapping_add(0xacu32);
                { let t = m.r32(edi.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2c1u32)))) & 0xff);
                edi = m.r32(0x6de508u32);
                eax = ecx;
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x6f1u32)))) & 0xff);
                edi = m.r32(0x6dfc14u32);
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = (ecx & !0xffff) | (((ecx & 0xff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(0x6dfc14u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(0x6de52du32))) & 0xff);
                fl = lift::sub(edx, eax, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 5; continue;
            }
            4 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0x194u32));
                ecx = esi.wrapping_add(0x194u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = esi.wrapping_add(0xacu32);
                m.w32(esp.wrapping_add(0x14u32), 0x2u32);
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
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.zf { pc = 6; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
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
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AABE0` weapons_smith_side_menu_layout_buttons
pub fn f_005aabe0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = m.r32(esp.wrapping_add(0xcu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x18u32);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(0x6dcd28u32);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dcd28u32;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x30u32));
                ebx = m.r32(esi.wrapping_add(0x18u32));
                esi = m.r32(esp.wrapping_add(0x1cu32));
                ecx = edi;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esi = eax;
                ebx = 0x2u32;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(esi, eax, 32); fl = f;
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
                esp = esp.wrapping_add(0x18);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AAC80` weapons_smith_side_menu_show
pub fn f_005aac80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                esi = ebx.wrapping_add(0xacu32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                ecx = esi;
                m.w8(ebx.wrapping_add(0x2c0u32), (0xffu32) as u8);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(ebx.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = m.r32(ebx.wrapping_add(0x14u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x24u32);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(0x6dcd28u32);
                ecx = 0x6dcd28u32;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebx.wrapping_add(0x30u32));
                ebp = m.r32(ebx.wrapping_add(0x18u32));
                edi = m.r32(esp.wrapping_add(0x28u32));
                ecx = esi;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), 0x64u32);
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x2c2u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = ebx.wrapping_add(0x280u32);
                m.w32(esp.wrapping_add(0x10u32), 0x2u32);
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
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(edi, 0x20u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if !fl.zf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w8(ebx.wrapping_add(0x2c2u32), (0x1u32) as u8);
                pc = 4; continue;
            }
            4 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6e55a8u32);
                esp = esp.wrapping_add(0x4);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
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

/// `0X005AADA0` weapons_smith_side_menu_show_buttons
pub fn f_005aada0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebp = 0x2u32;
                edi = ebx.wrapping_add(0xacu32);
                esi = edi;
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xffff) | ((u32::from(m.r8(ebx.wrapping_add(0x2c1u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esi = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let t = m.r32(esi.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x2c1u32)))) & 0xff);
                esi = m.r32(0x6de508u32);
                eax = ecx;
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x6f1u32)))) & 0xff);
                esi = m.r32(0x6dfc14u32);
                fl = lift::sub(eax, esi, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = (ecx & !0xffff) | (((ecx & 0xff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6dfc14u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(0x6de52du32))) & 0xff);
                fl = lift::sub(edx, eax, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ebx.wrapping_add(0x194u32));
                ecx = ebx.wrapping_add(0x194u32);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
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
            6 => {
                edx = m.r32(ebx.wrapping_add(0x194u32));
                ecx = ebx.wrapping_add(0x194u32);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
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

/// `0X005AAF70` weapons_smith_side_menu_update
pub fn f_005aaf70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                ecx = m.r32(0x6e55a8u32);
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
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6e55a8u32);
                esp = esp.wrapping_add(0x4);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2c0u32)))) & 0xff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w8(esp.wrapping_add(0x13u32), ((edx & 0xff)) as u8);
                ebp = esi.wrapping_add(0xacu32);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2c0u32)))) & 0xff);
                fl = lift::sub(ebx, ecx, 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(esi.wrapping_add(0x2c0u32), ((ebx & 0xff)) as u8);
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebx, 0x2u32, 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 8; continue;
            }
            8 => {
                pc = 12; continue;
            }
            9 => {
                edi = esi.wrapping_add(0xacu32);
                ebp = 0x2u32;
                pc = 10; continue;
            }
            10 => {
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
                if !fl.zf { pc = 10; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w8(esi.wrapping_add(0x2c0u32), ((ebx & 0xff)) as u8);
                pc = 12; continue;
            }
            12 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2c0u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.cf { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2c0u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(0x6e55a8u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5ebd70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 15; continue;
            }
            15 => {
                ecx = m.r32(0x6e55a8u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x440f70, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 17; continue;
            }
            17 => {
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
            _ => unreachable!(),
        }
    }
}

/// `0X005AB0F0` weapons_smith_side_menu_open
pub fn f_005ab0f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = m.r32(0x66d4f8u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                ecx = m.r32(0x6e51acu32);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xc324u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x2c1u32), ((edx & 0xff)) as u8);
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

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x4cb770 => f_004cb770(m, h, ecx, esp),
        0x4ccaa0 => f_004ccaa0(m, h, ecx, esp),
        0x5a4690 => f_005a4690(m, h, ecx, esp),
        0x5a4800 => f_005a4800(m, h, ecx, esp),
        0x5a4890 => f_005a4890(m, h, ecx, esp),
        0x5a4ad0 => f_005a4ad0(m, h, ecx, esp),
        0x5a4da0 => f_005a4da0(m, h, ecx, esp),
        0x5a8db0 => f_005a8db0(m, h, ecx, esp),
        0x5a8ed0 => f_005a8ed0(m, h, ecx, esp),
        0x5a9010 => f_005a9010(m, h, ecx, esp),
        0x5a90b0 => f_005a90b0(m, h, ecx, esp),
        0x5a91f0 => f_005a91f0(m, h, ecx, esp),
        0x5a9340 => f_005a9340(m, h, ecx, esp),
        0x5a9710 => f_005a9710(m, h, ecx, esp),
        0x5aa970 => f_005aa970(m, h, ecx, esp),
        0x5aaa30 => f_005aaa30(m, h, ecx, esp),
        0x5aabe0 => f_005aabe0(m, h, ecx, esp),
        0x5aac80 => f_005aac80(m, h, ecx, esp),
        0x5aada0 => f_005aada0(m, h, ecx, esp),
        0x5aaf70 => f_005aaf70(m, h, ecx, esp),
        0x5ab0f0 => f_005ab0f0(m, h, ecx, esp),
        _ => return None,
    }))
}
