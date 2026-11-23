//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X004901C0` ship_side_menu_draw_artillery_icons
pub fn f_004901c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(ecx.wrapping_add(0x3814u32));
                esp = esp.wrapping_sub(0x38);
                edx = m.r32(eax.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x1cu32), edx);
                edx = m.r32(eax);
                m.w32(esp.wrapping_add(0x34u32), edx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x38u32), edx);
                ebx = m.r32(eax.wrapping_add(0x8u32));
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x40u32), eax);
                pc = 4; continue;
            }
            3 => {
                ebx = m.r32(esp.wrapping_add(0x3cu32));
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(ecx.wrapping_add(0x3818u32));
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ecx = m.r32(eax.wrapping_add(0xcu32));
                edx = m.r32(ecx);
                m.w32(esp.wrapping_add(0x24u32), edx);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                ebp = m.r32(ecx.wrapping_add(0x8u32));
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x30u32), ecx);
                pc = 8; continue;
            }
            7 => {
                ebp = m.r32(esp.wrapping_add(0x2cu32));
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esp.wrapping_add(0x4cu32));
                ecx = ebx.wrapping_add(ebx.wrapping_mul(4));
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x50u32)))) & 0xff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(0, (eax & 0xff), 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = !eax;
                let r = eax & ecx; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x38);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            10 => {
                edx = m.r32(0x672cc8u32);
                esp = esp.wrapping_sub(0x8);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x54u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x54u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x54u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x54u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66f4b0u32); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst64(m, esp, fp.st(0)); fp.pop();
                { let a = []; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                { let v = lift::fld64(m, 0x66f4a8u32); let d = fp.st(0); fp.set(0, d + v); }
                esp = esp.wrapping_add(0x8);
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = fp.st(0); let d = fp.st(0); fp.set(0, d + v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                m.w32(esp.wrapping_add(0x4cu32), eax);
                m.w32(esp.wrapping_add(0xcu32), 0x0u32);
                { let v = f64::from(m.r32(esp.wrapping_add(0x4cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x4cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4cu32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66f4a4u32); let d = fp.st(0); fp.set(0, d / v); }
                m.w8(esp.wrapping_add(0x4cu32), (0x0u32) as u8);
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66bc74u32); let d = fp.st(0); fp.set(0, d / v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                edi = eax;
                fl = lift::sub(edi, 0x5u32, 32).1;
                m.w32(esp.wrapping_add(0x14u32), edi);
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w32(esp.wrapping_add(0x14u32), 0x5u32);
                edi = m.r32(esp.wrapping_add(0x14u32));
                ecx = edi;
                pc = 17; continue;
            }
            12 => {
                eax = edi.wrapping_add(edi.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x4cu32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x4cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x4cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x4cu32)); let d = fp.st(0); fp.set(0, d - v); }
                lift::fst32(m, esp.wrapping_add(0x4cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4cu32)); fp.push(v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = eax;
                m.w32(esp.wrapping_add(0xcu32), ecx);
                { let v = f64::from(m.r32(esp.wrapping_add(0xcu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4cu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d - v); }
                m.w8(esp.wrapping_add(0x4cu32), (0x1u32) as u8);
                { let v = lift::fld64(m, 0x66a7d0u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x100u32; fl = lift::logic(r, 32);
                eax = r;
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                m.w8(esp.wrapping_add(0x4cu32), (0x0u32) as u8);
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = edi;
                pc = 17; continue;
            }
            16 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x4cu32)))) & 0xff);
                eax = m.r32(esp.wrapping_add(0xcu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                pc = 17; continue;
            }
            17 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x50u32)))) & 0xff);
                let (r, f) = lift::imul(ecx, ebx); fl = f;
                ecx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = ecx;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x38);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            19 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x54u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x4cu32));
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x4cu32), edx);
                if !fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esi, eax);
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x54u32), ecx);
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(esi);
                eax = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x6cu32));
                esp = esp.wrapping_add(0x18);
                let (r, f) = lift::add(edi, ebx, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x54u32), eax);
                if !fl.zf { pc = 23; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_add(0x4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 28; continue; }
                pc = 25; continue;
            }
            25 => {
                ebx = eax;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x54u32), ebx);
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(esp.wrapping_add(0x34u32));
                edx = m.r32(esi);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                let (r, f) = lift::add(edi, ebp, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 26; continue; }
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(esp.wrapping_add(0x54u32));
                pc = 28; continue;
            }
            28 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x50u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esp.wrapping_add(0x34u32));
                ebx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ebp;
                let (r, f) = lift::imul(ebp, ecx); fl = f;
                ebp = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x4u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::add(ebp, ebx, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(esp.wrapping_add(0x4cu32));
                m.w32(esi, eax);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x38);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00490470` ship_side_menu_draw_hovered_ship_and_list
pub fn f_00490470(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0xcu32));
                eax = m.r32(esi.wrapping_add(0x4164u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x3cd4u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48ee90, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48cf50, ecx, &a); }
                esp = esp.wrapping_add(4);
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

/// `0X004904B0` ship_side_menu_draw_convoy_header
pub fn f_004904b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x5228u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_add(ecx.wrapping_mul(4));
                edx = m.r32(esi.wrapping_add(0x5238u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                m.w16(esi.wrapping_add(0x584cu32), ((edx & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x503ac0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(0x6b0d88u32);
                ecx = m.r32(0x699908u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esi.wrapping_add(0x5834u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba410u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 4; continue;
            }
            3 => {
                ecx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x584cu32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                edx = eax.wrapping_add(ecx).wrapping_add(0x160u32);
                eax = m.r32(0x699908u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x5834u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba418u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 4; continue;
            }
            4 => {
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x5248u32));
                edx = m.r32(esi.wrapping_add(0x5244u32));
                ecx = m.r32(esi.wrapping_add(0x524cu32));
                m.w32(esi.wrapping_add(0x5844u32), eax);
                eax = m.r32(esi.wrapping_add(0x5240u32));
                m.w32(esi.wrapping_add(0x5840u32), edx);
                edx = m.r32(esi.wrapping_add(0x523cu32));
                m.w32(esi.wrapping_add(0x5838u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0x5848u32), ecx);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x584cu32)))) & 0xffff);
                m.w32(esi.wrapping_add(0x583cu32), edx);
                edx = m.r32(0x6dd7a4u32);
                esp = esp.wrapping_add(0x10);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx).wrapping_add(0x42u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                ecx = (ecx & !0xff) | ((u32::from(fl.cf)) & 0xff);
                m.w8(esi.wrapping_add(0x584eu32), ((ecx & 0xff)) as u8);
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0xb4u32));
                eax = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esp.wrapping_add(0x8u32));
                eax = esi.wrapping_add(0x5834u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4916e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x8u32));
                edx = esi.wrapping_add(0x5834u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x491240, ecx, &a); }
                esp = esp.wrapping_add(8);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00490600` ship_side_menu_update_hover_row
pub fn f_00490600(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esi.wrapping_add(0x4164u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b77f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi.wrapping_add(0x387cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x40aad0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, ecx, 32).1;
                eax = m.r32(esp.wrapping_add(0x8u32));
                if !fl.zf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xacu32)), 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                ecx = m.r32(esi.wrapping_add(0x4164u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                pc = 6; continue;
            }
            5 => {
                eax = edi;
                pc = 6; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x8u32), eax);
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x4164u32));
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(ecx, m.r32(esi.wrapping_add(0xacu32)), 32).1;
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0xacu32));
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w32(esi.wrapping_add(0x4164u32), eax);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            11 => {
                m.w32(esi.wrapping_add(0x4164u32), 0xffffffffu32);
                pc = 12; continue;
            }
            12 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004906E0` ship_side_menu_refresh_owner_cargo_total
pub fn f_004906e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x28);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::sub(m.r32(ebp.wrapping_add(0x4098u32)), 0xffffffffu32, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(ebp.wrapping_add(0xacu32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 40; continue; }
                pc = 2; continue;
            }
            2 => {
                esi = m.r32(ebp.wrapping_add(0xa0u32));
                edx = m.r32(0x6dd7a4u32);
                pc = 3; continue;
            }
            3 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x3cu32)))) & 0xffff);
                ebx = (ebx & !0xff) | (((ecx & 0xff)) & 0xff);
                let r = (ebx & 0xff) & 0x4u32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                fl = lift::sub((ebx & 0xff), 0x4u32, 8).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = ecx & 0x400u32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(ecx, 0x400u32, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xffff) | ((u32::from(m.r8(eax))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esp.wrapping_add(0x3cu32))), 16).1;
                if fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(esi, 0x2u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 7; continue;
            }
            7 => {
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
            8 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x60u32);
                ecx = ebp.wrapping_add(0x409cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(ebp.wrapping_add(0x4098u32), ebx);
                m.w32(ebp.wrapping_add(0x4160u32), ebx);
                m.w32(ebp.wrapping_add(0x415cu32), ebx);
                m.w32(ebp.wrapping_add(0x5248u32), ebx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = ebp.wrapping_add(0x40fcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(ebp.wrapping_add(0xacu32));
                esp = esp.wrapping_add(0x18);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                if (fl.zf || fl.sf != fl.of) { pc = 34; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(ebp.wrapping_add(0xa0u32));
                esi = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(eax))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esp.wrapping_add(0x3cu32))), 16).1;
                if !fl.zf { pc = 33; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edx = eax.wrapping_add(0xb4u32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                esi = ebp.wrapping_add(0x40fcu32);
                m.w32(esp.wrapping_add(0x14u32), edx);
                pc = 11; continue;
            }
            11 => {
                edi = m.r32(edx.wrapping_add(0xffffffa0u32));
                fl = lift::logic(edi & edi, 32);
                m.w32(esp.wrapping_add(0x10u32), edi);
                if fl.zf { pc = 24; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0xffffffa0u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                let (r, f) = lift::add(m.r32(ebp.wrapping_add(0x4098u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(ebp.wrapping_add(0x4098u32), r);
                pc = 14; continue;
            }
            14 => {
                ebx = m.r32(esi.wrapping_add(0xffffffa0u32));
                fl = lift::sub(ecx, 0x14u32, 32).1;
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                if (fl.sf != fl.of) { pc = 19; continue; }
                pc = 15; continue;
            }
            15 => {
                edi = m.r32(esi);
                let (r, f) = lift::imul(edi, ebx); fl = f;
                edi = r;
                m.w32(esi, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                lift::fst32(m, esp.wrapping_add(0x20u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x20u32)); fp.push(v); }
                { let v = lift::fld32(m, edx); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x66f4bcu32); let d = fp.st(0); fp.set(0, d * v); }
                pc = 18; continue;
            }
            17 => {
                lift::fst32(m, esp.wrapping_add(0x20u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x20u32)); fp.push(v); }
                { let v = lift::fld32(m, edx); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x66f4b8u32); let d = fp.st(0); fp.set(0, d * v); }
                pc = 18; continue;
            }
            18 => {
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                eax = edi;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esi, edi);
                { let d = u64::from(ebx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                pc = 23; continue;
            }
            19 => {
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ebx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                edi = edx;
                let (r, f) = lift::imul(edi, m.r32(esi)); fl = f;
                edi = r;
                m.w32(esi, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                lift::fst32(m, esp.wrapping_add(0x20u32), fp.st(0)); fp.pop();
                ecx = m.r32(esp.wrapping_add(0x14u32));
                { let v = lift::fld32(m, esp.wrapping_add(0x20u32)); fp.push(v); }
                { let v = lift::fld32(m, ecx); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esi, eax);
                pc = 22; continue;
            }
            21 => {
                lift::fst32(m, esp.wrapping_add(0x20u32), fp.st(0)); fp.pop();
                edx = m.r32(esp.wrapping_add(0x14u32));
                { let v = lift::fld32(m, esp.wrapping_add(0x20u32)); fp.push(v); }
                { let v = lift::fld32(m, edx); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x66f4b4u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                m.w32(esi, edi);
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ebx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                ecx = edx;
                eax = m.r32(esi);
                let (r, f) = lift::sar(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                edx = ecx;
                let (r, f) = lift::shr(edx, 0x1fu32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let d = u64::from(ecx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                ecx = m.r32(esp.wrapping_add(0x10u32));
                pc = 23; continue;
            }
            23 => {
                m.w32(esi, eax);
                edx = m.r32(ebp.wrapping_add(0x5248u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(ebp.wrapping_add(0x5248u32), edx);
                eax = m.r32(esi.wrapping_add(0xffffffa0u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esi.wrapping_add(0xffffffa0u32), eax);
                eax = m.r32(esp.wrapping_add(0x24u32));
                pc = 24; continue;
            }
            24 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(edx, 0x4u32, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(esi, 0x4u32, 32); fl = f;
                esi = r;
                fl = lift::sub(ecx, 0x18u32, 32).1;
                m.w32(esp.wrapping_add(0x18u32), ecx);
                m.w32(esp.wrapping_add(0x14u32), edx);
                if (fl.sf != fl.of) { pc = 11; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x157u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 29; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(ebp.wrapping_add(0x415cu32));
                fl = lift::logic(ecx & ecx, 32);
                if !fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                let (r, f) = lift::add(m.r32(ebp.wrapping_add(0x4098u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(ebp.wrapping_add(0x4098u32), r);
                pc = 28; continue;
            }
            28 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x157u32)))) & 0xff);
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(ebp.wrapping_add(0x415cu32), edx);
                pc = 29; continue;
            }
            29 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x156u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 33; continue; }
                pc = 30; continue;
            }
            30 => {
                ecx = m.r32(ebp.wrapping_add(0x4160u32));
                fl = lift::logic(ecx & ecx, 32);
                if !fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                let (r, f) = lift::add(m.r32(ebp.wrapping_add(0x4098u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(ebp.wrapping_add(0x4098u32), r);
                pc = 32; continue;
            }
            32 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x156u32)))) & 0xff);
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(ebp.wrapping_add(0x4160u32), edx);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x156u32)))) & 0xff);
                eax = ecx;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(ebp.wrapping_add(0x5248u32));
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(ebp.wrapping_add(0x5248u32), eax);
                pc = 33; continue;
            }
            33 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(ebp.wrapping_add(0xacu32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(ebp.wrapping_add(0xccu32));
                ecx = m.r32(ebp.wrapping_add(0xd0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                ecx = m.r32(ebp.wrapping_add(0x4098u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp.wrapping_add(0x3ce4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 36; continue;
            }
            36 => {
                eax = m.r32(esp.wrapping_add(0x3cu32));
                ecx = m.r32(0x6dfc14u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 39; continue; }
                pc = 37; continue;
            }
            37 => {
                edx = m.r32(ebp.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(0x66e0ecu32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edx = m.r32(0x6dd7a4u32);
                esi = m.r32(ebp.wrapping_add(0x5248u32));
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx).wrapping_add(0x8u32)))) & 0xffff);
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(edi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = edi;
                ecx = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esi;
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                m.w32(ebp.wrapping_add(0x5248u32), esi);
                if !fl.sf { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                m.w32(ebp.wrapping_add(0x5248u32), ebx);
                pc = 39; continue;
            }
            39 => {
                edi = m.r32(ebp.wrapping_add(0x38c0u32));
                esi = ebp.wrapping_add(0x38b8u32);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                edx = m.r32(ebp.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                edi = m.r32(ebp.wrapping_add(0x38c0u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ebx = m.r32(ebp.wrapping_add(0x18u32));
                edx = m.r32(ebp.wrapping_add(0x38bcu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                edi = m.r32(ebp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x28u32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x28u32));
                ecx = 0x6da5f0u32;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x30u32), eax);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
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
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00490B10` ship_side_menu_prune_selection
pub fn f_00490b10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                eax = m.r32(esi.wrapping_add(0xacu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                edx = m.r32(0x6dd7a4u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ebp.wrapping_mul(2))))) & 0xffff);
                eax = ecx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3cu32)))) & 0xff);
                let r = (edx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), 0x1u32, 8).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                ebx = (ebx & !0xff) | (((edx & 0xff)) & 0xff);
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi.wrapping_add(0xb4u32));
                fl = lift::sub(edx, m.r32(esi.wrapping_add(0xb8u32)), 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                fl = lift::sub(edx, m.r32(0x6dfc14u32), 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x13u32, 16).1;
                if fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub((eax & 0xffff), 0xdu32, 16).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub((eax & 0xffff), 0xcu32, 16).1;
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x48fdf0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = esi;
                edi = eax;
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                { let a = []; eax = h.call_at(esp, m, 0x490600, ecx, &a); }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0xa0u32)), 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w8(esp.wrapping_add(0x20u32), (0x0u32) as u8);
                m.w32(esp.wrapping_add(0x1cu32), edi);
                { let t = m.r32(edx.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
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
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            12 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                edi = esi.wrapping_add(0x387cu32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x3880u32));
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = edi;
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edx = esp.wrapping_add(0x14u32);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            14 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00490F80` ship_side_menu_draw_ship_marks
pub fn f_00490f80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(0x6dd7a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, esi, 32); fl = f;
                edi = r;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x14u32), ebp);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x10u32), edx);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = edx.wrapping_add(eax.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x10u32), eax);
                edx = eax;
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x136u32)))) & 0xff);
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                fl = lift::logic((ebx & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 31; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x10u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 11; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                fl = lift::logic(u32::from(m.r8(eax.wrapping_add(esi).wrapping_add(0x136u32))) & (ebx & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(ecx.wrapping_add(0x385cu32));
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                ebx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(eax.wrapping_add(0xcu32));
                ebp = m.r32(ecx);
                edx = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x1cu32), edx);
                esi = m.r32(ecx.wrapping_add(0x8u32));
                eax = m.r32(ecx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                pc = 10; continue;
            }
            9 => {
                ebp = m.r32(esp.wrapping_add(0x18u32));
                esi = m.r32(esp.wrapping_add(0x20u32));
                ebx = m.r32(esp.wrapping_add(0x38u32));
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(esp.wrapping_add(0x38u32));
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x4cu32));
                edx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_add(0x20);
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                ebp = esi;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                m.w32(esp.wrapping_add(0x14u32), ebp);
                pc = 12; continue;
            }
            11 => {
                esi = m.r32(esp.wrapping_add(0x20u32));
                ebx = m.r32(esp.wrapping_add(0x38u32));
                pc = 12; continue;
            }
            12 => {
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 32; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x14u32)))) & 0xff);
                let r = (eax & 0xff) & 0x10u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                fl = lift::sub((eax & 0xff), 0x10u32, 8).1;
                if fl.zf { pc = 32; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x134u32))), 0x12u32, 16).1;
                if fl.zf { pc = 23; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 23; continue; }
                pc = 16; continue;
            }
            16 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x503ac0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffff00u32);
                pc = 24; continue;
            }
            18 => {
                ecx = m.r32(0x6dfc14u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff0000ffu32);
                pc = 24; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x8u32))), 0x0u32, 16).1;
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                pc = 24; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffff0000u32);
                pc = 24; continue;
            }
            23 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff282828u32);
                pc = 24; continue;
            }
            24 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x3cu32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                esp = esp.wrapping_add(0x4);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd7a8u32);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x10u32))), (ecx & 0xffff), 16).1;
                if !fl.zf { pc = 28; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(0x6cc388u32);
                eax = m.r32(ecx.wrapping_add(0x14u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 38; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 38; continue; }
                pc = 27; continue;
            }
            27 => {
                edx = m.r32(ecx.wrapping_add(0xcu32));
                ebx = eax;
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                esi = m.r32(edx.wrapping_add(0x8u32));
                edx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x24u32), edx);
                pc = 38; continue;
            }
            28 => {
                ecx = m.r32(0x6cc38cu32);
                eax = m.r32(ecx.wrapping_add(0x14u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 38; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 38; continue; }
                pc = 30; continue;
            }
            30 => {
                ebx = eax;
                eax = m.r32(ecx.wrapping_add(0xcu32));
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x1cu32), edx);
                esi = m.r32(eax.wrapping_add(0x8u32));
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                pc = 38; continue;
            }
            31 => {
                esi = m.r32(esp.wrapping_add(0x20u32));
                ebx = m.r32(esp.wrapping_add(0x38u32));
                pc = 32; continue;
            }
            32 => {
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x134u32))), 0x12u32, 16).1;
                if fl.zf { pc = 35; continue; }
                pc = 33; continue;
            }
            33 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = ebp;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x18);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            35 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(0x6cc390u32);
                esp = esp.wrapping_add(0x4);
                eax = m.r32(ecx.wrapping_add(0x14u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 38; continue; }
                pc = 36; continue;
            }
            36 => {
                eax = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                ebx = eax;
                edx = m.r32(ecx);
                m.w32(esp.wrapping_add(0x18u32), edx);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                esi = m.r32(ecx.wrapping_add(0x8u32));
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x24u32), ecx);
                pc = 38; continue;
            }
            38 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_add(0x1c);
                eax = esi.wrapping_add(edx);
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

/// `0X004916E0` ship_side_menu_draw_ship_row_owned
pub fn f_004916e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6604aeu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x44);
                edx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x68u32));
                esi = ecx;
                ecx = m.r32(0x6dfc14u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x18u32)))) & 0xffff);
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx))) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x64u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x491240, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 24; continue;
            }
            2 => {
                ebp = m.r32(esp.wrapping_add(0x64u32));
                eax = m.r32(esi.wrapping_add(0x3864u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esp.wrapping_add(0x44u32), 0x0u32);
                edx = m.r32(ebp);
                ecx = m.r32(ebp.wrapping_add(0x4u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6cbd5cu32);
                m.w32(esp.wrapping_add(0x3cu32), edx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x3874u32));
                esp = esp.wrapping_add(0x8);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                eax = m.r32(0x6c7cccu32);
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x64u32), eax);
                m.w32(esp.wrapping_add(0x5cu32), 0x0u32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                ecx = esp.wrapping_add(0x68u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x6cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w8(esp.wrapping_add(0x6cu32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x490c70, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x18u32)))) & 0xffff);
                eax = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(0xc);
                edx = esp;
                m.w32(edx, eax);
                eax = m.r32(esp.wrapping_add(0x50u32));
                m.w32(edx.wrapping_add(0x4u32), ecx);
                ecx = esi;
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x490f80, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(edi);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x3874u32));
                esp = esp.wrapping_add(0x10);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(eax.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                eax = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x30u32));
                ebx = m.r32(esp.wrapping_add(0x3cu32));
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                m.w32(esp.wrapping_add(0x3cu32), ebx);
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
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_add(0x14);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x6cu32));
                esp = esp.wrapping_add(0x4);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x68u32), ecx);
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x44u32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x48u32), ecx);
                eax = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x4cu32), eax);
                ebx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x50u32), ebx);
                pc = 6; continue;
            }
            5 => {
                ebx = m.r32(esp.wrapping_add(0x50u32));
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x50u32));
                eax = m.r32(esp.wrapping_add(0x40u32));
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
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
                edx = m.r32(esp.wrapping_add(0x68u32));
                ecx = m.r32(esp.wrapping_add(0x54u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                m.w32(esp.wrapping_add(0x58u32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x84u32));
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                edx = m.r32(esp.wrapping_add(0x58u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x68u32));
                ecx = m.r32(esi.wrapping_add(0x3864u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_add(0x2c);
                m.w32(esp.wrapping_add(0x3cu32), eax);
                eax = m.r32(ebp);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x3cu32);
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x491ca0, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(0x6cbd5cu32);
                esp = esp.wrapping_add(0x4);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(ebp.wrapping_add(0x4u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x3cu32), eax);
                eax = m.r32(esi.wrapping_add(0x382cu32));
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x68u32), ecx);
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0x44u32), ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x48u32), edx);
                ecx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x4cu32), ecx);
                ebx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x50u32), ebx);
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x50u32));
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
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
                eax = m.r32(edi.wrapping_add(0xcu32));
                ecx = m.r32(edi.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x44u32), eax);
                eax = m.r32(esi.wrapping_add(0x3830u32));
                m.w32(esp.wrapping_add(0x40u32), ecx);
                esp = esp.wrapping_add(0x1c);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x68u32), ecx);
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x44u32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x48u32), ecx);
                eax = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x4cu32), eax);
                ebx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x50u32), ebx);
                pc = 12; continue;
            }
            12 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x28u32)) as i32); fp.push(v); }
                ecx = m.r32(edi.wrapping_add(0x14u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                m.w32(esp.wrapping_add(0x30u32), eax);
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x50u32)) as i32); fp.push(v); }
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                lift::fst32(m, esp.wrapping_add(0x20u32), fp.st(0)); fp.pop();
                { let v = lift::fild64(m, esp.wrapping_add(0x2cu32)); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d / v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x20u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                m.w32(esp.wrapping_add(0x10u32), ebx);
                pc = 17; continue;
            }
            14 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x24u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x2cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x2cu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d / v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x20u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = m.r32(esp.wrapping_add(0x10u32));
                ebp = eax;
                edx = ecx.wrapping_add(ebp);
                fl = lift::sub(edx, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                ebp = ebx;
                let (r, f) = lift::sub(ebp, ecx, 32); fl = f;
                ebp = r;
                pc = 17; continue;
            }
            16 => {
                eax = ebx;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(esi.wrapping_add(0x386cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                eax = edx.wrapping_add(ebp);
                edx = m.r32(esp.wrapping_add(0x54u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = eax.wrapping_add(ecx);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 19; continue;
            }
            19 => {
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                edx = m.r32(esi.wrapping_add(0x3870u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x50u32));
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x24u32));
                edx = eax.wrapping_add(ecx);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esi = m.r32(esi.wrapping_add(0x38a8u32));
                esp = esp.wrapping_add(0x4);
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(edx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                edx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x38u32));
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                edx = eax.wrapping_add(ecx).wrapping_add(0xfffffffeu32);
                m.w32(esp.wrapping_add(0x3cu32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0x14u32));
                ebp = m.r32(esp.wrapping_add(0x28u32));
                esi = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(eax, esi, 32).1;
                if (fl.cf || fl.zf) { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffff0000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 23; continue;
            }
            23 => {
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(esi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = esp.wrapping_add(0x68u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba45cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x70u32));
                ecx = m.r32(esp.wrapping_add(0x48u32));
                edx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = 0x51eb851fu32;
                ecx = esp.wrapping_add(0x80u32);
                { let r = i64::from(eax as i32) * i64::from(ebp as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba460u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x58u32));
                edx = m.r32(esp.wrapping_add(0x8cu32));
                let (r, f) = lift::sub(ebx, ebp, 32); fl = f;
                ebx = r;
                esi = m.r32(esp.wrapping_add(0x64u32));
                eax = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = 0x51eb851fu32;
                ecx = esp.wrapping_add(0x98u32);
                { let r = u64::from(eax) * u64::from(m.r32(edi.wrapping_add(0x14u32))); eax = r as u32; edx = (r >> 32) as u32; fl.cf = edx != 0; fl.of = edx != 0; }
                let (r, f) = lift::shr(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba464u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x7cu32));
                edx = m.r32(esp.wrapping_add(0xa4u32));
                ecx = m.r32(esp.wrapping_add(0x78u32));
                esp = esp.wrapping_add(0x40);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x5cu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x64u32);
                m.w32(esp.wrapping_add(0x5cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x50);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00491CA0` ship_side_menu_draw_ship_stats_row
pub fn f_00491ca0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6604c0u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x1c);
                eax = m.r32(0x6c7cccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                ebx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x18u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x38u32), 0x0u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0x380cu32));
                esp = esp.wrapping_add(0x4);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ebx = m.r32(edx.wrapping_add(0x8u32));
                ebp = m.r32(edx.wrapping_add(0xcu32));
                pc = 4; continue;
            }
            3 => {
                ebx = m.r32(esp.wrapping_add(0x24u32));
                ebp = m.r32(esp.wrapping_add(0x28u32));
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esi = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                eax = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(esp.wrapping_add(0x60u32));
                esp = esp.wrapping_add(0x1c);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                m.w32(esi.wrapping_add(0x4u32), ecx);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x4u32));
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(0x6cbd28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(0x6cbd20u32);
                eax = m.r32(0x6cbd1cu32);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x50u32));
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
                esp = esp.wrapping_add(0x20);
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = esp.wrapping_add(0x14u32);
                ecx = m.r32(eax.wrapping_add(0x30u32));
                let (r, f) = lift::sar(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba468u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(edi.wrapping_add(0x38a8u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = ebx;
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi);
                eax = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x4u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(esi);
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esi.wrapping_add(0x4u32), eax);
                m.w32(esi, edx);
                eax = edx;
                edx = m.r32(edi.wrapping_add(0x3878u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esi, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(0x6cc3a4u32);
                esp = esp.wrapping_add(0x10);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x20u32), edx);
                ebx = m.r32(eax.wrapping_add(0x8u32));
                ebp = m.r32(eax.wrapping_add(0xcu32));
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_add(0x1c);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x4u32), ecx);
                eax = m.r32(edi.wrapping_add(0x5234u32));
                fl = lift::sub(eax, 0x28u32, 32).1;
                if !fl.cf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffff0000u32);
                pc = 12; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                pc = 12; continue;
            }
            12 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0x5234u32));
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba46cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(edi.wrapping_add(0x38a8u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebx;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(esi);
                let (r, f) = lift::sub(ecx, ebp, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x4u32), ecx);
                m.w32(esi, eax);
                ecx = m.r32(edi.wrapping_add(0x3878u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0x3810u32));
                esp = esp.wrapping_add(0x10);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ebx = m.r32(edx.wrapping_add(0x8u32));
                ebp = m.r32(edx.wrapping_add(0xcu32));
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_add(0x1c);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x4u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1au32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 19; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(edi.wrapping_add(0x3840u32));
                edx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 20; continue; }
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                edx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x28u32), edx);
                pc = 20; continue;
            }
            19 => {
                m.w32(esp.wrapping_add(0x24u32), 0x0u32);
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(ecx.wrapping_add(0x8u32));
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba470u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(edi.wrapping_add(0x38a8u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x24u32));
                ecx = eax;
                eax = ebx;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x40u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x44u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1au32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(edi.wrapping_add(0x3840u32));
                eax = m.r32(edx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x58u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x20);
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x50u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(esi);
                let (r, f) = lift::sub(ecx, ebp, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x4u32), ecx);
                m.w32(esi, eax);
                ecx = m.r32(edi.wrapping_add(0x3878u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0x3828u32));
                esp = esp.wrapping_add(0x14);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 25; continue; }
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ebx = m.r32(edx.wrapping_add(0x8u32));
                ebp = m.r32(edx.wrapping_add(0xcu32));
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                edx = ebp.wrapping_add(0x2u32);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w32(esi.wrapping_add(0x4u32), eax);
                eax = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4901c0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = 0xfffffffeu32;
                edi = eax;
                eax = m.r32(esi.wrapping_add(0x4u32));
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x4u32), eax);
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = edi;
                ecx = m.r32(esi.wrapping_add(0x4u32));
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(esi);
                esp = esp.wrapping_add(0x1c);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esi, eax);
                m.w32(esp.wrapping_add(0x34u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00492150` ship_side_menu_refresh_selection_stats
pub fn f_00492150(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc8u32)), 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                ebx = m.r32(esp.wrapping_add(0x30u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, edi, 32).1;
                m.w32(esp.wrapping_add(0x14u32), edi);
                m.w32(esi.wrapping_add(0x523cu32), edi);
                m.w32(esi.wrapping_add(0x5240u32), edi);
                m.w32(esi.wrapping_add(0x524cu32), edi);
                m.w8(esp.wrapping_add(0x13u32), (0x0u32) as u8);
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 3; continue;
            }
            3 => {
                ebp = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0x18u32), eax);
                pc = 4; continue;
            }
            4 => {
                edi = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp))) & 0xffff);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r8(ecx))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ecx.wrapping_add(0x11cu32));
                edi = m.r32(esi.wrapping_add(0x524cu32));
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                ebx = 0xc8u32;
                m.w32(esi.wrapping_add(0x524cu32), edi);
                eax = m.r32(ecx.wrapping_add(0x11cu32));
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ebx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ebx, 32); fl = f;
                edi = r;
                m.w32(esi.wrapping_add(0x524cu32), edi);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(ecx.wrapping_add(0x120u32));
                edx = m.r32(esi.wrapping_add(0x5240u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x523cu32));
                ebx = m.r32(esp.wrapping_add(0x30u32));
                m.w32(esi.wrapping_add(0x5240u32), edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x40u32)))) & 0xffff);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x523cu32), eax);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(ecx.wrapping_add(0x18u32));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(m.r32(ecx.wrapping_add(0x14u32)) as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(0x3cu32))), 0x0u32, 16).1;
                m.w32(esp.wrapping_add(0x14u32), edx);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(ebp, 0x2u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if !fl.zf { pc = 4; continue; }
                pc = 11; continue;
            }
            11 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(0x6de4b0u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.cf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                edx = m.r32(0x6de508u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = edx.wrapping_add(ecx.wrapping_mul(8));
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x52aad0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = eax;
                pc = 14; continue;
            }
            14 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0x5230u32)), 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x3864u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6cbd5cu32);
                m.w32(esp.wrapping_add(0x24u32), edx);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(esi.wrapping_add(0x38c0u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(m.r32(esi.wrapping_add(0xacu32)) as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0x5234u32), eax);
                if (fl.sf == fl.of) { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                m.w32(esi.wrapping_add(0x5234u32), 0x0u32);
                pc = 19; continue;
            }
            19 => {
                m.w32(esi.wrapping_add(0x5230u32), edi);
                eax = m.r32(0x6dfc14u32);
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if fl.zf { pc = 28; continue; }
                pc = 20; continue;
            }
            20 => {
                edx = m.r32(esi.wrapping_add(0x5238u32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                ebx = m.r32(0x66e0f0u32);
                ecx = m.r32(esi.wrapping_add(0x5234u32));
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                let r = ebp & 0xffffu32; fl = lift::logic(r, 32);
                ebp = r;
                eax = ebp;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ebx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = ebx;
                edi = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ecx;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                eax = 0x51eb851fu32;
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::imul(edi, edx); fl = f;
                edi = r;
                { let r = i64::from(eax as i32) * i64::from(edi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(ecx, 0x64u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = 0x64u32;
                pc = 24; continue;
            }
            22 => {
                fl = lift::logic(ecx & ecx, 32);
                if (fl.sf == fl.of) { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 24; continue;
            }
            24 => {
                eax = ebp;
                m.w32(esi.wrapping_add(0x5234u32), ecx);
                ebx = m.r32(0x66e0f8u32);
                edi = m.r32(esi.wrapping_add(0x5240u32));
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ebx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = ebx;
                ecx = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = edi;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = 0x51eb851fu32;
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::imul(ecx, edx); fl = f;
                ecx = r;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                m.w32(esi.wrapping_add(0x5240u32), edi);
                if !fl.sf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                m.w32(esi.wrapping_add(0x5240u32), 0x0u32);
                pc = 26; continue;
            }
            26 => {
                ebx = m.r32(0x66e0f4u32);
                eax = ebp;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ebx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edi = m.r32(esi.wrapping_add(0x523cu32));
                eax = ebx;
                ecx = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = edi;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = 0x51eb851fu32;
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::imul(ecx, edx); fl = f;
                ecx = r;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = edi;
                let (r, f) = lift::sar(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, 0x1u32, 32).1;
                m.w32(esi.wrapping_add(0x523cu32), eax);
                if (fl.sf == fl.of) { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                m.w32(esi.wrapping_add(0x523cu32), 0x1u32);
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
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00492430` ship_side_menu_open_town_info_or_office
pub fn f_00492430(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xbcu32)), 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xa0u32));
                ebp = m.r32(0x6dd7a4u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_add(0x4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edi = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ebp, 32); fl = f;
                edi = r;
                ebp = m.r32(0x6de508u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x39u32)))) & 0xff);
                eax = edx;
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ebx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ebx, 0x6u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                fl = lift::sub(m.r32(ebp.wrapping_add(ebx.wrapping_mul(8)).wrapping_add(0x76cu32)), 0x0u32, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x6e553cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x5b7390, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 10; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0xb4u32));
                ebx = m.r32(esi.wrapping_add(0xbcu32));
                fl = lift::sub(edx, ebx, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x66fe8cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6d9584u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4a9690, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 9; continue;
            }
            7 => {
                fl = lift::sub(edx, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x66fe98u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x5228u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6d9584u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4a9690, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0x3724u32));
                ecx = esi.wrapping_add(0x3724u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                eax = (eax & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(0x39u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(edi))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic(eax & eax, 32);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(esi.wrapping_add(0x82cu32));
                ecx = esi.wrapping_add(0x82cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xe0u32));
                eax = m.r32(esi.wrapping_add(0xccu32));
                m.w32(esi.wrapping_add(0xccu32), ecx);
                ecx = esi;
                m.w32(esi.wrapping_add(0x3720u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            12 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00492BC0` ship_side_menu_refresh_days_remaining
pub fn f_00492bc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x4164u32));
                edi = m.r32(esi.wrapping_add(0x3cd4u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                pc = 3; continue;
            }
            2 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), 0x2u32, 16).1;
                if fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub((ecx & 0xffff), 0xfu32, 16).1;
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub((ecx & 0xffff), 0x10u32, 16).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub((ecx & 0xffff), 0x3u32, 16).1;
                if !fl.zf { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(0x6de4b4u32);
                eax = m.r32(eax.wrapping_add(0x48u32));
                let r = (ecx & 0xff) ^ (ecx & 0xff); fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                edi = eax;
                pc = 10; continue;
            }
            9 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0x522cu32)), 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x3864u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(0x6cbd5cu32);
                m.w32(esp.wrapping_add(0x8u32), edx);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x38c0u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                eax = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x522cu32), edi);
                pc = 12; continue;
            }
            12 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00492CA0` ship_side_menu_commit_route_stop
pub fn f_00492ca0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = m.r32(0x6dd7a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = m.r32(ebx.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                eax = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd7a8u32);
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                eax = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 2; continue;
            }
            2 => {
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x132u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                fl = lift::sub((ebp & 0xffff), u32::from(m.r16(0x6dd72au32)), 16).1;
                esi = ebp;
                if !fl.cf { pc = 13; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6dd72cu32);
                pc = 4; continue;
            }
            4 => {
                eax = esi;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(edx.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x3u32)))) & 0xff);
                eax = ecx.wrapping_add(eax.wrapping_mul(4));
                fl = lift::logic((edx & 0xff) & 0x4u32, 8);
                if !fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub((ebp & 0xffff), (esi & 0xffff), 16).1;
                if !fl.zf { pc = 4; continue; }
                pc = 6; continue;
            }
            6 => {
                pc = 8; continue;
            }
            7 => {
                ebp = esi;
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(ebx.wrapping_add(0x5258u32));
                m.w32(esp.wrapping_add(0x14u32), esi);
                fl = lift::sub(edx, eax, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                eax = esi.wrapping_add(esi.wrapping_mul(4));
                eax = esi.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(4))))) & 0xffff);
                fl = lift::sub((ebp & 0xffff), (esi & 0xffff), 16).1;
                if !fl.zf { pc = 9; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = esp.wrapping_add(0x18u32);
                m.w32(esp.wrapping_add(0x10u32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x14u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), edi);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x5254u32)))) & 0xff);
                fl = lift::sub((ebp & 0xffff), (esi & 0xffff), 16).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(0x6de4b0u32);
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x18u32), 0x6au32);
                fl = lift::logic(eax & eax, 32);
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ecx | edx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::sub(0, ecx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sbb(ecx, ecx, fl.cf, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(eax, edi, 32).1;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                let (r, f) = lift::sbb(ecx, ecx, fl.cf, 32); fl = f;
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::sub(0, ecx, 32); fl = f;
                ecx = r;
                fl = lift::sub((esi & 0xffff), (ebp & 0xffff), 16).1;
                esi = m.r32(ebx.wrapping_add(0x5258u32));
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = ecx | edx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(esi & esi, 32);
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ecx | eax; fl = lift::logic(r, 32);
                ecx = r;
                if fl.zf { pc = 16; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 15; continue;
            }
            13 => {
                ecx = esp.wrapping_add(0x18u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(0x6de4b0u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x5254u32)))) & 0xff);
                fl = lift::sub(edx, eax, 32).1;
                if !fl.cf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(ebx.wrapping_add(0x5260u32));
                ecx = ebx.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edx = esp.wrapping_add(0x18u32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x5254u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x18u32), 0x6cu32);
                m.w32(esp.wrapping_add(0x1cu32), edi);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                m.w32(esp.wrapping_add(0x24u32), 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 15; continue;
            }
            15 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 16; continue;
            }
            16 => {
                edi = m.r32(ebx.wrapping_add(0x3884u32));
                esi = m.r32(ebx.wrapping_add(0x18u32));
                ecx = ebx.wrapping_add(0x387cu32);
                m.w32(ebx.wrapping_add(0x5250u32), 0x0u32);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                ecx = m.r32(ebx.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x18u32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(ebx.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(ebx.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
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

/// `0X00492EE0` ship_side_menu_reorder_route_stop
pub fn f_00492ee0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x64);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x2340u32));
                ecx = esi.wrapping_add(0x2340u32);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                edx = m.r32(0x6dd7a4u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                m.w32(esp.wrapping_add(0x1cu32), edi);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), edi);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = m.r32(0x6dd7a8u32);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x132u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x28u32), edi);
                edi = esi.wrapping_add(0x38f4u32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                ecx = edi;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebp = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x14u32), ebp);
                ebp = m.r32(esi.wrapping_add(0x38f8u32));
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                m.w32(esp.wrapping_add(0x34u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x24u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                m.w32(esp.wrapping_add(0x34u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x30u32));
                edx = (edx & !0xffff) | (((ebx & 0xffff)) & 0xffff);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd72au32)), 16).1;
                m.w32(esp.wrapping_add(0x30u32), ecx);
                if !fl.cf { pc = 31; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x3884u32));
                edi = m.r32(esi.wrapping_add(0x18u32));
                ecx = esi.wrapping_add(0x387cu32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x54u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x50u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x50u32);
                m.w32(esp.wrapping_add(0x5cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x5cu32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x5258u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2340u32));
                ecx = esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2340u32);
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x2340u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0x5258u32)), 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(eax);
                ecx = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                eax = m.r32(esp.wrapping_add(0x48u32));
                fl = lift::sub(edx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(eax, 0xf8u32, 32); fl = f;
                eax = r;
                fl = lift::sub(edi, 0x14u32, 32).1;
                m.w32(esp.wrapping_add(0x24u32), eax);
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 8; continue;
            }
            8 => {
                pc = 10; continue;
            }
            9 => {
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(esp.wrapping_add(0x10u32), edi);
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0x57d4u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(eax, ecx, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esi.wrapping_add(0x5258u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(eax, edx, 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                edi = m.r32(esi.wrapping_add(0x57d8u32));
                eax = edx;
                let (r, f) = lift::shl(eax, 0x5u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, edi, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let (r, f) = lift::imul(edx, m.r32(esp.wrapping_add(0x18u32))); fl = f;
                edx = r;
                ecx = esi.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x2340u32);
                esi = m.r32(esi.wrapping_add(0x18u32));
                eax = m.r32(ecx);
                edx = edx.wrapping_add(esi).wrapping_add(0x5au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x64);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            14 => {
                edi = m.r32(0x6dd72cu32);
                pc = 15; continue;
            }
            15 => {
                eax = ebx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(ecx.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x3u32)))) & 0xff);
                eax = edi.wrapping_add(eax.wrapping_mul(4));
                fl = lift::logic((ecx & 0xff) & 0x4u32, 8);
                if !fl.zf { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x18u32))), (ebx & 0xffff), 16).1;
                if !fl.zf { pc = 15; continue; }
                pc = 17; continue;
            }
            17 => {
                pc = 19; continue;
            }
            18 => {
                m.w32(esp.wrapping_add(0x18u32), ebx);
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                m.w32(esp.wrapping_add(0x24u32), ebx);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = ebx;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), ecx);
                pc = 21; continue;
            }
            21 => {
                fl = lift::sub(eax, m.r32(esp.wrapping_add(0x10u32)), 32).1;
                if !fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = ebx;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                pc = 23; continue;
            }
            23 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = ebx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(ecx.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(eax.wrapping_mul(4))))) & 0xffff);
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x18u32))), (ebx & 0xffff), 16).1;
                if !fl.zf { pc = 19; continue; }
                pc = 24; continue;
            }
            24 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x10u32)), 0xffffffffu32, 32).1;
                if !fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(esp.wrapping_add(0x30u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                ebp = esi.wrapping_add(0x2340u32);
                ebx = esi.wrapping_add(0x9fcu32);
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(esi.wrapping_add(0x57d8u32));
                eax = edi;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                edx = m.r32(ebx);
                let (r, f) = lift::imul(eax, m.r32(esp.wrapping_add(0x14u32))); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = eax.wrapping_add(ecx).wrapping_add(0x5au32);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(esi.wrapping_add(0x57d8u32));
                eax = edi;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                edx = m.r32(ebp);
                let (r, f) = lift::imul(eax, m.r32(esp.wrapping_add(0x14u32))); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = eax.wrapping_add(ecx).wrapping_add(0x5au32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(edi, 0x14u32, 32).1;
                if (fl.sf != fl.of) { pc = 27; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = esp.wrapping_add(0x60u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x34u32));
                edx = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0x68u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x60u32), 0x6bu32);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                m.w32(esp.wrapping_add(0x64u32), edx);
                m.w32(esp.wrapping_add(0x6cu32), ecx);
                if !fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                m.w32(esp.wrapping_add(0x70u32), 0x1u32);
                pc = 30; continue;
            }
            30 => {
                edx = esp.wrapping_add(0x60u32);
                ecx = 0x6df2f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 33; continue;
            }
            31 => {
                ecx = m.r32(esp.wrapping_add(0x30u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                ebp = esi.wrapping_add(0x2340u32);
                ebx = esi.wrapping_add(0x9fcu32);
                pc = 32; continue;
            }
            32 => {
                ecx = m.r32(esi.wrapping_add(0x57d8u32));
                eax = edi;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                edx = m.r32(ebx);
                let (r, f) = lift::imul(eax, m.r32(esp.wrapping_add(0x14u32))); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = eax.wrapping_add(ecx).wrapping_add(0x5au32);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(esi.wrapping_add(0x57d8u32));
                eax = edi;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                edx = m.r32(ebp);
                let (r, f) = lift::imul(eax, m.r32(esp.wrapping_add(0x14u32))); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = eax.wrapping_add(ecx).wrapping_add(0x5au32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(edi, 0x14u32, 32).1;
                if (fl.sf != fl.of) { pc = 32; continue; }
                pc = 33; continue;
            }
            33 => {
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                edi = m.r32(esi.wrapping_add(0x18u32));
                ecx = esi.wrapping_add(0x387cu32);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x30u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x5cu32), edx);
                m.w32(esp.wrapping_add(0x54u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x50u32);
                m.w32(esp.wrapping_add(0x58u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x54u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x64);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00493320` ship_side_menu_layout_buttons
pub fn f_00493320(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(0x14);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    eax = esp.wrapping_add(0x14u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4417d0, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esi.wrapping_add(0xecu32));
    edi = m.r32(0x6cbd9cu32);
    ebx = esi.wrapping_add(0xecu32);
    ecx = esi.wrapping_add(0x48cu32);
    m.w32(esp.wrapping_add(0x10u32), ecx);
    edx = esi.wrapping_add(0x574u32);
    ecx = ebx;
    ebp = esi.wrapping_add(0x2bcu32);
    m.w32(esp.wrapping_add(0x14u32), edx);
    { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    edx = m.r32(ebp);
    ecx = ebp;
    let (r, f) = lift::sub(edi, eax, 32); fl = f;
    edi = r;
    { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = esi.wrapping_add(0x1d4u32);
    let (r, f) = lift::sub(edi, eax, 32); fl = f;
    edi = r;
    eax = m.r32(ecx);
    { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    edx = m.r32(esi.wrapping_add(0x3a4u32));
    ecx = esi.wrapping_add(0x3a4u32);
    let (r, f) = lift::sub(edi, eax, 32); fl = f;
    edi = r;
    { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = m.r32(esp.wrapping_add(0x10u32));
    let (r, f) = lift::sub(edi, eax, 32); fl = f;
    edi = r;
    eax = m.r32(ecx);
    { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = m.r32(esp.wrapping_add(0x14u32));
    let (r, f) = lift::sub(edi, eax, 32); fl = f;
    edi = r;
    edx = m.r32(ecx);
    { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    let (r, f) = lift::sub(edi, eax, 32); fl = f;
    edi = r;
    eax = 0x92492493u32;
    { let r = i64::from(eax as i32) * i64::from(edi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
    let (r, f) = lift::add(edx, edi, 32); fl = f;
    edx = r;
    ecx = ebx;
    let (r, f) = lift::sar(edx, 0x2u32, 32, fl); fl = f;
    edx = r;
    eax = edx;
    let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
    eax = r;
    let (r, f) = lift::add(edx, eax, 32); fl = f;
    edx = r;
    eax = esp.wrapping_add(0x18u32);
    edi = edx;
    edx = m.r32(esp.wrapping_add(0x18u32));
    let (r, f) = lift::add(edx, edi, 32); fl = f;
    edx = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x1cu32), edx);
    edx = m.r32(ebx);
    { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(ebx);
    ecx = ebx;
    { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = m.r32(esp.wrapping_add(0x18u32));
    let (r, f) = lift::add(eax, edi, 32); fl = f;
    eax = r;
    let (r, f) = lift::add(ecx, eax, 32); fl = f;
    ecx = r;
    eax = m.r32(ebp);
    m.w32(esp.wrapping_add(0x18u32), ecx);
    ecx = esp.wrapping_add(0x18u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = ebp;
    { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(ebp);
    ecx = ebp;
    { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = m.r32(esp.wrapping_add(0x18u32));
    let (r, f) = lift::add(eax, edi, 32); fl = f;
    eax = r;
    let (r, f) = lift::add(ecx, eax, 32); fl = f;
    ecx = r;
    eax = m.r32(esi.wrapping_add(0x1d4u32));
    m.w32(esp.wrapping_add(0x18u32), ecx);
    ebx = esi.wrapping_add(0x1d4u32);
    ecx = esp.wrapping_add(0x18u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = ebx;
    { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(ebx);
    ecx = ebx;
    { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = m.r32(esp.wrapping_add(0x18u32));
    let (r, f) = lift::add(eax, edi, 32); fl = f;
    eax = r;
    let (r, f) = lift::add(ecx, eax, 32); fl = f;
    ecx = r;
    eax = m.r32(esi.wrapping_add(0x3a4u32));
    m.w32(esp.wrapping_add(0x18u32), ecx);
    ebx = esi.wrapping_add(0x3a4u32);
    ecx = esp.wrapping_add(0x18u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = ebx;
    { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi.wrapping_add(0x65cu32));
    ecx = esi.wrapping_add(0x65cu32);
    eax = esp.wrapping_add(0x18u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(ebx);
    ecx = ebx;
    { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = m.r32(esp.wrapping_add(0x18u32));
    esi = m.r32(esp.wrapping_add(0x10u32));
    let (r, f) = lift::add(eax, edi, 32); fl = f;
    eax = r;
    let (r, f) = lift::add(ecx, eax, 32); fl = f;
    ecx = r;
    eax = m.r32(esi);
    m.w32(esp.wrapping_add(0x18u32), ecx);
    ecx = esp.wrapping_add(0x18u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi;
    { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi);
    ecx = esi;
    { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = m.r32(esp.wrapping_add(0x14u32));
    let (r, f) = lift::add(eax, edi, 32); fl = f;
    eax = r;
    edi = m.r32(esp.wrapping_add(0x18u32));
    edx = esp.wrapping_add(0x18u32);
    let (r, f) = lift::add(edi, eax, 32); fl = f;
    edi = r;
    eax = m.r32(ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w32(esp.wrapping_add(0x1cu32), edi);
    { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
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
    eax
}

/// `0X004934B0` ship_side_menu_refresh_route_controls
pub fn f_004934b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6604d2u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x44);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(esi.wrapping_add(0x38f8u32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                edi = esi.wrapping_add(0x38f4u32);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = edi;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                m.w32(esp.wrapping_add(0x24u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0xa0u32));
                ecx = m.r32(0x6dd7a4u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = eax;
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x132u32)))) & 0xffff);
                ecx = esi.wrapping_add(0x5420u32);
                m.w32(esp.wrapping_add(0x10u32), ebp);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x57c4u32));
                edx = (edx & !0xffff) | (((ebp & 0xffff)) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd72au32)), 16).1;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                if !fl.cf { pc = 23; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x136u32)))) & 0xff);
                ecx = esi.wrapping_add(0x5348u32);
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            5 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                { let t = m.r32(eax.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0x9fcu32));
                ecx = esi.wrapping_add(0x9fcu32);
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(0x6dd72cu32);
                edx = (edx & !0xff) | ((0x4u32) & 0xff);
                pc = 10; continue;
            }
            10 => {
                eax = ebp;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(edi.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                fl = lift::logic(u32::from(m.r8(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x3u32))) & (edx & 0xff), 8);
                eax = ecx.wrapping_add(eax.wrapping_mul(4));
                if !fl.zf { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x10u32))), (ebp & 0xffff), 16).1;
                if !fl.zf { pc = 10; continue; }
                pc = 12; continue;
            }
            12 => {
                pc = 14; continue;
            }
            13 => {
                m.w32(esp.wrapping_add(0x10u32), ebp);
                pc = 14; continue;
            }
            14 => {
                edx = esi.wrapping_add(0x2340u32);
                edi = esi.wrapping_add(0x589cu32);
                m.w32(esp.wrapping_add(0x18u32), edx);
                pc = 16; continue;
            }
            15 => {
                ecx = m.r32(0x6dd72cu32);
                pc = 16; continue;
            }
            16 => {
                let r = ebp & 0xffffu32; fl = lift::logic(r, 32);
                ebp = r;
                eax = ebp.wrapping_add(ebp.wrapping_mul(4));
                ebp = ebp.wrapping_add(eax.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebp = ebp.wrapping_add(ebp.wrapping_mul(4));
                let (r, f) = lift::shl(ebp, 0x2u32, 32, fl); fl = f;
                ebp = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebp).wrapping_add(0x2u32)))) & 0xff);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6dda00u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x20u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x60u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6dd72cu32);
                ecx = edi;
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ebp).wrapping_add(0x3u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & 0x8u32, 8);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba474u32);
                m.w8(esi.wrapping_add(ebx).wrapping_add(0x6abcu32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4e59u32);
                pc = 21; continue;
            }
            18 => {
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba476u32);
                m.w8(esi.wrapping_add(ebx).wrapping_add(0x6abcu32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4e5au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x104u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 22; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba478u32);
                m.w8(esi.wrapping_add(ebx).wrapping_add(0x6abcu32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4e58u32);
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(edi);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx.wrapping_add(0x104u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(0x6dd72cu32);
                let (r, f) = lift::add(edx, 0xf8u32, 32); fl = f;
                edx = r;
                ecx = esp.wrapping_add(0x1cu32);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ebp)))) & 0xffff);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esp.wrapping_add(0x18u32), edx);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x5cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x10u32))), (ebp & 0xffff), 16).1;
                if !fl.zf { pc = 15; continue; }
                pc = 23; continue;
            }
            23 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x57d4u32)), ebx, 32).1;
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                edi = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esi.wrapping_add(0x57d4u32), ebx);
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                ecx = esi.wrapping_add(0x387cu32);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x40u32), ecx);
                m.w32(esp.wrapping_add(0x38u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x34u32);
                ecx = 0x6da5f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x40u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(esi.wrapping_add(0x57d4u32));
                ecx = esi.wrapping_add(0x5420u32);
                fl = lift::sub(eax, 0x14u32, 32).1;
                if (fl.sf == fl.of) { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                pc = 27; continue;
            }
            27 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esi.wrapping_add(0x57d4u32));
                edx = m.r32(esi.wrapping_add(0x5428u32));
                ecx = esi.wrapping_add(0x5428u32);
                fl = lift::sub(ebx, 0x5u32, 32).1;
                eax = (eax & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x57d4u32));
                ebx = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::sub(eax, 0x6u32, 32).1;
                if (fl.sf == fl.of) { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                m.w32(esp.wrapping_add(0x18u32), eax);
                pc = 30; continue;
            }
            29 => {
                ecx = ebx.wrapping_add(0x6u32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                pc = 30; continue;
            }
            30 => {
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(0x57d8u32)), 32).1;
                if fl.zf { pc = 43; continue; }
                pc = 31; continue;
            }
            31 => {
                ebp = m.r32(esi.wrapping_add(0x3884u32));
                edi = m.r32(esi.wrapping_add(0x18u32));
                ecx = esi.wrapping_add(0x387cu32);
                m.w32(esi.wrapping_add(0x57d8u32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x30u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x50u32), edx);
                m.w32(esp.wrapping_add(0x48u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x44u32);
                m.w32(esp.wrapping_add(0x4cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x48u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(ebx & ebx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 34; continue; }
                pc = 32; continue;
            }
            32 => {
                edi = esi.wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(0x2340u32);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                pc = 33; continue;
            }
            33 => {
                eax = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if !fl.zf { pc = 33; continue; }
                pc = 34; continue;
            }
            34 => {
                eax = ebx.wrapping_add(0x6u32);
                fl = lift::sub(eax, 0x14u32, 32).1;
                if (fl.sf == fl.of) { pc = 37; continue; }
                pc = 35; continue;
            }
            35 => {
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edi = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2340u32);
                ecx = 0x14u32;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                pc = 36; continue;
            }
            36 => {
                edx = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if !fl.zf { pc = 36; continue; }
                pc = 37; continue;
            }
            37 => {
                fl = lift::sub(ebx, m.r32(esp.wrapping_add(0x18u32)), 32).1;
                if (fl.sf == fl.of) { pc = 40; continue; }
                pc = 38; continue;
            }
            38 => {
                eax = ebx.wrapping_mul(8);
                edx = ebx;
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::shl(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                ecx = ebx.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                edi = esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x589cu32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                ebp = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x2340u32);
                pc = 39; continue;
            }
            39 => {
                eax = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if !fl.zf { pc = 39; continue; }
                pc = 40; continue;
            }
            40 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(esp.wrapping_add(0x28u32));
                edi = 0x1u32;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(edi, ebx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::imul(edi, m.r32(esp.wrapping_add(0x2cu32))); fl = f;
                edi = r;
                ebx = esi.wrapping_add(0x2340u32);
                m.w32(esp.wrapping_add(0x10u32), edx);
                m.w32(esp.wrapping_add(0x30u32), ecx);
                ebp = esi.wrapping_add(0x589cu32);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                m.w32(esp.wrapping_add(0x18u32), 0x14u32);
                pc = 41; continue;
            }
            41 => {
                edx = m.r32(esi.wrapping_add(0x18u32));
                eax = m.r32(ebp.wrapping_add(0xffffb160u32));
                ecx = ebp.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = edi.wrapping_add(edx).wrapping_add(0x5au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = edi.wrapping_add(ecx).wrapping_add(0x5au32);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esi.wrapping_add(0x18u32));
                ebx = m.r32(ebp);
                ecx = esi.wrapping_add(0x9fcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = edi.wrapping_add(edx).wrapping_add(0x5au32);
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(esp.wrapping_add(0x28u32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = ebp;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edi, ebx, 32); fl = f;
                edi = r;
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebx, 0xf8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                if !fl.zf { pc = 41; continue; }
                pc = 42; continue;
            }
            42 => {
                ebx = m.r32(esp.wrapping_add(0x14u32));
                pc = 43; continue;
            }
            43 => {
                eax = m.r32(esi.wrapping_add(0x57d4u32));
                fl = lift::sub(eax, 0x14u32, 32).1;
                if (fl.sf == fl.of) { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                ecx = m.r32(0x6a450cu32);
                edx = eax;
                let (r, f) = lift::shl(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x2340u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x57d4u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, 0x6u32, 32).1;
                edx = m.r32(esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2340u32));
                ecx = esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2340u32);
                eax = (eax & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 45; continue;
            }
            45 => {
                fl = lift::logic(ebx & ebx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 48; continue; }
                pc = 46; continue;
            }
            46 => {
                edi = esi.wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(0x2340u32);
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                pc = 47; continue;
            }
            47 => {
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                if !fl.zf { pc = 47; continue; }
                pc = 48; continue;
            }
            48 => {
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(0x57d4u32)), 32).1;
                if (fl.sf == fl.of) { pc = 51; continue; }
                pc = 49; continue;
            }
            49 => {
                eax = ebx.wrapping_mul(8);
                edx = ebx;
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), 0x0u32);
                let (r, f) = lift::shl(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                ecx = ebx.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                edi = esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x2340u32);
                pc = 50; continue;
            }
            50 => {
                ebx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                fl = lift::sub(ebx, 0x6u32, 32).1;
                ebx = (ebx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x57d4u32));
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 50; continue; }
                pc = 51; continue;
            }
            51 => {
                eax = m.r32(esi.wrapping_add(0x57d4u32));
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x14u32, 32).1;
                if (fl.sf == fl.of) { pc = 54; continue; }
                pc = 52; continue;
            }
            52 => {
                edx = eax.wrapping_mul(8);
                ebx = 0x14u32;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                ecx = eax.wrapping_add(edx.wrapping_mul(4));
                edi = esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x589cu32);
                pc = 53; continue;
            }
            53 => {
                edx = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 53; continue; }
                pc = 54; continue;
            }
            54 => {
                eax = m.r32(esi.wrapping_add(0x57d4u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x14u32, 32).1;
                if (fl.sf == fl.of) { pc = 57; continue; }
                pc = 55; continue;
            }
            55 => {
                ecx = eax;
                edi = 0x14u32;
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                esi = esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2340u32);
                pc = 56; continue;
            }
            56 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 0xf8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 56; continue; }
                pc = 57; continue;
            }
            57 => {
                ecx = m.r32(esp.wrapping_add(0x50u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x50);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00493C20` ship_side_menu_draw_value_coin_stack
pub fn f_00493c20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x383cu32));
                ecx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x383cu32));
                edx = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_add(0x10);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 14; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, 0xcb2u32, 32).1;
                if (fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = 0x1u32;
                pc = 14; continue;
            }
            3 => {
                fl = lift::sub(eax, 0x2422u32, 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = 0x2u32;
                pc = 14; continue;
            }
            5 => {
                fl = lift::sub(eax, 0x5302u32, 32).1;
                if (fl.sf == fl.of) { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = 0x3u32;
                pc = 14; continue;
            }
            7 => {
                fl = lift::sub(eax, 0x9952u32, 32).1;
                if (fl.sf == fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = 0x4u32;
                pc = 14; continue;
            }
            9 => {
                fl = lift::sub(eax, 0xf712u32, 32).1;
                if (fl.sf == fl.of) { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = 0x5u32;
                pc = 14; continue;
            }
            11 => {
                fl = lift::sub(eax, 0x16c4bu32, 32).1;
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = 0x6u32;
                pc = 14; continue;
            }
            13 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(eax, 0x1f8e2u32, 32).1;
                edx = (edx & !0xff) | ((u32::from((fl.sf == fl.of))) & 0xff);
                let (r, f) = lift::add(edx, 0x7u32, 32); fl = f;
                edx = r;
                eax = edx;
                pc = 14; continue;
            }
            14 => {
                esi = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esp.wrapping_add(0x4u32));
                let (r, f) = lift::sub(esi, ecx, 32); fl = f;
                esi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = eax;
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                eax = 0x55555555u32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_add(0x18);
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(esi, edx, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 16; continue; }
                pc = 17; continue;
            }
            17 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 18; continue;
            }
            18 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00493D10` ship_side_menu_refresh_condition_and_repair_days
pub fn f_00493d10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 16; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xb8u32)), 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x4164u32));
                ecx = m.r32(esi.wrapping_add(0x3cd4u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                pc = 4; continue;
            }
            3 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                pc = 4; continue;
            }
            4 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(0x6dd7a4u32);
                ecx = m.r32(0x6dfc14u32);
                edi = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ebx, 32); fl = f;
                edi = r;
                eax = m.r32(edi.wrapping_add(0x18u32));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(m.r32(edi.wrapping_add(0x14u32)) as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ebx = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x51a3a0, ecx, &a); }
                ebp = m.r32(0x66e0f0u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ebp as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = ebp;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ebx;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = 0x51eb851fu32;
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::imul(ecx, edx); fl = f;
                ecx = r;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(ebx, edx, 32); fl = f;
                ebx = r;
                fl = lift::sub(ebx, 0x64u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ebx = 0x64u32;
                pc = 9; continue;
            }
            7 => {
                fl = lift::logic(ebx & ebx, 32);
                if (fl.sf == fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x5234u32)), ebx, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x3864u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6cbd5cu32);
                m.w32(esp.wrapping_add(0x14u32), edx);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esi.wrapping_add(0x38c0u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(ebx & ebx, 32);
                m.w32(esi.wrapping_add(0x5234u32), ebx);
                if (fl.sf == fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w32(esi.wrapping_add(0x5234u32), 0x0u32);
                pc = 12; continue;
            }
            12 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x134u32)))) & 0xffff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((eax & 0xffff), 0x6u32, 16).1;
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub((eax & 0xffff), 0x7u32, 16).1;
                if !fl.zf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x39u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(edx.wrapping_mul(8));
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x52aad0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = eax;
                eax = m.r32(esi.wrapping_add(0x5230u32));
                fl = lift::sub(edi, eax, 32).1;
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x3864u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6cbd5cu32);
                m.w32(esp.wrapping_add(0x10u32), edx);
                m.w32(esp.wrapping_add(0x8u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                eax = m.r32(esi.wrapping_add(0x38c0u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5230u32), edi);
                pc = 16; continue;
            }
            16 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00493ED0` ship_side_menu_update_crew_controls
pub fn f_00493ed0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = m.r32(0x6dd7a4u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = m.r32(esi.wrapping_add(0xa0u32));
                m.w8(esp.wrapping_add(0xfu32), (0x0u32) as u8);
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x42u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                if !fl.cf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6dd7a0u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(0x6de4b4u32);
                ebx = eax;
                eax = m.r32(ebx.wrapping_add(0x4u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = 0x6719f361u32;
                { let r = u64::from(eax) * u64::from(ecx); eax = r as u32; edx = (r >> 32) as u32; fl.cf = edx != 0; fl.of = edx != 0; }
                eax = m.r32(esi.wrapping_add(0x5804u32));
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::shr(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::shr(ecx, 0x10u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5808u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x9u32)))) & 0xff);
                fl = lift::sub((edx & 0xff), (eax & 0xff), 8).1;
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x580au32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0xbu32)))) & 0xff);
                fl = lift::sub((eax & 0xff), (edx & 0xff), 8).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5809u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0xau32)))) & 0xff);
                fl = lift::sub((edx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w32(esi.wrapping_add(0x5804u32), ecx);
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0xau32)))) & 0xff);
                m.w8(esi.wrapping_add(0x5809u32), ((eax & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0xbu32)))) & 0xff);
                m.w8(esi.wrapping_add(0x580au32), ((ecx & 0xff)) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x9u32)))) & 0xff);
                m.w8(esp.wrapping_add(0xfu32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0x5808u32), ((edx & 0xff)) as u8);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x1c1cu32));
                ecx = esi.wrapping_add(0x1c1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 8; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x1c1cu32));
                ecx = esi.wrapping_add(0x1c1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esi.wrapping_add(0x2150u32));
                ebx = esi.wrapping_add(0x2150u32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(ebx);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x518750, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x2150u32);
                { let t = m.r32(ebx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x3cu32)))) & 0xffff);
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                let r = (ecx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x1u32, 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                let r = (edx & 0xff) & 0x2u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), 0x2u32, 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                let r = (ecx & 0xff) & 0x10u32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x10u32, 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = eax;
                let r = edx & 0x200u32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(edx, 0x200u32, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                let r = (ecx & 0xff) & 0x40u32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x40u32, 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                let r = eax & 0x1000u32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, 0x1000u32, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xfu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x2060u32));
                ecx = esi.wrapping_add(0x1ed8u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x40u32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                m.w32(ecx.wrapping_add(0x180u32), eax);
                if (fl.sf == fl.of) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x45c930, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 17; continue;
            }
            17 => {
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x134u32))), 0x0u32, 16).1;
                if !fl.zf { pc = 31; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(esi.wrapping_add(0x2060u32));
                edx = m.r32(esi.wrapping_add(0x1decu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x40u32)))) & 0xffff);
                ebp = esi.wrapping_add(0x1decu32);
                fl = lift::sub(eax, ecx, 32).1;
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2060u32));
                edx = m.r32(esi.wrapping_add(0x1d04u32));
                ecx = esi.wrapping_add(0x1d04u32);
                fl = lift::logic(eax & eax, 32);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x40u32))), 0x0u32, 16).1;
                edx = m.r32(esi.wrapping_add(0x1ed8u32));
                ebx = esi.wrapping_add(0x1ed8u32);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2060u32));
                edx = m.r32(esi.wrapping_add(0x2068u32));
                ecx = esi.wrapping_add(0x2068u32);
                fl = lift::logic(eax & eax, 32);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x42u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                if !fl.cf { pc = 22; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x134u32))), 0x12u32, 16).1;
                if fl.zf { pc = 22; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 23; continue;
            }
            22 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(esi.wrapping_add(0x1c1cu32));
                ecx = esi.wrapping_add(0x1c1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esi.wrapping_add(0x2060u32));
                ecx = ebx;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x45c930, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            25 => {
                ecx = esi.wrapping_add(0x1d04u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(esi.wrapping_add(0x2060u32));
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x45c930, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            27 => {
                ecx = esi.wrapping_add(0x2068u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xa0u32));
                ecx = m.r32(esi.wrapping_add(0x2060u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), 0x5u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x45c930, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            29 => {
                ecx = esi.wrapping_add(0x1c1cu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 32; continue; }
                pc = 30; continue;
            }
            30 => {
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), 0x7u32);
                edx = esp.wrapping_add(0x10u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            31 => {
                eax = m.r32(esi.wrapping_add(0x1decu32));
                ecx = esi.wrapping_add(0x1decu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1d04u32));
                ecx = esi.wrapping_add(0x1d04u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1ed8u32));
                ecx = esi.wrapping_add(0x1ed8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x2068u32));
                ecx = esi.wrapping_add(0x2068u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1c1cu32));
                ecx = esi.wrapping_add(0x1c1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            32 => {
                ebx = esi.wrapping_add(0x2150u32);
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 36; continue; }
                pc = 33; continue;
            }
            33 => {
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x15cu32)))) & 0xff);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), 0xdu32);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                fl = lift::logic(eax & eax, 32);
                eax = m.r32(0x6999ecu32);
                m.w32(esp.wrapping_add(0x14u32), edx);
                if !fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(0x69c98cu32);
                pc = 35; continue;
            }
            35 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esp.wrapping_add(0x10u32);
                ecx = 0x6df2f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 36; continue;
            }
            36 => {
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

/// `0X004942C0` ship_side_menu_update_battle_controls
pub fn f_004942c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x6e59ccu32);
                esp = esp.wrapping_sub(0x14);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::sub((eax & 0xff), 0xffu32, 8).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6e55d0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x6087f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            2 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x3858u32)), eax, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = esi;
                m.w32(esi.wrapping_add(0x3858u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ebp = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x3cu32)))) & 0xffff);
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                let r = (edx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), 0x1u32, 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                let r = (ecx & 0xff) & 0x2u32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x2u32, 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                let r = (edx & 0xff) & 0x10u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), 0x10u32, 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                let r = (ecx & 0xff) & 0x80u32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x80u32, 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = eax;
                let r = edx & 0x100u32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(edx, 0x100u32, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = (eax & 0xff) & 0x40u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                fl = lift::sub((eax & 0xff), 0x40u32, 8).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0x3858u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x130u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x131u32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(m.r32(ebp.wrapping_add(0x120u32)), ebx, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 18; continue;
            }
            17 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(esi.wrapping_add(0x2258u32));
                edi = esi.wrapping_add(0x2258u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 23; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x3858u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(0x6e59ccu32))) & 0xff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x138u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x10u32), 0x92u32);
                eax = ecx;
                m.w8(esp.wrapping_add(0x14u32), ((edx & 0xff)) as u8);
                let (r, f) = lift::shr(eax, 0x6u32, 32, fl); fl = f;
                eax = r;
                fl = lift::logic((ecx & 0xff) & 0x20u32, 8);
                m.w8(esp.wrapping_add(0x15u32), (0x1u32) as u8);
                m.w8(esp.wrapping_add(0x16u32), ((eax & 0xff)) as u8);
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                let r = ecx & 0x1fu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = 0x1u32;
                let (r, f) = lift::shl(edx, (ecx & 0xff), 32, fl); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                m.w32(esp.wrapping_add(0x18u32), edx);
                pc = 22; continue;
            }
            21 => {
                let r = ecx & 0x1fu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = 0x1u32;
                let (r, f) = lift::shl(eax, (ecx & 0xff), 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), ebx);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                pc = 22; continue;
            }
            22 => {
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 23; continue;
            }
            23 => {
                edi = m.r32(esi.wrapping_add(0x3858u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x582au32)))) & 0xff);
                fl = lift::sub(edi, ebx, 32).1;
                if fl.zf { pc = 34; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x26u32)))) & 0xff);
                fl = lift::sub((edx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 27; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x494580, ecx, &a); }
                edi = m.r32(esi.wrapping_add(0x3858u32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5829u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x26u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), (ecx & 0xff), 8).1;
                m.w8(esi.wrapping_add(0x582au32), ((eax & 0xff)) as u8);
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                m.w8(esi.wrapping_add(0x5829u32), (0xffu32) as u8);
                pc = 27; continue;
            }
            27 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x130u32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = 0x1b4e81b5u32;
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                ecx = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x131u32)))) & 0xff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = 0x1b4e81b5u32;
                let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5830u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if !fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                fl = lift::sub((edx & 0xff), u32::from(m.r8(esi.wrapping_add(0x5831u32))), 8).1;
                if fl.zf { pc = 36; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esi.wrapping_add(0x5824u32));
                m.w8(esi.wrapping_add(0x5830u32), ((ecx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x5831u32), ((edx & 0xff)) as u8);
                fl = lift::sub(m.r32(eax.wrapping_add(0x14u32)), 0x5u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 32; continue; }
                pc = 30; continue;
            }
            30 => {
                fl = lift::sub(m.r32(eax.wrapping_add(0x4u32)), ebx, 32).1;
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                ecx = m.r32(eax.wrapping_add(0xcu32));
                let (r, f) = lift::add(ecx, 0x50u32, 32); fl = f;
                ecx = r;
                edx = m.r32(ecx);
                m.w32(esp.wrapping_add(0x10u32), edx);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                edi = m.r32(ecx.wrapping_add(0x8u32));
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                pc = 33; continue;
            }
            32 => {
                edi = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                pc = 33; continue;
            }
            33 => {
                eax = m.r32(esi.wrapping_add(0x581cu32));
                ebp = m.r32(esi.wrapping_add(0x18u32));
                ebx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                edx = ecx.wrapping_add(eax);
                ecx = m.r32(esi.wrapping_add(0x5818u32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, 0x10u32, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                ecx = esp.wrapping_add(0x10u32);
                let (r, f) = lift::add(edx, 0x10u32, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x1cu32), edi);
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
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
            34 => {
                m.w8(esi.wrapping_add(0x5830u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x5831u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x5829u32), (0xffu32) as u8);
                ebx = m.r32(ebp.wrapping_add(0x30u32));
                let r = ebx & 0xfffffff0u32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sbb((ebx & 0xff), (ebx & 0xff), fl.cf, 8); fl = f;
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let r = ebx & 0x4u32; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub((ebx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x494580, ecx, &a); }
                m.w8(esi.wrapping_add(0x582au32), ((ebx & 0xff)) as u8);
                pc = 36; continue;
            }
            36 => {
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

/// `0X00494580` ship_side_menu_invalidate_battle_icon
pub fn f_00494580(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                edx = m.r32(ecx.wrapping_add(0x5824u32));
                esp = esp.wrapping_sub(0x20);
                eax = m.r32(edx.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::sub(eax, 0x3u32, 32).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if (fl.zf || fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(edx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(edx.wrapping_add(0xcu32));
                let (r, f) = lift::add(eax, 0x30u32, 32); fl = f;
                eax = r;
                esi = m.r32(eax);
                m.w32(esp.wrapping_add(0x10u32), esi);
                esi = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x14u32), esi);
                esi = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x18u32), esi);
                esi = m.r32(eax.wrapping_add(0xcu32));
                pc = 4; continue;
            }
            3 => {
                esi = m.r32(esp.wrapping_add(0x1cu32));
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(edx.wrapping_add(0x14u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(edx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(edx.wrapping_add(0xcu32));
                edi = m.r32(eax);
                m.w32(esp.wrapping_add(0x20u32), edi);
                edi = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x24u32), edi);
                ebp = m.r32(eax.wrapping_add(0x8u32));
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                pc = 8; continue;
            }
            7 => {
                ebp = m.r32(esp.wrapping_add(0x28u32));
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(m.r32(edx.wrapping_add(0x14u32)), 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(edx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(edx.wrapping_add(0xcu32));
                let (r, f) = lift::add(edx, 0x10u32, 32); fl = f;
                edx = r;
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                ebx = m.r32(edx.wrapping_add(0x8u32));
                edi = m.r32(edx.wrapping_add(0xcu32));
                pc = 12; continue;
            }
            11 => {
                edi = m.r32(esp.wrapping_add(0x2cu32));
                ebx = m.r32(esp.wrapping_add(0x28u32));
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(ecx.wrapping_add(0x3820u32));
                eax = m.r32(edx.wrapping_add(0x14u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(edx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(edx.wrapping_add(0xcu32));
                edx = m.r32(eax);
                m.w32(esp.wrapping_add(0x20u32), edx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x24u32), edx);
                edx = m.r32(eax.wrapping_add(0x8u32));
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                pc = 16; continue;
            }
            15 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = eax;
                eax = m.r32(ecx.wrapping_add(0x580cu32));
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(ecx.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = esi;
                m.w32(esp.wrapping_add(0x18u32), ebx);
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = eax;
                eax = m.r32(ecx.wrapping_add(0x5810u32));
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(ecx.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x14u32), eax);
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
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

/// `0X00494990` ship_side_menu_on_ship_removed
pub fn f_00494990(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 1; continue;
            }
            1 => {
                ebx = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 14; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xbcu32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc4u32)), 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xb8u32)), 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp.wrapping_add(0x1cu32));
                edx = ebx;
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(u32::from(m.r16(edx)), (edi & 0xffff), 16).1;
                if fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 9; continue;
            }
            9 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            10 => {
                ecx = esp.wrapping_add(0x1cu32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x48fdf0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_add(0xc);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = esp.wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w8(esp.wrapping_add(0x18u32), (0x0u32) as u8);
                m.w32(esp.wrapping_add(0x10u32), edx);
                { let t = m.r32(eax.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            12 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebx))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esp.wrapping_add(0x1cu32))), 16).1;
                if !fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x3868u32), 0xffffffffu32);
                pc = 14; continue;
            }
            14 => {
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

/// `0X00494A80` ship_side_menu_on_convoy_removed
pub fn f_00494a80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc8u32)), 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx).wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esp.wrapping_add(0x8u32))), 16).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x3868u32), 0xffffffffu32);
                pc = 6; continue;
            }
            6 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00494AF0` ship_side_menu_enqueue_join_convoy
pub fn f_00494af0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(ecx.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(0x14);
                fl = lift::logic(eax & eax, 32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(u32::from(m.r8(ecx.wrapping_add(0xb0u32))), 0x8u32, 8).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(ecx.wrapping_add(0x5238u32));
                edi = 0x1u32;
                ebx = eax.wrapping_add(ecx.wrapping_mul(2));
                pc = 4; continue;
            }
            3 => {
                edi = m.r32(ecx.wrapping_add(0xacu32));
                ebx = eax;
                pc = 4; continue;
            }
            4 => {
                ecx = esp.wrapping_add(0xcu32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0xcu32), 0x18u32);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(edi & edi, 32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x10u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = esp.wrapping_add(0xcu32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(esi.wrapping_mul(2))))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, edi, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
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

/// `0X00494B80` ship_side_menu_create_convoy
pub fn f_00494b80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6604e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x50);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ecx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(ebp.wrapping_add(0xb4u32));
                ecx = m.r32(ebp.wrapping_add(0xbcu32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(ebp.wrapping_add(0xa0u32));
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                pc = 9; continue;
            }
            2 => {
                ecx = m.r32(ebp.wrapping_add(0x3cccu32));
                let r = esi | 0xffffffffu32; fl = lift::logic(r, 32);
                esi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, ebx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), esi);
                m.w32(esp.wrapping_add(0x10u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ebp.wrapping_add(0xa0u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                edx = m.r32(0x6dd7a4u32);
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x519c40, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                edi = m.r32(edi.wrapping_add(0x120u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::sub(eax, edi, 32).1;
                if (fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(ebp.wrapping_add(0xa0u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                m.w32(esp.wrapping_add(0x14u32), edi);
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                pc = 7; continue;
            }
            6 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(ebp.wrapping_add(0x3cccu32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 9; continue;
            }
            9 => {
                ecx = esp.wrapping_add(0x18u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x70u32)))) & 0xff);
                edi = m.r32(esp.wrapping_add(0x74u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                eax = esp.wrapping_add(0x18u32);
                let r = edi & 0xffu32; fl = lift::logic(r, 32);
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x1cu32), 0x15u32);
                m.w32(esp.wrapping_add(0x20u32), esi);
                m.w32(esp.wrapping_add(0x24u32), edx);
                m.w32(esp.wrapping_add(0x28u32), 0x5u32);
                m.w32(esp.wrapping_add(0x2cu32), edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r8(esp.wrapping_add(0x70u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x70u32), ecx);
                edx = m.r32(edi.wrapping_mul(4).wrapping_add(0x6dda00u32));
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = esi.wrapping_add(esi.wrapping_mul(2));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6dd7a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x70u32), ebx);
                eax = ecx.wrapping_add(edx).wrapping_add(0x160u32);
                ecx = m.r32(0x699d00u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x84u32));
                esp = esp.wrapping_add(0x14);
                ecx = esp.wrapping_add(0x40u32);
                m.w8(esp.wrapping_add(0x44u32), ((ebx & 0xff)) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6cbb10u32);
                m.w16(esp.wrapping_add(0x4au32), ((ebx & 0xffff)) as u16);
                m.w32(esp.wrapping_add(0x4cu32), ebx);
                m.w8(esp.wrapping_add(0x50u32), (0xffu32) as u8);
                m.w8(esp.wrapping_add(0x58u32), (0x1u32) as u8);
                m.w16(esp.wrapping_add(0x5au32), ((ebx & 0xffff)) as u16);
                m.w32(esp.wrapping_add(0x5cu32), ebx);
                m.w32(esp.wrapping_add(0x60u32), ebx);
                m.w8(esp.wrapping_add(0x53u32), (0xffu32) as u8);
                m.w8(esp.wrapping_add(0x51u32), ((ebx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x52u32), ((ebx & 0xff)) as u8);
                m.w32(esp.wrapping_add(0x44u32), eax);
                m.w32(esp.wrapping_add(0x54u32), 0x3ecccccdu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x41d9b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x70u32);
                m.w32(esp.wrapping_add(0x68u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(ebp.wrapping_add(0xb4u32));
                eax = m.r32(ebp.wrapping_add(0xb8u32));
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = esp.wrapping_add(0x2cu32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(ebp.wrapping_add(0x3cccu32));
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 17; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edi = 0x18u32;
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(ebp.wrapping_add(0xa0u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(esi.wrapping_mul(2))))) & 0xffff);
                eax = eax.wrapping_add(esi.wrapping_mul(2));
                fl = lift::sub(edx, ecx, 32).1;
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w32(esp.wrapping_add(0x30u32), ecx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x2cu32), edi);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                edx = esp.wrapping_add(0x2cu32);
                m.w32(esp.wrapping_add(0x34u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(ebp.wrapping_add(0x3cccu32));
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 14; continue; }
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(esp.wrapping_add(0x60u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x5c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00494F90` ship_side_menu_update_departure_text
pub fn f_00494f90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                esi = m.r32(0x6dd7a4u32);
                ecx = m.r32(edi.wrapping_add(0xa0u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, esi, 32); fl = f;
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                edx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                esi = edx.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x14u32)))) & 0xff);
                let (r, f) = lift::shr((eax & 0xff), 0x1u32, 8, fl); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                eax = (eax & !0xff) | ((!(eax & 0xff)) & 0xff);
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 11; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), 0x1u32, 16).1;
                if !fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(edi.wrapping_add(0x5894u32));
                ebx = edi.wrapping_add(0x5894u32);
                eax = m.r32(edx.wrapping_add(0xfffffff8u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = esp.wrapping_add(0xcu32);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x16u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6de4b4u32);
                ecx = 0x6de4a0u32;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5311d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(esp.wrapping_add(0xcu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1fu32)))) & 0xff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6dda00u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0d00u32));
                ecx = m.r32(0x699a34u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(edi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                esp = esp.wrapping_add(0x18);
                fl = lift::sub(edx, eax, 32).1;
                pc = 9; continue;
            }
            6 => {
                fl = lift::sub((ecx & 0xffff), 0x1u32, 16).1;
                if fl.zf { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(edi.wrapping_add(0x5894u32));
                ecx = edi.wrapping_add(0x5894u32);
                edx = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 11; continue; }
                pc = 8; continue;
            }
            8 => {
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                ecx = m.r32(edi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(ecx, eax, 32).1;
                pc = 9; continue;
            }
            9 => {
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                pc = 11; continue;
            }
            11 => {
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

/// `0X00495150` ship_side_menu_reset_subpage
pub fn f_00495150(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xbcu32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xd8u32));
                ecx = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = esi;
                m.w32(esi.wrapping_add(0xccu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            3 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00495190` ship_side_menu_get_ship_pick_result
pub fn f_00495190(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6e59cdu32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(u32::from(m.r8(0x6e59ccu32)), 0xffu32, 8).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x2u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            3 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(0x6dd7a4u32);
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                edx = edx.wrapping_add(edx.wrapping_mul(2));
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(edi).wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                esi = m.r32(ecx.wrapping_add(0xa0u32));
                eax = (eax & !0xff) | ((0x8u32) & 0xff);
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(u32::from(m.r8(ecx.wrapping_add(0xb0u32))), (eax & 0xff), 8).1;
                if !fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi))) & 0xffff);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edi).wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(edx.wrapping_add(edi).wrapping_add(0x8u32))), 16).1;
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                pc = 8; continue;
            }
            8 => {
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

/// `0X00495210` ship_side_menu_draw_artillery_slots
pub fn f_00495210(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebp = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(ebp.wrapping_add(0x5824u32));
                fl = lift::sub(m.r32(ecx.wrapping_add(0x14u32)), 0x5u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                let (r, f) = lift::add(ecx, 0x50u32, 32); fl = f;
                ecx = r;
                edi = m.r32(ecx);
                edx = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x18u32), edx);
                esi = m.r32(ecx.wrapping_add(0x8u32));
                ebx = m.r32(ecx.wrapping_add(0xcu32));
                pc = 4; continue;
            }
            3 => {
                edi = m.r32(esp.wrapping_add(0x14u32));
                esi = m.r32(esp.wrapping_add(0x1cu32));
                ebx = m.r32(esp.wrapping_add(0x20u32));
                eax = m.r32(esp.wrapping_add(0x30u32));
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = esi.wrapping_add(0xffffffbeu32);
                eax = 0x2aaaaaabu32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x582cu32)))) & 0xffff);
                esp = esp.wrapping_add(0x1c);
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                eax = ebx.wrapping_add(0xdu32);
                edi = edx;
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), eax);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esp.wrapping_add(0x30u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx).wrapping_add(0x13cu32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x48ed80, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = edi.wrapping_add(0x2u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                fl = lift::logic((ebx & 0xff) & 0x1u32, 8);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::add(ebx, 0x2u32, 32); fl = f;
                ebx = r;
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, 0x18u32, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                esi = m.r32(esp.wrapping_add(0x10u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = 0x2u32;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x582eu32)))) & 0xffff);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                pc = 10; continue;
            }
            9 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx).wrapping_add(0x13cu32)))) & 0xff);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x48ed80, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = edi.wrapping_add(0x2u32);
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                fl = lift::logic((ebx & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(ebx, 0x2u32, 32); fl = f;
                ebx = r;
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, 0x18u32, 32).1;
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 13; continue;
            }
            13 => {
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

/// `0X00495550` ship_side_menu_scan_saved_routes
pub fn f_00495550(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66053bu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x38);
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
                ecx = esp.wrapping_add(0x2cu32);
                m.w32(esi.wrapping_add(0x82ccu32), ebp);
                { let a = []; eax = h.call_at(esp, m, 0x638f07, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba47cu32);
                ecx = esp.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x54u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x34u32);
                m.w8(esp.wrapping_add(0x58u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x638fca, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::sub(eax, ebp, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x2cu32);
                { let a = []; eax = h.call_at(esp, m, 0x6391ea, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x82ccu32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(eax, ebp, 32).1;
                m.w32(esi.wrapping_add(0x82ccu32), edx);
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x82c8u32));
                fl = lift::sub(eax, ebp, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(eax.wrapping_add(0xfffffffcu32));
                edi = eax.wrapping_add(0xfffffffcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64f253u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                edi = m.r32(esi.wrapping_add(0x82ccu32));
                m.w32(esi.wrapping_add(0x82c8u32), ebp);
                edx = edi.wrapping_mul(4).wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x28u32), eax);
                fl = lift::sub(eax, ebp, 32).1;
                m.w8(esp.wrapping_add(0x50u32), (0x2u32) as u8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64f253u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4103c0u32);
                ebx = eax.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(eax, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 7; continue;
            }
            6 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x34u32);
                m.w8(esp.wrapping_add(0x58u32), (0x1u32) as u8);
                m.w32(esi.wrapping_add(0x82c8u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x638fca, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = m.r32(esi.wrapping_add(0x82ccu32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, ebp, 32).1;
                ecx = (ecx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(edi, ebp, 32).1;
                edx = (edx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                fl = lift::logic(edx & ecx, 32);
                m.w32(esp.wrapping_add(0x18u32), ebx);
                if fl.zf { pc = 16; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = esp.wrapping_add(0x2cu32);
                { let a = []; eax = h.call_at(esp, m, 0x6391ea, ecx, &a); }
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639351, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x82c8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w8(esp.wrapping_add(0x54u32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x24u32);
                m.w8(esp.wrapping_add(0x50u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x82c8u32));
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x24u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x10u32), eax);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w8(esp.wrapping_add(0x50u32), (0x6u32) as u8);
                fl = lift::logic(ebp & ebp, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0x82c8u32));
                ecx = esp.wrapping_add(0x14u32);
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x82c8u32));
                ecx = eax.wrapping_add(edi.wrapping_mul(4));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f709, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f709, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, ebx, 32).1;
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 11; continue;
            }
            11 => {
                pc = 15; continue;
            }
            12 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x18u32)), edi, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esi.wrapping_add(0x82c8u32));
                ecx = ecx.wrapping_add(ebx.wrapping_mul(4));
                edx = ecx.wrapping_add(0xfffffffcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, edi, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 13; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0x82c8u32));
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ecx.wrapping_add(edi.wrapping_mul(4));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x18u32));
                pc = 15; continue;
            }
            15 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x18u32), ebx);
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                m.w8(esp.wrapping_add(0x50u32), (0x5u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x50u32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x20u32);
                m.w8(esp.wrapping_add(0x50u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                edi = m.r32(esi.wrapping_add(0x82ccu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic(eax & eax, 32);
                edx = (edx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(edi & edi, 32);
                eax = (eax & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                fl = lift::logic(eax & edx, 32);
                if !fl.zf { pc = 8; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = esp.wrapping_add(0x1cu32);
                m.w8(esp.wrapping_add(0x50u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x2cu32);
                m.w32(esp.wrapping_add(0x50u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x638f44, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x48u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x44);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00495810` ship_side_menu_get_class_name
pub fn f_00495810(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6b9e74u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = m.r32(esp.wrapping_add(0xcu32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = esi;
    m.w32(esp.wrapping_add(0x8u32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ecx = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x4901c0 => f_004901c0(m, h, ecx, esp),
        0x490470 => f_00490470(m, h, ecx, esp),
        0x4904b0 => f_004904b0(m, h, ecx, esp),
        0x490600 => f_00490600(m, h, ecx, esp),
        0x4906e0 => f_004906e0(m, h, ecx, esp),
        0x490b10 => f_00490b10(m, h, ecx, esp),
        0x490f80 => f_00490f80(m, h, ecx, esp),
        0x4916e0 => f_004916e0(m, h, ecx, esp),
        0x491ca0 => f_00491ca0(m, h, ecx, esp),
        0x492150 => f_00492150(m, h, ecx, esp),
        0x492430 => f_00492430(m, h, ecx, esp),
        0x492bc0 => f_00492bc0(m, h, ecx, esp),
        0x492ca0 => f_00492ca0(m, h, ecx, esp),
        0x492ee0 => f_00492ee0(m, h, ecx, esp),
        0x493320 => f_00493320(m, h, ecx, esp),
        0x4934b0 => f_004934b0(m, h, ecx, esp),
        0x493c20 => f_00493c20(m, h, ecx, esp),
        0x493d10 => f_00493d10(m, h, ecx, esp),
        0x493ed0 => f_00493ed0(m, h, ecx, esp),
        0x4942c0 => f_004942c0(m, h, ecx, esp),
        0x494580 => f_00494580(m, h, ecx, esp),
        0x494990 => f_00494990(m, h, ecx, esp),
        0x494a80 => f_00494a80(m, h, ecx, esp),
        0x494af0 => f_00494af0(m, h, ecx, esp),
        0x494b80 => f_00494b80(m, h, ecx, esp),
        0x494f90 => f_00494f90(m, h, ecx, esp),
        0x495150 => f_00495150(m, h, ecx, esp),
        0x495190 => f_00495190(m, h, ecx, esp),
        0x495210 => f_00495210(m, h, ecx, esp),
        0x495550 => f_00495550(m, h, ecx, esp),
        0x495810 => f_00495810(m, h, ecx, esp),
        _ => return None,
    }))
}
