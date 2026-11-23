//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X004410F0` window_base44_on_close_finished
pub fn f_004410f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                ecx = m.r32(0x6cbd8cu32);
                eax = m.r32(0x6cbd90u32);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = eax;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbd8cu32);
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbd8cu32);
                m.w32(0x6cbd8cu32, 0x0u32);
                m.w32(0x6cbd90u32, ecx);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = []; return h.call_at(esp, m, t, ecx, &a); }
            }
            4 => {
                fl = lift::sub(eax, m.r32(0x6ce6d0u32), 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x697a54u32))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6cbb40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42d750, ecx, &a); }
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            7 => {
                ecx = m.r32(0x6dfc14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4f5500, ecx, &a); }
                esp = esp.wrapping_add(20);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004415D0` window_base44_update
pub fn f_004415d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, 0x4u32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = 0x6cbbc8u32;
                m.w32(esi.wrapping_add(0x94u32), 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4af970, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x12cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 6; continue;
            }
            3 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = 0x6cbbc8u32;
                m.w32(esi.wrapping_add(0x94u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4af970, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x128u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(edi, 0x4u32, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(edi, 0x2u32, 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                eax = m.r32(esi.wrapping_add(0x9cu32));
                m.w32(esi.wrapping_add(0xa8u32), ecx);
                edx = m.r32(0x6dccf0u32);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                m.w32(esp.wrapping_add(0x10u32), edx);
                ebx = m.r32(esi.wrapping_add(0x30u32));
                { let v = lift::fild64(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                m.w32(esp.wrapping_add(0x18u32), ebx);
                lift::fst32(m, esp.wrapping_add(0xcu32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(0x6cbd98u32) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d / v); }
                { let v = f64::from(m.r32(esp.wrapping_add(0x18u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                fl = lift::sub(edi, 0x4u32, 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esi.wrapping_add(0x98u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w32(esi.wrapping_add(0x98u32), ebx);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            11 => {
                fl = lift::sub(edi, 0x2u32, 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                m.w32(esi.wrapping_add(0x98u32), ebx);
                if !fl.sf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                m.w32(esi.wrapping_add(0x98u32), 0x0u32);
                pc = 14; continue;
            }
            14 => {
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

/// `0X00464C80` viper_button_draw
pub fn f_00464c80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x48);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x60u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebx = m.r32(esp.wrapping_add(0x64u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                ebp = m.r32(esp.wrapping_add(0x30u32));
                eax = edi.wrapping_add(ecx);
                ecx = ebx.wrapping_add(edx);
                edx = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x40u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x44u32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 27; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 27; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x68u32));
                ebp = m.r32(esp.wrapping_add(0x74u32));
                edx = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(edx, edi);
                m.w32(edx.wrapping_add(0x4u32), ebx);
                m.w32(edx.wrapping_add(0x8u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4b18f0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                edx = m.r32(esi);
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                ecx = esi;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                { let t = m.r32(edx.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x94u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x94u32));
                ecx = esp;
                edx = m.r32(eax);
                m.w32(ecx, edi);
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w32(ecx.wrapping_add(0x8u32), ebp);
                ecx = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax;
                pc = 6; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(ecx);
                eax = esp;
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebx);
                m.w32(eax.wrapping_add(0x8u32), ebp);
                eax = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 6; continue;
            }
            6 => {
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0xb4u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                edx = ecx.wrapping_mul(8).wrapping_add(0x6dcd54u32);
                eax = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd54u32));
                m.w32(esp.wrapping_add(0x60u32), eax);
                eax = esp.wrapping_add(0x20u32);
                ebp = m.r32(edx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = m.r32(edx.wrapping_add(0x8u32));
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x6cu32), ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xb0u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 11; continue; }
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0xccu32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 12; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0xccu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                pc = 13; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                pc = 13; continue;
            }
            13 => {
                eax = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffff0000u32);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x28u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0xd0u32));
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x4);
                ecx = esi;
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let t = m.r32(edx.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, 0x3u32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 19; continue; }
                pc = 14; continue;
            }
            14 => {
                match m.r32(eax.wrapping_mul(4).wrapping_add(0x464f20u32)) { 0x464e41 => { pc = 15; continue; } 0x464e4a => { pc = 16; continue; } 0x464e53 => { pc = 17; continue; } _ => unreachable!() }
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0xc0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 18; continue;
            }
            16 => {
                ecx = m.r32(esi.wrapping_add(0xc8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 18; continue;
            }
            17 => {
                edx = m.r32(esi.wrapping_add(0xc4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 18; continue;
            }
            18 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x60u32), ebp);
                { let t = m.r32(eax.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, 0x2u32, 32).1;
                if !fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(esi.wrapping_add(0xe8u32));
                eax = m.r32(esi.wrapping_add(0xecu32));
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                pc = 21; continue;
            }
            21 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xb8u32)), 0x1u32, 32).1;
                if !fl.zf { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ebp = 0x1u32;
                if !fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                ebp = m.r32(esp.wrapping_add(0x60u32));
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esi.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(0xa8u32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(m.r32(eax.wrapping_add(0xfffffff8u32)), ebp, 32).1;
                if fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(esi.wrapping_add(0xb0u32));
                edx = m.r32(esp.wrapping_add(0x5cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xb4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                eax = ecx.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4c1780, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x54u32), ecx);
                ecx = esp.wrapping_add(0x48u32);
                m.w32(esp.wrapping_add(0x4cu32), ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x4cu32), ebp);
                m.w32(esp.wrapping_add(0x54u32), edx);
                { let t = m.r32(eax.wrapping_add(0xe0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 27; continue;
            }
            27 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x48);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0046F9B0` choice_list_dialog_open
pub fn f_0046f9b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ecx = m.r32(0x6cbdd8u32);
                edi = eax;
                m.w32(esp.wrapping_add(0x10u32), edi);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cbdd8u32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esi.wrapping_add(0xecu32), 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa4u32)))) & 0xffff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub((eax & 0xffff), 0x4u32, 16).1;
                m.w16(esi.wrapping_add(0xdau32), ((ebx & 0xffff)) as u16);
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6dcf20u32;
                eax = m.r32(eax.wrapping_mul(4).wrapping_add(0x6c10c8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w16(esi.wrapping_add(0xe4u32), ((eax & 0xffff)) as u16);
                pc = 3; continue;
            }
            2 => {
                m.w16(esi.wrapping_add(0xe4u32), ((ebx & 0xffff)) as u16);
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi.wrapping_add(0x58cu32));
                ecx = esi.wrapping_add(0x58cu32);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(eax, 0x5u32, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe4u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w16(esi.wrapping_add(0xe4u32), ((eax & 0xffff)) as u16);
                pc = 5; continue;
            }
            5 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa4u32)))) & 0xffff);
                fl = lift::sub(ecx, 0x4u32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 40; continue; }
                pc = 6; continue;
            }
            6 => {
                match m.r32(ecx.wrapping_mul(4).wrapping_add(0x47041cu32)) { 0x46fa75 => { pc = 7; continue; } 0x46fae6 => { pc = 12; continue; } 0x46fba5 => { pc = 18; continue; } 0x46fc16 => { pc = 23; continue; } 0x46fd3f => { pc = 33; continue; } _ => unreachable!() }
            }
            7 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 31; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esi.wrapping_add(0xacu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(edi)))) & 0xff);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6c0bb0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe4u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                m.w16(esi.wrapping_add(0xe4u32), ((eax & 0xffff)) as u16);
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::add(u32::from(m.r16(esi.wrapping_add(0xdau32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esi.wrapping_add(0xdau32), (r) as u16);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 11; continue;
            }
            11 => {
                pc = 31; continue;
            }
            12 => {
                ecx = esi;
                m.w32(esi.wrapping_add(0xacu32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x4721f0, ecx, &a); }
                ebp = m.r32(esi.wrapping_add(0xacu32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 16; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(ebp.wrapping_add(edi.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe6u32)))) & 0xffff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe4u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                m.w16(esi.wrapping_add(0xe4u32), ((eax & 0xffff)) as u16);
                pc = 15; continue;
            }
            15 => {
                let (r, f) = lift::add(u32::from(m.r16(esi.wrapping_add(0xdau32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esi.wrapping_add(0xdau32), (r) as u16);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 13; continue; }
                pc = 16; continue;
            }
            16 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = esi.wrapping_add(0x938u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe4u32)))) & 0xffff);
                m.w32(ecx.wrapping_add(0x170u32), eax);
                m.w8(ecx.wrapping_add(0x179u32), (0x1u32) as u8);
                eax = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 40; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4caad0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xccu32));
                edi = esi.wrapping_add(0x678u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x58u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f60a, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 40; continue;
            }
            18 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 31; continue; }
                pc = 19; continue;
            }
            19 => {
                edx = m.r32(esi.wrapping_add(0xacu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(edi)))) & 0xff);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6dda00u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe4u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                m.w16(esi.wrapping_add(0xe4u32), ((eax & 0xffff)) as u16);
                pc = 21; continue;
            }
            21 => {
                let (r, f) = lift::add(u32::from(m.r16(esi.wrapping_add(0xdau32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esi.wrapping_add(0xdau32), (r) as u16);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 19; continue; }
                pc = 22; continue;
            }
            22 => {
                pc = 31; continue;
            }
            23 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                m.w16(esi.wrapping_add(0xdau32), ((ebx & 0xffff)) as u16);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 31; continue; }
                pc = 24; continue;
            }
            24 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 31; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(esi.wrapping_add(0xacu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                let r = (edx & 0xffff) ^ (edx & 0xffff); fl = lift::logic(r, 16);
                edx = (edx & !0xffff) | ((r) & 0xffff);
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ecx)))) & 0xff);
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if !fl.cf { pc = 30; continue; }
                pc = 26; continue;
            }
            26 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = eax;
                eax = m.r32(ebp.wrapping_add(0xe8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe6u32)))) & 0xffff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe6u32)))) & 0xffff);
                edx = m.r32(ebp.wrapping_add(0xe4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::add(edi, 0x14u32, 32); fl = f;
                edi = r;
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6de4b0u32);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x19u32)))) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6dda00u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                pc = 28; continue;
            }
            28 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe4u32)))) & 0xffff);
                fl = lift::sub(edi, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                m.w16(esi.wrapping_add(0xe4u32), ((edi & 0xffff)) as u16);
                pc = 30; continue;
            }
            30 => {
                let (r, f) = lift::add(u32::from(m.r16(esi.wrapping_add(0xdau32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esi.wrapping_add(0xdau32), (r) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xa6u32))), 16).1;
                if fl.cf { pc = 25; continue; }
                pc = 31; continue;
            }
            31 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xc4u32)), ebx, 32).1;
                if fl.zf { pc = 40; continue; }
                pc = 32; continue;
            }
            32 => {
                let (r, f) = lift::add(u32::from(m.r16(esi.wrapping_add(0xdau32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esi.wrapping_add(0xdau32), (r) as u16);
                pc = 40; continue;
            }
            33 => {
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub((edi & 0xffff), (ebx & 0xffff), 16).1;
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                ecx = 0x1u32;
                pc = 35; continue;
            }
            35 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa8u32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), (ebx & 0xffff), 16).1;
                if fl.zf { pc = 37; continue; }
                pc = 36; continue;
            }
            36 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                pc = 37; continue;
            }
            37 => {
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xaau32)))) & 0xffff);
                let (r, f) = lift::add((ebp & 0xffff), (ecx & 0xffff), 16); fl = f;
                ebp = (ebp & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                ecx = edi.wrapping_add(ebp);
                fl = lift::sub((ecx & 0xffff), 0x14u32, 16).1;
                m.w16(esi.wrapping_add(0xdau32), ((ecx & 0xffff)) as u16);
                if (fl.cf || fl.zf) { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                m.w16(esi.wrapping_add(0xdau32), (0x14u32) as u16);
                pc = 39; continue;
            }
            39 => {
                m.w16(esi.wrapping_add(0xe4u32), ((eax & 0xffff)) as u16);
                m.w32(esi.wrapping_add(0xc4u32), ebx);
                pc = 40; continue;
            }
            40 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xacu32)), ebx, 32).1;
                if fl.zf { pc = 48; continue; }
                pc = 41; continue;
            }
            41 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 48; continue; }
                pc = 42; continue;
            }
            42 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa4u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.zf { pc = 44; continue; }
                pc = 43; continue;
            }
            43 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x471f10, ecx, &a); }
                pc = 48; continue;
            }
            44 => {
                fl = lift::sub((eax & 0xffff), 0x1u32, 16).1;
                if !fl.zf { pc = 46; continue; }
                pc = 45; continue;
            }
            45 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x471ff0, ecx, &a); }
                pc = 48; continue;
            }
            46 => {
                fl = lift::sub((eax & 0xffff), 0x2u32, 16).1;
                if !fl.zf { pc = 48; continue; }
                pc = 47; continue;
            }
            47 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4720d0, ecx, &a); }
                pc = 48; continue;
            }
            48 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6cbb00u32))) & 0xffff);
                edx = esp.wrapping_add(0x40u32);
                m.w16(esi.wrapping_add(0xe0u32), ((ecx & 0xffff)) as u16);
                eax = m.r32(0x6dcf20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dcf20u32;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = (ecx & !0xffff) | (((eax & 0xffff)) & 0xffff);
                let (r, f) = lift::shr((ecx & 0xffff), 0x1u32, 16, fl); fl = f;
                ecx = (ecx & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(esi.wrapping_add(0xe2u32), ((eax & 0xffff)) as u16);
                edx = m.r32(0x6cbb00u32);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                m.w16(esi.wrapping_add(0xe2u32), ((eax & 0xffff)) as u16);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe4u32)))) & 0xffff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe2u32)))) & 0xffff);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(m.r32(0x6cbb00u32) as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edx = ebx;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w16(esi.wrapping_add(0xe4u32), ((eax & 0xffff)) as u16);
                let (r, f) = lift::imul(eax, m.r32(0x6cbb00u32)); fl = f;
                eax = r;
                m.w16(esi.wrapping_add(0xe4u32), ((eax & 0xffff)) as u16);
                edi = m.r32(0x6cbb00u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebp = eax.wrapping_add(edi.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x34u32), ebp);
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(edi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edx = ebp;
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                m.w16(esi.wrapping_add(0xdcu32), ((edx & 0xffff)) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6cbb00u32))) & 0xffff);
                m.w16(esi.wrapping_add(0xdeu32), ((edx & 0xffff)) as u16);
                let (r, f) = lift::add(eax, 0x3u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::imul(eax, edi); fl = f;
                eax = r;
                edi = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                fl = lift::sub(eax, edi, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 50; continue; }
                pc = 49; continue;
            }
            49 => {
                eax = edi;
                edi = m.r32(0x6cbb00u32);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(edi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edx = edi.wrapping_add(edi);
                let (r, f) = lift::imul(eax, edi); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x38u32), eax);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                edi = m.r32(esp.wrapping_add(0x14u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w16(esi.wrapping_add(0xdau32), ((eax & 0xffff)) as u16);
                pc = 50; continue;
            }
            50 => {
                eax = ebp.wrapping_add(ebp.wrapping_mul(2));
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(0x6cbdd8u32);
                m.w32(esp.wrapping_add(0x28u32), eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x70u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::sub(edi, ecx, 32); fl = f;
                edi = r;
                ecx = esp.wrapping_add(0x28u32);
                let (r, f) = lift::sar(edi, 0x1u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x30u32), eax);
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cbaf4u32);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 53; continue; }
                pc = 51; continue;
            }
            51 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 53; continue; }
                pc = 52; continue;
            }
            52 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x40u32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x44u32), ecx);
                eax = m.r32(edx.wrapping_add(0x8u32));
                edx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x4cu32), edx);
                pc = 54; continue;
            }
            53 => {
                eax = m.r32(esp.wrapping_add(0x48u32));
                pc = 54; continue;
            }
            54 => {
                edi = eax.wrapping_add(eax);
                eax = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x34u32));
                edx = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x41c5a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x462390, ecx, &a); }
                ebx = m.r32(esi.wrapping_add(0x14u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe0u32)))) & 0xffff);
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = esi.wrapping_add(0xf8u32);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x50u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x461fd0, ecx, &a); }
                edi = m.r32(esi.wrapping_add(0x2cu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe0u32)))) & 0xffff);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x58u32), eax);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe2u32)))) & 0xffff);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x54u32), eax);
                let (r, f) = lift::imul(edx, ecx); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x40u32);
                m.w32(esp.wrapping_add(0x5cu32), edx);
                edx = esp.wrapping_add(0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a4a4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                edx = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x460ef0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic((eax & 0xffff) & (eax & 0xffff), 16);
                if fl.zf { pc = 56; continue; }
                pc = 55; continue;
            }
            55 => {
                edi = 0x1u32;
                pc = 56; continue;
            }
            56 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa8u32)))) & 0xffff);
                fl = lift::logic((ecx & 0xffff) & (ecx & 0xffff), 16);
                if fl.zf { pc = 58; continue; }
                pc = 57; continue;
            }
            57 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                pc = 58; continue;
            }
            58 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xaau32)))) & 0xffff);
                fl = lift::logic((edx & 0xffff) & (edx & 0xffff), 16);
                if fl.zf { pc = 60; continue; }
                pc = 59; continue;
            }
            59 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                pc = 60; continue;
            }
            60 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(edi, 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 68; continue; }
                pc = 61; continue;
            }
            61 => {
                ebp = m.r32(esi.wrapping_add(0xc4u32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 63; continue; }
                pc = 62; continue;
            }
            62 => {
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 64; continue;
            }
            63 => {
                eax = eax.wrapping_add(ecx).wrapping_add(0xffffffffu32);
                pc = 64; continue;
            }
            64 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0xf0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc4u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 66; continue; }
                pc = 65; continue;
            }
            65 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa8u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xaau32)))) & 0xffff);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                pc = 67; continue;
            }
            66 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xaau32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa8u32)))) & 0xffff);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = ecx.wrapping_add(eax).wrapping_add(0xffffffffu32);
                pc = 67; continue;
            }
            67 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                ecx = (ecx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                eax = ecx;
                pc = 73; continue;
            }
            68 => {
                ecx = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 70; continue; }
                pc = 69; continue;
            }
            69 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                pc = 70; continue;
            }
            70 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0xf0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 72; continue; }
                pc = 71; continue;
            }
            71 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ecx, eax, 32).1;
                edx = (edx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                eax = edx;
                pc = 73; continue;
            }
            72 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xa6u32))), 16).1;
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                pc = 73; continue;
            }
            73 => {
                edx = m.r32(ebx);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x10u32));
                edi = esi.wrapping_add(0x4a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x58cu32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe0u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x30u32), 0x64u32);
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x28u32), edx);
                edx = eax.wrapping_add(ecx).wrapping_add(0xffffffd1u32);
                eax = m.r32(edi);
                ecx = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x2cu32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0x938u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                ebx = esi.wrapping_add(0x680u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = esi.wrapping_add(0x768u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = esi.wrapping_add(0x850u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(esp.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                ecx = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                let (r, f) = lift::add(ecx, 0x14u32, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w32(esp.wrapping_add(0x24u32), edx);
                edx = m.r32(ebx);
                eax = esp.wrapping_add(0x1cu32);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = ecx.wrapping_add(eax).wrapping_add(0x2u32);
                eax = m.r32(esi.wrapping_add(0x768u32));
                ecx = esi.wrapping_add(0x768u32);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x768u32));
                ecx = esi.wrapping_add(0x768u32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = ecx.wrapping_add(eax).wrapping_add(0x2u32);
                eax = m.r32(esi.wrapping_add(0x850u32));
                ecx = esi.wrapping_add(0x850u32);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(ebp);
                let (r, f) = lift::add(eax, 0x14u32, 32); fl = f;
                eax = r;
                ecx = ebp;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(ebp);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x1cu32);
                let (r, f) = lift::sub(ecx, 0x3u32, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(edi);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(ebp);
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esp.wrapping_add(0x18u32));
                ecx = ebp;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                { let t = m.r32(eax.wrapping_add(0x58u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa4u32))), 0x4u32, 16).1;
                eax = m.r32(esi.wrapping_add(0x768u32));
                ecx = esi.wrapping_add(0x768u32);
                m.w8(esi.wrapping_add(0x674u32), (0x0u32) as u8);
                if !fl.zf { pc = 77; continue; }
                pc = 74; continue;
            }
            74 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x850u32));
                ecx = esi.wrapping_add(0x850u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 78; continue; }
                pc = 75; continue;
            }
            75 => {
                eax = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cc94au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 78; continue; }
                pc = 76; continue;
            }
            76 => {
                m.w8(esi.wrapping_add(0x674u32), (0x1u32) as u8);
                pc = 78; continue;
            }
            77 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x850u32));
                ecx = esi.wrapping_add(0x850u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 78; continue;
            }
            78 => {
                ecx = m.r32(esi.wrapping_add(0x14u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe0u32)))) & 0xffff);
                edx = m.r32(esi.wrapping_add(0x58cu32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                edi = esi.wrapping_add(0x58cu32);
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esi.wrapping_add(0xc0u32));
                edx = m.r32(edi);
                fl = lift::logic(ebx & ebx, 32);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc0u32));
                edx = m.r32(edi);
                fl = lift::logic(eax & eax, 32);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xd0u32));
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c1190u32));
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc8u32));
                m.w32(esi.wrapping_add(0xd4u32), 0x1u32);
                fl = lift::logic(eax & eax, 32);
                m.w16(esi.wrapping_add(0xd8u32), (0xffffu32) as u16);
                if fl.zf { pc = 80; continue; }
                pc = 79; continue;
            }
            79 => {
                m.w32(eax, 0x0u32);
                pc = 80; continue;
            }
            80 => {
                m.w32(esi.wrapping_add(0xad8u32), 0xffffffffu32);
                m.w8(esi.wrapping_add(0xadcu32), (0x0u32) as u8);
                m.w8(esi.wrapping_add(0x67cu32), (0x0u32) as u8);
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

/// `0X00470F80` choice_list_dialog_on_overwrite_answer
pub fn f_00470f80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = m.r32(esi);
                ecx = esi;
                m.w8(esi.wrapping_add(0xadcu32), (0x0u32) as u8);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x768u32));
                ecx = esi.wrapping_add(0x768u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x680u32));
                ecx = esi.wrapping_add(0x680u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x4a4u32));
                ecx = esi.wrapping_add(0x4a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x850u32));
                ecx = esi.wrapping_add(0x850u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x938u32));
                edi = esi.wrapping_add(0x938u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1084u32));
                ecx = m.r32(0x66bdc0u32);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                esi = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(esi, 0x0u32);
                pc = 3; continue;
            }
            3 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w32(eax, 0x2u32);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4d2ec0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xccu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esi);
                ecx = esi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = [m.r32(esp.wrapping_add(4))]; return h.call_at(esp, m, t, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00471420` choice_list_dialog_draw
pub fn f_00471420(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65f43au32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0xac);
                eax = m.r32(0x6c7cccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                m.w32(esp.wrapping_add(0x28u32), eax);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                ebx = m.r32(esp.wrapping_add(0xd4u32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                ebp = m.r32(esp.wrapping_add(0xd0u32));
                edi = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0xc4u32), 0x0u32);
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 113; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 113; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0xccu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esp.wrapping_add(0x34u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                edx = esp.wrapping_add(0x18u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x41caa0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6dcf20u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                eax = m.r32(0x66e0c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(0x66e0ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa4u32)))) & 0xffff);
                esp = esp.wrapping_add(0x10);
                fl = lift::sub((eax & 0xffff), 0x4u32, 16).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6c10c8u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdeu32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = eax.wrapping_add(ecx).wrapping_add(0x1u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdcu32)))) & 0xffff);
                edx = eax.wrapping_add(ecx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 5; continue;
            }
            4 => {
                eax = m.r32(0x6b0550u32);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdeu32)))) & 0xffff);
                eax = ecx.wrapping_add(edx).wrapping_add(0x1u32);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdcu32)))) & 0xffff);
                eax = ecx.wrapping_add(edx).wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 5; continue;
            }
            5 => {
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0xc);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe6u32)))) & 0xffff);
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
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x20u32));
                edi = m.r32(esp.wrapping_add(0x24u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe0u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe2u32)))) & 0xffff);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x49cu32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe4u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x20u32), edx);
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x1cu32), edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x24u32), edi);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa4u32)))) & 0xffff);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_add(0x8);
                let (r, f) = lift::sub(edi, ebp, 32); fl = f;
                edi = r;
                fl = lift::sub(edx, 0x4u32, 32).1;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                if (!fl.cf && !fl.zf) { pc = 112; continue; }
                pc = 6; continue;
            }
            6 => {
                match m.r32(edx.wrapping_mul(4).wrapping_add(0x471e48u32)) { 0x47161e => { pc = 7; continue; } 0x47172c => { pc = 22; continue; } 0x47184f => { pc = 39; continue; } 0x471a43 => { pc = 68; continue; } 0x471d75 => { pc = 104; continue; } _ => unreachable!() }
            }
            7 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), 0x0u32, 16).1;
                if (fl.cf || fl.zf) { pc = 17; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 17; continue; }
                pc = 9; continue;
            }
            9 => {
                ebp = m.r32(esi.wrapping_add(0x494u32));
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 18; continue; }
                pc = 10; continue;
            }
            10 => {
                ebx = m.r32(esp.wrapping_add(0x14u32));
                pc = 11; continue;
            }
            11 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                fl = lift::sub(ebp, edx, 32).1;
                if (fl.sf == fl.of) { pc = 18; continue; }
                pc = 12; continue;
            }
            12 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(ebp, eax, 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xacu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ebp)))) & 0xff);
                edx = esp.wrapping_add(0x1cu32);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6c0bb0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 15; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ebp)))) & 0xff);
                eax = esp.wrapping_add(0x18u32);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0bb0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 11; continue; }
                pc = 16; continue;
            }
            16 => {
                pc = 18; continue;
            }
            17 => {
                ebp = m.r32(esp.wrapping_add(0x10u32));
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 112; continue; }
                pc = 19; continue;
            }
            19 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(ebp, edx, 32).1;
                if !fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 21; continue;
            }
            21 => {
                ecx = m.r32(0x6c1078u32);
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 112; continue;
            }
            22 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), 0x0u32, 16).1;
                if (fl.cf || fl.zf) { pc = 32; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 32; continue; }
                pc = 24; continue;
            }
            24 => {
                ebp = m.r32(esi.wrapping_add(0x494u32));
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 33; continue; }
                pc = 25; continue;
            }
            25 => {
                ebx = m.r32(esp.wrapping_add(0x14u32));
                pc = 26; continue;
            }
            26 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                fl = lift::sub(ebp, edx, 32).1;
                if (fl.sf == fl.of) { pc = 33; continue; }
                pc = 27; continue;
            }
            27 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(ebp, eax, 32).1;
                if !fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xacu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ebp)))) & 0xff);
                edx = esp.wrapping_add(0x1cu32);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6dda00u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 30; continue;
            }
            29 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ebp)))) & 0xff);
                eax = esp.wrapping_add(0x18u32);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6dda00u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 30; continue;
            }
            30 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 26; continue; }
                pc = 31; continue;
            }
            31 => {
                pc = 33; continue;
            }
            32 => {
                ebp = m.r32(esp.wrapping_add(0x10u32));
                pc = 33; continue;
            }
            33 => {
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 112; continue; }
                pc = 34; continue;
            }
            34 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                fl = lift::sub(ebp, edx, 32).1;
                if !fl.zf { pc = 112; continue; }
                pc = 35; continue;
            }
            35 => {
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 112; continue; }
                pc = 36; continue;
            }
            36 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(ebp, eax, 32).1;
                if !fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                ecx = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 38; continue;
            }
            38 => {
                edx = m.r32(0x6c1078u32);
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 112; continue;
            }
            39 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), 0x0u32, 16).1;
                if (fl.cf || fl.zf) { pc = 64; continue; }
                pc = 40; continue;
            }
            40 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 64; continue; }
                pc = 41; continue;
            }
            41 => {
                ebp = m.r32(esi.wrapping_add(0x494u32));
                fl = lift::sub(edi, ecx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ebp);
                if (fl.sf == fl.of) { pc = 64; continue; }
                pc = 42; continue;
            }
            42 => {
                ebx = m.r32(esp.wrapping_add(0x14u32));
                pc = 43; continue;
            }
            43 => {
                ecx = m.r32(esi.wrapping_add(0xacu32));
                let r = (edx & 0xffff) ^ (edx & 0xffff); fl = lift::logic(r, 16);
                edx = (edx & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx))) & 0xff);
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if fl.cf { pc = 47; continue; }
                pc = 44; continue;
            }
            44 => {
                m.w8(ecx, (0xffu32) as u8);
                eax = m.r32(0x6c1088u32);
                ecx = esp.wrapping_add(0x3cu32);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                pc = 45; continue;
            }
            45 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                m.w8(ecx.wrapping_add(eax), ((edx & 0xff)) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 45; continue; }
                pc = 46; continue;
            }
            46 => {
                pc = 58; continue;
            }
            47 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(eax.wrapping_add(0xe8u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ebp))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 51; continue; }
                pc = 48; continue;
            }
            48 => {
                m.w8(esp.wrapping_add(ecx).wrapping_add(0x3cu32), ((edx & 0xff)) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x1u32)))) & 0xff);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 48; continue; }
                pc = 49; continue;
            }
            49 => {
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 51; continue; }
                pc = 50; continue;
            }
            50 => {
                m.w8(esp.wrapping_add(ecx).wrapping_add(0x3cu32), (0x20u32) as u8);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                pc = 51; continue;
            }
            51 => {
                ebp = m.r32(eax.wrapping_add(0xe4u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(ebp))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 53; continue; }
                pc = 52; continue;
            }
            52 => {
                m.w8(esp.wrapping_add(ecx).wrapping_add(0x3cu32), ((edx & 0xff)) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x1u32)))) & 0xff);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 52; continue; }
                pc = 53; continue;
            }
            53 => {
                ebp = m.r32(0x6de4b0u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x19u32)))) & 0xff);
                fl = lift::sub(edx, ebp, 32).1;
                if !fl.cf { pc = 57; continue; }
                pc = 54; continue;
            }
            54 => {
                m.w8(esp.wrapping_add(ecx).wrapping_add(0x3cu32), (0x20u32) as u8);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w8(esp.wrapping_add(ecx).wrapping_add(0x3cu32), (0x28u32) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x19u32)))) & 0xff);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                edx = m.r32(edx.wrapping_mul(4).wrapping_add(0x6dda00u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 56; continue; }
                pc = 55; continue;
            }
            55 => {
                m.w8(esp.wrapping_add(ecx).wrapping_add(0x3cu32), ((eax & 0xff)) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x1u32)))) & 0xff);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 55; continue; }
                pc = 56; continue;
            }
            56 => {
                m.w8(esp.wrapping_add(ecx).wrapping_add(0x3cu32), (0x29u32) as u8);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                pc = 57; continue;
            }
            57 => {
                m.w8(esp.wrapping_add(ecx).wrapping_add(0x3cu32), (0x0u32) as u8);
                pc = 58; continue;
            }
            58 => {
                ebp = m.r32(esp.wrapping_add(0x10u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(ebp, eax, 32).1;
                if !fl.zf { pc = 60; continue; }
                pc = 59; continue;
            }
            59 => {
                ecx = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = esp.wrapping_add(0x40u32);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 63; continue;
            }
            60 => {
                ecx = m.r32(esi.wrapping_add(0xacu32));
                eax = m.r32(0x6dfc14u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebp)))) & 0xff);
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 62; continue; }
                pc = 61; continue;
            }
            61 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff0000ffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = esp.wrapping_add(0x40u32);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 63; continue;
            }
            62 => {
                edx = esp.wrapping_add(0x3cu32);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 63; continue;
            }
            63 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(edi, eax, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ebp);
                if (fl.sf != fl.of) { pc = 43; continue; }
                pc = 64; continue;
            }
            64 => {
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 112; continue; }
                pc = 65; continue;
            }
            65 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0xd8u32))), 16).1;
                if !fl.zf { pc = 67; continue; }
                pc = 66; continue;
            }
            66 => {
                eax = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 67; continue;
            }
            67 => {
                ecx = m.r32(0x6c1088u32);
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 112; continue;
            }
            68 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                ebp = m.r32(esi.wrapping_add(0x494u32));
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(ebp, edx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ebp);
                ebx = (ebx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), (edx & 0xffff), 16).1;
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                fl = lift::logic(edx & ebx, 32);
                if fl.zf { pc = 73; continue; }
                pc = 69; continue;
            }
            69 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = ebp;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 71; continue; }
                pc = 70; continue;
            }
            70 => {
                ecx = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(0x6a0cb4u32);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 72; continue;
            }
            71 => {
                edx = m.r32(0x6a0cb4u32);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 72; continue;
            }
            72 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                pc = 73; continue;
            }
            73 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                ebx = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic((edx & 0xffff) & (edx & 0xffff), 16);
                if fl.zf { pc = 77; continue; }
                pc = 74; continue;
            }
            74 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x494u32)), 0x2u32, 32).1;
                if (fl.sf == fl.of) { pc = 76; continue; }
                pc = 75; continue;
            }
            75 => {
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esp.wrapping_add(0x10u32), ebx);
                pc = 76; continue;
            }
            76 => {
                fl = lift::logic((edx & 0xffff) & (edx & 0xffff), 16);
                if !fl.zf { pc = 78; continue; }
                pc = 77; continue;
            }
            77 => {
                ebp = ebx;
                pc = 81; continue;
            }
            78 => {
                eax = m.r32(esi.wrapping_add(0x494u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 80; continue; }
                pc = 79; continue;
            }
            79 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 81; continue;
            }
            80 => {
                ebp = ebx.wrapping_add(0xfffffffeu32);
                pc = 81; continue;
            }
            81 => {
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 90; continue; }
                pc = 82; continue;
            }
            82 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa8u32)))) & 0xffff);
                fl = lift::sub(ebp, edx, 32).1;
                if (fl.sf == fl.of) { pc = 90; continue; }
                pc = 83; continue;
            }
            83 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.zf { pc = 85; continue; }
                pc = 84; continue;
            }
            84 => {
                ecx = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 86; continue;
            }
            85 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                pc = 86; continue;
            }
            86 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xb0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esp.wrapping_add(0x18u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ebp)))) & 0xff);
                edx = esp.wrapping_add(0x1cu32);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6c0bb0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xb4u32));
                edx = esp.wrapping_add(0x3cu32);
                ecx = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b97acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x4cu32));
                ecx = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::add(ebx, 0x64u32, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xb8u32));
                esp = esp.wrapping_add(0x34);
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ebp)))) & 0xff);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                edx = esp.wrapping_add(0x30u32);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                if fl.zf { pc = 88; continue; }
                pc = 87; continue;
            }
            87 => {
                ecx = m.r32(0x6cc380u32);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(0x6cc380u32);
                pc = 89; continue;
            }
            88 => {
                ecx = m.r32(0x6cc384u32);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(0x6cc384u32);
                pc = 89; continue;
            }
            89 => {
                eax = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x38u32));
                edx = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                esp = esp.wrapping_add(0x1c);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(edi, eax, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ebx = ecx;
                if (fl.sf != fl.of) { pc = 82; continue; }
                pc = 90; continue;
            }
            90 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa8u32)))) & 0xffff);
                esp = esp.wrapping_add(0x4);
                fl = lift::logic((ecx & 0xffff) & (ecx & 0xffff), 16);
                if fl.zf { pc = 92; continue; }
                pc = 91; continue;
            }
            91 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                pc = 92; continue;
            }
            92 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = edx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebp = ecx.wrapping_add(eax);
                fl = lift::logic(ebp & ebp, 32);
                ebp = ebx;
                if fl.zf { pc = 97; continue; }
                pc = 93; continue;
            }
            93 => {
                fl = lift::logic((edx & 0xffff) & (edx & 0xffff), 16);
                if fl.zf { pc = 95; continue; }
                pc = 94; continue;
            }
            94 => {
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                pc = 95; continue;
            }
            95 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa8u32))), 0x0u32, 16).1;
                if fl.zf { pc = 97; continue; }
                pc = 96; continue;
            }
            96 => {
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                pc = 97; continue;
            }
            97 => {
                fl = lift::sub(edi, m.r32(esp.wrapping_add(0x24u32)), 32).1;
                if (fl.sf == fl.of) { pc = 112; continue; }
                pc = 98; continue;
            }
            98 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xaau32)))) & 0xffff);
                fl = lift::sub(ebp, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 112; continue; }
                pc = 99; continue;
            }
            99 => {
                edx = m.r32(esi.wrapping_add(0xbcu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ebp.wrapping_mul(2))))) & 0xffff);
                edx = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx).wrapping_add(0xeu32)))) & 0xff);
                eax = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0cb4u32));
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b97b0u32);
                ecx = esp.wrapping_add(0x2cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xbcu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(ebp.wrapping_mul(2))))) & 0xffff);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ecx = edx.wrapping_add(eax).wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x2cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(ebx, edx, 32).1;
                if !fl.zf { pc = 101; continue; }
                pc = 100; continue;
            }
            100 => {
                eax = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 102; continue;
            }
            101 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                pc = 102; continue;
            }
            102 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_add(0x4);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x34u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                esp = esp.wrapping_add(0x10);
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 98; continue; }
                pc = 103; continue;
            }
            103 => {
                pc = 112; continue;
            }
            104 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), 0x0u32, 16).1;
                if (fl.cf || fl.zf) { pc = 112; continue; }
                pc = 105; continue;
            }
            105 => {
                ebx = m.r32(esi.wrapping_add(0xacu32));
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 112; continue; }
                pc = 106; continue;
            }
            106 => {
                ebp = m.r32(esi.wrapping_add(0x494u32));
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 112; continue; }
                pc = 107; continue;
            }
            107 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                fl = lift::sub(ebp, edx, 32).1;
                if (fl.sf == fl.of) { pc = 112; continue; }
                pc = 108; continue;
            }
            108 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(ebp, eax, 32).1;
                if !fl.zf { pc = 110; continue; }
                pc = 109; continue;
            }
            109 => {
                ecx = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(ebx.wrapping_add(ebp.wrapping_mul(4)));
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 111; continue;
            }
            110 => {
                eax = m.r32(ebx.wrapping_add(ebp.wrapping_mul(4)));
                edx = m.r32(esp.wrapping_add(0x14u32));
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 111; continue;
            }
            111 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 107; continue; }
                pc = 112; continue;
            }
            112 => {
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 113; continue;
            }
            113 => {
                ecx = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0xc4u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0xbcu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0xb8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004AF600` anim_button_advance_frame
pub fn f_004af600(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ebx = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(ebx, edi, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xb4u32)), edi, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 2; continue;
            }
            2 => {
                pc = 12; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xd0u32));
                fl = lift::sub(ecx, edi, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::add(m.r32(esi.wrapping_add(0xa8u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0xa8u32), r);
                pc = 7; continue;
            }
            5 => {
                eax = m.r32(0x6dccf8u32);
                edx = m.r32(esi.wrapping_add(0xd4u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.cf || fl.zf) { pc = 13; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), edi);
                { let v = lift::fild64(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                m.w32(esp.wrapping_add(0x10u32), ecx);
                m.w32(esp.wrapping_add(0x14u32), edi);
                lift::fst32(m, esp.wrapping_add(0xcu32), fp.st(0)); fp.pop();
                { let v = lift::fild64(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = m.r32(esi.wrapping_add(0xa8u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0xa8u32), ecx);
                edx = m.r32(0x6dccf8u32);
                m.w32(esi.wrapping_add(0xd4u32), edx);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0xa8u32));
                ecx = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(ebx, 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esi.wrapping_add(0xc8u32), ebx);
                pc = 11; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                edx = m.r32(esi.wrapping_add(0xc0u32));
                eax = m.r32(esi.wrapping_add(0xc4u32));
                m.w32(esi.wrapping_add(0xacu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x94u32);
                m.w32(esi.wrapping_add(0xc8u32), edx);
                m.w32(esi.wrapping_add(0xd0u32), eax);
                m.w32(esi.wrapping_add(0xb8u32), edi);
                m.w32(esi.wrapping_add(0xc0u32), edi);
                m.w32(esi.wrapping_add(0xc4u32), edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c5850, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0xccu32), eax);
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(0x6dccf8u32);
                m.w32(esi.wrapping_add(0xa8u32), edi);
                m.w32(esi.wrapping_add(0xd4u32), edx);
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0xa8u32));
                ecx = m.r32(esi.wrapping_add(0xacu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x94u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4c5870, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w32(esi.wrapping_add(0xb4u32), eax);
                pc = 13; continue;
            }
            13 => {
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

/// `0X004C5E50` keyframe_timer_evaluate
pub fn f_004c5e50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, edi);
                edi = ecx;
                ebx = m.r32(edi.wrapping_add(0x4u32));
                fl = lift::logic(ebx & ebx, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(eax, 0x1u32);
                m.w32(ecx, ebx);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, edi.wrapping_add(0x14u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x5u32, 8);
                if fl.pf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(edi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x18u32), edx);
                pc = 4; continue;
            }
            4 => {
                ebp = m.r32(edi.wrapping_add(0x8u32));
                fl = lift::sub(ebp, 0x2u32, 32).1;
                if !fl.cf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            6 => {
                { let v = lift::fld32(m, edi.wrapping_add(0x1cu32)); fp.push(v); }
                { let v = lift::fld32(m, edi.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x41u32, 8);
                if fl.pf { pc = 4; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x21u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(edi.wrapping_add(0x8u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(edi.wrapping_add(0x18u32), eax);
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w8(edi.wrapping_add(0x20u32), (0x0u32) as u8);
                m.w32(edx, ecx);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(eax, 0x0u32);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            9 => {
                { let v = lift::fld32(m, edi.wrapping_add(0x1cu32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66bc6cu32); fp.com(v); }
                fp.pop();
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(edi.wrapping_add(0xcu32));
                m.w8(esp.wrapping_add(0x13u32), (0x0u32) as u8);
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x44u32, 8);
                if !fl.pf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                { let v = lift::fld32(m, edi.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, edi.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                pc = 11; continue;
            }
            11 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                m.w32(esp.wrapping_add(0x1cu32), eax);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                fl = lift::logic(ebp & ebp, 32);
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d - v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, edi.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                if (fl.cf || fl.zf) { pc = 20; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 20; continue; }
                pc = 13; continue;
            }
            13 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, ebx.wrapping_add(esi.wrapping_mul(4))); fp.com(v); }
                fp.pop();
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, ebx.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x4u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x5u32, 8);
                if fl.pf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                pc = 19; continue;
            }
            16 => {
                edx = ebp.wrapping_add(0xfffffffeu32);
                fl = lift::sub(esi, edx, 32).1;
                if !fl.cf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                pc = 19; continue;
            }
            18 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(ecx, ebp, 32).1;
                if fl.cf { pc = 12; continue; }
                pc = 20; continue;
            }
            20 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                eax = esi.wrapping_add(0xffffffffu32);
                if !fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = ebp.wrapping_add(0xffffffffu32);
                pc = 22; continue;
            }
            22 => {
                m.w32(edi.wrapping_add(0xcu32), eax);
                eax = m.r32(esp.wrapping_add(0x20u32));
                m.w32(eax, esi);
                ecx = m.r32(edi.wrapping_add(0x4u32));
                { let v = lift::fld32(m, ecx.wrapping_add(esi.wrapping_mul(4))); fp.push(v); }
                { let v = lift::fld32(m, ecx.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0xfffffffcu32)); let d = fp.st(0); fp.set(0, d - v); }
                ecx = ecx.wrapping_add(esi.wrapping_mul(4));
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66bc6cu32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x44u32, 8);
                if !fl.pf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, ecx.wrapping_add(0xfffffffcu32)); let d = fp.st(0); fp.set(0, d - v); }
                edx = m.r32(esp.wrapping_add(0x20u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xfu32)))) & 0xff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d / v); }
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let v = lift::fld32(m, 0x66c1e8u32); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, edx, fp.st(0)); fp.pop();
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            24 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(eax, 0x0u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x3u32)))) & 0xff);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C74C0` button_base_draw
pub fn f_004c74c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x48);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x60u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebx = m.r32(esp.wrapping_add(0x64u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                ebp = m.r32(esp.wrapping_add(0x30u32));
                eax = edi.wrapping_add(ecx);
                ecx = ebx.wrapping_add(edx);
                edx = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x40u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x44u32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 29; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(0x6dcb94u32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 29; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x68u32));
                ebp = m.r32(esp.wrapping_add(0x74u32));
                edx = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(edx, edi);
                m.w32(edx.wrapping_add(0x4u32), ebx);
                m.w32(edx.wrapping_add(0x8u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4b18f0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                edx = m.r32(esi);
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x64u32), ebx);
                ecx = esi;
                ebx = eax.wrapping_add(ebp);
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x94u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = m.r32(esp.wrapping_add(0x64u32));
                esp = esp.wrapping_sub(0xc);
                ecx = esp;
                eax = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x94u32));
                m.w32(ecx, edi);
                edx = m.r32(eax);
                m.w32(ecx.wrapping_add(0x4u32), ebp);
                m.w32(ecx.wrapping_add(0x8u32), ebx);
                ecx = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax;
                pc = 10; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                pc = 8; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                pc = 8; continue;
            }
            8 => {
                ebp = m.r32(esp.wrapping_add(0x64u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(ecx);
                eax = esp;
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 10; continue;
            }
            10 => {
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esi.wrapping_add(0xb4u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                edx = ecx.wrapping_mul(8).wrapping_add(0x6dcd54u32);
                eax = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd54u32));
                m.w32(esp.wrapping_add(0x60u32), eax);
                eax = esp.wrapping_add(0x20u32);
                ebx = m.r32(edx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = m.r32(edx.wrapping_add(0x8u32));
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x6cu32), ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xb0u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0xccu32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 16; continue;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0xccu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                pc = 17; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                pc = 16; continue;
            }
            16 => {
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                pc = 17; continue;
            }
            17 => {
                eax = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffff0000u32);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x28u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0xd0u32));
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x4);
                ecx = esi;
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let t = m.r32(edx.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, 0x3u32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 23; continue; }
                pc = 18; continue;
            }
            18 => {
                match m.r32(eax.wrapping_mul(4).wrapping_add(0x4c776cu32)) { 0x4c76aa => { pc = 19; continue; } 0x4c76b3 => { pc = 20; continue; } 0x4c76bc => { pc = 21; continue; } _ => unreachable!() }
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0xc0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 22; continue;
            }
            20 => {
                ecx = m.r32(esi.wrapping_add(0xc8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 22; continue;
            }
            21 => {
                edx = m.r32(esi.wrapping_add(0xc4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 22; continue;
            }
            22 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, 0x1u32, 32).1;
                m.w32(esp.wrapping_add(0x60u32), eax);
                if !fl.zf { pc = 26; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                eax = 0x1u32;
                if !fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(esp.wrapping_add(0x60u32));
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0xa8u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(m.r32(eax.wrapping_add(0xfffffff8u32)), ebx, 32).1;
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(esi.wrapping_add(0xb0u32));
                edx = m.r32(esp.wrapping_add(0x5cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xb4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                eax = ecx.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4c1780, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x54u32), ecx);
                ecx = esp.wrapping_add(0x48u32);
                m.w32(esp.wrapping_add(0x4cu32), ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x4cu32), ebx);
                m.w32(esp.wrapping_add(0x54u32), edx);
                { let t = m.r32(eax.wrapping_add(0xe0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 29; continue;
            }
            29 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x48);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C8B70` combo_box_clear_dirty_and_forward_to_list
pub fn f_004c8b70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x4b2410, ecx, &a); }
    eax = m.r32(esi.wrapping_add(0x180u32));
    ecx = esi.wrapping_add(0x180u32);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    { let t = m.r32(eax.wrapping_add(0xf8u32)); let a = []; return h.call_at(esp, m, t, ecx, &a); }
    eax
}

/// `0X004CB750` list_box_update_scrollbar
pub fn f_004cb750(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    eax = m.r32(ecx.wrapping_add(0xb4u32));
    let (r, f) = lift::add(ecx, 0xb4u32, 32); fl = f;
    ecx = r;
    { let t = m.r32(eax.wrapping_add(0xf4u32)); let a = []; return h.call_at(esp, m, t, ecx, &a); }
    eax
}

/// `0X004CD640` viper_list_box_set_cell_style
pub fn f_004cd640(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::sub((esi & 0xffff), 0x7fu32, 16).1;
                ebp = ecx;
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                if (!fl.cf && !fl.zf) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(ebp.wrapping_add(0xa4u32));
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(ebx, m.r32(ebp.wrapping_add(0x9cu32)), 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w8(esp.wrapping_add(0x13u32), (0x0u32) as u8);
                pc = 4; continue;
            }
            4 => {
                eax = esi;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, 0x7fu32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 20; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x4cd894u32)))) & 0xff);
                match m.r32(ecx.wrapping_mul(4).wrapping_add(0x4cd880u32)) { 0x4cd694 => { pc = 6; continue; } 0x4cd69b => { pc = 7; continue; } 0x4cd6dc => { pc = 11; continue; } 0x4cd774 => { pc = 19; continue; } 0x4cd7cc => { pc = 20; continue; } _ => unreachable!() }
            }
            6 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 21; continue;
            }
            7 => {
                esi = m.r32(ebp.wrapping_add(0x230u32));
                ecx = ebp.wrapping_add(0xb4u32);
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                edx = eax.wrapping_add(esi).wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                fl = lift::sub(ebx, edx, 32).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf000u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 16; continue;
            }
            9 => {
                fl = lift::sub(ebx, esi, 32).1;
                if !fl.zf { pc = 18; continue; }
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf00u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 16; continue;
            }
            11 => {
                esi = m.r32(ebp.wrapping_add(0x230u32));
                ecx = ebp.wrapping_add(0xb4u32);
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                eax = eax.wrapping_add(esi).wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf000u32);
                pc = 15; continue;
            }
            13 => {
                fl = lift::sub(ebx, esi, 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 14; continue;
            }
            14 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf00u32);
                pc = 15; continue;
            }
            15 => {
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                pc = 16; continue;
            }
            16 => {
                esi = (esi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf0u32);
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfu32);
                let r = esi | eax; fl = lift::logic(r, 32);
                esi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x24);
                let r = esi | eax; fl = lift::logic(r, 32);
                esi = r;
                pc = 21; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                pc = 18; continue;
            }
            18 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esi = (esi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfu32);
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x18);
                let r = esi | eax; fl = lift::logic(r, 32);
                esi = r;
                pc = 21; continue;
            }
            19 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf000u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esi = (esi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf00u32);
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf0u32);
                let r = esi | eax; fl = lift::logic(r, 32);
                esi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfu32);
                let r = esi | eax; fl = lift::logic(r, 32);
                esi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x30);
                let r = esi | eax; fl = lift::logic(r, 32);
                esi = r;
                pc = 21; continue;
            }
            20 => {
                m.w8(esp.wrapping_add(0x13u32), (0x0u32) as u8);
                pc = 21; continue;
            }
            21 => {
                edi = m.r32(esp.wrapping_add(0x24u32));
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(ebp.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(ebx.wrapping_mul(4)));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x20u32), eax);
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(ebp.wrapping_add(0x98u32));
                edx = m.r32(eax.wrapping_add(ebx.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                { let a = []; eax = h.call_at(esp, m, 0x4d2e70, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edi = eax;
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                { let a = []; eax = h.call_at(esp, m, 0x4d2eb0, ecx, &a); }
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                pc = 24; continue;
            }
            24 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 27; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(ebp.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(ebx.wrapping_mul(4)));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                edx = edi;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2e40, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp.wrapping_add(0x98u32));
                let (r, f) = lift::shr(edi, 0x10u32, 32, fl); fl = f;
                edi = r;
                ecx = m.r32(eax.wrapping_add(ebx.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2e80, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = m.r32(edx.wrapping_add(ebx.wrapping_mul(4)));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2d40, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                pc = 27; continue;
            }
            27 => {
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

/// `0X004D2FE0` text_box_layout_text
pub fn f_004d2fe0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = m.r32(esi);
                m.w32(esi.wrapping_add(0x94u32), ebx);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                m.w32(esi.wrapping_add(0x9cu32), ebx);
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi.wrapping_add(0xd8u32));
                eax = m.r32(eax.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edx.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x3u32, 32, fl); fl = f;
                ecx = r;
                edx = ecx.wrapping_add(0x6dcd54u32);
                ecx = ecx.wrapping_add(0x6dcd28u32);
                edi = m.r32(edx);
                m.w32(esp.wrapping_add(0x10u32), edi);
                edi = m.r32(edx.wrapping_add(0x4u32));
                edx = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xf0u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                ecx = esi;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0x94u32), eax);
                pc = 6; continue;
            }
            4 => {
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esi.wrapping_add(0x94u32), eax);
                pc = 6; continue;
            }
            5 => {
                ecx = m.r32(esi.wrapping_add(0xb0u32));
                m.w32(esi.wrapping_add(0x94u32), ecx);
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x98u32), eax);
                eax = m.r32(esi.wrapping_add(0xf4u32));
                fl = lift::sub(eax, 0x4u32, 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                pc = 10; continue;
            }
            8 => {
                fl = lift::sub(eax, 0x5u32, 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(eax, m.r32(esi.wrapping_add(0xb4u32)), 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                pc = 10; continue;
            }
            10 => {
                m.w32(esi.wrapping_add(0x98u32), eax);
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w32(esi.wrapping_add(0x94u32), ebx);
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x98u32)), ebx, 32).1;
                if (fl.sf == fl.of) { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                m.w32(esi.wrapping_add(0x98u32), ebx);
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi);
                ecx = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; return h.call_at(esp, m, t, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004D30F0` text_box_update_borders
pub fn f_004d30f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xecu32)))) & 0xffff);
                m.w32(esi.wrapping_add(0xb0u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfu32);
                m.w32(esi.wrapping_add(0xb4u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbd30, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                m.w32(esi.wrapping_add(0xa0u32), eax);
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let (r, f) = lift::add(m.r32(esi.wrapping_add(0xb0u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0xb0u32), r);
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbd30, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                m.w32(esi.wrapping_add(0xa4u32), eax);
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let (r, f) = lift::add(m.r32(esi.wrapping_add(0xb0u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0xb0u32), r);
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf00u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbd30, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                m.w32(esi.wrapping_add(0xa8u32), eax);
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add(m.r32(esi.wrapping_add(0xb4u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0xb4u32), r);
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf000u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbd30, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                m.w32(esi.wrapping_add(0xacu32), eax);
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::add(m.r32(esi.wrapping_add(0xb4u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0xb4u32), r);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi);
                ecx = esi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; return h.call_at(esp, m, t, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x4410f0 => f_004410f0(m, h, ecx, esp),
        0x4415d0 => f_004415d0(m, h, ecx, esp),
        0x464c80 => f_00464c80(m, h, ecx, esp),
        0x46f9b0 => f_0046f9b0(m, h, ecx, esp),
        0x470f80 => f_00470f80(m, h, ecx, esp),
        0x471420 => f_00471420(m, h, ecx, esp),
        0x4af600 => f_004af600(m, h, ecx, esp),
        0x4c5e50 => f_004c5e50(m, h, ecx, esp),
        0x4c74c0 => f_004c74c0(m, h, ecx, esp),
        0x4c8b70 => f_004c8b70(m, h, ecx, esp),
        0x4cb750 => f_004cb750(m, h, ecx, esp),
        0x4cd640 => f_004cd640(m, h, ecx, esp),
        0x4d2fe0 => f_004d2fe0(m, h, ecx, esp),
        0x4d30f0 => f_004d30f0(m, h, ecx, esp),
        _ => return None,
    }))
}
