//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X0052E700` get_ware_spot_price
pub fn f_0052e700(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0x14u32));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                esi = eax.wrapping_add(edx.wrapping_mul(8));
                edx = edi.wrapping_add(0x4fu32);
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x4u32));
                let (r, f) = lift::add(edx, esi, 32); fl = f;
                edx = r;
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                if (!fl.zf && fl.sf == fl.of) { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esi.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x4u32), 0x0u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                { let v = lift::fld32(m, 0x6739e8u32); fp.push(v); }
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            3 => {
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); fp.push(v); }
                { let v = lift::fld32(m, ecx.wrapping_add(0x64u32)); let d = fp.st(0); fp.set(0, d * v); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            4 => {
                ebx = m.r32(edx);
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if (!fl.zf && fl.sf == fl.of) { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                eax = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                { let v = lift::fld32(m, 0x6739e8u32); fp.push(v); }
                { let v = lift::fld32(m, 0x6739ecu32); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x6739e8u32); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            7 => {
                { let v = lift::fld32(m, ecx.wrapping_add(0x64u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x6739ecu32); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, ecx.wrapping_add(0x64u32)); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            8 => {
                ecx = m.r32(edx.wrapping_add(0x4u32));
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 12; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                eax = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                { let v = lift::fld32(m, 0x6739ecu32); fp.push(v); }
                { let v = lift::fld32(m, 0x6739f0u32); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x6739ecu32); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            11 => {
                { let v = lift::fld32(m, 0x673a08u32); fp.push(v); }
                { let v = lift::fld32(m, 0x6739f0u32); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x673a08u32); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            12 => {
                esi = m.r32(edx.wrapping_add(0x8u32));
                fl = lift::sub(eax, esi, 32).1;
                if (fl.sf == fl.of) { pc = 16; continue; }
                pc = 13; continue;
            }
            13 => {
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(esi, ecx, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), esi);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                eax = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                { let v = lift::fld32(m, 0x6739f0u32); fp.push(v); }
                { let v = lift::fld32(m, 0x6739f4u32); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x6739f0u32); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            15 => {
                { let v = lift::fld32(m, 0x673a0cu32); fp.push(v); }
                { let v = lift::fld32(m, 0x6739f4u32); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x673a0cu32); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(edx.wrapping_add(0xcu32));
                fl = lift::sub(eax, ebp, 32).1;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                if (fl.sf == fl.of) { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                eax = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                { let v = lift::fld32(m, 0x6739f4u32); fp.push(v); }
                { let v = lift::fld32(m, 0x6739f8u32); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x6739f4u32); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            19 => {
                { let v = lift::fld32(m, 0x673a10u32); fp.push(v); }
                { let v = lift::fld32(m, 0x6739f8u32); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x673a10u32); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            20 => {
                { let v = lift::fld32(m, 0x673a14u32); fp.push(v); }
                { let v = lift::fld32(m, edi.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
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

/// `0X0052E960` get_ware_stock_level_position
pub fn f_0052e960(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = m.r32(0x6de508u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = edx.wrapping_add(ecx.wrapping_mul(8));
                edx = m.r32(esp.wrapping_add(0x8u32));
                ecx = edx.wrapping_add(0x4fu32);
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0xcu32), eax);
                edx = m.r32(ecx);
                fl = lift::sub(eax, edx, 32).1;
                m.w32(esp.wrapping_add(0x8u32), edx);
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0xcu32)) as i32); fp.push(v); }
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x4u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x4u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x4u32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                esi = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::sub(eax, esi, 32).1;
                if (fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0xcu32)) as i32); fp.push(v); }
                m.w32(esp.wrapping_add(0xcu32), esi);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::fst32(m, esp.wrapping_add(0x4u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); let d = fp.st(0); fp.set(0, d / v); }
                { let v = lift::fld32(m, 0x66bc80u32); let d = fp.st(0); fp.set(0, d + v); }
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            4 => {
                edx = m.r32(ecx.wrapping_add(0x8u32));
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, esi, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                m.w32(esp.wrapping_add(0x8u32), edx);
                lift::fst32(m, esp.wrapping_add(0x4u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); let d = fp.st(0); fp.set(0, d / v); }
                { let v = lift::fld32(m, 0x66f4a4u32); let d = fp.st(0); fp.set(0, d + v); }
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            6 => {
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x8u32), ecx);
                if (fl.sf == fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                m.w32(esp.wrapping_add(0x8u32), ecx);
                lift::fst32(m, esp.wrapping_add(0x4u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); let d = fp.st(0); fp.set(0, d / v); }
                { let v = lift::fld32(m, 0x66fd54u32); let d = fp.st(0); fp.set(0, d + v); }
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            8 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0xcu32)) as i32); fp.push(v); }
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x4u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x4u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x4u32)); let d = fp.st(0); fp.set(0, d / v); }
                { let v = lift::fld32(m, 0x66fd54u32); let d = fp.st(0); fp.set(0, d + v); }
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x52e700 => f_0052e700(m, h, ecx, esp),
        0x52e960 => f_0052e960(m, h, ecx, esp),
        _ => return None,
    }))
}
