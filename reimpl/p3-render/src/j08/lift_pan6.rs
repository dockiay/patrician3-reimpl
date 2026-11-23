//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X005A2750` money_lender_side_menu_draw
pub fn f_005a2750(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x69b73cu32);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 0xbu32, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(0x69b73cu32);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
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

/// `0X005A3340` hanse_side_menu_draw
pub fn f_005a3340(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = m.r32(esp.wrapping_add(0x18u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7ebu32)))) & 0xff);
                let (r, f) = lift::add(edx, 0xbu32, 32); fl = f;
                edx = r;
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6dd960u32));
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
                eax = m.r32(esp.wrapping_add(0x28u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7ebu32)))) & 0xff);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6dd960u32));
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

/// `0X005A34C0` hanse_side_menu_load_layout
pub fn f_005a34c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x665152u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x14);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), ebx);
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
                edx = m.r32(ebx.wrapping_add(0xacu32));
                esi = ebx.wrapping_add(0xacu32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = ebx.wrapping_add(0x194u32);
                ebp = 0x6u32;
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
                let (r, f) = lift::add(ebx, 0x708u32, 32); fl = f;
                ebx = r;
                ebp = 0x69c18cu32;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 3; continue;
            }
            3 => {
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
                fl = lift::sub(edi, 0xffffffe8u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 6; continue;
            }
            5 => {
                edx = m.r32(edi.wrapping_add(0x6b201cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 6; continue;
            }
            6 => {
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, 0x20u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, 0xffffffe4u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c52c4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c52d0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax;
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x1cu32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x2cu32), 0xffffffffu32);
                m.w16(edx.wrapping_add(0x7e8u32), ((ecx & 0xffff)) as u16);
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

/// `0X005A3610` hanse_side_menu_prerender
pub fn f_005a3610(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp.wrapping_add(0x14u32), 0x7u32);
                pc = 1; continue;
            }
            1 => {
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
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
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

/// `0X005A3720` hanse_side_menu_layout_buttons
pub fn f_005a3720(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp.wrapping_add(0x14u32), 0x64u32);
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
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x7e8u32)))) & 0xffff);
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

/// `0X005A37C0` hanse_side_menu_refresh_button_states
pub fn f_005a37c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(eax.wrapping_add(0x468u32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7ebu32)))) & 0xff);
                let (r, f) = lift::sar(eax, (ecx & 0xff), 32, fl); fl = f;
                eax = r;
                let r = (eax & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 3; continue;
            }
            2 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 3; continue;
            }
            3 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w8(esi.wrapping_add(0x7f1u32), ((eax & 0xff)) as u8);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w8(esi.wrapping_add(0x7f2u32), ((eax & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7eeu32)))) & 0xff);
                m.w8(esi.wrapping_add(0x7efu32), (0x1u32) as u8);
                fl = lift::sub(ecx, edx, 32).1;
                m.w8(esi.wrapping_add(0x7f3u32), (0x1u32) as u8);
                m.w8(esp.wrapping_add(0xbu32), (u32::from(!fl.zf)) as u8);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w8(esi.wrapping_add(0x7eeu32), ((edx & 0xff)) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7ebu32)))) & 0xff);
                m.w8(esi.wrapping_add(0x7f4u32), (0x1u32) as u8);
                edx = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x9a2u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd748u32)), 16).1;
                if !fl.cf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6dd73cu32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(eax, 0xau32, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(eax, 0x36u32, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(0x6de508u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7ebu32)))) & 0xff);
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x9a2u32)))) & 0xffff);
                ecx = 0x6dd73cu32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x14u32))), 0x7u32, 8).1;
                if (fl.cf || fl.zf) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x7f0u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                m.w8(esi.wrapping_add(0x7f0u32), ((ebx & 0xff)) as u8);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            10 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xbu32)))) & 0xff);
                esi = m.r32(esp);
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

/// `0X005A38E0` hanse_side_menu_apply_button_states
pub fn f_005a38e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = m.r32(ebx.wrapping_add(0xacu32));
                esi = ebx.wrapping_add(0xacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                ecx = eax;
                let (r, f) = lift::shr(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esp.wrapping_add(0x10u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(edi).wrapping_add(0x7eeu32)))) & 0xff);
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
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
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x7eau32)))) & 0xff);
                fl = lift::sub(ecx, edi, 32).1;
                ecx = esi;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, 0x7u32, 32).1;
                if fl.cf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
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

/// `0X005A3980` hanse_side_menu_show
pub fn f_005a3980(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w8(edi.wrapping_add(0x7eau32), (0xffu32) as u8);
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
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x7e8u32)))) & 0xffff);
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
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x7ecu32)))) & 0xff);
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
                m.w8(edi.wrapping_add(0x7ecu32), (0x1u32) as u8);
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
                m.w32(esp, 0x6u32);
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
                m.w32(esp, 0x5u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(edi.wrapping_add(0x7edu32), (0x1u32) as u8);
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
                m.w32(esp, 0x6u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6e5558u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A3AF0` hanse_side_menu_show_buttons
pub fn f_005a3af0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = 0x7u32;
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

/// `0X005A3C00` hanse_side_menu_update
pub fn f_005a3c00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7eau32)))) & 0xff);
                m.w8(esp.wrapping_add(0xfu32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x94u32));
                edi = m.r32(0x66d4fcu32);
                edx = m.r32(0x66d504u32);
                fl = lift::sub(eax, edi, 32).1;
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::sub(eax, edx, 32).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ecx | edx; fl = lift::logic(r, 32);
                ecx = r;
                edx = m.r32(0x6e5558u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x12feu32)))) & 0xff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::logic((eax & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5a37c0, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
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
                m.w32(esp, 0x5u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(esi.wrapping_add(0x7edu32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e5558u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 46; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5a37c0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi.wrapping_add(0xacu32));
                edi = esi.wrapping_add(0xacu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x70u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x5a38e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebp = esi.wrapping_add(0xacu32);
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7edu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                m.w8(esi.wrapping_add(0x7edu32), (0x0u32) as u8);
                pc = 11; continue;
            }
            11 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7eau32)))) & 0xff);
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.zf { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w8(esi.wrapping_add(0x7eau32), ((ebx & 0xff)) as u8);
                pc = 13; continue;
            }
            13 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebx, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 14; continue;
            }
            14 => {
                pc = 18; continue;
            }
            15 => {
                edi = esi.wrapping_add(0xacu32);
                ebp = 0x7u32;
                pc = 16; continue;
            }
            16 => {
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
                if !fl.zf { pc = 16; continue; }
                pc = 17; continue;
            }
            17 => {
                m.w8(esi.wrapping_add(0x7eau32), ((ebx & 0xff)) as u8);
                pc = 18; continue;
            }
            18 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7eau32)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((eax & 0xff), 0x7u32, 8).1;
                if !fl.cf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
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
                pc = 20; continue;
            }
            20 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7eau32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xfu32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = m.r32(0x6e5558u32);
                fl = lift::sub((eax & 0xff), 0x6u32, 8).1;
                if !fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x8u32);
                pc = 24; continue;
            }
            23 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 24; continue;
            }
            24 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5c1080, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(0x6e5558u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x12feu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x440f70, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 27; continue;
            }
            27 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 32; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(0x6cc7e0u32);
                edx = m.r32(ecx.wrapping_add(0xe8u32));
                eax = m.r32(ecx.wrapping_add(0xd0u32));
                edx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x2u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 31; continue;
            }
            30 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 31; continue;
            }
            31 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            32 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 37; continue; }
                pc = 33; continue;
            }
            33 => {
                ecx = m.r32(0x6cc7e0u32);
                eax = m.r32(ecx.wrapping_add(0xe8u32));
                edx = m.r32(ecx.wrapping_add(0xd0u32));
                eax = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 36; continue;
            }
            35 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 36; continue;
            }
            36 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 37; continue;
            }
            37 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 42; continue; }
                pc = 38; continue;
            }
            38 => {
                ecx = m.r32(0x6cc7e0u32);
                edx = m.r32(ecx.wrapping_add(0xe8u32));
                eax = m.r32(ecx.wrapping_add(0xd0u32));
                edx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 41; continue;
            }
            40 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 41; continue;
            }
            41 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            42 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 46; continue; }
                pc = 43; continue;
            }
            43 => {
                ecx = m.r32(0x6cc7e0u32);
                eax = m.r32(ecx.wrapping_add(0xe8u32));
                edx = m.r32(ecx.wrapping_add(0xd0u32));
                eax = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            45 => {
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
                pc = 46; continue;
            }
            46 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
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

/// `0X005A3FE0` hanse_side_menu_open
pub fn f_005a3fe0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ebx = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x441180, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xc324u32)))) & 0xff);
                ecx = ebx;
                m.w8(ebx.wrapping_add(0x7ebu32), ((edx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x5a37c0, ecx, &a); }
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                edi = ebx.wrapping_add(0xacu32);
                pc = 1; continue;
            }
            1 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(esi).wrapping_add(0x7eeu32)))) & 0xff);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(esi, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
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

/// `0X005A4280` trading_office_side_menu_draw
pub fn f_005a4280(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6651f2u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x18);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x3cu32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ebp = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 11; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x50u32));
                edx = esp;
                ecx = esi;
                m.w32(edx, edi);
                edi = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(edx.wrapping_add(0x4u32), ebx);
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x440b20, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                m.w32(esp.wrapping_add(0x48u32), eax);
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
                eax = m.r32(0x69c97cu32);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 0xbu32, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x34u32));
                edx = m.r32(0x69c97cu32);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edi = m.r32(esp.wrapping_add(0x38u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esp.wrapping_add(0x3cu32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x80au32)))) & 0xff);
                esp = esp.wrapping_add(0x24);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ebx = ecx.wrapping_add(0x3cu32);
                edi = edx.wrapping_add(0x14u32);
                if fl.zf { pc = 8; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x38u32), eax);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x80cu32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6de4a0u32;
                m.w32(esp.wrapping_add(0x3cu32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(0x6a2468u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                let (r, f) = lift::sub(eax, 0x50u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x14u32);
                let (r, f) = lift::add(ebx, 0x14u32, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::add(edi, 0x14u32, 32); fl = f;
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x2cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = esi.wrapping_add(0x7e8u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x420a10, ecx, &a); }
                esp = esp.wrapping_add(24);
                ebp = eax;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x807u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x1u32, 8).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6a2474u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 7; continue;
            }
            6 => {
                edx = m.r32(0x6a2470u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 7; continue;
            }
            7 => {
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                esp = esp.wrapping_add(0xc);
                let (r, f) = lift::sub(ecx, 0x50u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebp, 0x14u32, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x50u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x2cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x420a10, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = esp.wrapping_add(0x38u32);
                m.w32(esp.wrapping_add(0x30u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 8; continue;
            }
            8 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(0x6cbd64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(0x6cbd5cu32);
                edx = m.r32(0x6cbd58u32);
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x20u32));
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
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x24);
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

/// `0X005A49D0` trading_office_side_menu_show_buttons
pub fn f_005a49d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5a4e00, ecx, &a); }
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
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

/// `0X005A4E00` trading_office_side_menu_refresh_buttons
pub fn f_005a4e00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x8u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebp = ecx;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x80au32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 6; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = esi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(eax.wrapping_mul(4));
                edx = m.r32(ebp.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32));
                edi = ebp.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(esi, 0x6u32, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(ebp.wrapping_add(0x80cu32));
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
                eax = m.r32(ebp.wrapping_add(0x61cu32));
                ecx = ebp.wrapping_add(0x61cu32);
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
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = esi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(eax.wrapping_mul(4));
                edx = m.r32(ebp.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32));
                ecx = ebp.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0xacu32);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 9; continue;
            }
            9 => {
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

/// `0X005A50B0` church_side_menu_draw
pub fn f_005a50b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x69feb8u32);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 0xbu32, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(0x69feb8u32);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
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

/// `0X005A5220` church_side_menu_load_layout
pub fn f_005a5220(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x665272u32);
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
                ebx = 0x3u32;
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
                ebx = ebp.wrapping_add(0x450u32);
                edi = 0x69fea8u32;
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
                fl = lift::sub(edi, 0x69feb8u32, 32).1;
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
                m.w32(esp, 0x6c5404u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c5410u32);
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
                m.w16(edx.wrapping_add(0x4d0u32), ((ecx & 0xffff)) as u16);
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

/// `0X005A5360` church_side_menu_render_buttons_to_cache
pub fn f_005a5360(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6de508u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d5u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                edi = esi.wrapping_add(0xacu32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x76cu32));
                let (r, f) = lift::shr(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                let r = (eax & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x12u32), ((eax & 0xff)) as u8);
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x14u32)), 0x1u32, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
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
                pc = 4; continue;
            }
            4 => {
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
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x4u32, 32).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 5; continue;
            }
            5 => {
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

/// `0X005A54C0` church_side_menu_draw_button_labels
pub fn f_005a54c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x4d0u32)))) & 0xffff);
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
                ebx = 0x4u32;
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

/// `0X005A5550` church_side_menu_on_show
pub fn f_005a5550(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                m.w32(esp.wrapping_add(0x8u32), eax);
                eax = m.r32(0x6e51acu32);
                m.w8(esp.wrapping_add(0x7u32), (0x1u32) as u8);
                eax = m.r32(eax.wrapping_add(0xc324u32));
                m.w8(esi.wrapping_add(0x4d5u32), ((eax & 0xff)) as u8);
                ecx = m.r32(0x6de4b0u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x76cu32));
                fl = lift::logic(((eax >> 8) & 0xff) & 0x1u32, 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w8(esp.wrapping_add(0x7u32), (0x0u32) as u8);
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = esi.wrapping_add(0xacu32);
                ecx = edi;
                m.w8(esi.wrapping_add(0x4d2u32), (0xffu32) as u8);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                ebx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), 0x64u32);
                eax = ecx;
                ecx = m.r32(esi.wrapping_add(0x14u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x14u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x4d0u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(edi);
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d3u32)))) & 0xff);
                ecx = ebx;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 10; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 5; continue;
            }
            5 => {
                edx = ebp.wrapping_mul(8);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                eax = ebp.wrapping_add(edx.wrapping_mul(4));
                edi = esi.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0xacu32);
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
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp;
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                ecx = ecx.wrapping_add(esi).wrapping_add(0x450u32);
                { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
                fl = lift::sub(ebp, 0x1u32, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x1cu32)), ebx, 32); fl = f;
                m.w32(esp.wrapping_add(0x1cu32), r);
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, 0x4u32, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 9; continue;
            }
            9 => {
                m.w8(esi.wrapping_add(0x4d3u32), (0x1u32) as u8);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_add(0x4);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d5u32)))) & 0xff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x794u32);
                { let a = []; eax = h.call_at(esp, m, 0x4fe360, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 12; continue;
            }
            12 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6de508u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d5u32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x794u32);
                { let a = []; eax = h.call_at(esp, m, 0x4fe360, ecx, &a); }
                fl = lift::sub(eax, 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 14; continue;
            }
            14 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6de508u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d5u32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x794u32);
                { let a = []; eax = h.call_at(esp, m, 0x4fe360, ecx, &a); }
                fl = lift::sub(eax, 0x2u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 16; continue;
            }
            16 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6de508u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d5u32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x794u32);
                { let a = []; eax = h.call_at(esp, m, 0x4fe360, ecx, &a); }
                fl = lift::sub(eax, 0x3u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 18; continue;
            }
            18 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6de508u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d5u32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x794u32);
                { let a = []; eax = h.call_at(esp, m, 0x4fe360, ecx, &a); }
                fl = lift::sub(eax, 0x4u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 20; continue;
            }
            20 => {
                m.w8(esi.wrapping_add(0x4d4u32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(0x6e556cu32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A57F0` church_side_menu_show_buttons
pub fn f_005a57f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = 0x4u32;
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6de508u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x4d5u32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x76cu32));
                fl = lift::logic(((eax >> 8) & 0xff) & 0x1u32, 8);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(ebx.wrapping_add(0x194u32));
                esi = ebx.wrapping_add(0x194u32);
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
                pc = 4; continue;
            }
            4 => {
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

/// `0X005A58F0` church_side_menu_on_close
pub fn f_005a58f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = edi.wrapping_add(0xacu32);
                ebx = 0x4u32;
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(edi.wrapping_add(0x4d8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a5c8u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x4u32, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(edi.wrapping_add(0x4d8u32));
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(0x66a5acu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(edi.wrapping_add(0x4d8u32));
                ecx = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a610u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi.wrapping_add(0x4d8u32));
                ecx = m.r32(0x6cbdb8u32);
                eax = m.r32(edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4430d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(edi.wrapping_add(0x4d8u32), 0x0u32);
                pc = 8; continue;
            }
            8 => {
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

/// `0X005A59A0` church_side_menu_update
pub fn f_005a59a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = m.r32(0x6e556cu32);
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
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x4d4u32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e556cu32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 33; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d2u32)))) & 0xff);
                eax = esi.wrapping_add(0xacu32);
                m.w8(esp.wrapping_add(0x13u32), ((edx & 0xff)) as u8);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                edi = eax;
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::logic(ebx & ebx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                eax = m.r32(0x6cc7e0u32);
                m.w8(eax.wrapping_add(0xfcu32), (0x0u32) as u8);
                m.w16(eax.wrapping_add(0xecu32), (0xe1u32) as u16);
                m.w8(esi.wrapping_add(0x4d4u32), (0x0u32) as u8);
                pc = 10; continue;
            }
            10 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d2u32)))) & 0xff);
                fl = lift::sub(ebx, ecx, 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w8(esi.wrapping_add(0x4d2u32), ((ebx & 0xff)) as u8);
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebx, 0x4u32, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 13; continue;
            }
            13 => {
                pc = 18; continue;
            }
            14 => {
                ebp = esi.wrapping_add(0xacu32);
                edi = 0x4u32;
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 15; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::logic(ebx & ebx, 32);
                m.w8(esi.wrapping_add(0x4d2u32), ((ebx & 0xff)) as u8);
                if !fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(0x6cc7e0u32);
                m.w8(eax.wrapping_add(0xfcu32), ((ebx & 0xff)) as u8);
                m.w16(eax.wrapping_add(0xecu32), ((ebx & 0xffff)) as u16);
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6cbdb8u32);
                { let a = []; eax = h.call_at(esp, m, 0x443110, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ebx = 0x4u32;
                edi = esi.wrapping_add(0xacu32);
                if fl.zf { pc = 21; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 19; continue; }
                pc = 20; continue;
            }
            20 => {
                pc = 22; continue;
            }
            21 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 21; continue; }
                pc = 22; continue;
            }
            22 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6de508u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d5u32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x76cu32));
                fl = lift::logic(((eax >> 8) & 0xff) & 0x1u32, 8);
                if !fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(esi.wrapping_add(0x194u32));
                ecx = esi.wrapping_add(0x194u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 24; continue;
            }
            24 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d2u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 28; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(0x6cbdb8u32);
                { let a = []; eax = h.call_at(esp, m, 0x442130, ecx, &a); }
                fl = lift::logic((eax & 0xffff) & (eax & 0xffff), 16);
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(0x6cbdb8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1bbcu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24))]; eax = h.call_at(esp, m, 0x443150, ecx, &a); }
                esp = esp.wrapping_add(28);
                m.w32(esi.wrapping_add(0x4d8u32), eax);
                pc = 27; continue;
            }
            27 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d2u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e556cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5c9d90, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x4d2u32), (0xffu32) as u8);
                m.w8(esi.wrapping_add(0x4d4u32), (0x1u32) as u8);
                pc = 28; continue;
            }
            28 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d2u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x4u32, 8).1;
                if !fl.cf { pc = 31; continue; }
                pc = 29; continue;
            }
            29 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = eax.wrapping_add(edx.wrapping_mul(4));
                edx = m.r32(esi.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0xacu32));
                ecx = esi.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0xacu32);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4d2u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                ecx = m.r32(0x6e556cu32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5c9d90, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 31; continue;
            }
            31 => {
                ecx = m.r32(0x6e556cu32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x440f70, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 33; continue;
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
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A5C80` church_side_menu_return_to_main_page
pub fn f_005a5c80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    ecx = m.r32(0x6e556cu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5c9d90, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(0x6cc7e0u32);
    { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
    ecx = m.r32(0x6cc7e0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w8(esi.wrapping_add(0x4d2u32), (0xffu32) as u8);
    m.w8(esi.wrapping_add(0x4d4u32), (0x1u32) as u8);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X005A8C40` town_hall_side_menu_draw
pub fn f_005a8c40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x69ddd0u32);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 0xbu32, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(0x69ddd0u32);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
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

/// `0X005A9BB0` barn_side_menu_release_button
pub fn f_005a9bb0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    eax = m.r32(esi.wrapping_add(0xacu32));
    ecx = esi.wrapping_add(0xacu32);
    { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = esi.wrapping_add(0x198u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
    m.w32(esi.wrapping_add(0x1d8u32), 0x0u32);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X005A9BF0` barn_side_menu_draw
pub fn f_005a9bf0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66547au32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x3c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x58u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x60u32));
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
                eax = m.r32(esp.wrapping_add(0x74u32));
                edx = esp;
                ecx = esi;
                m.w32(edx, edi);
                edi = m.r32(esp.wrapping_add(0x68u32));
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ebp = eax;
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
                eax = m.r32(0x6af500u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 0xbu32, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(0x6af500u32);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                ebx = m.r32(esp.wrapping_add(0x30u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_add(0x24);
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
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
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x1d8u32)), ebx, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, 0xffffffe2u32, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = ecx.wrapping_add(0x64u32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                let (r, f) = lift::add(edx, 0x1eu32, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, 0x28u32, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x30u32);
                m.w32(esp.wrapping_add(0x20u32), edx);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x4209e0, ecx, &a); }
                edx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x54u32), ebx);
                m.w32(esp.wrapping_add(0x5cu32), edx);
                eax = m.r32(esi.wrapping_add(0x1d8u32));
                m.w8(esp.wrapping_add(0x54u32), (0x1u32) as u8);
                fl = lift::sub(eax, 0x4u32, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(eax.wrapping_mul(4).wrapping_add(0x6afa20u32));
                ecx = esp.wrapping_add(0x5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            9 => {
                eax = m.r32(eax.wrapping_mul(4).wrapping_add(0x6afa20u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1dcu32)))) & 0xff);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6dda00u32));
                ecx = esp.wrapping_add(0x5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                ecx = esp.wrapping_add(0x24u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5aa6a0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x430f80, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x74u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x78u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x48u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x420a10, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = esp.wrapping_add(0x5cu32);
                m.w8(esp.wrapping_add(0x54u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x30u32);
                m.w32(esp.wrapping_add(0x54u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x420a00, ecx, &a); }
                pc = 11; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x48);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A9E80` barn_side_menu_load_layout
pub fn f_005a9e80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66548cu32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    edi = ecx;
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w32(esp.wrapping_add(0x14u32), 0x0u32);
    eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esp;
    m.w32(esp.wrapping_add(0x10u32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66de5cu32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(edi.wrapping_add(0xacu32));
    esi = edi.wrapping_add(0xacu32);
    ecx = esi;
    { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(esi);
    m.w32(edi.wrapping_add(0x16cu32), 0xff000000u32);
    let (r, f) = lift::add(edi, 0x198u32, 32); fl = f;
    edi = r;
    ecx = esi;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    { let t = m.r32(eax.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x1u32);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(0x6af4fcu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(0x6af4fcu32);
    edx = m.r32(esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = esi;
    { let t = m.r32(edx.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esp.wrapping_add(0x1cu32);
    m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    m.w32(0x0u32, ecx);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X005A9F40` barn_side_menu_prerender
pub fn f_005a9f40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
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
    edi = m.r32(esi.wrapping_add(0x1cu32));
    ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
    edx = m.r32(0x66d504u32);
    ebp = m.r32(esi.wrapping_add(0x14u32));
    ebx = m.r32(esi.wrapping_add(0x18u32));
    m.w32(esp.wrapping_add(0x1cu32), eax);
    eax = m.r32(esi.wrapping_add(0x94u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x1u32);
    let (r, f) = lift::sub(0, edi, 32); fl = f;
    edi = r;
    m.w32(esp.wrapping_add(0x1cu32), eax);
    m.w8(esp.wrapping_add(0x1bu32), ((ecx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x94u32), edx);
    m.w8(esi.wrapping_add(0xa4u32), (0x0u32) as u8);
    let (r, f) = lift::sub(0, ebp, 32); fl = f;
    ebp = r;
    let (r, f) = lift::sub(0, ebx, 32); fl = f;
    ebx = r;
    m.w32(esp.wrapping_add(0x2cu32), edi);
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
    ecx = m.r32(esi.wrapping_add(0x1d8u32));
    eax = m.r32(esi.wrapping_add(0xacu32));
    fl = lift::logic(ecx & ecx, 32);
    edi = esi.wrapping_add(0xacu32);
    ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = edi;
    { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(0xc);
    ecx = m.r32(esp.wrapping_add(0x30u32));
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
    eax
}

/// `0X005AA040` barn_side_menu_layout_buttons
pub fn f_005aa040(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(0xc);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    eax = esp.wrapping_add(0x4u32);
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
    edx = m.r32(esp.wrapping_add(0x8u32));
    let (r, f) = lift::sub(ecx, eax, 32); fl = f;
    ecx = r;
    let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
    ecx = r;
    let (r, f) = lift::add(edx, ecx, 32); fl = f;
    edx = r;
    ecx = esi;
    m.w32(esp.wrapping_add(0x8u32), edx);
    edx = m.r32(esi);
    { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(edi.wrapping_add(0x30u32));
    ecx = m.r32(edi.wrapping_add(0x18u32));
    edx = ((eax as i32) >> 31) as u32;
    let (r, f) = lift::sub(eax, edx, 32); fl = f;
    eax = r;
    let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
    eax = r;
    let (r, f) = lift::add(eax, ecx, 32); fl = f;
    eax = r;
    ecx = esp.wrapping_add(0x8u32);
    m.w32(esp.wrapping_add(0xcu32), eax);
    eax = m.r32(esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi;
    { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0xc);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X005AA0A0` barn_side_menu_show
pub fn f_005aa0a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = ecx;
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                esi = edi.wrapping_add(0xacu32);
                ebx = eax;
                ecx = esi;
                m.w8(edi.wrapping_add(0x1b8u32), (0xffu32) as u8);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0x2cu32));
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
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax;
                ecx = m.r32(edi.wrapping_add(0x18u32));
                let (r, f) = lift::sar(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(edi.wrapping_add(0x30u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), 0x64u32);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x1bau32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = ebx;
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
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi.wrapping_add(0x198u32);
                { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x14u32));
                m.w8(edi.wrapping_add(0x1bau32), (0x1u32) as u8);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                pc = 2; continue;
            }
            2 => {
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
                m.w32(esp, 0xbu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(edi.wrapping_add(0x1bbu32), (0x1u32) as u8);
                ecx = m.r32(0x6e55a0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cbdb8u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xbu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AA1C0` barn_side_menu_show_buttons
pub fn f_005aa1c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(0x10);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    ecx = m.r32(esi.wrapping_add(0x1d8u32));
    eax = m.r32(esi.wrapping_add(0xacu32));
    fl = lift::logic(ecx & ecx, 32);
    edi = esi.wrapping_add(0xacu32);
    ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = edi;
    { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(edi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    ecx = edi;
    { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esi.wrapping_add(0x14u32));
    ecx = m.r32(esi.wrapping_add(0x2cu32));
    edx = m.r32(esi.wrapping_add(0x30u32));
    m.w32(esp.wrapping_add(0x8u32), eax);
    let (r, f) = lift::add(ecx, eax, 32); fl = f;
    ecx = r;
    eax = m.r32(esi.wrapping_add(0x18u32));
    m.w32(esp.wrapping_add(0xcu32), eax);
    let (r, f) = lift::add(edx, eax, 32); fl = f;
    edx = r;
    eax = esp.wrapping_add(0x8u32);
    m.w32(esp.wrapping_add(0x10u32), ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = 0x6da5f0u32;
    m.w32(esp.wrapping_add(0x18u32), edx);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X005AA2E0` barn_side_menu_update
pub fn f_005aa2e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                eax = (eax & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x1b9u32)))) & 0xffff);
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
                edi = esi.wrapping_add(0xacu32);
                fl = lift::logic(eax & eax, 32);
                ecx = edi;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            2 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e55a0u32);
                ebx = m.r32(esi.wrapping_add(0x94u32));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x66d4fcu32);
                edx = m.r32(0x66d504u32);
                let (r, f) = lift::sub(0, (eax & 0xff), 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(ebx, ecx, 32).1;
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::sub(ebx, edx, 32).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ecx | edx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic((ecx & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_add(0x4);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xbu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x1bbu32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xbu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6e55a0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 26; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1b8u32)))) & 0xff);
                ecx = edi;
                m.w8(esp.wrapping_add(0xfu32), ((edx & 0xff)) as u8);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1bbu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                m.w8(esi.wrapping_add(0x1bbu32), (0x0u32) as u8);
                pc = 12; continue;
            }
            12 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1b8u32)))) & 0xff);
                fl = lift::sub(ebx, ecx, 32).1;
                if !fl.zf { pc = 16; continue; }
                pc = 13; continue;
            }
            13 => {
                m.w8(esi.wrapping_add(0x1b8u32), ((ebx & 0xff)) as u8);
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ecx, 0xe8u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebx, 0x1u32, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 15; continue;
            }
            15 => {
                pc = 17; continue;
            }
            16 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x1b8u32), ((ebx & 0xff)) as u8);
                pc = 17; continue;
            }
            17 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1b8u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x1u32, 8).1;
                if !fl.cf { pc = 20; continue; }
                pc = 18; continue;
            }
            18 => {
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1b8u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xfu32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(0x6e55a0u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5ea280, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(0x6e55a0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x440f70, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = m.r32(esi.wrapping_add(0x14u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x30u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), 0x64u32);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1b9u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = esi.wrapping_add(0x1dcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x5336b0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = eax;
                eax = m.r32(esi.wrapping_add(0x1d8u32));
                fl = lift::sub(ebx, eax, 32).1;
                if fl.zf { pc = 25; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x28u32), edx);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x1d8u32), ebx);
                ecx = m.r32(0x6e55a0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(0x6e55a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5ea280, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esi.wrapping_add(0x1d8u32));
                edx = m.r32(edi);
                fl = lift::logic(ebx & ebx, 32);
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 26; continue;
            }
            26 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
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

/// `0X005AA620` barn_side_menu_refresh_button_enabled
pub fn f_005aa620(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = (eax & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x1b9u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                ecx = esi.wrapping_add(0xacu32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AA800` weapons_smith_side_menu_draw
pub fn f_005aa800(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x6a2b2cu32);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 0xbu32, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(0x6a2b2cu32);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
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

/// `0X005AB360` town_gate_side_menu_draw
pub fn f_005ab360(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x28u32));
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
                if fl.zf { pc = 23; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 23; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x3cu32));
                edx = esp;
                ecx = esi;
                m.w32(edx, edi);
                edi = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(edx.wrapping_add(0x4u32), ebx);
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x440b20, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 23; continue; }
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
                ebp = eax;
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
                eax = m.r32(0x6a5dc4u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 0xbu32, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(0x6a5dc4u32);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x24);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f1u32)))) & 0xff);
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(8));
                eax = m.r32(0x6df270u32);
                ecx = eax.wrapping_add(edx.wrapping_mul(4));
                { let a = []; eax = h.call_at(esp, m, 0x633ae0, ecx, &a); }
                ecx = m.r32(0x6df270u32);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f1u32)))) & 0xff);
                edx = eax;
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = eax.wrapping_add(edx.wrapping_mul(8));
                edx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x8u32));
                fl = lift::sub(u32::from(m.r16(edx.wrapping_add(0x3au32))), 0x0u32, 16).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(0x6e55b8u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x94u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f1u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                edx = eax;
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                edx = eax.wrapping_add(edx.wrapping_mul(8));
                ecx = m.r32(ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x8u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x61cu32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(0x6de508u32);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x6f1u32)))) & 0xff);
                ecx = m.r32(0x6dfc14u32);
                fl = lift::sub(eax, ecx, 32).1;
                eax = m.r32(esp.wrapping_add(0x18u32));
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, 0x1eu32, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6a7024u32);
                pc = 18; continue;
            }
            9 => {
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, 0x2cu32, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, 0x1eu32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, 0xfu32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6a70f8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x3cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 19; continue;
            }
            10 => {
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(0x6de508u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x6f1u32)))) & 0xff);
                eax = m.r32(0x6dfc14u32);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                fl = lift::sub(edx, eax, 32).1;
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, 0x2cu32, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, 0x1eu32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, 0xfu32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6a70f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x3cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 19; continue;
            }
            12 => {
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, 0x2cu32, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, 0x1eu32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, 0xfu32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6a70f4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x3cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 19; continue;
            }
            13 => {
                ecx = m.r32(0x6dfc14u32);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f1u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x19u32)))) & 0xff);
                eax = m.r32(0x6dfc14u32);
                fl = lift::sub((ebx & 0xff), (ecx & 0xff), 8).1;
                if fl.zf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edx = ebx.wrapping_add(ebx.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                ebx = m.r32(0x6de508u32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x6f1u32)))) & 0xff);
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                let (r, f) = lift::sub(edx, 0x1eu32, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6a700cu32);
                pc = 18; continue;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x19u32)))) & 0xff);
                fl = lift::sub(u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0x39cu32))), 0x4u32, 8).1;
                if !fl.cf { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                let (r, f) = lift::sub(edx, 0x1eu32, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6a7010u32);
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, 0x2cu32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, 0xfu32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x3cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 19; continue;
            }
            19 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x904u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x420a10, ecx, &a); }
                esp = esp.wrapping_add(24);
                pc = 20; continue;
            }
            20 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
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
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 23; continue;
            }
            23 => {
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

/// `0X005AB6E0` town_gate_side_menu_load_layout
pub fn f_005ab6e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6655a2u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x14);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), ebx);
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
                ebp = ebx.wrapping_add(0x8f8u32);
                edi = 0x6a5da4u32;
                let (r, f) = lift::add(ebx, 0x7f0u32, 32); fl = f;
                ebx = r;
                pc = 3; continue;
            }
            3 => {
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
                m.w8(ebp, (0x1u32) as u8);
                let (r, f) = lift::add(ebx, 0x20u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(edi, 0x6a5dc4u32, 32).1;
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
                m.w32(esp, 0x6c5784u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c5790u32);
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
                m.w16(edx.wrapping_add(0x900u32), ((ecx & 0xffff)) as u16);
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

/// `0X005AB820` town_gate_side_menu_prerender
pub fn f_005ab820(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5ac240, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0xacu32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                pc = 1; continue;
            }
            1 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(eax).wrapping_add(0x8f8u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
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
                eax = m.r32(esp.wrapping_add(0x14u32));
                pc = 3; continue;
            }
            3 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(eax, 0x8u32, 32).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esp.wrapping_add(0x18u32));
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
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AB940` town_gate_side_menu_layout_buttons
pub fn f_005ab940(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                let (r, f) = lift::sub(eax, 0x2u32, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), ebx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x900u32)))) & 0xffff);
                ecx = eax;
                eax = m.r32(esi.wrapping_add(0x30u32));
                ebx = m.r32(esi.wrapping_add(0x18u32));
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(ebx).wrapping_add(0x8f8u32)))) & 0xff);
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
                fl = lift::sub(ebx, 0x8u32, 32).1;
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

/// `0X005AB9F0` town_gate_side_menu_show
pub fn f_005ab9f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ecx = edi;
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x5ac240, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0xacu32));
                esi = edi.wrapping_add(0xacu32);
                ecx = esi;
                m.w8(edi.wrapping_add(0x8f0u32), (0xffu32) as u8);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0x2cu32));
                ebx = m.r32(edi.wrapping_add(0x14u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ebp = m.r32(edi.wrapping_add(0x18u32));
                eax = ecx;
                m.w32(esp.wrapping_add(0x20u32), 0x64u32);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(edi.wrapping_add(0x8f4u32));
                let (r, f) = lift::sub(eax, 0x2u32, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x900u32)))) & 0xffff);
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
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = eax;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x8f2u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebx = edi.wrapping_add(0x7f0u32);
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ebp).wrapping_add(0x8f8u32)))) & 0xff);
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
                fl = lift::sub(ebp, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(edi.wrapping_add(0x8f2u32), (0x1u32) as u8);
                pc = 7; continue;
            }
            7 => {
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
                m.w32(esp, 0xau32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(edi.wrapping_add(0x8f3u32), (0x1u32) as u8);
                ecx = m.r32(0x6e55b8u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cbdb8u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005ABB60` town_gate_side_menu_show_buttons
pub fn f_005abb60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x5ac240, ecx, &a); }
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                esi = ebx.wrapping_add(0xacu32);
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(edi).wrapping_add(0x8f8u32)))) & 0xff);
                ecx = esi;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                eax = m.r32(esi);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, 0x8u32, 32).1;
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

/// `0X005ABC90` town_gate_side_menu_update
pub fn f_005abc90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = m.r32(0x6e55b8u32);
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
                m.w32(esp, 0xau32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x8f3u32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e55b8u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 4; continue;
            }
            4 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5abfa0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f0u32)))) & 0xff);
                edi = esi.wrapping_add(0xacu32);
                ecx = edi;
                m.w8(esp.wrapping_add(0x13u32), ((edx & 0xff)) as u8);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f3u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                m.w8(esi.wrapping_add(0x8f3u32), (0x0u32) as u8);
                pc = 9; continue;
            }
            9 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f0u32)))) & 0xff);
                fl = lift::sub(ebx, ecx, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w8(esi.wrapping_add(0x8f0u32), ((ebx & 0xff)) as u8);
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ecx, 0xe8u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebx, 0x8u32, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 12; continue;
            }
            12 => {
                pc = 16; continue;
            }
            13 => {
                ebp = edi;
                m.w32(esp.wrapping_add(0x14u32), 0x8u32);
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.zf { pc = 14; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w8(esi.wrapping_add(0x8f0u32), ((ebx & 0xff)) as u8);
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f0u32)))) & 0xff);
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f0u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6e55b8u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5efd30, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(0x6e55b8u32);
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
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                ecx = esi;
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x5ac240, ecx, &a); }
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                ebx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), 0x64u32);
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                let (r, f) = lift::sub(eax, 0x2u32, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x900u32)))) & 0xffff);
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
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(edi);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                ecx = ebx;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 22; continue;
            }
            22 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(ebp).wrapping_add(0x8f8u32)))) & 0xff);
                ecx = edi;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(edi);
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
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x1cu32)), ebx, 32); fl = f;
                m.w32(esp.wrapping_add(0x1cu32), r);
                pc = 25; continue;
            }
            24 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 25; continue;
            }
            25 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebp, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 22; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x2cu32), edx);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x24u32);
                m.w32(esp.wrapping_add(0x30u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
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
                esp = esp.wrapping_add(0x24);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005ABF40` town_gate_side_menu_open
pub fn f_005abf40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
    m.w8(esi.wrapping_add(0x8f1u32), ((edx & 0xff)) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x5abfa0, ecx, &a); }
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X005AC240` town_gate_side_menu_update_visible_buttons
pub fn f_005ac240(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f1u32)))) & 0xff);
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(8));
                eax = m.r32(0x6df270u32);
                ecx = eax.wrapping_add(edx.wrapping_mul(4));
                { let a = []; eax = h.call_at(esp, m, 0x633ae0, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f1u32)))) & 0xff);
                edx = eax;
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = eax.wrapping_add(edx.wrapping_mul(8));
                edx = m.r32(0x6df270u32);
                eax = m.r32(edx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x8u32));
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x3au32))), (ecx & 0xffff), 16).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(0x6e55b8u32);
                fl = lift::sub(u32::from(m.r8(edx.wrapping_add(0x94u32))), (ecx & 0xff), 8).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                m.w32(esi.wrapping_add(0x8f4u32), 0x3u32);
                m.w8(esi.wrapping_add(0x8f8u32), ((ecx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8f9u32), ((ecx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8fau32), ((ecx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8fbu32), ((ecx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8fcu32), ((ecx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8fdu32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8feu32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8ffu32), ((ebx & 0xff)) as u8);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            4 => {
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                m.w32(esi.wrapping_add(0x8f4u32), 0x4u32);
                m.w8(esi.wrapping_add(0x8f8u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8f9u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8fau32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8fbu32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8fcu32), ((ecx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8fdu32), ((ecx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8feu32), ((ecx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x8ffu32), ((ecx & 0xff)) as u8);
                eax = m.r32(0x6dfc14u32);
                ecx = 0x6dd73cu32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4ec694, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub((eax & 0xffff), 0xffffu32, 16).1;
                if fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6dd73cu32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0xcu32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x8f1u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x10u32)))) & 0xff);
                fl = lift::sub((edx & 0xff), (ecx & 0xff), 8).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x11u32))), (ecx & 0xff), 8).1;
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x8f4u32));
                m.w8(esi.wrapping_add(0x8fcu32), ((ebx & 0xff)) as u8);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0x8f4u32), eax);
                pc = 9; continue;
            }
            9 => {
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

/// `0X005AC570` market_hall_side_menu_draw
pub fn f_005ac570(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x6a0814u32);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 0xbu32, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(0x6a0814u32);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
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

/// `0X005AC6E0` market_hall_side_menu_load_layout
pub fn f_005ac6e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x665612u32);
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
                ebx = 0x2u32;
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
                edi = 0x69db7cu32;
                ebx = ebp.wrapping_add(0x368u32);
                pc = 3; continue;
            }
            3 => {
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
                fl = lift::sub(edi, 0x69db88u32, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esp.wrapping_add(0x30u32));
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c5824u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c5830u32);
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
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x18u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x28u32), 0xffffffffu32);
                m.w16(ebp.wrapping_add(0x3c8u32), ((ecx & 0xffff)) as u16);
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

/// `0X005AD140` shipyard_side_menu_draw
pub fn f_005ad140(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6656b0u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x1c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x40u32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ebp = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x28u32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 12; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x1cu32);
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
                eax = m.r32(esp.wrapping_add(0x54u32));
                edx = esp;
                ecx = esi;
                m.w32(edx, edi);
                edi = m.r32(esp.wrapping_add(0x48u32));
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
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                m.w32(esp.wrapping_add(0x24u32), eax);
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
                eax = m.r32(0x6a59d0u32);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 0xbu32, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x38u32));
                edx = m.r32(0x6a59d0u32);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x3cu32));
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x28);
                ecx = 0x6dcdd0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                ecx = m.r32(0x6c7cccu32);
                esp = esp.wrapping_add(0x8);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(0x6dd894u32);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ebp = eax.wrapping_add(0x10u32);
                eax = m.r32(esi.wrapping_add(0x6f4u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x34u32), ebx);
                edi = edx.wrapping_add(0xfu32);
                if !fl.cf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(0x6a59d8u32);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c58c4u32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x6e4u32);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f61f, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c58c6u32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = ebp.wrapping_add(0x1eu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = 0x6dcd28u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                eax = m.r32(0x6a59dcu32);
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = ebp.wrapping_add(0x32u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = 0x6dcdd0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x6f4u32));
                edx = m.r32(0x6dd7a4u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_add(0x4);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx).wrapping_add(0xeu32)))) & 0xff);
                eax = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0cb4u32));
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c58c8u32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6a59e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c58cau32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x3cu32), edx);
                eax = m.r32(esi.wrapping_add(0x6f4u32));
                edx = m.r32(0x6dd7a4u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w8(esp.wrapping_add(0x34u32), (0x1u32) as u8);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(eax.wrapping_add(0xe8u32));
                ecx = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x3cu32));
                fl = lift::sub(m.r32(eax.wrapping_add(0xfffffff8u32)), ebx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                m.w8(esp.wrapping_add(0x40u32), ((ecx & 0xff)) as u8);
                edx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f60a, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c58ccu32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x6f4u32));
                edx = m.r32(0x6dd7a4u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(eax.wrapping_add(0xe4u32));
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = ebp.wrapping_add(0x42u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(0x6a59e4u32);
                esp = esp.wrapping_add(0xc);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c58d0u32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x6e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f61f, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebp, 0x52u32, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = esp.wrapping_add(0x3cu32);
                m.w8(esp.wrapping_add(0x34u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 9; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x6e8u32));
                edx = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                fl = lift::logic(m.r32(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x76cu32)) & 0x20000u32, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(0x6a59d4u32);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(edx, 0x3cu32, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xa4u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(0x6cbd64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(0x6cbd5cu32);
                ecx = m.r32(0x6cbd58u32);
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x34u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 12; continue;
            }
            12 => {
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

/// `0X005AD590` shipyard_side_menu_load_layout
pub fn f_005ad590(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6656c2u32);
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
                edi = 0x6a59b8u32;
                ebp = 0xff404040u32;
                pc = 3; continue;
            }
            3 => {
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
                m.w32(esi.wrapping_add(0xc8u32), ebp);
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, 0x20u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, 0x6a59d0u32, 32).1;
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
                m.w32(esp, 0x6c58d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c58e0u32);
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
                m.w16(edx.wrapping_add(0x6f8u32), ((ecx & 0xffff)) as u16);
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

/// `0X005AD6D0` shipyard_side_menu_prerender
pub fn f_005ad6d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0x6e8u32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                ebp = m.r32(esi.wrapping_add(0x14u32));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                ebx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                m.w8(esp.wrapping_add(0x17u32), ((ecx & 0xff)) as u8);
                ecx = m.r32(0x66d504u32);
                eax = m.r32(eax.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x76cu32));
                esp = esp.wrapping_sub(0x8);
                m.w32(esi.wrapping_add(0x94u32), ecx);
                ecx = esp;
                let (r, f) = lift::shr(eax, 0x11u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                let r = (eax & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                edx = m.r32(esi);
                m.w32(ecx, ebp);
                m.w8(esp.wrapping_add(0x1eu32), ((eax & 0xff)) as u8);
                eax = m.r32(esi.wrapping_add(0x1cu32));
                m.w8(esi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                m.w32(ecx.wrapping_add(0x4u32), ebx);
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0xacu32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                pc = 1; continue;
            }
            1 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, 0x1u32, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
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
                eax = m.r32(esp.wrapping_add(0x14u32));
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(eax, 0x6u32, 32).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esp.wrapping_add(0x18u32));
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
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AD800` shipyard_side_menu_layout_buttons
pub fn f_005ad800(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0xcu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x6f8u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0xcu32), ebx);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(esi.wrapping_add(0x18u32));
                ecx = eax.wrapping_add(edx.wrapping_mul(2));
                edx = m.r32(edi);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = eax;
                esi = 0x6u32;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                ebx = eax.wrapping_add(ecx).wrapping_add(0xfffffffeu32);
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
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
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

/// `0X005AD890` shipyard_side_menu_show
pub fn f_005ad890(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(esi.wrapping_add(0x6e8u32));
                edx = m.r32(0x6de508u32);
                ebp = esi.wrapping_add(0xacu32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(ebp);
                ebx = m.r32(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x76cu32));
                ecx = ebp;
                let (r, f) = lift::shr(ebx, 0x11u32, 32, fl); fl = f;
                ebx = r;
                let r = (ebx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                m.w8(esi.wrapping_add(0x6fau32), (0xffu32) as u8);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edi = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x28u32), 0x64u32);
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x6f8u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(esi.wrapping_add(0x18u32));
                ecx = eax.wrapping_add(edx.wrapping_mul(2));
                edx = m.r32(ebp);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = eax;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx).wrapping_add(0xfffffffeu32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6fbu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                if !fl.zf { pc = 8; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = esi.wrapping_add(0x620u32);
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                edi = ebp;
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
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
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(eax, 0x1u32, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(ecx, 0x20u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(eax, 0x6u32, 32).1;
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w8(esi.wrapping_add(0x6fbu32), (0x1u32) as u8);
                pc = 8; continue;
            }
            8 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_add(0x4);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
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
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(esi.wrapping_add(0x6fcu32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(0x6e55c0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6fdu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                m.w8(esi.wrapping_add(0x6fau32), (0x0u32) as u8);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e55c0u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6fau32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5f4790, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            12 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
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

/// `0X005ADAB0` shipyard_side_menu_show_buttons
pub fn f_005adab0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(ecx.wrapping_add(0x6e8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                esi = ecx.wrapping_add(0xacu32);
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                ebx = m.r32(eax.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x76cu32));
                let (r, f) = lift::shr(ebx, 0x11u32, 32, fl); fl = f;
                ebx = r;
                let r = (ebx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 1; continue;
            }
            1 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(edi, 0x1u32, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(edi, 0x2u32, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 6; continue;
            }
            5 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, 0x6u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 7; continue;
            }
            7 => {
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

/// `0X005ADC00` shipyard_side_menu_update
pub fn f_005adc00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                ecx = m.r32(0x6e55c0u32);
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
                m.w32(esp, 0x2u32);
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
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(esi.wrapping_add(0x6fcu32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e55c0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 33; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6fau32)))) & 0xff);
                eax = esi.wrapping_add(0xacu32);
                m.w8(esp.wrapping_add(0x13u32), ((edx & 0xff)) as u8);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                edi = eax;
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6fcu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                eax = m.r32(0x6cc7e0u32);
                m.w8(eax.wrapping_add(0xfcu32), (0x0u32) as u8);
                m.w16(eax.wrapping_add(0xecu32), (0xd2u32) as u16);
                m.w8(esi.wrapping_add(0x6fcu32), (0x0u32) as u8);
                pc = 9; continue;
            }
            9 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6fau32)))) & 0xff);
                fl = lift::sub(ebx, ecx, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w8(esi.wrapping_add(0x6fau32), ((ebx & 0xff)) as u8);
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
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
                m.w8(esi.wrapping_add(0x6fau32), ((ebx & 0xff)) as u8);
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6fau32)))) & 0xff);
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6fau32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6e55c0u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5f4790, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cc7e0u32);
                m.w8(eax.wrapping_add(0xfcu32), (0x0u32) as u8);
                m.w16(eax.wrapping_add(0xecu32), (0x104u32) as u16);
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(0x6e55c0u32);
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
                eax = m.r32(esi.wrapping_add(0x6e8u32));
                edx = m.r32(0x6de508u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = edx.wrapping_add(ecx.wrapping_mul(8));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x52afc0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x6f0u32)), 32).1;
                if !fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(esi.wrapping_add(0x6e8u32));
                edi = m.r32(0x6de508u32);
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x81cu32)))) & 0xffff);
                edx = m.r32(esi.wrapping_add(0x6f4u32));
                fl = lift::sub(edx, ecx, 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esi.wrapping_add(0x6f0u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x28u32), edx);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x6e8u32));
                edx = m.r32(0x6de508u32);
                eax = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x6u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx).wrapping_add(0x81cu32)))) & 0xffff);
                m.w32(esi.wrapping_add(0x6f4u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6de508u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x52afc0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esp.wrapping_add(0x14u32);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                m.w32(esi.wrapping_add(0x6ecu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5311d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0d00u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x6e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c58f4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                edx = esp.wrapping_add(0x14u32);
                eax = esp.wrapping_add(0x18u32);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esi.wrapping_add(0x6f0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5311d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0d00u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x6e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c5900u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(esi.wrapping_add(0x6e8u32));
                eax = m.r32(0x6dfc14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x504ac0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = eax;
                ecx = esi.wrapping_add(0x534u32);
                fl = lift::sub(u32::from(m.r16(edi)), 0x0u32, 16).1;
                if fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 27; continue;
            }
            26 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 27; continue;
            }
            27 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x6e8u32));
                edx = m.r32(0x6dfc14u32);
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x504ac0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = eax;
                ecx = esi.wrapping_add(0xacu32);
                fl = lift::sub(u32::from(m.r16(edi)), 0x0u32, 16).1;
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 30; continue;
            }
            29 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 30; continue;
            }
            30 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x6e8u32));
                ecx = m.r32(0x6dfc14u32);
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x504ac0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = eax;
                let (r, f) = lift::add(esi, 0x44cu32, 32); fl = f;
                esi = r;
                ecx = esi;
                fl = lift::sub(u32::from(m.r16(edi)), 0x0u32, 16).1;
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
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
            32 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 33; continue;
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
                esp = esp.wrapping_add(0x20);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005AE0A0` shipyard_side_menu_open
pub fn f_005ae0a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x24u32));
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x441180, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6e51acu32);
                fl = lift::logic(edi & edi, 32);
                ecx = m.r32(eax.wrapping_add(0xc324u32));
                m.w8(esi.wrapping_add(0x6fdu32), (0x1u32) as u8);
                m.w32(esi.wrapping_add(0x6e8u32), ecx);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w8(esi.wrapping_add(0x6fdu32), (0x0u32) as u8);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x6e8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(edx.wrapping_mul(8));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x52afc0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x6f0u32)), 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0x6e8u32));
                edi = m.r32(0x6de508u32);
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x81cu32)))) & 0xffff);
                edx = m.r32(esi.wrapping_add(0x6f4u32));
                fl = lift::sub(edx, ecx, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esi.wrapping_add(0x6f0u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x18u32), edx);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x6e8u32));
                edx = m.r32(0x6de508u32);
                eax = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x6u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx).wrapping_add(0x81cu32)))) & 0xffff);
                m.w32(esi.wrapping_add(0x6f4u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6de508u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x52afc0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esp.wrapping_add(0x24u32);
                ecx = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                m.w32(esi.wrapping_add(0x6ecu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5311d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x8u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0d00u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x6e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c590cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                edx = esp.wrapping_add(0x24u32);
                eax = esp.wrapping_add(0x8u32);
                ecx = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esi.wrapping_add(0x6f0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5311d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x8u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0d00u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x6e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c5918u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0x6e8u32));
                eax = m.r32(0x6dfc14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x504ac0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = eax;
                ecx = esi.wrapping_add(0x534u32);
                fl = lift::sub(u32::from(m.r16(edi)), 0x0u32, 16).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            7 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x6e8u32));
                edx = m.r32(0x6dfc14u32);
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x504ac0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = eax;
                ecx = esi.wrapping_add(0xacu32);
                fl = lift::sub(u32::from(m.r16(edi)), 0x0u32, 16).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            10 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x6e8u32));
                ecx = m.r32(0x6dfc14u32);
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x504ac0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = eax;
                ecx = esi.wrapping_add(0x44cu32);
                fl = lift::sub(u32::from(m.r16(edi)), 0x0u32, 16).1;
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            13 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            14 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x194u32));
                ecx = esi.wrapping_add(0x194u32);
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x27cu32));
                ecx = esi.wrapping_add(0x27cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x364u32));
                ecx = esi.wrapping_add(0x364u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x18);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x5a2750 => f_005a2750(m, h, ecx, esp),
        0x5a28c0 => f_005a28c0(m, h, ecx, esp),
        0x5a2a00 => f_005a2a00(m, h, ecx, esp),
        0x5a2b20 => f_005a2b20(m, h, ecx, esp),
        0x5a2bf0 => f_005a2bf0(m, h, ecx, esp),
        0x5a2d60 => f_005a2d60(m, h, ecx, esp),
        0x5a2e80 => f_005a2e80(m, h, ecx, esp),
        0x5a3340 => f_005a3340(m, h, ecx, esp),
        0x5a34c0 => f_005a34c0(m, h, ecx, esp),
        0x5a3610 => f_005a3610(m, h, ecx, esp),
        0x5a3720 => f_005a3720(m, h, ecx, esp),
        0x5a37c0 => f_005a37c0(m, h, ecx, esp),
        0x5a38e0 => f_005a38e0(m, h, ecx, esp),
        0x5a3980 => f_005a3980(m, h, ecx, esp),
        0x5a3af0 => f_005a3af0(m, h, ecx, esp),
        0x5a3c00 => f_005a3c00(m, h, ecx, esp),
        0x5a3fe0 => f_005a3fe0(m, h, ecx, esp),
        0x5a4280 => f_005a4280(m, h, ecx, esp),
        0x5a4690 => f_005a4690(m, h, ecx, esp),
        0x5a4800 => f_005a4800(m, h, ecx, esp),
        0x5a4890 => f_005a4890(m, h, ecx, esp),
        0x5a49d0 => f_005a49d0(m, h, ecx, esp),
        0x5a4ad0 => f_005a4ad0(m, h, ecx, esp),
        0x5a4da0 => f_005a4da0(m, h, ecx, esp),
        0x5a4e00 => f_005a4e00(m, h, ecx, esp),
        0x5a50b0 => f_005a50b0(m, h, ecx, esp),
        0x5a5220 => f_005a5220(m, h, ecx, esp),
        0x5a5360 => f_005a5360(m, h, ecx, esp),
        0x5a54c0 => f_005a54c0(m, h, ecx, esp),
        0x5a5550 => f_005a5550(m, h, ecx, esp),
        0x5a57f0 => f_005a57f0(m, h, ecx, esp),
        0x5a58f0 => f_005a58f0(m, h, ecx, esp),
        0x5a59a0 => f_005a59a0(m, h, ecx, esp),
        0x5a5c80 => f_005a5c80(m, h, ecx, esp),
        0x5a8c40 => f_005a8c40(m, h, ecx, esp),
        0x5a8db0 => f_005a8db0(m, h, ecx, esp),
        0x5a8ed0 => f_005a8ed0(m, h, ecx, esp),
        0x5a9010 => f_005a9010(m, h, ecx, esp),
        0x5a90b0 => f_005a90b0(m, h, ecx, esp),
        0x5a91f0 => f_005a91f0(m, h, ecx, esp),
        0x5a9340 => f_005a9340(m, h, ecx, esp),
        0x5a9710 => f_005a9710(m, h, ecx, esp),
        0x5a9bb0 => f_005a9bb0(m, h, ecx, esp),
        0x5a9bf0 => f_005a9bf0(m, h, ecx, esp),
        0x5a9e80 => f_005a9e80(m, h, ecx, esp),
        0x5a9f40 => f_005a9f40(m, h, ecx, esp),
        0x5aa040 => f_005aa040(m, h, ecx, esp),
        0x5aa0a0 => f_005aa0a0(m, h, ecx, esp),
        0x5aa1c0 => f_005aa1c0(m, h, ecx, esp),
        0x5aa2e0 => f_005aa2e0(m, h, ecx, esp),
        0x5aa620 => f_005aa620(m, h, ecx, esp),
        0x5aa800 => f_005aa800(m, h, ecx, esp),
        0x5aa970 => f_005aa970(m, h, ecx, esp),
        0x5aaa30 => f_005aaa30(m, h, ecx, esp),
        0x5aabe0 => f_005aabe0(m, h, ecx, esp),
        0x5aac80 => f_005aac80(m, h, ecx, esp),
        0x5aada0 => f_005aada0(m, h, ecx, esp),
        0x5aaf70 => f_005aaf70(m, h, ecx, esp),
        0x5ab0f0 => f_005ab0f0(m, h, ecx, esp),
        0x5ab360 => f_005ab360(m, h, ecx, esp),
        0x5ab6e0 => f_005ab6e0(m, h, ecx, esp),
        0x5ab820 => f_005ab820(m, h, ecx, esp),
        0x5ab940 => f_005ab940(m, h, ecx, esp),
        0x5ab9f0 => f_005ab9f0(m, h, ecx, esp),
        0x5abb60 => f_005abb60(m, h, ecx, esp),
        0x5abc90 => f_005abc90(m, h, ecx, esp),
        0x5abf40 => f_005abf40(m, h, ecx, esp),
        0x5ac240 => f_005ac240(m, h, ecx, esp),
        0x5ac570 => f_005ac570(m, h, ecx, esp),
        0x5ac6e0 => f_005ac6e0(m, h, ecx, esp),
        0x5ac810 => f_005ac810(m, h, ecx, esp),
        0x5ac930 => f_005ac930(m, h, ecx, esp),
        0x5ac9c0 => f_005ac9c0(m, h, ecx, esp),
        0x5acb10 => f_005acb10(m, h, ecx, esp),
        0x5acc30 => f_005acc30(m, h, ecx, esp),
        0x5ad140 => f_005ad140(m, h, ecx, esp),
        0x5ad590 => f_005ad590(m, h, ecx, esp),
        0x5ad6d0 => f_005ad6d0(m, h, ecx, esp),
        0x5ad800 => f_005ad800(m, h, ecx, esp),
        0x5ad890 => f_005ad890(m, h, ecx, esp),
        0x5adab0 => f_005adab0(m, h, ecx, esp),
        0x5adc00 => f_005adc00(m, h, ecx, esp),
        0x5ae0a0 => f_005ae0a0(m, h, ecx, esp),
        _ => return None,
    }))
}
